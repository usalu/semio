# 🧩️ Text blocks of the WP7 pre-edit sq1.py: the platform-neutral host core, the native and the browser host owners.
CORE = r'''/// 🎛️ The channels {@link ArtifactHost::open} hands back to a caller: attach `channel_backbone`
/// to your `ArtifactStore`, and send control messages (or wakes) on `cmd_tx`. `Runner` is the
/// platform's per-document runner ticket (`()` in the browser, which has no runner).
pub struct ArtifactChannelsOf<Runner> {
    pub cmd_tx: ArtifactMailboxSender,
    pub document_key: ArtifactDocumentKey,
    pub runner: Runner,
    /// 🔗️ The store-side backbone end. The caller owns store attachment:
    /// `store.attach_backbone(Backbones::Channel(channels.channel_backbone))`.
    pub channel_backbone: ChannelBackbone,
}

pub struct OpenDocument<Runner> {
    pub generation: u64,
    pub cancel: semio_framework_async::CancelToken,
    pub cmd_tx: ArtifactMailboxSender,
    pub events: broadcast::Sender<ArtifactEvent>,
    pub presence: PresenceHeartbeatProducer,
    pub runner: Runner,
}

pub struct ArtifactHostState<Runner> {
    pub documents: std::collections::HashMap<ArtifactDocumentKey, OpenDocument<Runner>>,
    pub document_execution_target_leases: std::collections::HashMap<ArtifactDocumentKey, crate::os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    pub document_seeds: std::collections::HashMap<ArtifactDocumentKey, crate::os_directory::CanonicalCheckpointPairV1>,
    pub closing: std::collections::HashMap<u64, Runner>,
    pub next_generation: u64,
    pub host_references: usize,
}

impl<Runner> ArtifactHostState<Runner> {
    pub fn new() -> Self {
        Self {
            documents: std::collections::HashMap::new(),
            document_execution_target_leases: std::collections::HashMap::new(),
            document_seeds: std::collections::HashMap::new(),
            closing: std::collections::HashMap::new(),
            next_generation: 1,
            host_references: 1,
        }
    }

    pub fn claim_generation(&mut self) -> u64 {
        let generation = self.next_generation;
        self.next_generation = self.next_generation.checked_add(1).expect("artifact actor generation exhausted");
        generation
    }
}

/// 🏛️ The platform-neutral half of `ArtifactHost`: the registry of open per-document actors, their
/// event fan-out, presence cadence and the late-bound hub binding. Each platform crate owns the
/// `ArtifactHost` that composes this core with the actor it spawns, so this core never names an actor.
pub struct ArtifactHostCore<Runner> {
    pub inner: std::sync::Arc<std::sync::Mutex<ArtifactHostState<Runner>>>,
    pub pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pub credential: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<crate::os_directory::client::LocalHubCredential>>>>,
    pub socket_grant_source: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<dyn crate::os_directory::client::HubSocketGrantSource>>>>,
    pub cancel: semio_framework_async::CancelToken,
}

impl<Runner> ArtifactHostCore<Runner> {
    /// 🧩️ Creates a core on the process WorkerPool; callers must inject the same pool
    /// used by their service and renderer runtimes.
    pub fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>) -> Self {
        Self {
            inner: std::sync::Arc::new(std::sync::Mutex::new(ArtifactHostState::new())),
            pool,
            credential: std::sync::Arc::new(std::sync::RwLock::new(None)),
            socket_grant_source: std::sync::Arc::new(std::sync::RwLock::new(None)),
            cancel: semio_framework_async::CancelToken::root_now(),
        }
    }

    pub fn state(&self) -> std::sync::MutexGuard<'_, ArtifactHostState<Runner>> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// 🔗️ Counts one more owner of the shared registry and returns its handle.
    pub fn retain(&self) -> Self {
        let mut state = self.state();
        state.host_references = state.host_references.checked_add(1).expect("artifact host reference capacity exhausted");
        drop(state);
        Self { inner: self.inner.clone(), pool: self.pool.clone(), credential: self.credential.clone(), socket_grant_source: self.socket_grant_source.clone(), cancel: self.cancel.clone() }
    }

    /// 🔓️ Releases one owner; `true` when it was the last.
    pub fn release(&self) -> bool {
        let mut state = self.state();
        state.host_references = state.host_references.checked_sub(1).expect("artifact host reference underflow");
        state.host_references == 0
    }

    pub fn set_local_hub_credential(&self, credential: std::sync::Arc<crate::os_directory::client::LocalHubCredential>) {
        *self.credential.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(credential);
    }

    pub fn set_hub_socket_grant_source(&self, source: std::sync::Arc<dyn crate::os_directory::client::HubSocketGrantSource>) {
        *self.socket_grant_source.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(source);
    }

    /// 🪪️ Binds one complete verified execution-target lease before this document actor is opened.
    pub fn set_document_execution_target_lease(&self, document_key: &ArtifactDocumentKey, surface: crate::os_directory::DocumentExecutionTargetLeaseFieldsV1) -> bool {
        if !matches!(document_key, ArtifactDocumentKey::Hub { .. }) {
            return false;
        }
        let mut state = self.state();
        if state.documents.contains_key(document_key) || state.document_execution_target_leases.contains_key(document_key) {
            return false;
        }
        state.document_execution_target_leases.insert(document_key.clone(), surface);
        true
    }

    /// 🪢️ Hands the next actor of a hub document the canonical checkpoint pair its shell verified and loaded into the
    /// guest ([`crate::os_directory::DirectoryClient::document_canonical_checkpoint_pair`]): the actor starts at that
    /// pair — its history known, its first `SocketHelloV1` naming the pair's baseline frontier, so the hub tails only
    /// what came after it. Consumed by the next open of the same key.
    pub fn set_document_seed(&self, document_key: &ArtifactDocumentKey, pair: crate::os_directory::CanonicalCheckpointPairV1) -> bool {
        let ArtifactDocumentKey::Hub { space_id, document_id } = document_key else {
            return false;
        };
        if pair.scope.space_id != *space_id || pair.scope.document_id != *document_id {
            return false;
        }
        let mut state = self.state();
        if state.documents.contains_key(document_key) || state.document_seeds.contains_key(document_key) {
            return false;
        }
        state.document_seeds.insert(document_key.clone(), pair);
        true
    }

    /// 🟢️ Whether a credential and a grant source are bound; a browser adds its socket dialer on top.
    pub fn hub_binding_present(&self) -> bool {
        self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() && self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    /// 🎫️ Takes the lease and the seed bound for the next open of `document_key`.
    pub fn take_open_inputs(&self, document_key: &ArtifactDocumentKey) -> (Option<crate::os_directory::DocumentExecutionTargetLeaseFieldsV1>, Option<crate::os_directory::CanonicalCheckpointPairV1>) {
        let mut state = self.state();
        (state.document_execution_target_leases.remove(document_key), state.document_seeds.remove(document_key))
    }

    pub fn insert_document(&self, document_key: ArtifactDocumentKey, document: OpenDocument<Runner>) {
        self.state().documents.insert(document_key, document);
    }

    pub fn remove_document(&self, document_key: &ArtifactDocumentKey) -> Option<OpenDocument<Runner>> {
        self.state().documents.remove(document_key)
    }

    /// 📬️ A fresh event receiver for `document_id`. If the document is not open the receiver's
    /// sender is dropped, so it simply reports closed.
    pub async fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        self.subscribe_key(&ArtifactDocumentKey::local(document_id)).await
    }

    pub async fn subscribe_key(&self, document_key: &ArtifactDocumentKey) -> broadcast::Receiver<ArtifactEvent> {
        let guard = self.state();
        match guard.documents.get(document_key) {
            Some(document) => document.events.subscribe(),
            None => {
                let (_tx, rx) = broadcast::channel(1);
                rx
            }
        }
    }

    /// 🔔️ Sends a control message to a document's actor (e.g. a presence heartbeat or a wake).
    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        self.send_key(&ArtifactDocumentKey::local(document_id), message).await;
    }

    pub async fn send_key(&self, document_key: &ArtifactDocumentKey, message: ArtifactActorMsg) {
        if let Some(document) = self.state().documents.get(document_key) {
            let _ = document.cmd_tx.send(message);
        }
    }

    /// 💓️ Offers a generic cursor/viewport/app-presence heartbeat for one open document.
    /// Returns `true` only when the host actually queued a publish; faster offers are coalesced onto
    /// the document's producer and cannot flood the preview lane.
    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        self.presence_heartbeat_key(&ArtifactDocumentKey::local(document_id), now_ms, peer)
    }

    /// ⏯️ Records an open document's latest tool run summary for its presence heartbeats — the host
    /// calls this with each `AppFrame::Ephemeral.tool_run` it decodes. `false` when the document is not open.
    pub fn observe_presence_tool_run_key(&self, document_key: &ArtifactDocumentKey, tool_run: Option<PresenceToolRun>) -> bool {
        let mut state = self.state();
        let Some(document) = state.documents.get_mut(document_key) else { return false };
        document.presence.observe_tool_run(tool_run);
        true
    }

    pub fn presence_heartbeat_key(&self, document_key: &ArtifactDocumentKey, now_ms: u64, peer: PresencePeer) -> bool {
        let mut state = self.state();
        let Some(document) = state.documents.get_mut(document_key) else { return false };
        let Some(peer) = document.presence.offer(now_ms, peer) else { return false };
        document.cmd_tx.send(ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(peer) }).is_ok()
    }

    /// 🧹️ Ids of every currently-open document.
    /// 🚫️async: E1 pure lock-and-collect consumed by `impl Drop` (sync-only external trait) — see
    /// R9. No I/O, no suspension point.
    pub fn open_artifacts(&self) -> Vec<ArtifactDocumentKey> {
        self.state().documents.keys().cloned().collect()
    }
}
'''

