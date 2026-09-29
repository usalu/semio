
//#region 🗂️LocalCatalog
/// 🧭️ The local-catalog lane's guest commands and the landing app's re-hydration action.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LocalCatalogActionsV1 {
    pub admit: String,
    pub retire: String,
    pub rehydrate: String,
}

/// 📏️ The lane's bounds: one identifier, schema or name; one target path; the base64 pack + spr text of one admission; how
/// long a write (read back identically) or a read may take before it counts as failed.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LocalCatalogBoundsV1 {
    pub text_maximum_bytes: usize,
    pub target_maximum_bytes: usize,
    pub document_maximum_base64_bytes: usize,
    pub write_deadline_ms: u64,
    pub read_deadline_ms: u64,
}

/// 🧪️ One shared admission vector: a request (and the device's data folder) resolves to a refusal, or to the catalog entry,
/// the folder its events go to and the exact archive bytes written there.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LocalCatalogAdmissionVectorV1 {
    pub name: String,
    pub args: Option<Value>,
    pub data_dir: Option<String>,
    pub now_ms: u64,
    pub expect: LocalCatalogAdmissionExpectV1,
}

/// 🧪️ What an admission vector expects: `refusal`, or `document` + `folder` + `archiveHex`.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LocalCatalogAdmissionExpectV1 {
    #[serde(default)]
    pub refusal: Option<LocalCatalogNoticeV1>,
    #[serde(default)]
    pub document: Option<LocalDocument>,
    #[serde(default)]
    pub folder: Option<String>,
    #[serde(default)]
    pub archive_hex: Option<String>,
}

/// 🧪️ One shared catalog vector: the catalog persists as exactly `archiveHex` and reads back as itself.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LocalCatalogArchiveVectorV1 {
    pub catalog: LocalCatalog,
    pub archive_hex: String,
}

/// 📜️ The shared vocabulary file.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LocalCatalogVocabularyV1 {
    pub schema: String,
    pub description: String,
    pub actions: LocalCatalogActionsV1,
    pub code_prefix: String,
    pub bounds: LocalCatalogBoundsV1,
    pub notices: BTreeMap<String, ReplayRefusalLabelV1>,
    pub admissions: Vec<LocalCatalogAdmissionVectorV1>,
    pub catalog_archives: Vec<LocalCatalogArchiveVectorV1>,
}

/// 📜️ The shared local-catalog vocabulary, parsed once from the file ShellHost's lane reads too.
pub(crate) static LOCAL_CATALOG_V1: std::sync::LazyLock<LocalCatalogVocabularyV1> = std::sync::LazyLock::new(|| serde_json::from_str(include_str!("../../../🏛️ShellHost/🗂️local-catalog/🔣️.json")).expect("the shared local-catalog vocabulary parses"));

/// 🗣️ Everything the lane tells the user — its progress, its outcomes and every refusal, one per thing the user can act on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum LocalCatalogNoticeV1 {
    Keeping,
    Kept,
    Retired,
    NoDataFolder,
    InvalidRequest,
    DocumentUnknown,
    WriteFailed,
    Cancelled,
}

impl LocalCatalogNoticeV1 {
    pub(crate) const ALL: [Self; 8] = [Self::Keeping, Self::Kept, Self::Retired, Self::NoDataFolder, Self::InvalidRequest, Self::DocumentUnknown, Self::WriteFailed, Self::Cancelled];

    /// 🔑️ The notice's vocabulary key — its own kebab-case wire name.
    pub(crate) fn key(self) -> String {
        serde_json::to_value(self).ok().and_then(|value| value.as_str().map(str::to_owned)).unwrap_or_default()
    }

    /// 🗣️ The notice text in `locale` naming the document `name`; only an unknown locale reads English.
    pub(crate) fn text(self, locale: &str, name: &str) -> String {
        LOCAL_CATALOG_V1.notices.get(&self.key()).map(|label| if locale == "de" { label.de.as_str() } else { label.en.as_str() }).unwrap_or_default().replacen("{name}", name, 1)
    }

