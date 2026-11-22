"""🛹️ Pass 1 chisel of `humans/skateboarder`: hand-stated stations of the shoulder slope, the trouser front and the skeleton
(pelvis, femur, tibia, humerus, forearm, hand and foot bones) per scale. Every number below is chosen against the reference
table of `rework/assets/humans--skateboarder.md`; the helpers of `rework-02-lib.py` only write the solid JSON.
"""
import copy
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib02", os.path.join(T, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
SOURCE = os.path.join(T, "volumes", "humans", "skateboarder.json")
MM = 0.001
BONE = {"material": "bone", "tone": 0.9}
SOLE = math.radians(19.457)


def mm(keys):
    """📏️ Key rows stated in millimetres (u, cv, cw, rv, rw[, exponent]) to metres."""
    return [tuple(c * MM for c in k[:5]) + tuple(k[5:]) for k in keys]


def bone_loft(id, bone, keys, sub=2, **rounds):
    """🦴️ A bone as a loft through hand-stated sections."""
    return lib.loft(id, bone, BONE["material"], BONE["tone"], mm(keys), sub=sub, member=bone, part="member", **{k: v * MM for k, v in rounds.items()})


def bone_capsule(id, bone, p0, p1, r0, r1):
    """🦴️ A bone as a tapered capsule between two hand-stated points (mm)."""
    return lib.capsule(id, bone, BONE["material"], BONE["tone"], [c * MM for c in p0], [c * MM for c in p1], r0 * MM, r1 * MM, member=bone, part="member")


def bone_ellipsoid(id, bone, at, radii):
    """🦴️ A bone as an ellipsoid (mm)."""
    return lib.ellipsoid(id, bone, BONE["material"], BONE["tone"], [c * MM for c in at], [r * MM for r in radii], member=bone, part="member")


def foot(s, h, w):
    """🦶️ A point stated ahead of (s) and above (h) the ankle parallel to the sole, in the oblique frame of the foot bone."""
    return (s * math.cos(SOLE) - h * math.sin(SOLE), s * math.sin(SOLE) + h * math.cos(SOLE), w)


def pelvis(scale):
    """🦴️ Ilium as a wing in its true oblique-sagittal plane from the acetabulum to the crest, tied to the sacrum by its auricular part."""
    if scale == 50:
        wing = [(-8, 2, 84, 26, 16), (20, 6, 90, 28, 11), (50, 10, 106, 44, 8), (85, 12, 122, 62, 7.5), (112, 14, 130, 64, 7.5)]
    elif scale == 100:
        wing = [(-8, 2, 84, 26, 16), (50, 10, 106, 44, 8.5), (112, 14, 130, 64, 8)]
    else:
        wing = [(-8, 2, 84, 26, 17), (50, 10, 106, 44, 15), (112, 14, 128, 62, 15)]
    return [
        bone_loft("ilium-l", "pelvis", wing, sub=3 if scale == 50 else 2, round1=10),
        bone_capsule("ilium-auricular-l", "pelvis", (80, 62, 40), (85, 50, 118), 16, 15),
    ]


def femur(scale):
    """🦴️ Femur with head, neck (125 degrees to the shaft), greater trochanter, bowed oblique shaft and condyles broader than deep."""
    neck = bone_capsule("femur-neck-l", "thigh-l", (0, 0, 0), (34, 2, 50), 22, 15 if scale < 200 else 17)
    if scale == 50:
        shaft = [(30, 2, 58, 17, 16), (70, 5, 48, 15, 15), (140, 9, 36, 14, 14), (240, 11, 20, 14, 14), (330, 8, 8, 15, 16), (375, 3, 2, 22, 30), (398, 0, 0, 31, 40, 2.6), (414, -2, 0, 27, 38, 2.6)]
    elif scale == 100:
        shaft = [(30, 2, 58, 17, 16), (140, 9, 36, 14, 14), (330, 8, 8, 15, 16), (398, 0, 0, 31, 40, 2.6), (414, -2, 0, 27, 38, 2.6)]
    else:
        shaft = [(30, 2, 54, 17, 17), (240, 10, 20, 15, 15), (380, 2, 2, 24, 32), (412, -2, 0, 27, 38)]
    out = [neck, bone_loft("femur-l", "thigh-l", shaft, round0=8, round1=9)]
    if scale < 200:
        out.insert(1, bone_ellipsoid("trochanter-l", "thigh-l", (26, 0, 64), (24, 16, 14)))
    return out


