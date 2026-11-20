"""🦌 Round 3 root fix of animals/roe-deer: the rig from the skeletal landmarks of a 25 kg roe buck (standing, world metres, hip joint above x = 0), the anchors on it, every pose and every clip key re-solved on the new chain (hooves keep their ground contacts and stride), the note that called the buck a doe.

Usage: python3 r3-roe-deer-rig.py [--candidate | --apply]   (reads the pre-round-3 asset from 🗑️generated/r3-roe-deer/asset-before.json so that it can be re-run)
Landmarks and sources: r3-animals-roe-deer.md.
"""
import importlib.util
import json
import math
import sys

spec = importlib.util.spec_from_file_location("io", "r3-roe-deer-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

OLD = json.load(open("🗑️generated/r3-roe-deer/asset-before.json", encoding="utf-8"))["asset"]
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


def polar(a, angle, length):
    return (a[0] + length * dirv(angle)[0], a[1] + length * dirv(angle)[1])


HIP = (0.0, 0.660)
J = {
    "ls": (0.090, 0.712), "tl": (0.280, 0.706), "ct": (0.560, 0.612),
    "stifle": (0.092, 0.500), "hock": (-0.020, 0.305), "fet-h": (0.000, 0.076), "toe-h": (0.052, 0.0),
    "scap": (0.505, 0.705), "shoulder": (0.605, 0.575), "elbow": (0.535, 0.440), "carpus": (0.545, 0.245), "fet-f": (0.548, 0.076), "toe-f": (0.600, 0.0),
}
NECK_ANGLE, NECK_LEN = 52.0, 0.310
HEAD_ANGLE, HEAD_LEN = -30.0, 0.220
J["poll"] = polar(J["ct"], NECK_ANGLE, NECK_LEN)
J["muzzle"] = polar(J["poll"], HEAD_ANGLE, HEAD_LEN)


def head_point(u, v):
    c, s = dirv(HEAD_ANGLE)
    return (J["poll"][0] + u * c - v * s, J["poll"][1] + u * s + v * c)


J["tmj"] = head_point(0.036, -0.004)
J["incisor"] = head_point(0.205, -0.034)
J["ear"] = head_point(0.012, 0.047)
J["ear-tip"] = polar(J["ear"], 100.0, 0.135)

CHAIN = [
    ("pelvis", None, HIP, J["ls"]), ("lumbar", "pelvis", J["ls"], J["tl"]), ("thorax", "lumbar", J["tl"], J["ct"]), ("neck", "thorax", J["ct"], J["poll"]),
    ("head", "neck", J["poll"], J["muzzle"]), ("jaw", "head", J["tmj"], J["incisor"]), ("ear-n", "head", J["ear"], J["ear-tip"]), ("ear-f", "head", J["ear"], J["ear-tip"]),
]
for s in ("f", "n"):
    CHAIN += [
        ("thigh-h" + s, "pelvis", HIP, J["stifle"]), ("shank-h" + s, "thigh-h" + s, J["stifle"], J["hock"]), ("cannon-h" + s, "shank-h" + s, J["hock"], J["fet-h"]), ("hoof-h" + s, "cannon-h" + s, J["fet-h"], J["toe-h"]),
        ("scapula-f" + s, "thorax", J["scap"], J["shoulder"]), ("humerus-f" + s, "scapula-f" + s, J["shoulder"], J["elbow"]), ("forearm-f" + s, "humerus-f" + s, J["elbow"], J["carpus"]),
        ("cannon-f" + s, "forearm-f" + s, J["carpus"], J["fet-f"]), ("hoof-f" + s, "cannon-f" + s, J["fet-f"], J["toe-f"]),
    ]
WORLD = {b: (a, t, ang(a, t), dist(a, t)) for b, _p, a, t in CHAIN}
PARENT = {b: p for b, p, _a, _t in CHAIN}


def to_local(bone, p):
    a, _t, th, _l = WORLD[bone]
    c, s = dirv(th)
    dx, dy = p[0] - a[0], p[1] - a[1]
    return (round(dx * c + dy * s, 4), round(-dx * s + dy * c, 4))


LIMITS = {"neck": (-150, 45), "head": (-60, 75), "ear-n": (-75, 60), "ear-f": (-75, 60), "thigh-hn": (-60, 65), "thigh-hf": (-60, 65), "humerus-fn": (-55, 90), "humerus-ff": (-55, 90), "forearm-fn": (-40, 120), "forearm-ff": (-40, 120)}
COM = {"pelvis": (0.015, 0.655), "lumbar": (0.185, 0.640), "thorax": (0.425, 0.600), "neck": (0.650, 0.725), "head": head_point(0.095, 0.004), "jaw": head_point(0.11, -0.03)}


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


def two_bone(p, q, l1, l2, bend):
    d = min(max(dist(p, q), abs(l1 - l2) + 1e-6), l1 + l2 - 1e-6)
    base = ang(p, q)
    a = D(math.acos((l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    th1 = base + bend * a
    mid = (p[0] + l1 * dirv(th1)[0], p[1] + l1 * dirv(th1)[1])
    return th1, ang(mid, q), dist(p, q) - l1 - l2


NEW = new_rig()
NEW_LEN = {b["id"]: b["lengthM"] for b in NEW["bones"]}
NEW_REL = {b["id"]: b["angleDeg"] for b in NEW["bones"]}
REST_OLD = fk(OLD["rig"], {})
REST_NEW = fk(NEW, {})
OLD_ANCHOR = {a["bone"]: (a["x"], a["y"]) for a in OLD["anchors"] if a["kind"] == "contact"}
SOLE = to_local("hoof-hn", (J["toe-h"][0] - 0.021, 0.0))
SOLE_F = to_local("hoof-fn", (J["toe-f"][0] - 0.021, 0.0))


def at(state, bone, local):
    o, th, _l = state[bone]
    c, s = dirv(th)
    return (o[0] + local[0] * c - local[1] * s, o[1] + local[0] * s + local[1] * c)


def sole_of(hoof):
    return SOLE if hoof.startswith("hoof-h") else SOLE_F


OLD_HIP = -min(at(REST_OLD, "hoof-hn", OLD_ANCHOR["hoof-hn"])[1], at(REST_OLD, "hoof-fn", OLD_ANCHOR["hoof-fn"])[1])
LEGS = {}
for s in ("n", "f"):
    LEGS["h" + s] = (None, "thigh-h" + s, "shank-h" + s, "cannon-h" + s, "hoof-h" + s, +1)
    LEGS["f" + s] = ("scapula-f" + s, "humerus-f" + s, "forearm-f" + s, "cannon-f" + s, "hoof-f" + s, -1)
TRUNK = ["pelvis", "lumbar", "thorax", "neck", "head", "jaw", "ear-n", "ear-f"]
OLD_ALERT = {"neck": -8.0, "head": -4.0, "ear-n": -24.0, "ear-f": -30.0}
NEW_ALERT = {"neck": 10.0, "head": 8.0, "ear-n": -34.0, "ear-f": -40.0}
warn = []


def old_targets(bones, root_angle):
    """Per leg of an old pose: toe shift from the old rest toe (x relative to the hip, y above the old ground), world angle deltas of hoof, cannon and scapula from rest; and the hip height above the old ground."""
    st = fk(OLD["rig"], bones, root_angle)
    ground = min(at(st, LEGS[k][4], OLD_ANCHOR[LEGS[k][4]])[1] for k in LEGS)
    legs = {}
    for leg, (scap, _up, _low, cannon, hoof, _bend) in LEGS.items():
        t, t0 = at(st, hoof, OLD_ANCHOR[hoof]), at(REST_OLD, hoof, OLD_ANCHOR[hoof])
        legs[leg] = {"dx": t[0] - t0[0], "h": t[1] - ground, "hoof": norm(st[hoof][1] - REST_OLD[hoof][1]), "cannon": norm(st[cannon][1] - REST_OLD[cannon][1])}
        if scap:
            legs[leg]["scap"] = bones.get(scap, {}).get("angleDeg", 0.0)
    return legs, -ground


def solve(pid, trunk_off, legs, hip_h, root_angle, full=False):
    st = fk(NEW, {b: {"angleDeg": v} for b, v in trunk_off.items()}, root_angle)
    world = {b: st[b][1] for b in TRUNK}
    for leg, want in legs.items():
        scap, up, low, cannon, hoof, bend = LEGS[leg]
        toe0 = at(REST_NEW, hoof, sole_of(hoof))
        toe = (toe0[0] + want["dx"], want["h"] - hip_h)
        ha = REST_NEW[hoof][1] + want["hoof"]
        ca = REST_NEW[cannon][1] + want["cannon"]
        lx, ly = sole_of(hoof)
        fet = (toe[0] - lx * dirv(ha)[0] + ly * dirv(ha)[1], toe[1] - lx * dirv(ha)[1] - ly * dirv(ha)[0])
        knee = polar(fet, ca, -NEW_LEN[cannon])
        prox = (0.0, 0.0)
        if scap:
            sa = st["thorax"][1] + NEW_REL[scap] + want.get("scap", 0.0)
            world[scap] = sa
            prox = polar(st[scap][0], sa, NEW_LEN[scap])
        th1, th2, over = two_bone(prox, knee, NEW_LEN[up], NEW_LEN[low], bend)
        if over > 1e-4:
            warn.append("%s %s unreachable by %.3f" % (pid, leg, over))
        world[up], world[low], world[cannon], world[hoof] = th1, th2, ca, ha
    bones = {}
    for b in NEW["bones"]:
        p = b["parent"]
        rel = norm(world[b["id"]] - (world[p] if p else root_angle) - b["angleDeg"])
        lim = b["joint"]
        if not lim["minDeg"] - 1e-6 <= rel <= lim["maxDeg"] + 1e-6:
            warn.append("%s %s offset %.1f outside %s..%s" % (pid, b["id"], rel, lim["minDeg"], lim["maxDeg"]))
        if full or abs(rel) > 5e-4:
            bones[b["id"]] = {"angleDeg": round(rel, 4) + 0.0}
    return bones


def trunk_of(bones, family):
    out = {b: bones.get(b, {}).get("angleDeg", 0.0) for b in TRUNK}
    if family == "alert":
        for b, v in NEW_ALERT.items():
            out[b] = v + (out[b] - OLD_ALERT[b])
    return out


GRAZE_ROOT = -9.0
STANCE = {"hn": 0.0, "hf": -0.035, "fn": 0.0, "ff": 0.035}


def graze_trunk(old_bones, hip_h):
    best = None
    base = trunk_of(old_bones, None)
    for k in range(-1500, -600):
        neck_off = k / 10.0
        neck_w = GRAZE_ROOT + REST_NEW["neck"][1] + base["lumbar"] + base["thorax"] + neck_off
        head_w = -78.0
        trunk = {**base, "neck": neck_off, "head": norm(head_w - neck_w - NEW_REL["head"])}
        st = fk(NEW, {b: {"angleDeg": v} for b, v in trunk.items()}, GRAZE_ROOT)
        miss = abs(tip(st, "head")[1] + hip_h - 0.035)
        if best is None or miss < best[0]:
            best = (miss, trunk)
    if best[0] > 0.002:
        warn.append("grazing muzzle misses the ground by %.3f" % best[0])
    return best[1]


def family_of(pid):
    return "alert" if pid.startswith("alert") else "grazing" if pid == "grazing" else None


def convert(pid, bones, root, family, full=False):
    root_angle = (root or {}).get("angleDeg", 0.0)
    legs, old_hip = old_targets(bones, root_angle)
    hip_h = old_hip + HIP[1] - OLD_HIP
    if family == "grazing":
        hip_h = HIP[1] - 0.012
        legs["fn"].update({"dx": 0.19, "cannon": 21.0, "hoof": 10.0})
        legs["ff"].update({"dx": -0.11, "cannon": -13.0, "hoof": -6.0})
        return solve(pid, graze_trunk(bones, hip_h), legs, hip_h, GRAZE_ROOT, full), {"angleDeg": GRAZE_ROOT}
    if family == "alert":
        hip_h = HIP[1] - 0.003
        for leg, dx in STANCE.items():
            if legs[leg]["h"] < 1e-3:
                legs[leg].update({"dx": dx, "cannon": 0.0, "hoof": 0.0})
    return solve(pid, trunk_of(bones, family), legs, hip_h, root_angle, full), root


poses = []
OLD_POSE = {p["id"]: p for p in OLD["poses"]}
for pose in OLD["poses"]:
    p = json.loads(json.dumps(pose))
    p["bones"], root = convert(p["id"], pose["bones"], pose.get("root"), family_of(p["id"]))
    if root is not None:
        p["root"] = root
    poses.append(p)
NEW_POSE = {p["id"]: p for p in poses}

clips = []
for clip in OLD["clips"]:
    c = json.loads(json.dumps(clip))
    for key in c["keys"]:
        if "bones" not in key:
            continue
        fam = family_of(key["pose"])
        if len(key["bones"]) < 20:
            for b, v in key["bones"].items():
                if fam == "alert" and b in NEW_ALERT:
                    v["angleDeg"] = round(NEW_ALERT[b] + v["angleDeg"] - OLD_ALERT[b], 4)
            continue
        key["bones"], _root = convert("%s@%s" % (c["id"], key["t"]), key["bones"], OLD_POSE[key["pose"]].get("root"), fam, full=True)
    clips.append(c)


def sole(toe):
    return (toe[0] - 0.021, 0.0)


ANCHOR = {
    "contact-hn": ("hoof-hn", sole(J["toe-h"])), "contact-hf": ("hoof-hf", sole(J["toe-h"])), "contact-fn": ("hoof-fn", sole(J["toe-f"])), "contact-ff": ("hoof-ff", sole(J["toe-f"])),
    "nose": ("head", head_point(0.221, 0.004)), "eye-anchor": ("head", head_point(0.092, 0.03)), "back-plume": ("thorax", (0.36, 0.757)),
}
FOOTPRINT = {"contact-hn": (0.045, 0.032), "contact-hf": (0.045, 0.032), "contact-fn": (0.048, 0.035), "contact-ff": (0.048, 0.035)}
anchors = []
for old in OLD["anchors"]:
    a = dict(old)
    bone, where = ANCHOR[a["id"]]
    a["bone"] = bone
    a["x"], a["y"] = to_local(bone, where)
    if a["id"] in FOOTPRINT:
        a["footprintLengthM"], a["footprintWidthM"] = FOOTPRINT[a["id"]]
    anchors.append(a)

provenance = json.loads(json.dumps(OLD["provenance"]))
provenance["notes"] = {
    "en": "Side elevation of the near side of a roe buck walking to the right, head raised in the default pose; the far-side legs are separate chains drawn darker and behind. Skeletal landmarks, bone lengths and body proportions were re-authored in round 3 from the body size ranges of Andersen et al. (1998) and comparative ruminant osteology; they are authored estimates, not measurements of one animal.",
    "de": "Seitenansicht der nahen Seite eines nach rechts gehenden Rehbocks, in der Standardpose mit erhobenem Kopf; die Läufe der fernen Seite sind eigene Ketten, dunkler und dahinter gezeichnet. Skelettpunkte, Knochenlängen und Körperproportionen wurden in Runde 3 nach den Körpermaßen bei Andersen et al. (1998) und der vergleichenden Osteologie der Wiederkäuer neu gesetzt; es sind Schätzwerte, keine Messungen an einem Tier.",
}

for w in warn:
    print("WARN", w)
print("bones", " ".join("%s %.4f" % (b["id"], b["lengthM"]) for b in NEW["bones"][:17]))
for pid in ("alert", "grazing", "alert-foreleg-raised", "bounding-gathered", "bounding-pushoff", "walk-phase-3", "bound-phase-6"):
    ps = NEW_POSE[pid]
    st = fk(NEW, ps["bones"], (ps.get("root") or {}).get("angleDeg", 0.0))
    g = min(at(st, LEGS[k][4], sole_of(LEGS[k][4]))[1] for k in LEGS)
    print(pid, "hip %.3f soles" % -g, " ".join("%s(%.3f,%.3f)" % (k, at(st, LEGS[k][4], sole_of(LEGS[k][4]))[0], at(st, LEGS[k][4], sole_of(LEGS[k][4]))[1] - g) for k in LEGS), "muzzle (%.3f,%.3f) poll (%.3f,%.3f)" % (tip(st, "head")[0], tip(st, "head")[1] - g, st["head"][0][0], st["head"][0][1] - g))
materials = [m for m in OLD["materials"] if m["id"] != "bone"] + [{"id": "bone", "title": {"en": "Bone", "de": "Knochen"}, "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1900, "source": "authored-estimate"}]
members = {"provenance": provenance, "materials": materials, "rig": NEW, "anchors": anchors, "poses": poses, "clips": clips}
if "--candidate" in sys.argv:
    io.candidate(members, fresh="--fresh" in sys.argv)
    print("candidate written")
if "--apply" in sys.argv:
    io.put(members)
    print("applied")
