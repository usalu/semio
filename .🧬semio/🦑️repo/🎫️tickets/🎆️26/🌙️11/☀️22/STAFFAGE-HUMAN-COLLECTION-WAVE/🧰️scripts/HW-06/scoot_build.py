"""🛵️ Generator of `mobility-scooter-user`: the aged man of `scoot_base` on the scooter of `scoot_parts`, with rig, solved poses,
clips, thermal model with the 400 W drive nameplate, structure, every 2D shape and 3D solid of the four scales and all texts.
Run: python3 scoot_build.py"""
import copy
import math
import hw06lib as L
import scoot_base as SB
import scoot_parts as SP
import scoot_texts as X

SLUG = "mobility-scooter-user"
t = L.t
TOKEN = {"hoodie": "jacket", "jeans": "trousers", "sneaker": "shoe", "tee": "shirt", "beanie": "cap"}
REGION = {"hoodie": "jacket", "jeans": "trousers", "beanie": "cap"}
GRIP = (0.082 * SB.S, -0.026 * SB.S, 0.0)


def tok(x):
    return "-".join(TOKEN.get(p, p) for p in x.split("-"))


def rename(asset):
    reg = lambda x: REGION.get(x, x)
    for rep in asset["representations"].values():
        if rep["status"] != "authored": continue
        for shapes in rep["geometry"].values():
            for s in shapes:
                s["id"] = tok(s["id"])
                if "region" in s: s["region"] = reg(s["region"])
                if "regions" in s: s["regions"] = [reg(r) for r in s["regions"]]
    for vol in asset["volumes"].values():
        if vol["status"] != "authored": continue
        for lst in ("body", "thermal", "structure"):
            for s in vol[lst]:
                s["id"] = tok(s["id"])
                if "of" in s: s["of"] = tok(s["of"])
                if "region" in s: s["region"] = reg(s["region"])
                if "layer" in s: s["layer"] = tok(s["layer"])
    for r in asset["thermal"]["regions"]:
        r["id"] = reg(r["id"])
        for layer in r["layers"]: layer["id"] = tok(layer["id"])
    for e in asset["thermal"]["emitters"]:
        if "region" in e: e["region"] = reg(e["region"])
    for p in asset["poses"]:
        p["thermal"]["regions"] = {reg(k): v for k, v in p["thermal"]["regions"].items()}


def arm_ik(bones, base, side, target, wants=(0.0, -0.3)):
    """Solves upper arm, forearm and hand offsets so that the grip point of the hand reaches `target` with the hand pointing ahead."""
    hand, fore, arm = f"hand-{side}", f"forearm-{side}", f"arm-{side}"
    sg = 1 if side == "l" else -1

    def offs(x):
        o = dict(base)
        o[arm] = {"angleDeg": x[0], "abductionDeg": sg * x[1], "twistDeg": sg * x[2]}
        o[fore] = {"angleDeg": x[3], "abductionDeg": sg * x[4], "twistDeg": sg * x[5]}
        o[hand] = {"angleDeg": x[6], "abductionDeg": sg * x[7]}
        return o

    def cost(x):
        f = L.chain(bones, offs(x)); g = L.to_world(f[hand], GRIP)
        e = sum((g[i] - target[i]) ** 2 for i in range(3)) * 1e4
        u = f[hand]["u"]; e += 2 * (u[2] - wants[0]) ** 2 + (u[1] - wants[1]) ** 2
        e += sum(max(0, abs(v) - lim) ** 2 for v, lim in [(x[1], 40), (x[4], 30), (x[7], 30), (x[2], 90), (x[5], 100)]) * 0.01
        e += max(0, -x[3]) ** 2 + max(0, x[3] - 150) ** 2 + max(0, abs(x[6]) - 70) ** 2
        return e + 1e-5 * sum(v * v for v in x)
    best = None
    for start in ([40, 0, 0, 40, 0, 0, 0, 0], [60, 10, 20, 30, 0, 40, 10, 0], [20, -10, -20, 70, 0, -40, 0, 0]):
        x, v = L.nelder_mead(cost, start, 10, 6000)
        if best is None or v < best[1]: best = (x, v)
    o = offs(best[0])
    return {k: {kk: round(vv, 3) for kk, vv in o[k].items()} for k in (arm, fore, hand)}, best[1]


