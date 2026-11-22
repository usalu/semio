"""🦴 Stage 7 of the seated office worker: the skeleton of the sitter restated so that every bone meets its neighbour at the joint and the load path ends where the body bears: the bearing pads of soft tissue under the seat bones, heels, balls of the feet and heels of the hands are members that touch what carries them (cushion, floor, palm rest), per scale.

Run after stage 5: `python3 rework-03-seated-office-worker-7-structure.py` (repeatable; replaces solids by id, adds the new ones next to their bone).
"""
from rework_03_lib import *

A = "humans/seated-office-worker"
src = read(source_path(A))
V = src["volumes"]
FOOT = [0, 0, 0.28661, 0.95805]


def bone(id, b, rows, floor=0.0, **more):
    rows = [(r[0], r[1], r[2], max(r[3], floor), max(r[4], floor)) for r in rows]
    return loft(id, b, "bone", 0.62, rows, across=more.pop("across", False), member=b, part="member", **more)


def blob(id, b, radii, at, part="member", tone=0.62, floor=0.0, **more):
    return ell(id, b, "bone", tone, [max(r, floor) for r in radii], at, member=b, part=part, **more)


def skeleton(f):
    left = [
        bone("ilium-body-l", "pelvis", [(0.05, 0.06, 0.03, 0.018, 0.018), (0.025, 0.03, 0.055, 0.02, 0.02), (0.0, 0.0, 0.075, 0.022, 0.022)], f),
        bone("ilium-l", "pelvis", [(0.0, 0.0, 0.078, 0.022, 0.022), (0.05, 0.022, 0.108, 0.009, 0.05), (0.108, 0.02, 0.124, 0.007, 0.058), (0.118, 0.02, 0.125, 0.005, 0.04)], f, across=True),
        bone("ischium-l", "pelvis", [(-0.008, 0.008, 0.08, 0.014, 0.014), (-0.04, 0.02, 0.068, 0.013, 0.013), (-0.062, 0.018, 0.058, 0.014, 0.014), (-0.05, -0.026, 0.03, 0.009, 0.009), (-0.03, -0.052, 0.01, 0.009, 0.009)], f),
        bone("pubis-l", "pelvis", [(0.004, -0.02, 0.078, 0.012, 0.012), (-0.02, -0.048, 0.04, 0.01, 0.01), (-0.03, -0.058, 0.006, 0.011, 0.011)], f),
        bone("clavicle-l", "torso", [(0.25, -0.108, 0.016, 0.0075, 0.0075), (0.27, -0.097, 0.09, 0.0065, 0.0065), (0.281, -0.05, 0.165, 0.0075, 0.0075)], f),
        bone("scapula-l", "torso", [(0.105, 0.082, 0.092, 0.008, 0.006), (0.16, 0.074, 0.105, 0.03, 0.0065), (0.215, 0.064, 0.112, 0.042, 0.007), (0.255, 0.058, 0.112, 0.044, 0.007), (0.268, 0.058, 0.1, 0.03, 0.006)], f, across=True),
        bone("glenoid-l", "torso", [(0.235, 0.056, 0.14, 0.012, 0.012), (0.245, 0.01, 0.152, 0.013, 0.013), (0.25, -0.03, 0.154, 0.016, 0.016)], f),
        bone("acromion-l", "torso", [(0.245, 0.062, 0.085, 0.007, 0.007), (0.272, 0.03, 0.15, 0.007, 0.007), (0.28, -0.015, 0.18, 0.008, 0.008), (0.28, -0.044, 0.172, 0.008, 0.008)], f),
        blob("humerus-head-l", "arm-l", [0.024, 0.024, 0.024], [0.004, -0.004, 0]),
        bone("humerus-l", "arm-l", [(0.01, 0, 0, 0.0125, 0.0125), (0.16, 0.002, 0, 0.011, 0.011), (0.31, 0, 0, 0.0115, 0.0115)], f),
        blob("humerus-condyles-l", "arm-l", [0.014, 0.014, 0.031], [0.318, 0, 0]),
        bone("radius-l", "forearm-l", [(0.006, 0.004, 0.018, 0.009, 0.009), (0.13, 0.002, 0.02, 0.0075, 0.0075), (0.252, 0, 0.014, 0.011, 0.011)], f),
        bone("ulna-l", "forearm-l", [(-0.022, -0.012, -0.008, 0.011, 0.011), (0.02, -0.004, -0.014, 0.0105, 0.0105), (0.25, 0, -0.018, 0.0075, 0.0075)], f),
        blob("femur-head-l", "thigh-l", [0.024, 0.024, 0.024], [0, 0, 0]),
        bone("femur-neck-l", "thigh-l", [(0, 0, 0, 0.016, 0.016), (0.016, -0.004, 0.04, 0.014, 0.014), (0.026, -0.008, 0.066, 0.02, 0.02)], f),
        bone("femur-l", "thigh-l", [(0.022, -0.008, 0.064, 0.017, 0.017), (0.11, -0.002, 0.048, 0.0145, 0.0145), (0.3, 0.004, 0.018, 0.0145, 0.0145), (0.41, 0, 0, 0.02, 0.02)], f),
        blob("femur-condyles-l", "thigh-l", [0.03, 0.033, 0.042], [0.422, -0.006, 0]),
        blob("patella-l", "thigh-l", [0.012, 0.021, 0.022], [0.46, 0.012, 0]),
        blob("tibia-plateau-l", "shank-l", [0.02, 0.03, 0.04], [0.018, 0.002, 0]),
        bone("tibia-l", "shank-l", [(0.02, 0.01, -0.004, 0.019, 0.019), (0.12, 0.014, -0.006, 0.015, 0.015), (0.31, 0.01, -0.004, 0.0125, 0.0125), (0.428, 0.002, -0.006, 0.017, 0.017)], f),
        bone("fibula-l", "shank-l", [(0.03, -0.014, 0.034, 0.0085, 0.0085), (0.22, -0.012, 0.032, 0.006, 0.006), (0.432, -0.008, 0.03, 0.0095, 0.0095)], f),
        blob("talus-l", "foot-l", [0.026, 0.018, 0.02], [0.004, -0.012, 0], rotation=None),
        bone("calcaneus-l", "foot-l", [(-0.058, -0.052, 0, 0.014, 0.016), (-0.03, -0.05, 0, 0.019, 0.022), (0.0, -0.04, 0, 0.02, 0.02), (0.02, -0.034, 0, 0.018, 0.014)], f, across=True),
        bone("tarsals-l", "foot-l", [(0.018, -0.022, 0, 0.026, 0.016), (0.05, -0.036, 0, 0.03, 0.015), (0.075, -0.046, 0, 0.033, 0.013)], f, across=True),
        bone("metatarsals-l", "foot-l", [(0.07, -0.046, 0, 0.034, 0.012), (0.11, -0.06, 0, 0.04, 0.01), (0.142, -0.068, 0, 0.043, 0.009)], f, across=True),
        bone("toe-bones-l", "toe-l", [(-0.006, 0.022, 0, 0.043, 0.009), (0.03, 0.019, 0, 0.04, 0.008), (0.062, 0.016, 0, 0.03, 0.0075)], f, across=True),
        blob("heel-pad-l", "foot-l", [0.022, 0.013, 0.02], [-0.03, -0.081, 0], tone=0.5),
        blob("ball-pad-l", "foot-l", [0.02, 0.012, 0.035], [0.14, -0.082, 0], tone=0.5),
        blob("palm-pad-l", "hand-l", [0.016, 0.008, 0.024], [0.014, -0.012, 0], tone=0.5),
    ]
    for s in left:
        if s["bone"] == "foot-l":
            s.setdefault("transform", {})["rotation"] = FOOT
            if s["kind"] == "ellipsoid":
                import math
                x, y, z = s["transform"]["translation"]
                c, sn = math.cos(math.radians(33.3106)), math.sin(math.radians(33.3106))
                s["transform"]["translation"] = [round(x * c - y * sn, 5), round(x * sn + y * c, 5), z]
    out = [bone("sacrum", "pelvis", [(0.1, 0.036, 0, 0.03, 0.02), (0.05, 0.062, 0, 0.03, 0.015), (0, 0.08, 0, 0.016, 0.011), (-0.03, 0.082, 0, 0.008, 0.0075)], f, across=True),
           blob("seat-pad", "pelvis", [0.011, 0.03, 0.085], [-0.079, 0.02, 0], tone=0.5)]
    for s in left:
        out += both(s)
    return out


