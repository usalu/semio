"""Stage the dedicated natural source DTOs and their WIT adoption points."""
from difflib import unified_diff
from pathlib import Path
from runpy import run_path

ticket = Path(__file__).parent
receiver = run_path(str(ticket / "natural-pdf-guest-stage.py"))
output = receiver["output"]
changes = {}
root = Path("🧰️framework")
replication = root / "🔨️modules/📡️replication"
ingress = replication / "📡️wire/🎮️command/📥️ingress"
kernel = root / "🔨️modules/🎠️kernel"
plugin = root / "🛍️products/💻️os/🔨️modules/🔌️plugin"
names = "NATURAL_SOURCE_COMMAND_KIND, ActorInstanceOpenPurpose, NaturalSourcePrepare, NaturalSourceControlEvent, NaturalSourceWireDeclaration, NaturalSourceWireLoan, NaturalSourceWireControl, NaturalSourceWirePhase, NaturalSourceWireResult, NaturalSourceWireAck"


def change(path, replacements):
    base = path.read_text(encoding="utf-8")
    revised = base
    for old, new in replacements:
        assert revised.count(old) == 1, (path, old, revised.count(old))
        revised = revised.replace(old, new, 1)
    changes[path] = base, revised


wire = (ticket / "natural-pdf-wire-records.rs").read_text(encoding="utf-8")
changes[ingress / "📥️natural-source.rs"] = "", wire
entry = ingress / "🦀️.rs"
change(entry, [
    ('#[path = "🎞️original-media.rs"]', '#[path = "📥️natural-source.rs"]\nmod natural_source;\npub use natural_source::{' + names + '};\n#[path = "🎞️original-media.rs"]'),
    ("    OriginalMediaAck(OriginalMediaWireAck),", "    OriginalMediaAck(OriginalMediaWireAck),\n    NaturalSourceAck(NaturalSourceWireAck),"),
])
change(replication / "📦️packages/🦀️rust/🦀️.rs", [
    ("            ORIGINAL_MEDIA_COMMAND_KIND, OriginalMediaControlEvent, OriginalMediaWireAck,",
     "            " + names + ",\n            ORIGINAL_MEDIA_COMMAND_KIND, OriginalMediaControlEvent, OriginalMediaWireAck,"),
])
change(kernel / "🦀️.rs", [
    ("    OriginalMediaWireLoan, OriginalMediaWireControl, OriginalMediaControlEvent,", "    " + names + ",\n    OriginalMediaWireLoan, OriginalMediaWireControl, OriginalMediaControlEvent,"),
    ("    OriginalMediaControl(OriginalMediaControlEvent),", "    OriginalMediaControl(OriginalMediaControlEvent),\n    NaturalSourcePrepare(NaturalSourcePrepare),\n    NaturalSourceControl(NaturalSourceControlEvent),"),
    ("    InstanceOpen {\n        request: ActorInstanceOpenRequest,", "    InstanceOpen {\n        request: ActorInstanceOpenRequest,\n        purpose: ActorInstanceOpenPurpose,"),
])
change(root / "📦️packages/🦀️rust/🦀️.rs", [
    ("        ORIGINAL_MEDIA_COMMAND_KIND, OriginalMediaControlEvent, OriginalMediaWireAck,", "        " + names + ",\n        ORIGINAL_MEDIA_COMMAND_KIND, OriginalMediaControlEvent, OriginalMediaWireAck,"),
])
change(ingress / "🚚️batch-driver-behavior/🦀️.rs", [
    ("CommandIngressStatus::OriginalMediaAck(_) => return", "CommandIngressStatus::OriginalMediaAck(_) | CommandIngressStatus::NaturalSourceAck(_) => return"),
    ("CommandIngressStatus::Idle | CommandIngressStatus::OriginalMediaAck(_) =>", "CommandIngressStatus::Idle | CommandIngressStatus::OriginalMediaAck(_) | CommandIngressStatus::NaturalSourceAck(_) =>"),
])

events = """
  record natural-source-launch {
    declaration: natural-source-declaration,
    extent: u32,
  }
  variant actor-instance-open-purpose {
    ordinary,
    natural-source(natural-source-launch),
  }
  record natural-source-declaration {
    format-kind: string,
    extension: string,
    media-type: string,
    binary: bool,
  }
  record natural-source-prepare {
    instance-id: instance-id,
    seq: u64,
    declaration: natural-source-declaration,
    extent: u32,
  }
  record natural-source-loan {
    operation-id: string,
    generation: string,
    instance-id: instance-id,
    declaration: natural-source-declaration,
    extent: u32,
  }
  enum natural-source-control { begin, complete, cancel }
  record natural-source-control-event {
    loan: natural-source-loan,
    owner: u64,
    seq: u64,
    kind: u8,
    extent: u32,
    control: natural-source-control,
  }
"""
change(plugin / "🧬️schema/📜️events.wit", [
    ("  record original-media-loan {", events + "\n  record original-media-loan {"),
    ("    original-media-control(original-media-control-event),", "    original-media-control(original-media-control-event),\n    natural-source-prepare(natural-source-prepare),\n    natural-source-control(natural-source-control-event),"),
    ("  record instance-open-event {\n    instance: instance-id,", "  record instance-open-event {\n    instance: instance-id,\n    purpose: actor-instance-open-purpose,"),
])
ack = """
  enum natural-source-phase { prepare, begin, page, complete, cancel }
  enum natural-source-result { retry, pending, accepted, stale, refused }
  record natural-source-ack {
    loan: option<natural-source-loan>,
    owner: u64,
    seq: u64,
    kind: u8,
    extent: u32,
    phase: natural-source-phase,
    page-index: option<u32>,
    %result: natural-source-result,
  }
"""
change(plugin / "🧬️schema/📜️reactor.wit", [
    ("  use events.{event, original-media-loan};", "  use events.{event, original-media-loan, natural-source-loan};"),
    ("  record command-ingress-status {", ack + "\n  record command-ingress-status {\n    natural-source-ack: option<natural-source-ack>,"),
])
patch = []
for path, (base, staged) in changes.items():
    patch.extend(unified_diff(base.splitlines(keepends=True), staged.splitlines(keepends=True), f"a/{path}" if base else "/dev/null", f"b/{path}"))
(output / "wire-contract.patch").write_bytes("".join(patch).encode("utf-8"))
scope = output / "receiver-check"
(scope / "wire.rs").write_text(wire, encoding="utf-8")
with (scope / "lib.rs").open("a", encoding="utf-8") as crate:
    crate.write('#[path = "wire.rs"] mod wire;\n')
