"""🛵️ Stage 1 of `mobility-scooter-user`: the seated man of `wheelchair-user` without his chair, aged to 78 (1.72 m, 76 kg) and set on
the chassis root of a four-wheel mobility scooter (rear axle origin, 125 mm wheel radius).

Anthropometry: DIN 33402-2 (men 61-65, P50 1.74 m) minus the age loss of stature (about 1 cm per decade after 65, Sorkin et al. 1999)."""
import copy
import hw06lib as L

STATURE_OLD, STATURE = 1.75, 1.72
S = STATURE / STATURE_OLD
BODY_OLD, BODY = 75.0, 76.0
CHAIR = {"frame", "rear-wheel", "castor"}
AXLE = 0.125
DECK = 0.152
PELVIS_X = 0.05


def strip_chair(asset):
    for rep in asset["representations"].values():
        if rep["status"] != "authored": continue
        for view in rep["geometry"]:
            keep = []
            for s in rep["geometry"][view]:
                if s["bone"] not in CHAIR: keep.append(s)
                elif s["role"] == "ground" and view == "elevation":
                    s = copy.deepcopy(s); s["bone"] = "chassis"
                    L._map_shape(s, lambda p: (p[0], p[1] + 0.305 - AXLE)); keep.append(s)
            rep["geometry"][view] = keep
    for vol in asset["volumes"].values():
        if vol["status"] != "authored": continue
        gone = {s["id"] for s in vol["body"] if s["bone"] in CHAIR}
        vol["body"] = [s for s in vol["body"] if s["id"] not in gone]
        for lst in ("thermal", "structure"):
            vol[lst] = [s for s in vol[lst] if not ("of" in s and s["of"] in gone) and s.get("bone") not in CHAIR and s.get("member") not in CHAIR]
    asset["rig"]["bones"] = [b for b in asset["rig"]["bones"] if b["id"] not in ("rear-wheel", "castor")]
    for b in asset["rig"]["bones"]:
        if b["id"] == "frame":
            b.update({"id": "chassis", "x": 0.0, "y": AXLE, "lengthM": 0.92, "angleDeg": 0})
        if b["parent"] == "frame": b["parent"] = "chassis"
    asset["rig"]["root"] = "chassis"
    asset["anchors"] = [a for a in asset["anchors"] if a["bone"] not in CHAIR and a["id"] not in ("rim-grip", "rim-grip-l")]
    asset["thermal"]["regions"] = [r for r in asset["thermal"]["regions"] if r["id"] not in ("frame-metal", "tyres", "cushion")]
    asset["materials"] = [m for m in asset["materials"] if m["id"] not in ("aluminium", "rubber", "foam")]
    for p in asset["poses"]:
        p["bones"] = {k: v for k, v in p["bones"].items() if k not in CHAIR}
        for r in ("frame-metal", "tyres", "cushion"): p["thermal"]["regions"].pop(r, None)


def lowest(asset, bones_ids, frames, axis=1, high=False):
    v = asset["volumes"]["50"]["body"]; best = None
    for s in v:
        if s["bone"] not in bones_ids or s["kind"] in ("polyhedron", "revolve"): continue
        for p in L.samples(s):
            q = L.to_world(frames[s["bone"]], p)[axis]
            if best is None or (q > best if high else q < best): best = q
    return best


def base():
    doc = L.load("wheelchair-user")
    asset = copy.deepcopy(doc["asset"])
    fr = L.chain(asset["rig"]["bones"])
    old = {("chassis" if k == "frame" else k): v for k, v in fr.items()}
    drop = fr["pelvis"]["o"][1] - lowest(asset, {"foot-l", "toe-l"}, fr)
    strip_chair(asset)
    bones = asset["rig"]["bones"]
    pelvis_y = DECK + drop * S + 0.002
    factor = {b["id"]: (1.0 if b["id"] == "chassis" else S) for b in bones}
    L.rescale(asset, factor, lambda b: (PELVIS_X, pelvis_y - AXLE) if b["id"] == "pelvis" else None, old)
    k = BODY / BODY_OLD
    for b in bones:
        if "mass" in b and b["id"] != "chassis": b["mass"]["kg"] = round(b["mass"]["kg"] * k, 3)
    return doc, asset
