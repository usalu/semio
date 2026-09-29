"""🧯️ S20 F2a follow-up (faults overlay): `f2-code-like.py` qualified a `semio_framework_plugin::Fault::from("…")` call by
prefixing `semio_framework_plugin::app_fault(` in front of the already-qualified path — collapse the doubled crate path.
Idempotent. Usage: python3 f2-double-path.py <root>"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
DOUBLE = "semio_framework_plugin::semio_framework_plugin::app_fault("
changed = sites = 0
for path in (ROOT / "✏️s/🔌️plugins").rglob("*.rs"):
    if "target" in path.parts:
        continue
    text = path.read_text()
    if DOUBLE in text:
        sites += text.count(DOUBLE)
        path.write_text(text.replace(DOUBLE, "semio_framework_plugin::app_fault("))
        changed += 1
print(f"collapsed {sites} doubled paths in {changed} files")
