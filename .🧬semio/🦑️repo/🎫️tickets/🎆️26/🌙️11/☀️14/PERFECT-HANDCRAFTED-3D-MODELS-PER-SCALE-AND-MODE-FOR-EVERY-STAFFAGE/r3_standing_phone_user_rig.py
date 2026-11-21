"""🦴 The corrected rig of the standing phone user (round 3) and its forward kinematics: the same chain the product solves (`🔨️modules/🦴️rig`: sagittal angle about w, then spread about the bone normal, then twist about the bone axis), so that the texting grip can be stated and checked in numbers before any solid is modelled.

Joint heights shod on a 20 mm sole, stature 1.69 m: ankle 0.086, knee 0.490, hip joint 0.903, lumbosacral joint 0.998, shoulder joint 1.3975, C7 1.465, atlanto-occipital joint 1.561, vertex 1.710.
"""
import math

BONES = [
    ("pelvis", None, 0, 0.903, 0, 90, 0.0951),
    ("torso", "pelvis", 0.0951, 0.035, 0, -1.84, 0.46709),
    ("neck", "torso", 0.46709, 0, 0, -17.228, 0.10163),
    ("head", "neck", 0.10163, 0, 0, 19.068, 0.149),
    ("arm-l", "torso", 0.40033, -0.02216, 0.153, 181.84, 0.3135),
    ("forearm-l", "arm-l", 0.3135, 0, 0, 0, 0.247),
    ("hand-l", "forearm-l", 0.247, 0, 0, 0, 0.1805),
    ("thigh-l", "pelvis", 0, 0, 0.085, 180, 0.41325),
    ("shank-l", "thigh-l", 0.41325, 0, 0, 0, 0.40375),
    ("foot-l", "shank-l", 0.40375, 0, 0, 60.133, 0.17251),
    ("toe-l", "foot-l", 0.17251, 0, 0, 29.867, 0.0572),
    ("arm-r", "torso", 0.40033, -0.02216, -0.153, 181.84, 0.3135),
    ("forearm-r", "arm-r", 0.3135, 0, 0, 0, 0.247),
    ("hand-r", "forearm-r", 0.247, 0, 0, 0, 0.1805),
    ("thigh-r", "pelvis", 0, 0, -0.085, 180, 0.41325),
    ("shank-r", "thigh-r", 0.41325, 0, 0, 0, 0.40375),
    ("foot-r", "shank-r", 0.40375, 0, 0, 60.133, 0.17251),
    ("toe-r", "foot-r", 0.17251, 0, 0, 29.867, 0.0572),
    ("phone", "hand-r", 0.073, -0.038, 0.024, 40, 0.147),
    ("ponytail", "head", 0.044, 0.104, 0, 180, 0.075),
]


def comb(a, s, b, t):
    return tuple(a[i] * s + b[i] * t for i in range(3))


def chain(offsets=None, bones=BONES):
    """⛓ Frames (o, u, v, w) of every bone for pose offsets {bone: (angleDeg, abductionDeg, twistDeg)}."""
    offsets = offsets or {}
    out = {}
    for bid, parent, x, y, z, angle, _length in bones:
        p = out[parent] if parent else {"o": (0, 0, 0), "u": (1, 0, 0), "v": (0, 1, 0), "w": (0, 0, 1)}
        a, b, t = (tuple(offsets.get(bid, (0, 0, 0))) + (0, 0))[:3]
        o = tuple(p["o"][i] + p["u"][i] * x + p["v"][i] * y + p["w"][i] * z for i in range(3))
        r = math.radians(angle + a)
        u1, v1 = comb(p["u"], math.cos(r), p["v"], math.sin(r)), comb(p["u"], -math.sin(r), p["v"], math.cos(r))
        f = math.radians(b)
        u2, w2 = comb(u1, math.cos(f), p["w"], math.sin(f)), comb(u1, -math.sin(f), p["w"], math.cos(f))
        k = math.radians(t)
        out[bid] = {"o": o, "u": u2, "v": comb(v1, math.cos(k), w2, -math.sin(k)), "w": comb(v1, math.sin(k), w2, math.cos(k))}
    return out


def at(frame, x, y=0.0, z=0.0):
    return tuple(frame["o"][i] + frame["u"][i] * x + frame["v"][i] * y + frame["w"][i] * z for i in range(3))