def tibia(scale):
    """🦴️ Tibia: plateau broader than deep, tuberosity ahead, shaft, medial malleolus."""
    if scale == 50:
        keys = [(-2, 2, 0, 24, 37, 2.6), (14, 4, 0, 25, 38, 2.6), (45, 9, -1, 17, 17), (120, 8, -2, 14, 14), (300, 5, -4, 12.5, 12), (400, 3, -5, 13, 14), (430, 2, -8, 16, 22)]
    elif scale == 100:
        keys = [(-2, 2, 0, 24, 37, 2.6), (14, 4, 0, 25, 38, 2.6), (45, 9, -1, 17, 17), (300, 5, -4, 12.5, 12.5), (430, 2, -8, 16, 22)]
    else:
        keys = [(-2, 2, 0, 24, 36), (45, 9, -1, 17, 17), (300, 5, -4, 15, 15), (430, 2, -8, 16, 22)]
    return [bone_loft("tibia-l", "shank-l", keys, round1=8)]


def humerus(scale):
    """🦴️ Humerus: head, surgical neck, shaft, epicondyles across the elbow axis (lateral, not sagittal)."""
    if scale == 50:
        keys = [(-2, 0, 0, 23, 23), (18, 0, 2, 22, 21), (45, 0, 3, 13, 13), (170, 0, 1, 11.5, 11.5), (280, 0, 0, 12, 13), (315, 0, 0, 13, 26), (330, 0, 0, 13, 31, 2.4)]
    elif scale == 100:
        keys = [(-2, 0, 0, 23, 23), (18, 0, 2, 22, 21), (45, 0, 3, 13, 13), (280, 0, 0, 12, 13), (330, 0, 0, 13, 31, 2.4)]
    else:
        keys = [(-2, 0, 0, 22, 22), (45, 0, 3, 15, 15), (280, 0, 0, 15, 15), (330, 0, 0, 15, 31)]
    return [bone_loft("humerus-l", "upper-arm-l", keys, round1=6)]


def forearm(scale):
    """🦴️ Ulna and radius in the half-pronated hang of the arm: side by side at the elbow, radius on the thumb side at the wrist."""
    if scale == 50:
        ulna = [(-22, -10, -4, 11, 10), (-4, -9, -5, 12, 10), (25, -9, -5, 8, 8), (130, -11, -3, 6.5, 6.5), (245, -13, 0, 6, 6), (257, -14, 0, 7, 7)]
        radius = [(6, 6, 13, 9.5, 9.5), (22, 6, 12, 6.5, 6.5), (60, 7, 11, 7, 7), (150, 9, 6, 7, 7), (225, 9, 2, 9, 8), (256, 8, 1, 15, 10)]
    else:
        ulna = [(-22, -10, -4, 11, 10), (25, -9, -5, 8, 8), (257, -14, 0, 7.5, 7.5)]
        radius = [(6, 6, 13, 9.5, 9.5), (60, 7, 11, 7.5, 7.5), (256, 8, 1, 15, 10)]
    return [bone_loft("ulna-l", "forearm-l", ulna, round0=5, round1=4), bone_loft("radius-l", "forearm-l", radius, round0=3, round1=4)]


