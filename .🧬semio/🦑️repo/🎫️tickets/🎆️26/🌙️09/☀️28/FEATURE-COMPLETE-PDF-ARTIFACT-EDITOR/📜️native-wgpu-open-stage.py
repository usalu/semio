"""📂 Stages the native WGPU natural-file opening route against installed Save."""

from hashlib import sha256
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUT = TICKET / "🗑️generated/native-wgpu-save/open"
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
BRIDGE = ENGINE / "🌉️ProgramBridge/🎯️targets/🧊️wgpu"
SHELL = ENGINE / "🐚️Shell/🎯️targets/🧊️wgpu/📥️staging"
SAVE = BRIDGE / "🧳️wasm-program-exchange/💾️save"
DOC_OPEN = SHELL / "📂️doc-open"
ACTIONS = SHELL / "🔀️actions"
DISPATCH = ACTIONS / "🎬️shell-actions/🎯️dispatch-action"
SETTLE = ACTIONS / "📨️command-dispatch/📇️directory-and-hub/🦀️.rs"
WINDOW = SHELL / "🪝️window-actions/🧰️window-actions-and-commands/🦀️.rs"


def change(path, *pairs):
    """🔎 Verifies each insertion anchor occurs once in its live owner."""
    source = path.read_text(encoding="utf8")
    for before, after in pairs:
        assert source.count(before) == 1, (path, before)
        source = source.replace(before, after)
    return source


def source(name):
    """📄 Loads the exact candidate to be inserted."""
    return (TICKET / f"📜️native-wgpu-open-{name}.rs").read_text(encoding="utf8")


def stage():
    """🧩 Assembles the same window route, worker, and existing document attach pump."""
    root = BRIDGE / "🦀️.rs"
    yield root, change(root, ("pub use native_save::{NativeMediaDeclaration, NativeSaveOutcome};",
        "pub use native_save::{NATIVE_DOCUMENT_FILE_MAXIMUM, NativeMediaDeclaration, NativeSaveOutcome};"))
    root = SAVE / "🦀️.rs"
    yield root, change(root, ("#[path = \"📎️reference.rs\"] mod reference;", """#[path = "📎️reference.rs"] mod reference;
#[path = "📂️open.rs"] mod open;
pub use open::NATIVE_DOCUMENT_FILE_MAXIMUM;
pub(crate) use open::import_media;"""))
    yield SAVE / "📂️open.rs", source("exchange")
    entry = BRIDGE / "🌉️program-bridge-entry/🦀️.rs"
    yield entry, change(entry, ("    pub async fn read_app_document_archive(", """    /// 📥 Imports natural binary input through the mounted program's typed media door.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn import_native_file(
        &self, instance: u32, declaration: &NativeMediaDeclaration, bytes: Vec<u8>,
    ) -> Result<(), String> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        native_save::import_media(client, instance, declaration, bytes).await
    }

    pub async fn read_app_document_archive("""))
    doc = DOC_OPEN / "🎨️theme-file-import-and-accessibility"
    yield doc / "📂️open.rs", source("worker")
    file = doc / "🦀️.rs"
    yield file, change(file, ("pub use native_media_save::NativeMediaSaveTask;", """pub use native_media_save::NativeMediaSaveTask;
#[cfg(not(target_arch = "wasm32"))]
#[path = "📂️open.rs"]
mod native_file_open;
#[cfg(not(target_arch = "wasm32"))]
pub use native_file_open::{native_local_documents, NativeOpenedFile};"""))
    identity = DOC_OPEN / "🔖️identity-and-types/🦀️.rs"
    yield identity, change(identity, ("    Instantiated(u32),", """    Instantiated(u32),
    #[cfg(not(target_arch = "wasm32"))]
    NativeFile(Option<NativeOpenedFile>),"""))
    native = DISPATCH / "📂️open.rs"
    yield native, source("shell")
    action = DISPATCH / "🦀️.rs"
    yield action, change(action,
        ("mod plugins_chat;", """mod plugins_chat;

#[cfg(not(target_arch = "wasm32"))]
#[path = "📂️open.rs"]
mod native_file_open;"""),
        ('            #[cfg(not(target_arch = "wasm32"))]\n            if matches!(action.action.as_str(), semio_framework::manifest::EXPORT_MEDIA_FILE_ACTION_ID',
         '''            #[cfg(not(target_arch = "wasm32"))]
            if action.action == semio_framework::OPEN_ARTIFACT_FILE_ACTION_ID {
                return self.start_native_open(&action);
            }
            #[cfg(not(target_arch = "wasm32"))]
            if matches!(action.action.as_str(), semio_framework::manifest::EXPORT_MEDIA_FILE_ACTION_ID'''))
    yield WINDOW, change(WINDOW, ('                    let native_save = cfg!(not(target_arch = "wasm32")) && action.id == semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID;',
        '''                    let native_file = cfg!(not(target_arch = "wasm32")) &&
                        matches!(action.id.as_str(), semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID | semio_framework::OPEN_ARTIFACT_FILE_ACTION_ID);'''),
        ('                    let staged = !native_save && action_requires_staged_form(action);', '                    let staged = !native_file && action_requires_staged_form(action);'),
        ('                    let dispatch = if native_save {', '                    let dispatch = if native_file {'),
        ('                    let disabled = gated && !native_save &&', '                    let disabled = gated && !native_file &&'))
    attach = ACTIONS / "🎬️shell-actions/📎️sync-attach-and-open/🦀️.rs"
    yield attach, change(attach,
        ("        let binding_generation = self.sync.mint_sync_binding_generation()?;\n", ""),
        ("        let actor = self.directory.current_shell_actor(session.instance_id);",
         "        let binding_generation = self.sync.mint_sync_binding_generation()?;\n        let actor = self.directory.current_shell_actor(session.instance_id);"))
    yield SETTLE, change(SETTLE, ('            Ok(ShellDocumentOpenAnswer::Instantiated(instance_id)) => {', '''            #[cfg(not(target_arch = "wasm32"))]
            Ok(ShellDocumentOpenAnswer::NativeFile(imported)) => {
                if opening.cancel_requested || imported.is_none() {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.opening.document_opening = Some(opening);
                } else if let Err(error) = self.finish_native_open(&opening.app_id, imported.expect("checked imported file")).await {
                    self.opening.document_opening = Some(opening);
                    self.opening.fail_document_opening(error, &mut self.chrome, &self.prefs);
                }
            }
            Ok(ShellDocumentOpenAnswer::Instantiated(instance_id)) => {'''))


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    entries = []
    for index, (path, content) in enumerate(stage()):
        assert len(str(path).encode("utf-16-le")) // 2 <= 256, path
        target = OUT / f"{index:02}{path.suffix}"
        target.write_text(content, encoding="utf8")
        entries.append({"file": path.relative_to(ROOT).as_posix(), "source": target.name,
            "before": sha256(path.read_bytes()).hexdigest() if path.is_file() else None,
            "after": sha256(target.read_bytes()).hexdigest()})
    (OUT / "receipt.json").write_text(json.dumps(entries, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] staged {len(entries)} native WGPU Open paths")
