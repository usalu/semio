"""👕️ Generator of `humans/person-hanging-laundry`: the archive's 1.75 m man in softshell jacket, cap and chinos (body, rig and outlines of
`tour-guide`, the pointer, flag, lanyard and badge removed) hanging a wet T-shirt on a clothesline with two wooden pegs; the laundry
basket stands on the ground in front of his feet.

New: the shirt (with its pegs) carried by the right fist, a 1.2 m stretch of clothesline and the basket with its laundry as fixed parts
placed by the feet (they stay where they are while the person bends), four passive regions, three poses solved for the grips (kit.ts), the
hang-shirt loop and idle breathing at the line. Run: `python3 laundry.py [--solve]`.
"""
import copy
import json
import math
import sys

from lib import (GEN, SCALES, add_part, bun, find, load, mass_marker, prism_part, rect, rest_frames, revolved_part, save, strip_bones, strip_regions, strip_shapes, strip_solids, t)
import texts_laundry as TX

SLUG = "person-hanging-laundry"
GRIP = 0.08
SHIRT_W = 0.44
LINE = (0.42, 1.80)
LINE_SPAN = (-0.60, 0.60)
LINE_U = 0.006
BASKET = (0.52, 0.0)
BASKET_HALF = (0.19, 0.28)
BASKET_H = 0.27

LEGS = {
    "stand": {"thigh-l": {"angleDeg": 3}, "shank-l": {"angleDeg": -5.89}, "foot-l": {"angleDeg": 2.89}, "toe-l": {"angleDeg": 0},
              "thigh-r": {"angleDeg": 1}, "shank-r": {"angleDeg": -5}, "foot-r": {"angleDeg": 4}, "toe-r": {"angleDeg": 0}},
    "bend": {"thigh-l": {"angleDeg": 38}, "shank-l": {"angleDeg": -60}, "foot-l": {"angleDeg": 22}, "toe-l": {"angleDeg": 0},
             "thigh-r": {"angleDeg": 36}, "shank-r": {"angleDeg": -58}, "foot-r": {"angleDeg": 22}, "toe-r": {"angleDeg": 0}},
}
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]
POSE_SPEC = {
    "pegging-shirt": dict(legs="stand", trunk=4, neck=-6, head=-14, right=(LINE[0] - LINE_U, LINE[1], -0.22), plane="x", left="shirt"),
    "shaking-out": dict(legs="stand", trunk=-3, neck=4, head=6, right=(0.40, 1.32, -0.22), plane="x", left="shirt"),
    "lifting-from-basket": dict(legs="bend", trunk=-52, neck=12, head=16, right=(BASKET[0] - 0.02, 0.84, -0.22), plane="x", left="shirt"),
}


def unit(v):
    n = math.sqrt(sum(x * x for x in v))
    return [x / n for x in v]


def strip(a):
    strip_bones(a, ["pointer", "flag-pole"])
    strip_regions(a, ["badge", "pointer-metal", "pole-metal", "flag-cloth"])
    strip_shapes(a, ["lanyard-l", "lanyard-r", "badge", "badge-edge"])
    for sc in SCALES:
        strip_solids(a["volumes"][sc], lambda s: any(k in s["id"] for k in ("lanyard", "badge")))
    a["anchors"] = [x for x in a["anchors"] if x["id"] not in ("shoulder-r",)]
    a["materials"] = [m for m in a["materials"] if m["id"] not in ("pointer-aluminium", "flag-textile")]
    for side in ("l", "r"):
        foot = next(s for s in a["volumes"]["500"]["structure"] if s["id"] == f"foot-bones-{side}")
        foot["members"] = [f"toe-{side}"]