def apply(scale, new, drop=(), shift_ribs=0.0):
    lst = [s for s in V[scale]["structure"] if not s["id"].startswith("support-")]
    N = by_id(new)
    out, seen = [], set()
    for s in lst:
        if s["id"] in drop:
            continue
        if s["id"] in N:
            out.append(N[s["id"]])
            seen.add(s["id"])
            continue
        if shift_ribs and (s["id"].startswith("costal-") or s["id"] == "sternum"):
            s["transform"] = {"translation": [-0.18, shift_ribs, 0]}
        out.append(s)
    for s in new:
        if s["id"] not in seen:
            last = max((i for i, o in enumerate(out) if o.get("bone") == s["bone"] and o.get("part") == "member"), default=len(out) - 1)
            out.insert(last + 1, s)
    for s in out:
        if s["id"] == "joint-chair-backrest":
            s["radii"], s["transform"] = [{"200": 0.015, "500": 0.0377}.get(scale, 0.011)] * 3, {"translation": [0, 0, 0]}
        if s["id"] == "mass-desk-top":
            s["radii"] = [0.045, 0.012, 0.045] if scale != "500" else [0.06, 0.0125, 0.06]
        for k in "1234":
            if s["id"].endswith(f"phalanges-{k}-l") or s["id"].endswith(f"phalanges-{k}-r"):
                s["sections"][0]["at"][0] = 0.09
        if s["id"].startswith("metacarpal-"):
            s["sections"][0]["at"][0] = 0.02
        if s["id"] == "mass-chair-backrest":
            s["transform"]["translation"] = {"50": [0.31, 0.032, 0], "100": [0.31, 0.034, 0], "200": [0.33, 0.055, 0], "500": [0.3, 0.055, 0]}[scale]
        if s["id"].startswith("thumb-bones"):
            w = s["sections"][0]["at"][2]
            s["sections"][0]["at"] = [0.016, -0.005, 0.022 if w > 0 else -0.022]
    if not any(s.get("of") == "chair-seat-cushion" for s in out):
        at = max(i for i, s in enumerate(out) if s.get("member") == "chair-seat" and s.get("part") == "member" or s["id"] == "st-chair-column")
        out.insert(at + 1, {"id": "st-chair-seat-cushion", "of": "chair-seat-cushion", "member": "chair-seat", "part": "member"})
    V[scale]["structure"] = rnd(out)
    print(scale, len(out))


