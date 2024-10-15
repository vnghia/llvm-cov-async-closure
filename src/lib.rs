#![feature(async_closure)]

use futures::{stream, StreamExt};

async fn test_async_closure() -> Vec<i32> {
    stream::iter(0..3).then(async |i| i).collect().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let result = test_async_closure().await;
        assert_eq!(result, vec![0, 1, 2]);
    }
}
