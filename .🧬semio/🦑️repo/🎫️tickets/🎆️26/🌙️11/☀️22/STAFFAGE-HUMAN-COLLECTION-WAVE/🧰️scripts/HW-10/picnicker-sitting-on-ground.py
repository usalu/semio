"""🧺️ Generator of picnicker-sitting-on-ground: a man of 1.70 m sitting on a picnic blanket eating an apple, derived from seated-guitarist.

python3 picnicker-sitting-on-ground.py [stage] — `base` writes the scaled figure on the blanket with a rest probe, `draft` the full draft to 🗑️generated, `ship` the catalog file."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from hw10lib import *

SLUG = "picnicker-sitting-on-ground"
K = 1.70 / 1.76
MASS = 68.0
STAGE = sys.argv[1] if len(sys.argv) > 1 else "draft"


def base():
    d = load("seated-guitarist")
    a = d["asset"]
    a["id"] = SLUG
    drop(a, bone_ids=["guitar"], regions=["guitar", "stool"], materials=["spruce", "mahogany", "rosewood", "pine"],
         shape_pred=lambda s: s["bone"] == "stool", solid_pred=lambda l, s: s.get("bone") == "stool" or s.get("member") == "stool")
    a["anchors"] = [x for x in a["anchors"] if x["bone"] != "stool" and x["id"] not in ("rung-support",)]
    rename_bone(a, "stool", "blanket")
    b = bone(a, "blanket")
    b.update({"title": text("Picnic blanket on the ground", "Picknickdecke auf dem Boden"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 1.4})
    hl = bone(a, "hip-lift")
    hl.update({"x": 0, "y": 0.7, "angleDeg": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.95, "maxM": 0.3}})
    b.pop("mass", None)
    scale_asset(a, K, MASS)
    return d


FIT = f"{H}/{SLUG}.fit.json"
APPLE_R = 0.038
BLANKET_T = 0.006
LIFT = -0.571
FINGERS = ["index", "middle", "ring", "little"]


def apple_grip(side, curl):
    out = {}
    for i, f in enumerate(FINGERS):
        out[f"{f}-proximal-{side}"] = {"angleDeg": round(curl * (30 + 5 * i), 1)}
        out[f"{f}-middle-{side}"] = {"angleDeg": round(curl * (45 + 5 * i), 1)}
        out[f"{f}-distal-{side}"] = {"angleDeg": round(curl * (25 + 4 * i), 1)}
    out[f"thumb-proximal-{side}"] = {"angleDeg": round(curl * 20, 1)}
    out[f"thumb-distal-{side}"] = {"angleDeg": round(curl * 15, 1)}
    return out


def apple_bone(a):
    a["rig"]["bones"].append({"id": "apple", "parent": "hand-r", "title": text("Apple in the right hand", "Apfel in der rechten Hand"), "x": 0.08, "y": 0.0, "z": 0.045, "angleDeg": 90, "lengthM": APPLE_R,
                              "joint": {"type": "fixed"}, "mass": {"kg": 0.18, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
    a["anchors"] += [{"id": "apple-cog", "bone": "apple", "x": 0, "y": 0, "kind": "load", "title": text("Centre of the apple", "Mitte des Apfels")}]


def leg_vars(side):
    s = 1 if side == "l" else -1
    return [(f"thigh-{side}", "angleDeg", -60, 125), (f"thigh-{side}", "abductionDeg", -90 if s < 0 else -20, 20 if s < 0 else 90), (f"thigh-{side}", "twistDeg", -90, 90), (f"shank-{side}", "angleDeg", -150, 0),
            (f"foot-{side}", "angleDeg", -65, 60), (f"foot-{side}", "abductionDeg", -40, 40), (f"foot-{side}", "twistDeg", -60, 60)]


def build_poses(a):
    """Three sitting poses on the blanket; legs and arms are fitted with `fit.ts` (cached in the .fit.json design input)."""
    template = copy.deepcopy(a["poses"][0])
    template.pop("key", None)
    trunk = {"hip-lift": {"dy": LIFT}, "pelvis": {"angleDeg": 14}, "torso": {"angleDeg": -12}, "neck": {"angleDeg": -4}, "head": {"angleDeg": 2}}
    knee_l = lambda fr: fr["bones"]["thigh-l"]["tip"]
    cross = lambda fr: [{"kind": "point", "bone": "thigh-l", "local": [0.415, 0, 0], "world": [0.33, 0.17, 0.34], "weight": 1}, {"kind": "point", "bone": "thigh-r", "local": [0.415, 0, 0], "world": [0.33, 0.17, -0.34], "weight": 1},
                        {"kind": "point", "bone": "shank-l", "local": [0.42, 0, 0], "world": [0.30, 0.075, -0.10], "weight": 1}, {"kind": "point", "bone": "shank-r", "local": [0.42, 0, 0], "world": [0.24, 0.075, 0.12], "weight": 1},
                        {"kind": "axis", "bone": "foot-l", "axis": "u", "world": unit([0.15, -0.25, -1]), "weight": 0.02}, {"kind": "axis", "bone": "foot-r", "axis": "u", "world": unit([0.15, -0.25, 1]), "weight": 0.02},
                        {"kind": "above", "bone": "toe-l", "local": [0.09, 0, 0], "minY": 0.04, "weight": 1}, {"kind": "above", "bone": "toe-r", "local": [0.09, 0, 0], "minY": 0.04, "weight": 1}]
    legs = leg_vars("l") + leg_vars("r")
    apple_mouth = lambda fr: [{"kind": "point", "bone": "apple", "local": [0, 0, 0], "world": [fr["anchors"]["mouth"][0] + 0.045, fr["anchors"]["mouth"][1] - 0.012, -0.01], "weight": 1},
                              {"kind": "point", "bone": "hand-r", "local": [0, 0, 0], "world": [fr["anchors"]["mouth"][0] + 0.07, fr["anchors"]["mouth"][1] - 0.16, -0.06], "weight": 0.3}]
    hand_on_knee = lambda fr: [{"kind": "point", "bone": "hand-l", "local": [0.05, 0, 0], "world": [fr["bones"]["thigh-l"]["tip"][0] - 0.02, fr["bones"]["thigh-l"]["tip"][1] + 0.07, fr["bones"]["thigh-l"]["tip"][2] - 0.02], "weight": 1},
                               {"kind": "axis", "bone": "hand-l", "axis": "u", "world": unit([0.6, -0.5, 0.2]), "weight": 0.01}]
    specs = [
        ("cross-legged-eating", {**trunk, **apple_grip("r", 1), **apple_grip("l", 0.3),
                                 "upper-arm-r": {"angleDeg": 40, "abductionDeg": -20, "twistDeg": -30}, "forearm-r": {"angleDeg": 130, "twistDeg": 20}, "hand-r": {"angleDeg": 10},
                                 "upper-arm-l": {"angleDeg": 30, "abductionDeg": 20, "twistDeg": 20}, "forearm-l": {"angleDeg": 40}, "hand-l": {"angleDeg": 10},
                                 "thigh-l": {"angleDeg": 80, "abductionDeg": 45, "twistDeg": 60}, "shank-l": {"angleDeg": -135}, "foot-l": {"angleDeg": -10},
                                 "thigh-r": {"angleDeg": 80, "abductionDeg": -45, "twistDeg": -60}, "shank-r": {"angleDeg": -135}, "foot-r": {"angleDeg": -10}},
         [(legs, cross, 3), (ARM_R_G, apple_mouth, 2), (ARM_L_G, hand_on_knee, 2)]),
        ("cross-legged-resting", {**trunk, "torso": {"angleDeg": -16}, "neck": {"angleDeg": -6}, "head": {"angleDeg": -6}, **apple_grip("r", 1), **apple_grip("l", 0.3),
                                  "upper-arm-r": {"angleDeg": 20, "abductionDeg": -10, "twistDeg": -20}, "forearm-r": {"angleDeg": 60}, "hand-r": {"angleDeg": 10},
                                  "upper-arm-l": {"angleDeg": 30, "abductionDeg": 20, "twistDeg": 20}, "forearm-l": {"angleDeg": 40}, "hand-l": {"angleDeg": 10}},
         [(legs, cross, 0), (ARM_R_G, lambda fr: [{"kind": "point", "bone": "apple", "local": [0, 0, 0], "world": [fr["bones"]["thigh-r"]["tip"][0] - 0.03, fr["bones"]["thigh-r"]["tip"][1] + 0.08, fr["bones"]["thigh-r"]["tip"][2] + 0.04], "weight": 1}], 2), (ARM_L_G, hand_on_knee, 2)]),
        ("leaning-back", {"hip-lift": {"dy": LIFT}, "pelvis": {"angleDeg": 18}, "torso": {"angleDeg": 2}, "neck": {"angleDeg": -6}, "head": {"angleDeg": -4}, **apple_grip("r", 1), **apple_grip("l", 0.0),
                          "thigh-l": {"angleDeg": 120, "abductionDeg": 8, "twistDeg": 5}, "shank-l": {"angleDeg": -100}, "foot-l": {"angleDeg": 0},
                          "thigh-r": {"angleDeg": 115, "abductionDeg": -8, "twistDeg": -5}, "shank-r": {"angleDeg": -95}, "foot-r": {"angleDeg": 0},
                          "upper-arm-l": {"angleDeg": -35, "abductionDeg": 15, "twistDeg": 0}, "forearm-l": {"angleDeg": 5}, "hand-l": {"angleDeg": -50},
                          "upper-arm-r": {"angleDeg": 10, "abductionDeg": -10, "twistDeg": -20}, "forearm-r": {"angleDeg": 70}, "hand-r": {"angleDeg": 10}},
         [(legs, lambda fr: [{"kind": "point", "bone": "foot-l", "local": [-0.029, -0.077, 0], "world": [0.47, 0.012, 0.12], "weight": 1}, {"kind": "point", "bone": "foot-r", "local": [-0.029, -0.077, 0], "world": [0.44, 0.012, -0.13], "weight": 1},
                             {"kind": "point", "bone": "foot-l", "local": [0.13, -0.077, 0], "world": [0.60, 0.012, 0.135], "weight": 1}, {"kind": "point", "bone": "foot-r", "local": [0.13, -0.077, 0], "world": [0.57, 0.012, -0.15], "weight": 1},
                             {"kind": "above", "bone": "toe-l", "local": [0.09, -0.02, 0], "minY": 0.012, "weight": 50}, {"kind": "above", "bone": "toe-r", "local": [0.09, -0.02, 0], "minY": 0.012, "weight": 50}], 3),
          (ARM_L_G, lambda fr: [{"kind": "point", "bone": "hand-l", "local": [0.06, 0, 0], "world": [-0.24, 0.04, 0.26], "weight": 1}, {"kind": "above", "bone": "middle-distal-l", "local": [0.022, 0, 0], "minY": 0.012, "weight": 50}, {"kind": "above", "bone": "ring-distal-l", "local": [0.02, 0, 0], "minY": 0.012, "weight": 50}, {"kind": "above", "bone": "little-distal-l", "local": [0.018, 0, 0], "minY": 0.012, "weight": 50}, {"kind": "above", "bone": "index-distal-l", "local": [0.02, 0, 0], "minY": 0.012, "weight": 50}, {"kind": "axis", "bone": "hand-l", "axis": "u", "world": unit([-0.5, -0.2, 0.3]), "weight": 0.01}], 2),
          (ARM_R_G, lambda fr: [{"kind": "point", "bone": "apple", "local": [0, 0, 0], "world": [fr["bones"]["thigh-r"]["tip"][0] - 0.1, fr["bones"]["thigh-r"]["tip"][1] + 0.08, fr["bones"]["thigh-r"]["tip"][2] + 0.02], "weight": 1}], 2)]),
    ]
    cache = json.load(open(FIT)) if os.path.exists(FIT) else {}
    a["poses"] = []
    for pid, bones, steps in specs:
        p = copy.deepcopy(template)
        p["id"] = pid
        p["bones"] = bones
        p["contacts"] = []
        a["poses"].append(p)
    for pid, bones, steps in specs:
        if pid not in cache:
            if pid == "cross-legged-resting":
                for k, v in cache["cross-legged-eating"].items():
                    if k.split("-")[0] in ("thigh", "shank", "foot", "toe"):
                        pose(a, pid)["bones"][k] = copy.deepcopy(v)
            for arms, targets, restarts in steps:
                if restarts == 0:
                    continue
                fr = probe_asset(a, pid, SLUG)
                fit_pose(a, pid, arms, targets(fr), restarts=restarts, root_y=fr["root"]["y"])
            cache[pid] = pose(a, pid)["bones"]
            json.dump(cache, open(FIT, "w"), indent=1)
        pose(a, pid)["bones"] = cache[pid]


BX0, BX1, BZ = -0.5, 0.9, 0.6
APPLE_50 = [(-0.034, 0), (-0.0352, 0.012), (-0.0305, 0.0275), (-0.016, 0.0368), (0.0, 0.038), (0.016, 0.0372), (0.028, 0.0305), (0.0335, 0.019), (0.0315, 0.007), (0.029, 0)]
APPLE_200 = [(-0.035, 0), (-0.031, 0.026), (0.0, 0.038), (0.03, 0.028), (0.032, 0)]
APPLE_500 = [(-0.0375, 0), (-0.0265, 0.0265), (0.0, 0.0375), (0.0265, 0.0265), (0.0375, 0)]


def blanket_and_apple(a):
    """The blanket lies still on the ground (its bone never moves), so its front and plan shapes are drawn in world metres; the apple turns with the hand and is a solid of revolution."""
    b = bone(a, "blanket")
    b["mass"] = {"kg": 0.9, "comX": rnd((BX0 + BX1) / 2), "comY": BLANKET_T / 2, "distribution": "point", "source": "authored-estimate"}
    a["anchors"] += [{"id": "blanket-ground", "bone": "blanket", "x": rnd((BX0 + BX1) / 2), "y": 0, "kind": "contact", "title": text("Blanket on the grass", "Decke auf dem Gras"), "support": "contact", "normalDeg": 90, "footprintLengthM": BX1 - BX0, "footprintWidthM": 2 * BZ, "frictionCoefficient": 0.6},
                     {"id": "ankle-l", "bone": "foot-l", "x": 0.0, "y": -0.035, "z": 0.03, "kind": "contact", "title": text("Outer edge of the left foot", "Außenkante des linken Fußes"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.12, "footprintWidthM": 0.05, "frictionCoefficient": 0.6},
                     {"id": "ankle-r", "bone": "foot-r", "x": 0.0, "y": -0.035, "z": -0.03, "kind": "contact", "title": text("Outer edge of the right foot", "Außenkante des rechten Fußes"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.12, "footprintWidthM": 0.05, "frictionCoefficient": 0.6},
                     {"id": "palm-l", "bone": "hand-l", "x": 0.06, "y": 0, "kind": "contact", "title": text("Left palm on the blanket", "Linke Handfläche auf der Decke"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.08, "frictionCoefficient": 0.6}]
    elev = [[BX0, 0], [BX1, 0], [BX1, BLANKET_T], [BX0, BLANKET_T]]
    front = [[-BZ, 0], [BZ, 0], [BZ, BLANKET_T], [-BZ, BLANKET_T]]
    plan = [[BX0, -BZ], [BX1, -BZ], [BX1, BZ], [BX0, BZ]]
    for sc in SCALES:
        for view, pts in (("elevation", elev), ("front", front), ("plan", plan)):
            sh = {"id": "blanket", "bone": "blanket", "role": "body", "z": 0.05, "filled": True, "tone": 0.7, "region": "blanket", "material": "wool-blanket", "kind": "poly", "points": [[rnd(x), rnd(y)] for x, y in pts], "closed": True}
            if sc == "500":
                sh["penMm"] = 0.2
            if view == "plan" and sc in ("50", "100"):
                sh["hatch"] = "gingham"
            shapes(a, sc, view).insert(0, sh)
        rep = a["representations"][sc]
        if sc in ("50", "100"):
            rep["hatches"] = [h for h in rep.get("hatches", []) if h["id"] != "gingham"] + [{"id": "gingham", "title": text("Gingham check of the blanket", "Vichy-Karo der Decke"), "pattern": "cross", "angleDeg": 0, "spacingMm": 1.2 if sc == "50" else 0.8, "penMm": 0.1}]
        prof = {"50": APPLE_50, "100": APPLE_50, "200": APPLE_200, "500": APPLE_500}[sc]
        for view, z in (("elevation", 7.2), ("front", 7.2), ("plan", 7.2)):
            shapes(a, sc, view).append(revolve_profile_shape("apple", "apple", prof, 0.45, "apple", z, material="apple", **({"penMm": 0.13} if sc == "500" else {})))
        solids(a, sc, "body").extend([{"id": "blanket", "kind": "box", "bone": "blanket", "material": "wool-blanket", "role": "body", "tone": 0.7, "size": [BX1 - BX0, BLANKET_T, 2 * BZ], "transform": {"translation": [rnd((BX0 + BX1) / 2), BLANKET_T / 2, 0]}, "sheet": True},
                                      {"id": "apple", "kind": "revolve", "bone": "apple", "material": "apple", "role": "body", "tone": 0.45, "profile": [[rnd(u), rnd(r)] for u, r in prof], "axis": [1, 0, 0], "origin": [0, 0, 0]}])
        solids(a, sc, "thermal").extend([{"id": "th-blanket", "of": "blanket", "region": "blanket"}, {"id": "th-apple", "of": "apple", "region": "apple"}])
        solids(a, sc, "structure").extend([{"id": "st-blanket", "of": "blanket", "member": "blanket", "part": "member"}, {"id": "st-apple", "of": "apple", "member": "apple", "part": "member"}])


def plan_head(a):
    """The base draws head, hair, beard, ears and neck in the plan as thin slabs pre-stretched for the forward tilt of its own playing pose, which vanish on an upright head; here each becomes its true horizontal outline (read from the base's evaluated playing pose, untwisted and scaled) swept along its bone, so the plan stays true at any tilt."""
    code, out = run([BUN, f"{H}/planrings.ts", f"{CATALOG}/seated-guitarist/🔣️.json", "strumming", "head,neck"])
    if code:
        raise RuntimeError(out)
    base = json.loads(out.strip().splitlines()[-1])
    for sc in SCALES:
        elev = {s["id"]: s for s in shapes(a, sc, "elevation")}
        for sh in shapes(a, sc, "plan"):
            ring = base[sc].get(sh["id"])
            if ring is None or sh["bone"] != ring["bone"] or not sh.get("filled"):
                continue
            f = base["frames"][sh["bone"]]
            vh, wh = unit([f["v"][0], 0, f["v"][2]]), unit([f["w"][0], 0, f["w"][2]])
            pts = [[rnd(K * ((x - f["o"][0]) * vh[0] + (z - f["o"][2]) * vh[2])), rnd(K * ((x - f["o"][0]) * wh[0] + (z - f["o"][2]) * wh[2]))] for x, z in ring["ring"]]
            us = [q[0] for q in elev[sh["id"]]["points"]] if sh["id"] in elev else [0, bone(a, sh["bone"])["lengthM"]]
            sh["points"] = pts
            sh["solid"] = {"kind": "extruded", "sweep": "u", "z0": rnd(min(us)), "z1": rnd(max(us))}


ARM_R_G = [("upper-arm-r", "angleDeg", -60, 180), ("upper-arm-r", "abductionDeg", -90, 30), ("upper-arm-r", "twistDeg", -120, 90), ("forearm-r", "angleDeg", 0, 150), ("forearm-r", "twistDeg", -90, 90), ("hand-r", "angleDeg", -65, 75), ("hand-r", "abductionDeg", -40, 40)]
ARM_L_G = [(b.replace("-r", "-l"), f, (-hi if f != "angleDeg" else lo), (-lo if f != "angleDeg" else hi)) for b, f, lo, hi in ARM_R_G]


if __name__ == "__main__":
    d = base()
    a = d["asset"]
    if STAGE == "base":
        p = rest_pose(a)
        p["contacts"] = []
        write(d, f"{O}/[DEBUG]-{SLUG}-base.json")
        sys.exit()
    apple_bone(a)
    build_poses(a)
    blanket_and_apple(a)
    plan_head(a)
    import importlib.util
    spec = importlib.util.spec_from_file_location("content", f"{H}/{SLUG}-content.py")
    content = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(content)
    content.apply(a)
    path = f"{O}/[DEBUG]-{SLUG}-draft.json"
    write(d, path)
    print("floor solids restored", restore_floor(a, path, 1 / K + 0.002))
    write(d, path)
    fit_dimensions(a, path)
    write(d, path)
    if STAGE == "ship":
        write(d, f"{CATALOG}/{SLUG}/🔣️.json")
