"""🍳 Round 3, appliances/kitchen-extractor-hood: the dimensions of the 0.9 m pyramid chimney hood (reference: Bosch DWP96BC50B data sheet) and the small solid builders shared by the model and the root script.

Frame of every bone (all bones have angle 0): u = out from the wall, v = up, w = lateral (left is +). canopy origin = rim underside on the wall, chimney origin = canopy top plate on the wall, chimney-top origin = lip of the lower tube on the wall.
"""
import math

HALF_W = 0.45
DEPTH = 0.5
RIM = 0.045
TOP = 0.246
TOP_D = 0.2
TOP_HW = 0.13
FLANGE = 0.012
SLOPE_F = math.hypot(DEPTH - TOP_D, TOP - RIM)
SLOPE_S = math.hypot(HALF_W - TOP_HW, TOP - RIM)
ANGLE_F = math.atan2(TOP - RIM, -(DEPTH - TOP_D))
ANGLE_S = -math.atan2(HALF_W - TOP_HW, TOP - RIM)
LOWER_HW = 0.125
LOWER_D = 0.19
LOWER_H = 0.4
UPPER_HW = 0.122
UPPER_D = 0.187
UPPER_H = 0.254
UPPER_HIDDEN = 0.146
SHEET_TUBE = 0.0006
SHEET_CANOPY = 0.0006
DUCT_X = 0.085
DUCT_R = 0.075
STUB = 0.015
CASSETTE_W = 0.28
CASSETTE_U = (0.04, 0.36)
CASSETTE_C = (-0.288, 0.0, 0.288)
LAMP_C = (-0.2, 0.2)
LAMP_U = (0.4, 0.45)
LAMP_HW = 0.06
PANEL_V = 0.006
HOOK_V = 0.19
REST_V = 0.03
SCREW_HW = 0.135
FAN_AT = (0.105, 0.135, 0.0)


def r5(x):
    """🔢 Five decimals, no negative zero."""
    v = round(float(x), 5)
    return 0.0 if v == 0 else v


def pts(seq):
    """📍 A rounded point list."""
    return [[r5(a), r5(b)] for a, b in seq]


def rect(a0, a1, b0, b1):
    """⬜ A rectangle ring."""
    return pts([(a0, b0), (a1, b0), (a1, b1), (a0, b1)])


def solid(kind, sid, bone, material, role, tone, **kw):
    """🧱 The common members of a solid in schema order."""
    out = {"id": sid, "bone": bone, "material": material, "role": role, "tone": tone, "kind": kind}
    out.update(kw)
    return out


def ext(sid, bone, material, role, tone, plane, profile, a, b, holes=None, **kw):
    """📐 An extrude."""
    out = solid("extrude", sid, bone, material, role, tone, profile=pts(profile), plane=plane, **{"from": r5(a), "to": r5(b)})
    if holes:
        out["holes"] = [pts(h) for h in holes]
    out.update(kw)
    return out


def box(sid, bone, material, role, tone, lo, hi, **kw):
    """📦 A box between two corners."""
    size = [r5(hi[i] - lo[i]) for i in range(3)]
    at = [r5((hi[i] + lo[i]) / 2) for i in range(3)]
    out = solid("box", sid, bone, material, role, tone, size=size, transform={"translation": at})
    out.update(kw)
    return out


def rev(sid, bone, material, role, tone, profile, origin, axis, **kw):
    """🌀 A revolve."""
    out = solid("revolve", sid, bone, material, role, tone, profile=pts(profile), axis=axis, origin=[r5(c) for c in origin])
    out.update(kw)
    return out


def ell(sid, bone, material, role, tone, radii, at, **kw):
    """🥚 An ellipsoid."""
    out = solid("ellipsoid", sid, bone, material, role, tone, radii=[r5(c) for c in radii], transform={"translation": [r5(c) for c in at]})
    out.update(kw)
    return out


