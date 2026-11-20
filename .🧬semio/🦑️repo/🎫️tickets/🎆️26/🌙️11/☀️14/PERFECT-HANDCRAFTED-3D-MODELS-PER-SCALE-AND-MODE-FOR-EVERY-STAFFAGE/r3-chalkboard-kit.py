"""🖍️ Round 3 chalkboard: the real dimensions of the wall-mounted sliding board as built (edge-profile frames with corner pieces, top-hung leaf on two trolleys in a slotted header track, tray extrusion, wall cleats and buffers) and the small solid and shape writers every scale states its parts with.

World frame of the asset: X along the board (0 at mid width), Y up from the tray underside, Z across (room at -Z, wall plane at Z = +0.040). Bone frames: frame and tray u = X + 2, v = Y, w = Z; leaf u = X + 2 (pose leaf-left), v = Y - 0.068, w = Z. Every writer takes world coordinates and states them in the bone frame.
"""
import json
import math
import os
import re

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/chalkboard/🔣️.json"

FR, TY, LF = "frame", "tray", "leaf"
ALU, ENAMEL, MDF, PA, CHALK, SPONGE, WOOD = "aluminium-frame", "enamel-board", "mdf-core", "polyamide", "chalk", "sponge", "beech-wood"
ORIGIN = {FR: (0.0, 0.0), TY: (0.0, 0.0), LF: (0.0, 0.06)} if os.environ.get("R3_CHALKBOARD_RIG") == "before" else {FR: (-2.0, 0.0), TY: (-2.0, 0.0), LF: (-2.0, 0.068)}

HALF_W = 2.0
Z_WALL = 0.040
ZF0, ZF1 = 0.012, 0.032
ZL0, ZL1 = -0.012, 0.008
ZT0, ZT1 = -0.040, 0.032
ZH0 = -0.0165
Y_TRAY = 0.060
Y_BOARD_TOP = 1.222
Y_TOP = 1.260
Y_LEAF0, Y_LEAF1 = 0.068, 1.214
BAR = 0.035
BAR_D = 0.020
WALL = 0.0017
BITE = 0.012
POCKET = BAR - BITE
MULLION = 0.015
R_CORNER = 0.010
R_SOFT = 0.004
T_ENAMEL = 0.0006
T_CORE = 0.016
SLOT_S = (0.0095, 0.0155)
T_HEAD = 0.002
CHAMBER_Z = (-0.0145, 0.0105)
SLOT_Z = (-0.0065, 0.0025)
Z_HANG = -0.002
CAP = 0.0075
HANGERS = (-1.7, -0.3)
T_TRAY = 0.002
Y_LIP = 0.018
Z_BOX = 0.012
Y_HOOK = 1.2065
T_CLEAT = 0.0025
CLEAT_X = 1.9
Y_BUFFER = 0.0775
BUFFER_R = 0.015
HANDLE_X = (-1.9825, -0.0175)
HANDLE_Y = (0.352, 0.480)
HANDLE_R = 0.004
Z_GRIP = -0.036

RHO = {ALU: 2700.0, ENAMEL: 7850.0, MDF: 750.0, PA: 1150.0, CHALK: 2300.0, SPONGE: 60.0, WOOD: 720.0}


def r5(x):
    return round(x, 5) + 0.0


def arc(c, r, a0, a1, n=3):
    return [[c[0] + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), c[1] + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))] for k in range(n + 1)]


def clean(points):
    out = []
    for p in points:
        q = [r5(p[0]), r5(p[1])]
        if not out or out[-1] != q:
            out.append(q)
    if len(out) > 1 and out[-1] == out[0]:
        out.pop()
    return out


def rounded_rect(a0, b0, a1, b1, r, n=3):
    """🔲 A rectangle with corner radius r, counter-clockwise."""
    if r <= 0:
        return clean([[a0, b0], [a1, b0], [a1, b1], [a0, b1]])
    out = []
    for ca, cb, s in ((a1 - r, b0 + r, 270), (a1 - r, b1 - r, 0), (a0 + r, b1 - r, 90), (a0 + r, b0 + r, 180)):
        out.extend(arc((ca, cb), r, s, s + 90, n))
    return clean(out)


