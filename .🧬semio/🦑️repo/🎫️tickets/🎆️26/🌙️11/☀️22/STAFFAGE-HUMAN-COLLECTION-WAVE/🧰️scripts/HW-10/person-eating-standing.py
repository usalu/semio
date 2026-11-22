"""🥖️ Generator of person-eating-standing: a man of 1.78 m eating a baguette sandwich standing, derived from standing-phone-user.

python3 person-eating-standing.py [stage] — stage `base` writes the scaled figure without the prop, `draft` the full draft to 🗑️generated, `ship` the catalog file."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from hw10lib import *

SLUG = "person-eating-standing"
K = 1.78 / 1.69
MASS = 75.0
STAGE = sys.argv[1] if len(sys.argv) > 1 else "draft"

LEVA_MALE = {"pelvis": 0.1117, "torso": 0.3229, "head": 0.0694 * 0.809, "neck": 0.0694 * 0.191, "arm": 0.0271, "forearm": 0.0162, "hand": 0.0061, "thigh": 0.1416, "shank": 0.0433, "foot": 0.0137 * 0.92, "toe": 0.0137 * 0.08}
LEVA_MALE_COM = {"arm": 0.5772, "forearm": 0.4574, "thigh": 0.4095, "shank": 0.4459}


def base():
    d = load("standing-phone-user")
    a = d["asset"]
    a["id"] = SLUG
    drop(a, regions=["headphones", "phone"], materials=["headphone-polymer", "phone-glass-metal"], shape_pred=lambda s: "headphone" in s["id"] or s["bone"] == "phone", solid_pred=lambda l, s: "headphone" in s["id"] or s.get("bone") == "phone" or s.get("member") == "phone")
    a["anchors"] = [x for x in a["anchors"] if x["id"] not in ("ear", "phone-cog")]
    a["thermal"]["operating"] = []
    for p in a["poses"]:
        p["thermal"]["operating"] = {}
        p["loads"] = []
    for c in a["clips"]:
        c["tracks"] = [t for t in c["tracks"] if not t["target"].startswith("operating:")]
        c["events"] = [e for e in c["events"] if e.get("target") != "phone"]
    rename_bone(a, "phone", "sandwich")
    rename_region(a, "jeans", "chinos")
    rename_material(a, "denim", "cotton-twill")
    rename_material(a, "fleece-sweatshirt", "cotton-sweatshirt")
    rename_material(a, "canvas-sneaker", "leather-sneaker")
    scale_asset(a, K, MASS)
    for b in a["rig"]["bones"]:
        if "mass" not in b or b["id"] == "sandwich":
            continue
        kind = b["id"].rsplit("-", 1)[0] if b["id"].endswith(("-l", "-r")) else b["id"]
        b["mass"]["kg"] = rnd((MASS) * LEVA_MALE[kind] if kind in LEVA_MALE else b["mass"]["kg"], 4)
        if kind in LEVA_MALE_COM:
            b["mass"]["comX"] = rnd(LEVA_MALE_COM[kind] * b["lengthM"])
        if kind in LEVA_MALE:
            b["mass"]["source"], b["mass"]["reference"] = "literature", "de-leva-1996"
    return d


L = 0.21
BACK = 0.075
TIP = L - BACK
R0 = 0.033


def sandwich_bone(a):
    b = bone(a, "sandwich")
    b.update({"title": text("Baguette sandwich in the right hand", "Baguette-Sandwich in der rechten Hand"), "x": 0.09, "y": 0.0, "z": 0.05, "angleDeg": 90, "lengthM": TIP,
              "joint": {"type": "fixed"}, "mass": {"kg": 0.25, "comX": TIP - L / 2, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
    b.pop("abductionDeg", None), b.pop("twistDeg", None)
    a["anchors"].append({"id": "sandwich-cog", "bone": "sandwich", "x": TIP - L / 2, "y": 0, "kind": "load", "title": text("Sandwich centre of gravity", "Schwerpunkt des Sandwiches")})
    a["anchors"].append({"id": "bite-end", "bone": "sandwich", "x": TIP, "y": 0, "kind": "attachment", "title": text("Bitten end of the sandwich", "Angebissenes Ende des Sandwiches")})


LEGS = {"thigh-l": {"angleDeg": -1}, "shank-l": {"angleDeg": -5}, "foot-l": {"angleDeg": 6}, "toe-l": {"angleDeg": 0}, "thigh-r": {"angleDeg": -1}, "shank-r": {"angleDeg": -5}, "foot-r": {"angleDeg": 6}, "toe-r": {"angleDeg": 0}}
ARM_R = [("arm-r", "angleDeg", -60, 170), ("arm-r", "abductionDeg", -80, 30), ("arm-r", "twistDeg", -90, 90), ("forearm-r", "angleDeg", 0, 150), ("forearm-r", "twistDeg", -90, 90), ("hand-r", "angleDeg", -60, 60), ("hand-r", "abductionDeg", -35, 35), ("hand-r", "twistDeg", -35, 35)]
ARM_L = [(b.replace("-r", "-l"), f, (-hi if f != "angleDeg" else lo), (-lo if f != "angleDeg" else hi)) for b, f, lo, hi in ARM_R]


def fitted(a, pid, init, targets, arms, fixed, rest=(), restarts=0):
    pose_ = pose(a, pid)
    pose_["bones"] = {**LEGS, **copy.deepcopy(fixed), **copy.deepcopy(init)}
    tmp = copy.deepcopy(a)
    vars_ = [{"scope": "pose", "bone": b, "field": f, "init": init.get(b, {}).get(f, 0), "lo": lo, "hi": hi} for b, f, lo, hi in arms]
    vars_ += [{"scope": "rest", "bone": b, "field": f, "init": bone(a, b).get(f, 0), "lo": lo, "hi": hi} for b, f, lo, hi in rest]
    spec = {"asset": tmp, "pose": pid, "vars": vars_, "targets": targets, "regular": 2e-6, "restarts": restarts}
    res = fit(spec)
    for v in res["values"]:
        if v["scope"] == "rest":
            bone(a, v["bone"])[v["field"]] = v["value"]
        else:
            pose_["bones"].setdefault(v["bone"], {})[v["field"]] = v["value"]
    print(pid, "cost", res["cost"], [(t["kind"], t["bone"], t["world"], t["got"]) for t in res["report"]])
    return res


FIT = f"{H}/{SLUG}.fit.json"

BREAD_50 = [(-BACK, 0), (-0.0735, 0.011), (-0.0695, 0.0205), (-0.0625, 0.0275), (-0.052, 0.032), (-0.035, 0.0335), (0.03, 0.034), (0.1, 0.0335), (0.118, 0.0318), (0.127, 0.0285), (0.1325, 0.0225), (0.1345, 0.014), (TIP, 0)]
BREAD_100 = [(-BACK, 0), (-0.072, 0.0155), (-0.0635, 0.027), (-0.048, 0.0328), (0.03, 0.034), (0.112, 0.0325), (0.127, 0.0285), (0.134, 0.017), (TIP, 0)]
BREAD_200 = [(-BACK, 0), (-0.068, 0.023), (-0.05, 0.0328), (0.11, 0.0328), (0.13, 0.024), (TIP, 0)]
BREAD_500 = [(-BACK, 0), (-0.07, 0.026), (-0.05, 0.0375), (0.11, 0.0375), (0.13, 0.027), (TIP, 0)]
FILL_W = 0.037


def filling_profile():
    top = [(-0.063 + i * 0.0095, 0.0085 if i % 2 else 0.0055) for i in range(21)]
    return [(-0.064, -0.0055)] + [(u, v) for u, v in top] + [(0.127, 0.0065), (0.128, -0.0055)]


def sandwich_parts(a, zrest):
    """Shapes and solids of the sandwich per scale; the roll is revolved about the sandwich bone, the filling a slab across it."""
    fz = {"kind": "extruded", "sweep": "w", "z0": rnd(zrest - FILL_W), "z1": rnd(zrest + FILL_W)}
    sz = {"kind": "extruded", "sweep": "w", "z0": rnd(zrest - 0.0345), "z1": rnd(zrest + 0.0345)}
    fill = [[rnd(u), rnd(v)] for u, v in filling_profile()]
    for view, zb, zf in (("elevation", 5.92, 5.93), ("front", 4.62, 4.63), ("plan", 2.42, 2.43)):
        shapes(a, "50", view).extend([revolve_profile_shape("bread", "sandwich", BREAD_50, 0.82, "sandwich", zb, material="bread-roll"),
                                      {"id": "filling", "bone": "sandwich", "role": "body", "z": zf, "filled": True, "tone": 0.35, "region": "sandwich", "material": "sandwich-filling", "kind": "poly", "points": fill, "closed": True, "solid": dict(fz)}])
        shapes(a, "100", view).append(revolve_profile_shape("bread", "sandwich", BREAD_100, 0.82, "sandwich", zb, material="bread-roll"))
        shapes(a, "200", view).append(revolve_profile_shape("bread", "sandwich", BREAD_200, 0.82, "sandwich", zb, material="bread-roll"))
        shapes(a, "500", view).append(revolve_profile_shape("bread", "sandwich", BREAD_500, 0.82, "sandwich", zb, material="bread-roll", penMm=0.13))
    shapes(a, "100", "elevation").append({"id": "filling-seam", "bone": "sandwich", "role": "detail", "z": 5.93, "filled": False, "tone": 0.35, "region": "sandwich", "material": "sandwich-filling", "kind": "poly",
                                          "points": [[-0.062, -0.001], [0.126, -0.001], [0.126, 0.001], [-0.062, 0.001]], "closed": True, "solid": dict(sz)})
    rev = lambda prof: [[rnd(u, 5), rnd(r, 5)] for u, r in prof]
    for sc, prof in (("50", BREAD_50), ("100", BREAD_100), ("200", BREAD_200), ("500", BREAD_500)):
        body = solids(a, sc, "body")
        body.append({"id": "bread", "kind": "revolve", "bone": "sandwich", "material": "bread-roll", "role": "body", "tone": 0.82, "profile": rev(prof), "axis": [1, 0, 0], "origin": [0, 0, 0]})
        solids(a, sc, "thermal").append({"id": "th-bread", "of": "bread", "region": "sandwich"})
        solids(a, sc, "structure").append({"id": "st-sandwich", "of": "bread", "member": "sandwich", "part": "member"})
    solids(a, "50", "body").append({"id": "filling", "kind": "extrude", "bone": "sandwich", "material": "sandwich-filling", "role": "body", "tone": 0.35, "profile": fill, "plane": "xy", "from": -FILL_W, "to": FILL_W})
    solids(a, "50", "thermal").append({"id": "th-filling", "of": "filling", "region": "sandwich"})



def unit(v):
    n = math.sqrt(sum(x * x for x in v))
    return [x / n for x in v]


def frame_point(f, local):
    return [f["o"][k] + local[0] * f["u"][k] + local[1] * f["v"][k] + local[2] * f["w"][k] for k in range(3)]


BOTTLE_DOWN = 0.09
BOTTLE_UP = 0.115
BOTTLE_KG = 0.52


def bottle_bone(a):
    a["rig"]["bones"].append({"id": "bottle", "parent": "hand-l", "title": text("Water bottle in the left hand", "Wasserflasche in der linken Hand"), "x": 0.09, "y": 0.0, "z": -0.05, "angleDeg": 90, "lengthM": BOTTLE_UP,
                              "joint": {"type": "fixed"}, "mass": {"kg": BOTTLE_KG, "comX": -0.01, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
    a["anchors"] += [{"id": "bottle-cog", "bone": "bottle", "x": -0.01, "y": 0, "kind": "load", "title": text("Centre of gravity of the bottle", "Schwerpunkt der Flasche")},
                     {"id": "bottle-mouth", "bone": "bottle", "x": BOTTLE_UP, "y": 0, "kind": "attachment", "title": text("Mouth of the bottle", "Flaschenöffnung")}]


BOTTLE_50 = [(-0.09, 0), (-0.09, 0.027), (-0.086, 0.0318), (-0.08, 0.0325), (0.035, 0.0325), (0.06, 0.0285), (0.078, 0.019), (0.09, 0.0135), (0.095, 0.0135), (0.095, 0.0158), (0.115, 0.0158), (0.115, 0)]
BOTTLE_100 = [(-0.09, 0), (-0.09, 0.0285), (-0.083, 0.0325), (0.04, 0.0325), (0.075, 0.0205), (0.093, 0.0145), (0.095, 0.0158), (0.115, 0.0158), (0.115, 0)]
BOTTLE_200 = [(-0.09, 0), (-0.09, 0.0325), (0.045, 0.0325), (0.09, 0.016), (0.115, 0.016), (0.115, 0)]
BOTTLE_500 = [(-0.09, 0), (-0.09, 0.0375), (0.055, 0.0375), (0.115, 0.0255), (0.115, 0)]
CAP = [(0.095, 0), (0.095, 0.0162), (0.115, 0.0162), (0.115, 0)]
LABEL = [(-0.035, 0), (-0.035, 0.0331), (0.02, 0.0331), (0.02, 0)]


def bottle_parts(a):
    """The 0.5 L water bottle in the left hand: a body of revolution about the bottle bone, cap and label as bands at the fine scales."""
    for view, z in (("elevation", 5.5), ("front", 4.4), ("plan", 2.3)):
        shapes(a, "50", view).extend([revolve_profile_shape("bottle", "bottle", BOTTLE_50, 0.9, "water-bottle", z, material="pet-bottle"),
                                      revolve_profile_shape("bottle-label", "bottle", LABEL, 0.5, "water-bottle", z + 0.01, material="pet-bottle"),
                                      revolve_profile_shape("bottle-cap", "bottle", CAP, 0.3, "water-bottle", z + 0.02, material="pet-bottle")])
        shapes(a, "100", view).extend([revolve_profile_shape("bottle", "bottle", BOTTLE_100, 0.9, "water-bottle", z, material="pet-bottle"),
                                       revolve_profile_shape("bottle-cap", "bottle", CAP, 0.3, "water-bottle", z + 0.02, material="pet-bottle")])
        shapes(a, "200", view).append(revolve_profile_shape("bottle", "bottle", BOTTLE_200, 0.9, "water-bottle", z, material="pet-bottle"))
        shapes(a, "500", view).append(revolve_profile_shape("bottle", "bottle", BOTTLE_500, 0.9, "water-bottle", z, material="pet-bottle", penMm=0.13))
    for sc, prof in (("50", BOTTLE_50), ("100", BOTTLE_100), ("200", BOTTLE_200), ("500", BOTTLE_500)):
        solids(a, sc, "body").append({"id": "bottle", "kind": "revolve", "bone": "bottle", "material": "pet-bottle", "role": "body", "tone": 0.9, "profile": [[rnd(u), rnd(r)] for u, r in prof], "axis": [1, 0, 0], "origin": [0, 0, 0]})
        solids(a, sc, "thermal").append({"id": "th-bottle", "of": "bottle", "region": "water-bottle"})
        solids(a, sc, "structure").append({"id": "st-bottle", "of": "bottle", "member": "bottle", "part": "member"})


def build_poses(a):
    """Three named poses fitted with `fit.ts` (cached in the .fit.json design input; delete a pose entry to refit it)."""
    template = copy.deepcopy(a["poses"][0])
    chew_r = {"arm-r": {"angleDeg": 10, "abductionDeg": -5, "twistDeg": -20}, "forearm-r": {"angleDeg": 95, "twistDeg": 20}, "hand-r": {"angleDeg": 0}}
    hang_l = {"arm-l": {"angleDeg": 6, "abductionDeg": 6, "twistDeg": 20}, "forearm-l": {"angleDeg": 25, "twistDeg": -20}, "hand-l": {"angleDeg": 0}}
    hip_bottle = lambda fr: [{"kind": "point", "bone": "bottle", "local": [0, 0, 0], "world": [0.07, 0.86, 0.2], "weight": 1}, {"kind": "axis", "bone": "bottle", "axis": "u", "world": [0, 1, 0], "weight": 0.06}]
    sandwich_low = lambda fr: [{"kind": "point", "bone": "sandwich", "local": [TIP, 0, 0], "world": [0.40, 1.15, -0.02], "weight": 1},
                               {"kind": "axis", "bone": "sandwich", "axis": "u", "world": unit([0.6, 0.1, 0.8]), "weight": 0.05}, {"kind": "axis", "bone": "sandwich", "axis": "v", "world": [0, 1, 0], "weight": 0.01}]
    rest_s = [("sandwich", "angleDeg", -180, 180), ("sandwich", "abductionDeg", -89, 89), ("sandwich", "twistDeg", -180, 180)]
    rest_b = [("bottle", "angleDeg", -180, 180), ("bottle", "abductionDeg", -89, 89), ("bottle", "twistDeg", -180, 180)]
    specs = [
        ("taking-a-bite", {"torso": {"angleDeg": -2}, "neck": {"angleDeg": -7}, "head": {"angleDeg": -3}, "ponytail": {"angleDeg": 6}, **hang_l,
                           "arm-r": {"angleDeg": 65, "abductionDeg": -8, "twistDeg": -15}, "forearm-r": {"angleDeg": 120, "twistDeg": 10}, "hand-r": {"angleDeg": 10}},
         [(ARM_R, rest_s, lambda fr: [{"kind": "point", "bone": "sandwich", "local": [TIP, 0, 0], "world": [fr["anchors"]["mouth"][0] + 0.008, fr["anchors"]["mouth"][1], 0.0], "weight": 1},
                                      {"kind": "axis", "bone": "sandwich", "axis": "u", "world": unit([-0.9, 0.12, 0.4]), "weight": 0.1}, {"kind": "axis", "bone": "sandwich", "axis": "v", "world": [0, 1, 0], "weight": 0.01},
                                      {"kind": "perp", "bone": "sandwich", "axis": "u", "other": "hand-r", "otherAxis": "u", "weight": 0.02}], 6),
          (ARM_L, (), hip_bottle, 2)]),
        ("drinking-from-bottle", {"torso": {"angleDeg": 1}, "neck": {"angleDeg": 4}, "head": {"angleDeg": 10}, "ponytail": {"angleDeg": -8}, **chew_r,
                                  "arm-l": {"angleDeg": 60, "abductionDeg": 20, "twistDeg": 30}, "forearm-l": {"angleDeg": 120, "twistDeg": -10}, "hand-l": {"angleDeg": 10}},
         [(ARM_L, rest_b, lambda fr: [{"kind": "point", "bone": "bottle", "local": [BOTTLE_UP, 0, 0], "world": [fr["anchors"]["mouth"][0] + 0.006, fr["anchors"]["mouth"][1] - 0.004, 0.0], "weight": 1},
                                      {"kind": "axis", "bone": "bottle", "axis": "u", "world": unit([-0.82, -0.45, -0.25]), "weight": 0.06},
                                      {"kind": "perp", "bone": "bottle", "axis": "u", "other": "hand-l", "otherAxis": "u", "weight": 0.03}], 6),
          (ARM_R, (), sandwich_low, 2)]),
        ("chewing", {"torso": {"angleDeg": 0}, "neck": {"angleDeg": -1}, "head": {"angleDeg": 3}, "ponytail": {"angleDeg": -1}, **chew_r, **hang_l},
         [(ARM_R, (), sandwich_low, 2), (ARM_L, (), hip_bottle, 2)]),
    ]
    cache = json.load(open(FIT)) if os.path.exists(FIT) else {}
    a["poses"] = []
    for pid, bones, steps in specs:
        p = copy.deepcopy(template)
        p["id"] = pid
        p["bones"] = {**LEGS, **bones}
        a["poses"].append(p)
    for rid in ("sandwich", "bottle"):
        if rid in cache.get("rest", {}):
            bone(a, rid).update(cache["rest"][rid])
    for pid, bones, steps in specs:
        if pid not in cache:
            for arms, rest, targets, restarts in steps:
                fr = probe_asset(a, pid, SLUG)
                fit_pose(a, pid, arms, targets(fr), rest=rest, restarts=restarts, root_y=fr["root"]["y"])
                for b, f, lo, hi in rest:
                    cache.setdefault("rest", {}).setdefault(b, {})[f] = bone(a, b)[f]
            cache[pid] = pose(a, pid)["bones"]
            json.dump(cache, open(FIT, "w"), indent=1)
        pose(a, pid)["bones"] = cache[pid]


if __name__ == "__main__":
    d = base()
    a = d["asset"]
    print("mass sum", sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b))
    if STAGE == "base":
        rest_pose(a)
        write(d, f"{O}/[DEBUG]-{SLUG}-base.json")
        sys.exit()
    sandwich_bone(a)
    bottle_bone(a)
    build_poses(a)
    rest_pose(a, "rest-probe")
    zrest = probe_asset(a, "rest-probe", SLUG)["bones"]["sandwich"]["o"][2]
    a["poses"] = [p for p in a["poses"] if p["id"] != "rest-probe"]
    sandwich_parts(a, zrest)
    bottle_parts(a)
    import importlib.util
    spec = importlib.util.spec_from_file_location("content", f"{H}/{SLUG}-content.py")
    content = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(content)
    content.identity(a)
    content.materials(a)
    content.thermal(a)
    content.bind_poses(a)
    content.clips(a, pose(a, "chewing")["bones"]["head"]["angleDeg"])
    content.decisions(a)
    path = f"{O}/[DEBUG]-{SLUG}-draft.json"
    write(d, path)
    fit_dimensions(a, path)
    write(d, path)
    if STAGE == "ship":
        write(d, f"{CATALOG}/{SLUG}/🔣️.json")
