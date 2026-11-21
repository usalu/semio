"""🧰 Stages the checked Save path on the native WGPU bridge and shell without changing active build inputs."""

from hashlib import sha256
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUT = TICKET / "🗑️generated/native-wgpu-save/bridge"
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
BRIDGE = ENGINE / "🌉️ProgramBridge/🎯️targets/🧊️wgpu"
SHELL = ENGINE / "🐚️Shell/🎯️targets/🧊️wgpu/📥️staging"
EXCHANGE = BRIDGE / "🧳️wasm-program-exchange"
SAVE = EXCHANGE / "💾️save"
OPEN = SHELL / "📂️doc-open/🎨️theme-file-import-and-accessibility"
ACTION = SHELL / "🔀️actions/🎬️shell-actions/🎯️dispatch-action/🦀️.rs"


def rename(source, before, after):
    """📌 Match every current source anchor exactly."""
    if source.count(before) != 1:
        raise ValueError(f"Native Save guard drifted: {before}")
    return source.replace(before, after)


def read(path):
    """📖 Load the current canonical input as UTF-8."""
    return path.read_text(encoding="utf8")


def candidate_driver():
    """🛡️ Make terminal and cancellation semantics explicit at the original transport."""
    s = read(TICKET / "📜️native-wgpu-save-driver.rs")
    s = rename(s, "pub(super) enum NativeSaveOutcome", "pub enum NativeSaveOutcome")
    s = rename(s, "const MAX_DESCRIPTOR:", "pub(super) const MAX_DESCRIPTOR:")
    s = rename(s, "    pub media_type: semio_framework::MediaType,", "    pub media_type: semio_framework::MediaType,\n    pub reference: bool,")
    s = rename(s, "    if !current() { return Ok(NativeSaveOutcome::Cancelled); }\n    let document =", """    let mut observed = 0;
    let mut progress = |completed| {
        observed = u64::max(observed, completed);
        progress(observed);
    };
    if !current() { return Ok(NativeSaveOutcome::Cancelled); }
    let document =""")
    s = rename(s, "    if settled { return result; }", """    if settled {
        if matches!(result, Ok(NativeSaveOutcome::ReferenceRequired)) && format.reference {
            return super::reference::save_reference(peer, owner, format, path, &current, &mut progress).await;
        }
        return result;
    }""")
    s = rename(s, "let maximum_pages = MAX_TOTAL / CHUNK;", "let maximum_pages = MAX_TOTAL.div_ceil(CHUNK);")
    s = rename(s, "if data.len() > CHUNK || page == maximum_pages && (!terminal || !data.is_empty()) || !terminal && data.is_empty() {",
                   "if data.len() > CHUNK || terminal != data.is_empty() || page == maximum_pages && !terminal {")
    s = rename(s, """        if current() && refusal.is_none() && !data.is_empty()
            && let Some(active) = sink.as_mut()
            && let Err(error) = active.write_page(&data)
        {
            refusal = Some(error.to_string());
        }""", """        if current() && refusal.is_none() && !data.is_empty() {
            let active = sink.as_mut().ok_or("Save lost its staged destination")?;
            if let Err(error) = active.write_page(&data) { refusal = Some(error.to_string()); }
        }""")
    s = rename(s, """    match (result, stopped) {
        (Ok(NativeSaveOutcome::Cancelled), Ok(AppFrame::Done { in_reply_to })) if in_reply_to == seq => Ok(NativeSaveOutcome::Cancelled),
        (Ok(NativeSaveOutcome::Cancelled), _) => Err("Save cancellation did not settle its operation".into()),
        (result, _) => result,
    }""", """    if !matches!(stopped, Ok(AppFrame::Done { in_reply_to }) if in_reply_to == seq) {
        return match result {
            Err(reason) => Err(format!("Save cancellation did not settle its operation ({reason})")),
            _ => Err("Save cancellation did not settle its operation".into()),
        };
    }
    result""")
    s += """
/// 🧷 Admits one exact Save reply from the KernelClient's decoded frame batch.
pub(super) fn one_save_reply(
    frames: Vec<AppFrame>, sequence: u64, fault: impl FnOnce(&[u8], &[u8]) -> String,
) -> Result<AppFrame, String> {
    let mut replies = frames.into_iter().filter(|frame| matches!(frame,
        AppFrame::MediaExportSubmitted { .. } | AppFrame::MediaExportStatus { .. } |
        AppFrame::MediaExportChunk { .. } | AppFrame::Media { .. } | AppFrame::Done { .. } | AppFrame::Error { .. }));
    let answer = replies.next().ok_or("native Save received no protocol answer")?;
    if replies.next().is_some() { return Err("native Save received multiple protocol answers".into()); }
    match &answer {
        AppFrame::Error { in_reply_to: Some(seq), fault: body, report } if *seq == sequence => Err(fault(body, report)),
        AppFrame::MediaExportSubmitted { in_reply_to, .. } | AppFrame::MediaExportStatus { in_reply_to, .. } |
        AppFrame::MediaExportChunk { in_reply_to, .. } | AppFrame::Media { in_reply_to, .. } | AppFrame::Done { in_reply_to }
        if *in_reply_to == sequence => Ok(answer),
        _ => Err("native Save received a foreign protocol answer".into()),
    }
}
"""
    return s


