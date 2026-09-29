"""🧯️ S20 F2 (faults overlay): the SDK's last `?` over a `Result<_, String>` (no `From<String> for Fault` any more) —
missing framework-reserved action arguments become catalogued framework raises naming the action. Idempotent.
Usage: python3 f2-sdk-string-errors.py <root>"""
import sys
from pathlib import Path

F = Path(sys.argv[1]) / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
EDITS = [
    ('.ok_or_else(|| format!("history action {action} missing required argument"))?;',
     '.ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.action.argument-missing"), format!("history action {action} missing required argument")).with_parameter("action", action))?;'),
    ('.ok_or_else(|| "noteShellCommand missing required commandId".to_string())?;',
     '.ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.action.argument-missing"), "noteShellCommand missing required commandId").with_parameter("action", NOTE_SHELL_COMMAND_ACTION_ID))?;'),
]
text = F.read_text()
for before, after in EDITS:
    if after in text:
        continue
    if text.count(before) != 1:
        sys.exit(f"CONFLICT {before[:60]} ×{text.count(before)}")
    text = text.replace(before, after)
F.write_text(text)
print("sdk string errors: applied")