    /// 🩺️ The fault code the notice carries, so a probe, a law and the console name the same outcome.
    pub(crate) fn code(self) -> String {
        format!("{}{}", LOCAL_CATALOG_V1.code_prefix, self.key())
    }

    /// 🚦️ Progress and outcomes inform; every refusal warns.
    pub(crate) fn severity(self) -> semio_framework::Severity {
        if matches!(self, Self::Keeping | Self::Kept | Self::Retired) {
            semio_framework::Severity::Info
        } else {
            semio_framework::Severity::Warning
        }
    }
}

/// 📥️ One validated `os.local-catalog.admit`: the catalog entry, the exact archive its folder lane receives and that folder.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LocalCatalogAdmissionV1 {
    pub document: LocalDocument,
    pub archive: Vec<u8>,
    pub folder: String,
}

/// ✂️ A bounded text argument: at most `maximum_bytes` of UTF-8, no control character, no surrounding space.
fn local_catalog_text(value: Option<&Value>, maximum_bytes: usize, allow_empty: bool) -> Option<String> {
    let text = value?.as_str()?;
    let bounded = (allow_empty || !text.is_empty()) && text.len() <= maximum_bytes;
    let clean = !text.chars().any(|character| matches!(character, '\u{0}'..='\u{1f}' | '\u{7f}')) && text.trim_matches(|character: char| character.is_whitespace() || character == '\u{feff}') == text;
    (bounded && clean).then(|| text.to_string())
}

/// 🔤️ A bounded, padded standard-base64 argument's bytes.
fn local_catalog_base64(value: Option<&Value>) -> Option<Vec<u8>> {
    let text = value?.as_str()?;
    let body = text.trim_end_matches('=');
    let shaped = text.len() <= LOCAL_CATALOG_V1.bounds.document_maximum_base64_bytes && text.len() % 4 == 0 && text.len() - body.len() <= 2 && body.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'+' || byte == b'/');
    shaped.then(|| semio_framework_io_base64::base64_standard_decode(text).ok()).flatten()
}

/// 🏷️ How a notice names a kept document: its name, else its id.
fn local_catalog_display_name(document: &LocalDocument) -> String {
    if document.name.is_empty() {
        document.document_id.clone()
    } else {
        document.name.clone()
    }
}

/// 📁️ Where a catalog entry's events live: its own folder, or the folder of its file — ShellHost's `localCatalogBindingV1`.
pub(crate) fn local_catalog_folder_v1(document: &LocalDocument) -> String {
    if document.storage == LocalDocumentStorage::Folder {
        return document.target.clone();
    }
    let trimmed = document.target.trim_end_matches('/');
    match trimmed.rfind('/') {
        Some(cut) if cut > 0 => trimmed[..cut].to_string(),
        _ => "/".to_string(),
    }
}

/// 🔎️ Validates one `os.local-catalog.admit` request against the lane's bounds and resolves its target — ShellHost's
/// `localCatalogAdmissionV1`, pinned to it by the shared vectors: an empty folder target means `<dataDir>/os/local-documents/<id>`,
/// which needs a data folder; an id that is a path segment of its own (`/`, `.`, `..`) never names that folder.
pub(crate) fn local_catalog_admission_v1(args: Option<&Value>, data_dir: Option<&str>, now_ms: u64) -> Result<LocalCatalogAdmissionV1, LocalCatalogNoticeV1> {
    let invalid = LocalCatalogNoticeV1::InvalidRequest;
    let bounds = &LOCAL_CATALOG_V1.bounds;
    let args = args.and_then(Value::as_object);
    let field = |key: &str| args.and_then(|args| args.get(key));
    let storage = match field("storage").and_then(Value::as_str) {
        Some("folder") => Some(LocalDocumentStorage::Folder),
        Some("file") => Some(LocalDocumentStorage::File),
        _ => None,
    };
    let requested = match field("target") {
        None | Some(Value::Null) => Some(String::new()),
        target => local_catalog_text(target, bounds.target_maximum_bytes, true),
    };
    let fields = (
        local_catalog_text(field("documentId"), bounds.text_maximum_bytes, false),
        local_catalog_text(field("schema"), bounds.text_maximum_bytes, false),
        local_catalog_text(field("name"), bounds.text_maximum_bytes, true),
        storage,
        requested,
        local_catalog_base64(field("pack")),
        local_catalog_base64(field("spr")),
    );
    let (Some(document_id), Some(schema), Some(name), Some(storage), Some(requested), Some(pack), Some(spr)) = fields else { return Err(invalid) };
    if pack.is_empty() || document_id.contains('/') || document_id == "." || document_id == ".." {
        return Err(invalid);
    }
    let absolute = requested.starts_with('/') || std::path::Path::new(&requested).is_absolute();
    if (storage == LocalDocumentStorage::File || !requested.is_empty()) && !absolute {
        return Err(invalid);
    }
    let target = if requested.is_empty() {
        let root = data_dir.map(|dir| dir.trim_end_matches('/')).filter(|dir| !dir.is_empty()).ok_or(LocalCatalogNoticeV1::NoDataFolder)?;
        format!("{root}/os/local-documents/{document_id}")
    } else {
        requested
    };
    let archive = store_sync::os_spr::encode_document_archive_bytes(&store_sync::os_spr::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() }).map_err(|_| invalid)?;
    let document = LocalDocument { document_id, schema, name, storage, target, admitted_at_ms: now_ms };
    let folder = local_catalog_folder_v1(&document);
    Ok(LocalCatalogAdmissionV1 { document, archive, folder })
}