def edge_profile(slot=False):
    """🧩 The 35 x 20 mm aluminium edge profile in (s, d): s from the outer edge towards the panel, d from the front face to the back; soft front outer edge of radius 4, a closed chamber of 19.6 mm and the 12 mm pocket that takes the panel edge. Returns (outline, hole); with slot the chamber opens to the front through the 6 mm guide slot and there is no hole."""
    soft = arc((R_SOFT, R_SOFT), R_SOFT, 180, 270)
    web = POCKET - WALL
    tail = [[BAR, 0], [BAR, WALL], [POCKET, WALL], [POCKET, BAR_D - WALL], [BAR, BAR_D - WALL], [BAR, BAR_D], [0, BAR_D]]
    chamber = [[WALL, R_SOFT], [R_SOFT, WALL], [web, WALL], [web, BAR_D - WALL], [WALL, BAR_D - WALL]]
    if not slot:
        return clean(soft + tail), clean(chamber)
    s0, s1 = SLOT_S
    path = soft + [[s0, 0], [s0, WALL], [R_SOFT, WALL], [WALL, R_SOFT], [WALL, BAR_D - WALL], [web, BAR_D - WALL], [web, WALL], [s1, WALL], [s1, 0]] + tail
    return clean(path), None


def edge_envelope():
    """🧩 The outer envelope of the edge profile in (s, d): what the room air touches."""
    return clean(arc((R_SOFT, R_SOFT), R_SOFT, 180, 270) + [[BAR, 0], [BAR, BAR_D], [0, BAR_D]])


def mullion_profile():
    """🧩 The 30 mm H profile between the two back panels in (z, x): front and back flange, 1.7 mm web."""
    m, t = MULLION, WALL / 2
    return clean([[ZF0, -m], [ZF0, m], [ZF0 + WALL, m], [ZF0 + WALL, t], [ZF1 - WALL, t], [ZF1 - WALL, m], [ZF1, m], [ZF1, -m], [ZF1 - WALL, -m], [ZF1 - WALL, -t], [ZF0 + WALL, -t], [ZF0 + WALL, -m]])


def header_profile():
    """🛤️ The header track in (y, z): 48.5 x 38 mm extrusion on the top bar, the running chamber over the leaf with its two treads beside the 8 mm slot, a closed box behind it, soft front top edge. Returns (outline, hole)."""
    y0, y1, t = Y_BOARD_TOP, Y_TOP, T_HEAD
    c0, c1 = CHAMBER_Z
    s0, s1 = SLOT_Z
    path = [[y0, s0], [y0, ZH0]] + [[y1 - R_SOFT + R_SOFT * math.sin(math.radians(a)), ZH0 + R_SOFT - R_SOFT * math.cos(math.radians(a))] for a in (0, 30, 60, 90)]
    path += [[y1, ZF1], [y0, ZF1], [y0, s1], [y0 + t, s1], [y0 + t, c1], [y1 - t, c1], [y1 - t, c0], [y0 + t, c0], [y0 + t, s0]]
    hole = [[y0 + t, c1 + t], [y1 - t, c1 + t], [y1 - t, ZF1 - t], [y0 + t, ZF1 - t]]
    return clean(path), clean(hole)


def header_envelope():
    return clean([[Y_BOARD_TOP, ZH0]] + [[Y_TOP - R_SOFT + R_SOFT * math.sin(math.radians(a)), ZH0 + R_SOFT - R_SOFT * math.cos(math.radians(a))] for a in (0, 30, 60, 90)] + [[Y_TOP, ZF1], [Y_BOARD_TOP, ZF1]])


