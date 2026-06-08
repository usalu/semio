//! ⚙️ Offload CPU-heavy kernel work to the rayon thread pool.

use std::future::Future;

/// 🧵 Run a closure on the rayon pool (or inline when `parallel` is disabled).
pub async fn run_blocking<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    #[cfg(feature = "parallel")]
    {
        let (tx, rx) = futures::channel::oneshot::channel();
        rayon::spawn(move || {
            let _ = tx.send(f());
        });
        rx.await.expect("blocking task dropped")
    }
    #[cfg(not(feature = "parallel"))]
    {
        f()
    }
}

/// ⏳ Block the current thread until an async kernel call completes.
pub fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    pollster::block_on(future)
}
