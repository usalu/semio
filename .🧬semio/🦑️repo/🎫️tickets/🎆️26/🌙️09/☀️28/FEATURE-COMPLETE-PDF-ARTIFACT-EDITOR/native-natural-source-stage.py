"""📦 Stage the native natural-source sender and its exact launch crossing."""
from difflib import unified_diff
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
engine = next(root.glob("🧰*/🛍*/💻*/🔨*/📺*/🧑*"))
target = engine / "🎯️targets/🧊️wgpu"
runtime = target / "🧊️renderer/📦️kernel-runtime"
core = runtime / "🧮️kernel-core"
bridge = engine / "🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu"
shell = engine / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging"
generated = ticket / "🗑️generated/native-natural-source"
generated.mkdir(parents=True, exist_ok=True)
patch = []
changes = {}


def replace(path, *replacements):
    original = path.read_text()
    revised = original
    for old, new in replacements:
        assert revised.count(old) == 1, (path, old)
        revised = revised.replace(old, new)
    changes[path] = original, revised
    path = path.relative_to(root)
    patch.extend(unified_diff(original.splitlines(True), revised.splitlines(True), f"a/{path}", f"b/{path}"))
    return revised


pump = (ticket / "native-natural-source-pump.rs").read_text()
pump_path = bridge / "🎞️original-media/📂️natural-source.rs"
pump_path = pump_path.relative_to(root)
assert len(str(root / pump_path)) <= 256
patch.extend(unified_diff([], pump.splitlines(True), "/dev/null", f"b/{pump_path}"))
changes[root / pump_path] = "", pump

replace(
    bridge / "🦀️.rs",
    ("mod native_original_media;", """mod native_original_media;
#[cfg(not(target_arch = "wasm32"))]
#[path = "🎞️original-media/📂️natural-source.rs"]
mod native_natural_source;"""),
)

replace(
    bridge / "🧳️wasm-program-exchange/💾️save/🧮️declaration.rs",
    ("    pub fn filename(&self)", """    /// 🎟️ Selects the mounted ordinary PDF binary source declaration.
    pub fn natural_pdf_source(&self) -> Option<semio_framework::kernel::NaturalSourceWireDeclaration> {
        (self.reference && self.format_kind == "s.stdio.pdf@1.7"
            && self.mime == "application/pdf" && self.extension() == Some("pdf"))
            .then(|| semio_framework::kernel::NaturalSourceWireDeclaration {
                format_kind: self.format_kind.clone(), media_type: self.mime.clone(),
                extension: ".pdf".to_owned(), binary: true,
            })
    }

    pub fn filename(&self)"""),
)

replace(
    bridge / "🌉️program-bridge-entry/🦀️.rs",
    ("    /// 📤 Delivers one selected source", """    /// 📂 Launches one fresh instance for the selected extent and streams its source after issuance.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn import_native_pdf_source(
        &self, app_id: &str, declaration: &NativeMediaDeclaration, source: std::fs::File,
        extent: u32, cancel: &semio_framework_async::CancelToken, progress: impl FnMut(u64, u64),
        mut retain_instance: impl FnMut(u32),
    ) -> Result<Option<semio_framework::kernel::NaturalSourceWireAck>, String> {
        use semio_framework::kernel::{ActorInstanceOpenPurpose, NaturalSourcePrepare};
        let declaration = declaration.natural_pdf_source().ok_or("Open File requires the mounted PDF declaration")?;
        let ProgramBridgeBackend::Wasm { client, wasm_path } = &self.backend;
        if cancel.is_cancelled_now() { return Ok(None); }
        let sequence = semio_renderer_wgpu_kernel_protocol::next_seq()?;
        let instance = client.create_app_with_purpose(
            wasm_path, &self.plugin_id, app_id, self.app_document_schema(app_id),
            ActorInstanceOpenPurpose::NaturalSource { declaration: declaration.clone(), extent },
        ).await.map_err(|error| error.to_string())?;
        retain_instance(instance);
        let prepare = NaturalSourcePrepare { instance_id: instance, seq: sequence, declaration, extent };
        super::native_natural_source::stream(prepare, source, |event, closing| async move {
            let request = client.exchange_original_media(instance, event);
            let outcome = if closing {
                request.await
            } else {
                match semio_framework_async::select2(cancel.cancelled(), request).await {
                    semio_framework_async::Either::Left(()) => return Err("natural-source.cancelled".to_owned()),
                    semio_framework_async::Either::Right(outcome) => outcome,
                }
            }.map_err(|error| error.to_string())?;
            wasm_program_exchange::observe_ephemeral(instance, &outcome.frames).await;
            Ok(outcome.command_ingress)
        }, || !cancel.is_cancelled_now(), progress).await
    }

    /// 📤 Delivers one selected source"""),
)

