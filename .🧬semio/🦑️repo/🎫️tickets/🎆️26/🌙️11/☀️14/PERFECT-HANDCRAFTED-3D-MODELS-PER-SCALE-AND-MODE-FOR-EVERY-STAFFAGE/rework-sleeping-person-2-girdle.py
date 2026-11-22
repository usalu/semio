"""🦴 Gives `humans/sleeping-person` her shoulder and pelvic girdles on the lateral rest placement and re-sculpts the girdle bones of the structure lists by hand at every scale.

Rig: `clavicle-l/-r` from the sternoclavicular to the acromioclavicular joint and `ilium-l/-r` from the sacroiliac joint to the hip joint; arms and thighs are re-parented without moving. Every station below is hand-chosen for a 1.70 m woman lying supine and is written in the frame of `torso` (u cranial, v dorsal, w left, origin L5/S1) or `pelvis` (same axes, origin between the hip joints), then expressed in the frame of the girdle bone it rides on.

Usage: `python3 rework-sleeping-person-2-girdle.py candidate|write`
"""
import ast
import json
import math
import sys
import types
from pathlib import Path

T = Path(__file__).resolve().parent
SRC = T / "volumes/humans/sleeping-person.json"
CAT = T.parents[6] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/🔣️.json"
OUT = T / "🗑️generated/rework-sleeping-person/cand/🔣️.json"
LIMB = [0.5, 0.5, 0.5, 0.5]
SC = (0.432, -0.070, 0.022)
AC = (0.440, 0.000, 0.150)
SI = (0.065, 0.045, 0.035)
HIP = (0.0, 0.0, 0.087)


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def tr(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def app(a, p):
    return [sum(a[i][k] * p[k] for k in range(3)) for i in range(3)]


def rest(angle, abduction, twist):
    a, b, t = (math.radians(angle), -math.radians(abduction), -math.radians(twist))
    rz = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    ry = [[math.cos(b), 0, math.sin(b)], [0, 1, 0], [-math.sin(b), 0, math.cos(b)]]
    rx = [[1, 0, 0], [0, math.cos(t), -math.sin(t)], [0, math.sin(t), math.cos(t)]]
    return mul(mul(rz, ry), rx)


def angles(r):
    return (
        round(math.degrees(math.atan2(r[1][0], r[0][0])), 4),
        round(math.degrees(math.asin(max(-1, min(1, r[2][0])))), 4),
        round(-math.degrees(math.atan2(r[2][1], r[2][2])), 4),
    )


def basis(u, w):
    n = math.sqrt(sum(c * c for c in u))
    u = [c / n for c in u]
    d = sum(a * b for a, b in zip(u, w))
    w = [a - d * b for a, b in zip(w, u)]
    n = math.sqrt(sum(c * c for c in w))
    w = [c / n for c in w]
    v = [w[1] * u[2] - w[2] * u[1], w[2] * u[0] - w[0] * u[2], w[0] * u[1] - w[1] * u[0]]
    return tr([u, v, w])


def quat(m):
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
    return q


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def girdle(joint, tip, w):
    """🧭 Frame of a girdle bone in its parent: origin, basis (columns u, v, w), length and the three rest angles."""
    u = [b - a for a, b in zip(joint, tip)]
    b = basis(u, w)
    a = angles(b)
    return {"o": list(joint), "b": rest(*a), "len": round(math.sqrt(sum(c * c for c in u)), 5), "angles": a}


def frames(side):
    """🧭 Clavicle and hip-bone frames of one side (side +1 left, -1 right) in `torso` and `pelvis` coordinates."""
    m = lambda p: (p[0], p[1], p[2] * side)
    return girdle(m(SC), m(AC), (0, side, 0)), girdle(m(SI), m(HIP), (0, side, 0))


def into(frame, p):
    return [round(c, 5) for c in app(tr(frame["b"]), [a - b for a, b in zip(p, frame["o"])])]


def qinto(frame, q):
    return [round(c, 5) for c in qmul(quat(tr(frame["b"])), q)]


BONE = {"material": "bone", "role": "body", "tone": 0.85, "part": "member"}


def loft(ident, bone, frame, side, rows, tilt=0, **more):
    half = -math.radians(tilt) * side / 2
    turned = qmul([math.sin(half), 0, 0, math.cos(half)], LIMB)
    sections = []
    for x, y, z, rx, ry in rows:
        p = (x, y, z * side)
        sections.append({"at": into(frame, p) if frame else [round(c, 5) for c in p], "rx": rx, "ry": ry, "rotation": qinto(frame, turned) if frame else turned})
    return {"id": ident, "kind": "loft", "bone": bone, **BONE, "sections": sections, "capStart": True, "capEnd": True, "member": bone, **more}


def local(ident, bone, rows, side=1, **more):
    return {"id": ident, "kind": "loft", "bone": bone, **BONE, "sections": [{"at": [x, y, z * side], "rx": rx, "ry": ry, "rotation": LIMB} for x, y, z, rx, ry in rows], "capStart": True, "capEnd": True, "member": bone, **more}


def cap(ident, bone, frame, side, p0, p1, r0, r1):
    a, b = ((p[0], p[1], p[2] * side) for p in (p0, p1))
    return {"id": ident, "kind": "capsule", "bone": bone, **BONE, "p0": into(frame, a), "p1": into(frame, b), "r0": r0, "r1": r1, "member": bone}


def ball(ident, bone, frame, side, centre, radii, part="member"):
    c = (centre[0], centre[1], centre[2] * side)
    transform = {"translation": into(frame, c)}
    if len(set(radii)) > 1:
        transform["rotation"] = qinto(frame, [0, 0, 0, 1])
    return {"id": ident, "kind": "ellipsoid", "bone": bone, **BONE, "part": part, "radii": list(radii), "transform": transform, "member": bone}


def shoulder(scale, side):
    """🦴 Clavicle, scapula with spine, acromion, neck, glenoid and coracoid of one side."""
    tag = "l" if side > 0 else "r"
    bone = f"clavicle-{tag}"
    c, _ = frames(side)
    s = side
    if scale == 50:
        return [
            local(f"clavicle-{tag}", bone, [(-0.004, 0, 0, 0.010, 0.011), (0.03, 0, -0.007, 0.0065, 0.0065), (0.075, 0, -0.009, 0.0055, 0.0055), (0.115, 0, 0.005, 0.005, 0.007), (0.14, 0, 0.006, 0.0045, 0.009), (0.152, 0, 0.002, 0.004, 0.008)], s),
            loft(f"scapula-{tag}", bone, c, s, [(0.275, 0.068, 0.085, 0.004, 0.008), (0.31, 0.071, 0.09, 0.004, 0.028), (0.35, 0.074, 0.098, 0.004, 0.042), (0.39, 0.074, 0.105, 0.0045, 0.05), (0.42, 0.066, 0.105, 0.0045, 0.045), (0.432, 0.060, 0.10, 0.004, 0.035)]),
            cap(f"scapular-spine-{tag}", bone, c, s, (0.405, 0.080, 0.06), (0.43, 0.07, 0.125), 0.006, 0.007),
            cap(f"acromion-{tag}", bone, c, s, (0.43, 0.07, 0.125), (0.443, 0.012, 0.160), 0.007, 0.008),
            cap(f"scapular-neck-{tag}", bone, c, s, (0.400, 0.040, 0.140), (0.395, 0.072, 0.135), 0.008, 0.008),
            ball(f"glenoid-{tag}", bone, c, s, (0.4013, 0.030, 0.138), (0.018, 0.014, 0.009)),
            cap(f"coracoid-{tag}", bone, c, s, (0.412, 0.040, 0.138), (0.418, -0.005, 0.128), 0.005, 0.0045),
            ball(f"joint-clavicle-{tag}", bone, c, s, SC, (0.012, 0.012, 0.012), "joint"),
        ]
    if scale == 100:
        return [
            local(f"clavicle-{tag}", bone, [(-0.004, 0, 0, 0.009, 0.009), (0.075, 0, -0.008, 0.0075, 0.0075), (0.152, 0, 0.003, 0.0075, 0.0085)], s),
            loft(f"scapula-{tag}", bone, c, s, [(0.28, 0.070, 0.088, 0.0075, 0.012), (0.35, 0.073, 0.098, 0.0075, 0.042), (0.42, 0.066, 0.105, 0.0075, 0.046)]),
            cap(f"acromion-{tag}", bone, c, s, (0.412, 0.076, 0.07), (0.443, 0.014, 0.158), 0.0075, 0.008),
            cap(f"scapular-neck-{tag}", bone, c, s, (0.4013, 0.028, 0.140), (0.395, 0.07, 0.13), 0.0085, 0.0085),
            ball(f"joint-clavicle-{tag}", bone, c, s, SC, (0.012, 0.012, 0.012), "joint"),
        ]
    if scale == 200:
        return [
            local(f"clavicle-{tag}", bone, [(-0.004, 0, 0, 0.0065, 0.0065), (0.075, 0, -0.008, 0.0055, 0.0055), (0.152, 0, 0.003, 0.0055, 0.0065)], s, slender=True),
            cap(f"scapula-{tag}", bone, c, s, (0.405, 0.04, 0.135), (0.31, 0.07, 0.09), 0.015, 0.015),
            cap(f"acromion-{tag}", bone, c, s, (0.41, 0.062, 0.12), (0.443, 0.012, 0.158), 0.015, 0.015),
            ball(f"joint-clavicle-{tag}", bone, c, s, SC, (0.015, 0.015, 0.015), "joint"),
        ]
    return [local(f"clavicle-{tag}", bone, [(-0.004, 0, 0, 0.0065, 0.0065), (0.075, 0, -0.006, 0.0055, 0.0055), (0.154, 0, 0.003, 0.0055, 0.0065)], side, slender=True)]


def hip(scale, side):
    """🦴 Hip bone of one side: iliac wing, dorsal ilium to the sacroiliac joint, acetabulum, pubic rami and ischium."""
    tag = "l" if side > 0 else "r"
    bone = f"ilium-{tag}"
    _, h = frames(side)
    s = side
    if scale == 50:
        return [
            loft(f"iliac-wing-{tag}", bone, h, s, [(0.015, 0, 0.080, 0.028, 0.008), (0.045, 0, 0.092, 0.042, 0.006), (0.08, 0, 0.10, 0.055, 0.0055), (0.11, 0, 0.10, 0.062, 0.006), (0.124, 0, 0.10, 0.05, 0.006)], tilt=37),
            cap(f"iliac-dorsal-{tag}", bone, h, s, (0.09, 0.042, 0.075), (0.068, 0.05, 0.04), 0.011, 0.013),
            ball(f"acetabulum-{tag}", bone, h, s, (0, 0, 0.068), (0.03, 0.03, 0.013)),
            cap(f"pubis-{tag}", bone, h, s, (-0.004, -0.022, 0.062), (-0.012, -0.058, 0.006), 0.0085, 0.009),
            cap(f"ischium-{tag}", bone, h, s, (-0.02, 0.014, 0.066), (-0.078, 0.028, 0.056), 0.012, 0.015),
            cap(f"ischiopubic-ramus-{tag}", bone, h, s, (-0.078, 0.022, 0.054), (-0.035, -0.05, 0.012), 0.007, 0.007),
            ball(f"joint-ilium-{tag}", bone, h, s, SI, (0.012, 0.012, 0.012), "joint"),
        ]
    if scale == 100:
        return [
            loft(f"iliac-wing-{tag}", bone, h, s, [(0.0, 0, 0.07, 0.028, 0.0085), (0.06, 0, 0.096, 0.05, 0.0075), (0.122, 0, 0.10, 0.056, 0.0075)], tilt=37),
            cap(f"iliac-dorsal-{tag}", bone, h, s, (0.09, 0.042, 0.075), (0.068, 0.05, 0.04), 0.011, 0.013),
            cap(f"pubis-{tag}", bone, h, s, (-0.004, -0.022, 0.062), (-0.012, -0.058, 0.006), 0.0085, 0.009),
            cap(f"ischium-{tag}", bone, h, s, (-0.02, 0.014, 0.066), (-0.078, 0.028, 0.056), 0.012, 0.015),
            ball(f"joint-ilium-{tag}", bone, h, s, SI, (0.012, 0.012, 0.012), "joint"),
        ]
    if scale == 200:
        return [
            loft(f"hip-bone-{tag}", bone, h, s, [(-0.07, 0.02, 0.058, 0.018, 0.015), (0.0, 0, 0.07, 0.03, 0.015), (0.07, 0.01, 0.112, 0.05, 0.015), (0.12, 0.015, 0.128, 0.045, 0.015)]),
            cap(f"iliac-dorsal-{tag}", bone, h, s, (0.085, 0.04, 0.10), (0.068, 0.05, 0.04), 0.015, 0.015),
            ball(f"joint-ilium-{tag}", bone, h, s, SI, (0.015, 0.015, 0.015), "joint"),
        ]
    return [loft(f"hip-bone-{tag}", bone, h, s, [(-0.06, 0.02, 0.06, 0.0375, 0.0375), (0.02, 0.01, 0.075, 0.045, 0.0375), (0.115, 0.01, 0.10, 0.05, 0.0375)])]


def middle(scale):
    """🦴 Sacrum with coccyx on `pelvis` and the manubrium on `torso`."""
    if scale == 50:
        return [
            local("sacrum", "pelvis", [(-0.03, 0.062, 0, 0.004, 0.005), (-0.005, 0.068, 0, 0.006, 0.012), (0.03, 0.066, 0, 0.009, 0.03), (0.065, 0.052, 0, 0.013, 0.05), (0.09, 0.04, 0, 0.016, 0.052)]),
            {"id": "manubrium", "kind": "ellipsoid", "bone": "torso", **BONE, "radii": [0.024, 0.007, 0.03], "transform": {"translation": [0.428, -0.074, 0]}, "member": "torso"},
        ]
    if scale == 100:
        return [
            local("sacrum", "pelvis", [(-0.02, 0.064, 0, 0.0075, 0.0085), (0.03, 0.064, 0, 0.01, 0.03), (0.09, 0.04, 0, 0.016, 0.052)]),
            {"id": "manubrium", "kind": "ellipsoid", "bone": "torso", **BONE, "radii": [0.024, 0.008, 0.03], "transform": {"translation": [0.428, -0.072, 0]}, "member": "torso"},
        ]
    if scale == 200:
        return [local("sacrum", "pelvis", [(-0.01, 0.064, 0, 0.015, 0.015), (0.05, 0.056, 0, 0.015, 0.04), (0.09, 0.042, 0, 0.016, 0.05)])]
    return [{"id": "sacrum", "kind": "ellipsoid", "bone": "pelvis", **BONE, "radii": [0.06, 0.0375, 0.05], "transform": {"translation": [0.04, 0.045, 0]}, "member": "pelvis"}]


def hand(body, tag):
    """🦴 Hand skeleton along the modelled fingers: carpus, five metacarpals and two phalanx solids per digit (the thumb capsule of the body is its metacarpal)."""
    bone = f"hand-{tag}"
    s = -1 if tag == "l" else 1
    flesh = {x["id"]: x for x in body if x["bone"] == bone and x["kind"] == "capsule"}
    stick = lambda ident, p0, p1, r0, r1: {"id": ident, "kind": "capsule", "bone": bone, **BONE, "p0": list(p0), "p1": list(p1), "r0": r0, "r1": r1, "member": bone}
    out = [{"id": f"carpus-{tag}", "kind": "ellipsoid", "bone": bone, **BONE, "radii": [0.016, 0.009, 0.024], "transform": {"translation": [0.012, -0.002, 0]}, "member": bone}]
    thumb, tip = flesh[f"thumb-{tag}"], flesh[f"thumb-tip-{tag}"]
    out.append(stick(f"metacarpal-1-{tag}", (0.008, -0.006, 0.018 * s), thumb["p1"], 0.0055, 0.005))
    out.append(stick(f"phalanx-1-a-{tag}", thumb["p1"], [round((a + b) / 2, 5) for a, b in zip(tip["p0"], tip["p1"])], 0.0045, 0.0042))
    out.append(stick(f"phalanx-1-b-{tag}", [round((a + b) / 2, 5) for a, b in zip(tip["p0"], tip["p1"])], tip["p1"], 0.0042, 0.0038))
    for n in range(1, 5):
        near, far = flesh[f"finger-{n}-{tag}"], flesh[f"finger-{n}-tip-{tag}"]
        root = (0.022, -0.002, round(near["p0"][2] * 0.55, 5))
        out.append(stick(f"metacarpal-{n + 1}-{tag}", root, near["p0"], 0.005, 0.0048))
        out.append(stick(f"phalanx-{n + 1}-a-{tag}", near["p0"], near["p1"], 0.0045, 0.0042))
        out.append(stick(f"phalanx-{n + 1}-b-{tag}", far["p0"], far["p1"], 0.0042, 0.0038))
    return out


def details(scale, solids, body):
    """🔧 Lateral malleolus on the fibula, pillow core down on the mattress band, supports at 8 mm and at 1:50 the hand skeletons."""
    out = []
    for s in solids:
        if scale == 50 and s["id"].startswith(("metacarpals-", "finger-ray-", "thumb-ray-", "carpus-", "metacarpal-", "phalanx-")):
            if s["id"].startswith("metacarpals-"):
                out.extend(hand(body, s["id"][-1]))
            continue
        if s["id"].startswith("fibula-") and scale == 50 and s["sections"][-1]["at"][0] < 0.42:
            side = 1 if s["id"].endswith("-l") else -1
            s["sections"].append({"at": [0.425, -0.004, 0.024 * side], "rx": 0.009, "ry": 0.009})
        if s["id"] == "member-pillow" and scale == 50:
            s["transform"]["translation"][1] = 0.022
        if s["id"].startswith("support-quilt-bearing"):
            s["size"] = [0.075, 0.075, 0.075] if scale == 500 else [0.03, 0.008, 0.008]
            s["transform"]["translation"][1] = 0.0375 if scale == 500 else 0.004
        out.append(s)
    return out


GONE = {"clavicle-l", "clavicle-r", "scapula-l", "scapula-r", "pelvic-ring", "ilium-l", "ilium-r", "sacrum", "pelvic-girdle"}


NOTE = {
    "en": " Shoulder and pelvic girdle ride on their own bones: each clavicle runs from the sternoclavicular to the acromioclavicular joint and carries its scapula, each hip bone runs from the sacroiliac joint to the hip joint beside the sacrum, simplified to the smallest feature of this scale.",
    "de": " Schulter- und Beckengürtel sitzen auf eigenen Knochen: jedes Schlüsselbein läuft vom Sternoklavikular- zum Akromioklavikulargelenk und trägt sein Schulterblatt, jedes Hüftbein läuft vom Iliosakralgelenk zum Hüftgelenk neben dem Kreuzbein, vereinfacht auf das kleinste Merkmal dieses Maßstabs.",
}


def noted(decisions):
    for d in decisions:
        if d["en"].startswith("Structure at") and NOTE["en"] not in d["en"]:
            d["en"] += NOTE["en"]
            d["de"] += NOTE["de"]
    return decisions


def structure(scale, solids, body):
    solids = details(scale, solids, body)
    kept = [s for s in solids if s["id"] not in GONE and not s["id"].startswith(("joint-clavicle", "joint-ilium", "scapul", "acromion", "glenoid", "coracoid", "iliac-", "acetabulum", "pubis", "ischi", "hip-bone", "manubrium"))]
    at = next(i for i, s in enumerate(kept) if s["id"].startswith("femur"))
    new = middle(scale) + shoulder(scale, 1) + shoulder(scale, -1) + hip(scale, 1) + hip(scale, -1)
    return kept[:at] + new + kept[at:]


def rig(bones):
    """🦴 Inserts the four girdle bones and hangs arms and thighs from their tips without moving them."""
    kinds = {"arm": (0, "torso", "clavicle", "clavicle", "Schlüsselbein", (-10, 30)), "thigh": (1, "pelvis", "ilium", "hip bone", "Hüftbein", (-4, 4))}
    out = []
    for b in bones:
        limb, _, tag = b["id"].rpartition("-")
        if limb in kinds and tag in ("l", "r"):
            k, parent, stem, en, de, limits = kinds[limb]
            assert b["parent"] == parent, "already converted"
            side = 1 if tag == "l" else -1
            f = frames(side)[k]
            name = f"{stem}-{tag}"
            out.append({"id": name, "parent": parent, "title": {"en": ("Left " if side > 0 else "Right ") + en, "de": ("Linkes " if side > 0 else "Rechtes ") + de}, "x": f["o"][0], "y": f["o"][1], "z": f["o"][2], "angleDeg": f["angles"][0], "abductionDeg": f["angles"][1], "twistDeg": f["angles"][2], "lengthM": f["len"], "joint": {"type": "revolute", "minDeg": limits[0], "maxDeg": limits[1]}})
            a = angles(mul(tr(f["b"]), rest(b["angleDeg"], 0, 0)))
            o = into(f, (b["x"], b["y"], b.get("z", 0)))
            b.update({"parent": name, "x": o[0], "y": o[1], "z": o[2], "angleDeg": a[0], "abductionDeg": a[1], "twistDeg": a[2]})
        out.append(b)
    return out


def main():
    mode = sys.argv[1]
    text = CAT.read_text(encoding="utf-8")
    document = json.loads(text)
    source = json.loads(SRC.read_text(encoding="utf-8"))
    for scale in (50, 100, 200, 500):
        source["volumes"][str(scale)]["structure"] = structure(scale, source["volumes"][str(scale)]["structure"], source["volumes"][str(scale)]["body"])
        source["volumes"][str(scale)]["decisions"] = noted(source["volumes"][str(scale)]["decisions"])
    tree = ast.parse((T / "volumes-insert.py").read_text(encoding="utf-8"))
    tree.body = [node for node in tree.body if isinstance(node, (ast.FunctionDef, ast.Import, ast.ImportFrom))]
    insert = types.ModuleType("insert")
    exec(compile(tree, "volumes-insert.py", "exec"), insert.__dict__)
    old = '"rig": ' + insert.dump(document["asset"]["rig"], 2)
    assert text.count(old) == 1, "rig text not found once"
    document["asset"]["rig"]["bones"] = rig(document["asset"]["rig"]["bones"])
    area = '"id": "face-skin"'
    at = text.index(area)
    cut = text.index('"areaM2": 0.0871', at)
    assert cut - at < 600
    text2 = text[:cut] + '"areaM2": 0.12' + text[cut + len('"areaM2": 0.0871'):]
    for region in document["asset"]["thermal"]["regions"]:
        if region["id"] == "face-skin":
            region["areaM2"] = 0.12
    if mode == "candidate":
        document["asset"]["volumes"] = source["volumes"]
        OUT.parent.mkdir(parents=True, exist_ok=True)
        OUT.write_text(json.dumps(document, ensure_ascii=False), encoding="utf-8")
        print(OUT)
    elif mode == "write":
        CAT.write_text(text2.replace(old, '"rig": ' + insert.dump(document["asset"]["rig"], 2)), encoding="utf-8")
        SRC.write_text(json.dumps(source, ensure_ascii=False), encoding="utf-8")
        print("written", CAT, SRC)


main()