/// 🗃️ The catalog facet's persisted form — the whole catalog as the archive's pack value (ShellHost's `localCatalogArchiveV1`).
pub(crate) fn local_catalog_archive_v1(catalog: &LocalCatalog) -> Result<Vec<u8>, String> {
    let pack = store::pack_rt::encode_wire_value(&dsl::ToValue::to_value(catalog));
    store_sync::os_spr::encode_document_archive_bytes(&store_sync::os_spr::DocumentArchivePack { parent_pack: pack, parent_spr: Vec::new(), members: Vec::new() }).map_err(|error| error.to_string())
}

/// 🗃️ The catalog a persisted archive holds, or `None` when the bytes are not one.
pub(crate) async fn decode_local_catalog_archive_v1(bytes: &[u8]) -> Option<LocalCatalog> {
    let archive = store_sync::os_spr::decode_document_archive_bytes(bytes).await.ok()?;
    <LocalCatalog as dsl::FromValue>::from_value(store::pack_rt::decode_wire_value(&archive.parent_pack).ok()?).ok()
}

/// 🗃️ The landing app's `applyLocalCatalogDocument` arguments for one kept document: its id and its archive's own pack/spr
/// pair (base64), or `None` when the bytes are not a document archive.
pub(crate) async fn local_catalog_rehydration_v1(document_id: &str, archive: &[u8]) -> Option<DslValue> {
    let archive = store_sync::os_spr::decode_document_archive_bytes(archive).await.ok()?;
    Some(DslValue::Object(vec![
        ("documentId".to_string(), DslValue::String(document_id.to_string())),
        ("pack".to_string(), DslValue::String(semio_framework_io_base64::base64_standard_encode(&archive.parent_pack))),
        ("spr".to_string(), DslValue::String(semio_framework_io_base64::base64_standard_encode(&archive.parent_spr))),
    ]))
}

/// 🗂️ One job of the native lane. Jobs run one at a time, in order, on the I/O lane, so the catalog is only ever written from
/// the base the previous job left.
#[cfg(not(target_arch = "wasm32"))]
enum LocalCatalogJobV1 {
    Load,
    Admit { requester: Option<u32>, admission: Box<LocalCatalogAdmissionV1> },
    Retire { document_id: String },
    Rehydrate { instance_id: u32, document: LocalDocument },
}

/// 📨️ What one settled job answers on the UI turn.
#[cfg(not(target_arch = "wasm32"))]
enum LocalCatalogAnswerV1 {
    Loaded(LocalCatalog),
    Admitted { requester: Option<u32>, catalog: LocalCatalog, document: LocalDocument, rehydration: Option<DslValue> },
    Retired { catalog: LocalCatalog, name: String },
    Read { instance_id: u32, rehydration: Option<DslValue> },
    Failed { notice: LocalCatalogNoticeV1, name: String },
}

