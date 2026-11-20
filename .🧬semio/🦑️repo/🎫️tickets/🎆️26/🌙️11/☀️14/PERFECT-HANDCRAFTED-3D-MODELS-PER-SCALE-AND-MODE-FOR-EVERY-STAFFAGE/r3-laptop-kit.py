"""💻 Round 3, appliances/laptop: the measures of the real 14-inch aluminium clamshell and the helpers that write bone-local solids.

World frame: x ahead (towards the user, the screen faces +x), y up, z lateral (the user's left hand is at -z). Corrected rig: `base` starts at the rear edge of the chassis on the desk (x -0.109) and runs forward over its 0.218 m depth; `lid` turns about the axis of the hinge barrel, 4.5 mm in front of the rear edge and 2 mm above the deck, and runs to the free edge of the lid.
Bone-local frames: base u = x + 0.109, v = y; lid u along the lid from the hinge axis, v towards its aluminium back (closed lid: v = y - 0.0145), w lateral in both.
"""
import math

DEPTH, HALF_W = 0.218, 0.1565
CENTRE_U = DEPTH / 2
FOOT_H, SEAM_V, DECK_V = 0.002, 0.0085, 0.0125
LEDGE_U = 0.0095
PLAN_N = 34
WELL = (0.017, 0.121, 0.1395)
BED_V, KEY_V = 0.0110, 0.0122
KEY_ROWS = [(0.019, 0.0285), (0.0313, 0.0468), (0.0493, 0.0648), (0.0673, 0.0828), (0.0853, 0.1008), (0.1033, 0.1188)]
KEY_PITCH, KEY_GAP, KEY_UNITS = 0.019, 0.003, 14.5
PAD = (0.128, 0.206, 0.065)
PAD_V = 0.0123
FEET_U, FEET_W, FOOT_R = (0.020, 0.198), 0.134, 0.006
HINGE = (0.0045, 0.0145)
BARREL_R, BARREL_W = 0.0045, 0.135
LID_U0, LID_U1 = -0.0045, 0.2135
LID_CENTRE_U = (LID_U0 + LID_U1) / 2
LID_FACE_V, LID_BACK_V = -0.0014, 0.0045
GLASS = (0.011, 0.2105, 0.1535)
GLASS_V = (-0.0017, 0.0004)
SCREEN = (0.016, 0.2045, 0.1508)
SCREEN_V = (-0.0019, -0.0016)
OPEN_DEG = 105
BASE_X0 = -CENTRE_U
ALONG_V = [0.5, 0.5, 0.5, -0.5]
TONE = {"alu-anodised": 0.6, "display-glass": 0.1, "rubber-foot": 0.15, "keycap-plastic": 0.25, "hinge-steel": 0.4}


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(value + 0.0, 5)


def lid_world(u, v, deg=OPEN_DEG):
    """🌍 The world (x, y) of a lid-local point with the lid opened by `deg`."""
    a = math.radians(deg)
    return (BASE_X0 + HINGE[0] + u * math.cos(a) - v * math.sin(a), HINGE[1] + u * math.sin(a) + v * math.cos(a))


def superellipse(cu, rw, ru, exponent, steps):
    """🟢 The points (w, u) of the plan superellipse |w/rw|^n + |(u-cu)/ru|^n = 1, sampled like the kernel samples a loft ring."""
    e = 2 / exponent
    out = []
    for j in range(steps):
        t = 2 * math.pi * j / steps
        c, s = math.cos(t), math.sin(t)
        out.append((math.copysign(abs(c) ** e, c) * rw if abs(c) > 1e-12 else 0.0, cu + (math.copysign(abs(s) ** e, s) * ru if abs(s) > 1e-12 else 0.0)))
    return out


def half_width(cu, rw, ru, exponent, u):
    """↔ The half width of that superellipse at the depth u."""
    return rw * (1 - min(1.0, abs((u - cu) / ru)) ** exponent) ** (1 / exponent)


