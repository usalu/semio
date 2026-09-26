import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace("""/// 🚪️ Where the one frame-pumped document open stands: the guest's app instance is being created,
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
""", """/// 🚪️ Where the one frame-pumped document open stands: a hub document's component is being resolved by
/// the serving catalog generation, the guest's app instance is being created, the document's genesis is
/// being loaded into it, or the open settled as cancelled or failed — a settled record stays until the
/// user closes its band, like a settled plugin install.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellDocumentOpenPhase {
    Resolving,
    Instantiating,
    Seeding,
    Cancelled,
    Failed(String),
}

/// 📨️ What one detached open step answered: a hub document's resolved program (`None` when the local one
/// already is the lease's component), an app instance, or a seeded document.
enum ShellDocumentOpenAnswer {
    #[cfg(not(target_arch = "wasm32"))]
    Resolved(Option<ProgramBridgeEntry>),
    Instantiated(u32),
    Seeded,
}

/// ⏳️ The resolution step a hub document open is on, as its band reads it ([`ShellDocumentOpening::resolve_step`]).
#[cfg(not(target_arch = "wasm32"))]
fn document_open_resolve_step_code(step: semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStep) -> u8 {
    use semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStep;
    match step {
        ExecutionTargetModuleStep::Lease => 0,
        ExecutionTargetModuleStep::Component => 1,
        ExecutionTargetModuleStep::Descriptor => 2,
        ExecutionTargetModuleStep::Verified => 3,
    }
}
""")
replace("""pub struct ShellDocumentOpening {
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
}""", """pub struct ShellDocumentOpening {
    pub label: String,
    pub phase: ShellDocumentOpenPhase,
    pub cancel_requested: bool,
    pub started_at_ms: f64,
    plugin_id: String,
    app_id: String,
    document: Option<ShellDocumentOpenTarget>,
    pending: Option<ShellDetached<Result<ShellDocumentOpenAnswer, String>>>,
    prepared: Option<ShellPreparedDocumentOpen>,
    /// 🛑️ The token the detached hub resolution's requests check; cancelling the open cancels it.
    cancel: Option<CancelToken>,
    /// ⏳️ The resolution step the detached hub resolution last reported (lease, component, descriptor, verified).
    resolve_step: std::sync::Arc<std::sync::atomic::AtomicU8>,
}

impl ShellDocumentOpening {
    /// 🚪️ An open whose first detached step is `pending`, in `phase`.
    fn new(label: String, phase: ShellDocumentOpenPhase, plugin_id: String, app_id: String, document: Option<ShellDocumentOpenTarget>, pending: Option<ShellDetached<Result<ShellDocumentOpenAnswer, String>>>) -> Self {
        Self { label, phase, cancel_requested: false, started_at_ms: chrome_now_ms(), plugin_id, app_id, document, pending, prepared: None, cancel: None, resolve_step: std::sync::Arc::default() }
    }

    /// ⏳️ Whether a step is still out; a settled record only waits for its band to be closed.
    pub fn running(&self) -> bool {
        matches!(self.phase, ShellDocumentOpenPhase::Resolving | ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding)
    }
}""")

# relay: hub-resolved path before switch
replace("""        if let Some(space_id) = target.space_id {
            self.open_space_id = Some(space_id);
        }
        // 🎬️ The relay used to open every artifact inside whatever session happened to be mounted, so""", """        if let Some(space_id) = target.space_id.clone() {
            self.open_space_id = Some(space_id);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.begin_document_resolution(&target) {
            return;
        }
        // 🎬️ The relay used to open every artifact inside whatever session happened to be mounted, so""")
replace("""                let label = app.label.resolve(self.active_terminology(), self.active_locale()).to_string();
                let app_id = app.id.clone();
                let pending = ShellDetached::spawn(async move { program.create_app(&app_id).await.map(ShellDocumentOpenAnswer::Instantiated) });
                self.document_opening = Some(ShellDocumentOpening { label, phase: ShellDocumentOpenPhase::Instantiating, cancel_requested: false, started_at_ms: chrome_now_ms(), plugin_id, app, document, pending: Some(pending), prepared: None });
            }""", """                let label = app.label.resolve(self.active_terminology(), self.active_locale()).to_string();
                let app_id = app.id.clone();
                let pending = ShellDetached::spawn(async move { program.create_app(&app_id).await.map(ShellDocumentOpenAnswer::Instantiated) });
                self.document_opening = Some(ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Instantiating, plugin_id, app.id, document, Some(pending)));
            }""")
replace("""        let mut opening = opening.unwrap_or_else(|| ShellDocumentOpening { label, phase: ShellDocumentOpenPhase::Seeding, cancel_requested: false, started_at_ms: chrome_now_ms(), plugin_id: session.plugin_id.clone(), app: session.app.clone(), document: None, pending: None, prepared: None });""",
        """        let mut opening = opening.unwrap_or_else(|| ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Seeding, session.plugin_id.clone(), session.app.id.clone(), None, None));""")