queue = replace(
    core / "📥️original-media.rs",
    ("    OriginalMediaWireControl, COMMAND_MAXIMUM_BYTES, COMMAND_PAGE_MAXIMUM_BYTES, ORIGINAL_MEDIA_COMMAND_KIND};",
     """    NaturalSourceControlEvent, NaturalSourcePrepare, NaturalSourceWireDeclaration,
    OriginalMediaWireControl, COMMAND_MAXIMUM_BYTES, COMMAND_PAGE_MAXIMUM_BYTES,
    NATURAL_SOURCE_COMMAND_KIND, ORIGINAL_MEDIA_COMMAND_KIND};"""),
    ("    Control(OriginalMediaControlEvent),", """    Control(OriginalMediaControlEvent),
    NaturalPrepare(NaturalSourcePrepare),
    NaturalControl(NaturalSourceControlEvent),"""),
    ("            Event::CommandIngressPage { cursor, bytes } if cursor.kind", """            Event::NaturalSourcePrepare(prepare) if prepare.instance_id != 0 && prepare.seq != 0
                && valid_natural_declaration(&prepare.declaration, prepare.extent) => Ok(Self::NaturalPrepare(prepare)),
            Event::NaturalSourceControl(control) if valid_natural_control(&control) => Ok(Self::NaturalControl(control)),
            Event::CommandIngressPage { cursor, bytes } if cursor.kind == NATURAL_SOURCE_COMMAND_KIND
                && cursor.instance != 0 && cursor.owner == u64::from(cursor.instance) && cursor.generation != 0
                && cursor.seq != 0 && cursor.command_index == 0 && cursor.command_count == 1 && cursor.item_count == 1
                && cursor.page_count == cursor.metadata.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES as u32)
                && cursor.page_index < cursor.page_count
                && bytes.len() == (cursor.metadata as usize - cursor.page_index as usize * COMMAND_PAGE_MAXIMUM_BYTES).min(COMMAND_PAGE_MAXIMUM_BYTES) =>
                Ok(Self::Page(cursor, bytes)),
            Event::CommandIngressPage { cursor, bytes } if cursor.kind"""),
    ("            Self::Page(cursor, bytes) => Event::CommandIngressPage", """            Self::NaturalPrepare(prepare) => Event::NaturalSourcePrepare(prepare),
            Self::NaturalControl(control) => Event::NaturalSourceControl(control),
            Self::Page(cursor, bytes) => Event::CommandIngressPage"""),
    ("            Self::Page(..) => (1, COMMAND_PAGE_MAXIMUM_BYTES),", """            Self::NaturalPrepare(prepare) => (0, natural_declaration_bytes(&prepare.declaration)),
            Self::NaturalControl(control) => (0, natural_control_bytes(control)),
            Self::Page(..) => (1, COMMAND_PAGE_MAXIMUM_BYTES),"""),
    ("fn valid_control", """pub(super) fn natural_declaration_bytes(declaration: &NaturalSourceWireDeclaration) -> usize {
    declaration.format_kind.capacity() + declaration.extension.capacity() + declaration.media_type.capacity()
}

pub(super) fn valid_natural_declaration(declaration: &NaturalSourceWireDeclaration, extent: u32) -> bool {
    extent != 0 && declaration.binary && !declaration.format_kind.is_empty()
        && declaration.extension.strip_prefix('.').is_some_and(|extension| !extension.is_empty())
        && !declaration.media_type.is_empty()
        && natural_declaration_bytes(declaration) <= COMMAND_PAGE_MAXIMUM_BYTES - 32
}

fn valid_natural_control(control: &NaturalSourceControlEvent) -> bool {
    let loan = &control.loan;
    let valid_hex = |value: &str| value.len() == 16 && value != "0000000000000000"
        && value.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
    control.kind == NATURAL_SOURCE_COMMAND_KIND && control.seq != 0 && loan.instance_id != 0
        && control.owner == u64::from(loan.instance_id) && control.extent == loan.extent
        && valid_natural_declaration(&loan.declaration, loan.extent)
        && valid_hex(&loan.operation_id) && valid_hex(&loan.generation)
        && natural_control_bytes(control) <= COMMAND_PAGE_MAXIMUM_BYTES
}

fn natural_control_bytes(control: &NaturalSourceControlEvent) -> usize {
    let loan = &control.loan;
    natural_declaration_bytes(&loan.declaration) + loan.operation_id.capacity() + loan.generation.capacity()
}

fn valid_control"""),
)
(generated / "queue.rs").write_text(queue)

