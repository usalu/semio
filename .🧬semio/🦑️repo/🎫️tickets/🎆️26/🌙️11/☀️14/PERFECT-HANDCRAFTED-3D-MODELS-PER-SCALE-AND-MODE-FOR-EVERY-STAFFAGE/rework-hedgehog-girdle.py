"""🦔️ Rework of `animals/hedgehog`: shoulder and pelvic girdle on lateral rest placement, real joint limits, the skeleton solids that hang on them.

Usage: python3 rework-hedgehog-girdle.py p1|p2|build|publish
  p1       candidate with the four girdle bones as placeholders, prints their solved rest placement (rig-rest.ts --solve)
  p2       candidate with the solved girdle bones, prints the limbs re-parented without moving
  build    full candidate (rig and volumes) at 🗑️generated/rework-hedgehog/hedgehog/🔣️.json
  publish  writes the candidate's rig into the catalog file (re-read right before), the volumes into the source, and splices them
Input: rework-hedgehog-before.json, the rig and the volumes of the asset before this rework (the script is repeatable). Every number below is hand-chosen; world rest coordinates are [ahead, up, left] with the rig root at the origin, near side z < 0.
"""
import copy
import glob
import json
import math
import os
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
FILE = os.path.join(T, "../../../../../../..", "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/hedgehog/🔣️.json")
FILE = os.path.normpath(FILE)
SOURCE = os.path.join(T, "volumes/animals/hedgehog.json")
BEFORE = os.path.join(T, "rework-hedgehog-before.json")
OUT = os.path.join(T, "🗑️generated/rework-hedgehog/hedgehog")
CANDIDATE = os.path.join(OUT, "🔣️.json")
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]

SIDES = (("n", -1.0), ("f", 1.0))
SCAPULA_AT = (0.100, 0.019, 0.020)
SCAPULA_TIP = (0.118, -0.008, 0.028)
ILIUM_AT = (0.014, 0.011, 0.009)
ILIUM_TIP = (0.0, -0.010, 0.024)
LIMITS = {
    "chest": (-75, 15), "neck": (-90, 35), "head": (-50, 30),
    "fore": (-40, 60), "forearm": (-45, 70), "forepaw": (-45, 160),
    "hind": (-55, 45), "shank": (-65, 40), "hindpaw": (-50, 25),
}
GIRDLE = {
    "scapula-n": {"x": 0.03342, "y": 0.0068, "z": -0.02, "angleDeg": -39.7499, "abductionDeg": -13.8492},
    "scapula-f": {"x": 0.03342, "y": 0.0068, "z": 0.02, "angleDeg": -39.7499, "abductionDeg": 13.8492},
    "ilium-n": {"x": 0.01676, "y": 0.00601, "z": -0.009, "angleDeg": -142.1201, "abductionDeg": -30.7239},
    "ilium-f": {"x": 0.01676, "y": 0.00601, "z": 0.009, "angleDeg": -142.1201, "abductionDeg": 30.7239},
}
LIMBS = {
    "fore-n": {"parent": "scapula-n", "x": 0.03341, "y": 0.00006, "z": 0, "angleDeg": -88.6019, "abductionDeg": 0.3446, "twistDeg": -13.845},
    "fore-f": {"parent": "scapula-f", "x": 0.03341, "y": 0.00006, "z": 0, "angleDeg": -88.6019, "abductionDeg": -0.3446, "twistDeg": 13.845},
    "hind-n": {"parent": "ilium-n", "x": 0.02939, "y": -0.00001, "z": 0.00002, "angleDeg": 80.5572, "abductionDeg": 5.5691, "twistDeg": 30.2636},
    "hind-f": {"parent": "ilium-f", "x": 0.02939, "y": -0.00001, "z": -0.00002, "angleDeg": 80.5572, "abductionDeg": -5.5691, "twistDeg": -30.2636},
}


def sub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    l = math.sqrt(dot(a, a))
    return [a[0] / l, a[1] / l, a[2] / l]


def side(p, s):
    return [p[0], p[1], p[2] * s]


def lateral_frame(at, tip):
    """🧭️ Frame of a girdle bone as rig-rest.ts solves it: u joint to tip, w = world left made perpendicular to u, v = w x u."""
    u = unit(sub(tip, at))
    k = u[2]
    w = unit([-k * u[0], -k * u[1], 1 - k * u[2]])
    return {"o": list(at), "u": u, "v": cross(w, u), "w": w}


