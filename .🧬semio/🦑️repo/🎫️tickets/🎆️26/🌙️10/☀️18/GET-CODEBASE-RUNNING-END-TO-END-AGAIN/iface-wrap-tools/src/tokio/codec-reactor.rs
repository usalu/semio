/// 🌐️ The process's document-socket I/O reactor, for the hub DOCUMENT SOCKET and nothing else: the one [`store::sync::DocumentSocketReactor`] the native artifact actors poll their own socket against, so this gateway owns no runtime of its own.
///
/// 🐛️ A hub document opened here dials its socket on a `WorkerPool` thread that no Tokio runtime owns (`semio_framework_os_services::TokioHostRuntime` carries no reactor despite the name), so the first dial panicked with "there is no reactor running" until the caller entered a reactor around `ArtifactHost::open` and the guest codec calls (measured live on hub 7681, 2026-09-22). The same reactor drives the event pump of [`store::sync::ArtifactHost::pump_events_key`].
#[cfg(not(target_arch = "wasm32"))]
pub fn hub_socket_reactor() -> Result<&'static store::sync::DocumentSocketReactor, GatewayError> {
    store::sync::DocumentSocketReactor::shared().map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("starting the hub document-socket reactor: {error}")))
}
