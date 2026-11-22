"""🧍️ Builds `office-worker-carrying-laptop`: an office worker of 1.81 m (80 kg) in overcoat and scarf walking to a meeting with a
closed 15.6-inch laptop carried on the near forearm and pressed against the hip by the upper arm. Body, rig, outlines and the heel-to-toe
walk are derived from `walking-commuter` (scaled by F); the shoulder bag is removed; the laptop is drawn here per scale and view with its
solids; the carrying arm, poses, clips, thermal and structure are authored here.

python3 build_carrier.py [--out path]
"""
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from hw import *  # noqa: E402,F403
from texts_carrier import TEXTS  # noqa: E402

ID = "office-worker-carrying-laptop"
F = 1.025
MASS = 80.0
BASE_MASS = 74.7
G = 1 / F
LAP_L, LAP_H, LAP_T = 0.357 * G, 0.245 * G, 0.022 * G
CARRY = {"arm-r": {"angleDeg": -6}, "forearm-r": {"angleDeg": 84}, "hand-r": {"angleDeg": -8}}
U0, V0, W0 = -0.075 * G, 0.032 * G, 0.03 * G


def rig_props(a):
    a["rig"]["bones"].append({"id": "laptop", "parent": "forearm-r", "title": t("Closed laptop (15.6 inch) resting on the forearm", "Zugeklappter Laptop (15,6 Zoll) auf dem Unterarm"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": round(LAP_L, 5), "joint": {"type": "fixed"}, "mass": {"kg": 1.9, "comX": round(U0 + LAP_L / 2, 4), "comY": round(V0 + LAP_H / 2, 4), "distribution": "point", "source": "authored-estimate"}})


def parts(scale):
    """🧱️ The closed laptop as it stands on its long edge on the forearm (u along the forearm, v up the lid, w medial towards the hip)."""
    p = [Part("laptop-shell", "laptop", [LAP_L, LAP_H, LAP_T], [U0 + LAP_L / 2, V0 + LAP_H / 2, W0], "laptop", "laptop-aluminium", 0.68)]
    if scale == "50":
        p.append(Part("laptop-seam", "laptop", [LAP_L - 0.01, LAP_H - 0.01, 0.002], [U0 + LAP_L / 2, V0 + LAP_H / 2, W0 - LAP_T / 2 + 0.0015], None, "laptop-aluminium", 0.6, role="detail", filled=False, views=("elevation",), solid=False))
        p.append(Part("laptop-logo", "laptop", [0.03 * G, 0.03 * G, 0.002], [U0 + LAP_L / 2, V0 + LAP_H / 2, W0 - LAP_T / 2], None, "laptop-aluminium", 0.5, role="detail", filled=False, views=("elevation",), solid=False))
    for x in p:
        x.swept = True
        if scale in ("200", "500") and x.id == "laptop-shell":
            x.draw_w = LAP_T - 0.001
        x.zv = {"elevation": 5.0 + (0.01 if x.role == "detail" else 0), "front": 5.0, "plan": 6.0}
    return p


def add_props(a, frames):
    for sc in SCALES:
        ps = parts(sc)
        add_parts(a, sc, ps, frames, 5.0)
        vol = a["volumes"][sc]
        for p in ps:
            if p.solid:
                vol["thermal"].append({"id": f"th-{p.id}", "of": p.id, "region": p.region})
                vol["structure"].append({"id": f"st-{p.id}", "of": p.id, "member": "laptop", "part": "member"})
        vol["structure"].append({"id": "mass-laptop", "kind": "ellipsoid", "bone": "laptop", "material": "laptop-aluminium", "role": "detail", "tone": 0.2, "radii": [0.02, 0.02, 0.012] if sc in ("50", "100") else [0.04, 0.04, 0.04], "transform": {"translation": [round(U0 + LAP_L / 2, 5), round(V0 + LAP_H / 2, 5), W0]}, "member": "laptop", "part": "mass"})


def hand500(a, frames):
    """✋️ The 1:500 drawing of the near hand, which the base hid behind the bag strap: the outline of its 1:500 ellipsoid, swept across
    its lateral extent, in all three views."""
    import math
    e = next(x for x in a["volumes"]["500"]["body"] if x.get("bone") == "hand-r")
    (ru, rv, rw), (cu, cv, cw) = e["radii"], e["transform"]["translation"]
    pts = [[r(cu + ru * math.cos(2 * math.pi * i / 16)), r(cv + rv * math.sin(2 * math.pi * i / 16))] for i in range(16)]
    zc = frames["hand-r"]["o"][2] + cw
    solid = {"kind": "extruded", "sweep": "w", "z0": r(zc - rw), "z1": r(zc + rw)}
    g = a["representations"]["500"]["geometry"]
    z = {x["id"]: x["z"] for x in g["elevation"] if x["bone"] == "forearm-r"}.get("forearm-r", 5.0)
    g["elevation"].append(poly("hand-r", "hand-r", pts, round(z + 0.01, 3), 0.78, "hand-skin", "skin"))
    for view in ("front", "plan"):
        zv = {x["id"]: x["z"] for x in g[view] if x["bone"] == "forearm-r"}.get("forearm-r", 5.0)
        g[view].append(poly("hand-r", "hand-r", pts, round(zv + 0.01, 3), 0.78, "hand-skin", "skin", solid=dict(solid)))


def carry(bones, swing=0.0):
    out = {k: dict(v) for k, v in bones.items()}
    out.update({k: dict(v) for k, v in CARRY.items()})
    out["arm-r"]["angleDeg"] = CARRY["arm-r"]["angleDeg"] + swing
    out["hand-r"] = dict(CARRY["hand-r"])
    return out


def build(out):
    d = load("walking-commuter")
    a = d["asset"]
    a["id"] = ID
    remove_bones(a, ["bag"], drop_regions=("bag-leather",))
    a["anchors"] = [x for x in a["anchors"] if x["id"] != "strap-grip"]
    rig_props(a)
    tmp = f"{GEN}/tmp/{ID}-rig.json"
    save(d, tmp)
    frames = rest_frames(tmp)
    add_props(a, frames)
    sweep_from_elevation(a, ("forearm-r", "hand-r"))
    hand500(a, frames)
    a["volumes"]["50"]["body"].append({"id": "cuff-under-laptop", "kind": "box", "bone": "forearm-r", "material": "wool-coat", "role": "body", "tone": 0.3, "size": [0.035, 0.05, 0.016], "transform": {"translation": [0.2475, -0.01, 0.0175]}})
    a["representations"]["500"]["pens"]["body"] = max(a["representations"]["500"]["pens"]["body"], 0.18)
    a["materials"] += [m for m in [{"id": "laptop-aluminium", "title": t("Anodised aluminium laptop shell", "Eloxierte Aluminiumschale des Laptops"), "emissivity": 0.8, "solarAbsorptivity": 0.5, "source": "authored-estimate"}] if m["id"] not in {x["id"] for x in a["materials"]}]
    a["anchors"].append({"id": "laptop-grip", "bone": "hand-r", "x": 0.05, "y": 0.03, "kind": "attachment", "title": t("Fingers round the front edge of the laptop", "Finger um die Vorderkante des Laptops")})
    th = a["thermal"]
    th["regions"].append({"id": "laptop", "title": t("Closed laptop in standby", "Zugeklappter Laptop im Standby"), "kind": "surface", "model": "passive", "material": "laptop-aluminium", "areaM2": 0.19, "characteristicLengthM": 0.25, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 4000, "ratedPowerW": 65, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "iec-62623-2012"})
    poses = {p["id"]: p for p in a["poses"]}
    keep = []
    for p in a["poses"]:
        if p["id"] == "check-watch":
            continue
        p["bones"] = carry(p["bones"], {"contact-left": 2, "loading-left": 1.5, "passing-left": 0, "contact-right": -2, "loading-right": -1.5, "passing-right": 0}.get(p["id"], 0))
        p["thermal"]["regions"]["laptop"] = {"exposedFraction": 0.8, "sunlitFraction": 0}
        keep.append(p)
    a["poses"] = keep
    del poses
    a["clips"] = [c for c in a["clips"] if c["id"] != "glance-at-watch"]
    for c in a["clips"]:
        n = len(c["keys"])
        for i, k in enumerate(c["keys"]):
            if c["id"] == "walk-cycle":
                import math
                k["bones"] = carry(k.get("bones", {}), round(2 * math.cos(2 * math.pi * k["t"] / c["durationS"]), 3))
            elif "bones" in k:
                k["bones"] = carry(k["bones"])
        del n
    a["presentation"] = {"defaultPose": "standing", "defaultClip": "walk-cycle", "defaultScale": 100}
    scale_asset(a, F, MASS / BASE_MASS)
    for b in a["rig"]["bones"]:
        if b["id"] == "laptop":
            b["mass"]["kg"] = 1.9
    body = th["body"]
    stature = 1.77 * F
    body["massKg"] = MASS
    body["skinAreaM2"] = round(dubois(MASS, stature), 3)
    body["basalMetabolismW"] = round(mifflin(MASS, stature, 29, True), 1)
    TEXTS(a, stature)
    apply_ops(a, ID)
    prune_materials(a)
    save(d, out)
    dims = json.loads(bun(f"{W}/dims.ts", out))
    a["dimensions"] = {"heightM": dims["heightM"], "widthM": dims["widthM"], "depthM": dims["depthM"], "massKg": dims["massKg"]}
    save(d, out)
    return d


if __name__ == "__main__":
    out = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else f"{CAT}/{ID}/🔣️.json"
    build(out)
    print(out)
