"""🐍️ N2 — anchored, reversible edits that un-gate the wgpu shell's document relay for wasm32.

Usage: python3 🐍️n2-relay-edits.py apply|revert|status [bridge|relay]  (run from the repo root)
`bridge` is the JS program-bridge document door (compiles on wasm32 without the kernel's `sync`);
`relay` is the shell's un-gated document half, which needs `sync` on wasm32 — blocked by the frozen
kernel manifest (`ureq` rides the `sync` feature unconditionally), see `📓️n2-…md` §6.
Every hunk is an exact (old, new) pair; apply refuses when an anchor is missing or ambiguous, so a
peer's concurrent edit is reported instead of clobbered. revert swaps each pair back.
"""
import sys
from pathlib import Path

ROOT = Path.cwd()
WGPU = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
SHELL = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
BRIDGE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs"
RENDERER = WGPU + "/🧊️renderer/🦀️.rs"
CARGO = WGPU + "/📦️packages/🦀️rust/Cargo.toml"
NATIVE = '#[cfg(not(target_arch = "wasm32"))]\n'

INSTALL_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs"

HUNKS = {CARGO: [], SHELL: [], BRIDGE: [], RENDERER: [], INSTALL_LAWS: []}


def hunk(path, old, new, group=None):
    HUNKS[path].append((old, new, group or ("bridge" if path == BRIDGE else "relay")))


def ungate(path, item, indent=""):
    hunk(path, indent + NATIVE + indent + item, indent + item)


# ── Cargo: the browser target links the kernel's `store_sync::sync` vocabulary ────────────────
hunk(CARGO, '''semio-framework-async = { workspace = true }
getrandom = { version = "0.3.4", features = ["wasm_js"] }''', '''semio-framework-async = { workspace = true }
# 📡️ The browser half of the document relay: `ArtifactHost` and the `store_sync::sync` vocabulary the
# shell's one `open_document` body names on both targets, plus the `broadcast` receiver it drains.
semio-framework-os-kernel = { path = "../../../../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel", features = ["sync"] }
tokio = { version = "1", features = ["sync"], default-features = false }
getrandom = { version = "0.3.4", features = ["wasm_js"] }''')

# ── Shell: imports ──────────────────────────────────────────────────────────────────────────────
ungate(SHELL, "use store_sync::PresencePeer;\n")
ungate(SHELL, "use store_sync::sync::{ArtifactActorConfig, ArtifactActorMsg, ArtifactDocumentKey, ArtifactEvent, ArtifactHost, ArtifactMailboxSender, ArtifactSyncStatus, PersistenceBinding, RemoteState};\n")
hunk(SHELL, "    identity::Identity,\n    mint_directory_command_request_id,", "    identity::{Identity, actor_id},\n    mint_directory_command_request_id,")
hunk(SHELL, "identity::{IdentityOutcome, IdentityStatus, actor_id, claimed_local_hub_credential, restore_claimed},", "identity::{IdentityOutcome, IdentityStatus, claimed_local_hub_credential, restore_claimed},")

# ── Shell: pure identity/binding helpers ────────────────────────────────────────────────────────
ungate(SHELL, "fn shell_actor(identity: Option<&Identity>, session_id: &str, instance_id: u32) -> String {")
ungate(SHELL, "fn default_persistence_bindings(identity: Option<&Identity>, space_id: Option<&str>, data_dir: Option<&std::path::Path>, surface: Option<&str>) -> Vec<PersistenceBinding> {")
hunk(SHELL, '''        _ => folder.into_iter().collect(),
    }
}
''', '''        _ => folder.into_iter().collect(),
    }
}

/// 🏛️ The persistence transports the document host this shell build links actually serves. Both
/// builds link the kernel's `ArtifactHost`; its native actor owns the folder event log and the hub
/// WebSocket, while its browser actor has no filesystem and a hub `connect` with no transport yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellDocumentTransports {
    pub(crate) folder: bool,
    pub(crate) hub: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) const SHELL_DOCUMENT_TRANSPORTS: ShellDocumentTransports = ShellDocumentTransports { folder: true, hub: true };
#[cfg(target_arch = "wasm32")]
pub(crate) const SHELL_DOCUMENT_TRANSPORTS: ShellDocumentTransports = ShellDocumentTransports { folder: false, hub: false };

/// 🛂️ Refuses, before any actor exists, a binding the host would accept and then never serve: an
/// unserved folder or hub binding would hand the plugin a document that silently persists nowhere.
/// An empty binding set is the declared `EphemeralLocalOnly` class and every host serves it.
pub(crate) fn document_bindings_admitted(transports: ShellDocumentTransports, bindings: &[PersistenceBinding]) -> Result<(), &'static str> {
    for binding in bindings {
        match binding {
            PersistenceBinding::Hub { .. } if !transports.hub => return Err("document-binding.hub-transport-unavailable"),
            PersistenceBinding::Folder { .. } if !transports.folder => return Err("document-binding.folder-unavailable"),
            _ => {}
        }
    }
    Ok(())
}
''')
hunk(SHELL, '''/// 👥️ Projects only Hub-normalized peers for the shell's currently attached surface.
''' + NATIVE + "fn presence_peer_rows_for_surface(", '''/// 👥️ Projects only Hub-normalized peers for the shell's currently attached surface.
fn presence_peer_rows_for_surface(''')
hunk(SHELL, NATIVE + '''/// 🎯 Resolves the descriptor-bound canonical surface id this local selection may request. It is a''', '''/// 🎯 Resolves the descriptor-bound canonical surface id this local selection may request. It is a''')
ungate(SHELL, "fn wgpu_document_socket_surface(program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {")
ungate(SHELL, "fn bind_wgpu_document_socket_surface(host: &ArtifactHost, document_id: &str, artifact_schema: &str, bindings: &[PersistenceBinding], program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<(), String> {")

