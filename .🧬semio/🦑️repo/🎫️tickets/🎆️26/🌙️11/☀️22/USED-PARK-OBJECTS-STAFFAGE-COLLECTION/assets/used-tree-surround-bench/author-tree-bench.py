"""🌳️ Authoritative generator of the used tree surround bench (Baumrundbank) asset document.

Every number of the asset is stated here by hand and written to `🔣️.json` next to this file:
`python3 author-tree-bench.py` (re-run after every change; it prints masses, extents and areas).

@see ../../brief.md — the brief of the used park objects wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the staffage contract
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SQ3 = math.sqrt(3.0)
TAN30 = 1.0 / SQ3
COS30 = SQ3 / 2.0

A_OUT = 1.039
DEPTH = 0.450
A_IN = A_OUT - DEPTH
SLAT_W = 0.082
SLAT_GAP = 0.010
SLAT_T = 0.040
SEAT_TOP = 0.450
SLAT_BOT = SEAT_TOP - SLAT_T
END_GAP = 0.005
CARRIER_T = 0.008
CARRIER_D = (0.005, 0.035)
BEARER_R = (0.690, 1.180)
BEARER_W = 0.080
BEARER_H = 0.040
BEARER_TOP = SLAT_BOT - CARRIER_T
BEARER_BOT = BEARER_TOP - BEARER_H
LEG_R = (0.740, 1.100)
LEG_S = 0.060
LEG_WALL = 0.003
END_PLATE = (0.080, 0.008)
NUT_AF = 0.024
NUT_H = 0.013
THREAD_D = 0.016
FOOT = (0.100, 0.010)
PAD = (0.300, 0.080)
LIFT = 0.030
LEG_BOTTOM = 0.088
NUT_BOT = LEG_BOTTOM - END_PLATE[1] - NUT_H
ROBINIA = 730.0
STEEL = 7850.0
CONCRETE = 2300.0
G = 9.80665
SEGMENTS = list(range(6))
SEG_PHI = [60.0 * k for k in SEGMENTS]
BEARER_PSI = [((30.0 + 60.0 * k + 180.0) % 360.0) - 180.0 for k in SEGMENTS]
LIFTED = ("0", "i")
REPLACED = (4, 1)
SPLITS = {(1, 2): "+", (3, 0): "-"}
ALGAE_SEGMENTS = (2, 3)
SCALES = ["50", "100", "200", "500"]
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r5(v):
    """✂️ Rounds a metre value to 0.01 mm."""
    return round(v + 0.0, 5)


def unit(deg):
    """🧭️ The plan unit vector (x, z) at an angle from +x towards +z."""
    a = math.radians(deg)
    return (math.cos(a), math.sin(a))


def seg_point(k, a, h):
    """📍️ World plan point (x, z) of segment k at apothem a and tangential offset h."""
    n = unit(SEG_PHI[k])
    tt = (-n[1], n[0])
    return (a * n[0] + h * tt[0], a * n[1] + h * tt[1])


def half_len(a, gap):
    """📐️ Tangential half length of a slat edge at apothem a, ends set back by gap from the corner lines."""
    return a * TAN30 - gap / COS30


def slat_bands(merged):
    """🪵️ The radial bands (a0, a1, kind) of the seat boards of one segment: five slats or, merged, one board."""
    bands = []
    for j in range(5):
        a1 = A_OUT - j * (SLAT_W + SLAT_GAP)
        bands.append((a1 - SLAT_W, a1, j))
    if not merged:
        return bands
    return bands


def corner_line(psi, r, d, side):
    """📏️ World plan point at radius r along the corner line psi, offset d towards the segment on `side` (+1 the segment at psi-30, -1 the one at psi+30)."""
    c = unit(psi)
    p = (c[1], -c[0]) if side > 0 else (-c[1], c[0])
    return (r * c[0] + d * p[0], r * c[1] + d * p[1])


def rect_on_line(psi, r0, r1, d0, d1):
    """⬛️ The plan rectangle along corner line psi from r0 to r1 and lateral offsets d0..d1 (left of the line seen outward is +)."""
    c = unit(psi)
    p = (-c[1], c[0])
    pts = [(r0, d0), (r1, d0), (r1, d1), (r0, d1)]
    return [(r * c[0] + d * p[0], r * c[1] + d * p[1]) for r, d in pts]


def square_at(cx, cz, half, psi):
    """🔲️ A square of half side `half` centred at (cx, cz), turned to psi."""
    c = unit(psi)
    p = (-c[1], c[0])
    return [(cx + su * half * c[0] + sv * half * p[0], cz + su * half * c[1] + sv * half * p[1]) for su, sv in [(-1, -1), (1, -1), (1, 1), (-1, 1)]]


def ngon(cx, cz, radius, n, rot=0.0):
    """⬡️ A regular n-gon in plan."""
    return [(cx + radius * math.cos(math.radians(rot + 360.0 * i / n)), cz + radius * math.sin(math.radians(rot + 360.0 * i / n))) for i in range(n)]


def area2(poly):
    """➗️ Signed plan area of a polygon (x, z)."""
    s = 0.0
    for i in range(len(poly)):
        x0, z0 = poly[i]
        x1, z1 = poly[(i + 1) % len(poly)]
        s += x0 * z1 - x1 * z0
    return s / 2.0


def perimeter(poly):
    """〰️ Perimeter of a polygon."""
    return sum(math.dist(poly[i], poly[(i + 1) % len(poly)]) for i in range(len(poly)))


def ccw(poly):
    """🔄️ The polygon counter-clockwise in (x, z)."""
    return poly if area2(poly) > 0 else list(reversed(poly))


def centroid(poly):
    """🎯️ Area centroid of a polygon."""
    a = area2(poly)
    cx = cz = 0.0
    for i in range(len(poly)):
        x0, z0 = poly[i]
        x1, z1 = poly[(i + 1) % len(poly)]
        f = x0 * z1 - x1 * z0
        cx += (x0 + x1) * f
        cz += (z0 + z1) * f
    return (cx / (6 * a), cz / (6 * a))


class Bone:
    """🦴️ A rig bone with its world rest frame, computed like the product does."""

    def __init__(self, bid, parent, title, x, y, z, angle, abduction, length, joint, mass=None):
        self.id, self.parent, self.title = bid, parent, title
        self.x, self.y, self.z, self.angle, self.abduction, self.length = x, y, z, angle, abduction, length
        self.joint, self.mass = joint, mass
        if parent is None:
            po, pu, pv, pw = (0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)
        else:
            po, pu, pv, pw = parent.o, parent.u, parent.v, parent.w
        self.o = tuple(po[i] + x * pu[i] + y * pv[i] + z * pw[i] for i in range(3))
        ca, sa = math.cos(math.radians(angle)), math.sin(math.radians(angle))
        u1 = tuple(ca * pu[i] + sa * pv[i] for i in range(3))
        v1 = tuple(-sa * pu[i] + ca * pv[i] for i in range(3))
        cb, sb = math.cos(math.radians(abduction)), math.sin(math.radians(abduction))
        self.u = tuple(cb * u1[i] + sb * pw[i] for i in range(3))
        self.w = tuple(-sb * u1[i] + cb * pw[i] for i in range(3))
        self.v = v1

    def local(self, p):
        """🔁️ A world point in this bone's (u, v, w)."""
        d = [p[i] - self.o[i] for i in range(3)]
        return tuple(sum(d[i] * axis[i] for i in range(3)) for axis in (self.u, self.v, self.w))

    def json(self):
        """🧾️ The schema form of the bone."""
        out = {"id": self.id, "parent": self.parent and self.parent.id, "title": self.title, "x": r5(self.x), "y": r5(self.y)}
        if self.z:
            out["z"] = r5(self.z)
        out["angleDeg"] = self.angle
        if self.abduction:
            out["abductionDeg"] = self.abduction
        out["lengthM"] = r5(self.length)
        out["joint"] = self.joint
        if self.mass is not None:
            out["mass"] = self.mass
        return out


BONES = {}


def bone(*args, **kw):
    """➕️ Adds a bone."""
    b = Bone(*args, **kw)
    BONES[b.id] = b
    return b


def leg_key(k, side):
    """🔑️ The id suffix of a leg."""
    return f"{k}-{side}"


def lifted(k, side):
    """🌱️ Whether the root has lifted this foot."""
    return (str(k), side) == LIFTED


def pad_top(k, side):
    """⬆️ The pad top height of a leg (the root lifted one)."""
    return LIFT if lifted(k, side) else 0.0


def leg_radius(side):
    """↔️ Radius of a leg along its corner line."""
    return LEG_R[0] if side == "i" else LEG_R[1]


def leg_centre(k, side):
    """📍️ World plan centre of a leg."""
    c = unit(BEARER_PSI[k])
    r = leg_radius(side)
    return (r * c[0], r * c[1])


SEG_CENTROID = {}


def build_rig():
    """🦴️ Builds the rig: ground, six frames with two legs and two footing pads each, six seat segments and the lifting root."""
    ground = bone("ground", None, t("Ground and footing reference", "Boden- und Fundamentbezug"), 0.0, 0.0, 0.0, 0, 0, 0.0, {"type": "fixed"})
    for k in SEGMENTS:
        psi = BEARER_PSI[k]
        c = unit(psi)
        blen = BEARER_R[1] - BEARER_R[0]
        bmass = blen * (BEARER_W * BEARER_H - (BEARER_W - 0.008) * (BEARER_H - 0.008)) * STEEL
        b = bone(f"bearer-{k}", ground, t(f"Radial frame {k + 1} (bearer)", f"Radialrahmen {k + 1} (Träger)"), BEARER_R[0] * c[0], (BEARER_TOP + BEARER_BOT) / 2, BEARER_R[0] * c[1], 0, psi, blen, {"type": "fixed"},
                 mass={"kg": round(bmass, 3), "comX": r5(blen / 2), "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-10219-2"})
        for side in ("i", "o"):
            length = BEARER_BOT - pad_top(k, side)
            shs = (BEARER_BOT - LEG_BOTTOM) * (LEG_S ** 2 - (LEG_S - 2 * LEG_WALL) ** 2) * STEEL
            plate = END_PLATE[0] ** 2 * END_PLATE[1] * STEEL
            nut = 3 * SQ3 / 2 * (NUT_AF / SQ3) ** 2 * NUT_H * STEEL - math.pi * (THREAD_D / 2) ** 2 * NUT_H * STEEL
            thread_len = NUT_BOT + NUT_H - (pad_top(k, side) + FOOT[1])
            thread = math.pi * (THREAD_D / 2) ** 2 * thread_len * STEEL
            foot = FOOT[0] ** 2 * FOOT[1] * STEEL
            parts = [(shs, (BEARER_BOT - LEG_BOTTOM) / 2), (plate, BEARER_BOT - LEG_BOTTOM + END_PLATE[1] / 2), (nut, BEARER_BOT - NUT_BOT - NUT_H / 2), (thread, BEARER_BOT - NUT_BOT - NUT_H + thread_len / 2), (foot, length - FOOT[1] / 2)]
            mkg = sum(m for m, _ in parts)
            com = sum(m * u for m, u in parts) / mkg
            leg = bone(f"leg-{leg_key(k, side)}", b, t(f"Frame {k + 1} {'inner' if side == 'i' else 'outer'} leg with levelling foot", f"Rahmen {k + 1} {'inneres' if side == 'i' else 'äußeres'} Bein mit Stellfuß"),
                       leg_radius(side) - BEARER_R[0], -BEARER_H / 2, 0.0, -90, 0, length, {"type": "fixed"},
                       mass={"kg": round(mkg, 3), "comX": r5(com), "comY": 0, "distribution": "point", "source": "literature", "reference": "en-10219-2"})
            pmass = PAD[0] ** 2 * PAD[1] * CONCRETE
            bone(f"pad-{leg_key(k, side)}", leg, t(f"Footing slab under frame {k + 1} {'inner' if side == 'i' else 'outer'} leg", f"Fundamentplatte unter Rahmen {k + 1} {'innerem' if side == 'i' else 'äußerem'} Bein"),
                 length, 0.0, 0.0, 0, 0, PAD[1], {"type": "fixed"},
                 mass={"kg": round(pmass, 3), "comX": r5(PAD[1] / 2), "comY": 0, "distribution": "point", "source": "literature", "reference": "din-en-1339"})
    for k in SEGMENTS:
        polys = [slat_poly(k, a0, a1, j, split=True) for a0, a1, j in slat_bands(False)]
        masses = [abs(area2(p)) * SLAT_T * ROBINIA for p in polys]
        carriers = [carrier_poly(k, s) for s in (+1, -1)]
        cmass = [abs(area2(p)) * CARRIER_T * STEEL for p in carriers]
        cents = [centroid(p) for p in polys + carriers]
        ms = masses + cmass
        mx = sum(m * c[0] for m, c in zip(ms, cents)) / sum(ms)
        mz = sum(m * c[1] for m, c in zip(ms, cents)) / sum(ms)
        my = (sum(masses) * (SLAT_BOT + SLAT_T / 2) + sum(cmass) * (SLAT_BOT - CARRIER_T / 2)) / sum(ms)
        SEG_CENTROID[k] = (mx, my, mz)
        joint = {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.4} if k == 0 else {"type": "fixed"}
        bone(f"seg-{k}", ground, t(f"Seat segment {k + 1} (five slats on two carriers)", f"Sitzsegment {k + 1} (fünf Latten auf zwei Tragschienen)"), mx, SLAT_BOT, mz, 0, 0, 0.0, joint,
             mass={"kg": round(sum(ms), 3), "comX": 0, "comY": r5(my - SLAT_BOT), "distribution": "point", "source": "literature", "reference": "holzatlas-robinie"})
    bone("root", ground, t("Lime tree root lifting the inner foot of frame 1", "Lindenwurzel, die den inneren Fuß von Rahmen 1 anhebt"), ROOT_A[0][0], 0.0, ROOT_A[0][2], 0, 0, 0.0, {"type": "fixed"})


