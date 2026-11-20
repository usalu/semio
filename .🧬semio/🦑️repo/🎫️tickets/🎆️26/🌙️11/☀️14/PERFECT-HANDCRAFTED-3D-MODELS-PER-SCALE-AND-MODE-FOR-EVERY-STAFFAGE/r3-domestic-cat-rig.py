"""🐈 Round 3 root fix of animals/domestic-cat: the rig from the skeletal landmarks of a 4.2 kg short-haired cat standing alert (world metres, hip joint above x = 0, near side z < 0), the anchors on it and every pose re-solved on the new chain.

Usage: python3 r3-domestic-cat-rig.py [--candidate] [--apply]
Reads the pre-round-3 asset from 🗑️generated/r3-domestic-cat/asset-before.json so that it can be re-run. Landmarks and sources: r3-animals-domestic-cat.md.
"""
import importlib.util
import json
import math
import sys

spec = importlib.util.spec_from_file_location("io", "r3-domestic-cat-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

OUT = "🗑️generated/r3-domestic-cat"
OLD = json.load(open(OUT + "/asset-before.json", encoding="utf-8"))["asset"]
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


def step(p, length, angle):
    return (p[0] + length * dirv(angle)[0], p[1] + length * dirv(angle)[1])


HIP = (0.0, 0.235)
J = {
    "ls": (0.045, 0.250), "tl": (0.178, 0.246), "t7": (0.245, 0.236), "ct": (0.305, 0.208), "poll": (0.337, 0.272),
    "tail0": (-0.034, 0.250),
    "mtp": (-0.0228, 0.016), "mcp": (0.2303, 0.014),
    "shoulder": (0.285, 0.195),
}
J["hock"] = step(J["mtp"], 0.070, 100)
J["toe-h"] = step(J["mtp"], 0.034, -13)
J["wrist"] = step(J["mcp"], 0.040, 100)
J["toe-f"] = step(J["mcp"], 0.030, -11.5)
J["elbow"] = step(J["shoulder"], 0.090, 220)
J["scap"] = step(J["shoulder"], 0.064, 125)
FEMUR, TIBIA = 0.098, 0.104
HEAD_ANGLE, HEAD_LEN = -6.0, 0.0895
TAIL = [(0.064, 176), (0.064, 167), (0.063, 159), (0.059, 156), (0.050, 167)]


def knee(hip, hock, l1, l2, bend):
    d = dist(hip, hock)
    a = D(math.acos((l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    return step(hip, l1, ang(hip, hock) + bend * a)


J["stifle"] = knee(HIP, J["hock"], FEMUR, TIBIA, +1)
J["muzzle"] = step(J["poll"], HEAD_LEN, HEAD_ANGLE)


def head_point(u, v):
    c, s = dirv(HEAD_ANGLE)
    return (J["poll"][0] + u * c - v * s, J["poll"][1] + u * s + v * c)


J["tmj"] = head_point(0.018, -0.008)
J["chin"] = head_point(0.078, -0.029)
J["ear"] = head_point(0.012, 0.034)
J["ear-tip"] = step(J["ear"], 0.055, 82)
tail_pts = [J["tail0"]]
for ln, a in TAIL:
    tail_pts.append(step(tail_pts[-1], ln, a))

CHAIN = [
    ("pelvis", None, HIP, J["ls"]), ("lumbar", "pelvis", J["ls"], J["tl"]), ("thorax-a", "lumbar", J["tl"], J["t7"]), ("thorax-b", "thorax-a", J["t7"], J["ct"]), ("neck", "thorax-b", J["ct"], J["poll"]),
    ("head", "neck", J["poll"], J["muzzle"]), ("jaw", "head", J["tmj"], J["chin"]), ("ear-n", "head", J["ear"], J["ear-tip"]), ("ear-f", "head", J["ear"], J["ear-tip"]),
]
for i in range(5):
    CHAIN.append(("tail-%d" % (i + 1), "pelvis" if i == 0 else "tail-%d" % i, tail_pts[i], tail_pts[i + 1]))
for s in ("f", "n"):
    CHAIN += [
        ("thigh-h" + s, "pelvis", HIP, J["stifle"]), ("shank-h" + s, "thigh-h" + s, J["stifle"], J["hock"]), ("metatarsus-h" + s, "shank-h" + s, J["hock"], J["mtp"]), ("paw-h" + s, "metatarsus-h" + s, J["mtp"], J["toe-h"]),
        ("scapula-f" + s, "thorax-b", J["scap"], J["shoulder"]), ("humerus-f" + s, "scapula-f" + s, J["shoulder"], J["elbow"]), ("forearm-f" + s, "humerus-f" + s, J["elbow"], J["wrist"]),
        ("metacarpus-f" + s, "forearm-f" + s, J["wrist"], J["mcp"]), ("paw-f" + s, "metacarpus-f" + s, J["mcp"], J["toe-f"]),
    ]
WORLD = {b: (a, t, ang(a, t), dist(a, t)) for b, _p, a, t in CHAIN}
PARENT = {b: p for b, p, _a, _t in CHAIN}
LATERAL = {"thigh-h": 0.030, "shank-h": 0.012, "metatarsus-h": -0.008, "scapula-f": 0.022, "humerus-f": 0.018, "metacarpus-f": -0.006, "ear-": 0.024}


def lateral(bone):
    for stem, z in LATERAL.items():
        if bone.startswith(stem) and bone[-1] in "nf":
            return z if bone.endswith("f") else -z
    return None


def to_local(bone, p):
    a, _t, th, _l = WORLD[bone]
    c, s = dirv(th)
    dx, dy = p[0] - a[0], p[1] - a[1]
    return (round(dx * c + dy * s, 4), round(-dx * s + dy * c, 4))


COM = {"pelvis": (0.005, 0.232), "lumbar": (0.112, 0.214), "thorax-a": (0.21, 0.198), "thorax-b": (0.272, 0.196), "neck": (0.322, 0.24), "head": (0.378, 0.27)}


def new_rig():
    bones = []
    for old in OLD["rig"]["bones"]:
        b = {}
        a, _t, th, ln = WORLD[old["id"]]
        p = PARENT[old["id"]]
        for key, value in old.items():
            if key == "x":
                if p is None:
                    b["x"], b["y"] = 0, 0
                else:
                    b["x"], b["y"] = to_local(p, a)
                z = lateral(old["id"])
                if z is not None:
                    b["z"] = z
            elif key == "y":
                continue
            elif key == "angleDeg":
                b["angleDeg"] = round(th if p is None else norm(th - WORLD[p][2]), 3)
            elif key == "lengthM":
                b["lengthM"] = round(ln, 4)
            elif key == "mass":
                m = dict(value)
                if old["id"] in COM:
                    m["comX"], m["comY"] = to_local(old["id"], COM[old["id"]])
                else:
                    m["comX"], m["comY"] = round(ln / 2, 4), 0
                b["mass"] = m
            else:
                b[key] = value
        bones.append(b)
    return {**OLD["rig"], "bones": bones}


def fk(rig, offsets, root=None):
    root = root or {}
    out = {}
    for b in rig["bones"]:
        off = offsets.get(b["id"], {}).get("angleDeg", 0.0)
        if b["parent"] is None:
            th = b["angleDeg"] + off + root.get("angleDeg", 0.0)
            o = (root.get("x", 0.0), root.get("y", 0.0))
        else:
            po, pth, _pl = out[b["parent"]]
            c, s = dirv(pth)
            o = (po[0] + b["x"] * c - b["y"] * s, po[1] + b["x"] * s + b["y"] * c)
            th = pth + b["angleDeg"] + off
        out[b["id"]] = (o, th, b["lengthM"])
    return out


def tip(state, bone):
    o, th, ln = state[bone]
    return step(o, ln, th)


def anchor_at(asset, state, aid):
    a = next(x for x in asset["anchors"] if x["id"] == aid)
    o, th, _l = state[a["bone"]]
    c, s = dirv(th)
    return (o[0] + a["x"] * c - a["y"] * s, o[1] + a["x"] * s + a["y"] * c)


def two_bone(p, q, l1, l2, bend):
    far = dist(p, q) > l1 + l2 - 1e-6
    d = min(max(dist(p, q), abs(l1 - l2) + 1e-6), l1 + l2 - 1e-6)
    a = D(math.acos((l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    th1 = ang(p, q) + bend * a
    mid = step(p, l1, th1)
    q2 = step(p, d, ang(p, q))
    return th1, ang(mid, q2), far


NEW = new_rig()
NEW_LEN = {b["id"]: b["lengthM"] for b in NEW["bones"]}
REST_NEW = fk(NEW, {})
PAD_H = {"h": (0.004, 0.0), "f": (0.019, 0.0)}
CONTACT = {"contact-hn": ("paw-hn", (-0.004, 0.0)), "contact-hf": ("paw-hf", (-0.004, 0.0)), "contact-fn": ("paw-fn", (0.247, 0.0)), "contact-ff": ("paw-ff", (0.247, 0.0))}
ANCHOR = {**CONTACT, "contact-rump": ("pelvis", (-0.034, 0.196)), "nose": ("head", head_point(0.092, -0.009)), "mouth": ("jaw", head_point(0.086, -0.02)), "back-plume": ("thorax-a", (0.205, 0.2635))}
anchors = []
for old in OLD["anchors"]:
    a = dict(old)
    bone, at = ANCHOR[a["id"]]
    a["bone"] = bone
    a["x"], a["y"] = to_local(bone, at)
    anchors.append(a)
NEW_ASSET = {"rig": NEW, "anchors": anchors}

LEGS = {}
for s in ("n", "f"):
    LEGS["h" + s] = (None, "thigh-h" + s, "shank-h" + s, "metatarsus-h" + s, "paw-h" + s, +1, "contact-h" + s)
    LEGS["f" + s] = ("scapula-f" + s, "humerus-f" + s, "forearm-f" + s, "metacarpus-f" + s, "paw-f" + s, -1, "contact-f" + s)
AXIAL = ["pelvis", "lumbar", "thorax-a", "thorax-b", "neck", "head", "jaw", "ear-n", "ear-f", "tail-1", "tail-2", "tail-3", "tail-4", "tail-5"]
OLD_STAND_POSE = next(p for p in OLD["poses"] if p["id"] == "standing-alert")


def grounded(asset, state, contacts):
    """Shifts a state so that the lowest of the given contact anchors lies on y = 0; returns the shift."""
    return -min(anchor_at(asset, state, c)[1] for c in contacts)


OLD_STAND = fk(OLD["rig"], OLD_STAND_POSE["bones"], OLD_STAND_POSE.get("root"))
OLD_STAND_DY = grounded(OLD, OLD_STAND, OLD_STAND_POSE["contacts"])
NEW_STAND_DY = grounded(NEW_ASSET, REST_NEW, ["contact-hn", "contact-fn"])
warn = []


def to_offsets(world, root):
    bones = {}
    for b in NEW["bones"]:
        p = b["parent"]
        rel = norm(world[b["id"]] - (world[p] if p else root.get("angleDeg", 0.0)) - b["angleDeg"])
        if abs(rel) > 5e-3:
            bones[b["id"]] = {"angleDeg": round(rel, 2)}
    return bones


def solve(pid, axial_world, root, legs, hip_height):
    """axial_world: world angle per axial bone; legs[leg] = { toe: (x relative to the proximal bone origin, height above ground), paw / meta / scap: world angles } or { world: { segment: world angle } }; the ground lies `hip_height` below the hip joint."""
    world = {b["id"]: REST_NEW[b["id"]][1] for b in NEW["bones"]}
    world.update(axial_world)
    state = fk(NEW, to_offsets(world, root), root)
    ground = state["pelvis"][0][1] - hip_height
    for leg, want in legs.items():
        scap, up, low, meta, paw, bend, _contact = LEGS[leg]
        if "world" in want:
            for b in ([scap] if scap else []) + [up, low, meta, paw]:
                world[b] = want["world"][b.split("-")[0]]
            continue
        base = state[scap or up][0]
        prox = base
        if scap:
            world[scap] = want["scap"]
            prox = step(base, NEW_LEN[scap], want["scap"])
        toe = (base[0] + want["toe"][0], ground + want["toe"][1])
        hock = step(step(toe, -NEW_LEN[paw], want["paw"]), -NEW_LEN[meta], want["meta"])
        th1, th2, far = two_bone(prox, hock, NEW_LEN[up], NEW_LEN[low], bend)
        if far:
            warn.append("%s %s unreachable by %.3f" % (pid, leg, dist(prox, hock) - NEW_LEN[up] - NEW_LEN[low]))
        world[up], world[low], world[meta], world[paw] = th1, th2, want["meta"], want["paw"]
    offsets = to_offsets(world, root)
    return offsets, fk(NEW, offsets, root)


def old_targets(pose):
    """Per leg of an old pose: the toe relative to the proximal bone origin and to the ground, as a change from the old standing pose, and the changes of the world angles."""
    st = fk(OLD["rig"], pose["bones"], pose.get("root"))
    dy = grounded(OLD, st, pose["contacts"] or ["contact-hn"])
    out = {}
    for leg, (scap, up, _low, meta, paw, _bend, contact) in LEGS.items():
        base, base0 = st[scap or up][0], OLD_STAND[scap or up][0]
        t, t0 = tip(st, paw), tip(OLD_STAND, paw)
        out[leg] = {
            "dx": (t[0] - base[0]) - (t0[0] - base0[0]), "lift": (t[1] + dy) - (t0[1] + OLD_STAND_DY),
            "paw": norm(st[paw][1] - OLD_STAND[paw][1]), "meta": norm(st[meta][1] - OLD_STAND[meta][1]),
            "scap": norm(st[scap][1] - OLD_STAND[scap][1]) if scap else 0.0, "planted": contact in pose["contacts"],
        }
    axial = {b: norm(st[b][1] - OLD_STAND[b][1]) for b in AXIAL}
    return out, axial, (st["pelvis"][0][1] + dy) - (OLD_STAND["pelvis"][0][1] + OLD_STAND_DY)


def stand_toe(leg):
    scap, up, _low, _meta, paw, _bend, _contact = LEGS[leg]
    base = REST_NEW[scap or up][0]
    t = tip(REST_NEW, paw)
    return (t[0] - base[0], t[1] + NEW_STAND_DY)


STAGGER = {"hn": 0.0, "hf": 0.036, "fn": 0.0, "ff": -0.03}


def convert(pose, tweak=None):
    """An old pose carried to the new rig: axial bones keep their change from standing, planted paws stand flat on the ground, lifted paws keep their lift and angles, toes keep their travel, the hip keeps its change of height."""
    targets, axial, dh = old_targets(pose)
    tweak = tweak or {}
    root = {k: v for k, v in (pose.get("root") or {}).items() if k != "y"}
    axial_world = {b: REST_NEW[b][1] + axial[b] + tweak.get(b, 0.0) for b in AXIAL}
    legs = {}
    for leg, t in targets.items():
        scap, _up, _low, meta, paw, _bend, _contact = LEGS[leg]
        sx, sy = stand_toe(leg)
        legs[leg] = {
            "toe": (sx + tweak.get("reach", 1.0) * t["dx"] + tweak.get("dx-" + leg, 0.0), sy + (tweak.get("lift-" + leg, 0.0) if t["planted"] else max(0.004, t["lift"]))),
            "paw": REST_NEW[paw][1] + tweak.get("paw-" + leg, 0.0 if t["planted"] else t["paw"]),
            "meta": REST_NEW[meta][1] + tweak.get("meta-" + leg, t["meta"] * tweak.get("meta", 1.0)),
        }
        if scap:
            legs[leg]["scap"] = REST_NEW[scap][1] + t["scap"] + tweak.get("scap", 0.0)
    return solve(pose["id"], axial_world, root, legs, NEW_STAND_DY + tweak.get("dh", dh))


def standing():
    legs = {}
    for leg in LEGS:
        scap, _up, _low, meta, paw, _bend, _contact = LEGS[leg]
        sx, sy = stand_toe(leg)
        legs[leg] = {"toe": (sx + STAGGER[leg], sy), "paw": REST_NEW[paw][1], "meta": REST_NEW[meta][1] + (4.0 if leg == "hf" else -3.0 if leg == "ff" else 0.0)}
        if scap:
            legs[leg]["scap"] = REST_NEW[scap][1]
    return solve("standing-alert", {b: REST_NEW[b][1] for b in AXIAL}, {}, legs, NEW_STAND_DY)


TWEAK = {"sitting-tail-wrapped": {"dh": -0.184, "scap": -8.0, "tail-1": 13.0, "tail-2": -11.0, "tail-3": -9.0, "tail-4": -8.0, "tail-5": -22.0}}
poses = []
states = {}
for pose in OLD["poses"]:
    p = json.loads(json.dumps(pose))
    pid = p["id"]
    if pid == "standing-alert":
        p["bones"], states[pid] = standing()
    else:
        p["bones"], states[pid] = convert(pose, TWEAK.get(pid))
    poses.append(p)

limits = {}
for p in poses:
    for b, off in p["bones"].items():
        lo, hi = limits.get(b, (0.0, 0.0))
        limits[b] = (min(lo, off["angleDeg"]), max(hi, off["angleDeg"]))
for b in NEW["bones"]:
    lo, hi = limits.get(b["id"], (0.0, 0.0))
    j = dict(b["joint"])
    new_lo, new_hi = min(j["minDeg"], math.floor((lo - 5) / 5) * 5), max(j["maxDeg"], math.ceil((hi + 5) / 5) * 5)
    if (new_lo, new_hi) != (j["minDeg"], j["maxDeg"]):
        print("LIMIT %s %s..%s -> %s..%s (poses use %.1f..%.1f)" % (b["id"], j["minDeg"], j["maxDeg"], new_lo, new_hi, lo, hi))
        j["minDeg"], j["maxDeg"] = new_lo, new_hi
        b["joint"] = j

for w in warn:
    print("WARN", w)
print("landmarks", {k: (round(v[0], 4), round(v[1], 4)) for k, v in J.items()})
print("tail tip", tail_pts[-1])
for pid, st in states.items():
    pose = next(p for p in poses if p["id"] == pid)
    dy = grounded(NEW_ASSET, st, pose["contacts"])
    print(pid.ljust(22), "hip %.3f" % (st["pelvis"][0][1] + dy), " ".join("%s(%.3f,%.3f)" % (c[8:], anchor_at(NEW_ASSET, st, c)[0], anchor_at(NEW_ASSET, st, c)[1] + dy) for c in CONTACT), "nose (%.3f,%.3f)" % (tip(st, "head")[0], tip(st, "head")[1] + dy), "rump %.3f" % (anchor_at(NEW_ASSET, st, "contact-rump")[1] + dy), "tail tip %.3f" % (tip(st, "tail-5")[1] + dy), "planted", ",".join(c[8:] for c in pose["contacts"]))
materials = [m for m in OLD["materials"] if m["id"] != "bone"] + [{"id": "bone", "title": {"en": "Bone", "de": "Knochen"}, "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1900, "source": "authored-estimate"}]
members = {"materials": materials, "rig": NEW, "anchors": anchors, "poses": poses}
json.dump(members, open(OUT + "/rig-members.json", "w", encoding="utf-8"), ensure_ascii=False)
if "--candidate" in sys.argv:
    io.candidate(members, OUT + "/🔣️.json")
    print("candidate written")
if "--apply" in sys.argv:
    io.put(members)
    print("applied")