# ── Shell: sync channel ─────────────────────────────────────────────────────────────────────────
hunk(SHELL, '''/// @emoji 🧵️ One open document's live `framework/sync` actor channel held by the native wgpu shell.
/// Mirrors `os-shell.tsx`'s `openArtifactSessionsRef` entry: the shell owns the `cmd_tx`/event
/// receiver while the sandboxed plugin instance's store pumps through the registered
/// `ChannelBackbone` (see `framework/product/os/core/rs`'s `ArtifactHost` canonical sequence).
''' + NATIVE + "pub struct ShellSyncChannel {", '''/// @emoji 🧵️ One open document's live `framework/sync` actor channel held by the wgpu shell on both
/// targets. Mirrors `os-shell.tsx`'s `openArtifactSessionsRef` entry: the shell owns the `cmd_tx`/event
/// receiver while the sandboxed plugin instance's store pumps through the registered
/// `ChannelBackbone` (see `framework/product/os/core/rs`'s `ArtifactHost` canonical sequence).
pub struct ShellSyncChannel {''')
ungate(SHELL, "fn shell_sync_channel_owner(channel: &ShellSyncChannel) -> ShellSyncOwner {")
ungate(SHELL, "fn route_document_backbone_effects(actor_uri: &str, cmd_tx: &ArtifactMailboxSender, effects: Vec<semio_framework::kernel::Effect>) -> Result<Vec<semio_framework::kernel::Effect>, String> {")

# ── Shell: state fields ─────────────────────────────────────────────────────────────────────────
hunk(SHELL, '''    /// @emoji 🏛️ Shell-lifetime document-host actor registry (native only); the browser wgpu build
    /// has no native `ArtifactHost` — its sync flows through the React shell's `🏪️store/👷️worker/🟦️.ts`.
''' + "    " + NATIVE + '''    pub document_host: ArtifactHost,
    /// @emoji 🧵️ The currently attached document's live actor channel (native only).
''' + "    " + NATIVE + '''    pub sync_channel: Option<ShellSyncChannel>,
''' + "    " + NATIVE + '''    next_sync_binding_generation: u64,
''' + "    " + NATIVE + '''    sync_terminal_fault: Option<String>,
    /// @emoji 🚦️ Latest sync health for the active document's status badge (native only).
''' + "    " + NATIVE + '''    pub sync_status: Option<ArtifactSyncStatus>,''', '''    /// @emoji 🏛️ Shell-lifetime document-host actor registry, the kernel's `ArtifactHost` on both
    /// targets ([`SHELL_DOCUMENT_TRANSPORTS`] names which transports its actor serves).
    pub document_host: ArtifactHost,
    /// @emoji 🧵️ The currently attached document's live actor channel.
    pub sync_channel: Option<ShellSyncChannel>,
    next_sync_binding_generation: u64,
    sync_terminal_fault: Option<String>,
    /// @emoji 🚦️ Latest sync health for the active document's status badge.
    pub sync_status: Option<ArtifactSyncStatus>,''')
hunk(SHELL, "    " + NATIVE + "    pub sync_bootstrap_progress: Option<(u64, u64, u32, u32)>,", "    pub sync_bootstrap_progress: Option<(u64, u64, u32, u32)>,")
hunk(SHELL, "    " + NATIVE + "    pub presence_peers: Vec<PresencePeer>,", "    pub presence_peers: Vec<PresencePeer>,")
hunk(SHELL, "    " + NATIVE + "    pub presence_surface: Option<String>,", "    pub presence_surface: Option<String>,")

# ── Shell: construction ─────────────────────────────────────────────────────────────────────────
hunk(SHELL, '''            if let Some(credential) = claimed_local_hub_credential("native") {
                host.set_local_hub_credential(credential);
            }
            host
        };
''', '''            if let Some(credential) = claimed_local_hub_credential("native") {
                host.set_local_hub_credential(credential);
            }
            host
        };
        #[cfg(target_arch = "wasm32")]
        let document_host = ArtifactHost::new(std::sync::Arc::new(crate::renderer_worker_pool()));
''')
for field in ["document_host,", "sync_channel: None,", "next_sync_binding_generation: 1,", "sync_terminal_fault: None,", "sync_status: None,", "sync_bootstrap_progress: None,", "presence_peers: Vec::new(),", "presence_surface: None,"]:
    hunk(SHELL, "            " + NATIVE + "            " + field + "\n", "            " + field + "\n")