def production_changes():
    """🗺️ Keep the shared action and invocation owners while adding one native-only host path."""
    root = BRIDGE / "🦀️.rs"
    yield root, rename(read(root), "use wasm_program_exchange_child::wasm_program_exchange;", """use wasm_program_exchange_child::wasm_program_exchange;
#[cfg(not(target_arch = "wasm32"))]
use wasm_program_exchange_child::native_save;
#[cfg(not(target_arch = "wasm32"))]
pub use native_save::{NativeMediaDeclaration, NativeSaveOutcome};""")
    outer = EXCHANGE / "🦀️.rs"
    s = rename(read(outer), "pub(super) mod wasm_program_exchange {", """#[path = "💾️save/🦀️.rs"]
pub(super) mod native_save;

#[cfg(not(target_arch = "wasm32"))]
pub(super) mod wasm_program_exchange {""")
    for member in ("fn next_seq", "fn app_frame_error_message", "async fn exchange"):
        s = rename(s, "    " + member + "(", "    pub(super) " + member + "(")
    yield outer, s
    yield SAVE / "🦀️.rs", """//! 🌉 Native declared file Save over the established program exchange.
#[path = "🧮️declaration.rs"] mod declaration;
#[path = "📥️driver.rs"] mod driver;
#[path = "📤️sink.rs"] mod sink;
#[path = "📎️reference.rs"] mod reference;
#[path = "🔁️peer.rs"] mod peer;
pub use declaration::NativeMediaDeclaration;
pub use driver::NativeSaveOutcome;
pub(crate) use peer::save_media;
"""
    yield SAVE / "🧮️declaration.rs", read(TICKET / "📜️native-wgpu-save-declaration.rs")
    yield SAVE / "📥️driver.rs", candidate_driver()
    yield SAVE / "📤️sink.rs", read(TICKET / "📜️native-wgpu-save-sink.rs")
    yield SAVE / "📎️reference.rs", read(TICKET / "📜️native-wgpu-save-reference.rs")
    peer = rename(read(TICKET / "📜️native-wgpu-save-exchange.rs"),
        "use super::{app_frame_error_message, exchange, next_seq, read_app_document_identity, driver, KernelClient};",
        """use super::driver;
use super::super::{KernelClient, wasm_program_exchange::{app_frame_error_message, exchange, next_seq, read_app_document_identity}};""")
    yield SAVE / "🔁️peer.rs", rename(peer, "pub(super) async fn save_media(", "pub(crate) async fn save_media(")
    entry = BRIDGE / "🌉️program-bridge-entry/🦀️.rs"
    yield entry, rename(read(entry), "    pub async fn read_app_document_archive(", """    /// 📋 Resolves the checked export from this program's own mounted app declaration.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn declared_native_media_save(&self, app_id: &str, action_id: &str) -> Option<NativeMediaDeclaration> {
        let mut candidates = self.manifest.apps.iter().filter(|app| app.id == app_id);
        let app = candidates.next()?;
        if candidates.next().is_some() { return None; }
        NativeMediaDeclaration::for_app(app, action_id)
    }

    /// 💾 Streams one source-checked binary export to the selected native destination.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn save_native_media(
        &self, owner: &protocol::AppDocumentIdentity, declaration: &NativeMediaDeclaration, destination: &std::path::Path,
        cancelled: impl Fn() -> bool, progress: impl FnMut(u64),
    ) -> Result<NativeSaveOutcome, String> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        native_save::save_media(client, owner, declaration, destination, cancelled, progress).await
    }

    pub async fn read_app_document_archive(""")
    yield OPEN / "💾️save.rs", read(TICKET / "📜️native-wgpu-save-shell.rs")
    file = OPEN / "🦀️.rs"
    s = rename(read(file), "use super::*;\n", """use super::*;
#[cfg(not(target_arch = "wasm32"))]
#[path = "💾️save.rs"]
mod native_media_save;
#[cfg(not(target_arch = "wasm32"))]
pub use native_media_save::NativeMediaSaveTask;
""")
    s = rename(s, "        if self.import_transfers.is_empty() {", """        let idle = self.import_transfers.is_empty();
        #[cfg(not(target_arch = "wasm32"))]
        let idle = idle && self.native_media_save.is_none();
        if idle {""")
    s = rename(s, '        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("os.task-manager.tasks".into()),', '''        #[cfg(not(target_arch = "wasm32"))]
        if let Some(task) = &self.native_media_save {
            children.push(settings_text_row(&format!("{} · {} · {}", task.file, pick("File export", "Dateiexport"), task.plugin_id)));
            children.push(UiNode::Progress(semio_ui_declaration::ui::UiProgressNode {
                id: format!("os.task-manager.progress.documentTransfer:{}", task.id),
                completed: task.completed() as f64, total: None,
                value_text: Label::data(pick("Export progress", "Exportfortschritt")),
                presence: UiPresence::default(), menu: None,
            }));
            children.push(UiNode::Button(UiButtonNode {
                id: Some(format!("os.task-manager.cancel.documentTransfer:{}", task.id)),
                icon_id: IconName::Square,
                label: Label::data(if is_de { format!("{} abbrechen", task.file) } else { format!("Cancel {}", task.file) }),
                action: ActionDescriptor { controller_id: "framework".into(), action: IMPORT_TRANSFER_CANCEL_ACTION.into(),
                    args: semio_renderer_wgpu_host_base::action_args_json!({ "transfer": task.id }) },
                style: None, presence: UiPresence::default(), menu: None,
            }));
        }
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("os.task-manager.tasks".into()),''')
    yield file, s
    state = SHELL / "📂️doc-open/🧬️component/🦀️.rs"
    yield state, rename(read(state), "    pub import_transfer_next: u64,", """    pub import_transfer_next: u64,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_media_save: Option<NativeMediaSaveTask>,""")
    initialize = SHELL / "🐙️state/🧩️session-and-topology/🦀️.rs"
    yield initialize, rename(read(initialize), "                import_transfer_next: 1,", """                import_transfer_next: 1,
                #[cfg(not(target_arch = "wasm32"))]
                native_media_save: None,""")
    session = SHELL / "🚶️session/🧩️session-and-topology/🦀️.rs"
    s = rename(read(session), """        const MAX_COMPLETIONS_PER_FRAME: usize = 8;
        let mut changed = false;""", """        const MAX_COMPLETIONS_PER_FRAME: usize = 8;
        let (mut changed, completion) = self.opening.poll_media_save(&self.docs, &self.plugins);
        if let Some((file, result)) = completion {
            use semio_renderer_wgpu_program_bridge::NativeSaveOutcome;
            let local = |en: &str, de: &str| if self.prefs.locale_id == "de" { de.to_owned() } else { en.to_owned() };
            let (notice, severity, code) = match result {
                Ok(NativeSaveOutcome::Delivered) => (local("Saved", "Gespeichert"), semio_framework::Severity::Info, "exported"),
                Ok(NativeSaveOutcome::Cancelled) => (local("Save canceled", "Speichern abgebrochen"), semio_framework::Severity::Info, "export-cancelled"),
                Ok(NativeSaveOutcome::ReferenceRequired) => (local("Save needs the reference exporter", "Speichern erfordert den Referenzexport"), semio_framework::Severity::Error, "export-failed"),
                Err(why) => (why, semio_framework::Severity::Error, "export-failed"),
            };
            self.chrome.show_transient_notice(format!("{notice} · {file}"), severity, Some(code));
        }""")
    yield session, s
    actions = rename(read(ACTION), """            if !record_tutorial_after_acceptance && action.controller_id != "framework" {""", """            #[cfg(not(target_arch = "wasm32"))]
            if matches!(action.action.as_str(), semio_framework::manifest::EXPORT_MEDIA_FILE_ACTION_ID | semio_framework::manifest::EXPORT_SNAPSHOT_FILE_ACTION_ID | semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID) {
                let session = self.docs.action_session(&action, &self.plugins).ok_or("media-export.document-owner")?;
                if let Some(window_id) = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str) {
                    let spawned = window_id == "spawned" && self.plugins.space_mode && self.docs.spawned_session.as_ref().is_some_and(|current| current.instance_id == session.instance_id);
                    if !spawned && !session.view_state.window_instances.iter().any(|window| window.id == window_id) {
                        return Err("media-export.window-owner".into());
                    }
                }
                let program = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id)
                    .cloned().ok_or("media-export.program-unavailable")?;
                if let Some(declaration) = program.declared_native_media_save(&session.app.id, &action.action) {
                    if !self.opening.start_media_save(program, session.app.id.clone(), session.instance_id, declaration) {
                        return Err("media-export.transfer-busy".into());
                    }
                    return Ok(());
                }
                if action.action != semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID {
                    return Err("media-export.declaration-unavailable".into());
                }
            }
            if !record_tutorial_after_acceptance && action.controller_id != "framework" {""")
    yield ACTION, rename(actions, "                            self.as_session().cancel_import_transfer(transfer);", """                            #[cfg(not(target_arch = "wasm32"))]
                            if self.opening.cancel_media_save(transfer) { return Ok(()); }
                            self.as_session().cancel_import_transfer(transfer);""")

    window_actions = SHELL / "🪝️window-actions/🧰️window-actions-and-commands/🦀️.rs"
    s = rename(read(window_actions), '                    let staged = action_requires_staged_form(action);',
        """                    let native_save = cfg!(not(target_arch = "wasm32")) && action.id == semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID;
                    let staged = !native_save && action_requires_staged_form(action);""")
    s = rename(s, "                    let disabled = gated && !FRAMEWORK_RESERVED_ACTION_IDS.contains(&action.id.as_str());",
        "                    let disabled = gated && !native_save && !FRAMEWORK_RESERVED_ACTION_IDS.contains(&action.id.as_str());")
    yield window_actions, rename(s, '                    let dispatch = if staged {',
        """                    let dispatch = if native_save {
                        ActionDescriptor {
                            controller_id: session.app.controller_id.clone(), action: action.id.clone(),
                            args: semio_renderer_wgpu_host_base::action_args_json!({ "windowId": window_id }),
                        }
                    } else if staged {""")


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    entries = []
    for index, (path, text) in enumerate(production_changes()):
        if len(str(path).encode("utf-16-le")) // 2 > 256:
            raise ValueError(f"Path exceeds the portable envelope: {path}")
        candidate = OUT / f"{index:02}{path.suffix}"
        candidate.write_text(text, encoding="utf8")
        entries.append({"file": path.relative_to(ROOT).as_posix(), "source": candidate.name,
            "before": sha256(path.read_bytes()).hexdigest() if path.is_file() else None,
            "after": sha256(candidate.read_bytes()).hexdigest()})
    (OUT / "receipt.json").write_text(json.dumps(entries, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] staged {len(entries)} native WGPU Save paths")