def solve_poses(asset):
    bones = asset["rig"]["bones"]
    P = {p["id"]: p for p in asset["poses"]}
    lap = copy.deepcopy(P["seated-level"]["bones"])
    drive = dict(lap); drive.update({"lumbar": {"angleDeg": -13.0}, "neck": {"angleDeg": 6.0}, "head": {"angleDeg": 4.0}})
    for side, sg in (("l", 1), ("r", -1)):
        sol, err = arm_ik(bones, drive, side, (0.60, 0.865, sg * 0.205))
        drive.update(sol)
    back = dict(drive)
    back.update({"lumbar": {"angleDeg": -7.0}, "neck": {"angleDeg": 2.0}, "head": {"angleDeg": 6.0}})
    sol, _ = arm_ik(bones, back, "l", (0.60, 0.865, 0.205)); back.update(sol)
    sol, _ = arm_ik(bones, back, "r", (0.30, 1.30, -0.34), wants=(-0.3, 0.9)); back.update(sol)
    reach = dict(lap); reach.update({"lumbar": {"angleDeg": -22.0}, "neck": {"angleDeg": 4.0}, "head": {"angleDeg": -12.0}})
    sol, _ = arm_ik(bones, reach, "r", (0.78, 0.75, -0.06), wants=(0.0, -0.8)); reach.update(sol)
    return {"driving": drive, "parked": lap, "greeting": back, "reaching-into-basket": reach}


def shapes_for(part, rest, view):
    kw = dict(z=part.get("z", 4), tone=part["tone"], role=part["role"], region=part["region"], material=part["material"])
    if part["bone"] in ("rear-wheel", "front-wheel"):
        prof = [(p[0], p[1]) for p in part["profile"]]
        sh = L.poly_shape(part["id"], part["bone"], prof, **kw)
        if view != "elevation": sh["solid"] = {"kind": "extruded", "sweep": "w", "z0": part["from"], "z1": part["to"]}
        return sh
    return L.hull_shape(part["id"], part, rest[part["bone"]], view, **kw)


def add_scooter(asset, back_d):
    rest = L.chain(asset["rig"]["bones"])
    added = {}
    for key in L.SCALES:
        rep = asset["representations"][key]; vol = asset["volumes"][key]
        parts = SP.all_parts(key, back_d)
        added[key] = []
        for n, p in enumerate(parts):
            meta = {k: p.pop(k) for k in ("region",)}
            so = {k: v for k, v in p.items()}
            vol["body"].append(so); added[key].append(so)
            vol["thermal"].append({"id": f"th-{so['id']}", "of": so["id"], "region": meta["region"]})
            p["region"] = meta["region"]; p["z"] = 1 + n * 0.01
            for view in L.VIEWS: rep["geometry"][view].append(shapes_for(p, rest, view))
        for pid, member in (("deck", "chassis"), ("rear-shroud", "chassis"), ("battery-box", "chassis"), ("front-nose", "chassis"), ("front-fender", "chassis"),
                            ("seat-base", "chassis"), ("seat-post", "chassis"), ("heel-rest", "chassis"), ("seat-cushion", "chassis"), ("backrest", "chassis"), ("tiller", "chassis"), ("drive-motor", "chassis"), ("front-axle", "chassis")):
            if any(s["id"] == pid for s in vol["body"]): vol["structure"].append({"id": f"st-{pid}", "of": pid, "member": member, "part": "member"})
        vol["structure"].append(SP.seat_load(key))
        for s in [s for s in vol["body"] if s["bone"] in ("rear-wheel", "front-wheel") and ("tyre" in s["id"] or "wheel" in s["id"] or "hub" in s["id"])]:
            vol["structure"].append({"id": f"st-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"})
    L.assign_z(asset, added)


def build():
    doc, asset = SB.base()
    rename(asset)
    bones = asset["rig"]["bones"]
    chassis = next(b for b in bones if b["id"] == "chassis")
    chassis["title"] = t("Mobility scooter chassis with seat, tiller and battery", "Elektromobil-Fahrgestell mit Sitz, Lenksäule und Akku")
    chassis["mass"] = {"kg": 66.0, "comX": 0.2, "comY": 0.08, "distribution": "point", "source": "authored-estimate", "reference": "scooter-catalogues"}
    i = next(n for n, b in enumerate(bones) if b["id"] == "chassis") + 1
    bones[i:i] = [
        {"id": "rear-wheel", "parent": "chassis", "title": t("Rear wheel pair on the transaxle (10 inch)", "Hinterradpaar an der Antriebsachse (10 Zoll)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": SP.R_WHEEL,
         "joint": {"type": "revolute"}, "mass": {"kg": 4.2, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "scooter-catalogues"}},
        {"id": "front-wheel", "parent": "chassis", "title": t("Front wheel pair (10 inch)", "Vorderradpaar (10 Zoll)"), "x": SP.WHEELBASE, "y": 0, "angleDeg": 0, "lengthM": SP.R_WHEEL,
         "joint": {"type": "revolute"}, "mass": {"kg": 3.8, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "scooter-catalogues"}},
    ]
    rest = L.chain(bones)
    back_d = SP.backrest_frame(rest, asset)
    poses = solve_poses(asset)
    add_scooter(asset, back_d)
    X.finish(asset, poses)
    doc["asset"] = L.rnd(asset)
    return doc


if __name__ == "__main__":
    path, p = L.finalize(SLUG, build())
    print(path, p["powerW"], {k: (v["height"], v["width"], round(v["front"]["horizontal"], 3), round(v["plan"]["vertical"], 3)) for k, v in p["rows"].items()})
