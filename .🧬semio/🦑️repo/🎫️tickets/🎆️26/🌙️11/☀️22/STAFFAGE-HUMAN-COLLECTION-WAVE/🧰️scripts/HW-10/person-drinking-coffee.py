"""☕️ Generator of person-drinking-coffee: a woman of 1.72 m with a steaming paper cup of coffee, derived from standing-adult.

python3 person-drinking-coffee.py [stage] — `base` writes the scaled figure with a probe pose, `draft` the full draft to 🗑️generated, `ship` the catalog file."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from hw10lib import *

SLUG = "person-drinking-coffee"
K = 1.72 / 1.69
MASS = 64.0
CUP_KG = 0.37
STAGE = sys.argv[1] if len(sys.argv) > 1 else "draft"
FINGERS = ["index", "middle", "ring", "little"]


def grip(side, curl):
    """Finger and thumb offsets of a cylinder grip; curl 0..1 from open to round a 90 mm cup."""
    s = 1 if side == "r" else -1
    out = {}
    for i, f in enumerate(FINGERS):
        out[f"{f}-proximal-{side}"] = {"angleDeg": 0, "abductionDeg": s * round(curl * (38 + 4 * i), 1), "twistDeg": -90 * s}
        out[f"{f}-middle-{side}"] = {"angleDeg": round(curl * (48 + 4 * i), 1)}
        out[f"{f}-distal-{side}"] = {"angleDeg": round(curl * (26 + 3 * i), 1)}
    out[f"thumb-metacarpal-{side}"] = {"angleDeg": round(curl * 18, 1), "abductionDeg": s * round(10 + curl * 20, 1), "twistDeg": -90 * s}
    out[f"thumb-proximal-{side}"] = {"angleDeg": round(curl * 22, 1), "abductionDeg": s * 16}
    out[f"thumb-distal-{side}"] = {"angleDeg": round(curl * 18, 1), "abductionDeg": s * 6}
    return out


def base():
    d = load("standing-adult")
    a = d["asset"]
    a["id"] = SLUG
    a["anchors"] = [x for x in a["anchors"] if x["id"] != "ear"]
    scale_asset(a, K, MASS)
    return d


FIT = f"{H}/{SLUG}.fit.json"
CUP_H = 0.112
LEVEL = 0.101
LEGS = {"thigh-l": {"angleDeg": -2.5}, "shank-l": {"angleDeg": -2.5}, "foot-l": {"angleDeg": 5}, "toe-l": {"angleDeg": 0}, "thigh-r": {"angleDeg": -2.5}, "shank-r": {"angleDeg": -2.5}, "foot-r": {"angleDeg": 5}, "toe-r": {"angleDeg": 0}}
OPEN_L = {"arm-l": {"angleDeg": 0, "abductionDeg": 6}, "forearm-l": {"angleDeg": 9}, "hand-l": {"angleDeg": 3}}


def cup_bone(a):
    a["rig"]["bones"].append({"id": "cup", "parent": "hand-r", "title": text("Paper cup of coffee in the right hand", "Pappbecher mit Kaffee in der rechten Hand"), "x": 0.087, "y": -0.05, "z": 0.053, "angleDeg": 90, "lengthM": CUP_H,
                              "joint": {"type": "fixed"}, "mass": {"kg": CUP_KG, "comX": 0.048, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
    a["anchors"] += [{"id": "cup-cog", "bone": "cup", "x": 0.048, "y": 0, "kind": "load", "title": text("Centre of gravity of cup and coffee", "Schwerpunkt von Becher und Kaffee")},
                     {"id": "coffee-surface", "bone": "cup", "x": LEVEL, "y": 0, "kind": "emitter", "title": text("Coffee surface", "Kaffeeoberfläche")},
                     {"id": "rim", "bone": "cup", "x": CUP_H, "y": 0, "kind": "attachment", "title": text("Rim of the cup", "Becherrand")}]


CUP_50 = [(0, 0), (0, 0.0295), (0.0015, 0.0302), (0.108, 0.0448), (0.109, 0.0462), (0.112, 0.0462), (CUP_H, 0)]
CUP_100 = [(0, 0), (0, 0.0302), (0.108, 0.0448), (0.112, 0.0462), (CUP_H, 0)]
CUP_200 = [(0, 0), (0, 0.0302), (0.112, 0.0462), (CUP_H, 0)]
CUP_500 = [(0, 0), (0, 0.0375), (0.112, 0.0462), (CUP_H, 0)]
SLEEVE = [(0.03, 0), (0.03, 0.0362), (0.08, 0.043), (0.08, 0)]
COFFEE_R = 0.0428


def circle(r, n=24):
    return [[rnd(r * math.cos(2 * math.pi * i / n)), rnd(r * math.sin(2 * math.pi * i / n))] for i in range(n)]


def wall_loop(sleeve):
    out = [(0, 0.027), (0, 0.0295)] + ([(0.03, 0.0335), (0.03, 0.0362), (0.08, 0.043), (0.08, 0.0402)] if sleeve else []) + [(0.108, 0.0448), (0.109, 0.0462), (0.112, 0.0462), (0.112, 0.0447), (0.0025, 0.0287), (0.0025, 0.027), (0, 0.027)]
    return [[rnd(u), rnd(r)] for u, r in out]


def cup_parts(a):
    """The paper cup: wall, bottom and coffee as solids of revolution about the cup bone; the coffee surface shows in plan."""
    surf = lambda z0, z1: {"kind": "extruded", "sweep": "u", "z0": z0, "z1": z1}
    for view, z in (("elevation", 5.95), ("front", 4.65), ("plan", 2.45)):
        shapes(a, "50", view).extend([revolve_profile_shape("cup", "cup", CUP_50, 0.94, "cup", z, material="paper-cup"), revolve_profile_shape("cup-sleeve", "cup", SLEEVE, 0.55, "cup", z + 0.01, material="cardboard-sleeve")])
        shapes(a, "100", view).extend([revolve_profile_shape("cup", "cup", CUP_100, 0.94, "cup", z, material="paper-cup"), revolve_profile_shape("cup-sleeve", "cup", SLEEVE, 0.55, "cup", z + 0.01, material="cardboard-sleeve")])
        shapes(a, "200", view).append(revolve_profile_shape("cup", "cup", CUP_200, 0.94, "cup", z, material="paper-cup"))
        shapes(a, "500", view).append(revolve_profile_shape("cup", "cup", CUP_500, 0.94, ["cup", "coffee"], z, material="paper-cup", penMm=0.13))
    fill = [(0.0025, 0), (0.0025, 0.0287), (LEVEL, COFFEE_R), (LEVEL, 0)]
    for sc in ("50", "100", "200"):
        for view, z in (("elevation", 5.9), ("front", 4.6)):
            shapes(a, sc, view).append(revolve_profile_shape("coffee", "cup", fill, 0.12, "coffee", z, material="coffee"))
    for sc, r in (("50", COFFEE_R), ("100", COFFEE_R), ("200", COFFEE_R)):
        shapes(a, sc, "plan").append({"id": "coffee", "bone": "cup", "role": "body", "z": 2.47, "filled": True, "tone": 0.12, "region": "coffee", "material": "coffee", "kind": "poly", "points": circle(r), "closed": True, "solid": surf(0.1112, 0.1118)})
    for sc in ("50", "100", "200"):
        body = solids(a, sc, "body")
        body += [{"id": "cup-wall", "kind": "revolve", "bone": "cup", "material": "paper-cup", "role": "body", "tone": 0.94, "profile": wall_loop(sc != "200"), "axis": [1, 0, 0], "origin": [0, 0, 0]},
                 {"id": "cup-bottom", "kind": "revolve", "bone": "cup", "material": "paper-cup", "role": "body", "tone": 0.94, "profile": [[0.001, 0], [0.001, 0.027], [0.0025, 0.027], [0.0025, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0], "sheet": True},
                 {"id": "coffee", "kind": "revolve", "bone": "cup", "material": "coffee", "role": "body", "tone": 0.12, "profile": [[0.0025, 0], [0.0025, 0.0287], [LEVEL, rnd(COFFEE_R)], [LEVEL, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0]}]
        lay = {"layer": "cup-wall"} if sc == "50" else {}
        solids(a, sc, "thermal").extend([{"id": "th-cup-wall", "of": "cup-wall", "region": "cup", **lay}, {"id": "th-cup-bottom", "of": "cup-bottom", "region": "cup", **lay}, {"id": "th-coffee", "of": "coffee", "region": "coffee"},
                                         {"id": "th-coffee-steam", "kind": "ellipsoid", "bone": "cup", "material": "coffee", "role": "detail", "tone": 0.1, "radii": [{"50": 0.004, "100": 0.0078, "200": 0.0155}[sc], 0.03, 0.03], "transform": {"translation": [LEVEL, 0, 0]}, "region": "coffee", "tags": ["emitter", "coffee-steam"]}])
        solids(a, sc, "structure").append({"id": "st-cup", "of": "cup-wall", "member": "cup", "part": "member"})
    solids(a, "500", "body").extend([{"id": "cup", "kind": "revolve", "bone": "cup", "material": "paper-cup", "role": "body", "tone": 0.94, "profile": [[rnd(u), rnd(r)] for u, r in CUP_500], "axis": [1, 0, 0], "origin": [0, 0, 0]},
                                     {"id": "coffee", "kind": "revolve", "bone": "cup", "material": "coffee", "role": "body", "tone": 0.12, "profile": [[0.112, 0], [0.112, 0.044], [0.1135, 0.044], [0.1135, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0], "sheet": True}])
    solids(a, "500", "thermal").extend([{"id": "th-cup", "of": "cup", "region": "cup"}, {"id": "th-coffee", "of": "coffee", "region": "coffee"},
                                        {"id": "th-coffee-steam", "kind": "ellipsoid", "bone": "cup", "material": "coffee", "role": "detail", "tone": 0.1, "radii": [0.0375, 0.0375, 0.0375], "transform": {"translation": [0.1135, 0, 0]}, "region": "coffee", "tags": ["emitter", "coffee-steam"]}])
    solids(a, "500", "structure").append({"id": "st-cup", "of": "cup", "member": "cup", "part": "member"})


def fist_hands(a):
    """At 1:200 and 1:500 the hand masses are redrawn as closed grips 120 mm long: the fingers curl round the cup instead of lying flat."""
    for side, sgn in (("r", 1), ("l", -1)):
        for sc, pts in (("200", [[-0.0153, -0.0275], [0.05, -0.0397], [0.104, -0.0385], [0.117, -0.022], [0.12, 0.0], [0.117, 0.024], [0.104, 0.0395], [0.05, 0.0407], [-0.0153, 0.0275]]),
                        ("500", [[-0.0102, -0.0448], [0.108, -0.0448], [0.121, 0.0], [0.108, 0.0448], [-0.0102, 0.0448]])):
            for v in VIEWS:
                for sh in shapes(a, sc, v):
                    if sh["id"] == f"palm-{side}":
                        zr = (sh["solid"]["z0"] + sh["solid"]["z1"]) / 2 if sc == "200" else None
                        sh["points"] = pts
                        if sc == "200":
                            lo, hi = (-0.017, 0.035) if sgn > 0 else (-0.035, 0.017)
                            sh["solid"]["z0"], sh["solid"]["z1"] = rnd(zr + lo), rnd(zr + hi)
            for sol in solids(a, sc, "body"):
                if sol["id"] == f"hand-{side}":
                    secs = sol["sections"]
                    if sc == "200":
                        rot = secs[0]["rotation"]
                        prof = [(-0.0153, 0.0275), (0.05, 0.0402), (0.104, 0.039), (0.12, 0.012)]
                        sol["sections"] = [{"at": [u, 0.0005 if i else 0.0, rnd(0.009 * sgn)], "rx": 0.026 if i < 3 else 0.012, "ry": ry, "exponent": 2.6 if i else 2.4, "rotation": rot} for i, (u, ry) in enumerate(prof)]
                    else:
                        secs[1]["at"][0] = 0.108
                        secs[2]["at"][0] = 0.119


def build_poses(a):
    """Four named poses; the arm offsets come from `fit.ts`, cached in the .fit.json design input (delete it to refit)."""
    template = copy.deepcopy(a["poses"][0])
    template.pop("key", None)
    cold = {"clavicle-l": {"angleDeg": 5.0}, "clavicle-r": {"angleDeg": 5.0}}
    specs = {
        "holding-cup": ({"torso": {"angleDeg": 0}, "lumbar": {"angleDeg": 0}, "neck": {"angleDeg": -3}, "head": {"angleDeg": 0}, **OPEN_L, **grip("r", 1.0), **grip("l", 0.25),
                         "arm-r": {"angleDeg": 12, "abductionDeg": -8, "twistDeg": -30}, "forearm-r": {"angleDeg": 92, "twistDeg": 20}, "hand-r": {"angleDeg": 0}},
                        lambda fr: [{"kind": "point", "bone": "cup", "local": [0, 0, 0], "world": [0.33, 1.04, -0.09], "weight": 1}, {"kind": "axis", "bone": "cup", "axis": "u", "world": [0, 1, 0], "weight": 0.05}], ARM_R),
        "sipping": ({"torso": {"angleDeg": 0}, "lumbar": {"angleDeg": 0}, "neck": {"angleDeg": 2}, "head": {"angleDeg": 7}, **OPEN_L, **grip("r", 1.0), **grip("l", 0.25),
                     "arm-r": {"angleDeg": 55, "abductionDeg": -15, "twistDeg": -30}, "forearm-r": {"angleDeg": 130, "twistDeg": 20}, "hand-r": {"angleDeg": 10}},
                    lambda fr: [{"kind": "point", "bone": "cup", "local": [CUP_H, -0.042, 0], "world": [fr["anchors"]["mouth"][0] + 0.004, fr["anchors"]["mouth"][1] + 0.002, 0.0], "weight": 1},
                                {"kind": "axis", "bone": "cup", "axis": "u", "world": unit([-0.6, 0.8, 0]), "weight": 0.05}, {"kind": "axis", "bone": "cup", "axis": "v", "world": unit([0.8, 0.6, 0]), "weight": 0.02}], ARM_R),
        "blowing-on-coffee": ({"torso": {"angleDeg": -2}, "lumbar": {"angleDeg": -1}, "neck": {"angleDeg": -8}, "head": {"angleDeg": -12}, **OPEN_L, **grip("r", 1.0), **grip("l", 0.25),
                               "arm-r": {"angleDeg": 35, "abductionDeg": -12, "twistDeg": -30}, "forearm-r": {"angleDeg": 120, "twistDeg": 20}, "hand-r": {"angleDeg": 5}},
                              lambda fr: [{"kind": "point", "bone": "cup", "local": [CUP_H, 0, 0], "world": [fr["anchors"]["mouth"][0] + 0.09, fr["anchors"]["mouth"][1] - 0.07, -0.01], "weight": 1}, {"kind": "axis", "bone": "cup", "axis": "u", "world": [0, 1, 0], "weight": 0.05}], ARM_R),
        "warming-hands": ({"torso": {"angleDeg": -1.5}, "lumbar": {"angleDeg": -1}, "neck": {"angleDeg": -6}, "head": {"angleDeg": -8}, **cold, **grip("r", 1.0), **grip("l", 0.9),
                           "arm-r": {"angleDeg": 20, "abductionDeg": 2, "twistDeg": -45}, "forearm-r": {"angleDeg": 105, "twistDeg": 25}, "hand-r": {"angleDeg": -5},
                           "arm-l": {"angleDeg": 20, "abductionDeg": -2, "twistDeg": 45}, "forearm-l": {"angleDeg": 105, "twistDeg": -25}, "hand-l": {"angleDeg": -5}},
                          lambda fr: [{"kind": "point", "bone": "cup", "local": [0, 0, 0], "world": [0.27, 1.12, -0.01], "weight": 1}, {"kind": "axis", "bone": "cup", "axis": "u", "world": [0, 1, 0], "weight": 0.05}], ARM_R),
    }
    cache = json.load(open(FIT)) if os.path.exists(FIT) else {}
    a["poses"] = []
    for pid, (bones, targets, arms) in specs.items():
        p = copy.deepcopy(template)
        p["id"] = pid
        p["bones"] = {**LEGS, **bones}
        a["poses"].append(p)
    for pid, (bones, targets, arms) in specs.items():
        if pid not in cache:
            fr = probe_asset(a, pid, SLUG)
            fit_pose(a, pid, arms, targets(fr), restarts=3, root_y=fr["root"]["y"])
            if pid == "warming-hands":
                fr = probe_asset(a, pid, SLUG)
                cup = fr["bones"]["cup"]
                fit_pose(a, pid, ARM_L, [{"kind": "point", "bone": "hand-l", "local": [0.087, 0, -0.053], "world": frame_point(cup, [0.05, 0, 0]), "weight": 1},
                                         {"kind": "axis", "bone": "hand-l", "axis": "v", "world": cup["u"], "weight": 0.03},
                                         {"kind": "point", "bone": "hand-l", "local": [0, 0, 0], "world": frame_point(cup, [0.05, 0, 0.0]) [:2] + [frame_point(cup, [0.05, 0, 0])[2] + 0.09], "weight": 0.1}], restarts=3, root_y=fr["root"]["y"])
            cache[pid] = pose(a, pid)["bones"]
            json.dump(cache, open(FIT, "w"), indent=1)
        pose(a, pid)["bones"] = cache[pid]


if __name__ == "__main__":
    d = base()
    a = d["asset"]
    if STAGE == "base":
        p = rest_pose(a)
        q = rest_pose(a, "curl")
        q["bones"] = grip("r", 1.0)
        write(d, f"{O}/[DEBUG]-{SLUG}-base.json")
        sys.exit()
    cup_bone(a)
    build_poses(a)
    cup_parts(a)
    fist_hands(a)
    import importlib.util
    spec = importlib.util.spec_from_file_location("content", f"{H}/{SLUG}-content.py")
    content = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(content)
    content.apply(a)
    path = f"{O}/[DEBUG]-{SLUG}-draft.json"
    write(d, path)
    fit_dimensions(a, path)
    write(d, path)
    if STAGE == "ship":
        write(d, f"{CATALOG}/{SLUG}/🔣️.json")