# ── Shell: effect funnel ────────────────────────────────────────────────────────────────────────
hunk(SHELL, "                " + NATIVE + "                semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload } => {", "                semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload } => {")

# ── Shell: sync lifecycle ───────────────────────────────────────────────────────────────────────
ungate(SHELL, "fn sync_document_id(&self) -> Option<String> {", "    ")
ungate(SHELL, "fn parse_persistence_binding(uri: &str) -> Result<Vec<PersistenceBinding>, String> {", "    ")
ungate(SHELL, "fn mint_sync_binding_generation(&mut self) -> Result<u64, String> {", "    ")
ungate(SHELL, "fn active_sync_owner(&self) -> Option<ShellSyncOwner> {", "    ")
ungate(SHELL, "async fn detach_sync_backbone_internal(&mut self) -> Result<(), String> {", "    ")
hunk(SHELL, '''    /// @emoji 📬️ Drains the active document actor's event stream into the plugin store and the sync
    /// badge. Called once per native frame — the render loop already redraws continuously (winit
    /// `ControlFlow::Poll`), so a `try_recv` poll suffices and no `EventLoopProxy` wake is needed.''', '''    /// @emoji 📬️ Drains the active document actor's event stream into the plugin store and the sync
    /// badge. Called once per frame-deferred pump on both targets — the render loop already redraws
    /// continuously, so a `try_recv` poll suffices and no `EventLoopProxy` wake is needed.''')
hunk(SHELL, "    " + NATIVE + '''    pub async fn pump_sync_events(&mut self) -> bool {
        use tokio::sync::broadcast::error::TryRecvError;
        let shell_io_changed = self.poll_shell_io();''', '''    pub async fn pump_sync_events(&mut self) -> bool {
        use tokio::sync::broadcast::error::TryRecvError;
        #[cfg(not(target_arch = "wasm32"))]
        let shell_io_changed = self.poll_shell_io();
        #[cfg(target_arch = "wasm32")]
        let shell_io_changed = false;''')
hunk(SHELL, '''        let administration_changed = self.pump_space_administration().await;
        self.poll_auto_checkin().await;
        self.flush_pending_directory_commands().await;''', '''        let administration_changed = self.pump_space_administration().await;
        self.flush_pending_directory_commands().await;''')
hunk(SHELL, '''    /// 👥️ The roster `#s-presence-peers` paints, scoped to the attached surface. The browser build
    /// has no document-sync backbone (`ArtifactHost` is native-only here), so it answers an empty
    /// roster — which is exactly the state React's own browser footer renders as
    /// `No one else is here`, not a reason to omit the pill.
    fn footer_presence_rows(&self) -> Vec<ui_wgpu::wgpu::PresencePeerRow> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match self.presence_surface.as_deref() {
                Some(surface) => presence_peer_rows_for_surface(&self.presence_peers, Some(surface), surface),
                None => Vec::new(),
            }
        }
        #[cfg(target_arch = "wasm32")]
        Vec::new()
    }''', '''    /// 👥️ The roster `#s-presence-peers` paints, scoped to the attached surface. A document opened
    /// without a hub binding carries no surface and answers an empty roster — exactly the state
    /// React's footer renders as `No one else is here`, not a reason to omit the pill.
    fn footer_presence_rows(&self) -> Vec<ui_wgpu::wgpu::PresencePeerRow> {
        match self.presence_surface.as_deref() {
            Some(surface) => presence_peer_rows_for_surface(&self.presence_peers, Some(surface), surface),
            None => Vec::new(),
        }
    }''')
hunk(SHELL, '''    /// Target-neutral because React's footer is: its browser shell paints `Remote: detached` from
    /// `computeSyncPillState(null)` with no backbone attached at all. The wgpu browser build has no
    /// `ArtifactHost` either (`store_sync` is native-only here), so it resolves to exactly the same
    /// state instead of — as before this packet — painting no pill at all.
    fn sync_pill(&self) -> ShellSyncPill {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some((received_bytes, total_bytes, received_chunks, total_chunks)) = self.sync_bootstrap_progress {
                return ShellSyncPill::Recovering { received_bytes, total_bytes, received_chunks, total_chunks };
            }
            let Some(status) = self.sync_status.as_ref() else { return ShellSyncPill::Remote(ShellSyncRemote::Detached) };
            if !matches!(status.remote, RemoteState::Live { .. }) {
                return ShellSyncPill::Remote(match &status.remote {
                    RemoteState::Live { .. } => ShellSyncRemote::Connected,
                    RemoteState::Connecting => ShellSyncRemote::Connecting,
                    RemoteState::Backoff { .. } => ShellSyncRemote::Backoff,
                    RemoteState::Detached => ShellSyncRemote::Detached,
                });
            }
            if status.pending_mutations > 0 {
                return ShellSyncPill::Pending(status.pending_mutations);
            }
            ShellSyncPill::Persisted
        }
        #[cfg(target_arch = "wasm32")]
        ShellSyncPill::Remote(ShellSyncRemote::Detached)
    }''', '''    /// Target-neutral because React's footer is: its browser shell paints `Remote: detached` from
    /// `computeSyncPillState(null)` with no backbone attached at all, and a document the browser
    /// host opened local-only never reports a remote, so it resolves to that same state.
    fn sync_pill(&self) -> ShellSyncPill {
        if let Some((received_bytes, total_bytes, received_chunks, total_chunks)) = self.sync_bootstrap_progress {
            return ShellSyncPill::Recovering { received_bytes, total_bytes, received_chunks, total_chunks };
        }
        let Some(status) = self.sync_status.as_ref() else { return ShellSyncPill::Remote(ShellSyncRemote::Detached) };
        if !matches!(status.remote, RemoteState::Live { .. }) {
            return ShellSyncPill::Remote(match &status.remote {
                RemoteState::Live { .. } => ShellSyncRemote::Connected,
                RemoteState::Connecting => ShellSyncRemote::Connecting,
                RemoteState::Backoff { .. } => ShellSyncRemote::Backoff,
                RemoteState::Detached => ShellSyncRemote::Detached,
            });
        }
        if status.pending_mutations > 0 {
            return ShellSyncPill::Pending(status.pending_mutations);
        }
        ShellSyncPill::Persisted
    }''')
