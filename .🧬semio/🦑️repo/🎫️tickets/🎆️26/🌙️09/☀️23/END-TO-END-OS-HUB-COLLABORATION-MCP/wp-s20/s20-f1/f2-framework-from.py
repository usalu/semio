"""🧯️ S20 F2 (faults overlay): the framework's own `Fault::from(text)` sites (the untyped `app.message` fault is gone)
become framework raises — `Fault::new(FaultOrigin::…, FaultCode::new("<code>"), <the old text as the developer message>)`
plus `.with_parameter` for the values a person needs; each code gets its framework catalog entry (family A). Idempotent.
Usage: python3 f2-framework-from.py <root>"""
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
M = "🧰️framework/🛍️products/💻️os/🔨️modules"
SDK = f"{M}/🔌️plugin/🦀️.rs"
RETAINED = f"{M}/🔌️plugin/🧵️retained-command/🦀️.rs"
PRESENCE = f"{M}/🔌️plugin/👥️presence/♻️retirement/🦀️.rs"
RUN = f"{M}/🏃️run/🦀️.rs"
TRANSIENT = f"{M}/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs"
F = "semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new"
S = "super::FaultOrigin::Framework, super::FaultCode::new"


def framework(code: str, message: str, parameters: str = "", prefix: str = "", origin: str = F) -> str:
    return f'{prefix}Fault::new({origin}("{code}"), {message}){parameters}'


