"""🧪 Writes the candidate document (catalog asset with the volumes source merged over it) to 🗑️generated/rework-tram/cand/🔣️.json for the --file tools."""
import json, os
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/tram/🔣️.json"
doc = json.load(open(CAT))
doc["asset"]["volumes"] = json.load(open("volumes/vehicles/tram.json"))["volumes"]
os.makedirs("🗑️generated/rework-tram/cand", exist_ok=True)
json.dump(doc, open("🗑️generated/rework-tram/cand/🔣️.json", "w"), ensure_ascii=False)
