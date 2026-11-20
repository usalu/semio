"""🏢 Round 3 curtain-wall bay: the real build-up of one stick-system bay (50 mm face width, mullions 50 x 150, transoms 50 x 100, pressure plate with 12 mm cover cap, 6-16-6 glass unit and enamelled spandrel with an insulated back pan) and the solid and shape writers the scale scripts state their parts with.

Frames: world x along the bay (0 at the outer face of the left mullion), y up, z through the wall (exterior at -z). Bone `frame` sits at the origin; `igu` at (0.05, 0.9) and `spandrel` at (0.05, 0.05), both unrotated, so their local coordinates are world minus that origin. Profiles are stated in (a, d): a across the face, centred on the member axis, d = world z.
"""
import math

W, H, D = 1.4, 3.3, 0.2
FACE = 0.05
MULLION_X = (0.025, 1.375)
TRANSOM_Y = (("base", 0.025), ("mid", 0.875), ("head", 3.275))
SIGHT_X = (0.05, 1.35)

Z_CAP = -0.100
Z_CAP_IN = -0.098
Z_PLATE = -0.088
Z_GROOVE = -0.092
Z_GLASS_OUT = -0.084
Z_PANE_OUT = -0.078
Z_PANE_IN = -0.062
Z_GLASS_IN = -0.056
Z_LIP = -0.050
Z_FACE = -0.046
Z_CHANNEL = -0.058
Z_MULLION_BACK = 0.100
Z_TRANSOM_BACK = 0.050
WALL = 0.003
CAP_T = 0.002
CAP_R = 0.002
BACK_R = 0.002

A_CAP = 0.025
A_PLATE = 0.023
A_GLASS = 0.010
A_ISOLATOR = 0.007
A_CHANNEL = 0.006
A_SLOT = 0.0025
GROOVE = (0.013, 0.020)
LIPS = ((0.0105, 0.012), (0.0205, 0.022))

BITE = 0.015
GLASS_X = (0.035, 1.365)
VISION_Y = (0.885, 3.265)
SPANDREL_Y = (0.035, 0.865)
SEAL_W = 0.012
EDGE_ZONE = 0.065

Z_ENAMEL_IN = -0.078
Z_WOOL = -0.058
Z_PAN = 0.042
PAN_T = 0.0015

IGU_O = (0.05, 0.9)
SP_O = (0.05, 0.05)

RHO = {"aluminium": 2800, "glass": 2500, "epdm": 1150, "polyamide": 1300, "edge-seal": 1200, "wool": 60, "steel": 7850}


def r5(x):
    return round(x, 6) + 0.0


def arc(c, r, a0, a1, n=4):
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


def front_edge(lips=True):
    """🦷 The glazing face of a mullion or transom tube from a = +0.025 to a = -0.025: the two gasket grooves between their lips and the screw channel in the axis that takes the isolator."""
    pts = [[A_CAP, Z_FACE]]
    for sign in (1, -1):
        seq = []
        for lo, hi in reversed(LIPS):
            seq += [[hi, Z_FACE], [hi, Z_LIP], [lo, Z_LIP], [lo, Z_FACE]]
        seq += [[A_CHANNEL, Z_FACE], [A_CHANNEL, Z_CHANNEL], [A_SLOT, Z_CHANNEL], [A_SLOT, Z_LIP]]
        if sign < 0:
            seq = [[-a, d] for a, d in reversed(seq)]
        pts += seq if lips else [p for p in seq if abs(p[0]) <= A_CHANNEL]
    pts.append([-A_CAP, Z_FACE])
    return pts


def tube(back):
    """🟫 A mullion (back 0.100) or transom (back 0.050) tube in (a, d): 50 mm face, 3 mm walls, rounded room-side corners, the glazing face with grooves and screw channel; returns the outer ring and the chamber."""
    outer = front_edge()
    outer += arc((-A_CAP + BACK_R, back - BACK_R), BACK_R, 180, 90)
    outer += arc((A_CAP - BACK_R, back - BACK_R), BACK_R, 90, 0)
    chamber = [[-A_CAP + WALL, Z_FACE + WALL], [A_CAP - WALL, Z_FACE + WALL], [A_CAP - WALL, back - WALL], [-A_CAP + WALL, back - WALL]]
    return clean(outer), clean(chamber)


