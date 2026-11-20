"""🧰 Round 3 panel radiator: the handcrafted dimensions of the type 22 compact valve radiator (Ventilkompakt) and the section builders every scale uses.

Frame: bone `radiator` (u up from the lower edge of the panels at 0.11 m above the floor, v towards the wall, the rear face of the rear panel at v = 0 and the wall at v = +0.035, w lateral, +w = the asset's left = the right end seen from the room; the valve sits at the other end, -w, on the left seen from the room, the half the section A-A keeps).
Bone `valve-head` sits on the joint of the thermostat head (u 0.55, v -0.05), its axis along w.

@see ./round3-brief.md
@see ./r3-components-panel-radiator.md
"""
import math

L = 1.0
HALF = L / 2
PANEL_H = 0.585
TOP = 0.6
WALL = 0.035
REAR = (-0.022, 0.0)
FRONT = (-0.1, -0.078)
FIN_REAR = (-0.0495, -0.0215)
FIN_FRONT = (-0.0785, -0.0505)
CHANNELS = 30
GROOVE = 0.004
HUB = (0.0175, 0.5675)
HEAD_U, HEAD_V = 0.55, -0.05
HEAD_R, COLLAR_R = 0.026, 0.018
HEAD_W = (-0.497, -0.512, -0.515, -0.585, -0.595)
PORTS_W = (-0.465, -0.415)
PORT_V = -0.05
BLOCK = {"u": (-0.065, -0.03), "v": (-0.068, -0.032), "w": (-0.49, -0.39)}
FLOOR_U = -0.11
BRACKET_W = (0.38, 0.42)
UPPER_ARM = (0.47, 0.482)
LOWER_ARM = (0.05, 0.062)
FLANGE_T = 0.003
SLOT_V = (-0.015, -0.04, -0.06, -0.085)
SLOT_W, SLOT_LEN, SLOTS = 0.008, 0.1, 8
PLUG_R, PLUG_OUT = 0.011, 0.0075


def r5(x):
    """🔢 Five decimals, the precision of the sources."""
    return round(x + 0.0, 5)


def pts(seq):
    """📐 A point list rounded to the source precision."""
    return [[r5(a), r5(b)] for a, b in seq]