/// 🔄️ The running job: its retained-waker task, the answer channel, and the answer its deadline settles it with.
#[cfg(not(target_arch = "wasm32"))]
struct LocalCatalogRunV1 {
    task: std::sync::Arc<ShellPoolFuture>,
    answer: std::sync::mpsc::Receiver<LocalCatalogAnswerV1>,
    deadline_ms: u64,
    timeout: LocalCatalogAnswerV1,
}

/// 🗂️ The native shell's host-owned local document catalog (`os.config.local-catalog`, persisted local-only in
/// `<S_DATA_DIR>/os` on this shell's own folder backbone; in memory only without a data folder) — ShellHost's route-B lane:
/// `os.local-catalog.admit` writes the document into its folder lane and counts it only once the lane reads the same archive
/// back, then records `admitLocalDocument` in the facet; `os.local-catalog.retire` records `retireLocalDocument` (the events
/// stay on disk); the landing app is handed every kept document its instance was not handed yet. Opened at boot, with or
/// without a hub. Ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP (WG11, session 15).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
pub(crate) struct LocalCatalogLaneV1 {
    pub catalog: LocalCatalog,
    jobs: VecDeque<LocalCatalogJobV1>,
    running: Option<LocalCatalogRunV1>,
    rehydrated: Option<(u32, std::collections::BTreeSet<String>)>,
}

/// 🔑️ The re-hydration ledger key of one kept document — ShellHost's `${documentId}@${admittedAtMs}`.
#[cfg(not(target_arch = "wasm32"))]
fn local_catalog_seen_key(document: &LocalDocument) -> String {
    format!("{}@{}", document.document_id, document.admitted_at_ms)
}

/// 🗃️ Appends `next` to the catalog facet's own event log, when the lane has one.
#[cfg(not(target_arch = "wasm32"))]
async fn record_local_catalog_v1(root: Option<&std::path::Path>, next: &LocalCatalog) -> Result<(), String> {
    let Some(root) = root else { return Ok(()) };
    let schema = semio_framework_os_config::mutations::LOCAL_CATALOG_CONFIG_SCHEMA;
    let archive = local_catalog_archive_v1(next)?;
    store_sync::sync::FolderEventLogStorage::new(root.to_path_buf()).write_archive(schema, schema, &archive).await.map_err(|error| error.to_string())
}

/// ⚙️ Runs one job's folder I/O from the catalog `base` it starts on.
#[cfg(not(target_arch = "wasm32"))]
async fn run_local_catalog_job_v1(job: LocalCatalogJobV1, root: Option<std::path::PathBuf>, base: LocalCatalog) -> LocalCatalogAnswerV1 {
    use semio_framework_os_config::mutations::{admit_local_document, apply_local_catalog_config_mutation, retire_local_document, LOCAL_CATALOG_CONFIG_SCHEMA};
    match job {
        LocalCatalogJobV1::Load => {
            let Some(root) = root else { return LocalCatalogAnswerV1::Loaded(base) };
            let archive = store_sync::sync::FolderEventLogStorage::new(root).read_archive(LOCAL_CATALOG_CONFIG_SCHEMA).await.ok().flatten();
            LocalCatalogAnswerV1::Loaded(match archive {
                Some(bytes) => decode_local_catalog_archive_v1(&bytes).await.unwrap_or(base),
                None => base,
            })
        }
        LocalCatalogJobV1::Admit { requester, admission } => {
            let name = local_catalog_display_name(&admission.document);
            let failed = |name: String| LocalCatalogAnswerV1::Failed { notice: LocalCatalogNoticeV1::WriteFailed, name };
            let lane = store_sync::sync::FolderEventLogStorage::new(std::path::PathBuf::from(&admission.folder));
            let document_id = admission.document.document_id.as_str();
            if lane.write_archive(document_id, &admission.document.schema, &admission.archive).await.is_err() || lane.read_archive(document_id).await.ok().flatten().as_deref() != Some(admission.archive.as_slice()) {
                return failed(name);
            }
            let mut catalog = base;
            if apply_local_catalog_config_mutation(&mut catalog, &admit_local_document(admission.document.clone())).is_err() || record_local_catalog_v1(root.as_deref(), &catalog).await.is_err() {
                return failed(name);
            }
            let rehydration = local_catalog_rehydration_v1(document_id, &admission.archive).await;
            LocalCatalogAnswerV1::Admitted { requester, catalog, document: admission.document, rehydration }
        }
        LocalCatalogJobV1::Retire { document_id } => {
            let name = base.documents.iter().find(|document| document.document_id == document_id).map(local_catalog_display_name).unwrap_or_else(|| document_id.clone());
            let mut catalog = base;
            if apply_local_catalog_config_mutation(&mut catalog, &retire_local_document(&document_id)).is_err() || record_local_catalog_v1(root.as_deref(), &catalog).await.is_err() {
                return LocalCatalogAnswerV1::Failed { notice: LocalCatalogNoticeV1::WriteFailed, name };
            }
            LocalCatalogAnswerV1::Retired { catalog, name }
        }
        LocalCatalogJobV1::Rehydrate { instance_id, document } => {
            let archive = store_sync::sync::FolderEventLogStorage::new(std::path::PathBuf::from(local_catalog_folder_v1(&document))).read_archive(&document.document_id).await.ok().flatten();
            let rehydration = match archive {
                Some(archive) => local_catalog_rehydration_v1(&document.document_id, &archive).await,
                None => None,
            };
            LocalCatalogAnswerV1::Read { instance_id, rehydration }
        }
    }
}

