"""Author 60: scooter frame parts at 1:100 and 1:200 as real tubes, plate deck and revolved tyres (the drafts were flat plates and lenses). Patches volumes/humans/child-on-kick-scooter.json in place."""
import json
F = "volumes/humans/child-on-kick-scooter.json"
src = json.load(open(F))

def base(x):
    return {k: x[k] for k in ("id", "bone", "material", "tone", "role") if k in x}

def cap(x, p0, p1, r, **kw):
    return {**base(x), "kind": "capsule", "p0": p0, "p1": p1, "r0": r, "r1": r, **kw}

def tyre(x):
    return {**base(x), "kind": "revolve", "profile": [[-0.015, 0], [-0.015, 0.0625], [0, 0.0725], [0.015, 0.0625], [0.015, 0]], "axis": [0, 0, 1], "origin": [0, 0, 0]}

for sc in ("100", "200"):
    body = src["volumes"][sc]["body"]
    for i, x in enumerate(body):
        n = x["id"]
        if n == "stem":
            body[i] = cap(x, [0.69, 0.165, 0], [0.598, 0.815, 0], 0.0135 if sc == "100" else 0.0155)
        elif n == "head-tube":
            body[i] = cap(x, [0, 0, 0], [0.1063, 0, 0], 0.018, lateralScale=1.5556, transform={"translation": [0.64, 0.1, 0], "rotation": [0, 0, 0.41321628243057057, 0.9106329139308874]})
        elif n == "handlebar":
            body[i] = cap(x, [0.6, 0.84, -0.2075], [0.6, 0.84, 0.2075], 0.0175)
        elif n == "grip":
            body[i] = cap(x, [0.6, 0.828, -0.2075], [0.6, 0.828, -0.12], 0.0195 if sc == "100" else 0.0155, repeat={"count": 2, "translation": [0, 0, 0.3275]})
        elif n in ("tyre-rear", "tyre-front"):
            body[i] = tyre(x)
        elif n == "deck" and sc == "200":
            body[i] = {**base(x), "kind": "extrude", "profile": [[0.05, 0.08], [0.6, 0.08], [0.64, 0.11], [0.6, 0.134], [0.05, 0.134]], "plane": "xy", "from": -0.065, "to": 0.065}
json.dump(src, open(F, "w"), ensure_ascii=False, indent=2)
print("ok")