def hand(scale):
    """🖐️ Carpus, five rays (metacarpal with knuckle and the three phalanges following the curl of the body's fingers)."""
    if scale == 100:
        return [
            bone_loft("metacarpus-l", "hand-l", [(-6, 2, 0, 17, 8), (20, 3, 0, 25, 8.5), (60, 3, -1, 34, 8), (95, 2, -2, 37, 7.5)], round0=3),
            bone_loft("finger-bones-l", "hand-l", [(90, 0, -2, 37, 7.5), (137, 0, -8, 36, 7.5), (162, 0, -19, 35, 7.5), (181, 0, -33, 33, 7.5)], round1=4),
            bone_capsule("thumb-bones-l", "hand-l", (6, 20, -5), (38, 36, -12), 8, 7.5),
            bone_capsule("thumb-phalanges-l", "hand-l", (38, 36, -12), (96, 36, -20), 7.5, 7.5),
        ]
    out = [bone_loft("carpus-l", "hand-l", [(-6, 2, 0, 17, 8), (4, 2, 0, 23, 9.5), (16, 3, 0, 25, 9), (27, 3, 0, 22, 7)], round0=3, round1=3)]
    for n, (base, tip, reach) in enumerate([(15, 28.5, 180), (5, 10, 184), (-5, -10, 181), (-15, -28, 166)]):
        k = (reach - 93) / 87
        out.append(bone_loft(f"ray-l-{n + 2}", "hand-l", [
            (22, base, 0, 5.5, 5.5), (58, (base + tip) / 2, -1, 4, 4), (93, tip, -2, 6, 6),
            (93 + 23 * k, tip, -5, 4, 4), (93 + 44 * k, tip, -8, 5.2, 5.2), (93 + 58 * k, tip, -13, 3.9, 3.9),
            (93 + 69 * k, tip, -19, 4.8, 4.8), (93 + 79 * k, tip, -26, 3.8, 3.8), (reach, tip, -33, 3.8, 3.8)], round1=3))
    out.append(bone_loft("ray-l-1", "hand-l", [(4, 18, -4, 6.5, 6.5), (22, 29, -9, 4.5, 4.5), (38, 36, -12, 6, 6), (52, 39, -14, 4.2, 4.2), (66, 40, -16, 5.2, 5.2), (84, 38, -19, 3.9, 3.9), (98, 35, -20, 3.9, 3.9)], round1=3))
    return out


def foot_bones(scale):
    """🦶️ Talus, calcaneus, tarsal block, five metatarsals fanning to the ball, toe phalanges on the forefoot bone."""
    out = [
        bone_ellipsoid("talus-l", "foot-l", foot(4, -16, 0), (22, 15, 17)),
        bone_capsule("calcaneus-l", "foot-l", foot(-48, -50, 4), foot(18, -36, 4), 17, 13),
        bone_loft("tarsus-l", "foot-l", [(30, -21, 0, 13, 22, 2.6), (50, -17, 0, 12, 27, 2.6), (71, -13, 0, 10, 30, 2.6)], round0=4, round1=4),
    ]
    if scale == 100:
        out.append(bone_loft("metatarsus-l", "foot-l", [(64, -13, 0, 8, 26, 3), (110, -6, 0, 8, 33, 3), (154, 1, 0, 8, 38, 3)], round1=4))
        out.append(bone_loft("phalanges-l", "toe-l", [(-4, 1, -1, 7.5, 36, 3), (30, -2, -2, 7.5, 33, 3), (54, -6, -6, 7.5, 24, 3)], round1=4))
        return out
    for n, (w0, s0, s1, w1, r0, r1, start, reach) in enumerate([(-22, 58, 146, -27, 8, 8.5, -4, 56), (-11, 58, 150, -13, 6.5, 6, -2, 52), (0, 58, 146, 0, 6.5, 6, -4, 46), (10, 56, 140, 12, 6.5, 6, -8, 40), (20, 50, 130, 24, 7, 6, -17, 33)]):
        out.append(bone_capsule(f"metatarsal-l-{n + 1}", "foot-l", foot(s0, -34, w0), foot(s1, -50, w1), r0, r1))
        f = 1.5 if n == 0 else 1.0
        out.append(bone_loft(f"toe-ray-l-{n + 1}", "toe-l", [
            (start, 1, w1, 5.5 * f, 5.5 * f), (reach * 0.42, -1, w1, 4 * f, 4 * f), (reach * 0.55, -2, w1, 5 * f, 5 * f),
            (reach * 0.85, -5, w1, 3.9 * f, 3.9 * f), (reach, -6, w1, 3.9 * f, 3.9 * f)], round1=3))
    return out


REPLACED = {"ilium", "femur", "tibia", "humerus", "ulna", "radius", "metacarpus", "tarsus", "phalanges", "ilium-auricular", "femur-neck", "trochanter", "carpus",
            "finger-bones", "thumb-bones", "thumb-phalanges", "talus", "calcaneus", "metatarsus"}


def stem(id):
    """🏷️ The id without its side."""
    for side in ("-l", "-r"):
        if id.endswith(side):
            return id[:-2]
        if side + "-" in id:
            return id.split(side + "-")[0]
    return id