NATIVE_HOST = r'''
/// 🎛️ The channels of a native document actor, carrying its runner ticket.
pub type ArtifactChannels = ArtifactChannelsOf<ArtifactActorRunnerTicket>;

/// 🏛️ Registry of open per-document actors. One `ArtifactHost` per host process (wgpu native
/// or tests) owns every open document's actor + event fan-out.
pub struct ArtifactHost {
    core: ArtifactHostCore<ArtifactActorRunnerHandle>,
}

impl Clone for ArtifactHost {
    fn clone(&self) -> Self {
        Self { core: self.core.retain() }
    }
}

impl ArtifactHost {
    /// 🧩️ Creates a host on the process WorkerPool; callers must inject the same pool
    /// used by their service and renderer runtimes.
    pub fn new(pool: Arc<semio_framework_async::WorkerPool>) -> Self {
        Self { core: ArtifactHostCore::new(pool) }
    }

    pub fn set_local_hub_credential(&self, credential: Arc<crate::os_directory::client::LocalHubCredential>) {
        self.core.set_local_hub_credential(credential);
    }

    pub fn set_hub_socket_grant_source(&self, source: Arc<dyn crate::os_directory::client::HubSocketGrantSource>) {
        self.core.set_hub_socket_grant_source(source);
    }

    pub fn set_document_execution_target_lease(&self, document_key: &ArtifactDocumentKey, surface: crate::os_directory::DocumentExecutionTargetLeaseFieldsV1) -> bool {
        self.core.set_document_execution_target_lease(document_key, surface)
    }

    pub fn set_document_seed(&self, document_key: &ArtifactDocumentKey, pair: crate::os_directory::CanonicalCheckpointPairV1) -> bool {
        self.core.set_document_seed(document_key, pair)
    }

    /// 🟢️ Whether a hub document opened now can dial at once: a credential and a grant source.
    pub fn local_hub_ready(&self) -> bool {
        self.core.hub_binding_present()
    }

    /// 🚀️ Spawns (or replaces) the actor for `config.document_id` and returns the channels the
    /// caller wires into its store. Idempotent per id: opening an already-open id closes the old actor.
    pub async fn open(&self, config: ArtifactActorConfig) -> ArtifactChannels {
        let document_id = config.document_id.clone();
        let document_key = ArtifactDocumentKey::for_config(&config);
        let (document_execution_target_lease, seed) = self.core.take_open_inputs(&document_key);
        let _ = self.close_key(&document_key);
        let generation = self.core.state().claim_generation();
        let (channel_backbone, remote) = ChannelBackbone::pair(&format!("actor://{document_id}")).await;
        let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
        let (event_tx, _event_rx) = broadcast::channel(256);
        let document_cancel = self.core.cancel.child_now();
        let runner = spawn_actor(self.core.pool.clone(), generation, config, remote, cmd_rx, event_tx.clone(), self.core.credential.clone(), self.core.socket_grant_source.clone(), document_execution_target_lease, seed, document_cancel.clone()).await;
        let weak_host = Arc::downgrade(&self.core.inner);
        runner.set_terminal_empty_callback(Arc::new(move |generation| {
            if let Some(host) = weak_host.upgrade() {
                host.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing.remove(&generation);
            }
        }));
        let runner_ticket = runner.issue_ticket(self.core.inner.clone());
        let entry = OpenDocument {
            generation,
            cancel: document_cancel,
            cmd_tx: cmd_tx.clone(),
            events: event_tx,
            presence: PresenceHeartbeatProducer::default(),
            runner: runner.clone(),
        };
        self.core.insert_document(document_key.clone(), entry);
        runner.start();
        ArtifactChannelsOf { cmd_tx, document_key, runner: runner_ticket, channel_backbone }
    }

    pub async fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        self.core.subscribe(document_id).await
    }

    pub async fn subscribe_key(&self, document_key: &ArtifactDocumentKey) -> broadcast::Receiver<ArtifactEvent> {
        self.core.subscribe_key(document_key).await
    }

    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        self.core.send(document_id, message).await;
    }

    pub async fn send_key(&self, document_key: &ArtifactDocumentKey, message: ArtifactActorMsg) {
        self.core.send_key(document_key, message).await;
    }

    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        self.core.presence_heartbeat(document_id, now_ms, peer)
    }

    pub fn observe_presence_tool_run_key(&self, document_key: &ArtifactDocumentKey, tool_run: Option<PresenceToolRun>) -> bool {
        self.core.observe_presence_tool_run_key(document_key, tool_run)
    }

    pub fn presence_heartbeat_key(&self, document_key: &ArtifactDocumentKey, now_ms: u64, peer: PresencePeer) -> bool {
        self.core.presence_heartbeat_key(document_key, now_ms, peer)
    }

    /// ✂️ Transfers a document into generation-keyed retained close ownership. The runner
    /// rejects later mailbox ingress and drains one mailbox, backbone, actor, or job owner per grant.
    /// 🚫️async: E1 pure lock/remove + sync channel send, no real suspension point — consumed by
    /// `impl Drop` (sync-only external trait); see R9.
    pub fn close(&self, document_id: &str) -> Option<u64> {
        self.close_key(&ArtifactDocumentKey::local(document_id))
    }

    pub fn close_key(&self, document_key: &ArtifactDocumentKey) -> Option<u64> {
        let document = self.core.remove_document(document_key)?;
        document.cancel.cancel_now();
        let generation = document.generation;
        let runner = document.runner;
        self.core.state().closing.insert(generation, runner.clone());
        if matches!(runner.terminal_state(), ArtifactActorTerminalState::Complete(_)) {
            self.core.state().closing.remove(&generation);
            return Some(generation);
        }
        runner.request_close();
        Some(generation)
    }

    pub fn closing_runner(&self, generation: u64) -> Option<ArtifactActorRunnerHandle> {
        self.core.state().closing.get(&generation).cloned()
    }

    pub fn open_artifacts(&self) -> Vec<ArtifactDocumentKey> {
        self.core.open_artifacts()
    }
}

impl Drop for ArtifactHost {
    fn drop(&mut self) {
        if !self.core.release() {
            return;
        }
        self.core.cancel.cancel_now();
        for document_key in self.core.open_artifacts() {
            self.close_key(&document_key);
        }
    }
}
'''

