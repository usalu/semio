"""🚗️ The ends of the 1:50 body of `city-car`: the bodies of the four scales were 3.941, 3.932, 3.900 and 3.900 m long, so no declared length lay within one smallest feature of all of them. The front plate, the intake and the rear diffuser of 1:50, which stood 12 to 15 mm proud of the bumpers, are set 8, 5 and 4 mm closer to them (the body of 1:50 becomes 3.929 m) and the length is declared at 3.926 m. Writes the ops (absolute values, idempotent) at the head of 🗑️generated/silhouette/SAV/ops/city-car.json and the declared length into manifest-changes.json.
python3 city-car-ends.py"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio"
FILE = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-car/🔣️.json"
OPS = f"{HERE}/../../🗑️generated/silhouette/SAV/ops/city-car.json"
asset = json.load(open(FILE))["asset"]
body = {s["id"]: s for s in asset["volumes"]["50"]["body"]}
intake = json.loads(json.dumps(body["intake"]))
shift = 1.953 - max(s["at"][0] for s in intake["sections"])
for s in intake["sections"]:
    s["at"][0] = round(s["at"][0] + shift, 5)
ops = [
    {"op": "dims", "set": {"widthM": 3.926}},
    {"op": "solid-set", "scale": "50", "id": "plate-front", "set": {"transform": {**body["plate-front"]["transform"], "translation": [1.959, 0.47, 0]}}},
    {"op": "solid-set", "scale": "50", "id": "diffuser-rear", "set": {"transform": {**body["diffuser-rear"]["transform"], "translation": [-1.954, 0.375, 0.1]}}},
    {"op": "solid", "scale": "50", "id": "intake", "solid": intake},
]
sig = lambda o: (o["op"], o.get("scale"), o.get("id"))
old = json.load(open(OPS)) if os.path.exists(OPS) else (json.load(open(f"{HERE}/repair/city-car.json")) if os.path.exists(f"{HERE}/repair/city-car.json") else [])
rest = [o for o in old if sig(o) not in {sig(n) for n in ops}]
os.makedirs(os.path.dirname(OPS), exist_ok=True)
open(OPS, "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops + rest) + "\n]\n")
mf = f"{HERE}/manifest-changes.json"
m = json.load(open(mf)) if os.path.exists(mf) else {}
m["city-car"] = {**m.get("city-car", {}), "widthM": 3.926}
open(mf, "w").write(json.dumps(dict(sorted(m.items())), indent=2) + "\n")
print(f"city-car: {len(ops)} ops, intake moved by {shift * 1000:.1f} mm")
