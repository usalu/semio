"""🐍️ WG7 K2–K7 — browser document actor dials the hub over `semio.session.v1` (anchored, refuses on drift)."""
import re
from pathlib import Path

P = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs")
C = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs")
t = P.read_text(encoding="utf-8")
c = C.read_text(encoding="utf-8")

def swap(text, old, new):
    assert text.count(old) == 1, (text.count(old), old[:140])
    return text.replace(old, new, 1)

BROWSER = 'all(target_arch = "wasm32", not(target_env = "p2"))'

c = swap(c, '''fn decode_lower_hex_32(value: &str) -> Option<[u8; 32]> {''', '''/// 🔡️ Decodes one 64-digit lower-hex digest (a pack schema hash on the wire) into its 32 bytes.
pub fn decode_lower_hex_32(value: &str) -> Option<[u8; 32]> {''')

t = swap(t, '''//#region 🔖️WireBridge
// 🎯️ W6 kernel unification''', '''//#region 🔖️DocumentSocketConnect
/// @emoji 🧬️ The pack schema identity a document socket's hello and bootstrap are checked against: the
/// linked codec's when this process registered one, otherwise the verified execution-target lease's —
/// the hub-selected package this client mounted, whose descriptor carries the kind the host links no
/// codec for. Neither is no identity at all, and the caller refuses to dial rather than guess.
pub async fn document_pack_schema_hash(schema: &str, lease: Option<&crate::os_directory::DocumentExecutionTargetLeaseFieldsV1>) -> Option<[u8; 32]> {
    if let Ok(Some(codec)) = crate::os_store::document_codec(schema).await {
        return Some(codec.pack_schema_hash);
    }
    let lease = lease.filter(|lease| lease.artifact.schema == schema)?;
    crate::os_directory::client::decode_lower_hex_32(&lease.artifact.pack_schema_hash)
}

/// @emoji 🎫️ What a document actor asks of the hub's open plan for its binding.
pub fn document_socket_expectation(schema: &str, pack_schema_hash: [u8; 32], surface: Option<&str>, lease: Option<&crate::os_directory::DocumentExecutionTargetLeaseFieldsV1>) -> crate::os_directory::client::DocumentSocketExpectationV1 {
    crate::os_directory::client::DocumentSocketExpectationV1 { artifact_schema: schema.to_string(), pack_schema_hash, requested_surface_id: surface.map(str::to_string), lease: lease.cloned() }
}

/// @emoji 🧷️ The one binding a document actor holds a hub socket for.
#[derive(Clone, Copy, Debug)]
pub struct DocumentSocketBinding<'a> {
    pub hub_base_url: &'a str,
    pub space_id: &'a str,
    pub document_id: &'a str,
    pub schema: &'a str,
    pub pack_schema_hash: [u8; 32],
    pub surface: Option<&'a str>,
    pub lease: Option<&'a crate::os_directory::DocumentExecutionTargetLeaseFieldsV1>,
}

/// @emoji ⚖️ Whether one admitted authority may carry this binding at `now_ms`: unexpired, from the bound
/// hub origin, for exactly this scope, schema, pack identity and surface, and — when the client verified
/// an execution-target lease — for exactly that lease.
pub fn document_socket_authority_admits(authority: &crate::os_directory::client::DocumentSocketAuthorityV1, binding: &DocumentSocketBinding<'_>, now_ms: u64) -> bool {
    authority.expires_at_unix_ms > now_ms
        && authority.hub_origin.trim_end_matches('/') == binding.hub_base_url.trim_end_matches('/')
        && authority.scope.space_id == binding.space_id
        && authority.scope.document_id == binding.document_id
        && authority.artifact.schema == binding.schema
        && authority.pack_schema_hash == binding.pack_schema_hash
        && binding.surface.is_none_or(|surface| surface == authority.surface.surface_id)
        && binding.lease.is_none_or(|lease| authority.matches_lease_fields(lease))
}

/// @emoji 👋️ The client-first frame every document socket opens with.
pub fn document_socket_hello(schema: &str, pack_schema_hash: [u8; 32], resume_token: Option<String>, frontier: Option<RuntimeFrontierSummary>) -> ClientFrame {
    ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: schema.to_string(), pack_schema_hash, resume_token, frontier }
}

/// @emoji 🔐️ The one ordered subprotocol offer a document socket dials with — the grant receipt's
/// protocol, then the session capability — which a browser joins into the single
/// `Sec-WebSocket-Protocol` value the hub splits on `", "`.
pub fn document_socket_protocols(receipt_protocol: &str, capability: &str) -> [String; 2] {
    [receipt_protocol.to_string(), capability.to_string()]
}

/// @emoji 🎲️ One document actor's hybrid-logical-clock replica seed: platform entropy, so two replicas
/// of one user (two tabs, two processes) never tie on `(seed, counter)` — H4's per-replica law.
pub fn replica_hlc_seed() -> Result<u64, crate::os_identity::EntropyError> {
    crate::os_identity::entropy_u64()
}
//#endregion 🔖️DocumentSocketConnect

//#region 🔖️DocumentSocketDoor
/// @emoji 📬️ One observation of a browser document socket's receive side.
#[cfg(''' + BROWSER + ''')]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentSocketPoll {
    Frame(Vec<u8>),
    Pending,
    Closed(Option<u16>),
    Lost(u32),
}

/// @emoji 🔌️ One page-owned document socket as the browser actor drives it: never blocking, polled on
/// the actor's own cadence, and honest about loss — a dropped frame is [`DocumentSocketPoll::Lost`],
/// which the actor answers with a reconnect and a hub catch-up rather than a silent gap.
#[cfg(''' + BROWSER + ''')]
pub trait DocumentSocket {
    fn is_open(&self) -> bool;
    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), String>;
    fn poll(&mut self) -> DocumentSocketPoll;
    fn close(&mut self);
}

/// @emoji ☎️ Opens document sockets for the browser actor. The host injects it (the wgpu renderer's
/// duplex socket door), so the kernel names no page API and every browser socket has one owner.
#[cfg(''' + BROWSER + ''')]
pub trait DocumentSocketDialer {
    fn dial(&self, url: &str, protocols: &[String]) -> Result<Box<dyn DocumentSocket>, String>;
}
//#endregion 🔖️DocumentSocketDoor

//#region 🔖️WireBridge
// 🎯️ W6 kernel unification''')