replace(
    core / "🦀️.rs",
    ("use queued_original_media::QueuedOriginalMedia;", """use queued_original_media::QueuedOriginalMedia;
use semio_framework::kernel::ActorInstanceOpenPurpose;

/// 🎟️ Checks bounded launch intent metadata before accepting a CreateApp request.
pub fn valid_instance_open_purpose(purpose: &ActorInstanceOpenPurpose) -> bool {
    match purpose {
        ActorInstanceOpenPurpose::Ordinary => true,
        ActorInstanceOpenPurpose::NaturalSource { declaration, extent } =>
            queued_original_media::valid_natural_declaration(declaration, *extent),
    }
}"""),
    ("            Event::OriginalMediaControl(control) => control.loan.instance_id == instance,", """            Event::OriginalMediaControl(control) => control.loan.instance_id == instance,
            Event::NaturalSourcePrepare(prepare) => prepare.instance_id == instance,
            Event::NaturalSourceControl(control) => control.loan.instance_id == instance,"""),
    ("        owner: CreateAppRequestOwner,\n    },", """        owner: CreateAppRequestOwner,
        purpose: ActorInstanceOpenPurpose,
    },"""),
    ("            Self::CreateApp { .. } => (0, 0),", """            Self::CreateApp { purpose, .. } => (0, match purpose {
                ActorInstanceOpenPurpose::Ordinary => 0,
                ActorInstanceOpenPurpose::NaturalSource { declaration, .. } =>
                    queued_original_media::natural_declaration_bytes(declaration),
            }),"""),
)

replace(
    runtime / "📬️kernel-replay-client/🦀️.rs",
    ("        let owner=open_create_app_source", """        self.create_app_with_purpose(wasm_path, plugin_id, app_id, artifact_schema,
            semio_framework::kernel::ActorInstanceOpenPurpose::Ordinary).await
    }

    /// 🎟️ Carries one publication purpose through native instance admission.
    pub async fn create_app_with_purpose(
        &self, wasm_path: &std::path::Path, plugin_id: &str, app_id: &str, artifact_schema: &str,
        purpose: semio_framework::kernel::ActorInstanceOpenPurpose,
    ) -> Result<u32, ProgramFault> {
        if !semio_renderer_wgpu_kernel_protocol::valid_instance_open_purpose(&purpose) {
            return Err("kernel: invalid instance open purpose".into());
        }
        let owner=open_create_app_source"""),
    ("KernelRequest::CreateApp { owner }", "KernelRequest::CreateApp { owner, purpose }"),
)

