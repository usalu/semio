"""Stage the authenticated natural source receiver against live paid primitives."""
from difflib import unified_diff
from json import dumps
from pathlib import Path
from runpy import run_path

ticket = Path(__file__).parent
extraction = run_path(str(ticket / "natural-pdf-receiver-stage.py"))
receiver = extraction["path"]
module = receiver.parent
output = extraction["generated"]
changes = {receiver: (extraction["base"], extraction["revised"])}


def change(path, replacements):
    base = path.read_text(encoding="utf-8")
    revised = base
    for old, new in replacements:
        assert revised.count(old) == 1, (path, old, revised.count(old))
        revised = revised.replace(old, new, 1)
    changes[path] = base, revised
    return revised


def new(path, text):
    assert not path.exists(), path
    changes[path] = "", text


owner = change(module / "📥️command-owner.rs", [
    ("struct PendingPage {\n    header: CommandPageCursor,\n    body: ControlledRetirement<FixedCommandPage>,\n    copied: usize,\n    complete: bool,",
     "pub(crate) struct PendingPage {\n    pub(crate) header: CommandPageCursor,\n    pub(crate) body: ControlledRetirement<FixedCommandPage>,\n    pub(crate) copied: usize,\n    pub(crate) complete: bool,"),
    ("fn paid(", "pub(crate) fn paid("),
    ("fn moved(", "pub(crate) fn moved("),
    ("fn copy_demand<", "pub(crate) fn copy_demand<"),
])
guest = change(module / "🛂️guest.rs", [
    ("fn validate_source(", "pub(crate) fn validate_source("),
    ("fn run<R>(", "pub(crate) fn run<R>("),
])
change(module / "🦀️.rs", [
    ('#[path = "🧾️picker.rs"]', '#[path = "🪪️natural-source-gate.rs"]\npub mod natural_source_gate;\n#[path = "📥️natural-source.rs"]\npub mod natural_source;\n#[path = "🧾️picker.rs"]'),
])
gate = (ticket / "natural-pdf-admission.rs").read_text(encoding="utf-8")
natural = (ticket / "natural-pdf-guest-receiver.rs").read_text(encoding="utf-8")
new(module / "🪪️natural-source-gate.rs", gate)
new(module / "📥️natural-source.rs", natural)
lifetime = module.parents[2] / "⚛️reactor/🚪️lifetime/🦀️.rs"
change(lifetime, [
    ("pub use semio_plugin_decl_media::original_media::runtime as raw_media;",
     "pub use semio_plugin_decl_media::original_media::runtime as raw_media;\n"
     "use semio_framework::kernel::{CommandPageCursor, FixedCommandPage};\n"
     "use semio_plugin_decl_media::original_media::{natural_source::{NaturalSourceGuest, NaturalSourceStatus}, "
     "natural_source_gate::{NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES, NaturalSourceBegin, NaturalSourceDeclaration, "
     "NaturalSourceLoan, NaturalSourceOffer, NaturalSourceOperation}};"),
    ("    pub raw_media: raw_media::OriginalMediaAdmission,",
     "    pub raw_media: raw_media::OriginalMediaAdmission,\n"
     "    pub natural_source: Option<NaturalSourceGuest<{ NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES as usize }>>,\n"
     "    natural_source_launch: Option<NaturalSourceLaunchGrant>,"),
    ("if !self.raw_media.close(cx)?{return Ok(false)}cx.with_original_retirement",
     "if !self.raw_media.close(cx)?{return Ok(false)}if let Some(source)=self.natural_source.as_mut(){if !source.retire(true,cx)?{return Ok(false)}}cx.with_original_retirement"),
    ("raw_media:raw_media::OriginalMediaAdmission::empty(), native_created:", "raw_media:raw_media::OriginalMediaAdmission::empty(), natural_source:None, natural_source_launch:None, native_created:"),
    ("/// 🗃️ Fixed allocation-bound ownership", (ticket / "natural-pdf-slot-routing.rs").read_text(encoding="utf-8") + "\n/// 🗃️ Fixed allocation-bound ownership"),
])
patch = []
for path, (base, staged) in changes.items():
    patch.extend(unified_diff(base.splitlines(keepends=True), staged.splitlines(keepends=True), f"a/{path}" if base else "/dev/null", f"b/{path}"))
(output / "guest-receiver.patch").write_bytes("".join(patch).encode("utf-8"))

scope = output / "receiver-check"
scope.mkdir(exist_ok=True)
(scope / "receiver.rs").write_text(extraction["revised"], encoding="utf-8")
(scope / "gate.rs").write_text(gate, encoding="utf-8")
(scope / "natural.rs").write_text(natural, encoding="utf-8")
(scope / "command.rs").write_text(
    "use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage};\n"
    "use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, "
    "retained_clone::retained_clone_grant_admits, retirement::controlled::ControlledRetirement};\n"
    + owner[owner.index("pub(crate) struct PendingPage"):owner.index("/// 🎬 Holds one original input")]
    + owner[owner.index("pub(crate) fn paid"):owner.index("impl<T: Copy, const N: usize> OriginalMediaCommand")],
    encoding="utf-8",
)
(scope / "guest.rs").write_text(
    "use semio_framework_job::{OriginalSourceBinding, OriginalTurnFailure, StepContext};\n"
    "use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, ValueError, ValueRefusalKind};\n"
    + guest[guest.index("pub(crate) fn validate_source"):guest.index("impl<T: Copy> OriginalMediaPickerIntent")],
    encoding="utf-8",
)
(scope / "lib.rs").write_text(
    '#![allow(dead_code)]\n'
    '#[path = "guest.rs"] mod guest;\n#[path = "command.rs"] mod command_owner;\n'
    '#[path = "receiver.rs"] mod receiver;\n#[path = "gate.rs"] mod natural_source_gate;\n'
    '#[path = "natural.rs"] mod natural_source;\n'
    '#[path = ' + dumps(str((module / "🎟️loan.rs").resolve()), ensure_ascii=False) + '] mod ingress;\n',
    encoding="utf-8",
)
