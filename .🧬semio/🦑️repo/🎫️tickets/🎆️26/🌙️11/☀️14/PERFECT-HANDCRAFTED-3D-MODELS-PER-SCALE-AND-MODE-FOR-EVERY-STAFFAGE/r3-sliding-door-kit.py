"""🚪 Round 3 sliding door: the real dimensions of the door as built (slotted box track, rebated jambs, steel-framed glass leaf on two trolleys) and the small solid and shape writers every scale script states its parts with.

Frames: track bone u = world x (0 at the west jamb axis), v = world y - 2.1, w = world z (interior at -w); leaf bone u = 2.1 - world y (down from the track axis), v = world x - 1.5 (closed pose), w = world z.
"""
import math

TRACK_Y = 2.1
HALF = 0.05
T_TRACK = 0.004
R_OUT = 0.008
R_IN = 0.004
SLOT = 0.012
LIP_TOP = -0.03
TRACK_X0, TRACK_X1 = -0.05, 3.05
CAP_T = 0.008
JAMB_X = (0.0, 3.0)
JAMB_TOP = -0.05
FLOOR = -TRACK_Y
CHANNEL_W = 0.03
CHANNEL_D = 0.02
SEAL_T = 0.008
TRAVEL = 0.962
GUIDE_X = (1.965, 2.0)

U_TOP = 0.10
U_TOP_RAIL = 0.15
U_GLASS_BOT = 1.97
U_GROOVE = 2.045
U_RAIL_BOT = 2.07
U_BRUSH = 2.09
V_OUT = 0.5
V_IN = 0.45
W_LEAF = 0.025
T_LEAF = 0.002
R_LEAF = 0.004
GLASS_T = 0.01
BEAD = 0.015
GROOVE_W = 0.01
BRUSH_W = (0.012, 0.024)

HANGER_V = 0.35
NUT_U = (0.055, 0.063, 0.071)
CLEVIS_U = (0.071, 0.10)
PIN_U = 0.0855
ROD_U = (0.010, 0.08)
TROLLEY_HALF = 0.045
WHEEL_U = 0.031
WHEEL_R = 0.015
WHEEL_W = (0.025, 0.037)
WHEEL_DV = 0.025

HANDLE_V = 0.41
HANDLE_U = (0.88, 1.18)
LEG_U = (0.92, 1.14)
GRIP_W = 0.038
ROSETTE = (0.005, 0.013, 0.015)


def r5(x):
    return round(x, 5) + 0.0


def arc(c, r, a0, a1, n=4):
    return [[r5(c[0] + r * math.cos(math.radians(a0 + (a1 - a0) * k / n))), r5(c[1] + r * math.sin(math.radians(a0 + (a1 - a0) * k / n)))] for k in range(n + 1)]


