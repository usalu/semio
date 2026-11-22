"""🛞 Wheel loader, second round pass 1: planetary axles with a real differential bowl (ground clearance 0.45 m), saddle pads instead of a plate through the bowl, open frame floor at the front axle, stub pins for the lift cylinders, and the driveline (transmission, three propeller shafts, pinion noses) through holed bulkheads. Hand-chosen numbers, all four scales, plus the axle shapes of the 2D front and plan drawings."""
import json, math, sys
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
SRC = T + "/volumes/machinery/wheel-loader.json"
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/wheel-loader/🔣️.json"
YS = 0.8075
AXLE = {
    "50": [[-1.09, 0], [-1.09, 0.1], [-0.84, 0.1], [-0.84, 0.17], [-0.74, 0.17], [-0.7, 0.15], [-0.3, 0.15], [-0.2, 0.3], [-0.16, 0.36], [0.16, 0.36], [0.2, 0.3], [0.3, 0.15], [0.7, 0.15], [0.74, 0.17], [0.84, 0.17], [0.84, 0.1], [1.09, 0.1], [1.09, 0]],
    "100": [[-0.84, 0], [-0.84, 0.15], [-0.3, 0.15], [-0.2, 0.3], [-0.16, 0.36], [0.16, 0.36], [0.2, 0.3], [0.3, 0.15], [0.84, 0.15], [0.84, 0]],
    "200": [[-0.85, 0], [-0.85, 0.15], [-0.3, 0.15], [-0.18, 0.36], [0.18, 0.36], [0.3, 0.15], [0.85, 0.15], [0.85, 0]],
    "500": [[-0.86, 0], [-0.86, 0.15], [-0.3, 0.15], [-0.2, 0.36], [0.2, 0.36], [0.3, 0.15], [0.86, 0.15], [0.86, 0]],
}
DRAWN = {
    "50": [[-0.77, 0.17], [-0.74, 0.17], [-0.7, 0.15], [-0.3, 0.15], [-0.2, 0.3], [-0.16, 0.36], [0.16, 0.36], [0.2, 0.3], [0.3, 0.15], [0.7, 0.15], [0.74, 0.17], [0.77, 0.17]],
    "100": [[-0.77, 0.15], [-0.3, 0.15], [-0.2, 0.3], [-0.16, 0.36], [0.16, 0.36], [0.2, 0.3], [0.3, 0.15], [0.77, 0.15]],
    "200": [[-0.77, 0.15], [-0.3, 0.15], [-0.18, 0.36], [0.18, 0.36], [0.3, 0.15], [0.77, 0.15]],
}
def ring(cv, cw, r, n):
    return [[round(cv + r * math.cos(2 * math.pi * k / n), 4), round(cw + r * math.sin(2 * math.pi * k / n), 4)] for k in range(n)]
def steel(i, bone, tone=0.3, role="body", material="paint-steel"):
    return {"id": i, "bone": bone, "material": material, "role": role, "tone": tone}
def shaft(i, bone, x0, profile, role="detail"):
    return {**steel(i, bone, 0.25, role), "kind": "revolve", "profile": profile, "axis": [1, 0, 0], "origin": [x0, YS, 0.0]}
def case(i, half):
    return {**steel(i, "rear-frame", 0.3, "body", "cast-iron"), "kind": "extrude", "profile": [[-0.92, 1.26], [-0.92, 0.95], [-0.8, 0.66], [-0.42, 0.66], [-0.42, 1.26]], "plane": "xy", "from": -half, "to": half}
def put(lst, solid, after=None):
    for k, x in enumerate(lst):
        if x["id"] == solid["id"]:
            lst[k] = solid
            return
    at = next((k + 1 for k, x in enumerate(lst) if x["id"] == after), len(lst))
    lst.insert(at, solid)
