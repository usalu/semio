import json, collections
from pathlib import Path
root = Path("🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
manifest = json.loads((root / "🔣️.json").read_text(encoding="utf-8"))
folders = {c["folder"] for c in manifest["categories"]}
seen = collections.defaultdict(list)
bad = []
for f in root.glob("*/*/🔣️.json"):
    doc = json.loads(f.read_text(encoding="utf-8"))
    seen[f.parent.name].append(f.parent.parent.name)
    if doc["asset"]["id"] != f.parent.name: bad.append((str(f), doc["asset"]["id"]))
    if f.parent.parent.name not in folders: bad.append(("uncategorised", str(f)))
print("assets", len(seen), "dupes", {k: v for k, v in seen.items() if len(v) > 1}, "bad", bad)
