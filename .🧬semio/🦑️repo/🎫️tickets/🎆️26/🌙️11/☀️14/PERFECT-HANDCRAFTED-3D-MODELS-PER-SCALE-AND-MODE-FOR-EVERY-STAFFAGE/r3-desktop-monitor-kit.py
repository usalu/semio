"""🖥 Round 3, appliances/desktop-monitor: the measures of the real 23.8 to 24 inch fixed-stand monitor and the helpers that write bone-local solids.

World frame (rest pose = default pose "on"): x ahead (the screen faces +x), y up, z lateral (left of the asset is +). Corrected rig: `base` lies on the desk from the rear edge of the plate (x -0.096) forward over its 0.21 m depth; `neck` stands up from the plate top at x -0.06; `panel` hangs on the tilt hinge at the neck tip (x -0.06, y 0.255), its u axis runs up the screen tilted back by 5 degrees, its v axis backwards, w lateral.
Bone-local frames: base u = x + 0.096, v = y; neck u = y - 0.012, v = -(x + 0.06); panel u, v by the 95 degree world angle.
"""
import math

PANEL_U0, PANEL_U1, PANEL_W = -0.17, 0.153, 0.27
FRONT_V, MODULE_V = -0.040, -0.0305
GLASS_V = -0.0393
GLASS_U0, GLASS_U1, GLASS_W = -0.151, 0.1455, 0.2635
HOUSING_V1 = -0.004
HINGE = (-0.06, 0.255)
NECK_X, NECK_Y = -0.06, 0.012
NECK_LEN = 0.243
BASE_X0, BASE_DEPTH, BASE_HALF_W = -0.096, 0.21, 0.125
BASE_TOP = 0.0122
FOOT_H = 0.002
TILT = math.radians(95)
ORIGIN = {"base": (BASE_X0, 0.0), "neck": (NECK_X, NECK_Y), "panel": HINGE}
TONE = {"bezel-plastic": 0.18, "display-glass": 0.08, "housing-plastic": 0.55, "stand-plastic": 0.42, "rubber-foot": 0.12}


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(value + 0.0, 5)


def quat(cx, cy, cz):
    """🧮 The unit quaternion [x, y, z, w] of the rotation whose matrix columns are the images of the x, y and z axes."""
    m = [[cx[0], cy[0], cz[0]], [cx[1], cy[1], cz[1]], [cx[2], cy[2], cz[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1.0) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s]
    return [r6(c) for c in q]


SECTION_ALONG_V = quat((0, 0, 1), (1, 0, 0), (0, 1, 0))
SECTION_ALONG_U = quat((0, 0, 1), (0, -1, 0), (1, 0, 0))


def about_w(angle):
    """🔄 The quaternion of a turn by `angle` radians about the bone's w axis."""
    return [0.0, 0.0, r6(math.sin(angle / 2)), r6(math.cos(angle / 2))]


def base(sid, kind, bone, material, tone=None, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}


def box(sid, bone, material, u, v, w, radius=0.0, tone=None, role="body", rotation=None, **more):
    """📦 A box between the bone-local extents u, v, w (pairs)."""
    out = base(sid, "box", bone, material, tone, role)
    out["size"] = [r6(u[1] - u[0]), r6(v[1] - v[0]), r6(w[1] - w[0])]
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": [r6((u[0] + u[1]) / 2), r6((v[0] + v[1]) / 2), r6((w[0] + w[1]) / 2)]}
    if rotation is not None:
        out["transform"]["rotation"] = rotation
    out.update(more)
    return out


def face(sid, bone, material, outline_wu, v, holes=(), tone=None, role="body", **more):
    """🪟 A prism of a (w, u) outline pushed through the depth v[0] to v[1] (plane zx)."""
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(w), r6(u)] for w, u in outline_wu], "plane": "zx", "from": r6(v[0]), "to": r6(v[1])})
    if holes:
        out["holes"] = [[[r6(w), r6(u)] for w, u in ring] for ring in holes]
    out.update(more)
    return out


def loft(sid, bone, material, sections, rotation, tone=None, role="body", **more):
    """🧵 A capped loft of superelliptic sections given as (at, rx, ry, exponent) with one absolute rotation."""
    out = base(sid, "loft", bone, material, tone, role)
    out["sections"] = [{"at": [r6(c) for c in at], "rx": r6(rx), "ry": r6(ry), "exponent": n, "rotation": rotation} for at, rx, ry, n in sections]
    out.update({"capStart": True, "capEnd": True})
    out.update(more)
    return out