def edit(vol, sc):
    body = vol["body"]
    by = {x["id"]: x for x in body}
    for i, cx in (("axle-r", -1.675), ("axle-f", 1.675)):
        by[i]["profile"] = AXLE[sc]
        by[i]["origin"] = [cx, YS, 0.0]
    pad = by["axle-cradle-r"]
    pad["size"] = [0.5, 0.09 if sc == "50" else 0.15, 0.24]
    pad["transform"] = {"translation": [-1.675, 0.985 if sc == "50" else 0.975, 0.38]}
    pad["repeat"] = {"count": 2, "translation": [0, 0, -0.76]}
    new = []
    if sc in ("50", "100", "200"):
        thick = 0.081 if sc == "200" else 0.05
        half = 0.38 if sc == "200" else 0.4
        by["belly-plate"]["profile"] = [[0.16, 0.619], [1.31, 0.5959], [1.31, round(0.5959 + thick, 4)], [0.16, round(0.619 + thick, 4)]]
        new.append(({**steel("axle-pad-f", "front-frame", 0.45), "kind": "box", "size": [0.75, 0.09 if sc == "50" else 0.15, 0.2], "transform": {"translation": [1.675, 0.985 if sc == "50" else 0.975, 0.3]}, "repeat": {"count": 2, "translation": [0, 0, -0.6]}}, "axle-f"))
        skid = [[2.06, 0.5808], [2.1, 0.58], [2.3, 0.8], [2.24, 0.84], [2.06, 0.66]] if sc == "200" else [[2.06, 0.5808], [2.1, 0.58], [2.3, 0.8], [2.26, 0.83], [2.08, 0.63], [2.06, 0.6308]]
        new.append(({**steel("nose-skid", "front-frame", 0.45), "kind": "extrude", "profile": skid, "plane": "xy", "from": -half, "to": half}, "belly-plate"))
        stub = by["lift-cyl-crossmember"]
        stub["profile"] = [[0, 0], [0, 0.06], [0.32, 0.06], [0.32, 0]]
        stub["origin"] = [0.45, 0.9, 0.38]
        stub["repeat"] = {"count": 2, "translation": [0, 0, -1.08]}
    if sc in ("50", "100"):
        n = 16 if sc == "50" else 8
        r = 0.09 if sc == "50" else 0.1
        for i, v0, v1, w, u0, u1 in (("hitch-bulkhead", 0.66, 1.42, 0.34, -0.36, -0.30), ("front-bulkhead", 0.64, 1.46, 0.4, 0.10, 0.16)):
            b = by[i]
            for k in ("size", "transform", "cornerRadius"):
                b.pop(k, None)
            b.update({"kind": "extrude", "profile": [[v0, -w], [v1, -w], [v1, w], [v0, w]], "holes": [ring(YS, 0, r, n)], "plane": "yz", "from": u0, "to": u1})
        by["hitch-pin-lower"]["profile"] = [[0.0, 0.0], [0.0, 0.055], [0.16, 0.055], [0.16, 0.0]]
    if sc == "50":
        new += [
            (case("transmission", 0.27), "axle-cradle-r"),
            ({**steel("transmission-mount", "rear-frame", 0.45), "kind": "box", "size": [0.12, 0.06, 0.7], "transform": {"translation": [-0.66, 1.25, 0.0]}}, "transmission"),
            (shaft("pinion-r", "rear-frame", -1.335, [[0, 0], [0, 0.13], [0.1, 0.11], [0.145, 0.09], [0.145, 0.075], [0.165, 0.075], [0.165, 0]], "body"), "transmission-mount"),
            (shaft("propshaft-r", "rear-frame", -1.17, [[0, 0], [0, 0.075], [0.02, 0.075], [0.02, 0.045], [0.3, 0.045], [0.3, 0.075], [0.32, 0.075], [0.32, 0]]), "pinion-r"),
            (shaft("propshaft-m", "rear-frame", -0.42, [[0, 0], [0, 0.075], [0.02, 0.075], [0.02, 0.045], [0.36, 0.045], [0.36, 0.065], [0.42, 0.065], [0.42, 0]]), "propshaft-r"),
            (shaft("propshaft-f", "front-frame", 0.0, [[0, 0], [0, 0.065], [0.06, 0.065], [0.06, 0.045], [1.15, 0.045], [1.15, 0.075], [1.17, 0.075], [1.17, 0]]), "axle-f"),
            (shaft("pinion-f", "front-frame", 1.17, [[0, 0], [0, 0.075], [0.02, 0.075], [0.02, 0.09], [0.065, 0.11], [0.165, 0.13], [0.165, 0]], "body"), "propshaft-f"),
        ]
    if sc == "100":
        new += [
            (case("transmission", 0.34), "axle-cradle-r"),
            (shaft("propshaft-r", "rear-frame", -1.335, [[0, 0], [0, 0.13], [0.16, 0.09], [0.16, 0.045], [0.5, 0.045], [0.5, 0]]), "transmission"),
            (shaft("propshaft-m", "rear-frame", -0.42, [[0, 0], [0, 0.045], [0.42, 0.045], [0.42, 0]]), "propshaft-r"),
            (shaft("propshaft-f", "front-frame", 0.0, [[0, 0], [0, 0.045], [1.175, 0.045], [1.175, 0.09], [1.335, 0.13], [1.335, 0]]), "axle-f"),
        ]
    if sc == "200":
        new += [
            (case("transmission", 0.34), "axle-cradle-r"),
            (shaft("propshaft-r", "rear-frame", -1.32, [[0, 0], [0, 0.05], [1.32, 0.05], [1.32, 0]]), "transmission"),
            (shaft("propshaft-f", "front-frame", 0.0, [[0, 0], [0, 0.05], [1.32, 0.05], [1.32, 0]]), "axle-f"),
        ]
    for solid, after in new:
        put(body, solid, after)
        ref = {"id": "th-" + solid["id"], "of": solid["id"], "region": "front-frame", "layer": "plate"}
        put(vol["thermal"], ref, "th-axle-r")
    for x in vol["structure"]:
        if x["id"] == "member-articulation":
            x["profile"] = [[0, 0], [0, 0.055], [0.16, 0.055], [0.16, 0]]
