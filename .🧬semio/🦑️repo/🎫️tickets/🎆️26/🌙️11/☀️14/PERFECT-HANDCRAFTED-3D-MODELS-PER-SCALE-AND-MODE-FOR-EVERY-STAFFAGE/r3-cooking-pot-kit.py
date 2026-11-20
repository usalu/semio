"""🍲 Round 3 cooking pot: the real geometry of a 16 cm stainless stew pot (0.8 mm deep-drawn wall, 5 mm encapsulated sandwich base with an 8 mm base corner, 45 degree pouring rim, two riveted loop handles of 6 mm rod) and the solid writers every scale states its parts with.

World: x ahead (the handle axis), y up, z left, origin = centre of the base on the hob. Every number is metres.
Bone frames (u along the bone, v its normal, w lateral): pot, water and surface point up, so local = (Y - origin, -X, Z); handles spans from the left grip to the right grip, so local = (X + 0.125, Y - GRIP_Y, Z).
"""
import math

RI = 0.08
T = 0.0008
RO = RI + T
H = 0.1
RIM = 0.086
LIP_Y = 0.095
RIM_IN = RI * RIM / RO
FLOOR = 0.005
CAP_R = 0.075
EDGE_R = 0.0785
KNEE_Y = 0.0125
FOOT_R = RI * EDGE_R / RO
FILL = 0.055
BOB = 0.004

GRIP_Y = 0.086
GRIP_X = 0.122
ROD = 0.003
BEND = 0.01
BEND_Z = 0.012
ARM_Z = BEND_Z + BEND
REACH = 0.125
PAD_T = 0.0025
PAD_H = 0.018
PAD_W = 0.012
PAD_Y = 0.075
RIVET_Y = 0.071
ARM_Y = 0.08
PHI = math.atan2(0.02, math.sqrt(RO * RO - 0.02 * 0.02))

ORIGIN = {"pot": 0.0, "water": FLOOR, "surface": FILL}

BUBBLES = [("a", -0.03, 0.02, 0.006), ("b", 0.025, 0.03, 0.005), ("c", 0.0, -0.02, 0.007), ("d", 0.04, -0.015, 0.005), ("e", -0.04, -0.03, 0.005),
           ("f", 0.014, 0.006, 0.008), ("g", -0.012, 0.048, 0.0045), ("h", 0.052, 0.02, 0.004), ("i", -0.055, 0.002, 0.0045)]


def r5(x):
    return round(x, 5) + 0.0


def v3(p):
    return [r5(p[0]), r5(p[1]), r5(p[2])]


def outer_r(y):
    """🥘 Outer radius of the vessel at the height y: the bevelled edge of the sandwich base, the base corner, the straight wall, the 45 degree pouring rim."""
    if y <= FLOOR:
        return CAP_R + (EDGE_R - CAP_R) * y / FLOOR
    if y < KNEE_Y:
        return EDGE_R + (RO - EDGE_R) * (y - FLOOR) / (KNEE_Y - FLOOR)
    if y <= LIP_Y:
        return RO
    return RO + (RIM - RO) * (y - LIP_Y) / (H - LIP_Y)


def up(bone, p):
    """🧭 A world point in the frame of an upright bone (pot, water, surface)."""
    return (p[1] - ORIGIN[bone], -p[0], p[2])


def span(p):
    """🧭 A world point in the frame of the handles bone."""
    return (p[0] + REACH, p[1] - GRIP_Y, p[2])


