"""🪢 Stage one exact first-archive lane across the SDK, guest reactor and native queue."""
from difflib import SequenceMatcher, unified_diff
from pathlib import Path
from runpy import run_path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
os = next(root.glob("🧰*/🛍*/💻*/🔨*"))
plugin = next(os.glob("🔌*"))
engine = next(os.glob("📺*/🧑*"))
reactor = plugin / "⚛️reactor"
instance = plugin / "🧵️instance"
scope = ticket / "🗑️generated/native-first-archive-exchange"
scope.mkdir(parents=True, exist_ok=True)
changes = {}


def edit(path, old, new):
    path = path.resolve()
    base, current = changes.get(path, (None, None))
    if base is None:
        base = current = path.read_text()
    assert current.count(old) == 1, (path, old, current.count(old))
    changes[path] = base, current.replace(old, new, 1)


def add(path, name):
    assert len(str(path).encode("utf-16-le")) // 2 <= 256
    changes[path.resolve()] = "", (ticket / name).read_text()


def merge(updates):
    for path, (base, updated) in updates.items():
        path = path.resolve()
        if path not in changes:
            changes[path] = base, updated
            continue
        original, current = changes[path]
        assert original == base, path
        before, after = base.splitlines(True), updated.splitlines(True)
        for tag, left, end, right, stop in reversed(SequenceMatcher(None, before, after, autojunk=False).get_opcodes()):
            if tag == "equal":
                continue
            old, new = "".join(before[left:end]), "".join(after[right:stop])
            if tag == "insert":
                anchor = "".join(before[max(0, left - 2):left])
                assert anchor and current.count(anchor) == 1, (path, anchor)
                current = current.replace(anchor, anchor + new, 1)
            else:
                assert current.count(old) == 1, (path, old)
                current = current.replace(old, new, 1)
        changes[path] = base, current


for stage in ("natural-pdf-publication-stage.py", "native-natural-source-stage.py",
              "native-archive-readback-stage.py", "native-first-genesis-stage.py", "native-first-archive-disk-stage.py"):
    merge(run_path(str(ticket / stage))["changes"])

publication = instance / "🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs"
edit(publication, "let natural_receipt = if natural { size_of::<bool>() } else { 0 };",
     "let natural_receipt = if natural { size_of::<bool>() + size_of::<Option<(u64, [u8; 32])>>() } else { 0 };")
edit(publication, 'if natural { self.media.natural_source.as_mut().expect("matched natural publication").committed = true; }', '''
            if natural {
                let owner = self.media.natural_source.as_mut().expect("matched natural publication");
                owner.committed = true;
                owner.committed_frontier = Some((generation, revision));
            }'''.strip())

owner = next(path for path in changes if path.name == "🪪️natural-publication.rs" or
             "pub struct NaturalFilePublication {" in changes[path][1])
edit(owner, "    pub committed: bool,", "    pub committed: bool,\n    pub committed_frontier: Option<(u64, [u8; 32])>,")
edit(owner, "committed: false, input_donated", "committed: false, committed_frontier: None, input_donated")
source = next(path for path in changes if "pub struct NaturalSourceGuest<" in changes[path][1])
edit(source, "    terminal_receipt_sent: bool,", "    terminal_receipt_sent: bool,\n    first_archive_frontier: Option<(u64, [u8; 32])>,")
edit(source, "                terminal_receipt_sent: false,", "                terminal_receipt_sent: false,\n                first_archive_frontier: None,")
edit(source, "    pub fn terminal_receipt(", """    /// 🪪 Captures the actual first Store frontier before successful Complete can leave the guest.
    pub fn capture_first_archive_frontier(&mut self, frontier: (u64, [u8; 32]), cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if let Some(current) = self.first_archive_frontier {
            return if current == frontier { Ok(true) } else { Err(ValueError::invariant("natural first archive frontier changed").into()) };
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of_val(&self.first_archive_frontier), ..Default::default() };
        if cx.should_yield() || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
            return Ok(false);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        self.first_archive_frontier = Some(frontier);
        Ok(true)
    }

    pub fn first_archive_frontier(&self) -> Option<(u64, [u8; 32])> {
        if self.stage == Stage::Published && self.terminal_receipt_sent { self.first_archive_frontier } else { None }
    }

    pub fn terminal_receipt(""")
