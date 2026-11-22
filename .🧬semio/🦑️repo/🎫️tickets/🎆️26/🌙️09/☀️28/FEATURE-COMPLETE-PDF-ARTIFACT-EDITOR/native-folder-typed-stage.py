"""🚪 Stage native Shell and reactor routing for the folder archive page exchange."""
from pathlib import Path
from difflib import unified_diff
import runpy

ticket = Path(__file__).resolve().parent
base = runpy.run_path(str(ticket / "large-reopen-base.py"))
root, out = base["ROOT"], base["OUT"]
os = base["CHANNEL"].parent.parent
engine = os / "📺️renderer/🧑‍🎨engine"
reactor = os / "🔌️plugin/⚛️reactor"
changes = {}


def edit(path, old, new):
    original = path.read_text()
    current = changes.get(path, (original, original))[1]
    assert current.count(old) == 1, (path, old)
    changes[path] = original, current.replace(old, new)


def add(path, filename):
    assert not path.exists(), path
    changes[path] = "", (ticket / filename).read_text()


storage = os / "🏪️store/🔄️sync/🖥️native/🦀️.rs"
edit(storage, "pub use lifted_sync::{FOLDER_EVENT_MAGIC, FolderEventLogStorage, FolderTextStorage};",
     "pub use lifted_sync::{FOLDER_EVENT_MAGIC, FolderEventLogStorage, FolderTextStorage, NativeFolderArchiveSource, NativeFolderArchiveSeal, FOLDER_ARCHIVE_PAGE_BYTES};")

lifetime = reactor / "🚪️lifetime/🦀️.rs"
edit(lifetime, "pub use semio_plugin_decl_media::original_media::runtime as raw_media;", '''pub use semio_plugin_decl_media::original_media::runtime as raw_media;
#[path = "📥️folder-load.rs"]
mod folder_load;''')
add(lifetime.with_name("📥️folder-load.rs"), "native-folder-reopen-guest.rs")
edit(lifetime, "    pub raw_media: raw_media::OriginalMediaAdmission,", """    pub raw_media: raw_media::OriginalMediaAdmission,
    pub folder_archive: folder_load::MountedFolderArchiveLoad,""")
edit(lifetime, "            if !self.raw_media.close(cx)? { return Ok(false); }",
     "            if !self.folder_archive.cancel(cx)? || !self.raw_media.close(cx)? { return Ok(false); }")
edit(lifetime, "if !self.raw_media.close(cx)?{return Ok(false)}if let Some(source)=self.natural_source",
     "if !self.folder_archive.cancel(cx)?||!self.raw_media.close(cx)?{return Ok(false)}if let Some(source)=self.natural_source")
edit(lifetime, "raw_media:raw_media::OriginalMediaAdmission::empty(), natural_source:None,",
     "raw_media:raw_media::OriginalMediaAdmission::empty(), folder_archive:Default::default(), natural_source:None,")

app = reactor / "🔌️app/🦀️.rs"
anchor = "    /// 🎟️ Transfers one recursive archive owner into a fixed operation slot before decode."
edit(app, anchor, '''    /// 📎 Authorizes a durable folder record against the exact document and schema of this mounted app.
    fn validate_folder_archive_target(&self, _document: &str, _schema: &str) -> bool { false }

    /// 🫴 Accepts the verified original pages into this app's retained load registry.
    fn begin_paged_document_archive_load(
        &mut self, _operation: u64,
        _archive: &mut Option<protocol::channel::archive_ingress::VerifiedDocumentArchiveParent>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "paged document hydration is unavailable for this app").into())
    }

''' + anchor)
raw = reactor / "🏃️runtime/🧰️ops/🎞️raw-media/🦀️.rs"
edit(raw, "mod picker;", '''mod picker;
#[path = "📥️folder-load.rs"]
mod folder_load;
pub use folder_load::{drive_folder_archive, is_folder_archive_event};''')
add(raw.with_name("📥️folder-load.rs"), "native-folder-reopen-reactor.rs")
turn = reactor / "🔄️turn/🌀️poll-kernel-turn/🦀️.rs"
edit(turn, "    let selected_event = if prior_port_retired { select_turn_event(runtime, events, cx)? } else { None };",
     '''    let folder_archive = events.as_ref().is_some_and(semio_plugin_runtime_ops::ops_raw_media::is_folder_archive_event);
    if folder_archive && prior_port_retired {
        semio_plugin_runtime_ops::ops_raw_media::drive_folder_archive(runtime, events, &mut effects, budget.max_effects as usize, cx)?;
    }
    let selected_event = if prior_port_retired && !folder_archive { select_turn_event(runtime, events, cx)? } else { None };''')

