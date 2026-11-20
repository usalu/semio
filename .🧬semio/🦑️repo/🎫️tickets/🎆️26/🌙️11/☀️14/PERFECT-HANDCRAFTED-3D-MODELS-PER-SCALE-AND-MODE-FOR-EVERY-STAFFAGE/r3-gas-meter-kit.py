"""🧯 Round 3 gas meter kit: the measures of the real compact G4 diaphragm meter with a folded sheet-steel housing and 110 mm connection spacing (Elster BK-G4 compact: 193 mm wide, 221 mm high over the bare stubs, 157 mm deep, 1.9 kg, thread M30 x 2, cyclic volume 1.2 litres) and small writers for the solids. Every section is stated here or in `r3-gas-meter-model.py`; nothing is guessed from the drawings.

World frame: origin on the seam plane, on the axis plane of the two stubs, in the middle of the width; x across the front (the side elevation looks from -z at the index), y up, z depth from the index glass (z -0.090) to the rear of the seam bead (z +0.067). The body bone has angle 0, so its frame is the world frame. The pointer bone sits at (0, 0.027) and points up at rest (angle 90): u is world y, v is world -x, w is world z.
"""
import math

A, B = 0.0915, 0.062
N = 5.5
DRAFT = 0.0025
R = 0.018
YT, YB = 0.095, -0.100
YST, YSB = 0.077, -0.082
BEAD = (0.0965, 0.067, 0.0045)
XS = 0.055
STUB_R, BORE_R, BOSS_R = 0.015, 0.011, 0.018
NUT_R, NUT_Y0, NUT_Y1 = 0.0208, 0.108, 0.125
TAIL_R, TAIL_BORE = 0.013, 0.010
Y_STUB_TOP, Y_TOP = 0.1215, 0.145
IX, IY0, IY1, IR = 0.065, 0.006, 0.072, 0.006
IZ_BACK, IZ_FACE, IZ_FRONT = -0.0575, -0.0825, -0.090
WIN = (0.059, 0.009, 0.069)
DIAL = (0.0, 0.027, 0.018)
COUNTER = (0.104, 0.018, 0.059)
DIGIT = (0.011, 0.014, 0.0126, -0.0441)
PLATE = (0.055, -0.075, -0.035)
COM_Y = -0.02
YAX, ZAX = [0, 1, 0], [0, 0, 1]


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


def rrect(x0, y0, x1, y1, r, n=4):
    """▢ A rounded rectangle, counter-clockwise, n segments per corner (r 0: sharp)."""
    if r <= 0:
        return [(x1, y0), (x1, y1), (x0, y1), (x0, y0)]
    out = []
    for cx, cy, a0 in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n)
            out.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return out


def circle(cx, cy, r, n=24):
    """⭕ A circle polygon, counter-clockwise."""
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def hexagon(cz, cx, rad):
    """⬡ A hexagon in the zx plane with two corners on the x direction (across corners 2 rad, across flats sqrt(3) rad)."""
    return [(cz + rad * math.sin(i * math.pi / 3), cx + rad * math.cos(i * math.pi / 3)) for i in range(6)]


def superellipse(rx, rz, n=N, count=48):
    """🥚 The plan outline of a pressed shell, |x/rx|^n + |z/rz|^n = 1, as (z, x) points for the zx plane."""
    out = []
    for i in range(count):
        t = 2 * math.pi * i / count
        c, s = math.cos(t), math.sin(t)
        out.append((rz * math.copysign(abs(s) ** (2 / n), s), rx * math.copysign(abs(c) ** (2 / n), c)))
    return out


def shoulder(y_start, sign, steps):
    """🏺 The drawn shoulder of a shell: a quarter circle of radius R from the wall (inset 0) to the flat crown (inset R), as (y, inset) pairs without the start."""
    return [(y_start + sign * R * math.sin(math.radians(90 * k / steps)), R * (1 - math.cos(math.radians(90 * k / steps)))) for k in range(1, steps + 1)]


def shell_sections(upper, steps=6):
    """🫙 The loft sections of one deep-drawn shell from the seam to its crown: the wall with its draft, then the shoulder."""
    y_s, sign = (YST, 1) if upper else (YSB, -1)
    secs = [((0, 0, 0), A, B, N), ((0, y_s, 0), A - DRAFT, B - DRAFT, N)]
    for y, inset in shoulder(y_s, sign, steps):
        secs.append(((0, y, 0), A - DRAFT - inset, B - DRAFT - inset, N))
    return secs


def face_z(y, x=0.0):
    """🪞 The z of the shell front face at height y and position x (draft and the crown of the superellipse)."""
    y_s = YST if y >= 0 else -YSB
    k = min(abs(y) / y_s, 1.0)
    rx, rz = A - DRAFT * k, B - DRAFT * k
    return -rz * (1 - abs(x / rx) ** N) ** (1 / N)


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


def loft(sid, bone, material, role, tone, sections, cap_start=True, cap_end=True, **kw):
    """🧵 A loft of superelliptic sections (rx along x; ry along z for a path along y, along y for a path along z)."""
    secs = []
    for at, rx, ry, e in sections:
        sec = {"at": [r5(v) for v in at], "rx": r5(rx), "ry": r5(ry)}
        if e is not None:
            sec["exponent"] = e
        secs.append(sec)
    return solid("loft", sid, bone, material, role, tone, sections=secs, capStart=cap_start, capEnd=cap_end, **kw)


def ell(sid, bone, material, role, tone, radii, at, **kw):
    """🥚 An ellipsoid centred at `at`."""
    return solid("ellipsoid", sid, bone, material, role, tone, radii=[r5(v) for v in radii], transform={"translation": [r5(v) for v in at]}, **kw)


def twice(s):
    """👯 Repeats a solid authored at the inlet stub (x -XS) at the outlet stub."""
    s = dict(s)
    s["repeat"] = {"count": 2, "translation": [2 * XS, 0, 0]}
    return s


def ref(sid, of, **kw):
    """🔗 A thermal or structural reference to a body solid."""
    s = {"id": sid, "of": of}
    s.update(kw)
    return s


def th(s, region, **kw):
    """🌡️ Marks an own solid as thermal."""
    s = dict(s)
    s["region"] = region
    s.update(kw)
    return s


def st(s, member, part, **kw):
    """🏗️ Marks an own solid as structural."""
    s = dict(s)
    s["member"] = member
    s["part"] = part
    s.update(kw)
    return s