def slat_poly(k, a0, a1, j, split):
    """🪵️ World plan polygon of slat j of segment k (with its end split where the slat is split)."""
    h0, h1 = half_len(a0, END_GAP), half_len(a1, END_GAP)
    pts = [seg_point(k, a0, -h0), seg_point(k, a1, -h1), seg_point(k, a1, h1), seg_point(k, a0, h0)]
    sign = SPLITS.get((k, j)) if split else None
    if sign is None:
        return pts
    am = (a0 + a1) / 2
    s = 1 if sign == "+" else -1
    ea, eb = am + 0.005, am - 0.005
    tip = (half_len(am, END_GAP) - 0.30) * s
    if s > 0:
        return [pts[0], pts[1], pts[2], seg_point(k, ea, half_len(ea, END_GAP)), seg_point(k, am, tip), seg_point(k, eb, half_len(eb, END_GAP)), pts[3]]
    return [pts[0], seg_point(k, eb, -half_len(eb, END_GAP)), seg_point(k, am, tip), seg_point(k, ea, -half_len(ea, END_GAP)), pts[1], pts[2], pts[3]]


def carrier_poly(k, side):
    """🔩️ World plan polygon of the steel carrier of segment k on its bearer: side +1 on the bearer at phi+30, -1 at phi-30."""
    psi = SEG_PHI[k] + 30 * side
    lo, hi = (-CARRIER_D[1], -CARRIER_D[0]) if side > 0 else (CARRIER_D[0], CARRIER_D[1])
    return rect_on_line(psi, BEARER_R[0], BEARER_R[1], lo, hi)


ROOT_A = ((0.33 * COS30, -0.015, 0.33 * 0.5, 0.075), (0.59 * COS30, -0.115, 0.59 * 0.5, 0.065))
ROOT_B = ((0.59 * COS30, -0.115, 0.59 * 0.5, 0.065), (0.86 * COS30, -0.105, 0.86 * 0.5, 0.055))


class Part:
    """🧱️ One physical part of one scale: a vertical prism (plan polygon, y0, y1) or a capsule, its 3D bone and its 2D binding."""

    def __init__(self, pid, bone_id, poly, y0, y1, region, material, tone, role="body", draw_bone=None, member=None, capsule=None, hatch=None, plan_hatch=None, flags=None):
        self.id, self.bone, self.poly, self.y0, self.y1 = pid, bone_id, poly, y0, y1
        self.region, self.material, self.tone, self.role = region, material, tone, role
        self.draw_bone = draw_bone or bone_id
        self.member, self.capsule, self.hatch, self.plan_hatch = member, capsule, hatch, plan_hatch
        self.flags = flags or {}

    def solid(self):
        """🧊️ The body solid of the part in its bone's rest frame."""
        b = BONES[self.bone]
        base = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        base.update(self.flags)
        if self.capsule is not None:
            p0, p1 = self.capsule
            base.update({"kind": "capsule", "p0": [r5(v) for v in b.local(p0[:3])], "p1": [r5(v) for v in b.local(p1[:3])], "r0": p0[3], "r1": p1[3]})
            return base
        if abs(b.v[1] - 1.0) < 1e-9:
            prof = [b.local((x, b.o[1], z)) for x, z in ccw(self.poly)]
            base.update({"kind": "extrude", "plane": "zx", "profile": [[r5(p[2]), r5(p[0])] for p in prof], "from": r5(self.y0 - b.o[1]), "to": r5(self.y1 - b.o[1])})
            return base
        prof = [b.local((x, b.o[1], z)) for x, z in ccw(self.poly)]
        base.update({"kind": "extrude", "plane": "yz", "profile": [[r5(p[1]), r5(p[2])] for p in prof], "from": r5(b.o[1] - self.y1), "to": r5(b.o[1] - self.y0)})
        return base

    def xs(self):
        """↔️ World x range."""
        if self.capsule is not None:
            return (min(p[0] - p[3] for p in self.capsule), max(p[0] + p[3] for p in self.capsule))
        return (min(p[0] for p in self.poly), max(p[0] for p in self.poly))

    def zs(self):
        """↕️ World z range."""
        if self.capsule is not None:
            return (min(p[2] - p[3] for p in self.capsule), max(p[2] + p[3] for p in self.capsule))
        return (min(p[1] for p in self.poly), max(p[1] for p in self.poly))

    def ys(self):
        """⬆️ World y range."""
        if self.capsule is not None:
            return (min(p[1] - p[3] for p in self.capsule), max(p[1] + p[3] for p in self.capsule))
        return (self.y0, self.y1)

    def area(self):
        """📐️ Outer area of the prism or capsule alone."""
        if self.capsule is not None:
            (a, b) = self.capsule
            ln = math.dist(a[:3], b[:3])
            slant = math.sqrt(ln ** 2 + (a[3] - b[3]) ** 2)
            return math.pi * (a[3] + b[3]) * slant + 2 * math.pi * (a[3] ** 2 + b[3] ** 2)
        return 2 * abs(area2(self.poly)) + perimeter(self.poly) * (self.y1 - self.y0)

    def volume(self):
        """🧮️ Volume of the prism or capsule."""
        if self.capsule is not None:
            (a, b) = self.capsule
            ln = math.dist(a[:3], b[:3])
            return math.pi * ln * (a[3] ** 2 + a[3] * b[3] + b[3] ** 2) / 3 + 2 / 3 * math.pi * (a[3] ** 3 + b[3] ** 3)
        return abs(area2(self.poly)) * (self.y1 - self.y0)


def common(part, scale, view):
    """🖊️ The members every drawn shape of a part carries."""
    out = {"id": part.id, "bone": part.draw_bone, "role": part.role, "z": 0, "filled": True, "tone": part.tone, "region": part.region, "material": part.material}
    pen = {"50": 0.25, "100": 0.18, "200": 0.18, "500": 0.13}[scale]
    out["penMm"] = pen
    hatch = part.plan_hatch if view == "plan" else part.hatch
    if hatch:
        out["hatch"] = hatch
    return out


def depth_order(part, view):
    """🔢️ Paint order: nearer to the viewer later."""
    if view == "elevation":
        return round(10 - part.zs()[0] * 3, 3)
    if view == "front":
        return round(10 + part.xs()[1] * 3, 3)
    return round(10 + part.ys()[1] * 10, 3)


def shapes_of(part, scale):
    """✏️ The elevation, front and plan shapes of one part, drawn to the numbers of its solid."""
    out = {}
    if part.member is not None:
        frm, to, sec = part.member
        for view in ("elevation", "front", "plan"):
            s = {k: v for k, v in common(part, scale, view).items() if k not in ("hatch",)}
            s.update({"kind": "member", "from": frm, "to": to, "section": sec, "z": depth_order(part, view)})
            out[view] = [s]
        return out
    b = BONES[part.draw_bone]
    if part.capsule is not None:
        p0, p1 = part.capsule
        e = common(part, scale, "elevation")
        e.update({"kind": "capsule", "x0": r5(p0[0] - b.o[0]), "y0": r5(p0[1] - b.o[1]), "x1": r5(p1[0] - b.o[0]), "y1": r5(p1[1] - b.o[1]), "r0": p0[3], "r1": p1[3], "z": depth_order(part, "elevation")})
        f = common(part, scale, "front")
        f.update({"kind": "capsule", "x0": r5(p0[2]), "y0": r5(p0[1]), "x1": r5(p1[2]), "y1": r5(p1[1]), "r0": p0[3], "r1": p1[3], "z": depth_order(part, "front")})
        p = common(part, scale, "plan")
        p.update({"kind": "capsule", "x0": r5(p0[0]), "y0": r5(p0[2]), "x1": r5(p1[0]), "y1": r5(p1[2]), "r0": p0[3], "r1": p1[3], "z": depth_order(part, "plan")})
        return {"elevation": [e], "front": [f], "plan": [p]}
    x0, x1 = part.xs()
    z0, z1 = part.zs()
    e = common(part, scale, "elevation")
    e.update({"kind": "rect", "x": r5(x0 - b.o[0]), "y": r5(part.y0 - b.o[1]), "w": r5(x1 - x0), "h": r5(part.y1 - part.y0), "z": depth_order(part, "elevation")})
    f = common(part, scale, "front")
    f.update({"kind": "rect", "x": r5(z0), "y": r5(part.y0), "w": r5(z1 - z0), "h": r5(part.y1 - part.y0), "z": depth_order(part, "front")})
    p = common(part, scale, "plan")
    p.update({"kind": "poly", "points": [[r5(x), r5(z)] for x, z in ccw(part.poly)], "closed": True, "z": depth_order(part, "plan")})
    return {"elevation": [e], "front": [f], "plan": [p]}


