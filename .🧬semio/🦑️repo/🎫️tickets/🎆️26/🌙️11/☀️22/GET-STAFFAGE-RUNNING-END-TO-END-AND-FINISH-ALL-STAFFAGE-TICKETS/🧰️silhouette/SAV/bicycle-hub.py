"""🚲️ The front hub of `city-bicycle` at 1:50 in the front elevation and the plan: the drawing was one disc of 45 mm radius over the whole hub width (64 mm), the body is a stepped hub: a 4.5 mm flange of 20 mm radius, the 45 mm shell and a barrel of 22 to 34 mm radius towards the brake drum. The disc is shortened to the shell (37.5 mm wide, 0.75 mm on paper); the flange and the barrel are narrower than the 0.2 mm a printed feature needs and stay undrawn around the drawn axle (a stepped drawing of three discs was tried and refused by the print suitability, `suitability-feature`). Writes idempotent ops at the end of 🗑️generated/silhouette/SAV/ops/city-bicycle.json.
python3 bicycle-hub.py"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = f"{HERE}/../../🗑️generated/silhouette/SAV"
source = f"{GEN}/backup/city-bicycle.json" if os.path.exists(f"{GEN}/backup/city-bicycle.json") else "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-bicycle/🔣️.json"
asset = json.load(open(source))["asset"]
ops = []
for view in ("front", "plan"):
    hub = next(s for s in asset["representations"]["50"]["geometry"][view] if s["id"] == "hub-f")
    ops.append({"op": "shape-set", "scale": "50", "view": view, "id": "hub-f", "set": {"solid": {**hub["solid"], "z0": -0.0335, "z1": 0.006}}})
path = f"{GEN}/ops/city-bicycle.json"
old = json.load(open(path)) if os.path.exists(path) else []
sig = lambda o: (o.get("scale"), o.get("view"), o.get("id") or o.get("shape", {}).get("id"))
keep = [o for o in old if sig(o) not in {sig(n) for n in ops}]
os.makedirs(os.path.dirname(path), exist_ok=True)
open(path, "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in keep + ops) + "\n]\n")
print(f"city-bicycle: {len(ops)} hub ops")
