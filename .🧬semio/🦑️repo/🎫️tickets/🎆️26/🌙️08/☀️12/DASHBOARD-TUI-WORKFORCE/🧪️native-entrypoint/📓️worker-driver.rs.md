/// 🧑‍🏭️ Advances bounded native I/O turns on the application worker before presentation exists.
#[cfg(not(target_arch = "wasm32"))]
async fn drive_app_task<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    std::future::poll_fn(|cx| {
        let result = future.as_mut().poll(cx);
        let advanced = pump_renderer_io_sessions(16);
        if result.is_pending() && advanced != 0 {
            cx.waker().wake_by_ref();
        }
        result
    }).await
}