apply("50", skeleton(0.0), drop=("calcaneus-l", "calcaneus-r"), shift_ribs=0.008)
apply("100", [s for s in skeleton(0.0075)], shift_ribs=0.008)


def coarse(r, seat, tib):
    new = [blob("seat-pad", "pelvis", seat[0], seat[1], tone=0.5)]
    left = [
        bone("tibia-l", "shank-l", [(0, 0.006, 0, r, r), (0.2, 0.012, 0, max(r, 0.016), max(r, 0.016)), (tib, 0.002, 0, r, r)]),
        bone("forearm-bones-l", "forearm-l", [(0, 0, 0, max(r, 0.0152), max(r, 0.0152)), (0.262, 0, 0, max(r, 0.0152), max(r, 0.0152))]),
    ]
    for s in left:
        new += both(s)
    return new


t200 = bone("tarsals-l", "foot-l", [(-0.05, -0.048, 0, 0.024, 0.024), (0.0, -0.03, 0, 0.03, 0.03), (0.135, -0.066, 0, 0.03, 0.03)], transform={"rotation": FOOT})
bar200 = bone("shoulder-bar-l", "torso", [(0.27, 0.012, 0.0, 0.0152, 0.0152), (0.262, -0.036, 0.176, 0.0152, 0.0152)])
apply("200", coarse(0.018, ([0.02, 0.03, 0.085], [-0.075, 0.02, 0]), 0.445) + both(t200) + both(bar200))
apply("500", coarse(0.0377, ([0.0375, 0.04, 0.09], [-0.06, 0.02, 0]), 0.455))
write(source_path(A), src)
