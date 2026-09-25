import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


# A. types beside the plugin-install phase
replace(
    """#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellPluginInstallPhase {
    Resolving,
    Loading,
    Cancelled,
    Failed(String),
}
""",
    """#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellPluginInstallPhase {
    Resolving,
    Loading,
    Cancelled,
    Failed(String),
}

/// 🚪️ Where the one frame-pumped document open stands: the guest's app instance is being created,
/// the document's genesis is being loaded into it, or the open settled as cancelled or failed — a
/// settled record stays until the user closes its band, like a settled plugin install.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellDocumentOpenPhase {
    Instantiating,
    Seeding,
    Cancelled,
    Failed(String),
}

/// 📨️ What one detached open step answered.
enum ShellDocumentOpenAnswer {
    Instantiated(u32),
    Seeded,
}

/// 🎯️ The document half of an open: what the relay asked for.
struct ShellDocumentOpenTarget {
    document_id: String,
    schema: String,
}

/// 🧷️ An open whose host half is prepared — the prior document retired, the socket surface and
/// execution target bound — waiting for the guest to hold the document before its actor binds.
struct ShellPreparedDocumentOpen {
    document_id: String,
    schema: String,
    bindings: Vec<PersistenceBinding>,
    backbone_uri: Option<String>,
    hub_bound: bool,
    plugin: ProgramBridgeEntry,
    session: ActiveSession,
}

/// 🚪️ The one document open this shell runs at a time (`os.open-artifact`), advanced by the frame pump
/// ([`ShellState::advance_document_opening`]). Every guest turn it waits on — the app instance, the
/// genesis load — runs detached ([`ShellDetached`]), so the frame loop keeps building frames, painting
/// this open's band (phase, step, elapsed, cancel) and taking input while the guest works; before, the
/// whole open held the shell for ~17 s in a debug build (ticket 26/09/23 slice WG8).
pub struct ShellDocumentOpening {
    pub label: String,
    pub phase: ShellDocumentOpenPhase,
    pub cancel_requested: bool,
    pub started_at_ms: f64,
    plugin_id: String,
    app: AppDefinition,
    document: Option<ShellDocumentOpenTarget>,
    pending: Option<ShellDetached<Result<ShellDocumentOpenAnswer, String>>>,
    prepared: Option<ShellPreparedDocumentOpen>,
}

impl ShellDocumentOpening {
    /// ⏳️ Whether a step is still out; a settled record only waits for its band to be closed.
    pub fn running(&self) -> bool {
        matches!(self.phase, ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding)
    }
}
""",
)

# B. ShellDetached beside the pool future
replace(
    """#[cfg(not(target_arch = "wasm32"))]
/// 🔄️ A retained-waker future polled once per shared-pool turn; no worker waits for completion.
struct ShellPoolFuture {""",
    """//#region 🧵️ShellDetached
/// 🧵️ One request the shell hands off and never waits on: the future owns everything it touches and
/// runs on the shared pool (native) or the page's executor (browser), and the frame pump only asks,
/// once per frame, whether it answered — so the guest turn behind it never holds the shell, and with it
/// the frame build, checked out.
struct ShellDetached<T> {
    answer: std::sync::Arc<std::sync::Mutex<Option<T>>>,
}

impl<T: 'static> ShellDetached<T> {
    #[cfg(not(target_arch = "wasm32"))]
    fn spawn(request: impl std::future::Future<Output = T> + Send + 'static) -> Self
    where
        T: Send,
    {
        let answer = std::sync::Arc::new(std::sync::Mutex::new(None));
        let slot = answer.clone();
        crate::spawn_app_task(async move {
            let value = request.await;
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(value);
        });
        Self { answer }
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn(request: impl std::future::Future<Output = T> + 'static) -> Self {
        let answer = std::sync::Arc::new(std::sync::Mutex::new(None));
        let slot = answer.clone();
        crate::spawn_app_task(async move {
            let value = request.await;
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(value);
        });
        Self { answer }
    }

    /// 📬️ The answer, once, if it has arrived.
    fn take(&self) -> Option<T> {
        self.answer.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }
}

/// 🌱️ Loads the genesis the owning component mints for a hub document into the guest instance, off
/// the shell ([`store_sync::os_store::component_document_genesis`]); a document bound to no hub keeps
/// the guest's own document.
async fn seed_document_genesis(plugin: ProgramBridgeEntry, instance_id: u32, schema: String, document_id: String, hub_bound: bool) -> Result<(), String> {
    if !hub_bound {
        return Ok(());
    }
    match store_sync::os_store::component_document_genesis(&schema, &document_id).await.map_err(|error| format!("document genesis: {error}"))? {
        Some(genesis) => plugin.load_app_document_pack(instance_id, &genesis.pack, &genesis.spr).await.map_err(|error| format!("document genesis load: {error}")),
        None => Ok(()),
    }
}
//#endregion 🧵️ShellDetached

#[cfg(not(target_arch = "wasm32"))]
/// 🔄️ A retained-waker future polled once per shared-pool turn; no worker waits for completion.
struct ShellPoolFuture {""",
)

# C. field + init
replace(
    "    pub plugin_install: Option<ShellPluginInstall>,\n",
    """    pub plugin_install: Option<ShellPluginInstall>,
    /// 🚪️ The ONE document open in flight or settled, advanced by the frame pump
    /// ([`Self::advance_document_opening`]) and painted as its own band with a cancel control.
    pub document_opening: Option<ShellDocumentOpening>,
""",
)
replace("            plugin_install: None,\n", "            plugin_install: None,\n            document_opening: None,\n")

