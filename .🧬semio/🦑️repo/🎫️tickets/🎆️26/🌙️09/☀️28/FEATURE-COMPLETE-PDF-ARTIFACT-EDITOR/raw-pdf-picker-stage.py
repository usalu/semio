"""🎞️ Stage the real retained PDF picker issuer and its authenticated effect publication."""

import hashlib
import json
from pathlib import Path
from runpy import run_path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUT = TICKET / "🗑️generated"
BASE = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor")
DOC = BASE / "📄️document/🖼️image-authoring"
PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
REACTOR = PLUGIN / "⚛️reactor"
OPS = REACTOR / "🏃️runtime/🧰️ops"
RAW = PLUGIN / "📜️declaration/🎞️media/📥️original-media"
PUB = OPS / "🎞️raw-media/📤️picker.rs"
WORK = DOC / "📂️picker-work.rs"

APP_METHODS = """    /// 🎫️ Borrows the guest-only picker at the head of the retained effect outbox.
    fn original_picker_effect(&mut self) -> Option<&mut Effect> { None }
    /// ♻️ Drops a stale picker only after the caller has admitted all of its retained backing.
    fn discard_original_picker_effect(&mut self) {}
"""
APP_IMPLEMENTATION = """    fn original_picker_effect(&mut self) -> Option<&mut Effect> {
        self.tools.typed_effect_outbox.items.front_mut().filter(|effect|
            semio_plugin_decl_media::original_media::issue::pending_picker_effect(effect))
    }

    fn discard_original_picker_effect(&mut self) {
        if self.original_picker_effect().is_some() { self.tools.typed_effect_outbox.pop(); }
    }

"""
def stage():
    run_path(str(TICKET / "raw-reactor-stage.py"))["stage"]()
    overlay = json.loads((OUT / "raw-reactor-overlay.json").read_bytes())
    preimages, sources = overlay["preimages"], overlay["candidate"]

    def replace(text, old, new):
        if text.count(old) != 1:
            raise ValueError(f"Retained picker source drifted at {old}")
        return text.replace(old, new)

    def edit(path, old, new):
        key = str(path)
        if key not in sources:
            original = (ROOT / path).read_bytes()
            preimages[key] = hashlib.sha256(original).hexdigest()
            sources[key] = original.decode()
        sources[key] = replace(sources[key], old, new)

    def add(path, file):
        if (ROOT / path).exists() or len(str(ROOT / path)) > 256:
            raise ValueError(f"Retained picker product target already exists or is too long: {path}")
        key = str(path)
        preimages[key] = None
        sources[key] = (TICKET / file).read_text(encoding="utf8")

    add(RAW / "📨️issue.rs", "raw-pdf-issue-wire.rs")
    sources[str(RAW / "🦀️.rs")] += '#[path = "📨️issue.rs"]\npub mod issue;\npub use semio_framework_replication_command_ingress::ORIGINAL_MEDIA_COMMAND_KIND;\n'
    add(PUB, "raw-pdf-issue-publish.rs")
    edit(OPS / "🎞️raw-media/🦀️.rs", "use semio_framework::kernel::{", '#[path = "📤️picker.rs"]\nmod picker;\npub use picker::continue_original_picker_effects;\npub use semio_plugin_decl_media::original_media::issue::pending_picker_effect;\n\nuse semio_framework::kernel::{')
    trait = REACTOR / "🔌️app/🦀️.rs"
    edit(trait, "    fn take_typed_operation_effect(&mut self) -> Option<Effect>;", APP_METHODS + "    fn take_typed_operation_effect(&mut self) -> Option<Effect>;")
    impl = PLUGIN / "🧵️instance/🔌️plugin-app-impl/🦀️.rs"
    edit(impl, "    fn take_typed_operation_effect(&mut self) -> Option<Effect> {\n        self.tools.typed_effect_outbox.pop()",
         APP_IMPLEMENTATION + "    fn take_typed_operation_effect(&mut self) -> Option<Effect> {\n        if self.original_picker_effect().is_some() { return None; }\n        self.tools.typed_effect_outbox.pop()")
    turn = REACTOR / "🔄️turn/🌀️poll-kernel-turn/🦀️.rs"
    edit(turn, '    trace_turn_phase_retention("ingress");', '    let picker_pending = semio_plugin_runtime_ops::ops_raw_media::continue_original_picker_effects(runtime, cx)?;\n    trace_turn_phase_retention("ingress");')
    edit(turn, "pending_effects_runnable, reconcile_work", "pending_effects_runnable: pending_effects_runnable || picker_pending, reconcile_work")
    edit(turn, "    let result = semio_framework::kernel::TurnResult {", """    if effects.iter().any(semio_plugin_runtime_ops::ops_raw_media::pending_picker_effect) {
        return Err(reactor_close_fault("unadmitted original picker reached the public effect return").into());
    }
    let result = semio_framework::kernel::TurnResult {""")
    life = REACTOR / "🚪️lifetime/🦀️.rs"
    edit(life, "        if !self.raw_media.terminal_is_empty() && (!format_closed || !self.raw_media.close(cx)?) { return Ok(false); }",
         """        if !self.raw_media.terminal_is_empty() {
            if !format_closed || self.raw_media.picker.receipt_waiting() {
                self.raw_media.picker.request_supersession(cx)?;
                return Ok(false);
            }
            if !self.raw_media.close(cx)? { return Ok(false); }
        }""")

    add(WORK, "raw-pdf-picker-work.rs")
    edit(DOC / "🦀️.rs", "mod admission;", 'mod admission;\n#[path = "📂️picker-work.rs"]\nmod picker_work;\npub use picker_work::PngPickerWork;')
    local = BASE / "📥️local-image/🦀️.rs"
    edit(local, "PngTarget, import_rows, picker, png_invocation,", "PngPickerWork, PngTarget, import_rows, png_invocation,")
    edit(local, "use semio_plugin::retained_command::{ArtifactCommandWork, BoundedArtifactCommandWork};",
         "use semio_plugin::retained_command::ArtifactCommandWork;")
    edit(local, """        Box::new(BoundedArtifactCommandWork::new(tool, |command, snapshot, _, _, _, _, _, operation| {
            let Command::Native(Pdf17EditorCommand::PickImage { page, operator, x, y, width, height, guard }) = command else {
                return Err(Fault::from("PDF image picker command mismatch"));
            };
            picker(snapshot, operation, target(*page, *operator, [*x, *y, *width, *height]), guard)
        }, |_, _, _| Some(1)))""", """        Box::new(PngPickerWork::<App>::new(tool, |command| match command {
            Command::Native(Pdf17EditorCommand::PickImage { page, operator, x, y, width, height, guard }) =>
                Ok((target(*page, *operator, [*x, *y, *width, *height]), guard.as_str())),
            _ => Err(Fault::from("PDF image picker command mismatch")),
        }))""")

    for name, digest in preimages.items():
        actual = (ROOT / name)
        measured = hashlib.sha256(actual.read_bytes()).hexdigest() if actual.is_file() else None
        if measured != digest:
            raise ValueError(f"Retained picker changed after staging: {name}")

    (OUT / "raw-pdf-picker-overlay.json").write_text(json.dumps(overlay, ensure_ascii=False), encoding="utf8")
    for name in ("🦀️.rs", "📨️issue.rs"):
        (OUT / "rm-sdk" / name).write_text(sources[str(RAW / name)], encoding="utf8")
    (OUT / "rm-reactor" / "picker.rs").write_text(sources[str(PUB)], encoding="utf8")
    print(f"[DEBUG] Guarded {len(sources)} original picker reactor and PDF postimages")

if __name__ == "__main__":
    stage()