def quat(axis, angle):
    """🧭 A unit quaternion [x, y, z, w] about a coordinate axis."""
    s, c = math.sin(angle / 2), math.cos(angle / 2)
    return [round(s if axis == i else 0.0, 7) for i in range(3)] + [round(c, 7)]


def slope_front(sid, bone, material, role, tone, t, **kw):
    """🔺 The front slope sheet: a trapezoid in its own plane, t thick inwards, hinged on the upper edge of the rim."""
    out = ext(sid, bone, material, role, tone, "zx", [(-HALF_W, 0), (HALF_W, 0), (TOP_HW, SLOPE_F), (-TOP_HW, SLOPE_F)], 0, t, **kw)
    out["transform"] = {"translation": [DEPTH, RIM, 0.0], "rotation": quat(2, ANGLE_F)}
    return out


def slope_side(sid, bone, material, role, tone, t, side, **kw):
    """🔻 A side slope sheet (side +1 = left): a right trapezoid in its own plane, t thick inwards."""
    a, b = (-t, 0) if side > 0 else (0, t)
    out = ext(sid, bone, material, role, tone, "xy", [(0, 0), (DEPTH, 0), (TOP_D, SLOPE_S), (0, SLOPE_S)], a, b, **kw)
    out["transform"] = {"translation": [0.0, RIM, r5(side * HALF_W)], "rotation": quat(0, side * ANGLE_S)}
    return out


def back_plate(sid, bone, material, role, tone, t, **kw):
    """🧱 The back sheet on the wall above the rim."""
    return ext(sid, bone, material, role, tone, "yz", [(RIM, -HALF_W), (RIM, HALF_W), (TOP, TOP_HW), (TOP, -TOP_HW)], 0, t, **kw)


def top_plate(sid, bone, material, role, tone, t, **kw):
    """⬛ The top plate that carries the fan unit and the chimney."""
    return ext(sid, bone, material, role, tone, "zx", rect(-TOP_HW, TOP_HW, 0, TOP_D), TOP - t, TOP, **kw)


def channel(hw, d, t, lip=0.0):
    """🇺 The U section of a chimney tube in (w, u): front and two sides, open to the wall, with return lips at the wall edges."""
    if lip <= 0:
        return [(-hw, 0), (-hw, d), (hw, d), (hw, 0), (hw - t, 0), (hw - t, d - t), (-hw + t, d - t), (-hw + t, 0)]
    return [(-hw + lip, 0), (-hw, 0), (-hw, d), (hw, d), (hw, 0), (hw - lip, 0), (hw - lip, t), (hw - t, t), (hw - t, d - t), (-hw + t, d - t), (-hw + t, t), (-hw + lip, t)]


def tube(sid, bone, material, role, tone, r, t, v0, v1, **kw):
    """🚇 A round duct wall about the vertical axis through the duct centre."""
    loop = [(v0, r - t), (v1, r - t), (v1, r), (v0, r), (v0, r - t)]
    return rev(sid, bone, material, role, tone, loop, [DUCT_X, 0, 0], [0, 1, 0], **kw)


def disc(sid, bone, material, role, tone, r, v0, v1, at=(DUCT_X, 0, 0), axis=(0, 1, 0), **kw):
    """⏺️ A filled disc or cylinder."""
    return rev(sid, bone, material, role, tone, [(v0, 0), (v0, r), (v1, r), (v1, 0)], list(at), list(axis), **kw)


def ref(sid, of, **kw):
    """🔗 A reference to a body solid."""
    out = {"id": sid, "of": of}
    out.update(kw)
    return out


def th(s, region, layer=None, **kw):
    """🌡️ A thermal solid from a solid."""
    out = dict(s)
    out["region"] = region
    if layer:
        out["layer"] = layer
    out.update(kw)
    return out


def st(s, member, part, **kw):
    """🏗️ A structural solid from a solid."""
    out = dict(s)
    out["member"] = member
    out["part"] = part
    out.update(kw)
    return out


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}