# begin_document_resolution + advance Resolved arm
replace("""    /// ⏭️ Advances the retained open by the one detached step that answered, if any: an instance mounts""", """    /// 🧩️ Starts a hub document's open by resolving the component it runs by the SERVING catalog
    /// generation (binding decision of ticket 26/09/23, session 11 13:1x; React resolves the same generation
    /// through the hub's plugin-module routes): the local program only when its content hash is the lease's,
    /// else the verified store entry, else the hub's own execution-target bytes, verified and stored
    /// ([`DirectoryClient::resolve_execution_target_module`]) — all detached, with a cancellable band.
    /// Answers `false` for a relay that names no hub document of an explicit app, which opens as before.
    #[cfg(not(target_arch = "wasm32"))]
    fn begin_document_resolution(&mut self, target: &OpenArtifactRelayTarget) -> bool {
        let (Some(plugin_id), Some(app_id), Some(document_id), Some(schema), Some(space_id), Some(client)) =
            (target.plugin_id.clone(), target.app_id.clone(), target.document_id.clone(), target.schema.clone(), self.open_space_id.clone(), self.directory_client.clone())
        else {
            return false;
        };
        let local = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned();
        let local_app = local.as_ref().and_then(|program| program.manifest.apps.iter().find(|app| app.id == app_id).cloned());
        let surface_id = local_app.as_ref().map_or_else(|| semio_framework::manifest::surface_app_id(&target.dialect, target.role), |app| semio_framework::manifest::surface_app_id(&app.dialect, app.role));
        let label = local_app.map_or_else(|| app_id.clone(), |app| app.label.resolve(self.active_terminology(), self.active_locale()).to_string());
        let intent = semio_framework_os_kernel::os_directory::DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id.as_str()),
            requested_surface_id: Some(surface_id),
            client_instance_id: format!("wgpu-shell-{}", self.shell_session_id),
        };
        let ctx = self.directory_ctx();
        let cancel = ctx.cancel.clone();
        let resolve_step = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(0));
        let reported = resolve_step.clone();
        let local_sha = local.as_ref().and_then(|program| program.component_sha256.clone());
        let resolving_plugin = plugin_id.clone();
        let pending = ShellDetached::spawn(async move {
            let store = semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore::for_user();
            let resolved = client
                .resolve_execution_target_module(&ctx, &intent, local_sha.as_deref(), &store, |step| reported.store(document_open_resolve_step_code(step), std::sync::atomic::Ordering::Release))
                .await
                .map_err(|error| format!("document execution target: {error}"))?;
            let Some(files) = resolved.files else { return Ok(ShellDocumentOpenAnswer::Resolved(None)) };
            crate::program_bridge::load_resolved_program(&resolving_plugin, &files.component, &files.descriptor, &resolved.lease.component.sha256).await.map(|program| ShellDocumentOpenAnswer::Resolved(Some(program)))
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Resolving, plugin_id, app_id, Some(ShellDocumentOpenTarget { document_id, schema }), Some(pending));
        opening.cancel = Some(cancel);
        opening.resolve_step = resolve_step;
        self.document_opening = Some(opening);
        true
    }

    /// 🧩️ Continues a resolved hub open: a hub-resolved program replaces the local one of its plugin (a local
    /// program no longer mounts a hub document it is not the lease's component for), then the app instance
    /// is created on it; a local program that already is the lease's component seeds the document into the
    /// mounted session when it runs the app already.
    #[cfg(not(target_arch = "wasm32"))]
    async fn continue_resolved_open(&mut self, mut opening: ShellDocumentOpening, resolved: Option<ProgramBridgeEntry>) {
        let fresh = resolved.is_some();
        if let Some(program) = resolved {
            self.plugins.retain(|entry| entry.plugin_id != program.plugin_id);
            self.plugins.push(program);
        }
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id).cloned() else {
            self.document_opening = Some(opening);
            self.fail_document_opening("the resolved plugin is not mounted".to_string());
            return;
        };
        let Some(app) = program.manifest.apps.iter().find(|app| app.id == opening.app_id).cloned() else {
            self.document_opening = Some(opening);
            self.fail_document_opening(format!("{} declares no app {}", program.plugin_id, opening.app_id));
            return;
        };
        opening.label = app.label.resolve(self.active_terminology(), self.active_locale()).to_string();
        if !fresh && self.session.as_ref().is_some_and(|session| session.plugin_id == program.plugin_id && session.app.id == app.id) {
            let document = opening.document.take();
            self.document_opening = Some(opening);
            if let Some(document) = document {
                self.begin_document_seed(document).await;
            }
            return;
        }
        let app_id = app.id.clone();
        opening.pending = Some(ShellDetached::spawn(async move { program.create_app(&app_id).await.map(ShellDocumentOpenAnswer::Instantiated) }));
        opening.phase = ShellDocumentOpenPhase::Instantiating;
        self.document_opening = Some(opening);
    }

    /// ⏭️ Advances the retained open by the one detached step that answered, if any: an instance mounts""")