bridge = engine / "🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🌉️program-bridge-entry/🦀️.rs"
anchor = "    pub async fn load_app_document_archive(&self"
edit(bridge, anchor, '''    /// 📬 Sends one scoped folder record request on the native bounded event queue.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn exchange_folder_archive(
        &self, request: protocol::channel::archive_ingress::folder_archive_load::FolderArchiveLoadRequest<'_>,
    ) -> Result<protocol::channel::archive_ingress::folder_archive_load::FolderArchiveLoadReply, String> {
        use protocol::channel::archive_ingress::folder_archive_load::FolderArchiveLoadReply as Reply;
        use semio_framework::kernel::{Event, FolderArchiveLoadEvent, MessageEndpoint, PluginInstanceId};
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        let instance = request.key.instance.to_string();
        let payload = request.encode().ok_or("folder archive request exceeded its wire authority")?;
        let event = Event::FolderArchiveLoad(FolderArchiveLoadEvent { instance: request.key.instance, request: payload });
        let outcome = client.exchange_original_media(request.key.instance, event).await.map_err(|error| error.to_string())?;
        let mut answer = None;
        for effect in outcome.effects {
            if let Effect::SendMessage { target: MessageEndpoint::Shell { instance: target }, payload } = effect {
                if !Reply::is_wire(&payload) { continue; }
                if target.0 != instance || answer.is_some() { return Err("folder archive reply changed its Shell instance".into()); }
                answer = Some(Reply::decode(request, &payload).ok_or("folder archive reply changed its operation")?);
            }
        }
        Ok(answer.unwrap_or(Reply::Pending))
    }

    /// 🧾 Uses the ordinary archive status and acknowledgment path after a verified page transfer.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn finish_paged_folder_archive(&self, instance: u32, operation: u64) -> Result<(), ProgramFault> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        wasm_program_exchange::drive_app_document_archive(client, instance, protocol::DocumentArchiveLoadHost::admitted(operation)).await
    }

''' + anchor)
wasm = engine / "🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🧳️wasm-program-exchange/🦀️.rs"
edit(wasm, """        let mut host = protocol::DocumentArchiveLoadHost::new(archive.clone());
        loop {""", '''        drive_app_document_archive(client, instance_id, protocol::DocumentArchiveLoadHost::new(archive.clone())).await
    }

    pub(super) async fn drive_app_document_archive(client: &KernelClient, instance_id: u32, mut host: protocol::DocumentArchiveLoadHost) -> Result<(), ProgramFault> {
        loop {''')