impl ShellState {
    /// 🗣️ Speaks one lane notice in the shell's tongue under its fault code.
    fn speak_local_catalog(&mut self, notice: LocalCatalogNoticeV1, name: &str) {
        let code = notice.code();
        self.show_transient_notice(notice.text(&self.locale_id, name), notice.severity(), Some(code.as_str()));
    }

    /// 📥️ `os.local-catalog.admit` / `os.local-catalog.retire` from the landing app — validated, spoken and queued on the lane;
    /// any other `os.local-catalog.*` verb is not this lane's and is refused as unrouted.
    fn replay_local_catalog(&mut self, action_id: &str, args: Option<&Value>) -> Option<ReplayRefusalReasonV1> {
        let actions = &LOCAL_CATALOG_V1.actions;
        if action_id == actions.admit {
            match local_catalog_admission_v1(args, self.local_catalog_data_dir().as_deref(), chrome_now_ms() as u64) {
                Ok(admission) => {
                    self.speak_local_catalog(LocalCatalogNoticeV1::Keeping, &local_catalog_display_name(&admission.document));
                    self.queue_local_catalog_admission(admission);
                }
                Err(notice) => self.speak_local_catalog(notice, ""),
            }
            return None;
        }
        if action_id == actions.retire {
            let document_id = local_catalog_text(args.and_then(|args| args.get("documentId")), LOCAL_CATALOG_V1.bounds.text_maximum_bytes, false);
            match document_id.filter(|document_id| self.local_catalog_documents().iter().any(|document| &document.document_id == document_id)) {
                Some(document_id) => self.queue_local_catalog_retirement(document_id),
                None => self.speak_local_catalog(LocalCatalogNoticeV1::DocumentUnknown, ""),
            }
            return None;
        }
        Some(ReplayRefusalReasonV1::UnroutedCommand)
    }

    /// 📁️ The device data folder the lane keeps documents under, if this shell has one.
    #[cfg(not(target_arch = "wasm32"))]
    fn local_catalog_data_dir(&self) -> Option<String> {
        self.identity_env.as_ref().and_then(|env| env.data_dir.as_ref()).map(|dir| dir.to_string_lossy().into_owned())
    }

    /// 📁️ The browser shell has no data folder.
    #[cfg(target_arch = "wasm32")]
    fn local_catalog_data_dir(&self) -> Option<String> {
        None
    }