def rotation_rest(parent, target):
    """🧮️ (angleDeg, abductionDeg, twistDeg) of a fixed child whose frame (columns u, v, w in world) is `target` under the posed `parent` frame: R = Rw(a)·Rv(b)·Ru(c)."""
    P = [parent["u"], parent["v"], parent["w"]]
    M = [[sum(P[i][k] * target[j][k] for k in range(3)) for j in range(3)] for i in range(3)]
    ru = [M[0][0], M[1][0], M[2][0]]
    b = math.asin(max(-1, min(1, ru[2])))
    a_ = math.atan2(ru[1], ru[0])
    ca, sa, cb, sb = math.cos(a_), math.sin(a_), math.cos(b), math.sin(b)
    Rw = [[ca, -sa, 0], [sa, ca, 0], [0, 0, 1]]
    Rv = [[cb, 0, -sb], [0, 1, 0], [sb, 0, cb]]
    RwRv = [[sum(Rw[i][k] * Rv[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    Ru = [[sum(RwRv[k][i] * M[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    c = math.atan2(-Ru[2][1], Ru[1][1])
    return round(math.degrees(a_), 4), round(math.degrees(b), 4), round(math.degrees(c), 4)


def local_of(parent, p):
    d = [p[k] - parent["o"][k] for k in range(3)]
    return [round(sum(d[k] * parent[ax][k] for k in range(3)), 5) for ax in ("u", "v", "w")]


def fixed_parts(a, draft):
    """📌️ Places the clothesline (under the left foot's frame) and the basket (under the right foot's) where they stand in the pegging pose."""
    bun("kit.ts", "frames", draft, "hw13-legs", f"{GEN}/laundry-legs.json")
    fr = json.load(open(f"{GEN}/laundry-legs.json"))["hw13-legs"]["frames"]
    fl, frr = fr["foot-l"], fr["foot-r"]
    line_origin = [LINE[0], LINE[1], LINE_SPAN[0]]
    line_frame = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]]
    x, y, z = local_of(fl, line_origin)
    ang, abd, tw = rotation_rest(fl, line_frame)
    basket_origin = [BASKET[0], 0.0, BASKET[1]]
    basket_frame = [[0, 1, 0], [-1, 0, 0], [0, 0, 1]]
    bx, by, bz = local_of(frr, basket_origin)
    bang, babd, btw = rotation_rest(frr, basket_frame)
    bones = a["rig"]["bones"]
    bones.append({"id": "clothesline", "parent": "foot-l", "title": TX.BONES["clothesline"], "x": x, "y": y, "z": z, "angleDeg": ang, "abductionDeg": abd, "twistDeg": tw, "lengthM": LINE_SPAN[1] - LINE_SPAN[0], "joint": {"type": "fixed"}})
    bones.append({"id": "basket", "parent": "foot-r", "title": TX.BONES["basket"], "x": bx, "y": by, "z": bz, "angleDeg": bang, "abductionDeg": babd, "twistDeg": btw, "lengthM": 0.31, "joint": {"type": "fixed"},
                  "mass": {"kg": 3.9, "comX": 0.12, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "ainsworth-2011"}})
    a["anchors"].append({"id": "basket-base", "bone": "basket", "x": 0, "y": 0, "kind": "contact", "title": TX.ANCHORS["basket-base"], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.36, "footprintWidthM": 0.54, "frictionCoefficient": 0.5})


def rig(a):
    a["rig"]["bones"].append({"id": "shirt", "parent": "hand-r", "title": TX.BONES["shirt"], "x": GRIP, "y": 0, "angleDeg": 0, "lengthM": SHIRT_W,
                              "joint": {"type": "revolute", "minDeg": -170, "maxDeg": 170, "twistMinDeg": -180, "twistMaxDeg": 180},
                              "mass": {"kg": 0.36, "comX": 0.0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "iso-9920-2007"}})


def spec_of(pid, s, a):
    legs = LEGS[s["legs"]]
    bones = {**legs, "torso": {"angleDeg": s["trunk"]}, "neck": {"angleDeg": s["neck"]}, "head": {"angleDeg": s["head"]}}
    pr = list(s["right"])
    pl = [pr[0], pr[1], pr[2] + SHIRT_W]
    arm = lambda side: [[f"clavicle-{side}", "angleDeg", -8, 35], [f"upper-arm-{side}", "angleDeg", -60, 180], [f"upper-arm-{side}", "abductionDeg", -80, 80], [f"upper-arm-{side}", "twistDeg", -120, 120], [f"forearm-{side}", "angleDeg", 0, 150], [f"forearm-{side}", "twistDeg", -80, 80], [f"hand-{side}", "angleDeg", -65, 75], [f"hand-{side}", "abductionDeg", -30, 30]]
    seed = {"pegging-shirt": (120, 60), "shaking-out": (50, 60), "lifting-from-basket": (40, 20)}[pid]
    for side, sg in (("l", 1), ("r", -1)):
        bones[f"upper-arm-{side}"] = {"angleDeg": seed[0], "abductionDeg": sg * 8, "twistDeg": 0}
        bones[f"forearm-{side}"] = {"angleDeg": seed[1]}
    stages = [
        {"vars": arm("r") + [["shirt", "angleDeg", -170, 170], ["shirt", "abductionDeg", -89, 89], ["shirt", "twistDeg", -180, 180]], "terms": [
            {"kind": "point", "bone": "hand-r", "local": [GRIP, 0, 0], "target": pr},
            {"kind": "axis", "bone": "shirt", "axis": "u", "direction": [1, 0, 0], "weight": 30},
            {"kind": "axis", "bone": "shirt", "axis": "v", "direction": [0, 1, 0], "weight": 30}], "regularise": 2e-4},
        {"vars": arm("l"), "terms": [{"kind": "point", "bone": "hand-l", "local": [GRIP, 0, 0], "target": pl}], "regularise": 2e-4},
    ]
    return {"id": pid, "bones": bones, "contacts": FEET, "stages": stages}


def solve(a, draft, redo):
    out = f"{GEN}/laundry-solved.json"
    if redo:
        with open(f"{GEN}/laundry-spec.json", "w") as f:
            json.dump({"poses": [spec_of(pid, s, a) for pid, s in POSE_SPEC.items()]}, f)
        bun("kit.ts", "solve", draft, f"{GEN}/laundry-spec.json", out)
    with open(out) as f:
        return json.load(f)


def thermal_of(a, pid, src):
    th = copy.deepcopy(src)
    th["regions"] = {k: v for k, v in th["regions"].items() if k in {r["id"] for r in a["thermal"]["regions"]}}
    th["activityMet"] = {"pegging-shirt": 2.3, "shaking-out": 2.5, "lifting-from-basket": 2.8}[pid]
    th["breathing"] = "pegging"
    for r in ("wet-shirt", "line-and-pegs"):
        th["regions"][r] = {"exposedFraction": 1, "sunlitFraction": 0.8}
    th["regions"]["basket"] = {"exposedFraction": 0.8, "sunlitFraction": 0.5, "contact": {"fraction": 0.15, "tempC": 20}}
    th["regions"]["basket-laundry"] = {"exposedFraction": 0.6, "sunlitFraction": 0.5}
    th["regions"]["hand-skin"] = {"exposedFraction": 0.8, "sunlitFraction": 0.5}
    th["emitters"] = {"plume": 0.8}
    return th


# ------------------------------------------------------------------ parts
SHIRT = dict(region="wet-shirt", material="wet-cotton")
PEG = dict(region="line-and-pegs", material="beech-peg")
ROPE = dict(region="line-and-pegs", material="pp-line")
BASK = dict(region="basket", material="pp-basket")
WASH = dict(region="basket-laundry", material="wet-cotton")


def shirt_profile(scale):
    """👕️ The T-shirt hanging from its shoulders, as (v, w): v up from the pegged shoulder line, w from the right to the left shoulder."""
    if scale == "500":
        return [[0.0, -0.02], [-0.04, -0.16], [-0.24, -0.16], [-0.70, -0.04], [-0.70, 0.48], [-0.24, 0.60], [-0.04, 0.60], [0.0, 0.46]]
    if scale == "200":
        return [[0.0, 0.0], [-0.03, -0.10], [-0.20, -0.16], [-0.24, -0.04], [-0.70, -0.03], [-0.70, 0.47], [-0.24, 0.48], [-0.20, 0.60], [-0.03, 0.54], [0.0, 0.44], [0.0, 0.30], [-0.05, 0.22], [0.0, 0.14]]
    pts = [[0.0, 0.0], [-0.012, -0.05], [-0.03, -0.10], [-0.20, -0.16], [-0.235, -0.05], [-0.22, -0.035], [-0.45, -0.032], [-0.69, -0.03], [-0.70, 0.02], [-0.70, 0.42], [-0.69, 0.47],
           [-0.45, 0.472], [-0.22, 0.475], [-0.235, 0.49], [-0.20, 0.60], [-0.03, 0.54], [-0.012, 0.49], [0.0, 0.44], [0.0, 0.31], [-0.03, 0.27], [-0.055, 0.22], [-0.03, 0.17], [0.0, 0.13]]
    return pts if scale == "50" else [p for i, p in enumerate(pts) if i not in (1, 5, 6, 11, 12, 16, 19, 21)]


def add_laundry(a, scale):
    sh_lo, sh_hi = {"50": (-0.003, 0.003), "100": (-0.003, 0.003), "200": (-0.004, 0.004), "500": (-0.0125, 0.0125)}[scale]
    shapes, solid = prism_part("shirt", "shirt", shirt_profile(scale), "u", sh_lo, sh_hi, tone=0.82, z=7.0, sheet=True, **SHIRT)
    add_part(a, scale, shapes, solid, "wet-shirt", structure="shirt")
    if scale != "500":
        pw = 0.006 if scale in ("50", "100") else 0.008
        for name, w in (("peg-r", 0.016), ("peg-l", SHIRT_W - 0.016)):
            prof = rect(-0.045, 0.030, w - pw, w + pw, 0.002 if scale == "50" else 0.0)
            shapes, solid = prism_part(name, "shirt", prof, "u", -0.008, 0.016, tone=0.5, z=7.2, **PEG)
            add_part(a, scale, shapes, solid, "line-and-pegs", structure="shirt")
            if scale == "50":
                shapes, solid = prism_part(f"{name}-spring", "shirt", rect(-0.006, 0.006, w - pw - 0.001, w + pw + 0.001), "u", -0.009, 0.017, tone=0.2, z=7.25, role="detail", **dict(PEG, material="steel-spring"))
                add_part(a, scale, shapes, solid, "line-and-pegs")
    r = {"50": 0.0025, "100": 0.0025, "200": 0.004, "500": 0.0125}[scale]
    shapes, solid = revolved_part("clothesline", "clothesline", [[0, 0], [0, r], [LINE_SPAN[1] - LINE_SPAN[0], r], [LINE_SPAN[1] - LINE_SPAN[0], 0]], tone=0.35, z=6.9, slender=True, **ROPE)
    add_part(a, scale, shapes, solid, "line-and-pegs", structure="clothesline")
    bv, bw = BASKET_HALF
    rr = {"50": 0.03, "100": 0.025, "200": 0.0, "500": 0.0}[scale]
    outer = rect(-bv, bv, -bw, bw, rr, 3) if rr else rect(-bv, bv, -bw, bw)
    shapes, solid = prism_part("basket", "basket", outer, "u", 0.0, BASKET_H, tone=0.62, z=1.0, **BASK)
    if scale in ("50", "100", "200"):
        wall = {"50": 0.008, "100": 0.015, "200": 0.03}[scale]
        inner = rect(-bv + wall, bv - wall, -bw + wall, bw - wall, max(rr - wall, 0.0), 3) if rr else rect(-bv + wall, bv - wall, -bw + wall, bw - wall)
        solid["holes"] = [inner[::-1] if _area(inner) > 0 else inner]
        solid["from"] = 0.0
        add_part(a, scale, shapes, solid, "basket", structure="basket")
        floor_shapes, floor = prism_part("basket-floor", "basket", outer, "u", 0.0, max(wall, 0.012), tone=0.62, z=0.99, **BASK)
        add_part(a, scale, {}, floor, "basket", structure="basket")
    else:
        add_part(a, scale, shapes, solid, "basket", structure="basket")
    if scale in ("50", "100"):
        for name, sg in (("basket-grip-l", 1), ("basket-grip-r", -1)):
            prof = rect(-0.07, 0.07, sg * bw - (0.0 if sg > 0 else 0.018), sg * bw + (0.018 if sg > 0 else 0.0))
            shapes, solid = prism_part(name, "basket", prof, "u", 0.21, 0.255, tone=0.55, z=1.05, **BASK)
            add_part(a, scale, shapes, solid, "basket")
    inset = {"50": 0.012, "100": 0.02, "200": 0.03, "500": 0.0}[scale]
    top = {"50": 0.29, "100": 0.29, "200": 0.30, "500": 0.31}[scale]
    mound = rect(-bv + inset, bv - inset, -bw + inset, bw - inset, 0.04 if scale == "50" else 0.0, 3) if scale == "50" else rect(-bv + inset, bv - inset, -bw + inset, bw - inset)
    shapes, solid = prism_part("laundry-mound", "basket", mound, "u", 0.012 if scale != "500" else 0.0, top, tone=0.85, z=1.1, **WASH)
    add_part(a, scale, shapes, solid, "basket-laundry")
    if scale == "50":
        shapes, solid = prism_part("laundry-top", "basket", rect(-bv + 0.06, bv - 0.07, -bw + 0.08, bw - 0.10, 0.03, 3), "u", top - 0.002, top + 0.022, tone=0.8, z=1.12, **WASH)
        add_part(a, scale, shapes, solid, "basket-laundry")
    order = {"basket": (1.0, 9.0, 0.5), "basket-grip-l": (1.05, 9.05, 0.55), "basket-grip-r": (1.05, 9.05, 0.55), "laundry-mound": (0.9, 8.9, 0.6), "laundry-top": (0.92, 8.92, 0.62)}
    for i, v in enumerate(("elevation", "front", "plan")):
        for sh in a["representations"][scale]["geometry"][v]:
            if sh["bone"] == "basket" and sh["id"] in order:
                sh["z"] = order[sh["id"]][i]
    vol = a["volumes"][scale]
    vol["structure"].append(mass_marker(a, scale, "shirt", 0.36, [0.0, -0.30, 0.22], "wet-cotton"))
    vol["structure"].append(mass_marker(a, scale, "basket", 3.9, [0.12, 0, 0], "pp-basket"))


def _area(p):
    return sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p))) / 2


# ------------------------------------------------------------------ assembly


def clips(a):
    hang = {"id": "hang-shirt", "title": TX.CLIPS["hang-shirt"][0], "description": TX.CLIPS["hang-shirt"][1], "durationS": 7, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "lifting-from-basket", "ease": "smooth"}, {"t": 1.6, "pose": "shaking-out", "ease": "smooth"},
                     {"t": 2.0, "pose": "shaking-out", "ease": "smooth", "bones": shake(a, 6)}, {"t": 2.3, "pose": "shaking-out", "ease": "smooth", "bones": shake(a, -4)},
                     {"t": 2.6, "pose": "shaking-out", "ease": "smooth"}, {"t": 3.8, "pose": "pegging-shirt", "ease": "smooth"}, {"t": 5.6, "pose": "pegging-shirt", "ease": "smooth"},
                     {"t": 7, "pose": "lifting-from-basket", "ease": "smooth"}],
            "contacts": [{"anchor": x, "fromS": 0, "toS": 7} for x in FEET + ["basket-base"]],
            "phases": [{"id": "take", "fromS": 0, "toS": 1.6, "title": t("Take a shirt from the basket", "Ein Hemd aus dem Korb nehmen")}, {"id": "shake", "fromS": 1.6, "toS": 2.6, "title": t("Shake it out", "Ausschütteln")},
                       {"id": "peg", "fromS": 2.6, "toS": 5.6, "title": t("Raise and peg it", "Hochheben und anklammern")}, {"id": "return", "fromS": 5.6, "toS": 7, "title": t("Back to the basket", "Zurück zum Korb")}],
            "events": [{"t": 5.0, "kind": "motion", "title": t("Pegs pressed on", "Klammern aufgedrückt"), "target": "shirt"}],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 7, "v": 1}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.8}, {"t": 2.3, "v": 2.5}, {"t": 4.6, "v": 2.3}, {"t": 7, "v": 2.8}]}]}
    peg = find(a["poses"], "pegging-shirt")["bones"]

    def nudge(**kw):
        return {k: {**peg[k], "angleDeg": round(peg[k]["angleDeg"] + v, 2)} for k, v in kw.items()}
    idle = {"id": "idle-breathing", "title": TX.CLIPS["idle-breathing"][0], "description": TX.CLIPS["idle-breathing"][1], "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "pegging-shirt", "ease": "smooth"}, {"t": 1.25, "pose": "pegging-shirt", "ease": "smooth", "bones": nudge(torso=0.4, head=-1.2)}, {"t": 2.5, "pose": "pegging-shirt", "ease": "smooth", "bones": nudge(torso=-0.3, head=0.5)},
                     {"t": 3.75, "pose": "pegging-shirt", "ease": "smooth", "bones": nudge(torso=0.3, head=-0.8)}, {"t": 5, "pose": "pegging-shirt", "ease": "smooth"}],
            "contacts": [{"anchor": x, "fromS": 0, "toS": 5} for x in FEET + ["basket-base"]], "phases": [{"id": "hold", "fromS": 0, "toS": 5, "title": t("Holding the shirt at the line", "Das Hemd an der Leine halten")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]}]}
    return [hang, idle]


