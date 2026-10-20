import { create, edit, replaceBetween, replaceFrom } from "./lib.mjs";
import { MCP, NATIVE_ACTOR, SYNC, WS, src } from "./paths.mjs";

export function build() {
  const actor = `${NATIVE_ACTOR}/🦀️.rs`;
  create(`${NATIVE_ACTOR}/🌐️document-socket-reactor/🦀️.rs`, src("tokio/document-socket-reactor.rs"));
  create(`${SYNC}/🖥️native/🧪️tests/🌐️document-socket-reactor/🦀️.rs`, src("tokio/reactor-test.rs"));
  edit(actor, `pub use native_artifact_host::{ArtifactChannels, ArtifactHost};\n`, `pub use native_artifact_host::{ArtifactChannels, ArtifactHost};\n\n#[path = "🌐️document-socket-reactor/🦀️.rs"]\nmod document_socket_reactor;\npub use document_socket_reactor::{DocumentSocketReactor, DocumentSocketReactorError, DocumentSocketReactorScope};\nuse document_socket_reactor::pump_events;\n`);
  edit(actor, `mod echo_suppression_pin;`, `mod echo_suppression_pin;\n\n#[cfg(test)]\n#[path = "../🧪️tests/🌐️document-socket-reactor/🦀️.rs"]\nmod document_socket_reactor_tests;`);
  edit(`${NATIVE_ACTOR}/🏠️native-artifact-host/🦀️.rs`, `    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {`, src("tokio/host-pump.rs") + `    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {`);
  edit(`${SYNC}/🖥️native/🦀️.rs`, `pub use native_actor::{ArtifactActorRunnerHandle, ArtifactActorRunnerTicket, ArtifactActorTerminalJob, ArtifactActorTerminalReason, ArtifactActorTerminalState, ArtifactChannels, ArtifactHost};`, `pub use native_actor::{ArtifactActorRunnerHandle, ArtifactActorRunnerTicket, ArtifactActorTerminalJob, ArtifactActorTerminalReason, ArtifactActorTerminalState, ArtifactChannels, ArtifactHost, DocumentSocketReactor, DocumentSocketReactorError, DocumentSocketReactorScope};`);

  const codec = `${WS}/🗂️guest-document-codec/🦀️.rs`;
  replaceBetween(codec, "/// 🌐️ The process's tokio I/O reactor, for the hub DOCUMENT SOCKET and nothing else.", `.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("building the hub document-socket reactor: {error}")))\n}\n`, src("tokio/codec-reactor.rs"));

  const hub = `${WS}/🛂️gateway-backend/🌐️hub-session-documents/🦀️.rs`;
  replaceFrom(hub, "    pub(crate) fn watch_hub_document(", src("tokio/hub-watch.rs"));
  edit(`${MCP}/⚙️plugin-runtime/📦️packages/🦀️rust/Cargo.toml`, `tokio = { workspace = true, features = ["full"] }\n`, ``);
}