    /// 🗂️ The documents this device keeps.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn local_catalog_documents(&self) -> &[LocalDocument] {
        &self.local_catalog.catalog.documents
    }

    /// 🗂️ The browser shell keeps none.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn local_catalog_documents(&self) -> &[LocalDocument] {
        &[]
    }

    /// 📥️ Queues the verified write + record of one admission for the session that asked for it.
    #[cfg(not(target_arch = "wasm32"))]
    fn queue_local_catalog_admission(&mut self, admission: LocalCatalogAdmissionV1) {
        let requester = self.session.as_ref().map(|session| session.instance_id);
        self.local_catalog.jobs.push_back(LocalCatalogJobV1::Admit { requester, admission: Box::new(admission) });
    }

    /// 📥️ The browser shell has no folder backbone to write a document into.
    #[cfg(target_arch = "wasm32")]
    fn queue_local_catalog_admission(&mut self, _admission: LocalCatalogAdmissionV1) {
        self.speak_local_catalog(LocalCatalogNoticeV1::NoDataFolder, "");
    }

    /// 📤️ Queues the record of one retirement.
    #[cfg(not(target_arch = "wasm32"))]
    fn queue_local_catalog_retirement(&mut self, document_id: String) {
        self.local_catalog.jobs.push_back(LocalCatalogJobV1::Retire { document_id });
    }

    /// 📤️ Nothing is kept on the browser shell.
    #[cfg(target_arch = "wasm32")]
    fn queue_local_catalog_retirement(&mut self, _document_id: String) {
        self.speak_local_catalog(LocalCatalogNoticeV1::DocumentUnknown, "");
    }

    /// 🗂️ The folder the catalog facet persists in (`<S_DATA_DIR>/os`), or `None` when the catalog lives in memory only.
    #[cfg(not(target_arch = "wasm32"))]
    fn local_catalog_root(&self) -> Option<std::path::PathBuf> {
        self.identity_env.as_ref().and_then(|env| env.data_dir.as_ref()).map(|dir| dir.join("os"))
    }

    /// 🗂️ Opens the catalog facet at boot — local-first, with or without a hub.
    #[cfg(not(target_arch = "wasm32"))]
    fn open_local_catalog(&mut self) {
        if self.local_catalog_root().is_some() {
            self.local_catalog.jobs.push_back(LocalCatalogJobV1::Load);
        }
    }

    /// 🏠️ The landing app's live instance, when the session is it.
    #[cfg(not(target_arch = "wasm32"))]
    fn local_catalog_landing_instance(&self) -> Option<u32> {
        let config = self.host_config()?;
        self.session.as_ref().filter(|session| session.plugin_id == config.plugin_id && session.app.id == config.landing_app_id).map(|session| session.instance_id)
    }

    /// 🗃️ Queues a read of every kept document the landing app's live instance was not handed yet.
    #[cfg(not(target_arch = "wasm32"))]
    fn schedule_local_catalog_rehydration(&mut self) {
        let Some(instance_id) = self.local_catalog_landing_instance() else { return };
        let lane = &mut self.local_catalog;
        if lane.rehydrated.as_ref().map(|(owner, _)| *owner) != Some(instance_id) {
            lane.rehydrated = Some((instance_id, std::collections::BTreeSet::new()));
        }
        let Some((_, seen)) = lane.rehydrated.as_mut() else { return };
        for document in &lane.catalog.documents {
            if seen.insert(local_catalog_seen_key(document)) {
                lane.jobs.push_back(LocalCatalogJobV1::Rehydrate { instance_id, document: document.clone() });
            }
        }
    }

    /// 🗃️ Hands one kept document to the landing app instance it was read for, while that instance is still the session.
    #[cfg(not(target_arch = "wasm32"))]
    async fn rehydrate_local_document(&mut self, instance_id: u32, rehydration: DslValue) {
        let Some(controller_id) = self.session.as_ref().filter(|session| session.instance_id == instance_id).map(|session| session.app.controller_id.clone()) else { return };
        if let Err(error) = self.dispatch_action(ActionDescriptor { controller_id, action: LOCAL_CATALOG_V1.actions.rehydrate.clone(), args: Some(rehydration) }).await {
            Self::debug_log(&format!("[TRACE] wgpu shell local catalog re-hydration failed: {error}"));
        }
    }

    /// 🗂️ One bounded lane turn per frame: settles the running job (or its deadline), queues the landing app's re-hydration,
    /// then starts the next job on the I/O lane — an admission whose session is gone is cancelled before anything is written.
    #[cfg(not(target_arch = "wasm32"))]
    async fn pump_local_catalog(&mut self) -> bool {
        let mut changed = false;
        if let Some(run) = self.local_catalog.running.take() {
            let answer = match run.answer.try_recv() {
                Ok(answer) => answer,
                Err(std::sync::mpsc::TryRecvError::Empty) if crate::renderer_worker_pool().now_ms() < run.deadline_ms => {
                    self.local_catalog.running = Some(run);
                    return false;
                }
                Err(_) => {
                    run.task.cancel();
                    run.timeout
                }
            };
            self.settle_local_catalog(answer).await;
            changed = true;
        }
        self.schedule_local_catalog_rehydration();
        let Some(job) = self.local_catalog.jobs.pop_front() else { return changed };
        let bounds = &LOCAL_CATALOG_V1.bounds;
        let (deadline, timeout) = match &job {
            LocalCatalogJobV1::Load => (bounds.read_deadline_ms, LocalCatalogAnswerV1::Loaded(self.local_catalog.catalog.clone())),
            LocalCatalogJobV1::Admit { requester, admission } => {
                let name = local_catalog_display_name(&admission.document);
                if self.session.as_ref().map(|session| session.instance_id) != *requester {
                    self.speak_local_catalog(LocalCatalogNoticeV1::Cancelled, &name);
                    return true;
                }
                (bounds.write_deadline_ms, LocalCatalogAnswerV1::Failed { notice: LocalCatalogNoticeV1::WriteFailed, name })
            }
            LocalCatalogJobV1::Retire { document_id } => (bounds.write_deadline_ms, LocalCatalogAnswerV1::Failed { notice: LocalCatalogNoticeV1::WriteFailed, name: document_id.clone() }),
            LocalCatalogJobV1::Rehydrate { instance_id, .. } => (bounds.read_deadline_ms, LocalCatalogAnswerV1::Read { instance_id: *instance_id, rehydration: None }),
        };
        let (sender, answer) = std::sync::mpsc::channel();
        let (root, base) = (self.local_catalog_root(), self.local_catalog.catalog.clone());
        let pool = crate::renderer_worker_pool();
        let deadline_ms = pool.now_ms().saturating_add(deadline);
        let task = ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let _ = sender.send(run_local_catalog_job_v1(job, root, base).await);
        });
        self.local_catalog.running = Some(LocalCatalogRunV1 { task, answer, deadline_ms, timeout });
        true
    }

    /// 📨️ Folds one settled job: the catalog it left, the notice it speaks and the landing app's re-hydration.
    #[cfg(not(target_arch = "wasm32"))]
    async fn settle_local_catalog(&mut self, answer: LocalCatalogAnswerV1) {
        match answer {
            LocalCatalogAnswerV1::Loaded(catalog) => self.local_catalog.catalog = catalog,
            LocalCatalogAnswerV1::Admitted { requester, catalog, document, rehydration } => {
                self.local_catalog.catalog = catalog;
                self.speak_local_catalog(LocalCatalogNoticeV1::Kept, &local_catalog_display_name(&document));
                let Some(instance_id) = requester else { return };
                if let Some((_, seen)) = self.local_catalog.rehydrated.as_mut().filter(|(owner, _)| *owner == instance_id) {
                    seen.insert(local_catalog_seen_key(&document));
                }
                if let Some(rehydration) = rehydration {
                    self.rehydrate_local_document(instance_id, rehydration).await;
                }
            }
            LocalCatalogAnswerV1::Retired { catalog, name } => {
                self.local_catalog.catalog = catalog;
                self.speak_local_catalog(LocalCatalogNoticeV1::Retired, &name);
            }
            LocalCatalogAnswerV1::Read { instance_id, rehydration: Some(rehydration) } => self.rehydrate_local_document(instance_id, rehydration).await,
            LocalCatalogAnswerV1::Read { rehydration: None, .. } => {}
            LocalCatalogAnswerV1::Failed { notice, name } => self.speak_local_catalog(notice, &name),
        }
    }
}
//#endregion 🗂️LocalCatalog