replace("""        opening.pending = None;
        match answer {
            Ok(ShellDocumentOpenAnswer::Instantiated(instance_id)) => {""", """        opening.pending = None;
        match answer {
            #[cfg(not(target_arch = "wasm32"))]
            Ok(ShellDocumentOpenAnswer::Resolved(resolved)) => {
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                self.continue_resolved_open(opening, resolved).await;
            }
            Ok(ShellDocumentOpenAnswer::Instantiated(instance_id)) => {""")
replace("""                self.install_app_session(&opening.plugin_id, opening.app.clone(), instance_id);""", """                let Some(app) = self.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id).and_then(|program| program.manifest.apps.iter().find(|app| app.id == opening.app_id).cloned()) else {
                    self.document_opening = Some(opening);
                    self.fail_document_opening("the opened app left its plugin".to_string());
                    return true;
                };
                self.install_app_session(&opening.plugin_id, app, instance_id);""")
replace("""            Err(error) => {
                self.document_opening = Some(opening);
                self.fail_document_opening(error);
            }
        }
        true
    }""", """            Err(_) if opening.cancel_requested => {
                opening.phase = ShellDocumentOpenPhase::Cancelled;
                self.document_opening = Some(opening);
            }
            Err(error) => {
                self.document_opening = Some(opening);
                self.fail_document_opening(error);
            }
        }
        true
    }""")
replace("""        if let Some(opening) = self.document_opening.as_mut() {
            opening.cancel_requested = true;
        }
    }""", """        if let Some(opening) = self.document_opening.as_mut() {
            opening.cancel_requested = true;
            if let Some(cancel) = opening.cancel.as_ref() {
                cancel.cancel_now();
            }
        }
    }""")

# band
replace("""/// 🔢️ The steps an open takes (the app instance, then the document), for the band's `step/total`.
const DOCUMENT_OPEN_STEPS: u8 = 2;""", """/// 🔢️ The steps an open takes (a hub document's component, the app instance, then the document), for the
/// band's `step/total`. An open with no hub document to resolve starts at step 2.
const DOCUMENT_OPEN_STEPS: u8 = 3;""")
replace("""    let (key, step) = match opening.phase {
        ShellDocumentOpenPhase::Instantiating => ("document.open.instantiating", Some(1)),
        ShellDocumentOpenPhase::Seeding => ("document.open.seeding", Some(DOCUMENT_OPEN_STEPS)),
        ShellDocumentOpenPhase::Cancelled => ("document.open.cancelled", None),
        ShellDocumentOpenPhase::Failed(_) => ("document.open.failed", None),
    };
    let head = format!("{} {}", shell_chrome_string(key, is_de), opening.label);""", """    let (key, step) = match opening.phase {
        ShellDocumentOpenPhase::Resolving => ("document.open.resolving", Some(1)),
        ShellDocumentOpenPhase::Instantiating => ("document.open.instantiating", Some(2)),
        ShellDocumentOpenPhase::Seeding => ("document.open.seeding", Some(DOCUMENT_OPEN_STEPS)),
        ShellDocumentOpenPhase::Cancelled => ("document.open.cancelled", None),
        ShellDocumentOpenPhase::Failed(_) => ("document.open.failed", None),
    };
    let head = match opening.phase {
        ShellDocumentOpenPhase::Resolving => {
            let detail = ["document.open.resolving.lease", "document.open.resolving.component", "document.open.resolving.descriptor", "document.open.resolving.verified"][usize::from(opening.resolve_step.load(std::sync::atomic::Ordering::Acquire).min(3))];
            format!("{} {} ({})", shell_chrome_string(key, is_de), opening.label, shell_chrome_string(detail, is_de))
        }
        _ => format!("{} {}", shell_chrome_string(key, is_de), opening.label),
    };""")
replace("""        ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding => transient_notice_tone(semio_framework::Severity::Info, theme),""", """        ShellDocumentOpenPhase::Resolving | ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding => transient_notice_tone(semio_framework::Severity::Info, theme),""")
replace("""        ("document.open.instantiating", false) => "Starting",""", """        ("document.open.resolving", false) => "Resolving the hub's component for",
        ("document.open.resolving", true) => "Hub-Komponente wird ermittelt für",
        ("document.open.resolving.lease", false) => "asking the hub which component runs it",
        ("document.open.resolving.lease", true) => "Hub wird nach der ausführenden Komponente gefragt",
        ("document.open.resolving.component", false) => "downloading the component",
        ("document.open.resolving.component", true) => "Komponente wird heruntergeladen",
        ("document.open.resolving.descriptor", false) => "downloading its descriptor",
        ("document.open.resolving.descriptor", true) => "Deskriptor wird heruntergeladen",
        ("document.open.resolving.verified", false) => "verified",
        ("document.open.resolving.verified", true) => "geprüft",
        ("document.open.instantiating", false) => "Starting",""")
path.write_text(text)
print("ok")