replace(
    runtime / "🏊️kernel-pool/🚀️turn-dispatch-and-pool-runner/🦀️.rs",
    ("KernelRequest::CreateApp { owner } => {\n                KernelOutcome::Created(self.create_app(owner).await)",
     "KernelRequest::CreateApp { owner, purpose } => {\n                KernelOutcome::Created(self.create_app(owner, purpose).await)"),
)

replace(
    runtime / "🏊️kernel-pool/🧩️app-lifecycle-and-realm-close/🦀️.rs",
    ("pub(super) async fn create_app(&mut self, owner:CreateAppRequestOwner) -> Result<u32, ProgramFault> {",
     """pub(super) async fn create_app(
        &mut self, owner: CreateAppRequestOwner, purpose: semio_framework::kernel::ActorInstanceOpenPurpose,
    ) -> Result<u32, ProgramFault> {"""),
    ("        let open = Event::InstanceOpen {", "        let open = Event::InstanceOpen {\n            purpose,"),
)

replace(
    target / "📡️kernel-protocol/📥️lifted/🏊️kernel-pool/🦀️.rs",
    ("        let (terminal, processed, released, page_released) = match", """        let (request_pages, request_bytes) = state.slots[index].as_ref().expect("fixed shutdown request slot is occupied").0.command_credits();
        let (terminal, processed, released, page_released) = match"""),
    ("""            KernelRequest::CreateApp { owner } => {
                owner.begin_close();
                let terminal=owner.terminal_is_empty();
                (terminal,usize::from(terminal),0,0)
            }""", """            KernelRequest::CreateApp { owner, .. } => {
                owner.begin_close();
                let terminal = owner.terminal_is_empty() && request_bytes <= maximum_bytes;
                (terminal, usize::from(terminal), if terminal { request_bytes } else { 0 }, 0)
            }"""),
    ("                (terminal, processed, released, 0)\n            }\n            KernelRequest::AdvanceRetained",
     "                (terminal, processed, released, if terminal { request_pages } else { 0 })\n            }\n            KernelRequest::AdvanceRetained"),
)

replace(
    engine / "🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🔒️close-registry-and-queue-shutdown/🦀️.rs",
    ('"schema".to_string()) }, Arc::new(ResponseSlot::default()), None)',
     '"schema".to_string()), purpose: semio_framework::kernel::ActorInstanceOpenPurpose::Ordinary }, Arc::new(ResponseSlot::default()), None)'),
)

