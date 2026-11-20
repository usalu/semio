"""🐷 Round 3 root fix of animals/pig: the rig from real skeletal landmarks of a 100 kg finishing pig (standing, world metres, hip joint above x = 0), the anchors on it, the dimensions of the modelled animal and every pose re-solved on the new chain (hooves keep their ground contacts and stride).

Usage: python3 r3-pig-rig.py [--apply]   (reads the pre-round-3 asset from 🗑️generated/r3-pig/asset-before.json so that it can be re-run)
Landmarks and sources: r3-animals-pig.md.
"""
import importlib.util
import json
import math
import sys

spec = importlib.util.spec_from_file_location("io", "r3-pig-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

OLD = json.load(open("🗑️generated/r3-pig/asset-before.json", encoding="utf-8"))["asset"]
D = math.degrees
R = math.radians


def dirv(a):
    return (math.cos(R(a)), math.sin(R(a)))


def ang(a, b):
    return D(math.atan2(b[1] - a[1], b[0] - a[0]))


def dist(a, b):
    return math.hypot(b[0] - a[0], b[1] - a[1])


def norm(a):
    while a > 180:
        a -= 360
    while a <= -180:
        a += 360
    return a


def two_bone(p, q, l1, l2, bend):
    d = min(max(dist(p, q), abs(l1 - l2) + 1e-6), l1 + l2 - 1e-6)
    base = ang(p, q)
    a = D(math.acos((l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    th1 = base + bend * a
    mid = (p[0] + l1 * dirv(th1)[0], p[1] + l1 * dirv(th1)[1])
    return th1, ang(mid, q), dist(p, q) > l1 + l2


HIP = (0.0, 0.505)
FEMUR, TIBIA = 0.2, 0.185
J = {
    "ls": (0.1, 0.6), "tl": (0.345, 0.615), "ct": (0.69, 0.515), "poll": (0.855, 0.535),
    "tail0": (-0.105, 0.612), "tail1": (-0.146, 0.624), "tail2": (-0.138, 0.592),
    "hock": (-0.04, 0.205), "fet-h": (-0.02, 0.078), "toe-h": (0.035, 0.0),
    "scap": (0.6, 0.6), "shoulder": (0.705, 0.445), "elbow": (0.595, 0.305), "carpus": (0.605, 0.165), "fet-f": (0.6, 0.075), "toe-f": (0.65, 0.0),
}
_th = two_bone(HIP, J["hock"], FEMUR, TIBIA, +1)[0]
J["stifle"] = (HIP[0] + FEMUR * dirv(_th)[0], HIP[1] + FEMUR * dirv(_th)[1])
HEAD_ANGLE, HEAD_LEN = -24.0, 0.262
J["muzzle"] = (J["poll"][0] + HEAD_LEN * dirv(HEAD_ANGLE)[0], J["poll"][1] + HEAD_LEN * dirv(HEAD_ANGLE)[1])


def head_point(u, v):
    c, s = dirv(HEAD_ANGLE)
    return (J["poll"][0] + u * c - v * s, J["poll"][1] + u * s + v * c)


J["tmj"] = head_point(0.03, -0.008)
J["incisor"] = head_point(0.236, -0.04)
J["ear"] = head_point(0.035, 0.076)
J["ear-tip"] = (J["ear"][0] + 0.16 * dirv(-52)[0], J["ear"][1] + 0.16 * dirv(-52)[1])

CHAIN = [
    ("pelvis", None, HIP, J["ls"]), ("lumbar", "pelvis", J["ls"], J["tl"]), ("thorax", "lumbar", J["tl"], J["ct"]), ("neck", "thorax", J["ct"], J["poll"]),
    ("head", "neck", J["poll"], J["muzzle"]), ("jaw", "head", J["tmj"], J["incisor"]), ("ear-n", "head", J["ear"], J["ear-tip"]), ("ear-f", "head", J["ear"], J["ear-tip"]),
    ("tail-1", "pelvis", J["tail0"], J["tail1"]), ("tail-2", "tail-1", J["tail1"], J["tail2"]),
]
for s in ("n", "f"):
    CHAIN += [
        ("thigh-h" + s, "pelvis", HIP, J["stifle"]), ("shank-h" + s, "thigh-h" + s, J["stifle"], J["hock"]), ("metatarsus-h" + s, "shank-h" + s, J["hock"], J["fet-h"]), ("hoof-h" + s, "metatarsus-h" + s, J["fet-h"], J["toe-h"]),
        ("scapula-f" + s, "thorax", J["scap"], J["shoulder"]), ("humerus-f" + s, "scapula-f" + s, J["shoulder"], J["elbow"]), ("forearm-f" + s, "humerus-f" + s, J["elbow"], J["carpus"]),
        ("metacarpus-f" + s, "forearm-f" + s, J["carpus"], J["fet-f"]), ("hoof-f" + s, "metacarpus-f" + s, J["fet-f"], J["toe-f"]),
    ]
WORLD = {b: (a, t, ang(a, t), dist(a, t)) for b, _p, a, t in CHAIN}
PARENT = {b: p for b, p, _a, _t in CHAIN}


def to_local(bone, p):
    a, _t, th, _l = WORLD[bone]
    c, s = dirv(th)
    dx, dy = p[0] - a[0], p[1] - a[1]
    return (round(dx * c + dy * s, 4), round(-dx * s + dy * c, 4))


LIMITS = {"neck": (-70, 50), "head": (-70, 50)}
COM = {"pelvis": (0.02, 0.5), "lumbar": (0.22, 0.49), "thorax": (0.5, 0.46), "neck": (0.77, 0.5), "head": (0.95, 0.47)}


def new_rig():
    bones = []
    for old in OLD["rig"]["bones"]:
        b = dict(old)
        a, _t, th, ln = WORLD[b["id"]]
        p = PARENT[b["id"]]
        if p is None:
            b["x"], b["y"], b["angleDeg"] = 0, 0, round(th, 3)
        else:
            lx, ly = to_local(p, a)
            b["x"], b["y"], b["angleDeg"] = lx, ly, round(norm(th - WORLD[p][2]), 3)
        b["lengthM"] = round(ln, 4)
        if b["id"] in LIMITS:
            b["joint"] = {**b["joint"], "minDeg": LIMITS[b["id"]][0], "maxDeg": LIMITS[b["id"]][1]}
        m = dict(b["mass"])
        if b["id"] in COM:
            m["comX"], m["comY"] = to_local(b["id"], COM[b["id"]])
        else:
            m["comX"], m["comY"] = round(ln / 2, 4), 0
        b["mass"] = m
        bones.append(b)
    return {**OLD["rig"], "bones": bones}


def fk(rig, offsets, root_angle=0.0):
    out = {}
    for b in rig["bones"]:
        off = offsets.get(b["id"], {}).get("angleDeg", 0.0)
        if b["parent"] is None:
            th = b["angleDeg"] + off + root_angle
            o = (0.0, 0.0)
        else:
            po, pth, _pl = out[b["parent"]]
            c, s = dirv(pth)
            o = (po[0] + b["x"] * c - b["y"] * s, po[1] + b["x"] * s + b["y"] * c)
            th = pth + b["angleDeg"] + off
        out[b["id"]] = (o, th, b["lengthM"])
    return out


def tip(state, bone):
    o, th, ln = state[bone]
    return (o[0] + ln * dirv(th)[0], o[1] + ln * dirv(th)[1])


NEW = new_rig()
NEW_LEN = {b["id"]: b["lengthM"] for b in NEW["bones"]}
NEW_REL = {b["id"]: b["angleDeg"] for b in NEW["bones"]}
REST_OLD = fk(OLD["rig"], {})
REST_NEW = fk(NEW, {})
BELLY = (0.36, 0.294)
BRISKET = (0.6, 0.29)
LEGS = {}
for s in ("n", "f"):
    LEGS["h" + s] = (None, "thigh-h" + s, "shank-h" + s, "metatarsus-h" + s, "hoof-h" + s, +1)
    LEGS["f" + s] = ("scapula-f" + s, "humerus-f" + s, "forearm-f" + s, "metacarpus-f" + s, "hoof-f" + s, -1)
TRUNK = ["pelvis", "lumbar", "thorax", "neck", "head", "jaw", "ear-n", "ear-f", "tail-1", "tail-2"]
warn = []


def solve_pose(pid, trunk_off, legs, toe_scale=1.0):
    """legs[leg] = { toe: shift of the toe from rest relative to the root, hoof / cannon / scap: world angle deltas from rest } or { world: { segment: world angle } }."""
    st = fk(NEW, {b: {"angleDeg": v} for b, v in trunk_off.items()})
    world = {b: st[b][1] for b in TRUNK}
    for leg, want in legs.items():
        scap, up, low, cannon, hoof, bend = LEGS[leg]
        if "world" in want:
            for b in ([scap] if scap else []) + [up, low, cannon, hoof]:
                world[b] = want["world"][b.split("-")[0]]
            continue
        toe0 = tip(REST_NEW, hoof)
        toe = (toe0[0] + want["toe"][0] * toe_scale, toe0[1] + want["toe"][1] * toe_scale)
        ha = REST_NEW[hoof][1] + want.get("hoof", 0.0)
        ca = REST_NEW[cannon][1] + want.get("cannon", 0.0)
        fet = (toe[0] - NEW_LEN[hoof] * dirv(ha)[0], toe[1] - NEW_LEN[hoof] * dirv(ha)[1])
        knee = (fet[0] - NEW_LEN[cannon] * dirv(ca)[0], fet[1] - NEW_LEN[cannon] * dirv(ca)[1])
        prox = st[up][0] if not scap else None
        if scap:
            sa = REST_NEW[scap][1] + (world["thorax"] - REST_NEW["thorax"][1]) + want.get("scap", 0.0)
            world[scap] = sa
            so = st[scap][0]
            prox = (so[0] + NEW_LEN[scap] * dirv(sa)[0], so[1] + NEW_LEN[scap] * dirv(sa)[1])
        th1, th2, far = two_bone(prox, knee, NEW_LEN[up], NEW_LEN[low], bend)
        if far:
            warn.append("%s %s unreachable by %.3f" % (pid, leg, dist(prox, knee) - NEW_LEN[up] - NEW_LEN[low]))
        world[up], world[low], world[cannon], world[hoof] = th1, th2, ca, ha
    bones = {}
    for b in NEW["bones"]:
        p = b["parent"]
        rel = norm(world[b["id"]] - (world[p] if p else 0.0) - b["angleDeg"])
        if abs(rel) > 5e-4:
            bones[b["id"]] = {"angleDeg": round(rel, 4)}
            lim = b["joint"]
            if not lim["minDeg"] - 1e-6 <= rel <= lim["maxDeg"] + 1e-6:
                warn.append("%s %s offset %.1f outside %s..%s" % (pid, b["id"], rel, lim["minDeg"], lim["maxDeg"]))
    return bones


def old_deltas(pose):
    st = fk(OLD["rig"], pose["bones"])
    legs = {}
    for leg, (scap, _up, _low, cannon, hoof, _bend) in LEGS.items():
        t, t0 = tip(st, hoof), tip(REST_OLD, hoof)
        legs[leg] = {"toe": (t[0] - t0[0], t[1] - t0[1]), "hoof": norm(st[hoof][1] - REST_OLD[hoof][1]), "cannon": norm(st[cannon][1] - REST_OLD[cannon][1])}
        if scap:
            legs[leg]["scap"] = norm(st[scap][1] - REST_OLD[scap][1])
    return legs


def root_trunk():
    """Rooting: trunk pitched 5 degrees down in front, head steep, the snout disc 0.07 m above the soil."""
    best = None
    for k in range(-700, 0):
        neck_off = k / 10.0
        neck_w = REST_NEW["neck"][1] - 5.0 + neck_off
        head_w = neck_w - 38.0
        trunk = {"pelvis": -5.0, "neck": neck_off, "head": norm(head_w - neck_w - NEW_REL["head"])}
        st = fk(NEW, {b: {"angleDeg": v} for b, v in trunk.items()})
        miss = abs(tip(st, "head")[1] + HIP[1] - 0.07)
        if best is None or miss < best[0]:
            best = (miss, trunk)
    return best[1]


STAND = {"hn": {"toe": (0, 0)}, "fn": {"toe": (0, 0)}, "hf": {"toe": (0.055, 0)}, "ff": {"toe": (0.09, 0), "scap": 9.0}}
poses = []
for pose in OLD["poses"]:
    p = json.loads(json.dumps(pose))
    pid = p["id"]
    if pid == "standing-pig":
        p["bones"] = solve_pose(pid, {}, STAND)
    elif pid == "rooting":
        p["bones"] = solve_pose(pid, root_trunk(), {"hn": {"toe": (0, 0)}, "hf": {"toe": (0.075, 0)}, "fn": {"toe": (-0.03, 0), "cannon": -5.0, "hoof": -3.0}, "ff": {"toe": (0.09, 0), "cannon": 8.0, "hoof": 4.0}})
        p["root"] = {"y": HIP[1]}
    elif pid == "lying-wallowing":
        trunk = {"neck": 22.0 - REST_NEW["neck"][1], "head": norm(-12.0 - 22.0 - NEW_REL["head"])}
        scap = REST_NEW["scapula-fn"][1]
        p["bones"] = solve_pose(pid, trunk, {
            "hn": {"world": {"thigh": -22.0, "shank": -166.0, "metatarsus": -14.0, "hoof": -8.0}},
            "hf": {"world": {"thigh": -18.0, "shank": -163.0, "metatarsus": -16.0, "hoof": -9.0}},
            "fn": {"world": {"scapula": scap, "humerus": -168.0, "forearm": -24.0, "metacarpus": 176.0, "hoof": 172.0}},
            "ff": {"world": {"scapula": scap, "humerus": -163.0, "forearm": -20.0, "metacarpus": 174.0, "hoof": 170.0}},
        })
        p["root"] = {"y": round(HIP[1] - BELLY[1], 4)}
    else:
        p["bones"] = solve_pose(pid, {b: pose["bones"].get(b, {}).get("angleDeg", 0.0) for b in TRUNK}, old_deltas(pose), 1.0)
    poses.append(p)

ANCHOR = {
    "contact-hn": ("hoof-hn", J["toe-h"]), "contact-hf": ("hoof-hf", J["toe-h"]), "contact-fn": ("hoof-fn", J["toe-f"]), "contact-ff": ("hoof-ff", J["toe-f"]),
    "contact-belly": ("thorax", BELLY), "contact-brisket": ("thorax", BRISKET), "nose": ("head", head_point(0.264, 0.004)), "mouth": ("head", head_point(0.2, -0.03)), "back-plume": ("thorax", (0.42, 0.679)),
}
anchors = []
for old in OLD["anchors"]:
    a = dict(old)
    bone, at = ANCHOR[a["id"]]
    a["bone"] = bone
    a["x"], a["y"] = to_local(bone, at)
    anchors.append(a)

for w in warn:
    print("WARN", w)
print("stifle (%.3f, %.3f) muzzle (%.3f, %.3f) ear tip (%.3f, %.3f)" % (*J["stifle"], *J["muzzle"], *J["ear-tip"]))
for b in NEW["bones"]:
    if not b["id"].endswith("f") or b["id"] == "ear-f":
        print("  %-14s len %.4f world %7.2f" % (b["id"], b["lengthM"], WORLD[b["id"]][2]))
for pid in ("standing-pig", "rooting", "lying-wallowing", "walk-phase-3"):
    st = fk(NEW, next(p for p in poses if p["id"] == pid)["bones"])
    print(pid, "toes", " ".join("%s(%.3f,%.3f)" % (k, *tip(st, LEGS[k][4])) for k in LEGS), "muzzle (%.3f,%.3f)" % tip(st, "head"))
if "--apply" in sys.argv:
    members = {"rig": NEW, "anchors": anchors, "poses": poses}
    if "--dimensions" in sys.argv:
        k = sys.argv.index("--dimensions")
        members["dimensions"] = {**OLD["dimensions"], "heightM": float(sys.argv[k + 1]), "widthM": float(sys.argv[k + 2]), "depthM": float(sys.argv[k + 3])}
    if not any(m["id"] == "bone" for m in OLD["materials"]):
        members["materials"] = OLD["materials"] + [{"id": "bone", "title": {"en": "Bone", "de": "Knochen"}, "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1900, "source": "authored-estimate"}]
    io.put(members)
    print("applied")