ungate(SHELL, "fn current_shell_actor(&self, instance_id: u32) -> String {", "    ")
hunk(SHELL, "    " + NATIVE + '''    async fn refresh_history_snapshot(&mut self) {
        self.history_cursor = 0;
        self.history_entries.clear();
        self.history_current_checkpoint_id = None;
        self.last_uncommitted_edit_at_ms = None;
        self.auto_checkin_pending = false;
        self.checkpoint_dispatched = false;
        let Some(session) = self.session.as_ref() else { return };''', '''    async fn refresh_history_snapshot(&mut self) {
        self.history_cursor = 0;
        self.history_entries.clear();
        self.history_current_checkpoint_id = None;
        self.last_uncommitted_edit_at_ms = None;
        self.auto_checkin_pending = false;
        self.checkpoint_dispatched = false;
        self.seed_history_snapshot().await;
    }

    /// 🧾️ The seeding half of [`Self::refresh_history_snapshot`]: the `ReadHistory`/`ReadConflicts`
    /// exchange exists only on the native program bridge (the JS bridge has no `exchange` door), so
    /// the browser projection starts empty and folds every later `history_patch` exactly as native.
    #[cfg(not(target_arch = "wasm32"))]
    async fn seed_history_snapshot(&mut self) {
        let Some(session) = self.session.as_ref() else { return };''')
hunk(SHELL, '''        self.seed_open_conflicts().await;
    }

    /// 🧾️ ticket §C5 — folds an `InvocationResult.history_patch`''', '''        self.seed_open_conflicts().await;
    }

    #[cfg(target_arch = "wasm32")]
    async fn seed_history_snapshot(&mut self) {
        self.conflicts_seeded = false;
        self.open_conflicts.clear();
        self.selected_conflict_id = None;
    }

    /// 🧾️ ticket §C5 — folds an `InvocationResult.history_patch`''')
ungate(SHELL, "async fn checkpoint_before_detach(&mut self) {", "    ")