EDITS: list[tuple[str, str, str]] = [
    (SDK, 'return Err(super::Fault::from("registered fixture typed operation did not retire within 30 seconds"));',
     f'return Err({framework("interactive-job.fixture-retire-timeout", chr(34) + "registered fixture typed operation did not retire within 30 seconds" + chr(34), prefix="super::", origin=S)});'),
    (SDK, 'return Err(super::Fault::from(format!(\n                            "registered fixture typed operation exceeded its exact maintenance grant',
     'return Err(super::Fault::new(super::FaultOrigin::Framework, super::FaultCode::new("interactive-job.fixture-maintenance-grant-exceeded"), format!(\n                            "registered fixture typed operation exceeded its exact maintenance grant'),
    (SDK, 'return Err(super::Fault::from("registered fixture typed operation rejected its exact result ACK"));',
     f'return Err({framework("interactive-job.fixture-ack-rejected", chr(34) + "registered fixture typed operation rejected its exact result ACK" + chr(34), prefix="super::", origin=S)});'),
    (SDK, 'return Err(super::Fault::from("registered fixture typed operation rejected its exact local-interaction ACK"));',
     f'return Err({framework("interactive-job.fixture-ack-rejected", chr(34) + "registered fixture typed operation rejected its exact local-interaction ACK" + chr(34), prefix="super::", origin=S)});'),
    (SDK, 'other => Err(Fault::from(format!("unknown action id {other}"))),',
     f'other => Err({framework("plugin.action.unknown", "format!(" + chr(34) + "unknown action id {other}" + chr(34) + ")", ".with_parameter(" + chr(34) + "action" + chr(34) + ", other)")}),'),
    (SDK, "Some(kind @ (ActionKind::View | ActionKind::Shell)) => Err(Fault::from(format!(\"{kind:?}-kind command '{verb}' must not emit operations\"))),",
     "Some(kind @ (ActionKind::View | ActionKind::Shell)) => Err(" + framework("plugin.command.emits-operations", "format!(\"{kind:?}-kind command '{verb}' must not emit operations\")", ".with_parameter(\"command\", verb.to_string())") + "),"),
    (SDK, ".map_err(|error| Fault::from(error.to_string()))?;", ".map_err(|error| " + framework("plugin.ephemeral.apply-rejected", "error.to_string()") + ")?;"),
    (SDK, "return Err(Fault::from(format!(\"command '{command_id}' is not owned by app {owner_app_id}\")));",
     "return Err(" + framework("plugin.command.not-app-owned", "format!(\"command '{command_id}' is not owned by app {owner_app_id}\")", ".with_parameter(\"command\", command_id.to_string())") + ");"),
    (SDK, ".ok_or_else(|| Fault::from(format!(\"command '{command_id}' is not owned by app {owner_app_id}\")))?)",
     ".ok_or_else(|| " + framework("plugin.command.not-app-owned", "format!(\"command '{command_id}' is not owned by app {owner_app_id}\")", ".with_parameter(\"command\", command_id.to_string())") + ")?)"),
    (SDK, "return Err(Fault::from(format!(\"command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}\")));",
     "return Err(" + framework("plugin.command.not-mode-owned", "format!(\"command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}\")", ".with_parameter(\"command\", command_id.to_string())") + ");"),
    (SDK, ".ok_or_else(|| Fault::from(format!(\"command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}\")))?)",
     ".ok_or_else(|| " + framework("plugin.command.not-mode-owned", "format!(\"command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}\")", ".with_parameter(\"command\", command_id.to_string())") + ")?)"),
    (SDK, "_ => return Err(Fault::from(format!(\"command '{command_id}' is not app- or mode-owned\"))),",
     "_ => return Err(" + framework("plugin.command.owner-invalid", "format!(\"command '{command_id}' is not app- or mode-owned\")", ".with_parameter(\"command\", command_id.to_string())") + "),"),
    (SDK, 'return Err(Fault::from("plugin command handler received a non-plugin owner"));',
     "return Err(" + framework("plugin.command.owner-not-plugin", "\"plugin command handler received a non-plugin owner\"") + ");"),
    (SDK, 'return Err(Fault::from(format!("plugin command owner {plugin_id} does not match {}", self.manifest.plugin_id)));',
     "return Err(" + framework("plugin.command.owner-mismatch", "format!(\"plugin command owner {plugin_id} does not match {}\", self.manifest.plugin_id)") + ");"),
    (SDK, '.ok_or_else(|| Fault::from(format!("unknown plugin command: {}", invocation.address.command_id)))?;',
     ".ok_or_else(|| " + framework("plugin.command.unknown", "format!(\"unknown plugin command: {}\", invocation.address.command_id)", ".with_parameter(\"command\", invocation.address.command_id.to_string())") + ")?;"),
    (SDK, '.ok_or_else(|| Fault::from(format!("plugin command definition missing: {}", invocation.address.command_id)))?;',
     ".ok_or_else(|| " + framework("plugin.command.definition-missing", "format!(\"plugin command definition missing: {}\", invocation.address.command_id)", ".with_parameter(\"command\", invocation.address.command_id.to_string())") + ")?;"),
    (SDK, "return Err(Fault::from(format!(\"{:?}-kind plugin command '{}' must not emit mutations\", definition.kind, definition.id)));",
     "return Err(" + framework("plugin.command.emits-mutations", "format!(\"{:?}-kind plugin command '{}' must not emit mutations\", definition.kind, definition.id)", ".with_parameter(\"command\", definition.id.to_string())") + ");"),
    (PRESENCE, 'return Err(Fault::from("presence close terminal root or generation changed"));', "return Err(" + framework("plugin.presence.close-terminal-changed", "\"presence close terminal root or generation changed\"") + ");"),
    (PRESENCE, 'SnapshotRetirementStep::Pending { .. } => Err(Fault::from("presence retirement exceeded its exact grant")),', "SnapshotRetirementStep::Pending { .. } => Err(" + framework("plugin.presence.retirement-grant-exceeded", "\"presence retirement exceeded its exact grant\"") + "),"),
    (PRESENCE, 'SnapshotRetirementStep::Complete => Err(Fault::from("presence retirement completed with retained owners")),', "SnapshotRetirementStep::Complete => Err(" + framework("plugin.presence.retirement-owners-retained", "\"presence retirement completed with retained owners\"") + "),"),
    (PRESENCE, '.ok_or_else(|| Fault::from("presence close lost its exact terminal root"))?;', ".ok_or_else(|| " + framework("plugin.presence.close-terminal-lost", "\"presence close lost its exact terminal root\"") + ")?;"),
    (PRESENCE, "                Err(Fault::from(reason))\n", "                Err(" + framework("plugin.presence.close-refused", "reason") + ")\n"),
    (RUN, ".map_err(|error| semio_framework::Fault::from(error.to_string()))?;", ".map_err(|error| " + framework("os.opening-config.rejected", "error.to_string()", prefix="semio_framework::") + ")?;"),
    (RUN, "fault: dsl::encode_fault_bytes(&semio_framework::Fault::from(error.to_string())), report: Vec::new() }",
     "fault: dsl::encode_fault_bytes(&" + framework("os.app-frame.decode", "error.to_string()", prefix="semio_framework::") + "), report: Vec::new() }"),
    (TRANSIENT, '.ok_or_else(|| Fault::from("transient terminal owner or generation changed"));', ".ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new(\"plugin.transient.close-terminal-changed\"), \"transient terminal owner or generation changed\"));"),
    (TRANSIENT, 'store::SnapshotRetirementStep::Complete => Err(Fault::from("transient retirement completed without terminal-empty ownership")),', "store::SnapshotRetirementStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new(\"plugin.transient.retirement-not-empty\"), \"transient retirement completed without terminal-empty ownership\")),"),
]
RETAINED_CODE = re.compile(r'Fault::from\("(retained-command-[a-z-]+)"\)')


def main() -> None:
    texts: dict[str, str] = {}
    for rel, before, after in EDITS:
        text = texts.setdefault(rel, (ROOT / rel).read_text())
        if before in text:
            texts[rel] = text.replace(before, after)
        elif after not in text:
            sys.exit(f"CONFLICT {rel}: {before[:90]}")
    retained = texts.setdefault(RETAINED, (ROOT / RETAINED).read_text())
    texts[RETAINED] = RETAINED_CODE.sub(lambda match: f'Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("{match.group(1)}"), "{match.group(1)}")', retained)
    for rel, text in texts.items():
        if text != (ROOT / rel).read_text():
            (ROOT / rel).write_text(text)
            print(f"changed {rel.split('/')[-2]}/{rel.split('/')[-1]}")
    left = sum((ROOT / rel).read_text().count("Fault::from(") for rel in (SDK, RETAINED, PRESENCE, RUN, TRANSIENT))
    print(f"framework Fault::from sites left in these files: {left}")


main()
