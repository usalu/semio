"""📥 Stages the bounded native WGPU original-media ingress without switching the producer."""
import hashlib
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(Path("🧰️framework").glob("🛍*/💻*/🔨*/📺*/🧑*"))
WGPU = next(ROOT.glob("🎯*/🧊*"))
PROGRAM = next(ROOT.glob("🧱*/🌉*/🎯*/🧊*"))
RUNTIME = WGPU / "🧊️renderer/📦️kernel-runtime"
OUT = TICKET / "🗑️generated/native-wgpu-save/original-picker"


def digest(body):
    """🔑 Identifies the exact working tree bytes."""
    return hashlib.sha256(body.encode()).hexdigest()


def main():
    """🧩 Prepares guarded postimages for the current kernel and bridge sources."""
    OUT.mkdir(parents=True, exist_ok=True)
    sources, preimages = {}, {}
    
    def edit(path, before, after):
        if path not in sources:
            sources[path] = path.read_text()
            preimages[path] = digest(sources[path])
        assert sources[path].count(before) == 1, (path, before)
        sources[path] = sources[path].replace(before, after)

    def add(path, origin):
        assert path not in sources and not path.exists()
        sources[path] = (TICKET / origin).read_text()
        preimages[path] = None

    core = RUNTIME / "🧮️kernel-core/🦀️.rs"
    edit(core, "    MessageBackbone,\n", "    MessageBackbone,\n    OriginalMedia,\n")
    edit(core, "#[derive(Clone, Copy)]\npub enum QueuedKernelEventKind", """#[path = "📥️original-media.rs"]
mod queued_original_media;
use queued_original_media::QueuedOriginalMedia;

#[derive(Clone, Copy)]
pub enum QueuedKernelEventKind""")
    edit(core, "    pub message_payload: Option<Vec<u8>>,\n", "    pub message_payload: Option<Vec<u8>>,\n    original_media: Option<QueuedOriginalMedia>,\n")
    for line in core.read_text().splitlines():
        if "Ok(Self { kind: Some(QueuedKernelEventKind::" in line:
            edit(core, line, line[:-3] + ", original_media: None })")
    edit(core, "    pub fn into_event(mut self) -> Event {", """    pub fn try_from_original_media(instance: u32, event: Event) -> Result<Self, RejectedKernelEvents> {
        let same_instance = match &event {
            Event::OriginalMediaControl(control) => control.loan.instance_id == instance,
            Event::CommandIngressPage { cursor, .. } => cursor.instance == instance,
            Event::Wake => instance != 0,
            _ => false,
        };
        let original_media = if same_instance { QueuedOriginalMedia::admit(event) } else { Err(event) }
            .map_err(|event| RejectedKernelEvents { events: [event].into() })?;
        Ok(Self { kind: Some(QueuedKernelEventKind::OriginalMedia), surface: None, surface_body_key: None,
            surface_view_state: None, staged_instance: None, staged_request: None, message_endpoint: None,
            message_payload: None, original_media: Some(original_media) })
    }

    pub fn into_event(mut self) -> Event {""")
    edit(core, "            QueuedKernelEventKind::SurfaceVisible => Event::SurfaceVisible {", """            QueuedKernelEventKind::OriginalMedia => self.original_media.take().expect("queued Original media is present").into_event(),
            QueuedKernelEventKind::SurfaceVisible => Event::SurfaceVisible {""")
    edit(core, "        if let Some(length) = self.surface.as_ref().map(String::len) {", """        if let Some(original_media) = self.original_media.as_ref() {
            let (_, bytes) = original_media.credits();
            if bytes > maximum_bytes { return (false, 0, 0); }
            drop(self.original_media.take());
            self.kind = None;
            return (true, 1, bytes);
        }
        if let Some(length) = self.surface.as_ref().map(String::len) {""")
    edit(core, "    fn remaining_bytes(&self) -> usize {", """    fn credits(&self) -> (usize, usize) {
        if let Some(original_media) = self.original_media.as_ref() { return original_media.credits(); }
        (0, self.remaining_bytes())
    }

    fn remaining_bytes(&self) -> usize {""")
    edit(core, "Self::Exchange { event, .. } => (0, event.remaining_bytes()),", "Self::Exchange { event, .. } => event.credits(),")
    add(core.with_name("📥️original-media.rs"), "📜️native-wgpu-original-queue.rs")
    
    client = RUNTIME / "📬️kernel-replay-client/🦀️.rs"
    edit(client, "    pub async fn advance_retained(", """    /// 🎞️ Submits one authorized original-media control, fixed page or continuation.
    pub async fn exchange_original_media(&self, instance: u32, event: Event) -> Result<ExchangeOutcome, ProgramFault> {
        let event = match QueuedKernelEvent::try_from_original_media(instance, event) {
            Ok(event) => event,
            Err(owner) => {
                self.queue.enqueue_retained(KernelRequest::CloseRejectedEvents { owner }, Arc::new(ResponseSlot::default())).await;
                return Err("kernel: invalid original-media crossing".into());
            }
        };
        match self.submit(KernelRequest::Exchange { instance, event }).await {
            KernelOutcome::Exchanged(result) => result,
            _ => Err("kernel: unexpected response for original-media exchange".into()),
        }
    }

    pub async fn advance_retained(""")
    
    bridge = PROGRAM / "🦀️.rs"
    edit(bridge, "pub use native_save::{NATIVE_DOCUMENT_FILE_MAXIMUM, NativeMediaDeclaration, NativeSaveOutcome};", """pub use native_save::{NATIVE_DOCUMENT_FILE_MAXIMUM, NativeMediaDeclaration, NativeSaveOutcome};
#[cfg(not(target_arch = "wasm32"))]
#[path = "🎞️original-media/🦀️.rs"]
mod native_original_media;
#[cfg(not(target_arch = "wasm32"))]
pub use native_original_media::NativeOriginalMediaRequest;""")
    add(bridge.with_name("🎞️original-media") / "🦀️.rs", "📜️native-wgpu-original-media.rs")
    entry = PROGRAM / "🌉️program-bridge-entry/🦀️.rs"
    edit(entry, "    pub async fn read_app_document_archive(", """    /// 📤 Delivers one selected source through the same active component and exact picker loan.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn deliver_original_media(
        &self, request: &super::NativeOriginalMediaRequest,
        source: Result<Option<(&mut impl std::io::Read, &str, &str, u64)>, String>,
        mut observe: impl AsyncFnMut(Vec<Effect>),
        current: impl Fn() -> bool, progress: impl FnMut(u64, u64),
    ) -> Result<bool, String> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        let sequence = semio_renderer_wgpu_kernel_protocol::next_seq()?;
        let instance = request.instance_id();
        super::native_original_media::NativeOriginalMediaTransfer::run(
            request, sequence, source, async |event| {
                let outcome = client.exchange_original_media(instance, event).await.map_err(|error| error.to_string())?;
                observe(outcome.effects).await;
                Ok(outcome.command_ingress)
            }, current, progress,
        ).await
    }

    pub async fn read_app_document_archive(""")
    
    exchange = PROGRAM / "🧳️wasm-program-exchange/🦀️.rs"
    edit(exchange, "    async fn observe_ephemeral(", "    pub(crate) async fn observe_ephemeral(")
    edit(entry, "                observe(outcome.effects).await;", """                wasm_program_exchange::observe_ephemeral(instance, &outcome.frames).await;
                observe(outcome.effects).await;""")

    shell = next(ROOT.glob("🧱*/🐚*/🎯*/🧊*/📥*"))
    doc_open = shell / "📂️doc-open"
    effect = shell / "🪝️window-actions/⚙️settings-and-dock-persistence/🦀️.rs"
    edit(effect, "use super::*;\n", """use super::*;
#[cfg(not(target_arch = "wasm32"))]
#[path = "🎞️original-picker.rs"]
mod original_picker;
""")
    edit(effect, "                        let controller_id = controller_id.to_string();\n                        self.settle.submit_shell_io_future(async move { ShellIoCompletion::Import(", """                        if self.queue_original_media(controller_id, args.as_ref()) { continue; }
                        let controller_id = controller_id.to_string();
                        self.settle.submit_shell_io_future(async move { ShellIoCompletion::Import(""")
    add(effect.with_name("🎞️original-picker.rs"), "📜️native-wgpu-original-effect.rs")

    task_dir = doc_open / "🎨️theme-file-import-and-accessibility"
    task_owner = task_dir / "🦀️.rs"
    edit(task_owner, "pub use native_media_save::NativeMediaSaveTask;", """pub use native_media_save::NativeMediaSaveTask;
#[cfg(not(target_arch = "wasm32"))]
#[path = "🎞️original-picker.rs"]
mod native_original_media;
#[cfg(not(target_arch = "wasm32"))]
pub use native_original_media::{NativeOriginalMediaPoll, NativeOriginalMediaTask};""")
    edit(task_owner, "let idle = idle && self.native_media_save.is_none();",
         "let idle = idle && self.native_media_save.is_none() && self.native_original_media.is_none();")
    edit(task_owner, "        if let Some(task) = &self.native_media_save {", """        if let Some(task) = &self.native_original_media {
            children.extend(native_original_media::task_nodes(task, is_de));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(task) = &self.native_media_save {""")
    add(task_dir / "🎞️original-picker.rs", "📜️native-wgpu-original-task.rs")

    state = doc_open / "🧬️component/🦀️.rs"
    edit(state, "    pub native_media_save: Option<NativeMediaSaveTask>,", """    pub native_media_save: Option<NativeMediaSaveTask>,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_original_media: Option<NativeOriginalMediaTask>,""")
    initializer = shell / "🐙️state/🧩️session-and-topology/🦀️.rs"
    edit(initializer, "                native_media_save: None,", """                native_media_save: None,
                #[cfg(not(target_arch = "wasm32"))]
                native_original_media: None,""")
    dispatcher = shell / "🔀️actions/🎬️shell-actions/🎯️dispatch-action/🦀️.rs"
    edit(dispatcher, "if self.opening.cancel_media_save(transfer) { return Ok(()); }",
         "if self.opening.cancel_media_save(transfer) || self.opening.cancel_original_media(transfer) { return Ok(()); }")
    frame = shell / "🖐️pointer/🎬️shell-actions/🦀️.rs"
    edit(frame, "let shell_io_changed = self.as_session().poll_shell_io();", """let shell_io_changed = self.as_session().poll_shell_io() | self.as_window_actions().poll_original_media().await;""")

    records = []
    for index, (path, body) in enumerate(sources.items()):
        name = f"{index:02}.rs"
        assert len(str(path).encode("utf-16-le")) // 2 <= 256, path
        assert (digest(path.read_text()) if path.exists() else None) == preimages[path], path
        (OUT / name).write_text(body)
        records.append({"file": str(path), "before": preimages[path], "source": name, "after": digest(body)})
    (OUT / "receipt.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n")
    print(f"staged {len(records)} native WGPU original-media paths")


if __name__ == "__main__":
    main()