# ── Shell: the one attach body ──────────────────────────────────────────────────────────────────
hunk(SHELL, '''    /// @emoji 🔗️ Opens the shell's active app document on a `framework/sync` `ArtifactHost` actor and
    /// wires the sandboxed plugin store to it, following `framework/product/os/core/rs`'s
    /// `ArtifactHost` canonical sequence (open → subscribe → register host channel → program
    /// `attach-backbone`). The React shell's `openDocument` is the TS twin of this exact sequence.
    async fn attach_sync_backbone(&mut self, uri: String) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let plugin = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
            // 🎠️ H3-wgpu-native — `wasm_runtime()`/`register_host_backbone` retired, see
            // `detach_sync_backbone_internal`'s note; `attach_backbone` below now carries the honest
            // "not implemented yet" error for the whole mechanism.
            let document_id = self.sync_document_id().unwrap_or_else(|| "document".into());
            let schema = session.app.io.artifact_schema.clone();
            let bindings = Self::parse_persistence_binding(&uri)?;
            let window_id = self.active_window_id.as_deref().or(session.view_state.window_id.as_deref()).unwrap_or_else(|| session.app.window_kinds.first().id.as_str());
            let window_kind_id = self.live_window_kind_id(&session, window_id).unwrap_or_else(|| session.app.window_kinds.first().id.as_str()).to_string();
            // 📌️ ticket §C5 item 4 — checkpoint-on-close: this shell keeps exactly one session/document
            // mounted at a time, so "attach a different backbone" IS "close" for whatever was open —
            // same posture the React shell's own report documents ("switch away IS close here").
            self.checkpoint_before_detach().await;
            self.detach_sync_backbone_internal().await?;
            // 🔗️ The manual `remote://` sync-card override never carries a surface (§C6's
            // auto-binding is what threads one through — see `open_document` below), so the presence
            // roster stays empty for this path, same as before this lane.
            self.presence_surface = None;
            let actor_uri = format!("actor://{document_id}");
            let actor = self.current_shell_actor(session.instance_id);
            bind_wgpu_document_socket_surface(&self.document_host, &document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id)?;
            let channels = self.document_host.open(ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;
            let events = self.document_host.subscribe_key(&channels.document_key).await;
            let binding_generation = self.mint_sync_binding_generation()?;
            let binding_effects = match plugin.bind_document_backbone(session.instance_id, binding_generation, &actor_uri).await {
                Ok(effects) => effects,
                Err(error) => {
                    let _ = plugin.retire_document_backbone(session.instance_id, binding_generation, &actor_uri).await;
                    let _ = channels.cmd_tx.send(ArtifactActorMsg::Detach);
                    self.document_host.close_key(&channels.document_key);
                    return Err(format!("plugin document-backbone bind: {error}"));
                }
            };
            let cmd_tx = channels.cmd_tx.clone();
            let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
            self.sync_channel = Some(ShellSyncChannel {
                document_id,
                document_key: channels.document_key,
                actor_uri,
                instance_id: session.instance_id,
                plugin_id: session.plugin_id.clone(),
                binding_generation,
                cmd_tx,
                events,
                connected_at_ms: chrome_now_ms() as i64,
            });
            let channel = self.sync_channel.as_ref().expect("new sync channel is installed");
            let remaining = match route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, binding_effects) {
                Ok(remaining) => remaining,
                Err(error) => {
                    let _ = self.detach_sync_backbone_internal().await;
                    return Err(error);
                }
            };
            self.queue_host_effects(&session.app.controller_id, remaining);
            if let Some(error) = self.sync_terminal_fault.take() {
                let _ = self.detach_sync_backbone_internal().await;
                return Err(error);
            }
            self.sync_status = Some(ArtifactSyncStatus::default());
            if let Some(document_key) = self.sync_channel.as_ref().map(|channel| shell_hub_document_key(&channel.document_key)) {
                self.publish_hub_document_status(document_key, ShellHubRemoteV1::Detached);
            }
            self.sync_backbone_uri = Some(uri);
            self.sync_card_kind = None;
            Self::debug_log(&format!("[DEBUG] wgpu shell attached backbone {}", self.sync_backbone_uri.as_deref().unwrap_or_default()));
            self.refresh_history_snapshot().await;
            self.refresh_ui(UiDirtyScope::Full).await?;
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &session;
            self.sync_backbone_uri = Some(uri);
            self.sync_card_kind = None;
            web_sys::console::log_1(&"[DEBUG] attached backbone (browser wgpu: relayed via host-shim)".into());
            Ok(())
        }
    }''', '''    /// @emoji 🔗️ The sync card's manual attach: parses the card's uri into bindings and opens the
    /// session's own document through the ONE [`Self::open_document`] body, so the card and the
    /// `os.open-artifact` relay cannot drift into two attach sequences (the React shell's
    /// `openDocument` is the TS twin of that body).
    async fn attach_sync_backbone(&mut self, uri: String) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;
        let document_id = self.sync_document_id().ok_or("session missing")?;
        let bindings = Self::parse_persistence_binding(&uri)?;
        self.open_document(document_id, session.app.io.artifact_schema.clone(), bindings, None, Some(uri)).await
    }''')
hunk(SHELL, '''    /// override path. Used by the `os.open-artifact{documentId}` opening relay (§4) and by the
    /// identity-driven space-index auto-bind on the `/spaces/{id}` route (§6).
''' + "    " + NATIVE + '''    async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;''', '''    /// override path. Used by the `os.open-artifact{documentId}` opening relay (§4), by the sync
    /// card's manual attach (`backbone_uri` names what the card shows) and by the identity-driven
    /// space-index auto-bind on the `/spaces/{id}` route (§6). One body on both targets: the only
    /// platform split is [`document_bindings_admitted`], refused before any actor exists.
    async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<(), String> {
        document_bindings_admitted(SHELL_DOCUMENT_TRANSPORTS, &bindings)?;
        let session = self.session.clone().ok_or("session missing")?;''')
hunk(SHELL, '''        let cmd_tx = channels.cmd_tx.clone();
        let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        self.sync_backbone_uri = Some(actor_uri.clone());
        self.sync_channel =
            Some(ShellSyncChannel {''', '''        let cmd_tx = channels.cmd_tx.clone();
        let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        self.sync_backbone_uri = Some(backbone_uri.unwrap_or_else(|| actor_uri.clone()));
        self.sync_channel =
            Some(ShellSyncChannel {''')
