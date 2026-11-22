"""✏️ Tower crane 1:500 drawings: the hoist rope is one fall in all eight pieces (elevation of pieces 1 to 4 drew two)."""
import json

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"
doc = json.load(open(CATALOG, encoding="utf-8"))
for s in doc["asset"]["representations"]["500"]["geometry"]["elevation"]:
    if s.get("bone") in ("rope-1", "rope-2", "rope-3", "rope-4") and s["kind"] == "path":
        s["d"] = "M -0.007,-5.75 L 0.007,-5.75 L 0.007,0.0 L -0.007,0.0 Z"
json.dump(doc, open(CATALOG, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