def cap():
    """🧢 The snap-on cover cap in (a, d): 50 x 12 mm, 2 mm wall, the exterior corners rounded, open towards the glass."""
    pts = [[A_CAP, Z_PLATE]] + arc((A_CAP - CAP_R, Z_CAP + CAP_R), CAP_R, 0, -90) + arc((-A_CAP + CAP_R, Z_CAP + CAP_R), CAP_R, 270, 180)
    pts += [[-A_CAP, Z_PLATE], [-A_PLATE, Z_PLATE], [-A_PLATE, Z_CAP_IN], [A_PLATE, Z_CAP_IN], [A_PLATE, Z_PLATE]]
    return clean(pts)


def cap_block(back=Z_PLATE):
    """🧢 The cover cap with the pressure plate it encloses, as one outline in (a, d) down to `back` (the cap leg tips by default)."""
    pts = [[A_CAP, back]] + arc((A_CAP - CAP_R, Z_CAP + CAP_R), CAP_R, 0, -90) + arc((-A_CAP + CAP_R, Z_CAP + CAP_R), CAP_R, 270, 180) + [[-A_CAP, back]]
    return clean(pts)


def plate(grooves=True):
    """🔩 The pressure plate in (a, d): 46 x 10 mm inside the cap, the two grooves that hold the feet of the outer gaskets."""
    if not grooves:
        return clean([[-A_PLATE, Z_CAP_IN], [A_PLATE, Z_CAP_IN], [A_PLATE, Z_PLATE], [-A_PLATE, Z_PLATE]])
    g0, g1 = GROOVE
    return clean([[-A_PLATE, Z_CAP_IN], [A_PLATE, Z_CAP_IN], [A_PLATE, Z_PLATE], [g1, Z_PLATE], [g1, Z_GROOVE], [g0, Z_GROOVE], [g0, Z_PLATE],
                  [-g0, Z_PLATE], [-g0, Z_GROOVE], [-g1, Z_GROOVE], [-g1, Z_PLATE]])


def isolator():
    """🧊 The polyamide isolator in the 20 mm glass joint, 14 x 30 mm, from the plate down to the screw channel."""
    return clean([[-A_ISOLATOR, Z_PLATE], [A_ISOLATOR, Z_PLATE], [A_ISOLATOR, Z_CHANNEL], [-A_ISOLATOR, Z_CHANNEL]])


def gasket_out(sign):
    """⚫ The outer EPDM glazing gasket on side `sign`: 15 x 4 mm body on the glass, foot in the plate groove."""
    g0, g1 = GROOVE
    pts = [[A_GLASS, Z_GLASS_OUT], [A_CAP, Z_GLASS_OUT], [A_CAP, Z_PLATE], [g1, Z_PLATE], [g1, Z_GROOVE], [g0, Z_GROOVE], [g0, Z_PLATE], [A_GLASS, Z_PLATE]]
    return clean([[sign * a, d] for a, d in pts])


def gasket_in(sign):
    """⚫ The inner EPDM glazing gasket on side `sign`: 15 x 6 mm body under the glass, foot between the lips of the tube."""
    (l0, l1), (m0, m1) = LIPS
    pts = [[A_GLASS, Z_GLASS_IN], [A_CAP, Z_GLASS_IN], [A_CAP, Z_LIP], [m0, Z_LIP], [m0, Z_FACE], [l1, Z_FACE], [l1, Z_LIP], [A_GLASS, Z_LIP]]
    return clean([[sign * a, d] for a, d in pts])


def section_outline(back):
    """⬛ The cut member as drawn at 1:50: cap, plate, gaskets, glass edges, isolator and tube as one outline in (a, d)."""
    pts = [[A_CAP, back - BACK_R]] + arc((A_CAP - BACK_R, back - BACK_R), BACK_R, 0, 90)[1:] + arc((-A_CAP + BACK_R, back - BACK_R), BACK_R, 90, 180)[1:]
    pts += [[-A_CAP, Z_CAP + CAP_R]] + arc((-A_CAP + CAP_R, Z_CAP + CAP_R), CAP_R, 180, 270)[1:] + arc((A_CAP - CAP_R, Z_CAP + CAP_R), CAP_R, 270, 360)[1:]
    return clean(pts)


def ring_area(points):
    a = 0.0
    for k in range(len(points)):
        (x0, y0), (x1, y1) = points[k], points[(k + 1) % len(points)]
        a += x0 * y1 - x1 * y0
    return a / 2