def norm(v):
    n = math.sqrt(sum(c * c for c in v))
    return tuple(c / n for c in v)


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def quat_from_axes(x, y, z):
    """🧭 Unit quaternion [x, y, z, w] of the rotation whose columns are the given orthonormal axes."""
    m00, m01, m02 = x[0], y[0], z[0]
    m10, m11, m12 = x[1], y[1], z[1]
    m20, m21, m22 = x[2], y[2], z[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = ((m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s)
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        q = (0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s)
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        q = ((m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s)
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        q = ((m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s)
    n = math.sqrt(sum(c * c for c in q))
    return [round(c / n, 6) + 0.0 for c in q]


def quat_y(angle):
    """🧭 Rotation about the vertical by angle radians (x turns towards -z)."""
    return [0.0, round(math.sin(angle / 2), 6) + 0.0, 0.0, round(math.cos(angle / 2), 6) + 0.0]


def base(sid, kind, bone, material, role, tone):
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def circle(r, n, phase=0.0):
    return [(r * math.cos(phase + 2 * math.pi * k / n), r * math.sin(phase + 2 * math.pi * k / n)) for k in range(n)]


def disc(sid, bone, material, role, tone, y0, y1, r, n, hole=None, r_top=None, **extra):
    """⭕ A round plate or ring about the vertical axis between the world heights y0 and y1 on an upright bone: an n-sided prism (plane yz of the bone frame, pushed along u), with a bore `hole` and a conical flare to `r_top`."""
    s = base(sid, "extrude", bone, material, role, tone)
    s.update({"profile": [[r5(a), r5(b)] for a, b in circle(r, n)], "plane": "yz", "from": r5(y0 - ORIGIN[bone]), "to": r5(y1 - ORIGIN[bone])})
    if hole is not None:
        s["holes"] = [[[r5(a), r5(b)] for a, b in circle(hole, n)]]
    if r_top is not None:
        s["taperTo"] = [1, round(r_top / r, 6)]
    s.update(extra)
    return s


def ellipsoid_up(sid, bone, material, role, tone, at, r_up, r_side, **extra):
    """🫧 An ellipsoid of revolution about the vertical on an upright bone, centred on the world point at; `fine` states it ten times larger and scales it down, so that the kernel gives the small round its full segment count."""
    fine = extra.pop("fine", False)
    k = 10 if fine else 1
    s = base(sid, "ellipsoid", bone, material, role, tone)
    s["radii"] = [r5(r_up * k), r5(r_side * k), r5(r_side * k)]
    s["transform"] = {"translation": v3(up(bone, at))}
    if fine:
        s["transform"] = {"scale": [0.1, 0.1, 0.1], "translation": v3(up(bone, at))}
    s.update(extra)
    return s


def skin(sid, bone, material, role, tone, rings, **extra):
    """🧻 An open skin of revolution about the vertical through the rings (world height, radius), without caps; the sections are stated ten times wider and scaled back across the axis, so that the kernel gives the skin its full segment count and it stays as round as the prisms it lies on."""
    s = base(sid, "loft", bone, material, role, tone)
    s.update({"sections": [{"at": v3((y - ORIGIN[bone], 0.0, 0.0)), "rx": r5(10 * r), "ry": r5(10 * r)} for y, r in rings], "capStart": False, "capEnd": False})
    s["transform"] = {"scale": [1, 0.1, 0.1]}
    s.update(extra)
    return s


def bow_path(side, bend_steps, padded=True):
    """🧲 Centre line of one loop handle (side +1 right, -1 left) as world points with unit tangents: from the riveting pad on the wall (or, without pads, from the wall itself) along the far arm, round the bend, along the grip, round the second bend and back along the near arm."""
    r0 = RO + PAD_T + 0.0003 if padded else RO + 0.0001
    start = (r0 * math.cos(PHI), ARM_Y, r0 * math.sin(PHI))
    mid = (0.104, 0.0845, 0.0215)
    half = [(start, norm(tuple(b - a for a, b in zip(start, mid)))), (mid, norm((0.0315, 0.0052, 0.0013)))]
    for k in range(bend_steps + 1):
        a = math.pi / 2 * k / bend_steps
        half.append(((GRIP_X - BEND + BEND * math.sin(a), GRIP_Y, BEND_Z + BEND * math.cos(a)), (math.cos(a), 0.0, -math.sin(a))))
    back = [((p[0], p[1], -p[2]), (-t[0], -t[1], t[2])) for p, t in reversed(half)]
    path = half + back
    return [((side * p[0], p[1], p[2]), (side * t[0], t[1], t[2])) for p, t in path]


def bow(sid, side, tone, bend_steps, padded=True, r=ROD):
    """🧲 One loop handle of round rod as a loft in the handles frame; every section is square to the centre line with its x axis towards the vertical."""
    s = base(sid, "loft", "handles", "stainless", "body", tone)
    sections = []
    for p, t in bow_path(side, bend_steps, padded):
        x = norm(tuple(c - dot((0.0, 1.0, 0.0), t) * tc for c, tc in zip((0.0, 1.0, 0.0), t)))
        o = {"at": v3(span(p)), "rx": r5(r), "ry": r5(r), "rotation": quat_from_axes(x, cross(t, x), t)}
        sections.append(o)
    s.update({"sections": sections, "capStart": True, "capEnd": True})
    return s


def azimuths():
    """🧭 The four riveting points: id suffix, azimuth about the vertical (0 = ahead, towards +z positive)."""
    return [("right-far", PHI), ("right-near", -PHI), ("left-far", math.pi - PHI), ("left-near", math.pi + PHI)]


def on_wall(radius, y, azimuth):
    return (radius * math.cos(azimuth), y, radius * math.sin(azimuth))


def pad(sid, azimuth, tone):
    """🔩 The forged flat end of the rod lying on the wall: a 12 x 18 x 2.5 mm plate with its thickness along the wall normal."""
    s = base(sid, "box", "handles", "stainless", "body", tone)
    s["size"] = [PAD_T, PAD_H, PAD_W]
    s["cornerRadius"] = 0.001
    s["transform"] = {"translation": v3(span(on_wall(RO + PAD_T / 2 + 0.0002, PAD_Y, azimuth))), "rotation": quat_y(-azimuth)}
    return s


def rivet(sid, azimuth, tone, inside):
    """🔩 A rivet head on the wall normal: the 8 mm dome on the pad outside, or the 6 mm set head inside the pot."""
    s = base(sid, "ellipsoid", "handles", "stainless", "detail", tone)
    s["radii"] = [0.00125, 0.003, 0.003] if inside else [0.0018, 0.004, 0.004]
    s["transform"] = {"translation": v3(span(on_wall(RI - 0.0001 if inside else RO + PAD_T + 0.0002, RIVET_Y, azimuth))), "rotation": quat_y(-azimuth)}
    return s


def lug(sid, side, tone, y0, y1, outline):
    """🧲 A handle generalised to a flat lug: the plan outline (world x, z of the right handle, mirrored for the left) pushed through the heights y0..y1 in the handles frame."""
    s = base(sid, "extrude", "handles", "stainless", "body", tone)
    pts = [(z, side * x + REACH) for x, z in outline]
    if side < 0:
        pts.reverse()
    s.update({"profile": [[r5(a), r5(b)] for a, b in pts], "plane": "zx", "from": r5(y0 - GRIP_Y), "to": r5(y1 - GRIP_Y)})
    return s


def of(sid, target, **extra):
    s = {"id": sid, "of": target}
    s.update(extra)
    return s


def text(en, de):
    return {"en": en, "de": de}