def shake(a, deg):
    b = find(a["poses"], "shaking-out")["bones"]
    return {k: {**b[k], "angleDeg": round(b[k]["angleDeg"] + deg * f, 2)} for k, f in (("forearm-l", 1.0), ("forearm-r", 1.0), ("hand-l", -1.5), ("hand-r", -1.5))}


def base():
    a = load("tour-guide")
    a["id"] = SLUG
    a["title"] = TX.TITLE
    a["summary"] = TX.SUMMARY
    a["tags"] = ["human", "adult", "standing", "chores", "domestic", "laundry", "clothesline", "basket", "outdoor"]
    strip(a)
    a["materials"] += TX.MATERIALS
    pv = a["provenance"]
    pv["author"] = "Semio staffage human collection worker HW-13"
    pv["created"] = "2026-11-22"
    pv["references"] += TX.REFERENCES
    pv["notes"] = TX.NOTES
    rig(a)
    a["thermal"]["regions"] += TX.REGIONS
    a["thermal"]["respiration"]["patterns"] = [p for p in a["thermal"]["respiration"]["patterns"] if p["id"] == "rest"] + [TX.PEG_BREATH]
    a["thermal"]["assumptions"] = TX.THERMAL_ASSUMPTIONS
    a["structure"]["supports"] = []
    a["structure"]["assumptions"] = TX.STRUCTURE_ASSUMPTIONS
    th = copy.deepcopy(a["poses"][0]["thermal"])
    th["breathing"] = "rest"
    legs = {"id": "hw13-legs", "title": t("Legs", "Beine"), "description": t("Legs", "Beine"), "bones": LEGS["stand"], "ground": "auto", "contacts": FEET, "thermal": th, "loads": []}
    a["poses"] = [legs]
    draft = save(a, SLUG, f"{GEN}/draft/{SLUG}/🔣️.json")
    fixed_parts(a, draft)
    return a


