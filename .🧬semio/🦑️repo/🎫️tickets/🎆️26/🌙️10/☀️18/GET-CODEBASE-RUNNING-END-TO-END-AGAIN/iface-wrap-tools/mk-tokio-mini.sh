#!/bin/bash
# Compiles and runs the staged DocumentSocketReactor + event pump against real tokio in a throw-away crate under /private/tmp (stub store/async crates, real files via #[path]); never touches the repository.
set -eu
A="$(cd "$(dirname "$0")" && pwd)"
M=/private/tmp/iface-wrap/tokio-mini
rm -rf "$M"; mkdir -p "$M/src" "$M/stub-sync/src" "$M/stub-async/src"
cat > "$M/Cargo.toml" <<TOML
[package]
name = "tokio-mini"
version = "0.1.0"
edition = "2021"
[workspace]
[dependencies]
tokio = { version = "1", default-features = false, features = ["sync", "macros", "net", "rt", "time"] }
semio-framework-os-store-sync = { path = "stub-sync" }
semio-framework-async = { path = "stub-async" }
TOML
printf '[package]\nname = "semio-framework-os-store-sync"\nversion = "0.1.0"\nedition = "2021"\n' > "$M/stub-sync/Cargo.toml"
printf '[package]\nname = "semio-framework-async"\nversion = "0.1.0"\nedition = "2021"\n' > "$M/stub-async/Cargo.toml"
cat > "$M/stub-sync/src/lib.rs" <<'RS'
#[derive(Clone, Debug)]
pub enum ArtifactEvent { DocumentBackbone { message: Vec<u8> }, DocumentArchiveAbsent }
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactDocumentKey(pub String);
RS
cat > "$M/stub-async/src/lib.rs" <<'RS'
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
struct ThreadWaker(std::thread::Thread);
impl Wake for ThreadWaker { fn wake(self: Arc<Self>) { self.0.unpark(); } }
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    loop { if let Poll::Ready(value) = future.as_mut().poll(&mut context) { return value; } std::thread::park(); }
}
pub mod poll {
    pub fn resolve_ready<F: std::future::Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        match future.as_mut().poll(&mut context) { std::task::Poll::Ready(value) => value, std::task::Poll::Pending => panic!("not ready") }
    }
}
RS
HOST="$(sed 's/^/    /' "$A/src/tokio/host-pump.rs")"
cat > "$M/src/lib.rs" <<RS
pub mod actor {
    pub mod actor_runner_scheduling_terminal_and_spawn {
        pub(super) fn document_socket_io_reactor() -> Option<tokio::runtime::Handle> {
            static REACTOR: std::sync::OnceLock<Option<tokio::runtime::Handle>> = std::sync::OnceLock::new();
            REACTOR
                .get_or_init(|| {
                    let runtime = tokio::runtime::Builder::new_current_thread().enable_io().enable_time().build().ok()?;
                    let handle = runtime.handle().clone();
                    std::thread::Builder::new().name("semio-document-io".to_string()).stack_size(1 << 20).spawn(move || runtime.block_on(std::future::pending::<()>())).ok()?;
                    Some(handle)
                })
                .clone()
        }
    }
    #[path = "$A/src/tokio/document-socket-reactor.rs"]
    mod document_socket_reactor;
    pub use document_socket_reactor::{DocumentSocketReactor, DocumentSocketReactorError, DocumentSocketReactorScope};
    use document_socket_reactor::pump_events;
    use semio_framework_os_store_sync::{ArtifactDocumentKey, ArtifactEvent};
    pub struct ArtifactHost { pub events: tokio::sync::broadcast::Sender<ArtifactEvent> }
    impl ArtifactHost {
        pub async fn subscribe_key(&self, _key: &ArtifactDocumentKey) -> tokio::sync::broadcast::Receiver<ArtifactEvent> { self.events.subscribe() }
$(cat "$A/src/tokio/host-pump.rs")
    }
    #[cfg(test)]
    #[test]
    fn the_host_pump_runs_the_handler_on_the_reactor_thread() {
        let (events, _keep) = tokio::sync::broadcast::channel(8);
        let host = ArtifactHost { events: events.clone() };
        let (sender, receiver) = std::sync::mpsc::channel();
        host.pump_events_key(DocumentSocketReactor::shared().unwrap(), &ArtifactDocumentKey("k".into()), move |event| { if let ArtifactEvent::DocumentBackbone { message } = event { sender.send((message, std::thread::current().name().map(str::to_string))).unwrap(); } });
        events.send(ArtifactEvent::DocumentBackbone { message: b"x".to_vec() }).unwrap();
        let (message, thread) = receiver.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert_eq!((message, thread.as_deref()), (b"x".to_vec(), Some("semio-document-io")));
    }
    #[cfg(test)]
    #[path = "$A/src/tokio/reactor-test.rs"]
    mod document_socket_reactor_tests;
}
RS
cd "$M"
CARGO_TARGET_DIR=/private/tmp/iface-wrap/target nice -n 10 cargo test --offline 2>&1 | grep -E "^test |test result|^error" 