ungate(SHELL, "fn default_bindings_for_current_session(&self) -> (Vec<PersistenceBinding>, Option<String>) {", "    ")
hunk(SHELL, "return self.open_document(S_SPACE_INDEX_DOCUMENT_ID.to_string(), S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string(), bindings, surface).await;", "return self.open_document(S_SPACE_INDEX_DOCUMENT_ID.to_string(), S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string(), bindings, surface, None).await;")
hunk(SHELL, '''                #[cfg(not(target_arch = "wasm32"))]
                self.checkpoint_before_detach().await;
                #[cfg(not(target_arch = "wasm32"))]
                self.detach_sync_backbone_internal().await?;
                self.sync_backbone_uri = None;''', '''                self.checkpoint_before_detach().await;
                self.detach_sync_backbone_internal().await?;
                self.sync_backbone_uri = None;''')

# ── Shell: the relay's document half is one arm ─────────────────────────────────────────────────
hunk(SHELL, '''    /// 🌐️ Both targets. The relay has two halves and only the second one is native: resolving the
    /// owner of an artifact kind, installing it on demand and switching the session to its app is
    /// pure catalog + program-bridge work that the browser build already links, so a browser wgpu
    /// shell opens a foreign-kind artifact exactly like the native one. Binding that opened session
    /// to a hub document is the half that needs the `ArtifactHost` backbone
    /// (`🏪️store/🔄️sync`), which the browser build does not link — it refuses out loud there
    /// (`open-artifact.browser-document`) rather than dropping the caller's `documentId` silently.''', '''    /// 🌐️ One body on both targets: resolving the owner of an artifact kind, installing it on demand
    /// (O2's `install_plugin`, whose progress + cancel band the chrome paints), switching the
    /// session to its app and binding that session to its document on the shell's `ArtifactHost`.
    /// A document the host cannot serve is refused out loud (`open-artifact.document-failed`), never
    /// dropped: the caller's `documentId` either binds or explains itself.''')
hunk(SHELL, '''        let (Some(document_id), Some(schema)) = (target.document_id, target.schema) else {
            return;
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (bindings, surface) = self.default_bindings_for_current_session();
            if let Err(error) = self.open_document(document_id, schema, bindings, surface).await {
                Self::debug_log(&format!("[DEBUG] wgpu shell os.open-artifact relay failed: {error}"));
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (document_id, schema);
            let is_de = self.locale_id == "de";
            self.show_transient_notice(shell_chrome_string("open-artifact.browser-document", is_de), semio_framework::Severity::Warning, Some("open-artifact.browser-document"));
        }
    }''', '''        let (Some(document_id), Some(schema)) = (target.document_id, target.schema) else {
            return;
        };
        let (bindings, surface) = self.default_bindings_for_current_session();
        if let Err(error) = self.open_document(document_id, schema, bindings, surface, None).await {
            Self::debug_log(&format!("[DEBUG] wgpu shell os.open-artifact relay failed: {error}"));
            let is_de = self.locale_id == "de";
            self.show_transient_notice(shell_chrome_string("open-artifact.document-failed", is_de), semio_framework::Severity::Warning, Some("open-artifact.document-failed"));
        }
    }''')

# ── Shell: presence heartbeat step ──────────────────────────────────────────────────────────────
hunk(SHELL, '''    /// 💓️ Coalesces one bounded presence preview page for the shared I/O lane. A shell with no sync
    /// channel has no peer to heartbeat to, and the browser has no sync backbone at all, so the step
    /// there would only clear its own flag.
    fn request_presence_preview(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if self.sync_channel.is_some() {''', '''    /// 💓️ Coalesces one bounded presence preview page for the shared I/O lane. A shell with no sync
    /// channel has no peer to heartbeat to, so the step there would only clear its own flag.
    fn request_presence_preview(&mut self) {
        if self.sync_channel.is_some() {''')
hunk(SHELL, "    " + NATIVE + "    fn advance_presence_preview_step(&mut self) {\n        self.chrome_present.maintenance.presence_requested = false;\n        let Some(channel)", "    fn advance_presence_preview_step(&mut self) {\n        self.chrome_present.maintenance.presence_requested = false;\n        let Some(channel)")
hunk(SHELL, '''    }

    #[cfg(target_arch = "wasm32")]
    fn advance_presence_preview_step(&mut self) {
        self.chrome_present.maintenance.presence_requested = false;
    }

    fn advance_chrome_preferences_persist_step(&mut self) {''', '''    }

    fn advance_chrome_preferences_persist_step(&mut self) {''')

# ── Shell: hub projection helpers ───────────────────────────────────────────────────────────────
ungate(SHELL, "fn shell_hub_document_key(document_key: &ArtifactDocumentKey) -> String {")
ungate(SHELL, "fn shell_hub_remote(remote: &RemoteState) -> ShellHubRemoteV1 {")

# ── Shell: localized refusal ────────────────────────────────────────────────────────────────────
hunk(SHELL, '''        ("open-artifact.browser-document", false) => "Opened the app, but document sync is unavailable in this browser build",
        ("open-artifact.browser-document", true) => "App geöffnet, aber Dokumentsynchronisierung ist in diesem Browser-Build nicht verfügbar",''', '''        ("open-artifact.document-failed", false) => "Opened the app, but its document could not be attached here",
        ("open-artifact.document-failed", true) => "App geöffnet, aber das Dokument konnte hier nicht angehängt werden",''')