appointment = next(path for path in changes if "async fn advance_natural_owner(" in changes[path][1])
edit(appointment, "        if !cancelled && !source.finish_format", """        if !cancelled && owner.committed {
            let frontier = owner.committed_frontier.ok_or_else(|| ValueError::invariant("natural commit lost its exact Store frontier"))?;
            if frontier != (self.stores.store.generation_now(), self.stores.store.content_revision_now()) {
                return Err(ValueError::invariant("natural first archive Store frontier was superseded").into());
            }
            if !source.capture_first_archive_frontier(frontier, cx)? { return Ok(()); }
        }
        if !cancelled && !source.finish_format""")
app_impl = instance / "🔌️plugin-app-impl/🦀️.rs"
_, app_current = changes[app_impl.resolve()]
appointment_name = appointment.name
app_module = next(line for line in app_current.splitlines() if line.startswith("#[path") and appointment_name in line)
edit(app_impl, app_module, '''#[path = "📤️first-archive.rs"]
mod first_archive;
''' + app_module)
add(app_impl.with_name("📤️first-archive.rs"), "native-first-archive-sdk.rs")

app = reactor / "🔌️app/🦀️.rs"
edit(app, "    fn natural_source_format_closed(&self) -> bool { true }", """    fn natural_source_format_closed(&self) -> bool { true }

    /// 📤 Supplies one funded first-genesis archive turn when its app owns the canonical first SPR.
    fn first_genesis_archive_turn(
        &mut self, _source: &semio_plugin_decl_media::original_media::natural_source::NaturalFileSourceGuest,
        request: semio_os_kernel::channel::FirstArchiveRequest, output: &mut Vec<u8>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        if !semio_os_kernel::channel::FirstArchiveReply::Refused.encode_into(request, output) {
            return Err(semio_framework_value::ValueError::invariant("first archive response recipient is absent").into());
        }
        Ok(())
    }""")

raw = reactor / "🏃️runtime/🧰️ops/🎞️raw-media"
edit(raw / "🦀️.rs", "mod natural_source;", """mod natural_source;
#[path = "📤️first-archive.rs"]
mod first_archive;
pub use first_archive::{drive_first_archive, is_first_archive_event};""")
add(raw / "📤️first-archive.rs", "native-first-archive-reactor.rs")
turn = reactor / "🔄️turn/🌀️poll-kernel-turn/🦀️.rs"
edit(turn, "    let selected_event = if prior_port_retired { select_turn_event(runtime, events, cx)? } else { None };",
     """    let first_archive = events.as_ref().is_some_and(semio_plugin_runtime_ops::ops_raw_media::is_first_archive_event);
    if first_archive && prior_port_retired {
        semio_plugin_runtime_ops::ops_raw_media::drive_first_archive(runtime, events, &mut effects, budget.max_effects as usize, cx)?;
    }
    let selected_event = if prior_port_retired && !first_archive { select_turn_event(runtime, events, cx)? } else { None };""")

target = engine / "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime"
core = target / "🧮️kernel-core/🦀️.rs"
edit(core, "    pub fn is_original_media(&self) -> bool { self.original_media.is_some() }", """    pub fn is_original_media(&self) -> bool {
        self.original_media.is_some() || (matches!(self.kind, Some(QueuedKernelEventKind::MessageShell))
            && self.message_payload.as_deref().is_some_and(|bytes| protocol::channel::FirstArchiveRequest::decode(bytes).is_some()))
    }""")
pool = target / "🏊️kernel-pool/🚀️turn-dispatch-and-pool-runner/🦀️.rs"
edit(pool, "                    if let Ok(frame) = protocol::decode_app_frame(payload).await {", """                    if protocol::channel::FirstArchiveReply::is_wire(payload) {
                        effects.push(effect);
                        continue;
                    }
                    if let Ok(frame) = protocol::decode_app_frame(payload).await {""")