src = json.load(open(SRC))
for sc in ("50", "100", "200", "500"):
    edit(src["volumes"][sc], sc)
json.dump(src, open(SRC, "w"), ensure_ascii=False, indent=1)
if "--2d" in sys.argv:
    text = open(CAT).read()
    before = json.loads(text)
    asset = before.get("asset", before)
    want = {}
    for sc in asset["representations"]:
        if sc not in DRAWN: continue
        prof = DRAWN[sc]
        g = asset["representations"][sc]["geometry"]
        for view in g:
            if view not in ("front", "plan"): continue
            for k, sh in enumerate(g[view]):
                if sh["id"] not in ("axle-r", "axle-f"): continue
                cx = -1.675 if sh["id"] == "axle-r" else 1.675
                if view == "front":
                    pts = [[-0.77, YS]] + [[z, round(YS + r, 4)] for z, r in prof] + [[0.77, YS]] + [[z, round(YS - r, 4)] for z, r in reversed(prof)]
                else:
                    pts = [[cx, -0.77]] + [[round(cx + r, 4), z] for z, r in prof] + [[cx, 0.77]] + [[round(cx - r, 4), z] for z, r in reversed(prof)]
                want[(sc, view, k)] = pts
                if sh["points"] == pts: continue
                for pad in range(0, 24):
                    old = json.dumps(sh["points"], indent=1).replace("\n", "\n" + " " * pad)
                    if text.count(old):
                        text = text.replace(old, json.dumps(pts, indent=1).replace("\n", "\n" + " " * pad), 1)
                        break
                else:
                    raise SystemExit("shape text not found: %s %s %s" % (sc, view, sh["id"]))
    after = json.loads(text)
    asset2 = after.get("asset", after)
    for (sc, view, k), pts in want.items():
        assert asset2["representations"][sc]["geometry"][view][k]["points"] == pts, (sc, view, k)
        asset["representations"][sc]["geometry"][view][k]["points"] = pts
    assert before == after, "something else changed"
    open(CAT, "w").write(text)
    print("2D axle shapes redrawn:", len(want))
