#!/usr/bin/env python3
from pathlib import Path
import re

ROOT = Path("/Users/ueli/Documents/semio")
DIRS = [ROOT / "✏️s/🔌️plugin", ROOT / "🧰️framework"]

DEFAULT_BAD = re.compile(r"Ok\(Emit::default\(\),")
CONFIG_BAD = re.compile(r"(=> Ok\(Emit::config\([^;]+?\)),\s*$", re.MULTILINE)


def fix_file(path: Path) -> bool:
    text = path.read_text()
    orig = text
    text = DEFAULT_BAD.sub("Ok(Emit::default()),", text)

    def fix_config(m: re.Match) -> str:
        inner = m.group(1)
        return inner + "),"

    text = CONFIG_BAD.sub(fix_config, text)
    if text != orig:
        path.write_text(text)
        return True
    return False


changed = []
for base in DIRS:
    if not base.exists():
        continue
    for path in base.rglob("📦️lib.rs"):
        if fix_file(path):
            changed.append(path)

print(f"fixed {len(changed)} files")
for p in changed[:20]:
    print(p.relative_to(ROOT))
if len(changed) > 20:
    print(f"... and {len(changed) - 20} more")