def rounded_rect(x0, y0, x1, y1, r, n=4):
    """🔲 A rectangle with corner radius r, counter-clockwise."""
    if r <= 0:
        return [[r5(x0), r5(y0)], [r5(x1), r5(y0)], [r5(x1), r5(y1)], [r5(x0), r5(y1)]]
    out = []
    for cx, cy, a0 in ((x1 - r, y0 + r, 270), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        out.extend(arc((cx, cy), r, a0, a0 + 90, n))
    return out


def track_profile():
    """🛤️ The slotted box track in (v, w): 100 x 100 x 4 cold-formed, outer radius 8, the running chamber with its two treads and the 24 mm slot between two upturned 20 mm lips, as one simple ring."""
    h, t, s = HALF, T_TRACK, SLOT
    ring = [[-h, s], [-h, h - R_OUT]]
    ring += arc((-h + R_OUT, h - R_OUT), R_OUT, 180, 90)[1:]
    ring += [[h - R_OUT, h]]
    ring += arc((h - R_OUT, h - R_OUT), R_OUT, 90, 0)[1:]
    ring += [[h, -h + R_OUT]]
    ring += arc((h - R_OUT, -h + R_OUT), R_OUT, 0, -90)[1:]
    ring += [[-h + R_OUT, -h]]
    ring += arc((-h + R_OUT, -h + R_OUT), R_OUT, -90, -180)[1:]
    ring += [[-h, -s], [LIP_TOP, -s], [LIP_TOP, -s - t], [-h + t, -s - t], [-h + t, -h + t + R_IN]]
    ring += arc((-h + t + R_IN, -h + t + R_IN), R_IN, 180, 270)[1:]
    ring += [[h - t - R_IN, -h + t]]
    ring += arc((h - t - R_IN, -h + t + R_IN), R_IN, 270, 360)[1:]
    ring += [[h - t, h - t - R_IN]]
    ring += arc((h - t - R_IN, h - t - R_IN), R_IN, 0, 90)[1:]
    ring += [[-h + t + R_IN, h - t]]
    ring += arc((-h + t + R_IN, h - t - R_IN), R_IN, 90, 180)[1:]
    ring += [[-h + t, s + t], [LIP_TOP, s + t], [LIP_TOP, s]]
    out = []
    for p in ring:
        q = [r5(p[0]), r5(p[1])]
        if not out or out[-1] != q:
            out.append(q)
    if out[-1] == out[0]:
        out.pop()
    return out


def jamb_profile(cx, side, inset=0.0, r=R_OUT):
    """🏛️ A rebated jamb post in (w, u): 100 x 100 with the 60 x 20 receiving channel on the opening side (side +1 opens to +u); inset shrinks it by a wall thickness."""
    h = HALF - inset
    cw = CHANNEL_W + inset
    face = cx + side * h
    bottom = cx + side * (HALF - CHANNEL_D - inset)
    back = cx - side * h
    rr = max(r - inset, 0.0)
    corner = lambda w, u, a0: arc((w, u), rr, a0, a0 + 90) if rr > 0 else [[r5(w), r5(u)]]
    if side > 0:
        pts = corner(-h + rr, back + rr, 180) + corner(h - rr, back + rr, 270) + corner(h - rr, face - rr, 0)
        pts += [[r5(cw), r5(face)], [r5(cw), r5(bottom)], [r5(-cw), r5(bottom)], [r5(-cw), r5(face)]]
        pts += corner(-h + rr, face - rr, 90)
    else:
        pts = corner(-h + rr, face + rr, 180) + [[r5(-cw), r5(face)], [r5(-cw), r5(bottom)], [r5(cw), r5(bottom)], [r5(cw), r5(face)]]
        pts += corner(h - rr, face + rr, 270) + corner(h - rr, back - rr, 0) + corner(-h + rr, back - rr, 90)
    out = []
    for p in pts:
        if not out or out[-1] != p:
            out.append(p)
    return out


def bottom_rail_profile(inset=0.0):
    """🧱 The bottom rail in (w, u): 50 x 100 with the 20 x 25 guide groove in its underside; inset shrinks it by the wall."""
    w, g = W_LEAF - inset, GROOVE_W + inset
    u0, u1, ug = U_GLASS_BOT + inset, U_RAIL_BOT - inset, U_GROOVE - inset
    return [[r5(-w), r5(u0)], [r5(w), r5(u0)], [r5(w), r5(u1)], [r5(g), r5(u1)], [r5(g), r5(ug)], [r5(-g), r5(ug)], [r5(-g), r5(u1)], [r5(-w), r5(u1)]]


def hexagon(cx, cy, across_flats):
    R = across_flats / math.sqrt(3)
    return [[r5(cx + R * math.cos(math.radians(60 * k + 30))), r5(cy + R * math.sin(math.radians(60 * k + 30)))] for k in range(6)]


def polygon(points):
    """⚖️ Area, centroid and second moment about the horizontal centroid axis of a simple polygon in (y, z) with y the bending direction."""
    a = cy = 0.0
    iyy = 0.0
    n = len(points)
    for k in range(n):
        (y0, z0), (y1, z1) = points[k], points[(k + 1) % n]
        c = z0 * y1 - z1 * y0
        a += c
        cy += (y0 + y1) * c
        iyy += (y0 * y0 + y0 * y1 + y1 * y1) * c
    a /= 2
    cy /= 6 * a
    iyy /= 12
    return abs(a), cy, abs(iyy) - abs(a) * cy * cy


def solid(i, kind, bone, material, tone, role="body", **extra):
    return {"id": i, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **extra}


def box(i, bone, material, tone, size, at, role="body", **extra):
    return solid(i, "box", bone, material, tone, role, size=[r5(s) for s in size], transform={"translation": [r5(c) for c in at]}, **extra)


def prism(i, bone, material, tone, profile, plane, lo, hi, role="body", holes=None, **extra):
    out = solid(i, "extrude", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], plane=plane, **{"from": r5(lo), "to": r5(hi)}, **extra)
    if holes:
        out["holes"] = [[[r5(a), r5(b)] for a, b in h] for h in holes]
    return out


def rod(i, bone, material, tone, origin, axis, length, radius, role="body", **extra):
    """🥢 A solid round bar from origin along axis."""
    return solid(i, "revolve", bone, material, tone, role, profile=[[0.0, r5(radius)], [r5(length), r5(radius)]], axis=list(axis), origin=[r5(c) for c in origin], **extra)


def turned(i, bone, material, tone, origin, axis, profile, role="body", **extra):
    return solid(i, "revolve", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], axis=list(axis), origin=[r5(c) for c in origin], **extra)


def capsule(i, bone, material, tone, p0, p1, r, role="body", **extra):
    return solid(i, "capsule", bone, material, tone, role, p0=[r5(c) for c in p0], p1=[r5(c) for c in p1], r0=r5(r), r1=r5(r), **extra)


def ellipsoid(i, bone, material, tone, radii, at, role="detail", **extra):
    return solid(i, "ellipsoid", bone, material, tone, role, radii=[r5(c) for c in radii], transform={"translation": [r5(c) for c in at]}, **extra)


def repeat(s, count, step=None, axis=None, angle=None, origin=None):
    s["repeat"] = {"count": count}
    if step:
        s["repeat"]["translation"] = [r5(c) for c in step]
    if axis:
        s["repeat"]["rotationAxis"] = list(axis)
        s["repeat"]["rotationRad"] = angle
        s["repeat"]["origin"] = [r5(c) for c in origin]
    return s


def cut(s, *faces):
    s["sectionFaces"] = list(faces)
    return s


def flag(s, **extra):
    s.update(extra)
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


def shape(i, bone, role, z, filled, tone, geometry, region=None, regions=None, material=None, pen=None):
    out = {"id": i, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if pen:
        out["penMm"] = pen
    out.update(geometry)
    return out


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": r5(min(x0, x1)), "y": r5(min(y0, y1)), "w": r5(abs(x1 - x0)), "h": r5(abs(y1 - y0))}


def poly(points, closed=True):
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in points], "closed": closed}


def box_poly(x0, y0, x1, y1):
    return poly([[x1, y0], [x1, y1], [x0, y1], [x0, y0]])


def disc(cx, cy, r):
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(r), "ry": r5(r)}


def stadium(x0, y0, x1, y1, r):
    return {"kind": "capsule", "x0": r5(x0), "y0": r5(y0), "x1": r5(x1), "y1": r5(y1), "r0": r5(r), "r1": r5(r)}
