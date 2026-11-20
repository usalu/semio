"""🛢️ Round 3 kit of `machinery/oil-pumpjack`: one table of real dimensions of a conventional crank-balanced 74-in-stroke beam pumping unit (API 11E letters A, C, I, P, H, G, R; Lufkin C-228D-173-74 class) that the corrected rig, the 2D drawings of every scale and every view, and the solids of every scale and list all read. World rest coordinates: x ahead (towards the motor), y up, z lateral (left +); every part is stated by hand here and converted to the rest frame of its bone.

See `r3-machinery-oil-pumpjack.md` for the reference table and the rulings behind each number.
"""

import math

sin, cos, pi, rad = math.sin, math.cos, math.pi, math.radians

A_HEAD = 3.0
C_TAIL = 2.7
P_PITMAN = 2.65
R_CRANK = 0.85
PIVOT = (0.0, 4.4)
CRANK = (2.7, 1.75)
WELL_X = -3.0
ROD_Y = 3.18
MOTOR = (4.9, 0.9)

SLAB = (-1.9, 5.9, 0.0, 0.3, 1.2)
SKID_X0, SKID_X1, SKID_Y0, SKID_Y1, SKID_Z = -1.6, 5.6, 0.3, 0.55, 1.0
IB_Z, IB_BF, IB_TF, IB_TW = 0.9, 0.2, 0.015, 0.012
CROSS_X = [-1.3, 0.3, 1.9, 3.5, 5.1]

FOOT_X, FOOT_Z = 1.5, 0.85
TOP_X, TOP_Z, TOP_Y = 0.08, 0.2, 4.11
LEG = 0.17
CAP_Y0, CAP_Y1, CAP_HW = 4.11, 4.15, 0.30
BLOCK_HX, BLOCK_Y1, BLOCK_Z0, BLOCK_Z1 = 0.15, 4.48, 0.24, 0.36
BRACE_Y, BRACE_T = 1.68, 0.10

BEAM_X0, BEAM_X1 = -2.15, 2.92
BEAM_V0, BEAM_V1 = 0.22, 0.83
BEAM_BF, BEAM_TF, BEAM_TW = 0.325, 0.019, 0.0127
HOUSING_R, HOUSING_HW, HOUSING_PLATE = 0.20, 0.30, 0.03
HANGER_R, HANGER_HW = 0.16, 0.22

HEAD_A0, HEAD_A1 = 160.0, 200.0
HEAD_REAR_X, HEAD_REAR_TOP, HEAD_REAR_BOT = -2.02, 0.98, -0.25
HEAD_HW, FACE_HW, FACE_T = 0.28, 0.35, 0.03

PIT_Z0, PIT_Z1 = 0.77, 0.89
PIT_DEPTH = 0.16
EYE_R, EYE_Z0, EYE_Z1 = 0.14, 0.75, 0.91
EQ_HW, EQ_U0, EQ_U1, EQ_V = 0.91, -0.07, 0.17, 0.11

ARM_Z0, ARM_Z1 = 0.59, 0.75
CW_Z0, CW_Z1 = 0.43, 0.59
CW_R0, CW_R1, CW_A = 0.55, 1.12, 38.0
TAIL_C, TAIL_R, TIP_C, TIP_R = -0.08, 0.22, 0.96, 0.16
HUB_R, HUB_Z0, HUB_Z1 = 0.26, 0.48, 0.78
PIN_R, PIN_Z1 = 0.085, 0.93

GB_HW = 0.42
INPUT = (1.25, -0.55)
GUARD_Z0, GUARD_Z1 = -1.13, -0.93
GUARD_RA, GUARD_RB = 0.58, 0.20

MOTOR_R, MOTOR_CORE, MOTOR_Z0, MOTOR_Z1 = 0.27, 0.225, -0.48, 0.06
COWL_R, COWL_Z1 = 0.235, 0.26
SHAFT_Z0 = -0.98

CARRIER_HZ, CARRIER_HU, CARRIER_HV = 0.28, 0.06, 0.05

FRAMES = {
    "base": ((0.0, 0.0), 0),
    "skid": ((-1.6, 0.3), 0),
    "post": ((0.0, 0.55), 90),
    "beam": (PIVOT, 0),
    "horsehead": (PIVOT, 180),
    "pitman": ((PIVOT[0] + C_TAIL, PIVOT[1]), -90),
    "gearbox": (CRANK, 0),
    "crank": (CRANK, 0),
    "rod": ((WELL_X, ROD_Y), 90),
    "motor": (MOTOR, 0),
}


