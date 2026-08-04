#!/usr/bin/env python3
from pathlib import Path
import re

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugin")

SIG = re.compile(
    r"fn command_from_action\(&self, action: &str, args: Option<&(?:serde_json::)?Value>\) -> Result<([^,]+), String>"
)


def ensure_fault_import(text: str) -> str:
    if re.search(r"\bFault\b", text) and "Fault," in text or ", Fault" in text or "Fault " in text.split("use semio_framework_plugin")[1][:500] if "use semio_framework_plugin" in text else False:
        if "Fault," in text or ", Fault," in text or "{ Fault" in text:
            return text
    if "use semio_framework_plugin::" not in text and "use semio_framework_plugin {" not in text:
        return text
    if re.search(r"(DocumentView, Emit),", text):
        return text.replace("DocumentView, Emit,", "DocumentView, Emit, Fault,", 1)
    if re.search(r"DocumentView, Emit,", text):
        return text.replace("DocumentView, Emit,", "DocumentView, Emit, Fault,", 1)
    if re.search(r"Emit, HostEffect", text):
        return text.replace("Emit, HostEffect", "Emit, Fault, HostEffect", 1)
    if re.search(r"Emit, Label", text):
        return text.replace("Emit, Label", "Emit, Fault, Label", 1)
    return text


changed = []
for path in ROOT.rglob("📦️lib.rs"):
    text = path.read_text()
    if "fn command_from_action" not in text or ", String>" not in text:
        continue
    new, n = SIG.subn(r"fn command_from_action(&self, action: &str, args: Option<&Value>) -> Result<\1, Fault>", text)
    if n == 0:
        new, n = SIG.subn(
            r"fn command_from_action(&self, action: &str, args: Option<&serde_json::Value>) -> Result<\1, Fault>",
            text,
        )
    if n:
        new = ensure_fault_import(new)
        path.write_text(new)
        changed.append(path)

print(f"updated {len(changed)} files")
for p in changed:
    print(p.relative_to(ROOT.parent.parent))