def base(sid, bone, material, role, tone, kind):
    """🧱 The members every solid has."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def box(sid, at, size, corner=0.0, bone="radiator", material="painted-steel", role="body", tone=0.7, **extra):
    """📦 A box centred at `at` with edge lengths `size` along u, v, w."""
    s = base(sid, bone, material, role, tone, "box")
    s["size"] = [r5(x) for x in size]
    if corner > 0:
        s["cornerRadius"] = r5(corner)
    s["transform"] = {"translation": [r5(x) for x in at]}
    s.update(extra)
    return s


def span_box(sid, u, v, w, corner=0.0, **kw):
    """📦 A box between the given (min, max) ranges along u, v and w."""
    at = [(u[0] + u[1]) / 2, (v[0] + v[1]) / 2, (w[0] + w[1]) / 2]
    size = [u[1] - u[0], v[1] - v[0], w[1] - w[0]]
    return box(sid, at, size, corner, **kw)


def extrude(sid, plane, profile, frm, to, holes=None, bone="radiator", material="painted-steel", role="body", tone=0.7, **extra):
    """🥫 An extrusion: plane xy (u,v) along w, yz (v,w) along u, zx (w,u) along v."""
    s = base(sid, bone, material, role, tone, "extrude")
    s["profile"] = pts(profile)
    if holes:
        s["holes"] = [pts(h) for h in holes]
    s["plane"] = plane
    s["from"] = r5(frm)
    s["to"] = r5(to)
    s.update(extra)
    return s


def revolve(sid, profile, axis, origin, bone="radiator", material="brass", role="detail", tone=0.3, **extra):
    """🏺 A closed [axial, radial] profile revolved about `axis` through `origin`."""
    s = base(sid, bone, material, role, tone, "revolve")
    s["profile"] = pts(profile)
    s["axis"] = axis
    s["origin"] = [r5(x) for x in origin]
    s.update(extra)
    return s


def ellipsoid(sid, at, radii, bone="radiator", material="painted-steel", role="body", tone=0.5, **extra):
    """🥚 An ellipsoid centred at `at`."""
    s = base(sid, bone, material, role, tone, "ellipsoid")
    s["radii"] = [r5(x) for x in radii]
    s["transform"] = {"translation": [r5(x) for x in at]}
    s.update(extra)
    return s


def groove_depth(w):
    """〰️ Depth of the pressed seam between two vertical water channels at w: 30 channels at 33.3 mm pitch, a 8 mm flat seam, a rounded flank over 9.5 mm, full thickness on the channel crown."""
    pitch = L / CHANNELS
    k = round((w + HALF) / pitch)
    if k <= 0 or k >= CHANNELS:
        return 0.0
    d = abs(w - (-HALF + k * pitch))
    if d <= 0.004:
        return GROOVE
    if d >= 0.0135:
        return 0.0
    t = (d - 0.004) / 0.0095
    return GROOVE * (0.5 + 0.5 * math.cos(math.pi * t))


def profile_ws():
    """📍 The w stations of the channel profile: crowns and both ends of every seam flank."""
    pitch = L / CHANNELS
    ws = [-HALF, HALF]
    for k in range(1, CHANNELS):
        c = -HALF + k * pitch
        for off in (-0.0135, -0.0105, -0.0075, -0.004, 0.004, 0.0075, 0.0105, 0.0135):
            ws.append(c + off)
    return sorted(set(round(w, 6) for w in ws))


def channel_profile(v_lo, v_hi):
    """🌊 The horizontal section (v, w) of one pressed water panel between v_lo and v_hi: two 1.25 mm sheets welded along 29 vertical seams, both faces grooved `GROOVE` deep, full thickness on the channel crowns and the end channels."""
    ws = profile_ws()
    return [(v_lo + groove_depth(w), w) for w in ws] + [(v_hi - groove_depth(w), w) for w in reversed(ws)]


def fin_profile(v_base, v_top, w0, w1, sheet=0.0006, pitch=0.025):
    """🪗 The horizontal section (v, w) of one trapezoidal convector fin sheet (`sheet` thick), its feet at `v_base` (welded to the panel), its crowns at `v_top`, pitch 25 mm: foot 6.5, flank 6, crown 6.5, flank 6 mm."""
    n = int(round((w1 - w0) / pitch))
    centre = []
    for k in range(n):
        a = w0 + k * pitch
        centre += [(v_base, a), (v_base, a + 0.0065), (v_top, a + 0.0125), (v_top, a + 0.019)]
    centre.append((v_base, w0 + n * pitch))
    return offset_strip(centre, sheet / 2)


def offset_strip(line, h):
    """📏 A closed strip of half thickness `h` around an open polyline (v, w), mitred at every bend."""
    def normal(a, b):
        dv, dw = b[0] - a[0], b[1] - a[1]
        n = math.hypot(dv, dw)
        return (-dw / n, dv / n)
    side = []
    for sign in (1, -1):
        out = []
        for i, p in enumerate(line):
            ns = [normal(line[j], line[j + 1]) for j in (i - 1, i) if 0 <= j < len(line) - 1]
            mv, mw = sum(n[0] for n in ns) / len(ns), sum(n[1] for n in ns) / len(ns)
            k = math.hypot(mv, mw)
            mv, mw = mv / k, mw / k
            scale = h / max(0.3, mv * ns[0][0] + mw * ns[0][1])
            out.append((p[0] + sign * mv * scale, p[1] + sign * mw * scale))
        side.append(out)
    return side[0] + list(reversed(side[1]))


def slot_holes():
    """🕳️ The 4 x 8 slots of the top grille (8 x 100 mm, 15.7 mm bridges) in the (v, w) plane."""
    gap = (0.91 - SLOTS * SLOT_LEN) / (SLOTS - 1)
    out = []
    for v in SLOT_V:
        for k in range(SLOTS):
            a = -0.455 + k * (SLOT_LEN + gap)
            out.append([(v - SLOT_W / 2, a), (v - SLOT_W / 2, a + SLOT_LEN), (v + SLOT_W / 2, a + SLOT_LEN), (v + SLOT_W / 2, a)])
    return out


def hexagon(cv, cw, af):
    """⬡ A hexagon nut section (v, w) with `af` across flats, flats facing the ends (±w), corners towards the room and the wall."""
    rc = af / math.sqrt(3)
    return [(cv + rc * math.sin(math.radians(a)), cw + rc * math.cos(math.radians(a))) for a in (30, 90, 150, 210, 270, 330)]