# ── Shell: law mount ────────────────────────────────────────────────────────────────────────────
hunk(SHELL, '''#[cfg(test)]
#[path = "../../🧪️tests/🎬️wgpu-plugin-install/🦀️.rs"]
mod plugin_install_tests;
''', '''#[cfg(test)]
#[path = "../../🧪️tests/🎬️wgpu-plugin-install/🦀️.rs"]
mod plugin_install_tests;

#[cfg(test)]
#[path = "../../🧪️tests/📂️wgpu-document-relay/🦀️.rs"]
mod document_relay_tests;
''')

# ── Laws: the relay's refusal key is target-neutral now (`📂️wgpu-document-relay` owns it) ─────
hunk(INSTALL_LAWS, '''    assert_eq!(target.artifact_ref, "s.cad.cad@1/*", "the surface suffix is normalized off the stored coordinate");
}

/// 🧪️ The browser's document half refuses out loud: the relay's wasm32 arm reads this key, and an
/// untranslated one would paint the key itself into the banner.
#[test]
fn the_browser_document_refusal_is_localized() {
    let english = shell_chrome_string("open-artifact.browser-document", false);
    let german = shell_chrome_string("open-artifact.browser-document", true);
    assert!(english.contains("document sync"), "{english}");
    assert!(german.contains("Dokumentsynchronisierung"), "{german}");
    assert_ne!(english, german);
}''', '''    assert_eq!(target.artifact_ref, "s.cad.cad@1/*", "the surface suffix is normalized off the stored coordinate");
}''')

# ── Renderer: the kernel aliases the browser halves name ───────────────────────────────────────
hunk(RENDERER, NATIVE + "extern crate semio_framework_os_kernel as protocol;\n", "extern crate semio_framework_os_kernel as protocol;\n", "bridge")
hunk(RENDERER, NATIVE + "extern crate semio_framework_os_kernel as store_sync;\n", "extern crate semio_framework_os_kernel as store_sync;\n")

# ── Renderer: one pump on both targets ──────────────────────────────────────────────────────────
hunk(RENDERER, '''                FrameDeferredWork::PumpSync => {
                    #[cfg(not(target_arch = "wasm32"))]
                    interaction.shell.pump_sync_events().await;
                    // 📇️ The browser has no native document-sync backbone, but it DOES have the
                    // directory lane (identity → Space Administration → command FIFO) since ticket
                    // 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1e — same cadence, same code.
                    #[cfg(target_arch = "wasm32")]
                    interaction.shell.pump_directory_events().await;
                }''', '''                FrameDeferredWork::PumpSync => {
                    interaction.shell.pump_sync_events().await;
                }''')

# ── Program bridge: the document door on the JS backend ─────────────────────────────────────────
for name, sig, native_call, js_call, err in [
    ("bind_document_backbone", "(&self, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String>", "wasm_program_exchange::bind_document_backbone(client, instance_id, binding_generation, uri).await", 'document_backbone_js(handle, instance_id, "bind", binding_generation, uri).await', "bind_document_backbone unavailable"),
    ("retire_document_backbone", "(&self, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String>", "wasm_program_exchange::retire_document_backbone(client, instance_id, binding_generation, uri).await", 'document_backbone_js(handle, instance_id, "retire", binding_generation, uri).await', "retire_document_backbone unavailable"),
    ("receive_document_backbone", "(&self, instance_id: u32, uri: &str, payload: Vec<u8>) -> Result<Vec<Effect>, String>", "wasm_program_exchange::receive_document_backbone(client, instance_id, uri, payload).await", "receive_document_backbone_js(handle, instance_id, uri, &payload).await", "receive_document_backbone unavailable"),
    ("apply_mutations", "(&self, instance_id: u32, operations: &[u8]) -> Result<(), String>", "wasm_program_exchange::apply_mutations(client, instance_id, operations).await", 'call_js_bytes(handle, "applyMutations", instance_id, &[operations]).await', "apply_mutations unavailable"),
]:
    hunk(BRIDGE, "    " + NATIVE + f'''    pub async fn {name}{sig} {{
        match &self.backend {{
            ProgramBridgeBackend::Wasm {{ client, .. }} => {native_call},
            #[cfg(target_arch = "wasm32")]
            _ => Err("{err}".into()),
        }}
    }}''', f'''    pub async fn {name}{sig} {{
        match &self.backend {{
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm {{ client, .. }} => {native_call},
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {js_call},
        }}
    }}''')
hunk(BRIDGE, "    " + NATIVE + '''    pub async fn load_app_document_archive(&self, instance_id: u32, archive: &protocol::DocumentArchivePack) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::load_app_document_archive(client, instance_id, archive).await,
        }
    }''', '''    pub async fn load_app_document_archive(&self, instance_id: u32, archive: &protocol::DocumentArchivePack) -> Result<(), String> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::load_app_document_archive(client, instance_id, archive).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let bytes = protocol::encode_document_archive_bytes(archive).map_err(|error| error.to_string())?;
                call_js_bytes(handle, "loadAppDocumentArchive", instance_id, &[bytes.as_slice()]).await
            }
        }
    }''')