def clipped(points, u_min, cu, rw, ru, exponent):
    """✂ A plan outline (w, u) cut off straight at u_min: the run of points behind the cut is replaced by the two points on the cut."""
    w = half_width(cu, rw, ru, exponent, u_min)
    out, cut = [], False
    for p in points:
        if p[1] > u_min + 1e-9:
            out.append(p)
        elif not cut:
            out += [(-w, u_min), (w, u_min)]
            cut = True
    return out


def chamfered(w0, w1, u0, u1, c):
    """⬢ A rectangle (w, u) with its four corners cut by c."""
    return [(w0 + c, u0), (w1 - c, u0), (w1, u0 + c), (w1, u1 - c), (w1 - c, u1), (w0 + c, u1), (w0, u1 - c), (w0, u0 + c)]


def disc(cw, cu, radius, steps):
    """⚪ A regular polygon (w, u)."""
    return [(cw + radius * math.sin(2 * math.pi * k / steps), cu + radius * math.cos(2 * math.pi * k / steps)) for k in range(steps)]


def base(sid, kind, bone, material, tone=None, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}


def box(sid, bone, material, u, v, w, radius=0.0, tone=None, role="body", **more):
    """📦 A box between the bone-local extents u, v, w (pairs)."""
    out = base(sid, "box", bone, material, tone, role)
    out["size"] = [r6(u[1] - u[0]), r6(v[1] - v[0]), r6(w[1] - w[0])]
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": [r6((u[0] + u[1]) / 2), r6((v[0] + v[1]) / 2), r6((w[0] + w[1]) / 2)]}
    out.update(more)
    return out


def plate(sid, bone, material, outline_wu, v, holes=(), tone=None, role="body", **more):
    """🍽 A prism of a plan outline (w, u) pushed through the thickness v[0] to v[1] (plane zx)."""
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(w), r6(u)] for w, u in outline_wu], "plane": "zx", "from": r6(v[0]), "to": r6(v[1])})
    if holes:
        out["holes"] = [[[r6(w), r6(u)] for w, u in ring] for ring in holes]
    out.update(more)
    return out


def slab(sid, bone, material, cu, rows, exponent=PLAN_N, tone=None, role="body", **more):
    """🥞 A loft of plan superellipses stacked through the thickness: rows of (v, half width, half depth)."""
    out = base(sid, "loft", bone, material, tone, role)
    out["sections"] = [{"at": [r6(cu), r6(v), 0.0], "rx": r6(rw), "ry": r6(ru), "exponent": exponent, "rotation": ALONG_V} for v, rw, ru in rows]
    out.update({"capStart": True, "capEnd": True})
    out.update(more)
    return out


def axle(sid, bone, material, profile, origin=(0.0, 0.0, 0.0), tone=None, role="body", **more):
    """🛢 A body of revolution about the lateral axis w through `origin`: profile of (w, radius)."""
    out = base(sid, "revolve", bone, material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": [0, 0, 1], "origin": [r6(c) for c in origin]})
    out.update(more)
    return out


def stud(sid, bone, material, profile, at, tone=None, role="body", **more):
    """🔘 A body of revolution about the vertical axis v standing at `at`: profile of (height, radius)."""
    out = base(sid, "revolve", bone, material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": [0, 1, 0], "origin": [0, 0, 0], "transform": {"translation": [r6(c) for c in at]}})
    out.update(more)
    return out


def repeated(solid, count, step):
    """🔁 Copies a solid `count` times by the step (u, v, w)."""
    solid["repeat"] = {"count": count, "translation": [r6(c) for c in step]}
    return solid


def rig(old):
    """🦴 The corrected rig: the base bone lies under the chassis from its rear edge to its front edge, the lid bone turns about the axis of the hinge barrel."""
    spec = {
        "base": {"x": BASE_X0, "y": 0, "lengthM": DEPTH, "com": (CENTRE_U, 0.0074)},
        "lid": {"x": HINGE[0], "y": HINGE[1], "lengthM": LID_U1, "com": (0.1, 0.0008)},
    }
    bones = []
    for bone in old["bones"]:
        s = spec[bone["id"]]
        bone = dict(bone, x=s["x"], y=s["y"], lengthM=s["lengthM"])
        bone["mass"] = dict(bone["mass"], comX=s["com"][0], comY=s["com"][1])
        bones.append(bone)
    return dict(old, bones=bones)