bridge = engine / "🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🌉️program-bridge-entry/🦀️.rs"
edit(bridge, "    /// 📤 Delivers one selected source", """    /// 📬 Exchanges one exact first-genesis message on the retained native queue.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn exchange_first_archive(
        &self, request: protocol::channel::FirstArchiveRequest, cancel: Option<&semio_framework_async::CancelToken>,
    ) -> Result<Option<Vec<u8>>, String> {
        use semio_framework::kernel::{Event, MessageEndpoint, PluginInstanceId};
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        let instance = request.key.instance;
        let key = instance.to_string();
        let event = Event::Message {
            source: MessageEndpoint::Shell { instance: PluginInstanceId(key.clone()) }, payload: request.encode().to_vec(),
        };
        let exchange = client.exchange_events(instance, vec![event]);
        let outcome = if let Some(cancel) = cancel {
            match semio_framework_async::select2(cancel.cancelled(), exchange).await {
                semio_framework_async::Either::Left(()) => return Err("first archive cancelled".into()),
                semio_framework_async::Either::Right(outcome) => outcome,
            }
        } else { exchange.await }.map_err(|error| error.to_string())?;
        let mut reply = None;
        for effect in outcome.effects {
            if let Effect::SendMessage { target: MessageEndpoint::Shell { instance }, payload } = effect {
                if !protocol::channel::FirstArchiveReply::is_wire(&payload) { continue; }
                if instance.0 != key || protocol::channel::FirstArchiveReply::decode(request, &payload).is_none()
                    || reply.is_some() { return Err("foreign or repeated first archive reply".into()); }
                reply = Some(payload);
            }
        }
        Ok(reply)
    }

    /// 📤 Delivers one selected source""")

shell = engine / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/📂️open.rs"
edit(shell, "use super::*;", """use super::*;
#[path = "📤️first-archive.rs"]
mod first_archive;""")
add(shell.with_name("📤️first-archive.rs"), "native-first-archive-drive.rs")
edit(shell, "    let mut opened = if declaration.natural_pdf_source().is_some() {\n        let (file, extent) = open_selected_file(&path, u32::MAX as u64)?;",
     """    let maximum_bytes = declaration.natural_file_payload_limit().await?;
    let (file, extent) = open_selected_file(&path, u32::MAX as u64)?;
    let mut completion = None;
    let mut opened = if let Some(source_decl) = declaration.natural_pdf_source().filter(|_| extent > maximum_bytes as u64) {""")
edit(shell, "        if imported.is_none() { return Ok(None); }", """        let Some(imported) = imported else { return Ok(None); };
        completion = Some((semio_framework::kernel::ActorInstanceOpenPurpose::NaturalSource {
            declaration: source_decl, extent: extent as u32,
        }, imported));""")
edit(shell, "        let maximum_bytes = declaration.natural_file_payload_limit().await?;\n        let Some(bytes) = read_selected_file(&path, maximum_bytes,",
     "        let Some(bytes) = read_selected_file(file, extent, maximum_bytes,")
edit(shell, "async fn read_selected_file(path: &Path, maximum_bytes: usize, cancelled: impl Fn() -> bool) -> Result<Option<Vec<u8>>, String> {\n    let (file, extent) = open_selected_file(path, maximum_bytes as u64)?;",
     """async fn read_selected_file(file: std::fs::File, extent: u64, maximum_bytes: usize, cancelled: impl Fn() -> bool) -> Result<Option<Vec<u8>>, String> {
    if extent > maximum_bytes as u64 { return Err("Open File has an inadmissible input extent".into()); }""")
edit(shell, '    let archive = opened.program.read_app_document_archive(instance).await.map_err(|error| error.to_string())?;\n', '')
edit(shell, '''    let encoded = protocol::encode_document_archive_bytes(&archive).map_err(|error| error.to_string())?;
    if cancel.is_cancelled_now() || opened.program.read_app_document_identity(instance).await.map_err(|error| error.to_string())? != owner {
        return Ok(None);
    }
    if cancel.is_cancelled_now() { return Ok(None); }
    let storage = store_sync::sync::FolderEventLogStorage::new(opened.folder.clone());
    storage.create_archive_if_absent(document, schema, &encoded).await.map_err(|error| error.to_string())?;
    let readback = storage.archive_matches(document, &encoded, || cancel.is_cancelled_now());
    if cancel.is_cancelled_now() { return Ok(None); }
    if !readback.map_err(|error| error.to_string())? { return Err("Open File could not verify the local document archive".into()); }''', '''    let storage = store_sync::sync::FolderEventLogStorage::new(opened.folder.clone());
    if let Some((purpose, accepted)) = completion {
        let archived = first_archive::persist(&storage, &purpose, &accepted, &owner, schema,
            |request, closing| opened.program.exchange_first_archive(request, (!closing).then_some(&cancel)),
            || !cancel.is_cancelled_now()).await?;
        if archived.as_ref() != Some(&owner) || cancel.is_cancelled_now()
            || opened.program.read_app_document_identity(instance).await.map_err(|error| error.to_string())? != owner {
            return Ok(None);
        }
    } else {
        let archive = opened.program.read_app_document_archive(instance).await.map_err(|error| error.to_string())?;
        let encoded = protocol::encode_document_archive_bytes(&archive).map_err(|error| error.to_string())?;
        if cancel.is_cancelled_now() || opened.program.read_app_document_identity(instance).await.map_err(|error| error.to_string())? != owner {
            return Ok(None);
        }
        storage.create_archive_if_absent(document, schema, &encoded).await.map_err(|error| error.to_string())?;
        let readback = storage.archive_matches(document, &encoded, || cancel.is_cancelled_now());
        if cancel.is_cancelled_now() { return Ok(None); }
        if !readback.map_err(|error| error.to_string())? { return Err("Open File could not verify the local document archive".into()); }
    }''')