def along_v(sid, bone, material, rows, tone=None, role="body", **more):
    """🧵 A loft through the depth v: rows (v, centre u, half height ry along u, half width rx along w, exponent)."""
    return loft(sid, bone, material, [((uc, v, 0.0), rx, ry, n) for v, uc, ry, rx, n in rows], SECTION_ALONG_V, tone, role, **more)


def along_u(sid, bone, material, rows, tone=None, role="body", **more):
    """🧵 A loft along the bone u: rows (u, centre v, half depth ry along v, half width rx along w, exponent)."""
    return loft(sid, bone, material, [((u, vc, 0.0), rx, ry, n) for u, vc, ry, rx, n in rows], SECTION_ALONG_U, tone, role, **more)


def turned(sid, bone, material, profile, axis, origin, tone=None, role="body", **more):
    """⭕ A revolve of an (axial, radial) profile about a bone-local axis through a bone-local origin."""
    out = base(sid, "revolve", bone, material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": axis, "origin": [r6(c) for c in origin]})
    out.update(more)
    return out


def blob(sid, bone, material, centre, radii, tone=None, role="body", **more):
    """🥚 An ellipsoid at a bone-local centre with semi-axes along u, v, w."""
    out = base(sid, "ellipsoid", bone, material, tone, role)
    out.update({"radii": [r6(r) for r in radii], "transform": {"translation": [r6(c) for c in centre]}})
    out.update(more)
    return out


def repeat(solid, count, du=0.0, dv=0.0, dw=0.0):
    """🔁 Copies a solid `count` times along a bone-local step."""
    solid["repeat"] = {"count": count, "translation": [r6(du), r6(dv), r6(dw)]}
    return solid


def rounded(a, b, radius, steps=4):
    """🔲 A counter-clockwise rectangle a = (a0, a1) by b = (b0, b1) with one corner radius."""
    if radius <= 0:
        return [(a[0], b[0]), (a[1], b[0]), (a[1], b[1]), (a[0], b[1])]
    points = []
    for ca, cb, start in ((a[1] - radius, b[0] + radius, 270), (a[1] - radius, b[1] - radius, 0), (a[0] + radius, b[1] - radius, 90), (a[0] + radius, b[0] + radius, 180)):
        for k in range(steps + 1):
            angle = math.radians(start + 90 * k / steps)
            points.append((ca + radius * math.cos(angle), cb + radius * math.sin(angle)))
    return points


def superellipse(cx, cy, rx, ry, exponent, steps=48):
    """🟢 A counter-clockwise polygon of the superellipse |x/rx|^n + |y/ry|^n = 1 about (cx, cy), sampled like the kernel's loft rings."""
    e = 2 / exponent
    out = []
    for k in range(steps):
        t = 2 * math.pi * k / steps
        c, s = math.cos(t), math.sin(t)
        out.append((cx + rx * math.copysign(abs(c) ** e, c), cy + ry * math.copysign(abs(s) ** e, s)))
    return out


def panel_world(u, v):
    """🌍 A panel-local point in the rest world (x ahead, y up)."""
    return (HINGE[0] + u * math.cos(TILT) - v * math.sin(TILT), HINGE[1] + u * math.sin(TILT) + v * math.cos(TILT))


def neck_local(x, y):
    """📐 A world point in the neck frame (u up, v backwards)."""
    return (y - NECK_Y, -(x - NECK_X))


def base_local(x, y):
    """📐 A world point in the base frame (u ahead, v up)."""
    return (x - BASE_X0, y)


def rig(old):
    """🦴 The corrected rig: the base bone runs over the plate from its rear edge, the neck stands upright to the tilt hinge, the panel hangs on the hinge 5 degrees back from the neck and ends at the top edge; masses after the Dell SE2422H split (panel 84 %, stand 16 % of the total)."""
    spec = {
        "base": dict(x=BASE_X0, y=0.0, angleDeg=0, lengthM=BASE_DEPTH, kg=0.4, com=(0.105, 0.006)),
        "neck": dict(x=NECK_X - BASE_X0, y=NECK_Y, angleDeg=90, lengthM=NECK_LEN, kg=0.2, com=(0.12, 0.008)),
        "panel": dict(x=NECK_LEN, y=0.0, angleDeg=5, lengthM=PANEL_U1, kg=3.0, com=(-0.01, -0.024)),
    }
    bones = []
    for bone in old["bones"]:
        s = spec[bone["id"]]
        bone = dict(bone, x=r6(s["x"]), y=r6(s["y"]), angleDeg=s["angleDeg"], lengthM=s["lengthM"])
        bone["mass"] = dict(bone["mass"], kg=s["kg"], comX=s["com"][0], comY=s["com"][1])
        bones.append(bone)
    return dict(old, bones=bones)
