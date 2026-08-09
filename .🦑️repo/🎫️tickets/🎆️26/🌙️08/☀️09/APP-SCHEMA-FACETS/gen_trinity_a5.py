#!/usr/bin/env python3
from pathlib import Path

PLUGIN = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity")
REWRITE = PLUGIN / "🎛️apps/♻️rewrite"
JACK = PLUGIN / "🎛️apps/🔌️jack"

def write(path: Path, content: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)
    print("wrote", path.relative_to(PLUGIN))

# Load payloads from sibling data dir written separately
DATA = Path(__file__).with_name("gen_trinity_a5_data")
for rel in sorted(DATA.rglob("*")):
    if rel.is_file():
        dest = PLUGIN / rel.relative_to(DATA)
        write(dest, rel.read_text())
print("done")