def r4(x):
    v = round(x, 4)
    return 0.0 if v == 0 else v


def r5(x):
    v = round(x, 5)
    return 0.0 if v == 0 else v


def axes(bone):
    (ox, oy), deg = FRAMES[bone]
    a = rad(deg)
    return (ox, oy), (cos(a), sin(a)), (-sin(a), cos(a))


def L2(bone, p):
    """World rest (x, y) to bone-local (u, v)."""
    (ox, oy), u, v = axes(bone)
    dx, dy = p[0] - ox, p[1] - oy
    return [r5(dx * u[0] + dy * u[1]), r5(dx * v[0] + dy * v[1])]


def L3(bone, p):
    return L2(bone, p[:2]) + [r5(p[2])]


def D3(bone, d):
    _, u, v = axes(bone)
    return [r5(d[0] * u[0] + d[1] * u[1]), r5(d[0] * v[0] + d[1] * v[1]), r5(d[2])]


def W2(bone, p):
    """Bone-local (u, v) to world rest (x, y)."""
    (ox, oy), u, v = axes(bone)
    return (ox + p[0] * u[0] + p[1] * v[0], oy + p[0] * u[1] + p[1] * v[1])


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * cos(rad(a0 + (a1 - a0) * i / n)), cy + r * sin(rad(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def circle(cx, cy, r, n):
    return [(cx + r * cos(2 * pi * i / n), cy + r * sin(2 * pi * i / n)) for i in range(n)]


def hull2(c1, r1, c2, r2, n1, n2):
    """Convex hull of two circles (belt guard, crank arm), counter-clockwise."""
    dx, dy = c2[0] - c1[0], c2[1] - c1[1]
    d = math.hypot(dx, dy)
    base = math.degrees(math.atan2(dy, dx))
    phi = math.degrees(math.acos((r1 - r2) / d))
    return arc(c2[0], c2[1], r2, base - phi, base + phi, n2) + arc(c1[0], c1[1], r1, base + phi, base + 360 - phi, n1)


def sector(cx, cy, r0, r1, a, n_out, n_in, chamfer=0.0):
    """Annular sector about +u of half angle a (counterweight), chamfered outer corners."""
    out = []
    if chamfer:
        ca = math.degrees(chamfer / r1)
        out += [(cx + (r1 - chamfer) * cos(rad(-a)), cy + (r1 - chamfer) * sin(rad(-a)))]
        out += arc(cx, cy, r1, -a + ca, a - ca, n_out)
        out += [(cx + (r1 - chamfer) * cos(rad(a)), cy + (r1 - chamfer) * sin(rad(a)))]
    else:
        out += arc(cx, cy, r1, -a, a, n_out)
    out += arc(cx, cy, r0, a, -a, n_in)
    return out


def head_outline(n):
    """Horsehead side outline in BEAM coordinates: face arc of radius A about the pivot, rear on the beam end."""
    pts = arc(0, 0, A_HEAD, HEAD_A1, HEAD_A0, n)
    return pts + [(HEAD_REAR_X, HEAD_REAR_TOP), (HEAD_REAR_X, HEAD_REAR_BOT)]


def face_band(n):
    return arc(0, 0, A_HEAD, HEAD_A1, HEAD_A0, n) + arc(0, 0, A_HEAD - FACE_T, HEAD_A0, HEAD_A1, n)


def housing(cx, hw, r, plate_hw, top, plate, n):
    """Bearing housing under a beam flange (beam coordinates): bolting plate, cheeks, round bottom about the pin."""
    pts = [(cx - plate_hw, top), (cx + plate_hw, top), (cx + plate_hw, top - plate), (cx + hw, top - plate)]
    if hw > r:
        pts.append((cx + hw, 0.0))
    pts += arc(cx, 0, r, 0, -180, n)
    if hw > r:
        pts.append((cx - hw, 0.0))
    pts += [(cx - hw, top - plate), (cx - plate_hw, top - plate)]
    return pts


def gearbox_outline(n_top, n_in):
    """Reducer housing side profile (gearbox coordinates, low-speed shaft at 0,0): arc over the bull gear, sloping top to the input end, base flange."""
    top = arc(0, 0, 0.60, 40, 155, n_top)
    top.reverse()
    rear = arc(INPUT[0], INPUT[1], 0.40, 70, 0, n_in)
    return [(-0.64, -0.97), (-0.70, -0.97), (-0.70, -1.05), (1.72, -1.05), (1.72, -0.97), (1.65, -0.97), (1.65, -0.55)] + list(reversed(rear))[1:] + list(reversed(top))


def gearbox_simple():
    return [(-0.70, -1.05), (1.72, -1.05), (1.72, -0.97), (1.65, -0.97), (1.65, -0.40), (0.42, 0.43), (0.0, 0.60), (-0.42, 0.43), (-0.6, 0.0), (-0.64, -0.97), (-0.70, -0.97)]


def motor_fins(n_fins, core, tip, half_fin):
    pts = []
    step = 360.0 / n_fins
    for i in range(n_fins):
        c = 90 + i * step
        hf = math.degrees(half_fin / tip)
        hc = math.degrees(half_fin / core)
        pts += [(core * cos(rad(c - hc)), core * sin(rad(c - hc))), (tip * cos(rad(c - hf)), tip * sin(rad(c - hf))), (tip * cos(rad(c + hf)), tip * sin(rad(c + hf))), (core * cos(rad(c + hc)), core * sin(rad(c + hc)))]
        pts += arc(0, 0, core, c + hc, c + step - hc, 2)[1:-1]
    return pts


def cap_y0(scale):
    return {500: 4.075, 100: 4.10, 200: 4.10}.get(scale, CAP_Y0)


def eq_u0(scale):
    return -0.20 if scale == 500 else EQ_U0


def carrier_h(scale):
    return (0.08, 0.08) if scale == 500 else (CARRIER_HU, CARRIER_HV)


def leg_axis(sx, sz):
    return (sx * FOOT_X, 0.55, sz * FOOT_Z), (sx * TOP_X, TOP_Y, sz * TOP_Z)


def strip(p0, p1, half, y0, y1, a=0, b=1):
    """Elevation (a=0: x, b=1: y) or front (a=2: z) outline of a straight leg of half width `half`, cut at heights y0 and y1."""
    d = (p1[a] - p0[a], p1[1] - p0[1])
    n = math.hypot(*d)
    d = (d[0] / n, d[1] / n)
    nrm = (-d[1], d[0])
    out = []
    for y, sides in ((y0, (1, -1)), (y1, (-1, 1))):
        for s in sides:
            ox, oy = p0[a] + s * half * nrm[0], p0[1] + s * half * nrm[1]
            t = (y - oy) / d[1]
            out.append((ox + t * d[0], y))
    return out


def quat(x, y, z):
    m00, m10, m20 = x
    m01, m11, m21 = y
    m02, m12, m22 = z
    t = m00 + m11 + m22
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1 + m00 - m11 - m22) * 2
        q = [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    elif m11 > m22:
        s = math.sqrt(1 + m11 - m00 - m22) * 2
        q = [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
    else:
        s = math.sqrt(1 + m22 - m00 - m11) * 2
        q = [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
    n = math.sqrt(sum(c * c for c in q))
    return [r5(c / n) for c in q]


def unit(a):
    n = math.sqrt(sum(c * c for c in a)) or 1.0
    return [c / n for c in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


class Solids:
    """Writers of the six solid kinds in the rest frame of a bone, from world rest coordinates."""

    def __init__(self):
        self.items = []

    def _put(self, d, extra):
        for k, v in extra.items():
            if v is not None:
                d[k] = v
        self.items.append(d)
        return d

    def head(self, id, kind, bone, material, tone, role="body"):
        return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}

    def box(self, id, bone, material, tone, centre, ext, cr=None, role="body", **extra):
        """Axis-aligned world box (extents ext along world x, y, z) on a bone whose frame is a quarter turn of the world."""
        d = self.head(id, "box", bone, material, tone, role)
        u = [abs(c) for c in D3(bone, [ext[0], 0, 0])]
        v = [abs(c) for c in D3(bone, [0, ext[1], 0])]
        size = [r5(u[0] + v[0]), r5(u[1] + v[1]), r5(ext[2])]
        d["size"] = size
        if cr:
            d["cornerRadius"] = r5(cr)
        d["transform"] = {"translation": L3(bone, centre)}
        return self._put(d, extra)

    def ell(self, id, bone, material, tone, centre, radii_world, role="body", **extra):
        d = self.head(id, "ellipsoid", bone, material, tone, role)
        u = [abs(c) for c in D3(bone, [radii_world[0], 0, 0])]
        v = [abs(c) for c in D3(bone, [0, radii_world[1], 0])]
        d["radii"] = [r5(u[0] + v[0]), r5(u[1] + v[1]), r5(radii_world[2])]
        d["transform"] = {"translation": L3(bone, centre)}
        return self._put(d, extra)

    def rev(self, id, bone, material, tone, origin, axis, profile, role="body", **extra):
        d = self.head(id, "revolve", bone, material, tone, role)
        d["profile"] = [[r5(a), r5(b)] for a, b in profile]
        d["axis"] = D3(bone, unit(axis))
        d["origin"] = L3(bone, origin)
        return self._put(d, extra)

    def cyl(self, id, bone, material, tone, a, b, r, role="body", **extra):
        ln = math.sqrt(sum((q - p) ** 2 for p, q in zip(a, b)))
        return self.rev(id, bone, material, tone, a, [q - p for p, q in zip(a, b)], [[0, r], [ln, r]], role, **extra)

    def side(self, id, bone, material, tone, pts, z0, z1, role="body", local=False, holes=None, **extra):
        """Extrude of a side (x, y) profile across z0..z1; pts in world rest coordinates, or bone-local when local=True."""
        d = self.head(id, "extrude", bone, material, tone, role)
        conv = (lambda p: [r5(p[0]), r5(p[1])]) if local else (lambda p: L2(bone, p))
        prof = [conv(p) for p in pts]
        if signed_area(prof) < 0:
            prof.reverse()
        d["profile"] = prof
        if holes:
            d["holes"] = [[conv(p) for p in h] for h in holes]
        d["plane"] = "xy"
        d["from"], d["to"] = r5(min(z0, z1)), r5(max(z0, z1))
        return self._put(d, extra)

    def along(self, id, bone, material, tone, section, u0, u1, role="body", **extra):
        """Extrude of a (v, w) section along the bone axis u from u0 to u1 (rolled beams)."""
        d = self.head(id, "extrude", bone, material, tone, role)
        prof = [[r5(a), r5(b)] for a, b in section]
        if signed_area(prof) < 0:
            prof.reverse()
        d["profile"] = prof
        d["plane"] = "yz"
        d["from"], d["to"] = r5(min(u0, u1)), r5(max(u0, u1))
        return self._put(d, extra)

    def loft(self, id, bone, material, tone, secs, hint=(0, 0, 1), caps=(True, True), role="body", **extra):
        """secs: (world centre, rx along hint, ry across, exponent)."""
        d = self.head(id, "loft", bone, material, tone, role)
        cs = [L3(bone, s[0]) for s in secs]
        hl = D3(bone, list(hint))
        out = []
        for i, s in enumerate(secs):
            a = cs[max(i - 1, 0)]
            b = cs[min(i + 1, len(cs) - 1)]
            z = unit([q - p for p, q in zip(a, b)])
            x = [h - c * dot(hl, z) for h, c in zip(hl, z)]
            x = unit(x)
            y = cross(z, x)
            sec = {"at": cs[i], "rx": r5(s[1]), "ry": r5(s[2])}
            if len(s) > 3:
                sec["exponent"] = s[3]
            sec["rotation"] = quat(x, y, z)
            out.append(sec)
        d["sections"] = out
        d["capStart"], d["capEnd"] = caps
        return self._put(d, extra)


def signed_area(p):
    return 0.5 * sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))


def ibeam(v0, v1, hw, tf, tw):
    """(v, w) section of a rolled I: flanges of half width hw and thickness tf, web tw, from v0 to v1."""
    return [(v0, -hw), (v0, hw), (v0 + tf, hw), (v0 + tf, tw / 2), (v1 - tf, tw / 2), (v1 - tf, hw), (v1, hw), (v1, -hw), (v1 - tf, -hw), (v1 - tf, -tw / 2), (v0 + tf, -tw / 2), (v0 + tf, -hw)]