def seat_parts(scale):
    """🪑️ The seat boards and carriers of all six segments at one scale."""
    parts = []
    if scale == "50":
        for k in SEGMENTS:
            for a0, a1, j in slat_bands(False):
                replaced = (k, j) == REPLACED
                region = "replaced-slat" if replaced else "seat-slats"
                material = "robinia-fresh" if replaced else "robinia-weathered"
                tone = 0.78 if replaced else 0.56 - 0.02 * j
                hatch = "algae" if k in ALGAE_SEGMENTS and not replaced else f"grain-{int((SEG_PHI[k] + 90) % 180)}"
                parts.append(Part(f"s{k}-slat-{j + 1}", f"seg-{k}", slat_poly(k, a0, a1, j, True), SLAT_BOT, SEAT_TOP, region, material, tone, plan_hatch=hatch))
            for side, tag in ((+1, "a"), (-1, "b")):
                parts.append(Part(f"s{k}-carrier-{tag}", f"seg-{k}", carrier_poly(k, side), BEARER_TOP, SLAT_BOT, "frame-coating", "steel-powder-coated", 0.3))
        return parts
    if scale == "100":
        for k in SEGMENTS:
            for a0, a1, j in slat_bands(False):
                replaced = (k, j) == REPLACED
                hatch = "algae" if k in ALGAE_SEGMENTS and not replaced else f"grain-{int((SEG_PHI[k] + 90) % 180)}"
                parts.append(Part(f"s{k}-slat-{j + 1}", f"seg-{k}", slat_poly(k, a0, a1, j, False), SLAT_BOT, SEAT_TOP, "replaced-slat" if replaced else "seat-slats", "robinia-fresh" if replaced else "robinia-weathered", 0.78 if replaced else 0.54, plan_hatch=hatch))
        return parts
    gap = {"200": 0.0, "500": 0.0}[scale]
    bottom = {"100": SLAT_BOT, "200": SLAT_BOT, "500": SEAT_TOP - 0.075}[scale]
    for k in SEGMENTS:
        cuts = [A_IN, A_OUT]
        if k == REPLACED[0]:
            j = REPLACED[1]
            a1 = A_OUT - j * (SLAT_W + SLAT_GAP) + SLAT_GAP / 2
            a0 = a1 - SLAT_W - SLAT_GAP
            cuts = [A_IN, a0, a1, A_OUT]
        for i in range(len(cuts) - 1):
            lo, hi = cuts[i], cuts[i + 1]
            replaced = len(cuts) == 4 and i == 1
            pts = [seg_point(k, lo, -half_len(lo, gap)), seg_point(k, hi, -half_len(hi, gap)), seg_point(k, hi, half_len(hi, gap)), seg_point(k, lo, half_len(lo, gap))]
            pid = f"s{k}-replaced-slat" if replaced else (f"s{k}-seat" if len(cuts) == 2 else f"s{k}-seat-{'inner' if i == 0 else 'outer'}")
            hatch = None
            if scale == "100":
                hatch = "algae" if k in ALGAE_SEGMENTS and not replaced else f"grain-{int((SEG_PHI[k] + 90) % 180)}"
            parts.append(Part(pid, f"seg-{k}", pts, bottom, SEAT_TOP, "replaced-slat" if replaced else "seat-slats", "robinia-fresh" if replaced else "robinia-weathered", 0.78 if replaced else 0.54, plan_hatch=hatch))
    return parts


def frame_parts(scale):
    """🔩️ The six radial frames: bearer, legs, levelling feet and footing slabs at one scale."""
    parts = []
    btop = {"50": BEARER_TOP, "100": SLAT_BOT, "200": SLAT_BOT, "500": SEAT_TOP - 0.075}[scale]
    bbot = {"50": BEARER_BOT, "100": BEARER_BOT, "200": BEARER_BOT, "500": SEAT_TOP - 0.15}[scale]
    bw = {"50": BEARER_W, "100": BEARER_W, "200": BEARER_W, "500": BEARER_W}[scale]
    legs = {"50": LEG_S, "100": LEG_S, "200": LEG_S, "500": 0.075}[scale]
    for k in SEGMENTS:
        psi = BEARER_PSI[k]
        blen = BEARER_R[1] - BEARER_R[0]
        b = BONES[f"bearer-{k}"]
        poly = rect_on_line(psi, BEARER_R[0], BEARER_R[1], -bw / 2, bw / 2)
        mid = (btop + bbot) / 2 - b.o[1]
        parts.append(Part(f"b{k}-bearer", f"bearer-{k}", poly, bbot, btop, "frame-coating", "steel-powder-coated", 0.34,
                          member=({"x": 0, "y": r5(mid), "z": 0}, {"x": r5(blen), "y": r5(mid), "z": 0}, {"kind": "rect", "w": r5(btop - bbot), "h": r5(bw)})))
        for side in ("i", "o"):
            cx, cz = leg_centre(k, side)
            top = pad_top(k, side)
            key = leg_key(k, side)
            leg_bone = BONES[f"leg-{key}"]
            if scale == "50":
                lbot = LEG_BOTTOM
            elif scale == "100":
                lbot = LEG_BOTTOM
            elif scale == "200":
                lbot = top + 0.030
            else:
                lbot = top + 0.075
            ltop = bbot
            parts.append(Part(f"b{k}-leg-{side}", f"leg-{key}", square_at(cx, cz, legs / 2, psi), lbot, ltop, "frame-coating", "steel-powder-coated", 0.4,
                              member=({"x": r5(leg_bone.o[1] - ltop), "y": 0, "z": 0}, {"x": r5(leg_bone.o[1] - lbot), "y": 0, "z": 0}, {"kind": "rect", "w": r5(legs), "h": r5(legs)})))
            if scale == "50":
                parts.append(Part(f"b{k}-plate-{side}", "ground", square_at(cx, cz, END_PLATE[0] / 2, psi), LEG_BOTTOM - END_PLATE[1], LEG_BOTTOM, "feet-rust", "steel-rusted", 0.28, hatch="rust", plan_hatch="rust"))
                parts.append(Part(f"b{k}-nut-{side}", "ground", ngon(cx, cz, NUT_AF / SQ3, 6, psi), NUT_BOT, NUT_BOT + NUT_H, "feet-rust", "steel-rusted", 0.24))
                parts.append(Part(f"b{k}-thread-{side}", "ground", ngon(cx, cz, THREAD_D / 2, 16), top + FOOT[1], NUT_BOT, "feet-rust", "steel-rusted", 0.22))
                parts.append(Part(f"b{k}-foot-{side}", "ground", square_at(cx, cz, FOOT[0] / 2, psi), top, top + FOOT[1], "feet-rust", "steel-rusted", 0.26, hatch="rust", plan_hatch="rust"))
            elif scale == "100":
                parts.append(Part(f"b{k}-plate-{side}", "ground", square_at(cx, cz, END_PLATE[0] / 2, psi), LEG_BOTTOM - 0.015, LEG_BOTTOM, "feet-rust", "steel-rusted", 0.28, hatch="rust", plan_hatch="rust"))
                parts.append(Part(f"b{k}-thread-{side}", "ground", ngon(cx, cz, THREAD_D / 2, 12), top + 0.015, LEG_BOTTOM - 0.015, "feet-rust", "steel-rusted", 0.22))
                parts.append(Part(f"b{k}-foot-{side}", "ground", square_at(cx, cz, FOOT[0] / 2, psi), top, top + 0.015, "feet-rust", "steel-rusted", 0.26, hatch="rust", plan_hatch="rust"))
            else:
                hgt = 0.030 if scale == "200" else 0.075
                parts.append(Part(f"b{k}-foot-{side}", "ground", square_at(cx, cz, (FOOT[0] if scale == "200" else 0.075) / 2, psi), top, top + hgt, "feet-rust", "steel-rusted", 0.26))
            if scale in ("200", "500") and k != int(LIFTED[0]):
                if side == "o":
                    c = unit(psi)
                    parts.append(Part(f"b{k}-pads", "ground", rect_on_line(psi, LEG_R[0] - PAD[0] / 2, LEG_R[1] + PAD[0] / 2, -PAD[0] / 2, PAD[0] / 2), -PAD[1], 0.0, "footing-pads", "concrete-weathered", 0.7))
                continue
            parts.append(Part(f"b{k}-pad-{side}", "ground", square_at(cx, cz, PAD[0] / 2, psi), top - PAD[1], top, "footing-pads", "concrete-weathered", 0.7, hatch="concrete" if scale in ("50", "100") else None, plan_hatch="concrete" if scale in ("50", "100") else None))
    return parts


def root_parts(scale):
    """🌳️ The lime root that lifted the inner foot of frame 1: a flare rising out of the gravel and a run under the slab."""
    if scale == "500":
        return [Part("root", "root", None, 0, 0, "root-bark", "bark", 0.36, capsule=(ROOT_A[0], (0.80 * COS30, -0.115, 0.80 * 0.5, 0.065)))]
    return [Part("root-flare", "root", None, 0, 0, "root-bark", "bark", 0.36, capsule=ROOT_A), Part("root-run", "root", None, 0, 0, "root-bark", "bark", 0.34, capsule=ROOT_B)]


def parts_of(scale):
    """📦️ Every part of one scale."""
    return seat_parts(scale) + frame_parts(scale) + root_parts(scale)


def details_of(scale):
    """🔍️ Detail and texture strokes of the traces of use: bolt heads, splits, graffiti, sticker residue, mower scuffs."""
    out = {"elevation": [], "front": [], "plan": []}
    if scale not in ("50", "100"):
        return out
    pen_d = {"50": 0.18, "100": 0.13}[scale]
    if scale == "50":
        for k in SEGMENTS:
            for a0, a1, j in slat_bands(False):
                am = (a0 + a1) / 2
                for s in (-1, 1):
                    h = half_len(am, END_GAP) - 0.016
                    x, z = seg_point(k, am, s * h)
                    out["plan"].append({"id": f"s{k}-bolt-{j + 1}-{'a' if s > 0 else 'b'}", "bone": f"seg-{k}", "role": "detail", "z": 30, "filled": False, "tone": 0.2, "kind": "ellipse", "cx": r5(x), "cy": r5(z), "rx": 0.0095, "ry": 0.0095, "penMm": pen_d, "region": "frame-coating", "material": "steel-powder-coated"})
        for (k, j), sign in SPLITS.items():
            a1 = A_OUT - j * (SLAT_W + SLAT_GAP)
            am = a1 - SLAT_W / 2
            s = 1 if sign == "+" else -1
            pts = [seg_point(k, am + 0.02, s * (half_len(am, END_GAP) - 0.32)), seg_point(k, am + 0.012, s * (half_len(am, END_GAP) - 0.45)), seg_point(k, am + 0.018, s * (half_len(am, END_GAP) - 0.55))]
            out["plan"].append({"id": f"s{k}-check-{j + 1}", "bone": f"seg-{k}", "role": "texture", "z": 31, "filled": False, "tone": 0.15, "kind": "poly", "points": [[r5(x), r5(z)] for x, z in pts], "closed": False, "penMm": 0.13, "region": "seat-slats", "material": "robinia-weathered"})
    k = 5
    a1 = A_OUT
    tag = [(0.0, 0.0), (0.06, 0.03), (0.1, -0.02), (0.16, 0.035), (0.21, -0.01), (0.27, 0.02)]
    pts = [seg_point(k, a1 - 0.03 - dz * 0.6, -0.15 + dx) for dx, dz in tag]
    out["plan"].append({"id": "s5-graffiti-tag", "bone": "seg-5", "role": "texture", "z": 32, "filled": False, "tone": 0.1, "kind": "poly", "points": [[r5(x), r5(z)] for x, z in pts], "closed": False, "penMm": 0.13, "region": "seat-slats", "material": "robinia-weathered"})
    st = [seg_point(0, A_OUT - 0.012, 0.18), seg_point(0, A_OUT - 0.07, 0.18), seg_point(0, A_OUT - 0.07, 0.24), seg_point(0, A_OUT - 0.012, 0.24)]
    out["plan"].append({"id": "s0-sticker-residue", "bone": "seg-0", "role": "texture", "z": 32, "filled": False, "tone": 0.1, "kind": "poly", "points": [[r5(x), r5(z)] for x, z in st], "closed": True, "penMm": 0.13, "region": "seat-slats", "material": "robinia-weathered"})
    for k in (4, 5):
        cx, cz = leg_centre(k, "o")
        b = BONES["ground"]
        y0, y1 = 0.14, 0.20
        out["elevation"].append({"id": f"b{k}-mower-scuff", "bone": "ground", "role": "texture", "z": 40, "filled": False, "tone": 0.15, "kind": "poly", "points": [[r5(cx - 0.02), y0], [r5(cx + 0.005), r5((y0 + y1) / 2)], [r5(cx + 0.02), y1]], "closed": False, "penMm": 0.13, "region": "frame-coating", "material": "steel-powder-coated"})
    return out


