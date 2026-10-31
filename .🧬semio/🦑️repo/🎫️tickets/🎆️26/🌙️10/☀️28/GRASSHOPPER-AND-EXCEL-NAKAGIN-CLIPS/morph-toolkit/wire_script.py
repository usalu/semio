"""🔌 Byte-level, idempotent insertion of the morph test module into the python test list of script.ts."""
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
path = root / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts"
data = path.read_bytes()
anchor = '"🎓️teaching.🏛️architecture.🏢️nakagin.🧪️tests.🐍️", '.encode()
new = '"🎓️teaching.🏛️architecture.🔀️morph.🧪️tests.🐍️", '.encode()
if new not in data:
    assert data.count(anchor) == 1
    path.write_bytes(data.replace(anchor, anchor + new))
print("[DEBUG] wired", new in path.read_bytes())
