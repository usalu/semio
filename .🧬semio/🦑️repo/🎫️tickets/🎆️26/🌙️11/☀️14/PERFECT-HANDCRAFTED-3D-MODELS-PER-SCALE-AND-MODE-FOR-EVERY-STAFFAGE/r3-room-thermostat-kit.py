"""🎚️ Round 3, appliances/room-thermostat: the measures of the real object and the small solid writers shared by the model and the root script.

Frames (corrected rig): bone `housing` runs from the lower to the upper fixing screw (u up, v = to the left seen face-on, w = towards the wall); bone `dial` sits on the spindle axis and runs along the index (u up at rest). Wall plane w = +0.0135, housing face w = -0.0015, dial face w = -0.0130, index w = -0.0135.
"""
import math

ABS, LCD, LED = "abs-plastic", "lcd-glass", "led-lens"

HALF = 0.045
CORNER_R = 0.008
SCREW_PITCH = 0.060
CENTRE_U = SCREW_PITCH / 2
DIAL_U = CENTRE_U - 0.014
W_WALL = 0.0135
W_BASE = 0.0105
W_FACE = -0.0015
W_SHELL = 0.0005
DIAL_R = 0.021
W_GRIP = -0.0100
W_CAP = -0.0130
W_INDEX = -0.0135
DISPLAY_U = CENTRE_U + 0.0265
DISPLAY = (0.021, 0.036)
LED_AT = (CENTRE_U + 0.0265, -0.0315)
LED_R = 0.0035


def r6(value):
    return round(value, 6)


def circle(radius, count, cu=0.0, cv=0.0, phase=0.0):
    """⭕ A regular polygon of `count` corners on a circle, stated by its radius and centre."""
    return [[r6(cu + radius * math.cos(phase + 2 * math.pi * k / count)), r6(cv + radius * math.sin(phase + 2 * math.pi * k / count))] for k in range(count)]


def fluted(crest, trough, flutes):
    """🌼 The grip outline of the knob: `flutes` finger flutes, crest and trough radius alternating."""
    return [[r6((crest if k % 2 == 0 else trough) * math.cos(math.pi * k / flutes)), r6((crest if k % 2 == 0 else trough) * math.sin(math.pi * k / flutes))] for k in range(2 * flutes)]


def rounded_square(half, radius, per_corner, cu=0.0, cv=0.0):
    """🔲 A square of half side `half` with corner arcs of `radius`, `per_corner` + 1 points per arc."""
    out = []
    for quadrant, (su, sv) in enumerate([(1, 1), (-1, 1), (-1, -1), (1, -1)]):
        for k in range(per_corner + 1):
            a = math.pi / 2 * (quadrant + k / per_corner)
            out.append([r6(cu + su * (half - radius) + radius * math.cos(a)), r6(cv + sv * (half - radius) + radius * math.sin(a))])
    return out


def wall_panel(side, half, radius, inner_half, inner_radius, per_corner):
    """🧱 One of the four skirt walls of a rounded-square shell between its two corner diagonals: side 0 is +u (top), 1 is +v, 2 is -u, 3 is -v; `per_corner` is even."""
    def run(points):
        n = per_corner + 1
        before, own = (side - 1) % 4, side
        return points[before * n + per_corner // 2:before * n + n] + points[own * n:own * n + per_corner // 2 + 1]
    return run(rounded_square(half, radius, per_corner)) + run(rounded_square(inner_half, inner_radius, per_corner))[::-1]


INWARD = ["-u", "-v", "+u", "+v"]


def rect(cu, cv, su, sv):
    return [[r6(cu - su / 2), r6(cv - sv / 2)], [r6(cu + su / 2), r6(cv - sv / 2)], [r6(cu + su / 2), r6(cv + sv / 2)], [r6(cu - su / 2), r6(cv + sv / 2)]]


def stadium(cu, cv, length, radius, per_end):
    """💊 A slot along u: two half circles of `radius` whose centres are `length` apart."""
    out = []
    for k in range(per_end + 1):
        a = -math.pi / 2 + math.pi * k / per_end
        out.append([r6(cu + length / 2 + radius * math.cos(a)), r6(cv + radius * math.sin(a))])
    for k in range(per_end + 1):
        a = math.pi / 2 + math.pi * k / per_end
        out.append([r6(cu - length / 2 + radius * math.cos(a)), r6(cv + radius * math.sin(a))])
    return out


def base(ident, kind, bone, material, role, tone):
    return {"id": ident, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def extrude(ident, bone, material, role, tone, profile, w0, w1, holes=None, taper=None, at=None, **extra):
    solid = base(ident, "extrude", bone, material, role, tone)
    solid["profile"] = profile
    if holes:
        solid["holes"] = holes
    solid.update({"plane": "xy", "from": w0, "to": w1})
    if taper:
        solid["taperTo"] = taper
    if at:
        solid["transform"] = {"translation": list(at)}
    solid.update(extra)
    return solid


def loft(ident, bone, material, role, tone, stations, exponent, cu, **extra):
    solid = base(ident, "loft", bone, material, role, tone)
    solid["sections"] = [{"at": [cu, 0, w], "rx": half, "ry": half, "exponent": exponent} for w, half in stations]
    solid.update({"capStart": True, "capEnd": True})
    solid.update(extra)
    return solid


def box(ident, bone, material, role, tone, size, at, radius=None, **extra):
    solid = base(ident, "box", bone, material, role, tone)
    solid["size"] = list(size)
    if radius:
        solid["cornerRadius"] = radius
    solid["transform"] = {"translation": list(at)}
    solid.update(extra)
    return solid


def ellipsoid(ident, bone, material, role, tone, radii, at, **extra):
    solid = base(ident, "ellipsoid", bone, material, role, tone)
    solid["radii"] = list(radii)
    solid["transform"] = {"translation": list(at)}
    solid.update(extra)
    return solid


def of(ident, target, **extra):
    return {"id": ident, "of": target, **extra}