def moments(ring):
    """📐 Area, first and second moment about d = 0 of one ring in (a, d), counted positive whatever its winding."""
    a = sy = iyy = 0.0
    for j in range(len(ring)):
        (x0, y0), (x1, y1) = ring[j], ring[(j + 1) % len(ring)]
        c = x0 * y1 - x1 * y0
        a += c / 2
        sy += (y0 + y1) * c / 6
        iyy += (y0 * y0 + y0 * y1 + y1 * y1) * c / 12
    return (a, sy, iyy) if a > 0 else (-a, -sy, -iyy)


def section(rings):
    """⚖️ Area, centroid depth, second moment about the centroidal axis along a (bending in d: wind on the wall) and extreme fibre of a profile given as outer ring plus holes."""
    parts = [moments(r) for r in rings]
    area = parts[0][0] - sum(p[0] for p in parts[1:])
    sy = parts[0][1] - sum(p[1] for p in parts[1:])
    iyy = parts[0][2] - sum(p[2] for p in parts[1:])
    cy = sy / area
    return area, cy, iyy - area * cy * cy, max(abs(p[1] - cy) for p in rings[0])


def rect(x0, y0, x1, y1):
    return [[r5(x0), r5(y0)], [r5(x1), r5(y0)], [r5(x1), r5(y1)], [r5(x0), r5(y1)]]


def solid(i, kind, bone, material, tone, role="body", **extra):
    return {"id": i, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **extra}


def box(i, bone, material, tone, lo, hi, role="body", **extra):
    """📦 A box from corner lo to corner hi in the bone frame."""
    size = [r5(h - l) for l, h in zip(lo, hi)]
    at = [r5((h + l) / 2) for l, h in zip(lo, hi)]
    return solid(i, "box", bone, material, tone, role, size=size, transform={"translation": at}, **extra)


def prism(i, bone, material, tone, profile, plane, lo, hi, role="body", holes=None, **extra):
    out = solid(i, "extrude", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], plane=plane, **{"from": r5(lo), "to": r5(hi)}, **extra)
    if holes:
        out["holes"] = [[[r5(a), r5(b)] for a, b in h] for h in holes]
    return out


def along_y(i, bone, material, tone, profile, x0, y0, y1, role="body", holes=None, **extra):
    """↕️ A profile given in (a, d) extruded upwards along a vertical member axis at x0 (plane zx: points are [z, x])."""
    zx = lambda ring: [[d, x0 + a] for a, d in ring]
    return prism(i, bone, material, tone, zx(profile), "zx", y0, y1, role, [zx(h) for h in holes] if holes else None, **extra)


def along_x(i, bone, material, tone, profile, yc, x0, x1, role="body", holes=None, **extra):
    """↔️ A profile given in (a, d) extruded along a horizontal member axis at height yc (plane yz: points are [y, z])."""
    yz = lambda ring: [[yc + a, d] for a, d in ring]
    return prism(i, bone, material, tone, yz(profile), "yz", x0, x1, role, [yz(h) for h in holes] if holes else None, **extra)


def pane(i, bone, material, tone, outer, z0, z1, hole=None, role="glazing", **extra):
    """🪟 A flat pane or ring in the xy plane from z0 to z1."""
    return prism(i, bone, material, tone, rect(*outer), "xy", z0, z1, role, [rect(*hole)] if hole else None, **extra)


def rod(i, bone, material, tone, origin, axis, length, radius, role="body", **extra):
    return solid(i, "revolve", bone, material, tone, role, profile=[[0.0, r5(radius)], [r5(length), r5(radius)]], axis=list(axis), origin=[r5(c) for c in origin], **extra)


def ellipsoid(i, bone, material, tone, radii, at, role="body", **extra):
    return solid(i, "ellipsoid", bone, material, tone, role, radii=[r5(c) for c in radii], transform={"translation": [r5(c) for c in at]}, **extra)


def repeat(s, count, step):
    s["repeat"] = {"count": count, "translation": [r5(c) for c in step]}
    return s


def cut(s, *faces):
    s["sectionFaces"] = list(faces)
    return s


def th(s, region, layer=None):
    s["region"] = region
    if layer:
        s["layer"] = layer
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


def local(bone_origin, x0, y0, x1, y1):
    return (x0 - bone_origin[0], y0 - bone_origin[1], x1 - bone_origin[0], y1 - bone_origin[1])