def tray_profile():
    """🥢 The tray extrusion in (y, z): 2 mm floor with the 18 mm front lip, the 50 mm trough and the closed 20 x 60 mm box at the back that carries the bottom bar. Returns (outline, hole)."""
    t = T_TRAY
    path = [[0, ZT0 + 0.004]] + [[0.004 - 0.004 * math.cos(math.radians(a)), ZT0 + 0.004 - 0.004 * math.sin(math.radians(a))] for a in (30, 60, 90)]
    path += [[Y_LIP - 0.001, ZT0], [Y_LIP, ZT0 + 0.0005], [Y_LIP, ZT0 + t - 0.0005], [Y_LIP - 0.001, ZT0 + t], [2 * t, ZT0 + t], [t, ZT0 + 2 * t], [t, Z_BOX], [Y_TRAY, Z_BOX], [Y_TRAY, ZT1], [0, ZT1]]
    hole = [[t, Z_BOX + t], [Y_TRAY - t, Z_BOX + t], [Y_TRAY - t, ZT1 - t], [t, ZT1 - t]]
    return clean(path), clean(hole)


def tray_envelope():
    path = [[0, ZT0 + 0.004]] + [[0.004 - 0.004 * math.cos(math.radians(a)), ZT0 + 0.004 - 0.004 * math.sin(math.radians(a))] for a in (30, 60, 90)]
    path += [[Y_LIP - 0.002, ZT0], [Y_LIP, ZT0 + 0.002], [Y_LIP, Z_BOX], [Y_TRAY, Z_BOX], [Y_TRAY, ZT1], [0, ZT1]]
    return clean(path)


def wall_cleat_profile():
    """🪝 The wall cleat in (y, z): 2.5 mm aluminium Z, plate on the wall, upstand set off by 3 mm."""
    t = T_CLEAT
    return clean([[1.150, Z_WALL], [1.150, Z_WALL - t], [1.1775, Z_WALL - t], [1.1805, Z_WALL - 2 * t - 0.0005], [Y_HOOK, Z_WALL - 2 * t - 0.0005], [Y_HOOK, Z_WALL - t - 0.0005], [1.1815, Z_WALL - t - 0.0005], [1.1785, Z_WALL]])


def board_cleat_profile():
    """🪝 The board cleat in (y, z): plate on the back of top bar and header, web over the wall upstand, downstand behind it."""
    t = T_CLEAT
    return clean([[1.245, ZF1], [1.245, ZF1 + t], [Y_HOOK + t, ZF1 + t], [Y_HOOK + t, Z_WALL - 0.0003], [1.186, Z_WALL - 0.0003], [1.186, Z_WALL - 0.0003 - t], [Y_HOOK, Z_WALL - 0.0003 - t], [Y_HOOK, ZF1]])


def glide_profile():
    """🧲 The leaf's bottom glide in (y, z): plate on the back of the leaf bottom rail, neck through the guide slot, head in the chamber of the bottom bar."""
    return clean([[0.068, ZL1], [0.088, ZL1], [0.088, 0.0115], [0.075, 0.0115], [0.075, 0.0142], [0.078, 0.0142], [0.078, 0.0172], [0.067, 0.0172], [0.067, 0.0142], [0.070, 0.0142], [0.070, 0.0115], [0.068, 0.0115]])


def hexagon(ca, cb, across_flats):
    R = across_flats / math.sqrt(3)
    return clean([[ca + R * math.cos(math.radians(60 * k + 30)), cb + R * math.sin(math.radians(60 * k + 30))] for k in range(6)])


def area(points):
    a = 0.0
    for k in range(len(points)):
        (a0, b0), (a1, b1) = points[k], points[(k + 1) % len(points)]
        a += a0 * b1 - a1 * b0
    return abs(a) / 2


def section_area(outline, hole=None):
    return area(outline) - (area(hole) if hole else 0.0)


def solid(i, kind, bone, material, tone, role="body", **extra):
    return {"id": i, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **extra}


def at(bone, x, y, z):
    ox, oy = ORIGIN[bone]
    return [r5(x - ox), r5(y - oy), r5(z)]


