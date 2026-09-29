"""🗂️ S20 fault classes: the hand fix-ups `class-apply.py` reports (imports it cannot place: nested `use` groups).
Idempotent. Usage: python3 class-fixups.py"""
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
EDITS = [
    ("✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
     "DslValue, Editor, Emit, Fault, GranularityDefinition,", "DslValue, Editor, Emit, Fault, FaultClass, GranularityDefinition,"),
]
for rel, old, new in EDITS:
    path = OVERLAY / rel
    text = path.read_text()
    if new in text:
        continue
    assert text.count(old) == 1, rel
    path.write_text(text.replace(old, new))
    print("fixed", rel.split("/")[2])