history = os / "📡️spr/📜️history/🦀️.rs"
edit(history, "pub use semio_os_history_record_decode::*;", '''pub use semio_os_history_record_decode::*;
#[path = "🎟️first-genesis.rs"]
mod first_genesis;
pub use first_genesis::FirstGenesisHistory;''')
add(history.with_name("🎟️first-genesis.rs"), "native-first-genesis-history.rs")

store_text = os / "🏪️store/🚪️io/📝️text/🦀️.rs"
edit(store_text, "pub use first_document_archive::FirstGenesisArchive;", '''pub use first_document_archive::FirstGenesisArchive;
use first_document_archive::FirstGenesisArchiveCursor;
#[path = "📦️document-spr-pack-and-history-composition/📤️first-archive.rs"]
mod mounted_first_archive;
pub use mounted_first_archive::MountedFirstArchive;''')
add(store_text.parent / "📦️document-spr-pack-and-history-composition/📤️first-archive.rs",
    "native-first-archive-mounted-owner.rs")

media = instance / "📄️docs/📥️staging/🧱️components/🦀️.rs"
edit(media, "pub struct MediaTransfers<A: ArtifactApp> {", '''pub struct MediaTransfers<A: ArtifactApp> {
    pub first_archive: store::os_store::text_format::MountedFirstArchive,''')
edit(media, "            original_picker:None,", '''            original_picker:None,
            first_archive: Default::default(),''')
edit(media, "        pending_reserved_producer_permit = ManuallyDrop::new(None),",
     "        first_archive = Default::default(),\n        pending_reserved_producer_permit = ManuallyDrop::new(None),")

edit(app_impl, "    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {", '''
    fn first_genesis_archive_turn(
        &mut self, source: &semio_plugin_decl_media::original_media::natural_source::NaturalFileSourceGuest,
        request: semio_os_kernel::channel::FirstArchiveRequest, output: &mut Vec<u8>, cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        self.advance_first_archive(source, request, output, cx)
    }

    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.media.first_archive.has_resident() { return Ok(self.media.first_archive.retirement_demand()); }'''.lstrip("\n"))
edit(app_impl, "        if self.constructor.has_constructor_receipt(){return Ok(PluginLifecycleStep::AwaitingInput",
     '''        if self.media.first_archive.has_resident() {
            let demand = self.media.first_archive.retirement_demand();
            return semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx, loan, demand, |same, _| {
                let before = same.retained_progress();
                same.with_original_retirement(|same| self.media.first_archive.retire(same))?;
                let progress = semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before, same.retained_progress())?;
                Ok((PluginLifecycleStep::Progress(progress), false))
            });
        }
        if self.constructor.has_constructor_receipt(){return Ok(PluginLifecycleStep::AwaitingInput''')
terminal = instance / "🔌️plugin-app-impl/🔒️close-terminal/🦀️.rs"
edit(terminal, "    pub(super) fn close_terminal_is_empty(&self) -> bool {",
     '''    pub(super) fn close_terminal_is_empty(&self) -> bool {
        if self.media.first_archive.has_resident() { return false; }''')

patch = []
for path, (base, updated) in changes.items():
    relative = path.relative_to(root)
    patch.extend(unified_diff(base.splitlines(True), updated.splitlines(True), f"a/{relative}" if base else "/dev/null", f"b/{relative}"))
(scope / "first-archive-mounted.patch").write_text("".join(patch))