def sagittal_frame(o, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return {"o": list(o), "u": [c, s, 0], "v": [-s, c, 0], "w": [0, 0, 1]}


TRUNK = sagittal_frame((0, 0, 0), 18.43)
CHEST = sagittal_frame((0.0696 * math.cos(math.radians(18.43)), 0.0696 * math.sin(math.radians(18.43)), 0), 18.43 - 34.99)


def local(f, p):
    d = sub(p, f["o"])
    return [round(dot(d, f["u"]), 5), round(dot(d, f["v"]), 5), round(dot(d, f["w"]), 5)]


def world(f, q):
    return [f["o"][i] + q[0] * f["u"][i] + q[1] * f["v"][i] + q[2] * f["w"][i] for i in range(3)]


def quaternion(x, y):
    """🌀️ [x, y, z, w] of the rotation whose columns are x, y and x cross y."""
    z = cross(x, y)
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [round(v, 5) for v in q]


IDENTITY = {"o": [0, 0, 0], "u": [1, 0, 0], "v": [0, 1, 0], "w": [0, 0, 1]}


def member(id, bone, sections, **flags):
    return {"id": id, "kind": "loft", "bone": bone, "material": "bone", "role": "body", "tone": 0.9, "sections": sections, "capStart": True, "capEnd": True, "member": bone, "part": "member", **flags}


def along(rows, e=2):
    """🦴️ Sections of a loft along the bone's own axis: rows [x, v offset, w offset, half extent along v, half extent along w]."""
    return [{"at": [r[0], r[1], r[2]], "rx": r[3], "ry": r[4], "exponent": e, "rotation": [0.5, 0.5, 0.5, 0.5]} for r in rows]


def rod(f, points, radii):
    """🪢️ Round rod through world points, written in the frame f."""
    out = []
    for i, p in enumerate(points):
        d = unit(sub(points[min(i + 1, len(points) - 1)], points[max(i - 1, 0)]))
        a = cross(d, [0, 1, 0])
        a = unit(a if dot(a, a) > 0.04 else cross(d, [0, 0, 1]))
        b = cross(d, a)
        la, lb = [dot(a, f[k]) for k in "uvw"], [dot(b, f[k]) for k in "uvw"]
        r = radii[i] if isinstance(radii, (list, tuple)) else radii
        out.append({"at": local(f, p), "rx": r, "ry": r, "exponent": 2, "rotation": quaternion(la, lb)})
    return out


def run(args):
    r = subprocess.run([BUN, "rig-rest.ts", *args], cwd=T, capture_output=True, text=True)
    return (r.stdout + r.stderr).strip()


def bone(doc, id):
    return next(b for b in doc["asset"]["rig"]["bones"] if b["id"] == id)


def rig(doc, phase):
    bones = doc["asset"]["rig"]["bones"]
    titles = {
        "scapula": {"en": "Shoulder blade, {s} side: lies on the rib cage from its vertebral border down and forward to the shoulder joint", "de": "Schulterblatt, {t} Seite: liegt auf dem Brustkorb vom Wirbelrand nach vorn unten zum Schultergelenk"},
        "ilium": {"en": "Hip bone, {s} side: from the sacroiliac joint down, back and out to the hip joint", "de": "Hüftbein, {t} Seite: vom Kreuzdarmbeingelenk nach hinten unten außen zum Hüftgelenk"},
    }
    for name, s in SIDES:
        words = {"s": "near" if s < 0 else "far", "t": "nahe" if s < 0 else "ferne"}
        for kind, parent, at, tip, limb, limit in (("scapula", "chest", SCAPULA_AT, SCAPULA_TIP, "fore", 15), ("ilium", "trunk", ILIUM_AT, ILIUM_TIP, "hind", 3)):
            id = f"{kind}-{name}"
            a, t = side(at, s), side(tip, s)
            new = {"id": id, "parent": parent, "title": {k: v.format(**words) for k, v in titles[kind].items()}, "x": 0, "y": 0, "z": 0, "angleDeg": 0, "lengthM": round(math.sqrt(dot(sub(t, a), sub(t, a))), 5), "joint": {"type": "revolute", "minDeg": -limit, "maxDeg": limit}}
            if id in GIRDLE:
                new.update(GIRDLE[id])
            bones.insert(next(i for i, b in enumerate(bones) if b["id"] == f"{limb}-{name}"), new)
            if phase == "build":
                bone(doc, f"{limb}-{name}").update(LIMBS[f"{limb}-{name}"])
    if phase == "build":
        for b in bones:
            key = b["id"].rsplit("-", 1)[0] if b["id"][-2:] in ("-n", "-f") else b["id"]
            if key in LIMITS:
                b["joint"]["minDeg"], b["joint"]["maxDeg"] = LIMITS[key]
        for pose in doc["asset"]["poses"]:
            for id, offset in pose["bones"].items():
                j = bone(doc, id)["joint"]
                assert j["minDeg"] <= offset.get("angleDeg", 0) <= j["maxDeg"], (pose["id"], id, offset)


def ribs(solids):
    """🫁️ The cranial ribs narrow towards the thoracic inlet so that the shoulder blades lie outside the rib cage."""
    for s in solids:
        if s["id"].startswith("rib-") and s["bone"] == "chest":
            x = s["sections"][0]["at"][0]
            f = max(0.5, min(1.0, 1.0 - 0.5 * (x - 0.036) / 0.036))
            for q in s["sections"]:
                q["at"][2] = round(q["at"][2] * f, 5)


def girdle50(st):
    drop = {f"{p}-{n}" for p in ("scapula-spine", "scapula-cranial", "scapula-caudal", "scapula-vertebral", "ilium") for n in "nf"}
    st[:] = [s for s in st if s["id"] not in drop]
    by = {s["id"]: s for s in st}
    by["sacrum"]["sections"] = [
        {"at": [x, y, 0], "rx": rx, "ry": ry, "exponent": 2, "rotation": [0.70711, 0, 0.70711, 0]}
        for x, y, rx, ry in ((0.026, 0.0115, 0.0055, 0.004), (0.0175, 0.0085, 0.0112, 0.0042), (0.008, 0.009, 0.0085, 0.0038), (-0.002, 0.009, 0.005, 0.0038))
    ]
    lumbar = by["lumbar"]["sections"]
    lumbar[:2] = [dict(lumbar[1], at=[0.0245, 0.0122, 0])]
    caudal = by["caudal"]["sections"]
    caudal[:0] = [dict(caudal[0], at=[-0.002, 0.009, 0]), dict(caudal[0], at=[-0.016, 0.0096, 0])]
    at = next(i for i, s in enumerate(st) if s["id"] == "humerus-n")
    new = []
    for name, s in SIDES:
        sc = lateral_frame(side(SCAPULA_AT, s), side(SCAPULA_TIP, s))
        il = lateral_frame(side(ILIUM_AT, s), side(ILIUM_TIP, s))
        lat = s
        new.append(member(f"scapula-blade-{name}", f"scapula-{name}", along(((-0.0015, 0.0005, 0, 0.007, 0.0004), (0.004, 0.0005, 0, 0.0085, 0.0004), (0.012, 0.0003, 0, 0.007, 0.0004), (0.021, 0, 0, 0.0042, 0.0004), (0.0275, 0, 0, 0.0026, 0.0004), (0.0325, 0, 0, 0.003, 0.0004)), 4), sheet=True))
        ridge = [world(sc, q) for q in ((0.003, 0.001, lat * 0.0011), (0.016, 0.0008, lat * 0.0026), (0.027, 0.0005, lat * 0.0036), (0.0335, 0.004, lat * 0.0042))]
        new.append(member(f"scapula-spine-{name}", f"scapula-{name}", rod(sc, ridge, 0.0007), slender=True))
        sternal = world(CHEST, (0.0805, -0.0232, lat * 0.0012))
        middle = [(ridge[3][i] + sternal[i]) / 2 + (0.004, 0.001, 0)[i] for i in range(3)]
        clavicle = member(f"clavicle-{name}", "chest", rod(CHEST, [ridge[3], middle, sternal], 0.00075), slender=True)
        new.append(clavicle)
        new.append(member(f"ilium-{name}", f"ilium-{name}", along(((-0.0105, 0, 0, 0.0026, 0.001), (-0.003, 0, 0, 0.0032, 0.0012), (0.006, 0, 0, 0.0028, 0.0014), (0.015, 0, 0, 0.0022, 0.0018), (0.0235, 0, 0, 0.0028, 0.0024), (0.0294, 0, 0, 0.0034, 0.003))), slender=True))
        ischium = [side(p, -s) for p in ((-0.002, -0.0105, -0.0232), (-0.011, -0.0105, -0.02), (-0.02, -0.012, -0.017))]
        new.append(member(f"ischium-{name}", f"ilium-{name}", rod(il, ischium, (0.0019, 0.0015, 0.0019)), slender=True))
        pubis = [side(p, -s) for p in ((-0.001, -0.012, -0.0225), (-0.006, -0.019, -0.012), (-0.011, -0.0225, -0.0012), (-0.017, -0.018, -0.01), (-0.02, -0.0125, -0.016))]
        new.append(member(f"pubis-{name}", f"ilium-{name}", rod(il, pubis, 0.0013), slender=True))
    st[at:at] = new


def girdle_coarse(st):
    st[:] = [s for s in st if not s["id"].startswith("scapula-spine-")]
    at = next(i for i, s in enumerate(st) if s["id"] == "humerus-n")
    new = []
    for name, s in SIDES:
        ls = round(math.sqrt(dot(sub(SCAPULA_TIP, SCAPULA_AT), sub(SCAPULA_TIP, SCAPULA_AT))), 5)
        li = round(math.sqrt(dot(sub(ILIUM_TIP, ILIUM_AT), sub(ILIUM_TIP, ILIUM_AT))), 5)
        new.append(member(f"scapula-spine-{name}", f"scapula-{name}", along(((0, 0, 0, 0.0008, 0.0008), (ls / 2, 0, 0, 0.0008, 0.0008), (ls, 0, 0, 0.0008, 0.0008))), slender=True))
        new.append(member(f"ilium-{name}", f"ilium-{name}", along(((-0.006, 0, 0, 0.00085, 0.00085), (li / 2, 0, 0, 0.00085, 0.00085), (li, 0, 0, 0.00085, 0.00085))), slender=True))
    st[at:at] = new


TUFT100 = {"keep": 3, "r0": 0.0078, "r1": 0.0012, "sink": 0.004}
HEAD100 = {"eye": 9.2 / 7.6, "ear": 9.2 / 7.8}


def body100(vol):
    """🌵️ 1:100 (15 mm): two of every three fascicles of each ring, staggered from ring to ring, restated as one tuft cone of 15.6 mm base that keeps the root patch, the lean and the tip of the fascicle it stands for; eye and ear at the thinnest stroke of the head drawing."""
    body = vol["body"]
    rings = {}
    for s in body:
        if s["id"].startswith("fascicle-"):
            group, ring, index = s["id"].split("-")[1:]
            rings.setdefault(group, sorted({int(x["id"].split("-")[2]) for x in body if x["id"].startswith(f"fascicle-{group}-")}))
    out = []
    for s in body:
        if s["id"].startswith("fascicle-"):
            group, ring, index = s["id"].split("-")[1:]
            if (rings[group].index(int(ring)) + int(index)) % TUFT100["keep"] == 0:
                continue
            vs = s["vertices"]
            root = [sum(v[i] for v in vs[:9]) / 9 for i in range(3)]
            mean = [sum(v[i] for v in vs) / len(vs) for i in range(3)]
            d = unit(sub(mean, root))
            reach = max(dot(sub(v, root), d) for v in vs)
            p0 = [root[i] - d[i] * TUFT100["sink"] for i in range(3)]
            p2 = [root[i] + d[i] * reach for i in range(3)]
            r0, r1 = TUFT100["r0"], TUFT100["r1"]
            out.append({"id": s["id"], "kind": "loft", "bone": s["bone"], "material": s["material"], "role": s["role"], "tone": s["tone"], "sections": rod(IDENTITY, [p0, p2], (r0, r1)), "capStart": True, "capEnd": True})
            continue
        key = s["id"].split("-")[0]
        if key in HEAD100:
            s["radii"] = [round(r * HEAD100[key], 5) for r in s["radii"]]
        out.append(s)
    body[:] = out
    ids = {s["id"] for s in body}
    vol["thermal"][:] = [s for s in vol["thermal"] if "of" not in s or s["of"] in ids]
    count = sum(1 for s in body if s["id"].startswith("fascicle-"))
    vol["decisions"][0] = {
        "en": f"At 1:100 the smallest feature is 15 mm, wider than a fascicle of four spines (9 mm). The spine coat is therefore {count} tufts: two of every three fascicles of each ring of the 1:50 coat, staggered from ring to ring, restated as one cone of 15.6 mm base that keeps the root patch, the backward lean and the tip of the fascicle it stands for, so the envelope of the coat, its direction in the foraging stance and the radiating mantle of the rolled animal are those of 1:50. Eye and ear are 9.2 mm, the thinnest stroke of the head drawing at this scale.",
        "de": f"Bei 1:100 ist das kleinste Merkmal 15 mm und damit breiter als ein Büschel aus vier Stacheln (9 mm). Das Stachelkleid besteht deshalb aus {count} Büscheln: zwei von drei Büscheln jedes Rings des Kleids von 1:50, von Ring zu Ring versetzt, neu gefasst als ein Kegel mit 15,6 mm Fuß, der Ansatzfläche, Rückwärtsneigung und Spitze des Büschels behält, für das er steht. Hüllform, Strichrichtung beim Suchen und der strahlige Mantel des eingerollten Tiers entsprechen so 1:50. Auge und Ohr messen 9,2 mm, den dünnsten Strich der Kopfzeichnung in diesem Maßstab.",
    }


ENVELOPE = 0.006
COARSE = {
    "200": {"en": "At 1:200 the smallest feature is 30 mm, longer than a spine (22 mm): no spine and no tuft is a part of this scale. The coat is the smooth envelope of the spine tips: the two dome halves stand 6 mm outside the 1:100 dome that carries the tufts, the saddle closes the back between them and the four coat bowls lie inside until the animal rolls up.", "de": "Bei 1:200 ist das kleinste Merkmal 30 mm und damit länger als ein Stachel (22 mm): Stacheln und Büschel sind in diesem Maßstab keine Teile. Das Kleid ist die glatte Hüllfläche der Stachelspitzen: Die beiden Kuppelhälften stehen 6 mm außerhalb der Kuppel von 1:100, die die Büschel trägt, der Sattel schließt den Rücken dazwischen, und die vier Schalen liegen innen, bis sich das Tier einrollt."},
    "500": {"en": "At 1:500 the smallest feature is 75 mm, half the width of the animal: the coat is one smooth envelope of the spine tips in two dome halves over the flexing back, 6 mm outside the dome that carries the tufts at 1:100; spines, tufts and fringe are not parts of this scale.", "de": "Bei 1:500 ist das kleinste Merkmal 75 mm, die halbe Breite des Tiers: Das Kleid ist eine glatte Hüllfläche der Stachelspitzen in zwei Kuppelhälften über dem beweglichen Rücken, 6 mm außerhalb der Kuppel, die bei 1:100 die Büschel trägt; Stacheln, Büschel und Saum sind in diesem Maßstab keine Teile."},
}


def body_coarse(vol, scale):
    """🥚️ 1:200 and 1:500: the coat as the envelope of the spine tips, no tufts."""
    body = vol["body"]
    body[:] = [s for s in body if not s["id"].startswith("fascicle-")]
    for s in body:
        if s["id"] in ("dome", "dome-front"):
            for q in s["sections"]:
                q["rx"], q["ry"] = round(q["rx"] + ENVELOPE, 5), round(q["ry"] + ENVELOPE, 5)
    ids = {s["id"] for s in body}
    vol["thermal"][:] = [s for s in vol["thermal"] if "of" not in s or s["of"] in ids]
    vol["decisions"][0] = COARSE[scale]


def costal(st):
    """🫁️ Costal cartilages: every rib of the chest runs on from its ventral end to the sternum."""
    sternum = [q["at"] for q in next(s for s in st if s["id"] == "sternum")["sections"]]
    for s in st:
        if s["id"].startswith("rib-") and s["bone"] == "chest":
            end = s["sections"][-1]
            best = None
            for a, b in zip(sternum, sternum[1:]):
                d = [b[0] - a[0], b[1] - a[1]]
                k = max(0.0, min(1.0, ((end["at"][0] - a[0]) * d[0] + (end["at"][1] - a[1]) * d[1]) / (d[0] * d[0] + d[1] * d[1])))
                p = [a[0] + k * d[0], a[1] + k * d[1]]
                gap = math.hypot(p[0] - end["at"][0], p[1] - end["at"][1])
                if best is None or gap < best[0]:
                    best = (gap, p)
            sign = -1 if end["at"][2] < 0 else 1
            s["sections"].append(dict(end, at=[round(best[1][0], 5), round(best[1][1], 5), sign * 0.0012]))


RING = {"centre": 0.045, "a": 0.09, "b": 0.052, "flank": -0.02, "brow": 0.036, "rump": 0.014, "r": 0.0014}
BANDS = {
    "frontodorsalis": ("head", ((0.014, 0.011, 0.002), (-0.012, 0.0135, 0.0015), (-0.0377, 0.0122, 0.0005)), 0.0012),
    "caudodorsalis": ("trunk", ((-0.006, 0.0095, 0.0015), (-0.026, 0.0105, 0.0012), (-0.0446, 0.0085, 0.0005)), 0.0009),
}
COAT = {"r": ((0.0, -0.022, 0.014), 101.0), "f": ((0.1161, -0.022, 0.014), 100.0)}


def orbicularis(st):
    """⭕️ One orbicularis ring round the edge of the spiny skin, a quadrant on each coat bone; the frontodorsalis bands on the head and the caudodorsalis bands on the loin run from the skull roof and from the sacrum to where the ring crosses the midline in the foraging stance."""
    for s in st:
        if s["id"].startswith("orbicularis-"):
            end, name = s["id"][-2], s["id"][-1]
            sd = -1.0 if name == "n" else 1.0
            o, deg = COAT[end]
            f = sagittal_frame(side(o, sd), deg)
            angles = (0, 22.5, 45, 67.5, 90) if end == "f" else (180, 157.5, 135, 112.5, 90)
            rise = RING["brow"] if end == "f" else RING["rump"]
            points = []
            for a in angles:
                c, sn = math.cos(math.radians(a)), math.sin(math.radians(a))
                points.append([RING["centre"] + RING["a"] * c, RING["flank"] + rise * c * c, sd * RING["b"] * sn])
            s["sections"] = rod(f, points, RING["r"])
    at = next(i for i, s in enumerate(st) if s["id"] == "orbicularis-fn")
    for band, (bone_id, points, r) in BANDS.items():
        for name, sd in SIDES:
            st.insert(at, member(f"{band}-{name}", bone_id, rod(IDENTITY, [side(q, sd) for q in points], r), slender=True))


GIRDLE_TEXT = {
    "50": {
        "en": "Girdles at 1:50: each shoulder blade is a thin plate with its spine and acromion on its own rig bone, lying on the rib cage from the vertebral border (20 mm beside the midline) down and forward to the shoulder joint (28 mm beside it); a clavicle runs from the acromion to the front of the sternum. Each hip bone hangs on its own rig bone from the sacroiliac joint (9 mm beside the midline) down, back and out to the hip joint (24 mm beside it): ilium to the acetabulum, ischium to the tuber, pubis to the symphysis and back to the tuber round the obturator opening. The sacrum lies between the hip bones, the tail vertebrae follow it. The cranial ribs narrow to the thoracic inlet so that the shoulder blades lie outside the rib cage, and every rib of the chest runs on as costal cartilage to the sternum. The orbicularis is one ring round the edge of the spiny skin, a quadrant on each coat bone, held by the frontodorsalis bands from the skull roof and the caudodorsalis bands from the sacrum.",
        "de": "Gürtel bei 1:50: Jedes Schulterblatt ist eine dünne Platte mit Gräte und Schulterhöhe auf einem eigenen Rig-Knochen und liegt auf dem Brustkorb vom Wirbelrand (20 mm neben der Mitte) nach vorn unten zum Schultergelenk (28 mm daneben); ein Schlüsselbein läuft von der Schulterhöhe zum vorderen Brustbeinende. Jedes Hüftbein hängt an einem eigenen Rig-Knochen vom Kreuzdarmbeingelenk (9 mm neben der Mitte) nach hinten unten außen zum Hüftgelenk (24 mm daneben): Darmbein bis zur Pfanne, Sitzbein bis zum Sitzbeinhöcker, Schambein zur Symphyse und zurück zum Höcker um das Hüftloch. Das Kreuzbein liegt zwischen den Hüftbeinen, die Schwanzwirbel schließen an. Die vorderen Rippen verengen sich zur Brustöffnung, sodass die Schulterblätter außen auf dem Brustkorb liegen, und jede Rippe der Brust läuft als Rippenknorpel bis zum Brustbein. Der Orbicularis ist ein Ring um den Rand der Stachelhaut, ein Viertel je Mantelknochen, gehalten von den Frontodorsalis-Bändern vom Schädeldach und den Caudodorsalis-Bändern vom Kreuzbein.",
    },
    "coarse": {
        "en": "Girdles at this scale: the shoulder blade and the hip bone are one slender rod each along their own rig bones, from the vertebral border to the shoulder joint and from the sacroiliac joint to the hip joint, at the true breadths; the pelvis block stands for sacrum, ischium and pubis. The cranial rib hoops narrow to the thoracic inlet and close on the sternum. The orbicularis is one ring round the edge of the spiny skin in four quadrants, held by the frontodorsalis bands from the skull roof and the caudodorsalis bands from the sacrum.",
        "de": "Gürtel in diesem Maßstab: Schulterblatt und Hüftbein sind je ein schlanker Stab entlang ihrer eigenen Rig-Knochen, vom Wirbelrand zum Schultergelenk und vom Kreuzdarmbeingelenk zum Hüftgelenk, in den wahren Breiten; der Beckenblock steht für Kreuzbein, Sitzbein und Schambein. Die vorderen Rippenbögen verengen sich zur Brustöffnung und schließen am Brustbein. Der Orbicularis ist ein Ring um den Rand der Stachelhaut in vier Vierteln, gehalten von den Frontodorsalis-Bändern vom Schädeldach und den Caudodorsalis-Bändern vom Kreuzbein.",
    },
}


def volumes(vol):
    body100(vol["100"])
    body_coarse(vol["200"], "200")
    body_coarse(vol["500"], "500")
    for scale in ("50", "100", "200", "500"):
        ribs(vol[scale]["structure"])
        costal(vol[scale]["structure"])
        orbicularis(vol[scale]["structure"])
        vol[scale]["decisions"].append(GIRDLE_TEXT["50" if scale == "50" else "coarse"])
    for scale in ("50", "100", "200", "500"):
        st = vol[scale]["structure"]
        (girdle50 if scale == "50" else girdle_coarse)(st)


def candidate(phase):
    doc = json.load(open(FILE, encoding="utf-8"))
    before = json.load(open(BEFORE, encoding="utf-8"))
    doc["asset"]["rig"] = before["rig"]
    rig(doc, phase)
    if phase == "build":
        doc["asset"]["volumes"] = before["volumes"]
        volumes(doc["asset"]["volumes"])
    os.makedirs(OUT, exist_ok=True)
    json.dump(doc, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False)
    return doc


def fmt(p):
    return ",".join(f"{v:.6f}" for v in p)


def main():
    phase = sys.argv[1]
    if phase == "p1":
        candidate(phase)
        for name, s in SIDES:
            for kind, at, tip in (("scapula", SCAPULA_AT, SCAPULA_TIP), ("ilium", ILIUM_AT, ILIUM_TIP)):
                a, t = side(at, s), side(tip, s)
                print(run(["--file", CANDIDATE, "--solve", f"{kind}-{name}", "--at", fmt(a), "--u", fmt(sub(t, a)), "--w", "0,0,1"]))
    elif phase == "p2":
        candidate(phase)
        for name, s in SIDES:
            for limb, parent in (("fore", "scapula"), ("hind", "ilium")):
                print(run(["--file", CANDIDATE, "--solve", f"{limb}-{name}", "--parent", f"{parent}-{name}"]))
    elif phase == "build":
        candidate(phase)
        print(run(["--file", CANDIDATE, "--bone", "scapula-n", "--bone", "fore-n", "--bone", "ilium-f", "--bone", "hind-f"]))
        print("frames here:", json.dumps(lateral_frame(side(SCAPULA_AT, -1), side(SCAPULA_TIP, -1))), json.dumps(lateral_frame(side(ILIUM_AT, 1), side(ILIUM_TIP, 1))))
    elif phase == "publish":
        doc = candidate("build")
        spec = __import__("importlib.util").util.spec_from_file_location("io", os.path.join(T, "r3-hedgehog-io.py"))
        io = __import__("importlib.util").util.module_from_spec(spec)
        spec.loader.exec_module(io)
        open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + io.dump(doc["asset"]["volumes"], 0, "compact") + "}\n")
        io.put({"rig": doc["asset"]["rig"]})
        print(subprocess.run(["python3", "volumes-insert.py", "volumes/animals/hedgehog.json", FILE], cwd=T, capture_output=True, text=True).stdout.strip()[-300:])


main()
