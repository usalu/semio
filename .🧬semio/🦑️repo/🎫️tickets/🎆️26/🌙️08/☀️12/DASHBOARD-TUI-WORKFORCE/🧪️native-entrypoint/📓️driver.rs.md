/// 🚪️ Drives a headless process future and retires its mounted native I/O owners.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn drive_native_entrypoint<F: std::future::Future>(future: F) -> F::Output {
    use std::sync::atomic::Ordering;
    use std::task::Poll;
    let mut future = Some(Box::pin(future));
    let mut output = None;
    semio_framework_async::block_on(std::future::poll_fn(|context| {
        if let Some(active) = future.as_mut() {
            if let Poll::Ready(value) = active.as_mut().poll(context) {
                output = Some(value);
                future = None;
            }
        }
        crate::pump_renderer_io_sessions(16);
        let pending = crate::RENDERER_IO_SLOTS.iter().any(|slot| matches!(slot.state.load(Ordering::Acquire), crate::RENDERER_IO_LIVE | crate::RENDERER_IO_CHECKED_OUT));
        if !pending {
            if let Some(value) = output.take() { return Poll::Ready(value); }
        } else {
            context.waker().wake_by_ref();
        }
        Poll::Pending
    }))
}