def box(i, bone, material, tone, x, y, z, role="body", radius=None, **extra):
    """📦 A box from its world ranges x = (x0, x1), y = (y0, y1), z = (z0, z1)."""
    out = solid(i, "box", bone, material, tone, role, size=[r5(x[1] - x[0]), r5(y[1] - y[0]), r5(z[1] - z[0])], **extra)
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": at(bone, (x[0] + x[1]) / 2, (y[0] + y[1]) / 2, (z[0] + z[1]) / 2)}
    return out


def prism(i, bone, material, tone, profile, plane, lo, hi, role="body", holes=None, **extra):
    """🧱 An extrusion of a world profile: plane yz = (Y, Z) along X, zx = (Z, X) along Y, xy = (X, Y) along Z."""
    ox, oy = ORIGIN[bone]
    shift = {"yz": (oy, 0.0, ox), "zx": (0.0, ox, oy), "xy": (ox, oy, 0.0)}[plane]
    move = lambda ring: [[r5(a - shift[0]), r5(b - shift[1])] for a, b in ring]
    out = solid(i, "extrude", bone, material, tone, role, profile=move(profile), plane=plane, **{"from": r5(lo - shift[2]), "to": r5(hi - shift[2])}, **extra)
    if holes:
        out["holes"] = [move(h) for h in holes if h]
        if not out["holes"]:
            del out["holes"]
    return out


def turned(i, bone, material, tone, origin, axis, profile, role="body", **extra):
    return solid(i, "revolve", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], axis=list(axis), origin=at(bone, *origin), **extra)


def rod(i, bone, material, tone, origin, axis, length, radius, role="body", **extra):
    return turned(i, bone, material, tone, origin, axis, [[0.0, radius], [length, radius]], role, **extra)


def capsule(i, bone, material, tone, p0, p1, r, role="body", **extra):
    return solid(i, "capsule", bone, material, tone, role, p0=at(bone, *p0), p1=at(bone, *p1), r0=r5(r), r1=r5(r), **extra)


def repeat(s, count, step):
    s["repeat"] = {"count": count, "translation": [r5(c) for c in step]}
    return s


def flag(s, **extra):
    s.update(extra)
    return s


def th(s, region, layer=None, faces=None):
    s["region"] = region
    if layer:
        s["layer"] = layer
    if faces:
        s["sectionFaces"] = list(faces)
    return s


def th_of(i, of, region, layer=None, faces=None):
    out = {"id": i, "of": of, "region": region}
    if layer:
        out["layer"] = layer
    if faces:
        out["sectionFaces"] = list(faces)
    return out


def st(s, member, part="member", anchor=None):
    s["member"] = member
    s["part"] = part
    if anchor:
        s["anchor"] = anchor
    return s


def st_of(i, of, member, part="member", anchor=None):
    out = {"id": i, "of": of, "member": member, "part": part}
    if anchor:
        out["anchor"] = anchor
    return out


def text(en, de):
    return {"en": en, "de": de}


def shape(i, bone, role, z, filled, tone, geometry, region=None, regions=None, material=None, pen=None):
    out = {"id": i, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if pen:
        out["penMm"] = pen
    return out


def rect(a0, b0, a1, b1):
    return {"kind": "rect", "x": r5(min(a0, a1)), "y": r5(min(b0, b1)), "w": r5(abs(a1 - a0)), "h": r5(abs(b1 - b0))}


def poly(points, closed=True):
    return {"kind": "poly", "points": [[r5(a), r5(b)] for a, b in points], "closed": closed}


def disc(ca, cb, r):
    return {"kind": "ellipse", "cx": r5(ca), "cy": r5(cb), "rx": r5(r), "ry": r5(r)}


def member(source, key):
    """🧾 The text range of a top-level asset member in the catalog file."""
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        elif depth == 0 and char in ",\n":
            return found.end(), index
        index += 1


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    return isinstance(value, list) and len(value) > 0 and all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value)


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def dumped(value):
    """🧾 A value in the catalog layout at member depth: two-space JSON, lists of coordinate pairs on one line."""
    return dump(value, 2)
