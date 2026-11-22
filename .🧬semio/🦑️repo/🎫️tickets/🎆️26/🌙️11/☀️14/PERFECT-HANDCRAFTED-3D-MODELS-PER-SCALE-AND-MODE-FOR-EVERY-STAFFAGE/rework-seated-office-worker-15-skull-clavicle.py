"""💀 Stage 15 of the seated office worker: a skull with sockets at 1:50 and the clavicle that rises with the raised arm.

Skull (structure, 1:50): the face was one block in front of the cranium. It is now the maxilla below the orbits, the nasal bridge between them, the two zygomatic bones with their arches back to the temporal bone and the supraorbital bar above, so the orbits are real recesses whose floor is the cranium.

Pose `leaning-back`: the left hand lies behind the head with the elbow out; the clavicle now rises 18 degrees (scapulothoracic share of the elevation) and the offsets of the upper arm are re-solved so that the arm keeps its posed direction.

Run: `python3 rework-seated-office-worker-15-skull-clavicle.py` (candidate), `--apply` writes catalog pose and volumes source.
"""
import math
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
LIFT = 18.0


def rz(d):
    """🌀 Turn about w: u towards v."""
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def ry(d):
    """🌀 Turn about v, standard right-handed sense."""
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[c, 0, s], [0, 1, 0], [-s, 0, c]]


def rx(d):
    """🌀 Turn about u, standard right-handed sense."""
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[1, 0, 0], [0, c, -s], [0, s, c]]


def mul(*ms):
    """✖️ Product of 3x3 matrices."""
    out = ms[0]
    for m in ms[1:]:
        out = [[sum(out[i][k] * m[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    return out


def tr(m):
    """🔁 Transpose."""
    return [[m[j][i] for j in range(3)] for i in range(3)]


def rest(angle, abduction, twist):
    """🦴 Rest or offset rotation of a bone: Rw(angle) Rv(abduction) Ru(twist) in the rig's senses."""
    return mul(rz(angle), ry(-abduction), rx(-twist))


def solve(m, near):
    """🧮 The (angle, abduction, twist) of a rotation, on the branch nearest to the given triple."""
    beta = math.degrees(math.asin(max(-1, min(1, -m[2][0]))))
    a = math.degrees(math.atan2(m[1][0], m[0][0]))
    g = math.degrees(math.atan2(m[2][1], m[2][2]))
    wrap = lambda v: (v + 180) % 360 - 180
    options = [(a, -beta, -g), (wrap(a + 180), -wrap(180 - beta), -wrap(g + 180))]
    return min(options, key=lambda o: sum(abs(wrap(x - y)) for x, y in zip(o, near)))


doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
v = a["volumes"]
bones = {b["id"]: b for b in a["rig"]["bones"]}
arm = bones["arm-l"]
ra = rest(arm["angleDeg"], arm.get("abductionDeg", 0), arm.get("twistDeg", 0))
pose = next(p for p in a["poses"] if p["id"] == "leaning-back")
if "clavicle-l" not in pose["bones"]:
    old = pose["bones"]["arm-l"]
    triple = (old["angleDeg"], old.get("abductionDeg", 0), old.get("twistDeg", 0))
    x = mul(tr(ra), tr(rz(LIFT)), ra, rest(*triple))
    new = solve(x, triple)
    assert all(abs(p - q) < 1e-9 for r1, r2 in zip(rest(*new), x) for p, q in zip(r1, r2))
    pose["bones"]["clavicle-l"] = {"angleDeg": LIFT}
    pose["bones"]["arm-l"] = {"angleDeg": rnd(new[0], 3), "abductionDeg": rnd(new[1], 3), "twistDeg": rnd(new[2], 3)}
    print("[DEBUG] arm-l", triple, "->", pose["bones"]["arm-l"])

BONE = dict(member="head", part="member")
structure = [s for s in v["50"]["structure"] if s["id"] not in ("face-bones", "maxilla", "nasal-bridge", "supraorbital", "zygomatic-l", "zygomatic-r", "zygomatic-arch-l", "zygomatic-arch-r")]
at = next(i for i, s in enumerate(structure) if s["id"] == "mandible")
skull = [
    loft("maxilla", "head", "bone", 0.56, [(-0.054, -0.066, 0, 0.028, 0.026), (-0.046, -0.067, 0, 0.034, 0.03), (-0.03, -0.068, 0, 0.042, 0.031), (-0.014, -0.068, 0, 0.05, 0.03), (-0.006, -0.067, 0, 0.047, 0.026)], **BONE),
    loft("nasal-bridge", "head", "bone", 0.56, [(-0.012, -0.082, 0, 0.0125, 0.016), (0.006, -0.083, 0, 0.011, 0.015), (0.022, -0.082, 0, 0.0105, 0.013), (0.036, -0.08, 0, 0.013, 0.011)], **BONE),
    loft("supraorbital", "head", "bone", 0.6, [(0.03, -0.062, -0.062, 0.007, 0.007), (0.037, -0.079, -0.044, 0.0075, 0.0075), (0.039, -0.086, -0.022, 0.0075, 0.0075), (0.036, -0.088, 0, 0.008, 0.008), (0.039, -0.086, 0.022, 0.0075, 0.0075), (0.037, -0.079, 0.044, 0.0075, 0.0075), (0.03, -0.062, 0.062, 0.007, 0.007)], across=False, **BONE),
]
for side, z in (("l", 1), ("r", -1)):
    skull.append(ell(f"zygomatic-{side}", "head", "bone", 0.56, (0.014, 0.012, 0.012), (-0.004, -0.076, 0.05 * z), **BONE))
    skull.append(loft(f"zygomatic-arch-{side}", "head", "bone", 0.6, [(-0.002, -0.07, 0.056 * z, 0.006, 0.006), (0.0, -0.045, 0.066 * z, 0.005, 0.005), (0.002, -0.018, 0.068 * z, 0.005, 0.005), (0.004, 0.0, 0.064 * z, 0.006, 0.006)], across=False, **BONE))
    skull.append(loft(f"orbital-rim-{side}", "head", "bone", 0.6, [(-0.004, -0.076, 0.05 * z, 0.007, 0.007), (0.014, -0.074, 0.056 * z, 0.0065, 0.0065), (0.03, -0.062, 0.062 * z, 0.007, 0.007)], across=False, **BONE))
structure = [s for s in structure if not s["id"].startswith("orbital-rim-")]
structure[at:at] = skull
v["50"]["structure"] = structure
write(candidate_path(A), doc)
print("[DEBUG] candidate", candidate_path(A))
if "--apply" in sys.argv:
    live = read(catalog_path(A))
    live["asset"]["poses"] = a["poses"]
    write(catalog_path(A), live)
    write(source_path(A), {"volumes": v})
    print("applied")
