"""🌿 Writes the root members of the round-3 miscanthus candidate into the catalog asset: re-reads the catalog file right before writing, replaces only the text of the named members of `asset` in the catalog's own formatting (the formatter of `volumes-insert.py`), keeps member order and every other byte, and checks that nothing else differs.

Usage: python3 r3-miscanthus-grass-roots.py
See: ./r3-vegetation-miscanthus-grass.md
"""
import importlib.util
import json
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
CATALOG = TICKET.parents[6] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/miscanthus-grass/🔣️.json"
CANDIDATE = TICKET / "🗑️generated/r3-miscanthus-grass/🔣️.json"
MEMBERS = ["rig", "anchors", "summary", "provenance", "thermal", "representations"]

spec = importlib.util.spec_from_file_location("volumes_insert", TICKET / "volumes-insert.py")
vi = importlib.util.module_from_spec(spec)
argv, sys.argv = sys.argv, [sys.argv[0]]
try:
    spec.loader.exec_module(vi)
except SystemExit:
    pass
sys.argv = argv

text = CATALOG.read_text(encoding="utf-8")
old = json.loads(text)["asset"]
new = json.loads(CANDIDATE.read_text(encoding="utf-8"))["asset"]
others = [k for k in old if k not in MEMBERS and k != "volumes" and old[k] != new[k]]
if others:
    sys.exit(f"members outside the root fix differ, nothing written: {others}")
changed = []
for member in MEMBERS:
    if old[member] == new[member]:
        continue
    found = vi.member_range(text, "    ", member)
    if found is None:
        sys.exit(f"member {member} not found")
    start, end = found
    head = text[start:end].split(":", 1)[0]
    text = text[:start] + head + ": " + vi.dump(new[member], 2) + text[end:]
    changed.append(member)
check = json.loads(text)["asset"]
if any(check[m] != new[m] for m in MEMBERS) or list(check) != list(old):
    sys.exit("round trip failed, nothing written")
CATALOG.write_text(text, encoding="utf-8")
print(f"written {CATALOG.name}: {', '.join(changed) or 'nothing changed'}")