restore = engine / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/📎️local-folders/🦀️.rs"
edit(restore, "use super::*;", '''use super::*;
#[cfg(not(target_arch = "wasm32"))]
#[path = "📥️restore.rs"]
mod native_restore;''')
add(restore.with_name("📥️restore.rs"), "native-folder-reopen-drive.rs")
edit(restore, '''        let Some(bytes) = store_sync::sync::FolderEventLogStorage::new(std::path::PathBuf::from(path)).read_archive(document_id).await.map_err(|error| error.to_string())? else { return Ok(false) };
        self.restore_document_archive_bytes(&bytes).await?;
        Ok(true)''',
     '''        use protocol::channel::archive_ingress::folder_archive_load::FolderArchiveLoadKey;
        use std::sync::atomic::{AtomicU64, Ordering};
        static OPERATION: AtomicU64 = AtomicU64::new(1);
        let session = self.docs.session.clone().ok_or("session missing")?;
        let schema = &session.app.io.artifact_schema;
        let storage = store_sync::sync::FolderEventLogStorage::new(std::path::PathBuf::from(path));
        let limit = native_restore::NATIVE_FOLDER_ARCHIVE_REOPEN_MAXIMUM_BYTES;
        let cancelled = || false;
        let Some(mut source) = storage.open_archive_source(document_id, schema, limit, &cancelled).map_err(|error| error.to_string())? else {
            return Ok(false);
        };
        let seal = source.seal();
        if seal.extent <= protocol::channel::archive_reopen_route::DOCUMENT_ARCHIVE_CONTIGUOUS_REOPEN_MAXIMUM_BYTES {
            let mut bytes = Vec::new();
            bytes.try_reserve_exact(seal.extent).map_err(|error| error.to_string())?;
            bytes.resize(seal.extent, 0);
            let mut offset = 0;
            while offset < seal.extent {
                let count = source.read_next(&mut bytes[offset..], &cancelled).map_err(|error| error.to_string())?;
                if count == 0 { return Err("folder record ended prematurely".into()); }
                offset += count;
            }
            if !storage.archive_source_is_current(document_id, schema, seal, limit, &cancelled).map_err(|error| error.to_string())? {
                return Err("folder archive changed during restore".into());
            }
            self.restore_document_archive_bytes(&bytes).await?;
            return Ok(true);
        }
        let plugin = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
        let sequence = OPERATION.fetch_add(1, Ordering::Relaxed);
        let key = FolderArchiveLoadKey { instance: session.instance_id, activation: 0, lifetime: 0, sequence };
        let operation = key.operation().ok_or("folder archive operation authority exhausted")?;
        native_restore::restore(&storage, source, key, document_id, schema, cancelled,
            async |request| plugin.exchange_folder_archive(request).await).await?;
        plugin.finish_paged_folder_archive(session.instance_id, operation).await.map_err(|error| error.to_string())?;
        self.as_session().refresh_history_snapshot().await;
        self.refresh_ui(UiDirtyScope::Full).await?;
        Ok(true)''')

dispatch = reactor / "🔄️turn/📨️event-dispatch/🦀️.rs"
edit(dispatch, "Event::OriginalMediaControl(_)|Event::NaturalSourcePrepare(_)|Event::NaturalSourceControl(_))) {", "Event::OriginalMediaControl(_)|Event::NaturalSourcePrepare(_)|Event::NaturalSourceControl(_)|Event::FolderArchiveLoad(_))) {")
edit(dispatch, "Event::OriginalMediaControl(_) | Event::NaturalSourcePrepare(_) | Event::NaturalSourceControl(_) => unreachable!", "Event::OriginalMediaControl(_) | Event::NaturalSourcePrepare(_) | Event::NaturalSourceControl(_) | Event::FolderArchiveLoad(_) => unreachable!")
edit(turn, "|| matches!(events.as_ref(), Some(Event::NaturalSourcePrepare(_) | Event::NaturalSourceControl(_)));", "|| matches!(events.as_ref(), Some(Event::NaturalSourcePrepare(_) | Event::NaturalSourceControl(_) | Event::FolderArchiveLoad(_)));")

patch = []
for path, (old, new) in changes.items():
    assert len(str(path).encode("utf-16-le")) // 2 <= 256, path
    assert (path.read_text() if path.is_file() else "") == old, path
    relative = path.relative_to(root)
    patch.extend(unified_diff(old.splitlines(True), new.splitlines(True), f"a/{relative}" if old else "/dev/null", f"b/{relative}"))
(out / "native-folder-typed.patch").write_text("".join(patch))
(out / "folder_guest.rs").write_text((ticket / "native-folder-reopen-guest.rs").read_text())
(out / "folder_drive.rs").write_text((ticket / "native-folder-reopen-drive.rs").read_text())