replace(
    shell / "📂️doc-open/🎨️theme-file-import-and-accessibility/📂️open.rs",
    ("use std::{io::Read, path::{Path, PathBuf}};", """use std::{io::Read, path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicU8, Ordering}}};"""),
    ("        let pending = ShellDetached::spawn", """        let progress = Arc::new(AtomicU8::new(0));
        let worker_progress = progress.clone();
        let pending = ShellDetached::spawn"""),
    ("folder, worker_cancel).await", "folder, worker_cancel, worker_progress).await"),
    ("        opening.cancel = Some(cancel);", "        opening.cancel = Some(cancel);\n        opening.native_transfer_progress = progress;"),
    ("    folder: PathBuf, cancel: CancelToken,", "    folder: PathBuf, cancel: CancelToken, progress: Arc<AtomicU8>,"),
    ("""    let maximum_bytes = declaration.natural_file_payload_limit().await?;
    let Some(bytes) = read_selected_file(&path, maximum_bytes, || cancel.is_cancelled_now()).await? else { return Ok(None); };
    let instance = program.create_app(&app).await.map_err(|error| error.to_string())?;
    let mut opened = NativeOpenedFile { program, source_instance, instance, document_id: String::new(), folder, published: false };
    if cancel.is_cancelled_now() { return Ok(None); }
    if !wait_native_file_import(&cancel, opened.program.import_native_file(instance, &declaration, bytes)).await? {
        return Ok(None);
    }""", """    let mut opened = if declaration.natural_pdf_source().is_some() {
        let (file, extent) = open_selected_file(&path, u32::MAX as u64)?;
        let mut retained = None;
        let imported = program.import_native_pdf_source(
            &app, &declaration, file, extent as u32, &cancel,
            |done, total| progress.store((done * 100 / total) as u8 + 1, Ordering::Release),
            |instance| retained = Some(NativeOpenedFile {
                program: program.clone(), source_instance, instance, document_id: String::new(),
                folder: folder.clone(), published: false,
            }),
        ).await?;
        if imported.is_none() { return Ok(None); }
        retained.ok_or("Open File lost the natural source instance")?
    } else {
        let maximum_bytes = declaration.natural_file_payload_limit().await?;
        let Some(bytes) = read_selected_file(&path, maximum_bytes, || cancel.is_cancelled_now()).await? else { return Ok(None); };
        let instance = program.create_app(&app).await.map_err(|error| error.to_string())?;
        let opened = NativeOpenedFile { program, source_instance, instance, document_id: String::new(), folder, published: false };
        if cancel.is_cancelled_now() || !wait_native_file_import(&cancel, opened.program.import_native_file(instance, &declaration, bytes)).await? {
            return Ok(None);
        }
        opened
    };
    let instance = opened.instance;"""),
    ("""async fn read_selected_file(path: &Path, maximum_bytes: usize, cancelled: impl Fn() -> bool) -> Result<Option<Vec<u8>>, String> {
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > maximum_bytes as u64 {
        return Err("Open File has an inadmissible input extent".into());
    }
    let mut input = file.take(maximum_bytes as u64 + 1);
    let mut data = Vec::with_capacity(metadata.len() as usize);""", """fn open_selected_file(path: &Path, maximum_bytes: u64) -> Result<(std::fs::File, u64), String> {
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > maximum_bytes {
        return Err("Open File has an inadmissible input extent".into());
    }
    Ok((file, metadata.len()))
}

async fn read_selected_file(path: &Path, maximum_bytes: usize, cancelled: impl Fn() -> bool) -> Result<Option<Vec<u8>>, String> {
    let (file, extent) = open_selected_file(path, maximum_bytes as u64)?;
    let mut input = file.take(maximum_bytes as u64 + 1);
    let mut data = Vec::with_capacity(extent as usize);"""),
    ("data.len() as u64 != metadata.len()", "data.len() as u64 != extent"),
)

replace(
    shell / "📂️doc-open/🔖️identity-and-types/🦀️.rs",
    ("    pub resolve_step: std::sync::Arc<std::sync::atomic::AtomicU8>,", """    pub resolve_step: std::sync::Arc<std::sync::atomic::AtomicU8>,
    pub native_transfer_progress: std::sync::Arc<std::sync::atomic::AtomicU8>,
    pub observed_native_transfer_progress: u8,"""),
    ("resolve_step: std::sync::Arc::default() }", """resolve_step: std::sync::Arc::default(),
            native_transfer_progress: std::sync::Arc::default(), observed_native_transfer_progress: 0 }"""),
)

replace(
    shell / "🔀️actions/📨️command-dispatch/📇️directory-and-hub/🦀️.rs",
    ("""        let Some(answer) = self.opening.document_opening.as_ref().and_then(|opening| opening.pending.as_ref()).and_then(ShellDetached::take) else { return false };""", """        let Some(opening) = self.opening.document_opening.as_mut() else { return false; };
        let Some(answer) = opening.pending.as_ref().and_then(ShellDetached::take) else {
            let progress = opening.native_transfer_progress.load(std::sync::atomic::Ordering::Acquire);
            return std::mem::replace(&mut opening.observed_native_transfer_progress, progress) != progress;
        };"""),
)

replace(
    shell / "🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs",
    ('        _ => format!("{} {}", shell_chrome_string(key, is_de), opening.label),', """        ShellDocumentOpenPhase::Instantiating if opening.observed_native_transfer_progress != 0 =>
            format!("{} {} · {}%", shell_chrome_string(key, is_de), opening.label, opening.observed_native_transfer_progress - 1),
        _ => format!("{} {}", shell_chrome_string(key, is_de), opening.label),"""),
)

(generated / "native-natural-source.patch").write_text("".join(patch))