WASM_HOST = r'''
/// 🎛️ The channels of a browser document actor, which has no runner.
pub type ArtifactChannels = ArtifactChannelsOf<()>;

/// 🏛️ Registry of open per-document actors of the browser wgpu build: one `ArtifactHost` per page
/// owns every open document's actor + event fan-out, and the page's document-socket dialer.
pub struct ArtifactHost {
    core: ArtifactHostCore<()>,
    document_socket_dialer: SharedDialer,
}

impl Clone for ArtifactHost {
    fn clone(&self) -> Self {
        Self { core: self.core.retain(), document_socket_dialer: self.document_socket_dialer.clone() }
    }
}

impl ArtifactHost {
    /// 🧩️ Creates a host on the process WorkerPool; callers must inject the same pool
    /// used by their service and renderer runtimes.
    pub fn new(pool: Arc<semio_framework_async::WorkerPool>) -> Self {
        Self { core: ArtifactHostCore::new(pool), document_socket_dialer: Arc::new(RwLock::new(None)) }
    }

    pub fn set_local_hub_credential(&self, credential: Arc<crate::os_directory::client::LocalHubCredential>) {
        self.core.set_local_hub_credential(credential);
    }

    pub fn set_hub_socket_grant_source(&self, source: Arc<dyn crate::os_directory::client::HubSocketGrantSource>) {
        self.core.set_hub_socket_grant_source(source);
    }

    pub fn set_document_execution_target_lease(&self, document_key: &ArtifactDocumentKey, surface: crate::os_directory::DocumentExecutionTargetLeaseFieldsV1) -> bool {
        self.core.set_document_execution_target_lease(document_key, surface)
    }

    pub fn set_document_seed(&self, document_key: &ArtifactDocumentKey, pair: crate::os_directory::CanonicalCheckpointPairV1) -> bool {
        self.core.set_document_seed(document_key, pair)
    }

    /// ☎️ Installs the browser's document-socket dialer; every later hub document actor dials through it.
    pub fn set_document_socket_dialer(&self, dialer: Arc<dyn DocumentSocketDialer>) {
        *self.document_socket_dialer.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(dialer);
    }

    /// 🟢️ Whether a hub document opened now can dial at once: a credential, a grant source and
    /// the socket dialer the page owns.
    pub fn local_hub_ready(&self) -> bool {
        self.core.hub_binding_present() && self.document_socket_dialer.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    /// 🚀️ Spawns (or replaces) the actor for `config.document_id` and returns the channels the
    /// caller wires into its store. Idempotent per id: opening an already-open id closes the old actor.
    pub async fn open(&self, config: ArtifactActorConfig) -> ArtifactChannels {
        let document_id = config.document_id.clone();
        let document_key = ArtifactDocumentKey::for_config(&config);
        let (document_execution_target_lease, seed) = self.core.take_open_inputs(&document_key);
        let _ = self.close_key(&document_key);
        let generation = self.core.state().claim_generation();
        let (channel_backbone, remote) = ChannelBackbone::pair(&format!("actor://{document_id}")).await;
        let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
        let (event_tx, _event_rx) = broadcast::channel(256);
        let document_cancel = self.core.cancel.child_now();
        spawn_actor(
            config,
            remote,
            cmd_rx,
            event_tx.clone(),
            WasmActorHub { credential: self.core.credential.clone(), socket_grant_source: self.core.socket_grant_source.clone(), dialer: self.document_socket_dialer.clone(), lease: document_execution_target_lease, seed, cancel: document_cancel.clone() },
        )
        .await;
        let entry = OpenDocument { generation, cancel: document_cancel, cmd_tx: cmd_tx.clone(), events: event_tx, presence: PresenceHeartbeatProducer::default(), runner: () };
        self.core.insert_document(document_key.clone(), entry);
        ArtifactChannelsOf { cmd_tx, document_key, runner: (), channel_backbone }
    }

    pub async fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        self.core.subscribe(document_id).await
    }

    pub async fn subscribe_key(&self, document_key: &ArtifactDocumentKey) -> broadcast::Receiver<ArtifactEvent> {
        self.core.subscribe_key(document_key).await
    }

    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        self.core.send(document_id, message).await;
    }

    pub async fn send_key(&self, document_key: &ArtifactDocumentKey, message: ArtifactActorMsg) {
        self.core.send_key(document_key, message).await;
    }

    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        self.core.presence_heartbeat(document_id, now_ms, peer)
    }

    pub fn observe_presence_tool_run_key(&self, document_key: &ArtifactDocumentKey, tool_run: Option<PresenceToolRun>) -> bool {
        self.core.observe_presence_tool_run_key(document_key, tool_run)
    }

    pub fn presence_heartbeat_key(&self, document_key: &ArtifactDocumentKey, now_ms: u64, peer: PresencePeer) -> bool {
        self.core.presence_heartbeat_key(document_key, now_ms, peer)
    }

    /// ✂️ Detaches a document's actor and drops its registry entry.
    /// 🚫️async: E1 pure lock/remove + sync channel send, no real suspension point — consumed by
    /// `impl Drop` (sync-only external trait); see R9.
    pub fn close(&self, document_id: &str) -> Option<u64> {
        self.close_key(&ArtifactDocumentKey::local(document_id))
    }

    pub fn close_key(&self, document_key: &ArtifactDocumentKey) -> Option<u64> {
        let document = self.core.remove_document(document_key)?;
        document.cancel.cancel_now();
        let _ = document.cmd_tx.send(ArtifactActorMsg::Detach);
        Some(document.generation)
    }

    pub fn open_artifacts(&self) -> Vec<ArtifactDocumentKey> {
        self.core.open_artifacts()
    }
}

impl Drop for ArtifactHost {
    fn drop(&mut self) {
        if !self.core.release() {
            return;
        }
        self.core.cancel.cancel_now();
        for document_key in self.core.open_artifacts() {
            self.close_key(&document_key);
        }
    }
}
'''
