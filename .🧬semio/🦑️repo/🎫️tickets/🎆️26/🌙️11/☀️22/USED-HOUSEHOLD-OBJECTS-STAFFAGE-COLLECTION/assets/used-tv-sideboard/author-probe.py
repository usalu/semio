"""🔬 Writes a probe variant of the candidate with named front shapes removed: `python3 author-probe.py <view> <id,id,...>`; output in 🗑️generated/used-tv-sideboard-x/🔣️.json."""
import json, sys, pathlib
HERE = pathlib.Path(__file__).resolve().parent
doc = json.loads((HERE / "🔣️.json").read_text())
view, ids = sys.argv[1], set(sys.argv[2].split(","))
geo = doc["asset"]["representations"]["50"]["geometry"]
geo[view] = [s for s in geo[view] if s["id"] not in ids]
out = HERE / "🗑️generated" / "used-tv-sideboard-x" / "🔣️.json"
out.write_text(json.dumps(doc, ensure_ascii=False))
print("[DEBUG] removed", len(ids))