def main():
    redo = "--solve" in sys.argv
    a = base()
    draft = save(a, SLUG, f"{GEN}/draft/{SLUG}/🔣️.json")
    solved = solve(a, draft, redo)
    src = a["poses"][0]["thermal"]
    a["poses"] = []
    for pid in POSE_SPEC:
        a["poses"].append({"id": pid, "title": TX.POSES[pid][0], "description": TX.POSES[pid][1], "bones": solved[pid]["bones"], "ground": "auto", "contacts": FEET + ["basket-base"], "thermal": None, "loads": []})
    for p in a["poses"]:
        p["thermal"] = thermal_of(a, p["id"], src)
    for sc in SCALES:
        add_laundry(a, sc)
        a["representations"][sc]["decisions"] = TX.DECISIONS[sc]
        a["volumes"][sc]["decisions"] = TX.volume_decisions(sc, a["volumes"][sc]["decisions"])
    a["clips"] = clips(a)
    a["dimensions"] = dict(TX.DIMENSIONS)
    a["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"]), 3)
    a["thermal"]["body"]["massKg"] = 78.59
    a["presentation"] = {"defaultPose": "pegging-shirt", "defaultClip": "hang-shirt", "defaultScale": 100}
    for pid, p in solved.items():
        print("[DEBUG] residuals", pid, [f"{x:.1e}" for x in p["residual"]])
    print(save(a, SLUG))


if __name__ == "__main__":
    main()
