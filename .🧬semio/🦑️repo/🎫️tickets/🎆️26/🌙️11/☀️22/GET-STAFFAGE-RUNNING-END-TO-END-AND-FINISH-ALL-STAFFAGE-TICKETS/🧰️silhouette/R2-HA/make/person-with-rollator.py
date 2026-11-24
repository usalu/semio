"""🛞️ Hand ops of `person-with-rollator`: in the front view of 1:100 the moulded wheel discs are drawn by their outer ring only (the disc with its spoke openings was swept sideways, which opened slots across the wheel where the body has rim, hub and spoke). Writes hand/person-with-rollator.json from the state before this round (`ASSET_FILE`) or the catalog file."""
import json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
a = json.load(open(os.environ.get("ASSET_FILE") or f"{CAT}/person-with-rollator/🔣️.json"))["asset"]
ops = []
for s in a["representations"]["100"]["geometry"]["front"]:
    if s["id"].startswith("moulded-") and s["kind"] == "path":
        rings = re.findall(r"M[^Z]*Z", s["d"])
        ops.append({"op": "shape-set", "scale": "100", "view": "front", "id": s["id"], "set": {"d": rings[0]}})
        print(s["id"], len(rings), "rings")
open(f"{HERE}/../hand/person-with-rollator.json", "w").write("[\n" + ",\n".join(json.dumps(o) for o in ops) + "\n]\n")