def structure(volume, scale):
    """🩻️ Replaces the named bones of a scale's structure list by the hand-built ones, left and mirrored right, at the place of the first one."""
    new = []
    parts = pelvis(scale) + femur(scale) + tibia(scale) + humerus(scale)
    if scale < 200:
        parts += forearm(scale) + hand(scale) + foot_bones(scale)
    for p in parts:
        new += [p, lib.mirror(p)]
    stems = {stem(p["id"]) for p in parts} | {"ray", "metatarsal", "toe-ray"}
    old = volume["structure"]
    gone = [x for x in old if x.get("kind") and (stem(x["id"]) in stems or (scale < 200 and stem(x["id"]) in REPLACED)) and x.get("part") == "member"]
    at = old.index(gone[0])
    kept = [x for x in old if x not in gone]
    volume["structure"] = kept[:at] + new + kept[at:]


def restate(solid, shells, rows, start):
    """✏️ Replaces the sections of a body loft from station `start` (mm) on by hand-stated rows (u, cv, rv, rw, exponent), keeping the inset of every thermal shell."""
    old = solid["sections"]
    first = next(i for i, s in enumerate(old) if s["at"][0] >= start * MM - 1e-9)
    template = old[first]
    insets = []
    for shell in shells:
        ss = shell["sections"]
        i = min(first, len(ss) - 1)
        insets.append((ss[i]["at"][0] - old[min(first, len(old) - 1)]["at"][0] if len(ss) != len(old) else 0, old[first]["rx"] - ss[i]["rx"], old[first]["ry"] - ss[i]["ry"], len(ss) == len(old)))

    def sections(dw, dv):
        out = []
        for u, cv, rv, rw, e in rows:
            s = copy.deepcopy(template)
            s["at"], s["rx"], s["ry"], s["exponent"] = [lib.r5(u * MM), lib.r5(cv * MM), 0], lib.r5(rw * MM - dw), lib.r5(rv * MM - dv), e
            out.append(s)
        return out

    solid["sections"] = old[:first] + sections(0, 0)
    for shell, (_, dw, dv, same) in zip(shells, insets):
        if not same:
            print("[DEBUG] shell", shell["id"], "has other stations than", solid["id"], "- left as it is")
            continue
        shell["sections"] = shell["sections"][:first] + sections(dw, dv)


def body(volume, scale):
    """🧥️ Shoulder slope of the hoodie (trapezius line from the neck root to the acromion) and, at 1:50, the trouser front over the thigh roots."""
    solids = {x["id"]: x for x in volume["body"]}
    thermal = {x["id"]: x for x in volume["thermal"] if x.get("kind") == "loft"}
    shells = [thermal[i] for i in ("th-hoodie-torso-air", "th-hoodie-tshirt") if i in thermal]
    if scale == 50:
        restate(solids["hoodie"], shells, [(455, 2, 94, 190, 2.8), (478, 4, 86, 176, 2.6), (496, 4, 76, 150, 2.4), (508, 4, 70, 122, 2.2), (519, 4, 62, 94, 2.1), (527, 4, 52, 70, 2)], 455)
        seat = [thermal[i] for i in ("th-trousers-seat-leg-air",) if i in thermal]
        restate(solids["trousers-seat"], seat, [(-72, 20, 97, 140, 2.3), (-40, 22, 113, 180, 2.4), (0, 17, 116, 184, 2.4), (50, 9, 114, 178, 2.4), (100, 5, 108, 168, 2.4)], -72)
    elif scale == 100:
        restate(solids["hoodie"], shells, [(440, 0, 100, 190, 2.8), (478, 4, 86, 174, 2.6), (497, 4, 76, 146, 2.4), (510, 4, 68, 116, 2.2), (524, 4, 56, 76, 2)], 440)
    elif scale == 200:
        restate(solids["hoodie"], shells, [(478, 13, 93, 168, 2.4), (506, 11, 82, 124, 2.2), (530, 8, 74, 80, 2)], 478)


def main():
    """🔨️ Applies the pass to the volume source."""
    document = json.load(open(SOURCE, encoding="utf-8"))
    for scale in (50, 100, 200):
        volume = document["volumes"][str(scale)]
        body(volume, scale)
        structure(volume, scale)
        print("[DEBUG]", scale, "body", len(volume["body"]), "thermal", len(volume["thermal"]), "structure", len(volume["structure"]))
    json.dump(document, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    open(SOURCE, "a", encoding="utf-8").write("\n")


main()