hunk(BRIDGE, '''#[cfg(target_arch = "wasm32")]
async fn create_app_js(handle: &Rc<JsValue>, app_id: &str) -> Result<u32, String> {''', '''#[cfg(target_arch = "wasm32")]
/// ⏳️ Calls one bridge function and settles its promise, keeping the rejection's own reason.
async fn call_js(handle: &Rc<JsValue>, name: &str, args: &Array) -> Result<JsValue, String> {
    let function = get_fn(handle.as_ref(), name)?;
    let result = function.apply(&JsValue::NULL, args).map_err(|error| format!("{name} failed: {}", describe_js_rejection(&error)))?;
    match result.dyn_ref::<js_sys::Promise>() {
        Some(promise) => JsFuture::from(promise.clone()).await.map_err(|error| format!("{name} promise failed: {}", describe_js_rejection(&error))),
        None => Ok(result),
    }
}

#[cfg(target_arch = "wasm32")]
/// 📦️ Calls a bridge function whose arguments are one instance id and byte payloads.
async fn call_js_bytes(handle: &Rc<JsValue>, name: &str, instance_id: u32, payloads: &[&[u8]]) -> Result<(), String> {
    let args = Array::new();
    args.push(&JsValue::from_f64(f64::from(instance_id)));
    for payload in payloads {
        args.push(&js_sys::Uint8Array::from(*payload));
    }
    call_js(handle, name, &args).await.map(|_| ())
}

#[cfg(target_arch = "wasm32")]
/// 📡️ The host effects a document-backbone door call left for the shell, read from the SAME
/// `InvocationResponse` JSON projection every other bridge verb answers with — so a guest's
/// `SendMessage { Backbone }` reaches `route_document_backbone_effects` exactly as it does natively.
fn document_backbone_effects(name: &str, answer: &JsValue) -> Result<Vec<Effect>, String> {
    let text = answer.as_string().ok_or_else(|| format!("{name} result not string"))?;
    dsl::os_pack::json::from_json_str::<semio_framework::kernel::InvocationResult>(&text).map(|result| result.requested_effects).map_err(|error| format!("{name} result parse failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
/// 📡️ Binds or retires one instance's document backbone — `bindingGeneration` crosses as a decimal
/// string because a JS `number` cannot carry a `u64` exactly.
async fn document_backbone_js(handle: &Rc<JsValue>, instance_id: u32, operation: &str, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String> {
    let args = Array::new();
    args.push(&JsValue::from_f64(f64::from(instance_id)));
    args.push(&JsValue::from_str(operation));
    args.push(&JsValue::from_str(&binding_generation.to_string()));
    args.push(&JsValue::from_str(uri));
    document_backbone_effects("documentBackbone", &call_js(handle, "documentBackbone", &args).await?)
}

#[cfg(target_arch = "wasm32")]
/// 📥️ Delivers one hot backbone message to the instance and answers the effects it left.
async fn receive_document_backbone_js(handle: &Rc<JsValue>, instance_id: u32, uri: &str, payload: &[u8]) -> Result<Vec<Effect>, String> {
    let args = Array::new();
    args.push(&JsValue::from_f64(f64::from(instance_id)));
    args.push(&JsValue::from_str(uri));
    args.push(&js_sys::Uint8Array::from(payload));
    document_backbone_effects("receiveDocumentBackbone", &call_js(handle, "receiveDocumentBackbone", &args).await?)
}

#[cfg(target_arch = "wasm32")]
async fn create_app_js(handle: &Rc<JsValue>, app_id: &str) -> Result<u32, String> {''')
hunk(BRIDGE, '''    #[cfg(test)]
    include!("../../🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs");
}''', '''    #[cfg(test)]
    include!("../../🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs");
}

#[cfg(test)]
#[path = "../../🧪️tests/📡️wgpu-document-backbone-effect/🦀️.rs"]
mod document_backbone_effect_tests;''')


def run(mode, group=None):
    problems = []
    for rel, pairs in HUNKS.items():
        path = ROOT / rel
        text = path.read_text(encoding="utf-8")
        for index, (old, new, owner) in enumerate(pairs):
            if group and owner != group:
                continue
            source, target = (old, new) if mode == "apply" else (new, old)
            if mode == "status":
                problems.append(f"{rel.split('/')[-3]}#{index}: old={text.count(old)} new={text.count(new)}")
                continue
            settled = text.count(target) >= 1 and (text.count(source) == 0 or (source in target and text.count(source) == text.count(target)))
            if settled:
                continue
            if text.count(source) == 1:
                text = text.replace(source, target, 1)
            else:
                problems.append(f"{mode} {rel.split('/')[-3]}#{index}: source×{text.count(source)} target×{text.count(target)} :: {source[:90]!r}")
        if mode != "status":
            path.write_text(text, encoding="utf-8")
    print("\n".join(problems) if problems else f"{mode} {group or 'all'}: hunks settled")
    return 1 if problems and mode != "status" else 0


if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else "status", sys.argv[2] if len(sys.argv) > 2 else None))
