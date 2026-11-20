"""🔥 Round 3 gas boiler kit: the measures of the real 20 kW wall-hung condensing boiler (Vaillant ecoTEC plus class: casing 720 x 440 x 335 mm) and small writers for the solids. Every section is stated here or in `r3-gas-boiler-model.py`; nothing is guessed from the drawings.

World frame: origin at the lowest point of the connection stubs below the casing centre, y up, x across the front (the side elevation looks from -z at the front face), z depth from the front face (z -0.155) to the wall (z +0.175). Every bone has angle 0, so a bone frame is the world frame moved to the bone origin: casing and connections (0, 0), collar (0, 0.77), mode-knob (0.04, 0.14), flame and flame-tall (0, 0.541).
"""
import math

Y0, Y1 = 0.05, 0.77
XW = 0.22
ZF, ZW = -0.155, 0.175
ZSEAM = 0.155
RC = 0.014
WIN = (0.0, 0.56)
WIN_R = 0.0365
COLLAR = 0.77
COLLAR_Z = 0.04
KNOB = (0.04, 0.14)
FLAME = 0.541
STUB_X = [-0.16 + 0.08 * k for k in range(5)]
STUB_Z = 0.07
BURNER_Z = -0.107


def r5(x):
    """🔢 Rounds to 0.01 mm so the source stays readable."""
    return round(x + 0.0, 5)


def pts(seq):
    """📐 Rounds a point list."""
    return [[r5(a), r5(b)] for a, b in seq]


def loop(seq):
    """➰ Closes a revolve profile into a loop (first point repeated last)."""
    seq = [tuple(p) for p in seq]
    return seq + [seq[0]]


def rrect(x0, y0, x1, y1, r, n=6):
    """▢ A rounded rectangle, counter-clockwise, n segments per corner (r 0: sharp)."""
    if r <= 0:
        return [(x1, y0), (x1, y1), (x0, y1), (x0, y0)]
    out = []
    for cx, cy, a0 in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n)
            out.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return out


def densify(poly, step):
    """➗ Splits every straight edge longer than step so the caps of an extrude triangulate without long slivers."""
    out = []
    for i, a in enumerate(poly):
        b = poly[(i + 1) % len(poly)]
        n = max(1, int(math.ceil(math.hypot(b[0] - a[0], b[1] - a[1]) / step)))
        for k in range(n):
            out.append((a[0] + (b[0] - a[0]) * k / n, a[1] + (b[1] - a[1]) * k / n))
    return out


def circle(cx, cy, r, n=32):
    """⭕ A circle polygon, counter-clockwise."""
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def hexagon(cz, cx, rad):
    """⬡ A hexagon in the zx plane with its corners at plus and minus x (across corners 2 rad, across flats sqrt(3) rad)."""
    return [(cz + rad * math.sin(i * math.pi / 3), cx + rad * math.cos(i * math.pi / 3)) for i in range(6)]


def solid(kind, sid, bone, material, role, tone, **kw):
    """🧱 One solid with the keys in schema order."""
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update({k: v for k, v in kw.items() if v is not None})
    return s


def box(sid, bone, material, role, tone, size, at, radius=None, **kw):
    """📦 A box centred at `at` in the bone frame."""
    return solid("box", sid, bone, material, role, tone, size=[r5(v) for v in size], cornerRadius=radius, transform={"translation": [r5(v) for v in at]}, **kw)


def ext(sid, bone, material, role, tone, plane, profile, a, b, holes=None, **kw):
    """🧊 An extrude of a polygon (with holes) on a coordinate plane from a to b."""
    s = solid("extrude", sid, bone, material, role, tone, profile=pts(profile), plane=plane, **kw)
    s["from"], s["to"] = r5(a), r5(b)
    if holes:
        s["holes"] = [pts(h) for h in holes]
    return s


def rev(sid, bone, material, role, tone, profile, origin, axis, **kw):
    """🌀 A revolve: profile of [axial, radial] about the axis through origin; a loop revolves the enclosed ring."""
    return solid("revolve", sid, bone, material, role, tone, profile=pts(profile), axis=list(axis), origin=[r5(v) for v in origin], **kw)


def loft(sid, bone, material, role, tone, sections, **kw):
    """🧵 A loft of superelliptic sections (rx along x, ry along z, stacked along y in the bone frame), capped at both ends."""
    secs = []
    for at, rx, ry, e in sections:
        sec = {"at": [r5(v) for v in at], "rx": r5(rx), "ry": r5(ry)}
        if e is not None:
            sec["exponent"] = e
        secs.append(sec)
    return solid("loft", sid, bone, material, role, tone, sections=secs, capStart=True, capEnd=True, **kw)


def cap(sid, bone, material, role, tone, p0, p1, r, **kw):
    """💊 A capsule of radius r from p0 to p1."""
    return solid("capsule", sid, bone, material, role, tone, p0=[r5(v) for v in p0], p1=[r5(v) for v in p1], r0=r5(r), r1=r5(r), **kw)


def ell(sid, bone, material, role, tone, radii, at, **kw):
    """🥚 An ellipsoid centred at `at`."""
    return solid("ellipsoid", sid, bone, material, role, tone, radii=[r5(v) for v in radii], transform={"translation": [r5(v) for v in at]}, **kw)


def ref(sid, of, **kw):
    """🔗 A thermal or structural reference to a body solid."""
    s = {"id": sid, "of": of}
    s.update(kw)
    return s


def th(s, region, layer, **kw):
    """🌡️ Marks an own solid as thermal."""
    s = dict(s)
    s["region"] = region
    if layer:
        s["layer"] = layer
    s.update(kw)
    return s


def st(s, member, part, **kw):
    """🏗️ Marks an own solid as structural."""
    s = dict(s)
    s["member"] = member
    s["part"] = part
    s.update(kw)
    return s
