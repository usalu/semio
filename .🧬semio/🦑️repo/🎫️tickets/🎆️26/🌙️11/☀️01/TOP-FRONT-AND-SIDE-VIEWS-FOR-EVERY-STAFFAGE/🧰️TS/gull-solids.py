import json
import os
import sys

p = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/herring-gull/🔣️.json"
raw = open(p, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw
doc = json.loads(raw)
asset = doc["asset"]
ATTR = {"id", "bone", "role", "z", "tone", "filled", "region", "regions", "material", "hatch", "pen", "solid", "title", "description"}
LATERAL = {"arm-n": (-0.097, -0.063), "hand-n": (-0.095, -0.065), "arm-f": (0.063, 0.097), "hand-f": (0.065, 0.095)}
SOURCE = {"arm-n": ("near-arm", "arm"), "hand-n": ("near-hand", "hand"), "arm-f": ("far-arm", "near-arm", "arm"), "hand-f": ("far-hand", "near-hand", "hand")}
z0_override = float(sys.argv[1]) if len(sys.argv) > 1 else None

for scale in ("50", "100", "200"):
    geometry = asset["representations"][scale]["geometry"]
    elevation = {s["id"]: s for s in geometry["elevation"]}
    for view in ("front", "plan"):
        for index, shape in enumerate(geometry[view]):
            bone = shape["bone"]
            if bone not in LATERAL:
                continue
            source = next(elevation[i] for i in SOURCE[bone] if i in elevation)
            new = {k: v for k, v in shape.items() if k in ATTR and k != "solid"}
            new.update({k: v for k, v in source.items() if k not in ATTR})
            z0, z1 = LATERAL[bone]
            new["solid"] = {"kind": "extruded", "z0": z0, "z1": z1}
            ordered = {k: new[k] for k in shape if k in new}
            ordered.update({k: v for k, v in new.items() if k not in ordered})
            geometry[view][index] = ordered

out = json.dumps(doc, ensure_ascii=False, indent=2) + ("\n" if raw.endswith("\n") else "")
if crlf:
    out = out.replace("\n", "\r\n")
open(p + ".tmp", "w", encoding="utf-8", newline="").write(out)
os.replace(p + ".tmp", p)
print("gull wings are solids")