t = swap(t, '''    socket_grant_source: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<dyn crate::os_directory::client::HubSocketGrantSource>>>>,
    cancel: semio_framework_async::CancelToken,
}

impl Clone for ArtifactHost {
    fn clone(&self) -> Self {
        let mut state = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.host_references = state.host_references.checked_add(1).expect("artifact host reference capacity exhausted");
        drop(state);
        Self { inner: self.inner.clone(), pool: self.pool.clone(), credential: self.credential.clone(), socket_grant_source: self.socket_grant_source.clone(), cancel: self.cancel.clone() }
    }
}''', '''    socket_grant_source: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<dyn crate::os_directory::client::HubSocketGrantSource>>>>,
    #[cfg(''' + BROWSER + ''')]
    document_socket_dialer: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<dyn DocumentSocketDialer>>>>,
    cancel: semio_framework_async::CancelToken,
}

impl Clone for ArtifactHost {
    fn clone(&self) -> Self {
        let mut state = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.host_references = state.host_references.checked_add(1).expect("artifact host reference capacity exhausted");
        drop(state);
        Self {
            inner: self.inner.clone(),
            pool: self.pool.clone(),
            credential: self.credential.clone(),
            socket_grant_source: self.socket_grant_source.clone(),
            #[cfg(''' + BROWSER + ''')]
            document_socket_dialer: self.document_socket_dialer.clone(),
            cancel: self.cancel.clone(),
        }
    }
}''')

t = swap(t, '''            socket_grant_source: std::sync::Arc::new(std::sync::RwLock::new(None)),
            cancel: semio_framework_async::CancelToken::root_now(),
        }
    }''', '''            socket_grant_source: std::sync::Arc::new(std::sync::RwLock::new(None)),
            #[cfg(''' + BROWSER + ''')]
            document_socket_dialer: std::sync::Arc::new(std::sync::RwLock::new(None)),
            cancel: semio_framework_async::CancelToken::root_now(),
        }
    }''')

t = swap(t, '''    pub fn local_hub_ready(&self) -> bool {
        self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() && self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }''', '''    /// @emoji ☎️ Installs the browser's document-socket dialer; every later hub document actor dials through it.
    #[cfg(''' + BROWSER + ''')]
    pub fn set_document_socket_dialer(&self, dialer: std::sync::Arc<dyn DocumentSocketDialer>) {
        *self.document_socket_dialer.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(dialer);
    }

    /// @emoji 🟢️ Whether a hub document opened now can dial at once: a credential, a grant source and,
    /// in a browser, the socket dialer the page owns.
    pub fn local_hub_ready(&self) -> bool {
        let admitted = self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() && self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        #[cfg(''' + BROWSER + ''')]
        let admitted = admitted && self.document_socket_dialer.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        admitted
    }''')

t = swap(t, '''        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        spawn_actor(self.pool.clone(), config, remote, cmd_rx, event_tx.clone()).await;
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        let _ = (&self.pool, config, remote, cmd_rx);''', '''        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        spawn_actor(
            config,
            remote,
            cmd_rx,
            event_tx.clone(),
            wasm_actor::WasmActorHub { credential: self.credential.clone(), socket_grant_source: self.socket_grant_source.clone(), dialer: self.document_socket_dialer.clone(), lease: document_execution_target_lease, cancel: document_cancel.clone() },
        )
        .await;
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        let _ = (&self.pool, config, remote, cmd_rx, document_execution_target_lease);''')


t = swap(t, "use tokio::sync::{broadcast, mpsc};\n", "use tokio::sync::broadcast;\n")
t = swap(t, """//! - **Browser wgpu build** (`wasm32-unknown-unknown`): the actor runs on the owned browser-local
//!   executor with a `web_sys::WebSocket` semio_hub transport (no threads, no filesystem). The
//!   production browser shell instead uses a TS twin (`🏪️store/👷️worker/🟦️.ts`, WS-E); this wasm actor
//!   keeps the crate coherent for a future in-wasm host.""", """//! - **Browser wgpu build** (`wasm32-unknown-unknown`): the actor runs on the owned browser-local
//!   executor and dials its hub through the same `semio.session.v1` admission as the native actor, over
//!   the socket the host's injected `DocumentSocketDialer` opens (no threads, no filesystem). The React
//!   shell's browser host keeps its TS twin (`🏪️store/👷️worker/🟦️.ts`, WS-E).""")
start = t.index("//#region 🔖️WasmActor\n")
end = t.index("//#endregion 🔖️WasmActor\n") + len("//#endregion 🔖️WasmActor\n")
t = t[:start] + Path(".tmp-ticket/wp-wg7/k4-wasm-actor.rs.txt").read_text(encoding="utf-8") + t[end:]

P.write_text(t, encoding="utf-8")
C.write_text(c, encoding="utf-8")
print("k2-k7 browser actor: applied")
