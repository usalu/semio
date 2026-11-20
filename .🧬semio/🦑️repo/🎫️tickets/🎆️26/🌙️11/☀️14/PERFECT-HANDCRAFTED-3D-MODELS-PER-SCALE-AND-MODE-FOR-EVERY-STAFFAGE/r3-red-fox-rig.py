"""🦊 Round 3 root fix of animals/red-fox: the rig from the skeletal landmarks of a 6 kg red fox (standing, world metres, hip joint above x = 0), the anchors on it, every pose re-solved on the new chain (paws keep their ground contacts and stride, neck, head and brush keep their old world carriage outside the default pose) and the clip key overrides of the head.

Usage: python3 r3-red-fox-rig.py [--apply]   (reads the pre-round-3 asset from 🗑️generated/r3-red-fox/asset-before.json so that it can be re-run)
Landmarks and sources: r3-animals-red-fox.md.
"""
import importlib.util
import json
import math
import sys

spec = importlib.util.spec_from_file_location("io", "r3-red-fox-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

OLD = json.load(open("🗑️generated/r3-red-fox/asset-before.json", encoding="utf-8"))["asset"]
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


def step(a, angle, length):
    return (a[0] + length * dirv(angle)[0], a[1] + length * dirv(angle)[1])


HIP = (0.0, 0.375)
J = {
    "ls": (0.050, 0.400), "tl": (0.225, 0.404), "ct": (0.410, 0.362),
    "tail0": (-0.030, 0.423),
    "stifle": (0.055, 0.257), "hock": (0.012, 0.123), "mtp": (0.024, 0.036), "toe-h": (0.066, 0.0),
    "scap": (0.372, 0.408), "shoulder": (0.424, 0.328), "elbow": (0.378, 0.217), "carpus": (0.380, 0.099), "mcp": (0.391, 0.034), "toe-f": (0.428, 0.0),
}
J["occ"] = step(J["ct"], 50.0, 0.145)
HEAD_ANGLE, HEAD_LEN = -14.0, 0.155
J["nose"] = step(J["occ"], HEAD_ANGLE, HEAD_LEN)
J["tail1"] = step(J["tail0"], 205.0, 0.15)
J["tail2"] = step(J["tail1"], 200.0, 0.14)
J["tail3"] = step(J["tail2"], 192.0, 0.11)


def head_point(u, v):
    c, s = dirv(HEAD_ANGLE)
    return (J["occ"][0] + u * c - v * s, J["occ"][1] + u * s + v * c)


J["tmj"] = head_point(0.028, -0.012)
J["incisor"] = head_point(0.136, -0.019)
J["ear"] = head_point(0.020, 0.034)
J["ear-tip"] = step(J["ear"], 80.0, 0.09)

CHAIN = [
    ("pelvis", None, HIP, J["ls"]), ("lumbar", "pelvis", J["ls"], J["tl"]), ("thorax", "lumbar", J["tl"], J["ct"]), ("neck", "thorax", J["ct"], J["occ"]),
    ("head", "neck", J["occ"], J["nose"]), ("jaw", "head", J["tmj"], J["incisor"]), ("ear-n", "head", J["ear"], J["ear-tip"]), ("ear-f", "head", J["ear"], J["ear-tip"]),
    ("tail-1", "pelvis", J["tail0"], J["tail1"]), ("tail-2", "tail-1", J["tail1"], J["tail2"]), ("tail-3", "tail-2", J["tail2"], J["tail3"]),
]
for s in ("n", "f"):
    CHAIN += [
        ("thigh-h" + s, "pelvis", HIP, J["stifle"]), ("shank-h" + s, "thigh-h" + s, J["stifle"], J["hock"]), ("metatarsus-h" + s, "shank-h" + s, J["hock"], J["mtp"]), ("paw-h" + s, "metatarsus-h" + s, J["mtp"], J["toe-h"]),
        ("scapula-f" + s, "thorax", J["scap"], J["shoulder"]), ("humerus-f" + s, "scapula-f" + s, J["shoulder"], J["elbow"]), ("forearm-f" + s, "humerus-f" + s, J["elbow"], J["carpus"]),
        ("metacarpus-f" + s, "forearm-f" + s, J["carpus"], J["mcp"]), ("paw-f" + s, "metacarpus-f" + s, J["mcp"], J["toe-f"]),
    ]
WORLD = {b: (a, t, ang(a, t), dist(a, t)) for b, _p, a, t in CHAIN}
PARENT = {b: p for b, p, _a, _t in CHAIN}


def to_local(bone, p):
    a, _t, th, _l = WORLD[bone]
    c, s = dirv(th)
    dx, dy = p[0] - a[0], p[1] - a[1]
    return (round(dx * c + dy * s, 4), round(-dx * s + dy * c, 4))


DIMENSIONS = {"heightM": 0.587, "widthM": 1.1, "depthM": 0.152, "massKg": 6}
COM = {"pelvis": (0.02, 0.385), "lumbar": (0.14, 0.355), "thorax": (0.33, 0.325), "neck": (0.455, 0.41), "head": (0.565, 0.462)}


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
        m = dict(b["mass"])
        if b["id"] in COM:
            m["comX"], m["comY"] = to_local(b["id"], COM[b["id"]])
        else:
            m["comX"], m["comY"] = round(ln / 2, 4), 0
        b["mass"] = m
        bones.append(b)
    return {**OLD["rig"], "bones": bones}


def fk(rig, offsets):
    out = {}
    for b in rig["bones"]:
        off = offsets.get(b["id"], {}).get("angleDeg", 0.0)
        if b["parent"] is None:
            th = b["angleDeg"] + off
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


def two_bone(p, q, l1, l2, bend):
    d = min(max(dist(p, q), abs(l1 - l2) + 1e-6), l1 + l2 - 1e-6)
    base = ang(p, q)
    a = D(math.acos((l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    th1 = base + bend * a
    mid = (p[0] + l1 * dirv(th1)[0], p[1] + l1 * dirv(th1)[1])
    return th1, ang(mid, q), dist(p, q) - l1 - l2


NEW = new_rig()
NEW_LEN = {b["id"]: b["lengthM"] for b in NEW["bones"]}
REST_OLD = fk(OLD["rig"], {})
REST_NEW = fk(NEW, {})
LEGS = {}
for s in ("n", "f"):
    LEGS["h" + s] = (None, "thigh-h" + s, "shank-h" + s, "metatarsus-h" + s, "paw-h" + s, +1)
    LEGS["f" + s] = ("scapula-f" + s, "humerus-f" + s, "forearm-f" + s, "metacarpus-f" + s, "paw-f" + s, -1)
SPINE = ["pelvis", "lumbar", "thorax"]
CARRIED = ["neck", "head", "tail-1", "tail-2", "tail-3"]
RELATIVE = ["jaw", "ear-n", "ear-f"]
DROP = REST_OLD and (0.4067 - HIP[1])
warn = []


def solve_pose(pid, old_pose, legs, keep_world):
    """Spine bones keep their old offsets; neck, head and brush keep their old world angle when `keep_world`; legs[leg] = { toe, paw, cannon, scap } deltas from rest or { world: { bone: world angle } }."""
    old_state = fk(OLD["rig"], old_pose["bones"])
    world = {}
    offsets = {}
    for b in NEW["bones"]:
        bid = b["id"]
        p = b["parent"]
        old_off = old_pose["bones"].get(bid, {}).get("angleDeg", 0.0)
        if bid in SPINE or bid in RELATIVE:
            world[bid] = (world[p] if p else 0.0) + b["angleDeg"] + old_off
        elif bid in CARRIED:
            world[bid] = old_state[bid][1] if keep_world else world[p] + b["angleDeg"]
    st = {}
    for b in NEW["bones"]:
        bid = b["id"]
        if bid not in world:
            continue
        p = b["parent"]
        if p is None:
            st[bid] = ((0.0, 0.0), world[bid], b["lengthM"])
        else:
            po, pth, _ = st[p]
            c, s = dirv(pth)
            st[bid] = ((po[0] + b["x"] * c - b["y"] * s, po[1] + b["x"] * s + b["y"] * c), world[bid], b["lengthM"])
    by = {b["id"]: b for b in NEW["bones"]}
    for leg, want in legs.items():
        scap, up, low, cannon, paw, bend = LEGS[leg]
        if "world" in want:
            for b in ([scap] if scap else []) + [up, low, cannon, paw]:
                world[b] = want["world"][b]
            continue
        toe0 = tip(REST_NEW, paw)
        toe = (toe0[0] + want["toe"][0], toe0[1] + want["toe"][1])
        pa = REST_NEW[paw][1] + want.get("paw", 0.0)
        ca = REST_NEW[cannon][1] + want.get("cannon", 0.0)
        prox = (0.0, 0.0)
        if scap:
            sa = REST_NEW[scap][1] + (world["thorax"] - REST_NEW["thorax"][1]) + want.get("scap", 0.0)
            world[scap] = sa
            po, pth, _ = st["thorax"]
            c, s = dirv(pth)
            so = (po[0] + by[scap]["x"] * c - by[scap]["y"] * s, po[1] + by[scap]["x"] * s + by[scap]["y"] * c)
            prox = step(so, sa, NEW_LEN[scap])
        for _ in range(40):
            mtp = step(toe, pa + 180.0, NEW_LEN[paw])
            knee = step(mtp, ca + 180.0, NEW_LEN[cannon])
            th1, th2, over = two_bone(prox, knee, NEW_LEN[up], NEW_LEN[low], bend)
            if over <= -0.002:
                break
            want_dir = ang(prox, knee)
            ca = ca + 0.25 * norm(want_dir - ca)
            pa = pa + 0.12 * norm(want_dir - pa)
        if over > 0:
            warn.append("%s %s unreachable by %.3f" % (pid, leg, over))
        world[up], world[low], world[cannon], world[paw] = th1, th2, ca, pa
    bones = {}
    for b in NEW["bones"]:
        p = b["parent"]
        rel = norm(world[b["id"]] - (world[p] if p else 0.0) - b["angleDeg"])
        entry = {k: v for k, v in old_pose["bones"].get(b["id"], {}).items() if k != "angleDeg"}
        lim = b["joint"]
        if lim["minDeg"] - 5 <= rel <= lim["maxDeg"] + 5:
            rel = min(max(rel, lim["minDeg"]), lim["maxDeg"])
        if abs(rel) > 5e-4:
            entry = {"angleDeg": round(rel, 4), **entry}
            if not lim["minDeg"] - 1e-6 <= rel <= lim["maxDeg"] + 1e-6:
                warn.append("%s %s offset %.1f outside %s..%s" % (pid, b["id"], rel, lim["minDeg"], lim["maxDeg"]))
        if entry:
            bones[b["id"]] = entry
    return bones


def old_deltas(pose):
    st = fk(OLD["rig"], pose["bones"])
    legs = {}
    for leg, (scap, _up, _low, cannon, paw, _bend) in LEGS.items():
        t, t0 = tip(st, paw), tip(REST_OLD, paw)
        legs[leg] = {"toe": (t[0] - t0[0], t[1] - t0[1]), "paw": norm(st[paw][1] - REST_OLD[paw][1]), "cannon": norm(st[cannon][1] - REST_OLD[cannon][1])}
        if scap:
            legs[leg]["scap"] = norm(st[scap][1] - REST_OLD[scap][1])
    return legs


def old_world(pose):
    st = fk(OLD["rig"], pose["bones"])
    return {leg: {"world": {b: st[b][1] for b in ([scap] if scap else []) + [up, low, cannon, paw]}} for leg, (scap, up, low, cannon, paw, _bend) in LEGS.items()}


poses = []
for pose in OLD["poses"]:
    p = json.loads(json.dumps(pose))
    pid = p["id"]
    if pid == "standing-alert":
        p["bones"] = solve_pose(pid, pose, {"hn": {"toe": (0, 0)}, "fn": {"toe": (0, 0)}, "hf": {"toe": (0.08, 0), "cannon": 10.0}, "ff": {"toe": (0.055, 0)}}, False)
    elif pid == "curled-sleeping":
        lying = json.loads(json.dumps(pose))
        lying["bones"]["pelvis"] = {"angleDeg": 12.0}
        lying["bones"]["lumbar"] = {k: v for k, v in pose["bones"]["lumbar"].items() if k != "angleDeg"}
        scap = REST_NEW["scapula-fn"][1] + 12.0
        p["bones"] = solve_pose(pid, lying, {
            "hn": {"world": {"thigh-hn": -10.0, "shank-hn": -170.0, "metatarsus-hn": -5.0, "paw-hn": -25.0}},
            "hf": {"world": {"thigh-hf": -7.0, "shank-hf": -172.0, "metatarsus-hf": -3.0, "paw-hf": -22.0}},
            "fn": {"world": {"scapula-fn": scap, "humerus-fn": -125.0, "forearm-fn": 5.0, "metacarpus-fn": -3.0, "paw-fn": 0.0}},
            "ff": {"world": {"scapula-ff": scap, "humerus-ff": -121.0, "forearm-ff": 7.0, "metacarpus-ff": -4.0, "paw-ff": 0.0}},
        }, True)
        p["root"] = {**p.get("root", {}), "y": 0.078}
    else:
        p["bones"] = solve_pose(pid, pose, old_deltas(pose), True)
        if "root" in p and "y" in p["root"]:
            p["root"] = {**p["root"], "y": round(p["root"]["y"] - DROP, 4)}
    poses.append(p)

HEAD_SHIFT = norm(next(b for b in OLD["rig"]["bones"] if b["id"] == "head")["angleDeg"] - next(b for b in NEW["bones"] if b["id"] == "head")["angleDeg"])
clips = json.loads(json.dumps(OLD["clips"]))
for clip in clips:
    for key in clip["keys"]:
        head = key.get("bones", {}).get("head")
        if head is not None and "angleDeg" in head:
            pose_head = next(q for q in poses if q["id"] == key["pose"])["bones"].get("head", {}).get("angleDeg", 0.0)
            old_head = next(q for q in OLD["poses"] if q["id"] == key["pose"])["bones"].get("head", {}).get("angleDeg", 0.0)
            head["angleDeg"] = round(pose_head + (head["angleDeg"] - old_head), 4)

ANCHOR = {
    "contact-hn": ("paw-hn", J["toe-h"]), "contact-hf": ("paw-hf", J["toe-h"]), "contact-fn": ("paw-fn", J["toe-f"]), "contact-ff": ("paw-ff", J["toe-f"]),
    "nose": ("head", head_point(0.156, -0.004)), "mouth": ("head", head_point(0.118, -0.026)), "back-plume": ("thorax", (0.25, 0.437)),
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
print("head key shift", HEAD_SHIFT)
for b in NEW["bones"]:
    o = next(x for x in OLD["rig"]["bones"] if x["id"] == b["id"])
    if b["id"].endswith("f") and b["id"][:-1] + "n" in NEW_LEN:
        continue
    print("%-14s len %.4f -> %.4f  x %.4f -> %.4f  y %.4f -> %.4f  angle %.3f -> %.3f" % (b["id"], o["lengthM"], b["lengthM"], o["x"], b["x"], o["y"], b["y"], o["angleDeg"], b["angleDeg"]))
for pid in ("standing-alert", "mousing-crouch", "curled-sleeping", "trot-phase-2", "pounce-flight"):
    q = next(p for p in poses if p["id"] == pid)
    st = fk(NEW, q["bones"])
    print(pid, q.get("root"), "toes", " ".join("%s(%.3f,%.3f)" % (k, *tip(st, LEGS[k][4])) for k in LEGS), "nose (%.3f,%.3f)" % tip(st, "head"))
if "--apply" in sys.argv:
    io.put({"dimensions": DIMENSIONS, "rig": NEW, "anchors": anchors, "poses": poses, "clips": clips})
    print("applied")
if "--candidate" in sys.argv:
    doc = json.load(open(io.FILE, encoding="utf-8"))
    doc["asset"].update({"dimensions": DIMENSIONS, "rig": NEW, "anchors": anchors, "poses": poses, "clips": clips})
    json.dump(doc, open("🗑️generated/r3-red-fox/🔣️.json", "w", encoding="utf-8"), ensure_ascii=False)
    print("candidate written")