# D. switch_to_app split
replace(
    """        #[cfg(not(target_arch = "wasm32"))]
        if let (Some(home), Some(current)) = (self.directory_home.as_mut(), self.session.as_ref()) {
            if home.is_instance(&current.plugin_id, current.instance_id) {
                home.view_state = current.view_state.clone();
            }
        }
        let instance_id = program.create_app(&app.id).await?;
        let view_state = ViewModel {""",
    """        let instance_id = program.create_app(&app.id).await?;
        self.install_app_session(plugin_id, app, instance_id);
        self.refresh_ui(UiDirtyScope::Full).await
    }

    /// 🪪️ Mounts `instance_id` of `app` as this shell's session: the retained Home hands back its view
    /// state first, then the session and its first window become current. The synchronous tail of
    /// [`Self::switch_to_app`], shared with the frame-pumped open ([`Self::advance_document_opening`]),
    /// which renders on the settle lane instead of inline.
    fn install_app_session(&mut self, plugin_id: &str, app: AppDefinition, instance_id: u32) {
        #[cfg(not(target_arch = "wasm32"))]
        if let (Some(home), Some(current)) = (self.directory_home.as_mut(), self.session.as_ref()) {
            if home.is_instance(&current.plugin_id, current.instance_id) {
                home.view_state = current.view_state.clone();
            }
        }
        let view_state = ViewModel {""",
)
replace(
    """        self.active_window_id = Some(app.window_kinds.first().id.clone());
        self.session = Some(ActiveSession { plugin_id: plugin_id.to_string(), instance_id, app, view_state });
        self.refresh_ui(UiDirtyScope::Full).await
    }
""",
    """        self.active_window_id = Some(app.window_kinds.first().id.clone());
        self.session = Some(ActiveSession { plugin_id: plugin_id.to_string(), instance_id, app, view_state });
    }
""",
)

# E. open_document split
old_open_start = text.index("    async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<(), String> {")
old_open_end = text.index("    /// 🪪️ The kind identity of a hub document whose schema this process resolves no codec for", old_open_start)
new_open = '''    async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<(), String> {
        let prepared = self.prepare_document_open(document_id, schema, bindings, surface, backbone_uri).await?;
        seed_document_genesis(prepared.plugin.clone(), prepared.session.instance_id, prepared.schema.clone(), prepared.document_id.clone(), prepared.hub_bound).await?;
        self.finish_document_open(prepared).await?;
        self.refresh_ui(UiDirtyScope::Full).await
    }

    /// 🧷️ The host half of an open, before the guest holds the document: the mounted document is
    /// checkpointed and retired, the socket surface and the execution target bound.
    async fn prepare_document_open(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<ShellPreparedDocumentOpen, String> {
        document_bindings_admitted(SHELL_DOCUMENT_TRANSPORTS, &bindings)?;
        let session = self.session.clone().ok_or("session missing")?;
        let plugin = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
        let window_id = self.active_window_id.as_deref().or(session.view_state.window_id.as_deref()).unwrap_or_else(|| session.app.window_kinds.first().id.as_str());
        let window_kind_id = self.live_window_kind_id(&session, window_id).unwrap_or_else(|| session.app.window_kinds.first().id.as_str()).to_string();
        // 🎠️ H3-wgpu-native — `wasm_runtime()`/`register_host_backbone` retired, see
        // `detach_sync_backbone_internal`'s note.
        // 📌️ ticket §C5 item 4 — checkpoint-on-close, same "switch away IS close" posture as
        // `attach_sync_backbone` above (this shell keeps exactly one document mounted at a time).
        self.checkpoint_before_detach().await;
        self.detach_sync_backbone_internal().await?;
        self.presence_surface = surface;
        bind_wgpu_document_socket_surface(&self.document_host, &document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id)?;
        self.bind_document_execution_target(&document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id).await?;
        let hub_bound = bindings.iter().any(|binding| matches!(binding, PersistenceBinding::Hub { .. }));
        Ok(ShellPreparedDocumentOpen { document_id, schema, bindings, backbone_uri, hub_bound, plugin, session })
    }

    /// 🔗️ Binds the prepared document once the guest holds it: the document actor opens, the guest's
    /// backbone binds to it and the history projection re-seeds. The caller owes the refresh.
    async fn finish_document_open(&mut self, prepared: ShellPreparedDocumentOpen) -> Result<(), String> {
        let ShellPreparedDocumentOpen { document_id, schema, bindings, backbone_uri, plugin, session, .. } = prepared;
        let actor_uri = format!("actor://{document_id}");
        let actor = self.current_shell_actor(session.instance_id);
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
        self.sync_backbone_uri = Some(backbone_uri.unwrap_or_else(|| actor_uri.clone()));
        self.sync_channel =
            Some(ShellSyncChannel { document_id, document_key: channels.document_key, actor_uri, instance_id: session.instance_id, plugin_id: session.plugin_id.clone(), binding_generation, cmd_tx, events, connected_at_ms: chrome_now_ms() as i64 });
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
        self.sync_card_kind = None;
        self.refresh_history_snapshot().await;
        Ok(())
    }

'''
text = text[:old_open_start] + new_open + text[old_open_end:]
path.write_text(text)
print("ok")