def representation(scale):
    """🗺️ The authored drawing of one scale."""
    pens = {"50": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.18, "ground": 0.5},
            "100": {"body": 0.25, "detail": 0.13, "texture": 0.13, "glazing": 0.13, "ground": 0.35},
            "200": {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.25},
            "500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}[scale]
    ranges = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}[scale]
    hatches = []
    if scale in ("50", "100"):
        sp = {"50": 0.6, "100": 0.9}[scale]
        for ang in (30, 90, 150):
            hatches.append({"id": f"grain-{ang}", "title": t(f"Greyed robinia grain at {ang} degrees", f"Vergraute Robinienmaserung unter {ang} Grad"), "pattern": "lines", "angleDeg": ang, "spacingMm": sp, "penMm": 0.13})
        hatches.append({"id": "algae", "title": t("Green algae film on the shaded side", "Grüner Algenbelag auf der Schattenseite"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.5 if scale == "50" else 0.7, "penMm": 0.13})
        hatches.append({"id": "rust", "title": t("Rust and flaking coating", "Rost und abblätternde Beschichtung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.4 if scale == "50" else 0.6, "penMm": 0.13})
        hatches.append({"id": "concrete", "title": t("Weathered concrete slab", "Verwitterte Betonplatte"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8 if scale == "50" else 1.0, "penMm": 0.13})
    geometry = {"elevation": [], "front": [], "plan": []}
    for part in parts_of(scale):
        for view, shapes in shapes_of(part, scale).items():
            geometry[view].extend(shapes)
    for view, shapes in details_of(scale).items():
        geometry[view].extend(shapes)
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": ranges[0], "maxDenominator": ranges[1]}, "pens": pens, "hatches": hatches, "geometry": geometry, "decisions": REP_DECISIONS[scale]}


def thermal_list(scale, body):
    """🌡️ Thermal solids: every body solid is its own region surface."""
    regions = {p.id: p.region for p in parts_of(scale)}
    return [{"id": f"th-{s['id']}", "of": s["id"], "region": regions[s["id"]]} for s in body]


def box_on(bone_id, u0, u1, half_v, half_w, sid, material, part, member, tone=0.3, role="detail", anchor=None):
    """📦️ A structural box along u of a bone (from u0 to u1, centred in v and w)."""
    s = {"id": sid, "kind": "box", "bone": bone_id, "material": material, "role": role, "tone": tone, "size": [r5(u1 - u0), r5(2 * half_v), r5(2 * half_w)], "transform": {"translation": [r5((u0 + u1) / 2), 0, 0]}, "member": member, "part": part}
    if anchor:
        s["anchor"] = anchor
    return s


def structure_list(scale, body):
    """🏗️ The structural idealisation of one scale: seat members, bearers, legs with their levelling feet, footing slabs, joints, supports and masses."""
    f = FEATURE[scale]
    ids = {s["id"] for s in body}
    out = []
    for s in body:
        if s["bone"].startswith("seg-") or s["bone"].startswith("bearer-") or s["bone"].startswith("leg-"):
            out.append({"id": f"m-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"})
    for k in SEGMENTS:
        for side in ("i", "o"):
            key = leg_key(k, side)
            leg = BONES[f"leg-{key}"]
            top = pad_top(k, side)
            L = leg.length
            u_leg_bot = leg.o[1] - {"50": LEG_BOTTOM, "100": LEG_BOTTOM, "200": top + 0.030, "500": top + 0.075}[scale]
            if scale == "50":
                out.append(box_on(leg.id, u_leg_bot, u_leg_bot + END_PLATE[1], END_PLATE[0] / 2, END_PLATE[0] / 2, f"m-b{k}-plate-{side}", "steel-rusted", "member", leg.id))
                out.append(box_on(leg.id, u_leg_bot + END_PLATE[1], L - FOOT[1], THREAD_D / 2, THREAD_D / 2, f"m-b{k}-thread-{side}", "steel-rusted", "member", leg.id))
                out.append(box_on(leg.id, L - FOOT[1], L, FOOT[0] / 2, FOOT[0] / 2, f"m-b{k}-foot-{side}", "steel-rusted", "member", leg.id))
            elif scale == "100":
                out.append(box_on(leg.id, u_leg_bot, u_leg_bot + 0.015, END_PLATE[0] / 2, END_PLATE[0] / 2, f"m-b{k}-plate-{side}", "steel-rusted", "member", leg.id))
                out.append(box_on(leg.id, u_leg_bot + 0.015, L - 0.015, THREAD_D / 2, THREAD_D / 2, f"m-b{k}-thread-{side}", "steel-rusted", "member", leg.id))
                out.append(box_on(leg.id, L - 0.015, L, FOOT[0] / 2, FOOT[0] / 2, f"m-b{k}-foot-{side}", "steel-rusted", "member", leg.id))
            else:
                fw = FOOT[0] if scale == "200" else 0.075
                out.append(box_on(leg.id, u_leg_bot, L, fw / 2, fw / 2, f"m-b{k}-foot-{side}", "steel-rusted", "member", leg.id))
            pad = BONES[f"pad-{key}"]
            out.append(box_on(pad.id, 0, PAD[1], PAD[0] / 2, PAD[0] / 2, f"m-b{k}-pad-{side}", "concrete-weathered", "member", pad.id, tone=0.6))
            out.append(box_on(pad.id, PAD[1] - min(f, PAD[1]), PAD[1], PAD[0] / 2, PAD[0] / 2, f"support-foot-{key}", "concrete-weathered", "support", pad.id, tone=0.2, role="ground", anchor=f"foot-{key}"))
            if scale in ("50", "100", "200"):
                jt = max(f, 0.008)
                out.append(box_on(leg.id, -jt / 2, jt / 2, LEG_S / 2 + 0.002, LEG_S / 2 + 0.002, f"weld-b{k}-{side}", "steel-powder-coated", "joint", leg.id, tone=0.5))
    if scale in ("50", "100"):
        d = max(0.010, FEATURE[scale])
        for k in SEGMENTS:
            seg = BONES[f"seg-{k}"]
            for side, tag in ((+1, "a"), (-1, "b")):
                psi = SEG_PHI[k] + 30 * side
                for r in (0.80, 1.07):
                    off = -sum(CARRIER_D) / 2 if side > 0 else sum(CARRIER_D) / 2
                    x, z = corner_line(psi, r, 0, 1)
                    c = unit(psi)
                    pp = (-c[1], c[0])
                    x, z = r * c[0] + off * pp[0], r * c[1] + off * pp[1]
                    loc = seg.local((x, 0, z))
                    out.append({"id": f"bolt-s{k}-{tag}-{int(r * 100)}", "kind": "extrude", "bone": seg.id, "material": "steel-powder-coated", "role": "detail", "tone": 0.25, "plane": "zx",
                                "profile": [[r5(loc[2] + d / 2 * math.sin(math.radians(a))), r5(loc[0] + d / 2 * math.cos(math.radians(a)))] for a in range(0, 360, 45)],
                                "from": r5(BEARER_BOT - seg.o[1]), "to": r5(SLAT_BOT + 0.02 - seg.o[1]), "member": seg.id, "part": "joint"})
    mr = {"50": 0.02, "100": 0.02, "200": 0.03, "500": 0.055}[scale]
    for b in BONES.values():
        if b.mass is None:
            continue
        m = b.mass
        out.append({"id": f"mass-{b.id}", "kind": "ellipsoid", "bone": b.id, "material": "steel-powder-coated" if not b.id.startswith(("seg", "pad")) else ("robinia-weathered" if b.id.startswith("seg") else "concrete-weathered"), "role": "detail", "tone": 0.3,
                    "radii": [mr, mr, mr], "transform": {"translation": [m["comX"], m["comY"], 0]}, "member": b.id, "part": "mass"})
    return out


def volume(scale):
    """🧊️ The handcrafted 3D model of one scale."""
    body = [p.solid() for p in parts_of(scale)]
    return {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal_list(scale, body), "structure": structure_list(scale, body), "decisions": VOL_DECISIONS[scale]}


REP_DECISIONS = {
    "50": {
        "silhouette": t("Hexagonal tree surround bench 2.08 m across the flats and 2.4 m across the seat corners, drawn part by part: the 30 greyed robinia slats 82 x 40 mm with their 10 mm drainage gaps and mitred ends 10 mm apart at the six radial joints, the steel carrier under each segment end, the six radial frames of 80 x 40 mm hollow section on twelve 60 mm square legs, the levelling feet with end plate, nut, M16 thread and foot plate, the 300 x 300 x 80 mm concrete slabs flush with the gravel and the lime root that has lifted the inner slab of frame 1 by 30 mm.", "Sechseckige Baumrundbank, 2,08 m über die Flächen und 2,4 m über die Sitzecken, Teil für Teil gezeichnet: die 30 vergrauten Robinienlatten 82 x 40 mm mit 10-mm-Wasserfugen und auf Gehrung geschnittenen Enden, 10 mm auseinander an den sechs Radialstößen, die Stahltragschiene unter jedem Segmentende, die sechs Radialrahmen aus Hohlprofil 80 x 40 mm auf zwölf 60-mm-Vierkantbeinen, die Stellfüße mit Kopfplatte, Mutter, M16-Gewinde und Fußplatte, die Betonplatten 300 x 300 x 80 mm bündig in der wassergebundenen Decke und die Lindenwurzel, die die innere Platte von Rahmen 1 um 30 mm angehoben hat."),
        "retained": t("Seat height 0.45 m, seat depth 0.45 m, every slat, gap and mitre, the split slat ends, the fresh replacement slat in segment 5, every leg, foot and slab, the raised slab and the root.", "Sitzhöhe 0,45 m, Sitztiefe 0,45 m, jede Latte, Fuge und Gehrung, die gespaltenen Lattenenden, die frische Ersatzlatte in Segment 5, jedes Bein, jeder Fuß und jede Platte, die angehobene Platte und die Wurzel."),
        "merged": t("Nothing; the 6 mm eased slat edges and the 3 mm walls of the hollow sections are drawn as their outer outline.", "Nichts; die 6-mm-Kantenrundung der Latten und die 3-mm-Wände der Hohlprofile sind als Außenumriss gezeichnet."),
        "omitted": t("Chewing gum under the seat, the welds and the end caps of the hollow sections, the carriage bolt shanks.", "Kaugummi unter der Sitzfläche, die Schweißnähte und Endkappen der Hohlprofile, die Schäfte der Schlossschrauben."),
        "thinElements": t("The 8 mm carriers, end plates and the 10 mm foot plates are drawn at their true thickness (0.16 and 0.2 mm on paper); the 10 mm splits are cut into the slat outlines.", "Die 8-mm-Tragschienen und Kopfplatten und die 10-mm-Fußplatten sind in wahrer Dicke gezeichnet (0,16 und 0,2 mm auf dem Papier); die 10-mm-Risse sind in die Lattenumrisse geschnitten."),
        "hatching": t("Plan: grain lines along every slat in its own direction, stipple for the algae film on the two shaded segments, stipple for the concrete, dots for rust on end plates and foot plates.", "Grundriss: Maserlinien längs jeder Latte in ihrer Richtung, Punktraster für den Algenbelag auf den zwei beschatteten Segmenten, Punktraster für den Beton, Punkte für Rost auf Kopf- und Fußplatten."),
        "styles": t("Weathered slats 0.48 to 0.56, the replacement slat 0.78, frames 0.34 to 0.4, rusted feet 0.22 to 0.28, slabs 0.7, root 0.36; body pen 0.25 mm on every part, details 0.18 mm, traces 0.13 mm.", "Verwitterte Latten 0,48 bis 0,56, Ersatzlatte 0,78, Rahmen 0,34 bis 0,4, rostige Füße 0,22 bis 0,28, Platten 0,7, Wurzel 0,36; Körperstift 0,25 mm an jedem Teil, Details 0,18 mm, Spuren 0,13 mm."),
        "overlays": t("One thermal region per shape; carriage bolt heads, drying checks, a graffiti tag, sticker residue and mower scuffs on the outer legs as detail and texture strokes.", "Eine thermische Region je Form; Köpfe der Schlossschrauben, Trockenrisse, ein Graffiti-Tag, Aufkleberreste und Mähspuren an den äußeren Beinen als Detail- und Texturstriche."),
    },
    "100": {
        "silhouette": t("The 30 slats of the six segments with their gaps, the replacement slat, the radial frames, the legs with end plate, thread and foot plate, the slabs and the root.", "Die 30 Latten der sechs Segmente mit ihren Fugen, die Ersatzlatte, die Radialrahmen, die Beine mit Kopfplatte, Gewinde und Fußplatte, die Platten und die Wurzel."),
        "retained": t("Seat height and depth, the six radial joints, the replacement slat, the twelve legs and slabs, the raised slab and the root.", "Sitzhöhe und -tiefe, die sechs Radialstöße, die Ersatzlatte, die zwölf Beine und Platten, die angehobene Platte und die Wurzel."),
        "merged": t("The 8 mm carriers into the frame bearer, the nut into the thread; the 10 mm slat gaps stay as the line between the slat outlines.", "Die 8-mm-Tragschienen in den Rahmenträger, die Mutter ins Gewinde; die 10-mm-Lattenfugen bleiben als Linie zwischen den Lattenumrissen."),
        "omitted": t("Mitre gaps, splits, bolt heads, graffiti and stickers.", "Gehrungsfugen, Risse, Schraubenköpfe, Graffiti und Aufkleber."),
        "thinElements": t("End plates and foot plates drawn 15 mm (0.15 mm print minimum), the 16 mm thread at its true diameter, the split ends closed.", "Kopf- und Fußplatten mit 15 mm gezeichnet (Druckminimum 0,15 mm), das 16-mm-Gewinde mit wahrem Durchmesser, die gespaltenen Enden geschlossen."),
        "hatching": t("Grain lines and the algae stipple in plan, stipple on the slabs, dots on the feet.", "Maserlinien und Algenpunktraster im Grundriss, Punktraster auf den Platten, Punkte an den Füßen."),
        "styles": t("Slats 0.54, replacement slat 0.78, frames 0.34 to 0.4, feet 0.22 to 0.28, slabs 0.7, root 0.36; body pen 0.18 mm.", "Latten 0,54, Ersatzlatte 0,78, Rahmen 0,34 bis 0,4, Füße 0,22 bis 0,28, Platten 0,7, Wurzel 0,36; Körperstift 0,18 mm."),
        "overlays": t("One region per shape; mower scuffs as texture strokes.", "Eine Region je Form; Mähspuren als Texturstriche."),
    },
    "200": {
        "silhouette": t("Six seat boards closing the hexagon, the replacement slat, six bearers, twelve legs on 30 mm foot blocks, the slab strips and the root.", "Sechs Sitzbretter schließen das Sechseck, die Ersatzlatte, sechs Träger, zwölf Beine auf 30-mm-Fußblöcken, die Plattenstreifen und die Wurzel."),
        "retained": t("Overall size, seat height, the ring form around the trunk, the legs, the slabs and the raised slab.", "Gesamtgröße, Sitzhöhe, die Ringform um den Stamm, die Beine, die Platten und die angehobene Platte."),
        "merged": t("Thread, nut and foot plate into one foot block of 30 mm, slats into boards, carriers into the bearers, the inner and outer slab of a frame (60 mm apart, 0.12 mm printed) into one strip except at the lifted frame 1.", "Gewinde, Mutter und Fußplatte zu einem 30-mm-Fußblock, Latten zu Brettern, Tragschienen in die Träger, innere und äußere Platte eines Rahmens (60 mm auseinander, 0,12 mm gedruckt) zu einem Streifen außer am angehobenen Rahmen 1."),
        "omitted": t("Gaps, splits, bolts and every trace of use below 30 mm.", "Fugen, Risse, Schrauben und jede Gebrauchsspur unter 30 mm."),
        "thinElements": t("Every part at least 30 mm (0.15 mm on paper).", "Jedes Teil mindestens 30 mm (0,15 mm auf dem Papier)."),
        "hatching": t("None.", "Keine."),
        "styles": t("Boards 0.54, replacement slat 0.78, frames 0.34 to 0.4, feet 0.26, slabs 0.7, root 0.36.", "Bretter 0,54, Ersatzlatte 0,78, Rahmen 0,34 bis 0,4, Füße 0,26, Platten 0,7, Wurzel 0,36."),
        "overlays": t("One region per shape.", "Eine Region je Form."),
    },
    "500": {
        "silhouette": t("The hexagonal seat ring as six 75 mm boards on six 75 mm bearers and twelve 75 mm legs on foot blocks and slabs, and the root.", "Der sechseckige Sitzring als sechs 75-mm-Bretter auf sechs 75-mm-Trägern und zwölf 75-mm-Beinen auf Fußblöcken und Platten, und die Wurzel."),
        "retained": t("Overall size, seat height, the ring around the trunk, twelve legs and slabs.", "Gesamtgröße, Sitzhöhe, der Ring um den Stamm, zwölf Beine und Platten."),
        "merged": t("Slats and carriers into a 75 mm board, bearer and carrier into a 75 mm band below it, the feet into 75 mm blocks of leg width (so the gap between them still prints at 1:1000), the slabs of a frame into one strip, the root into one tapering capsule.", "Latten und Tragschienen zu einem 75-mm-Brett, Träger und Tragschiene zu einem 75-mm-Band darunter, die Füße zu 75-mm-Blöcken in Beinbreite (damit die Lücke dazwischen bei 1:1000 noch druckt), die Platten eines Rahmens zu einem Streifen, die Wurzel zu einer sich verjüngenden Kapsel."),
        "omitted": t("The 30 mm lift of the slab reads only as a step; every trace of use is below the scale.", "Die 30-mm-Anhebung der Platte liest sich nur als Absatz; jede Gebrauchsspur liegt unter dem Maßstab."),
        "thinElements": t("Every part at least 75 mm (0.15 mm print minimum).", "Jedes Teil mindestens 75 mm (Druckminimum 0,15 mm)."),
        "hatching": t("None.", "Keine."),
        "styles": t("Boards 0.54, replacement slat 0.78, frames 0.34 to 0.4, feet 0.26, slabs 0.7, root 0.36; body pen 0.13 mm.", "Bretter 0,54, Ersatzlatte 0,78, Rahmen 0,34 bis 0,4, Füße 0,26, Platten 0,7, Wurzel 0,36; Körperstift 0,13 mm."),
        "overlays": t("One region per shape.", "Eine Region je Form."),
    },
}

VOL_DECISIONS = {
    "50": [t("Body for 7.5 mm: every slat as its trapezoid prism with mitred ends and the 10 mm through-splits cut into two slat ends, the replacement slat in fresh robinia, two 30 x 8 mm carriers per segment, the six 80 x 40 mm bearers, twelve 60 mm legs (hollow sections modelled by their outer section, capped), end plates, hexagon nuts, M16 threads, foot plates, the twelve 300 x 300 x 80 mm slabs and the two-piece root; the slab of the inner foot of frame 1 stands 30 mm higher and its thread shows 30 mm less.", "Körper für 7,5 mm: jede Latte als ihr Trapezprisma mit Gehrungsenden, die 10-mm-Durchrisse in zwei Lattenenden geschnitten, die Ersatzlatte aus frischer Robinie, zwei Tragschienen 30 x 8 mm je Segment, die sechs Träger 80 x 40 mm, zwölf 60-mm-Beine (Hohlprofile mit ihrem Außenquerschnitt, verschlossen), Kopfplatten, Sechskantmuttern, M16-Gewinde, Fußplatten, die zwölf Platten 300 x 300 x 80 mm und die zweiteilige Wurzel; die Platte des inneren Fußes von Rahmen 1 steht 30 mm höher, sein Gewinde zeigt 30 mm weniger."),
           t("Structure: the slats and carriers as members of their segment, bearers and legs as members of their bones, end plate, thread and foot plate as the lower member of each leg, the slab as the footing member with its support solid at the slab bottom on the soil, the leg welds and the M10 carrier bolts as joints, one mass marker per bone at its centre of mass.", "Tragwerk: Latten und Tragschienen als Stäbe ihres Segments, Träger und Beine als Stäbe ihrer Knochen, Kopfplatte, Gewinde und Fußplatte als unterer Stab jedes Beins, die Platte als Fundamentstab mit ihrem Auflagerkörper an der Plattenunterseite auf dem Boden, die Beinschweißungen und die M10-Schrauben der Tragschienen als Gelenke, ein Massenmarker je Knochen in seinem Schwerpunkt.")],
    "100": [t("Body for 15 mm: the 30 slats 82 x 40 mm without their 10 mm splits, the carriers merged into a 48 mm bearer, the nut into the 16 mm thread, end plates and foot plates at 15 mm, slabs and root as at 1:50.", "Körper für 15 mm: die 30 Latten 82 x 40 mm ohne ihre 10-mm-Risse, die Tragschienen in einen 48-mm-Träger übernommen, die Mutter ins 16-mm-Gewinde, Kopf- und Fußplatten mit 15 mm, Platten und Wurzel wie bei 1:50."),
            t("Structure: slats, bearers and legs as members, end plate, thread and foot plate as the lower leg members, slabs with their supports, welds and carrier bolts (15 mm) as joints, mass markers.", "Tragwerk: Latten, Träger und Beine als Stäbe, Kopfplatte, Gewinde und Fußplatte als untere Beinstäbe, Platten mit ihren Auflagern, Schweißungen und Tragschienenschrauben (15 mm) als Gelenke, Massenmarker.")],
    "200": [t("Body for 30 mm: six boards with the mitre gaps closed, the replacement slat, 48 mm bearers, legs down to 30 mm foot blocks that take plate, nut and thread in, one slab strip per frame (two slabs at the lifted frame 1) and the two root pieces.", "Körper für 30 mm: sechs Bretter mit geschlossenen Gehrungsfugen, die Ersatzlatte, 48-mm-Träger, Beine bis zu 30-mm-Fußblöcken, die Platte, Mutter und Gewinde aufnehmen, ein Plattenstreifen je Rahmen (zwei Platten am angehobenen Rahmen 1) und die zwei Wurzelstücke."),
            t("Structure: boards, bearers, legs, foot blocks and slabs as members, welds as joints, supports at the slab bottoms, mass markers; the carrier bolts are below the scale.", "Tragwerk: Bretter, Träger, Beine, Fußblöcke und Platten als Stäbe, Schweißungen als Gelenke, Auflager an den Plattenunterseiten, Massenmarker; die Tragschienenschrauben liegen unter dem Maßstab.")],
    "500": [t("Body for 75 mm: six 75 mm boards (slats and carriers), 75 mm bearer bands below them, 75 mm legs on 75 mm foot blocks, the slabs (inner and outer slab of a frame, 60 mm apart, as one strip except at the lifted frame 1) and the root as one tapering capsule.", "Körper für 75 mm: sechs 75-mm-Bretter (Latten und Tragschienen), 75-mm-Trägerbänder darunter, 75-mm-Beine auf 75-mm-Fußblöcken, die Platten (innere und äußere Platte eines Rahmens, 60 mm auseinander, als ein Streifen außer am angehobenen Rahmen 1) und die Wurzel als eine sich verjüngende Kapsel."),
            t("Structure: boards, bearers, legs, foot blocks and slabs as members, supports at the slab bottoms, mass markers; welds and bolts are below the scale.", "Tragwerk: Bretter, Träger, Beine, Fußblöcke und Platten als Stäbe, Auflager an den Plattenunterseiten, Massenmarker; Schweißungen und Schrauben liegen unter dem Maßstab.")],
}


REFERENCES = [
    {"id": "din-en-1728", "citation": "DIN EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability (seat static load, seat front edge static load, seat fatigue)."},
    {"id": "din-en-16139", "citation": "DIN EN 16139:2013 Furniture - Strength, durability and safety - Requirements for non-domestic seating (level 2 for public areas with heavy and frequent use)."},
    {"id": "din-en-1991-1-1", "citation": "DIN EN 1991-1-1:2010 Eurocode 1: Actions on structures - Densities, self-weight, imposed loads; densities of concrete (24 kN/m3 reinforced, 23 kN/m3 plain) and steel (78.5 kN/m3)."},
    {"id": "din-18040-3", "citation": "DIN 18040-3:2014 Barrier-free construction - Public circulation areas and open spaces: seating 0.46 to 0.48 m high recommended at intervals along paths; tree benches of 0.45 m are the common municipal type."},
    {"id": "din-en-338", "citation": "DIN EN 338:2016 Structural timber - Strength classes: hardwood class D40 (bending strength 40 N/mm2, mean modulus of elasticity parallel 13 kN/mm2, mean density 660 kg/m3 at 12 percent moisture)."},
    {"id": "holzatlas-robinie", "citation": "Wagenführ, R. and Wagenführ, A. (2022). Holzatlas, 7th ed., Hanser: Robinia pseudoacacia, mean density 0.73 g/cm3 at 12 percent moisture, durability class 1-2 (DIN EN 350), the standard hardwood of German playground and park furniture."},
    {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded structural hollow sections - Tolerances, dimensions and sectional properties: RHS 80 x 40 x 4 (8.96 cm2, 7.03 kg/m), SHS 60 x 60 x 3 (6.84 cm2, 5.37 kg/m)."},
    {"id": "en-iso-1461", "citation": "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles; with EN 13438 powder coating of galvanised steel (duplex system, DB 703 iron-mica anthracite as the usual municipal finish)."},
    {"id": "din-en-1339", "citation": "DIN EN 1339:2003 Concrete paving flags - Requirements and test methods; the 300 x 300 x 80 mm flag (Gehwegplatte) used as a footing slab."},
    {"id": "din-18920", "citation": "DIN 18920:2014 Vegetation technology in landscaping - Protection of trees, plants and vegetation areas during construction work: no excavation in the root zone (crown drip line plus 1.5 m), hence furniture under trees on surface slabs without dug foundations."},
    {"id": "fll-ztv-baumpflege", "citation": "FLL (2017). ZTV-Baumpflege: Zusätzliche Technische Vertragsbedingungen und Richtlinien für Baumpflege, 6th ed., Forschungsgesellschaft Landschaftsentwicklung Landschaftsbau, Bonn (root protection, root heave under paving)."},
    {"id": "galk-strassenbaumliste", "citation": "GALK Straßenbaumliste (2023), Deutsche Gartenamtsleitungskonferenz: Tilia cordata and Tilia x europaea as park and street trees, trunk diameters of 0.5 to 0.8 m at 60 to 80 years."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (hardwood 0.18 W/mK and 1600 J/kgK at 700 kg/m3, steel 50 W/mK and 450 J/kgK, concrete 2.0 W/mK and 1000 J/kgK)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley: total hemispherical emissivity of wood 0.82 to 0.92, of oxidised steel 0.79, of weathered concrete 0.88 to 0.93, solar absorptivities of grey and dark painted surfaces."},
    {"id": "levinson-2007-weathering", "citation": "Berdahl, P., Akbari, H., Levinson, R., Miller, W. A. (2008). Weathering of roofing materials - An overview. Construction and Building Materials 22(4), 423-433: solar reflectance of exposed surfaces falls by soiling and photodegradation; weathered grey wood and chalked dark coatings absorb more than new ones (authored values for this asset from this trend)."},
    {"id": "municipal-tree-bench-catalogues", "citation": "German municipal street-furniture catalogue types of hexagonal tree surround benches (Baumrundbank, sechseckig, ohne Lehne): 2.0 to 2.6 m across the corners, seat height 0.45 m, seat depth 0.40 to 0.50 m, 40 to 45 mm hardwood slats on galvanised and powder-coated steel frames with levelling feet, supplied in six bolted segments for erection around an existing tree; catalogue-typical data without any brand."},
    {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment (clothing surface temperatures of seated persons about 28 to 32 degC)."},
]


def materials():
    """🧱️ The materials, weathered where the bench has weathered."""
    return [
        {"id": "robinia-weathered", "title": t("Weathered robinia (greyed, checked)", "Verwitterte Robinie (vergraut, rissig)"), "emissivity": 0.9, "solarAbsorptivity": 0.68, "densityKgM3": ROBINIA, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 13000000000, "strengthPa": 40000000, "source": "authored-estimate", "reference": "levinson-2007-weathering"},
        {"id": "robinia-fresh", "title": t("Fresh robinia (replacement slat)", "Frische Robinie (Ersatzlatte)"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": ROBINIA, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 13000000000, "strengthPa": 40000000, "source": "literature", "reference": "holzatlas-robinie"},
        {"id": "steel-powder-coated", "title": t("Galvanised steel, powder-coated anthracite, chalked", "Verzinkter Stahl, anthrazit pulverbeschichtet, gekreidet"), "emissivity": 0.9, "solarAbsorptivity": 0.88, "densityKgM3": STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "steel-rusted", "title": t("Rusting steel of the levelling feet", "Rostender Stahl der Stellfüße"), "emissivity": 0.8, "solarAbsorptivity": 0.78, "densityKgM3": STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
        {"id": "concrete-weathered", "title": t("Weathered concrete paving flag", "Verwitterte Betongehwegplatte"), "emissivity": 0.92, "solarAbsorptivity": 0.68, "densityKgM3": CONCRETE, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 30000000000, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "bark", "title": t("Lime root bark", "Rinde der Lindenwurzel"), "emissivity": 0.95, "solarAbsorptivity": 0.72, "densityKgM3": 900, "conductivityWmK": 0.25, "specificHeatJKgK": 2000, "source": "authored-estimate", "reference": "incropera-2018"},
    ]


AREAS = {}


def thermal_model():
    """🌡️ The passive thermal model: sun under a lime canopy on greyed hardwood, dark steel, rusty feet, slabs and the root."""
    def region(rid, title, kind, material, geometry, length, sky, layer, cap, reference, source="authored-estimate"):
        return {"id": rid, "title": title, "kind": kind, "model": "passive", "material": material, "areaM2": round(AREAS[rid], 3), "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky,
                "layers": [layer], "heatCapacityJM2K": cap, "source": source, "reference": reference}

    def layer(lid, title, kind, material, thick, k, cap, reference):
        return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": thick, "resistanceM2KW": round(thick / k, 6), "heatCapacityJM2K": cap, "source": "literature", "reference": reference}

    wood_cap = round(ROBINIA * 1600 * min(SLAT_T / 2, math.sqrt(0.18 / (ROBINIA * 1600) * 3600)))
    return {
        "environment": {"airTempC": 26, "meanRadiantTempC": 26, "skyTempC": 14, "groundTempC": 20, "relativeHumidity": 0.5, "windMS": 1.0, "windDirectionDeg": 200, "solarWM2": 750, "diffuseFraction": 0.2, "pressurePa": 101325},
        "passengers": [],
        "operating": [],
        "regions": [
            region("seat-slats", t("Greyed robinia seat slats", "Vergraute Robinien-Sitzlatten"), "surface", "robinia-weathered", "horizontal-plate-up", 0.45, 0.35,
                   layer("robinia-slat", t("Weathered robinia slat", "Verwitterte Robinienlatte"), "timber", "robinia-weathered", SLAT_T, 0.18, wood_cap, "iso-10456-2007"), wood_cap, "levinson-2007-weathering"),
            region("replaced-slat", t("Fresh replacement slat", "Frische Ersatzlatte"), "surface", "robinia-fresh", "horizontal-plate-up", 0.082, 0.35,
                   layer("robinia-fresh-slat", t("Fresh robinia slat", "Frische Robinienlatte"), "timber", "robinia-fresh", SLAT_T, 0.18, wood_cap, "iso-10456-2007"), wood_cap, "holzatlas-robinie", "literature"),
            region("frame-coating", t("Powder-coated steel frames and carriers", "Pulverbeschichtete Stahlrahmen und Tragschienen"), "surface", "steel-powder-coated", "vertical-plate", 0.06, 0.25,
                   layer("hollow-section-wall", t("Hollow section wall 3 to 4 mm", "Hohlprofilwand 3 bis 4 mm"), "metal", "steel-powder-coated", 0.004, 50, round(STEEL * 450 * 0.004), "en-10219-2"), round(STEEL * 450 * 0.004), "incropera-2018"),
            region("feet-rust", t("Rusting levelling feet", "Rostende Stellfüße"), "surface", "steel-rusted", "vertical-plate", 0.05, 0.15,
                   layer("foot-steel", t("Foot plate and thread steel", "Stahl von Fußplatte und Gewinde"), "metal", "steel-rusted", 0.010, 50, round(STEEL * 450 * 0.010), "incropera-2018"), round(STEEL * 450 * 0.010), "incropera-2018", "literature"),
            region("footing-pads", t("Concrete footing slabs", "Betonfundamentplatten"), "surface", "concrete-weathered", "horizontal-plate-up", 0.3, 0.3,
                   layer("paving-flag", t("Concrete paving flag 80 mm", "Betongehwegplatte 80 mm"), "masonry", "concrete-weathered", PAD[1], 2.0, round(CONCRETE * 1000 * min(PAD[1], math.sqrt(2.0 / (CONCRETE * 1000) * 3600))), "iso-10456-2007"), round(CONCRETE * 1000 * min(PAD[1], math.sqrt(2.0 / (CONCRETE * 1000) * 3600))), "iso-10456-2007", "literature"),
            {"id": "root-bark", "title": t("Lime root bark", "Rinde der Lindenwurzel"), "kind": "bark", "model": "passive", "material": "bark", "areaM2": round(AREAS["root-bark"], 3), "characteristicLengthM": 0.15, "geometry": "cylinder", "skyViewFactor": 0.3,
             "layers": [layer("root-bark-layer", t("Bark and sapwood", "Rinde und Splintholz"), "timber", "bark", 0.02, 0.25, round(900 * 2000 * 0.02), "incropera-2018")], "heatCapacityJM2K": round(900 * 2000 * 0.02), "source": "authored-estimate", "reference": "fll-ztv-baumpflege"},
        ],
        "emitters": [],
        "assumptions": [
            t("Passive surfaces only: absorbed sun, long-wave exchange with sky and ground, convection and conduction with heat capacity; the bench draws no power (0 W) and has no emitters.", "Nur passive Flächen: absorbierte Sonne, Langwellenaustausch mit Himmel und Boden, Konvektion und Leitung mit Wärmekapazität; die Bank nimmt keine Leistung auf (0 W) und hat keine Emitter."),
            t("Weathering changes the optics: greyed, checked robinia absorbs about 0.68 of the sun against about 0.5 for the fresh replacement slat, the chalked anthracite coating about 0.88, rusting feet 0.78 with emissivity 0.8 (authored from the weathering trend of Berdahl et al. 2008 and the tables of Incropera).", "Verwitterung ändert die Optik: vergraute, rissige Robinie absorbiert etwa 0,68 der Sonne gegen etwa 0,5 bei der frischen Ersatzlatte, die gekreidete Anthrazitbeschichtung etwa 0,88, rostende Füße 0,78 bei Emissionsgrad 0,8 (nach dem Verwitterungstrend von Berdahl et al. 2008 und den Tabellen von Incropera)."),
            t("The bench stands under the crown of the lime it surrounds: in service only about a third of the seat is in sun (dappled shade), so sunlit fractions are scenario data of each pose and of the shade clip; the dark steel frames heat faster than the slats because of their low heat capacity.", "Die Bank steht unter der Krone der Linde, die sie umgibt: im Betrieb liegt nur etwa ein Drittel der Sitzfläche in der Sonne (Lichtflecken), die besonnten Anteile sind daher Szenariodaten jeder Pose und des Schattenclips; die dunklen Stahlrahmen erwärmen sich wegen ihrer kleinen Wärmekapazität schneller als die Latten."),
            t("Areal heat capacity is rho*cp*min(thickness, sqrt(alpha*3600 s)): the part of the section that takes part within one hour (slats 20 mm of 40, slabs the full 80 mm).", "Die flächenbezogene Wärmekapazität ist rho*cp*min(Dicke, sqrt(alpha*3600 s)): der Teil des Querschnitts, der binnen einer Stunde teilnimmt (Latten 20 von 40 mm, Platten die vollen 80 mm)."),
            t("Slabs and root lie mostly in the gravel: their area is the whole outer area, the pose binds the exposed top as exposedFraction and the soil contact at 18 degC as contact.", "Platten und Wurzel liegen größtenteils in der Kiesdecke: ihre Fläche ist die ganze Außenfläche, die Pose bindet die freie Oberseite als exposedFraction und den Bodenkontakt bei 18 degC als contact."),
            t("Seated visitors cover about 0.16 m2 of slat each (ISO 7730 clothing surface 30 degC).", "Sitzende Besucher bedecken je etwa 0,16 m2 Latte (Bekleidungsoberfläche 30 degC nach ISO 7730)."),
        ],
    }


def anchors():
    """🪝️ Foot contacts under the twelve slabs and the two hand holds of the depot crew on segment 1."""
    out = []
    for k in SEGMENTS:
        for side in ("i", "o"):
            key = leg_key(k, side)
            out.append({"id": f"foot-{key}", "bone": f"pad-{key}", "x": PAD[1], "y": 0, "kind": "contact", "title": t(f"Slab bottom on the soil, frame {k + 1} {'inner' if side == 'i' else 'outer'}", f"Plattenunterseite auf dem Boden, Rahmen {k + 1} {'innen' if side == 'i' else 'außen'}"),
                        "support": "contact", "normalDeg": 90, "footprintLengthM": PAD[0], "footprintWidthM": PAD[0], "frictionCoefficient": 0.6})
    seg = BONES["seg-0"]
    for s, tag, title in ((+1, "left", t("Left hand hold of the depot crew", "Linker Griff des Bauhofteams")), (-1, "right", t("Right hand hold of the depot crew", "Rechter Griff des Bauhofteams"))):
        a = A_OUT - SLAT_W / 2
        x, z = seg_point(0, a, s * (half_len(a, END_GAP) - 0.10))
        loc = seg.local((x, SLAT_BOT, z))
        out.append({"id": f"lift-{tag}", "bone": "seg-0", "x": r5(loc[0]), "y": r5(loc[1]), "z": r5(loc[2]), "kind": "load", "title": title})
    return out


def binding(seat, frame, slabs=(0.25, 0.25), root=(0.3, 0.2), contact_seat=None):
    """🌬️ Region bindings of a pose: exposed and sunlit fractions per region."""
    out = {
        "seat-slats": {"exposedFraction": seat[0], "sunlitFraction": seat[1]},
        "replaced-slat": {"exposedFraction": seat[0], "sunlitFraction": seat[1]},
        "frame-coating": {"exposedFraction": 1.0, "sunlitFraction": frame},
        "feet-rust": {"exposedFraction": 0.9, "sunlitFraction": frame * 0.5, "contact": {"fraction": 0.1, "tempC": 20}},
        "footing-pads": {"exposedFraction": slabs[0], "sunlitFraction": slabs[1], "contact": {"fraction": round(1 - slabs[0], 3), "tempC": 18}},
        "root-bark": {"exposedFraction": root[0], "sunlitFraction": root[1], "contact": {"fraction": round(1 - root[0], 3), "tempC": 18}},
    }
    if contact_seat:
        out["seat-slats"]["contact"] = {"fraction": contact_seat, "tempC": 30}
        out["replaced-slat"]["contact"] = {"fraction": contact_seat, "tempC": 30}
    return out


FEET = [f"foot-{leg_key(k, s)}" for k in SEGMENTS for s in ("i", "o")]


def sitter_loads():
    """🧑‍🤝‍🧑️ Six seated adults, one on every segment, facing out with their weight 0.15 m behind the front edge."""
    out = []
    for k in SEGMENTS:
        seg = BONES[f"seg-{k}"]
        x, z = seg_point(k, A_OUT - 0.15, 0.0)
        loc = seg.local((x, SEAT_TOP, z))
        out.append({"id": f"sitter-{k + 1}", "kind": "occupant", "title": t(f"Seated adult on segment {k + 1} (75 kg)", f"Sitzender Erwachsener auf Segment {k + 1} (75 kg)"), "bone": seg.id, "x": r5(loc[0]), "y": r5(loc[1]), "z": r5(loc[2]), "fxN": 0, "fyN": round(-75 * G, 1), "fzN": 0, "source": "authored-estimate", "reference": "din-en-16139"})
    return out


def carry_loads():
    """🤲️ The hands of the depot crew as applied carrying forces: each worker holds half the weight of segment 1 at an outer slat end (the two ends lie symmetric about its centre of mass)."""
    half = round(BONES["seg-0"].mass["kg"] * 9.80665 / 2, 1)
    return [{"id": f"carry-{tag}", "kind": "equipment", "title": t(f"Depot worker carrying half of segment 1 at its {tag} end", f"Bauhofmitarbeiter trägt die Hälfte von Segment 1 am {'linken' if tag == 'left' else 'rechten'} Ende"), "anchor": f"lift-{tag}", "fxN": 0, "fyN": half, "source": "authored-estimate"} for tag in ("left", "right")]


def poses():
    """📥️ In service (default), maintenance with segment 1 lifted out, in use with six sitters."""
    return [
        {"id": "in-service", "title": t("In service", "Im Betrieb"), "description": t("Empty bench around the lime on its twelve slabs in the water-bound gravel, the inner slab of frame 1 lifted 30 mm by a root, dappled shade on the seat.", "Leere Bank um die Linde auf ihren zwölf Platten in der wassergebundenen Decke, die innere Platte von Rahmen 1 von einer Wurzel um 30 mm angehoben, Lichtflecken auf der Sitzfläche."),
         "bones": {}, "ground": "authored", "contacts": FEET, "thermal": {"regions": binding((1.0, 0.35), 0.3), "emitters": {}}, "loads": []},
        {"id": "maintenance", "title": t("Maintenance: segment lifted out", "Wartung: Segment ausgehoben"), "description": t("The depot crew has undone the four M10 bolts of segment 1 and lifts it 0.3 m off its two bearers by hand to replace split slats; frames, feet and slabs stay behind on the ground.", "Das Bauhofteam hat die vier M10-Schrauben von Segment 1 gelöst und hebt es von Hand 0,3 m von seinen zwei Trägern, um gespaltene Latten zu ersetzen; Rahmen, Füße und Platten bleiben am Boden zurück."),
         "bones": {"seg-0": {"dy": 0.3}}, "ground": "authored", "contacts": FEET, "thermal": {"regions": binding((1.0, 0.4), 0.4), "emitters": {}}, "loads": carry_loads()},
        {"id": "hands-on", "key": True, "title": t("Hands on the segment", "Hände am Segment"), "description": t("Key of the lift-out clip: the crew carries segment 1 while it still lies on its bearers.", "Schlüssel des Aushebe-Clips: das Team trägt Segment 1, solange es noch auf seinen Trägern liegt."),
         "bones": {}, "ground": "authored", "contacts": FEET, "thermal": {"regions": binding((1.0, 0.35), 0.3), "emitters": {}}, "loads": carry_loads()},
        {"id": "in-use", "title": t("In use: six sitters around the tree", "In Gebrauch: sechs Sitzende um den Baum"), "description": t("Six adults sit around the trunk, one on every segment, facing out into the park.", "Sechs Erwachsene sitzen um den Stamm, einer auf jedem Segment, mit Blick hinaus in den Park."),
         "bones": {}, "ground": "authored", "contacts": FEET, "thermal": {"regions": binding((0.8, 0.28), 0.25, contact_seat=0.2), "emitters": {}}, "loads": sitter_loads()},
    ]


def clips():
    """🎞️ Lifting segment 1 out, six visitors sitting down, and the canopy shade moving round the bench."""
    feet_spans = [{"anchor": a, "fromS": 0, "toS": None} for a in FEET]

    def spans(dur, extra=()):
        return [{"anchor": a, "fromS": 0, "toS": dur} for a in FEET] + [{"anchor": a, "fromS": f, "toS": dur} for a, f in extra]

    sit_tracks = []
    sit_events = []
    for k in SEGMENTS:
        t0 = 0.5 + 1.1 * k
        sit_tracks.append({"target": f"load:sitter-{k + 1}", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": round(t0, 2), "v": 0}, {"t": round(t0 + 1.0, 2), "v": 1}, {"t": 8, "v": 1}]})
        sit_events.append({"t": round(t0 + 1.0, 2), "kind": "load-on", "title": t(f"Visitor {k + 1} sits down", f"Besucher {k + 1} setzt sich"), "target": f"sitter-{k + 1}"})
    shade = []
    for rid, lo, hi in (("seat-slats", 0.15, 0.7), ("replaced-slat", 0.15, 0.7), ("frame-coating", 0.1, 0.6), ("feet-rust", 0.05, 0.3), ("footing-pads", 0.1, 0.5), ("root-bark", 0.1, 0.4)):
        shade.append({"target": f"region:{rid}:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": lo}, {"t": 30, "v": hi}, {"t": 60, "v": lo}]})
    return [
        {"id": "segment-lift-out", "title": t("Segment lifted out by the depot", "Segment vom Bauhof ausgehoben"), "description": t("The crew undoes the four carrier bolts of segment 1, takes it at its outer slat ends and lifts it straight up 0.3 m off the bearers; the frames stay on their slabs.", "Das Team löst die vier Tragschienenschrauben von Segment 1, fasst es an den äußeren Lattenenden und hebt es 0,3 m senkrecht von den Trägern; die Rahmen bleiben auf ihren Platten."),
         "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 1.0, "pose": "in-service", "ease": "smooth"}, {"t": 1.5, "pose": "hands-on", "ease": "smooth"}, {"t": 5, "pose": "maintenance", "ease": "smooth"}, {"t": 6, "pose": "maintenance", "ease": "smooth"}],
         "contacts": spans(6),
         "phases": [{"id": "unbolting", "fromS": 0, "toS": 1.5, "title": t("Bolts undone", "Schrauben gelöst")}, {"id": "lifting", "fromS": 1.5, "toS": 6, "title": t("Segment lifted", "Segment gehoben")}],
         "events": [{"t": 1.5, "kind": "load-on", "title": t("Crew takes the segment", "Team fasst das Segment"), "target": "carry-left"}, {"t": 5, "kind": "motion", "title": t("Segment clear of the bearers", "Segment frei von den Trägern")}],
         "tracks": []},
        {"id": "visitors-sit-down", "title": t("Visitors sit down around the tree", "Besucher setzen sich um den Baum"), "description": t("One visitor after the other sits down on the six segments; the slabs carry the growing load into the root zone.", "Ein Besucher nach dem anderen setzt sich auf die sechs Segmente; die Platten tragen die wachsende Last in den Wurzelraum."),
         "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 8, "pose": "in-use", "ease": "smooth"}],
         "contacts": spans(8), "phases": [{"id": "arriving", "fromS": 0, "toS": 8, "title": t("Visitors arrive", "Besucher kommen")}], "events": sit_events, "tracks": sit_tracks},
        {"id": "canopy-shade-cycle", "title": t("Canopy shade moving round the bench", "Kronenschatten wandert um die Bank"), "description": t("Illustrative afternoon: the shadow of the lime crown moves off the seat and back; the dark frames warm and cool faster than the slats and the slabs.", "Beispielhafter Nachmittag: der Schatten der Lindenkrone wandert von der Sitzfläche und zurück; die dunklen Rahmen erwärmen und kühlen sich schneller als Latten und Platten."),
         "durationS": 60, "fps": 30, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 30, "pose": "in-service", "ease": "smooth"}, {"t": 60, "pose": "in-service", "ease": "smooth"}],
         "contacts": spans(60), "phases": [{"id": "shade", "fromS": 0, "toS": 30, "title": t("Shade leaves", "Schatten weicht")}, {"id": "sun", "fromS": 30, "toS": 60, "title": t("Shade returns", "Schatten kehrt zurück")}],
         "events": [{"t": 30, "kind": "thermal", "title": t("Most sun on the seat", "Meiste Sonne auf der Sitzfläche")}], "tracks": shade},
    ]


def compute_areas():
    """📐️ Outer area of every region at 1:50 (prism areas less the faces two solids of one region share)."""
    parts = parts_of("50")
    areas = {}
    for p in parts:
        areas[p.region] = areas.get(p.region, 0.0) + p.area()
    carrier = abs(area2(carrier_poly(0, 1)))
    areas["frame-coating"] -= 12 * 2 * carrier + 12 * 2 * LEG_S ** 2
    thread = math.pi * (THREAD_D / 2) ** 2
    areas["feet-rust"] -= 12 * (2 * thread + 2 * thread + 2 * (THREAD_D * math.pi * NUT_H) * 0 + 2 * NUT_H * 0)
    areas["root-bark"] -= 4 * math.pi * ROOT_A[1][3] ** 2
    AREAS.update(areas)


def extents():
    """📦️ The rest bounding box of the 1:50 body."""
    parts = parts_of("50")
    xs = [v for p in parts for v in p.xs()]
    ys = [v for p in parts for v in p.ys()]
    zs = [v for p in parts for v in p.zs()]
    return (min(xs), max(xs)), (min(ys), max(ys)), (min(zs), max(zs))


def main():
    """🖨️ Writes the asset document and prints the numbers of the reference table."""
    build_rig()
    compute_areas()
    ex, ey, ez = extents()
    mass = sum(b.mass["kg"] for b in BONES.values() if b.mass)
    asset = {
        "id": "used-tree-surround-bench",
        "category": "furniture",
        "title": t("Used Tree Surround Bench", "Gebrauchte Baumrundbank"),
        "summary": t("A hexagonal municipal tree surround bench around an old lime, 2.4 m across the seat corners: six segments of five greyed and split robinia slats on powder-coated steel frames with twelve levelling feet on concrete slabs laid flush in the gravel, one slat replaced in fresh wood and one inner slab lifted 30 mm by a root; the depot lifts a segment out, six visitors sit round the trunk.",
                     "Eine sechseckige kommunale Baumrundbank um eine alte Linde, 2,4 m über die Sitzecken: sechs Segmente aus je fünf vergrauten und gespaltenen Robinienlatten auf pulverbeschichteten Stahlrahmen mit zwölf Stellfüßen auf bündig in die Kiesdecke gelegten Betonplatten, eine Latte in frischem Holz ersetzt und eine innere Platte von einer Wurzel um 30 mm angehoben; der Bauhof hebt ein Segment aus, sechs Besucher sitzen um den Stamm."),
        "tags": ["used", "park", "public-space", "seating", "tree-bench", "bench", "street-furniture", "timber", "robinia", "steel", "galvanised", "concrete", "weathered", "root-heave"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                       "notes": t("Construction after German municipal catalogue types of the backless hexagonal tree surround bench: seat 0.45 m high and 0.45 m deep (DIN 18040-3, DIN EN 16139), six bolted segments of five robinia slats 82 x 40 mm with 10 mm gaps on two steel carriers 30 x 8 mm, six radial frames of RHS 80 x 40 x 4 laid flat on two SHS 60 x 60 x 3 legs each, M16 levelling feet, 300 x 300 x 80 mm paving flags as footings laid without excavation in the root zone (DIN 18920). Masses from the modelled sections and densities (robinia 730, steel 7850, concrete 2300 kg/m3); the lime root is the tree's and carries no mass of the asset. Nothing is measured.",
                                  "Konstruktion nach Katalogtypen deutscher Kommunen für die lehnenlose sechseckige Baumrundbank: Sitz 0,45 m hoch und 0,45 m tief (DIN 18040-3, DIN EN 16139), sechs verschraubte Segmente aus je fünf Robinienlatten 82 x 40 mm mit 10-mm-Fugen auf zwei Stahltragschienen 30 x 8 mm, sechs Radialrahmen aus flach liegendem Rechteckrohr 80 x 40 x 4 auf je zwei Vierkantbeinen 60 x 60 x 3, M16-Stellfüße, Gehwegplatten 300 x 300 x 80 mm als Fundamente ohne Aushub im Wurzelraum (DIN 18920). Massen aus den modellierten Querschnitten und Dichten (Robinie 730, Stahl 7850, Beton 2300 kg/m3); die Lindenwurzel gehört zum Baum und trägt keine Masse des Assets. Nichts ist gemessen.")},
        "dimensions": {"heightM": r5(ey[1] - ey[0]), "widthM": r5(ex[1] - ex[0]), "depthM": r5(ez[1] - ez[0]), "massKg": round(mass, 1)},
        "materials": materials(),
        "rig": {"kind": "structural", "root": "ground", "bones": [b.json() for b in BONES.values()]},
        "anchors": anchors(),
        "representations": {s: representation(s) for s in SCALES},
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal_model(),
        "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": FEET,
                      "assumptions": [
                          t("Load path: slats span tangentially between the carriers of their segment, the carriers bear on the six radial RHS bearers, each bearer stands on an inner and an outer SHS leg, each leg on its levelling foot and concrete flag, and the flag bottom bears on the soil of the root zone (unilateral contact, friction 0.6).", "Lastpfad: die Latten spannen tangential zwischen den Tragschienen ihres Segments, die Tragschienen liegen auf den sechs radialen Rechteckrohrträgern, jeder Träger steht auf einem inneren und einem äußeren Vierkantbein, jedes Bein auf seinem Stellfuß und seiner Betonplatte, und die Plattenunterseite trägt auf dem Boden des Wurzelraums (einseitiger Kontakt, Reibung 0,6)."),
                          t("Sitters are 75 kg point loads per segment; the design level is DIN EN 16139 level 2 for public seating, tested by the static seat loads of DIN EN 1728; no member stress or fatigue check is made here.", "Sitzende sind 75-kg-Einzellasten je Segment; Bemessungsstufe ist DIN EN 16139 Stufe 2 für öffentliches Sitzen, geprüft mit den statischen Sitzlasten der DIN EN 1728; hier werden weder Spannungen noch Ermüdung nachgewiesen."),
                          t("In maintenance the lifted segment hangs on the two hands of the depot crew at its outer slat ends, applied as their carrying forces (64 N each, together the 127 N weight of the segment) because supports of the archive are springs of equal stiffness that would share the weight of the whole bench with the hands; the frames, feet and slabs stay on the ground.", "Bei der Wartung hängt das gehobene Segment an den zwei Händen des Bauhofteams an seinen äußeren Lattenenden, angesetzt als ihre Tragkräfte (je 64 N, zusammen die 127 N Gewicht des Segments), weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht der ganzen Bank mit den Händen teilen würden; Rahmen, Füße und Platten bleiben am Boden."),
                          t("The root has lifted the inner slab of frame 1 by 30 mm; the levelling foot was turned in by the same amount, so the seat is level and the bench stays in service.", "Die Wurzel hat die innere Platte von Rahmen 1 um 30 mm angehoben; der Stellfuß wurde um dasselbe Maß hineingedreht, so dass der Sitz waagrecht ist und die Bank im Betrieb bleibt."),
                      ], "clinical": False},
        "volumes": {s: volume(s) for s in SCALES},
        "presentation": {"defaultPose": "in-service", "defaultClip": "visitors-sit-down", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print(f"[DEBUG] extents x {ex} y {ey} z {ez}")
    print(f"[DEBUG] dimensions {asset['dimensions']}")
    for b in BONES.values():
        if b.mass:
            print(f"[DEBUG] mass {b.id} {b.mass['kg']}")
    print(f"[DEBUG] areas {AREAS}")
    for s in SCALES:
        v = asset["volumes"][s]
        print(f"[DEBUG] 1:{s} body {len(v['body'])} thermal {len(v['thermal'])} structure {len(v['structure'])} shapes " + " ".join(f"{k}={len(x)}" for k, x in asset['representations'][s]['geometry'].items()))


if __name__ == "__main__":
    main()
