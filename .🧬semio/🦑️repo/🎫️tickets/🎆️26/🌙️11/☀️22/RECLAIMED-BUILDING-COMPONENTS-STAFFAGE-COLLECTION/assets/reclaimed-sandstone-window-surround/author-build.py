"""🏛️ Authoritative chisel of the reclaimed sandstone window surround: states every hand-chosen number and writes `🔣️.json`.

The stones, the pallet, the timber spacers and the crane fork are axis-aligned prisms in world rest coordinates (the stored pose);
every 3D solid, every 2D shape of the three views and every structural and thermal number is derived from the same prisms per scale.

Usage: python3 author-build.py  (prints the reference numbers the report quotes)

@see ../../brief.md — the asset contract of this ticket
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/design.md — volume rules
"""
import json
import math
from pathlib import Path

from shapely.geometry import MultiPolygon, Polygon
from shapely.geometry.polygon import orient
from shapely.ops import unary_union

HERE = Path(__file__).resolve().parent
OUT = HERE / "🔣️.json"
SLUG = "reclaimed-sandstone-window-surround"
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}

RHO_STONE = 2100.0
RHO_TIMBER = 470.0
RHO_STEEL = 7850.0
RHO_MORTAR = 1600.0
G = 9.81
WALL_LOAD = 0.20 * 1.20 * 18000.0

OPEN_W = 1.00
OPEN_H = 1.50
JAMB_W = 0.22
JAMB_D = 0.20
HORN = 0.08
SILL_L = OPEN_W + 2 * JAMB_W + 2 * HORN
SILL_H = 0.16
LINTEL_H = 0.30
FRAME_H = SILL_H + OPEN_H + LINTEL_H

PAL_L = 1.80
PAL_Z = 0.40
RUN = 0.10
DECK_T = 0.022
DECK_W = 0.145
DECK_TOP = RUN + DECK_T
BEARER = 0.08
BEARER_X = (0.3485, 1.4515)
STONE_BED = DECK_TOP + BEARER
SPACER = 0.08
ZC = -0.485
Z_A = ZC - 0.265
Z_B = ZC + 0.115
X_F = 2.10
LIFT = 0.40
TINE_T = 0.05
ENTRY = DECK_TOP + 0.010
BEAR = STONE_BED - TINE_T
TINE_X = (0.60, 1.20)
TINE_L = 0.85
INSERT = -0.80

SILL_O = (0.10, STONE_BED)
JL_O = (0.15, STONE_BED + SILL_H + SPACER)
LINTEL_O = (0.10, STONE_BED)
JR_O = (0.15, STONE_BED + LINTEL_H + SPACER)
SPA_Y = STONE_BED + SILL_H
SPB_Y = STONE_BED + LINTEL_H
DECK_X = [k * (PAL_L - DECK_W) / 6 for k in range(7)]
DROP_X = DECK_X[2] + DECK_W / 2 - BEARER_X[0]


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def R(value, digits=6):
    """🔢️ A number rounded for the document."""
    out = round(float(value), digits)
    return 0.0 if out == 0 else out


class Prism:
    """🧱️ An axis-aligned prism in world rest metres: `profile` in the plane across `axis` ((x, y) for z, (y, z) for x, (z, x) for y), pushed from `lo` to `hi`."""

    def __init__(self, axis, profile, lo, hi, holes=(), fill2d=False):
        self.axis, self.lo, self.hi, self.fill2d = axis, lo, hi, fill2d
        self.poly = orient(Polygon(profile, [list(h) for h in holes]), 1.0)

    def length(self):
        return self.hi - self.lo

    def volume(self):
        return self.poly.area * self.length()

    def area(self):
        return 2 * self.poly.area + self.poly.length * self.length()

    def centroid(self):
        c = self.poly.centroid
        m = (self.lo + self.hi) / 2
        if self.axis == "z":
            return (c.x, c.y, m)
        if self.axis == "x":
            return (m, c.x, c.y)
        return (c.y, m, c.x)

    def bounds3(self):
        a0, b0, a1, b1 = self.poly.bounds
        if self.axis == "z":
            return (a0, a1), (b0, b1), (self.lo, self.hi)
        if self.axis == "x":
            return (self.lo, self.hi), (a0, a1), (b0, b1)
        return (b0, b1), (self.lo, self.hi), (a0, a1)


def box(x0, x1, y0, y1, z0, z1, axis="x"):
    """📦️ A rectangular prism given by its world ranges, pushed along `axis`."""
    if axis == "x":
        return Prism("x", [(y0, z0), (y1, z0), (y1, z1), (y0, z1)], x0, x1)
    if axis == "y":
        return Prism("y", [(z0, x0), (z1, x0), (z1, x1), (z0, x1)], y0, y1)
    return Prism("z", [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], z0, z1)


def circle(cx, cy, r, n=12):
    """⭕️ A polygon approximating a circle, counter-clockwise."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


class Part:
    """🧩️ One physical part on one bone: its prisms, material, region and tone, drawn as one shape per view."""

    def __init__(self, pid, bone, material, region, tone, prisms, title=None):
        self.id, self.bone, self.material, self.region, self.tone, self.prisms = pid, bone, material, region, tone, prisms
        self.sheet = False

    def volume(self):
        return sum(p.volume() for p in self.prisms)


def sill_section(scale, segment):
    """🪟️ The sill section in (t up, zeta into the wall) for one segment: horns level under the jambs, the opening part with Wasserschlag, upstand and drip."""
    if segment == "horn" or scale == 500:
        return [(0.0, -0.05), (0.0, 0.20), (SILL_H, 0.20), (SILL_H, -0.05)]
    if scale == 50:
        pts = [(-0.05, 0.0), (-0.035, 0.0), (-0.035, 0.015), (-0.018, 0.015), (-0.018, 0.0), (0.20, 0.0), (0.20, 0.12), (0.10, 0.12), (0.10, 0.16), (0.06, 0.16), (0.06, 0.13), (-0.05, 0.10)]
    elif scale == 100:
        pts = [(-0.05, 0.0), (0.20, 0.0), (0.20, 0.12), (0.10, 0.12), (0.10, 0.16), (0.06, 0.16), (0.06, 0.13), (-0.05, 0.10)]
    else:
        pts = [(-0.05, 0.0), (0.20, 0.0), (0.20, 0.16), (0.06, 0.16), (0.06, 0.13), (-0.05, 0.10)]
    return [(t, z) for z, t in pts]


def jamb_section(scale):
    """🧱️ The jamb section in (s from the opening edge, zeta into the wall): chamfered face arris, window rebate at the back of the opening edge."""
    if scale == 50:
        return [(0.02, 0.0), (0.22, 0.0), (0.22, 0.20), (0.055, 0.20), (0.055, 0.10), (0.0, 0.10), (0.0, 0.02)]
    if scale in (100, 200):
        return [(0.0, 0.0), (0.22, 0.0), (0.22, 0.20), (0.055, 0.20), (0.055, 0.10), (0.0, 0.10)]
    return [(0.0, 0.0), (0.22, 0.0), (0.22, 0.20), (0.0, 0.20)]


def lintel_section(scale, segment):
    """🏛️ The lintel section in (t up, zeta into the wall): crown band with drip groove, soffit chamfer and head rebate in the opening, spalled horn corner."""
    if scale == 500:
        pts = [(-0.04, 0.0), (0.20, 0.0), (0.20, LINTEL_H), (-0.04, LINTEL_H)]
        return [(t, z) for z, t in pts]
    crown = [(-0.04, 0.30), (-0.04, 0.24)]
    if scale == 50:
        crown += [(-0.030, 0.24), (-0.030, 0.252), (-0.018, 0.252), (-0.018, 0.24)]
    crown += [(0.0, 0.24)]
    if segment == "opening":
        low = [(0.02, 0.0), (0.10, 0.0), (0.10, 0.055), (0.20, 0.055)] if scale == 50 else [(0.0, 0.0), (0.10, 0.0), (0.10, 0.055), (0.20, 0.055)]
        pts = low + [(0.20, 0.30)] + crown + ([(0.0, 0.02)] if scale == 50 else [])
    elif segment == "spall":
        pts = [(0.0, 0.04), (0.20, 0.04), (0.20, 0.26), (0.16, 0.30)] + crown
    elif segment == "bearing-upper":
        pts = [(0.0, 0.04), (0.20, 0.04), (0.20, 0.30)] + crown
    else:
        pts = [(0.0, 0.0), (0.20, 0.0), (0.20, 0.30)] + crown
    return [(t, z) for z, t in pts]


def stone_prisms_x(section_tz, u0, u1, origin, z_plane, holes=()):
    """➡️ A stone segment along x from its (t, zeta) section: world profile (y, z)."""
    oy = origin[1]
    prof = [(oy + t, z_plane + z) for t, z in section_tz]
    return Prism("x", prof, origin[0] + u0, origin[0] + u1, holes)


def jamb_part(scale, pid, origin, z_plane, numeral):
    """🧱️ A jamb lying on its outer side face: v = 0.22 - s up, zeta along z; dowel holes, pintle stubs and mortar on the back at 1:50."""
    ox, oy = origin
    sec = [(oy + (JAMB_W - s), z_plane + z) for s, z in jamb_section(scale)]
    prisms = []
    stubs, mortar = [], []
    if scale == 50:
        hole = lambda: circle(oy + 0.11, z_plane + 0.10, 0.0125)
        prisms.append(Prism("x", sec, ox, ox + 0.04, [hole()]))
        prisms.append(Prism("x", sec, ox + 0.04, ox + OPEN_H - 0.04))
        prisms.append(Prism("x", sec, ox + OPEN_H - 0.04, ox + OPEN_H, [hole()]))
        for u in (0.30, 1.20):
            stubs.append(box(ox + u - 0.01, ox + u + 0.01, oy + JAMB_W - 0.04, oy + JAMB_W - 0.02, z_plane + 0.10, z_plane + 0.115))
        m0 = 0.40 if numeral == "II" else 0.75
        mortar.append(box(ox + m0, ox + m0 + 0.45, oy + 0.02, oy + 0.12, z_plane + JAMB_D, z_plane + JAMB_D + 0.012))
    else:
        prisms.append(Prism("x", sec, ox, ox + OPEN_H))
    return prisms, stubs, mortar


def layout(scale):
    """🗺️ Every part of the asset at one scale, in world rest coordinates (the stored pose)."""
    parts = []
    add = lambda *a: parts.append(Part(*a))
    if scale == 500:
        add("pallet", "pallet", "softwood-weathered", "pallet-timber", 0.72, [box(0, PAL_L, 0, DECK_TOP, ZC - PAL_Z, ZC + PAL_Z)])
    else:
        for k, z in enumerate((-0.35, 0.0, 0.35)):
            add(f"runner-{k + 1}", "pallet", "softwood-weathered", "pallet-timber", 0.66 + 0.02 * k, [box(0, PAL_L, 0, RUN, ZC + z - RUN / 2, ZC + z + RUN / 2)])
        if scale == 200:
            deck = Part("deck", "pallet", "softwood-weathered", "pallet-timber", 0.74, [box(0, PAL_L, RUN, DECK_TOP, ZC - PAL_Z, ZC + PAL_Z)])
            deck.sheet = True
            parts.append(deck)
        else:
            for k, x in enumerate(DECK_X):
                add(f"deck-board-{k + 1}", "pallet", "softwood-weathered", "pallet-timber", 0.72 + 0.02 * (k % 3), [box(x, x + DECK_W, RUN, DECK_TOP, ZC - PAL_Z, ZC + PAL_Z, "z")])
    for k, x in enumerate(BEARER_X):
        add(f"bearer-{k + 1}", "pallet", "softwood-weathered", "spacer-timber", 0.62, [box(x - BEARER / 2, x + BEARER / 2, DECK_TOP, STONE_BED, ZC - (0.315 if scale == 500 else 0.36), ZC + (0.315 if scale == 500 else 0.36), "z")])
    for k, x in enumerate(BEARER_X):
        add(f"spacer-a-{k + 1}", "spacers-a", "softwood-weathered", "spacer-timber", 0.6, [box(x - SPACER / 2, x + SPACER / 2, SPA_Y, SPA_Y + SPACER, *((Z_A + 0.01, Z_A + JAMB_D - 0.01) if scale == 500 else (ZC - 0.32, ZC - 0.06)), "z")])
        add(f"spacer-b-{k + 1}", "spacers-b", "softwood-weathered", "spacer-timber", 0.6, [box(x - SPACER / 2, x + SPACER / 2, SPB_Y, SPB_Y + SPACER, *((Z_B + 0.01, Z_B + JAMB_D - 0.01) if scale == 500 else (ZC + 0.07, ZC + 0.32)), "z")])

    sill = []
    if scale == 50:
        for u0, u1, uc in ((0.0, 0.30, 0.19), (1.30, SILL_L, 1.41)):
            sill.append(stone_prisms_x([(0.0, -0.05), (0.0, 0.20), (0.12, 0.20), (0.12, -0.05)], u0, u1, SILL_O, Z_A))
            sill.append(Prism("y", [(Z_A - 0.05, SILL_O[0] + u0), (Z_A + 0.20, SILL_O[0] + u0), (Z_A + 0.20, SILL_O[0] + u1), (Z_A - 0.05, SILL_O[0] + u1)], SILL_O[1] + 0.12, SILL_O[1] + SILL_H, [circle(Z_A + 0.10, SILL_O[0] + uc, 0.015)]))
        sill.append(stone_prisms_x(sill_section(50, "opening"), 0.30, 1.30, SILL_O, Z_A))
    elif scale == 500:
        sill.append(stone_prisms_x(sill_section(500, "horn"), 0.0, SILL_L, SILL_O, Z_A))
    else:
        sill.append(stone_prisms_x(sill_section(scale, "horn"), 0.0, 0.30, SILL_O, Z_A))
        sill.append(stone_prisms_x(sill_section(scale, "opening"), 0.30, 1.30, SILL_O, Z_A))
        sill.append(stone_prisms_x(sill_section(scale, "horn"), 1.30, SILL_L, SILL_O, Z_A))
    add("sill", "sill", "sandstone-weathered", "sill-stone", 0.52, sill)
    if scale == 50:
        add("sill-mortar", "sill", "lime-mortar", "sill-stone", 0.82, [box(SILL_O[0] + 0.25, SILL_O[0] + 0.75, SILL_O[1] + 0.03, SILL_O[1] + 0.10, Z_A + 0.20, Z_A + 0.212)])

    lintel = []
    if scale == 50:
        for u0, u1, uc in ((0.0, 0.30, 0.19), (1.30, SILL_L, 1.41)):
            lintel.append(Prism("y", [(Z_B, LINTEL_O[0] + u0), (Z_B + 0.20, LINTEL_O[0] + u0), (Z_B + 0.20, LINTEL_O[0] + u1), (Z_B, LINTEL_O[0] + u1)], LINTEL_O[1], LINTEL_O[1] + 0.04, [circle(Z_B + 0.10, LINTEL_O[0] + uc, 0.015)]))
        lintel.append(stone_prisms_x(lintel_section(50, "bearing-upper"), 0.0, 0.30, LINTEL_O, Z_B))
        lintel.append(stone_prisms_x(lintel_section(50, "opening"), 0.30, 1.30, LINTEL_O, Z_B))
        lintel.append(stone_prisms_x(lintel_section(50, "bearing-upper"), 1.30, 1.48, LINTEL_O, Z_B))
        lintel.append(stone_prisms_x(lintel_section(50, "spall"), 1.48, SILL_L, LINTEL_O, Z_B))
    elif scale == 100:
        lintel.append(stone_prisms_x(lintel_section(100, "bearing"), 0.0, 0.30, LINTEL_O, Z_B))
        lintel.append(stone_prisms_x(lintel_section(100, "opening"), 0.30, 1.30, LINTEL_O, Z_B))
        lintel.append(stone_prisms_x(lintel_section(100, "bearing"), 1.30, 1.48, LINTEL_O, Z_B))
        sp = [(0.0, 0.0), (0.0, 0.20), (0.26, 0.20), (0.30, 0.16), (0.30, -0.04), (0.24, -0.04), (0.24, 0.0)]
        lintel.append(stone_prisms_x(sp, 1.48, SILL_L, LINTEL_O, Z_B))
    elif scale == 200:
        lintel.append(stone_prisms_x(lintel_section(200, "bearing"), 0.0, 0.30, LINTEL_O, Z_B))
        lintel.append(stone_prisms_x(lintel_section(200, "opening"), 0.30, 1.30, LINTEL_O, Z_B))
        lintel.append(stone_prisms_x(lintel_section(200, "bearing"), 1.30, SILL_L, LINTEL_O, Z_B))
    else:
        lintel.append(stone_prisms_x(lintel_section(500, "bearing"), 0.0, SILL_L, LINTEL_O, Z_B))
    add("lintel", "lintel", "sandstone-weathered", "lintel-stone", 0.5, lintel)
    if scale == 50:
        add("lintel-mortar", "lintel", "lime-mortar", "lintel-stone", 0.82, [box(LINTEL_O[0] + 0.55, LINTEL_O[0] + 1.15, LINTEL_O[1] + 0.08, LINTEL_O[1] + 0.22, Z_B + 0.20, Z_B + 0.212)])

    for pid, bone, origin, zp, numeral in (("jamb-left", "jamb-left", JL_O, Z_A, "II"), ("jamb-right", "jamb-right", JR_O, Z_B, "III")):
        prisms, stubs, mortar = jamb_part(scale, pid, origin, zp, numeral)
        add(pid, bone, "sandstone-weathered", "jamb-stones", 0.54, prisms)
        for k, s in enumerate(stubs):
            add(f"{pid}-pintle-{k + 1}", bone, "wrought-iron", "jamb-stones", 0.25, [s])
        for m in mortar:
            add(f"{pid}-mortar", bone, "lime-mortar", "jamb-stones", 0.82, [m])

    parts.extend(fork_parts(scale))
    return parts


FORK_X0 = TINE_X[0]
FORK_Z0 = ZC + PAL_Z


def fork_parts(scale):
    """🏗️ The parked crane fork set down on the yard ground with its tine tips against the pallet, as parts in world rest coordinates (u from the left tine centre, w from the tine tips towards the mast)."""
    w_eye, u_eye = FORK_EYE_W, FORK_EYE_U
    gx = lambda u: FORK_X0 + u
    gz = lambda w: FORK_Z0 + w
    parts = []
    tt = TINE_T if scale != 500 else 0.075
    for k, (u0, u1) in enumerate(((-0.05, 0.05), (0.55, 0.65))):
        if scale in (50, 100):
            prof = [(0.0, gz(0.0)), (0.0, gz(TINE_L)), (tt, gz(TINE_L)), (tt, gz(0.10)), (0.015, gz(0.0))]
        else:
            prof = [(0.0, gz(0.0)), (0.0, gz(TINE_L)), (tt, gz(TINE_L)), (tt, gz(0.0))]
        parts.append(Part(f"tine-{k + 1}", "fork", "steel-painted", "crane-fork", 0.36, [Prism("x", prof, gx(u0), gx(u1))]))
    c = (gz(TINE_L), gz(TINE_L + 0.12))
    if scale == 50:
        car = [Prism("x", [(0.0, c[0]), (0.15, c[0]), (0.15, c[1]), (0.0, c[1])], gx(-0.042), gx(0.642), [[(0.008, c[0] + 0.008), (0.008, c[1] - 0.008), (0.142, c[1] - 0.008), (0.142, c[0] + 0.008)]]),
               box(gx(-0.05), gx(-0.042), 0.0, 0.15, c[0], c[1]), box(gx(0.642), gx(0.65), 0.0, 0.15, c[0], c[1])]
    else:
        car = [box(gx(-0.05), gx(0.65), 0.0, 0.15, c[0], c[1])]
    parts.append(Part("carriage", "fork", "steel-painted", "crane-fork", 0.32, car))
    m = (gx(0.24), gx(0.36), gz(TINE_L), gz(TINE_L + 0.12))
    if scale == 50:
        mast = Prism("y", [(m[2], m[0]), (m[3], m[0]), (m[3], m[1]), (m[2], m[1])], 0.15, 0.98, [[(m[2] + 0.008, m[0] + 0.008), (m[2] + 0.008, m[1] - 0.008), (m[3] - 0.008, m[1] - 0.008), (m[3] - 0.008, m[0] + 0.008)]], fill2d=True)
    else:
        mast = box(m[0], m[1], 0.15, 0.98, m[2], m[3], "y")
    parts.append(Part("mast", "fork", "steel-painted", "crane-fork", 0.34, [mast]))
    a0, a1 = gz(w_eye - 0.06), gz(TINE_L + 0.12)
    if scale == 50:
        arm = [Prism("z", [(m[0], 0.98), (m[1], 0.98), (m[1], 1.10), (m[0], 1.10)], a0 + 0.008, a1 - 0.008, [[(m[0] + 0.008, 0.988), (m[0] + 0.008, 1.092), (m[1] - 0.008, 1.092), (m[1] - 0.008, 0.988)]], fill2d=True),
               box(m[0], m[1], 0.98, 1.10, a0, a0 + 0.008, "z"), box(m[0], m[1], 0.98, 1.10, a1 - 0.008, a1, "z")]
    else:
        arm = [box(m[0], m[1], 0.98, 1.10, a0, a1, "z")]
    parts.append(Part("arm", "fork", "steel-painted", "crane-fork", 0.34, arm))
    e = gz(w_eye)
    half = {50: 0.01, 100: 0.01, 200: 0.015, 500: 0.0375}[scale]
    if scale in (50, 100):
        holes = [circle(1.15, e, 0.025)] if scale == 50 else []
        eye = Prism("x", [(1.10, e - 0.05), (1.10, e + 0.05), (1.20, e + 0.05), (1.20, e - 0.05)], gx(u_eye - half), gx(u_eye + half), holes)
    else:
        eye = box(gx(u_eye - half), gx(u_eye + half), 1.10, 1.20, e - 0.05, e + 0.05)
    parts.append(Part("eye", "fork", "steel-painted", "crane-fork", 0.3, [eye]))
    if scale == 500:
        return [Part("fork", "fork", "steel-painted", "crane-fork", 0.34, [q for part in parts for q in part.prisms])]
    return parts


def com_of(parts, bones):
    """⚖️ Mass and centre of mass of the parts on the given bones (world rest)."""
    rho = {"sandstone-weathered": RHO_STONE, "softwood-weathered": RHO_TIMBER, "steel-painted": RHO_STEEL, "lime-mortar": RHO_MORTAR, "wrought-iron": 7700.0}
    m = 0.0
    s = [0.0, 0.0, 0.0]
    for p in parts:
        if p.bone not in bones:
            continue
        for q in p.prisms:
            mq = q.volume() * rho[p.material]
            c = q.centroid()
            m += mq
            for i in range(3):
                s[i] += mq * c[i]
    return m, tuple(v / m for v in s) if m else (0, 0, 0)


FORK_EYE_W = 0.5
FORK_EYE_U = 0.30
for _ in range(30):
    probe = layout(50)
    pk_m, pk_c = com_of(probe, {"sill", "lintel", "jamb-left", "jamb-right", "spacers-a", "spacers-b"})
    fk_m, fk_c = com_of(probe, {"fork"})
    z = (pk_m * pk_c[2] + fk_m * (fk_c[2] + INSERT)) / (pk_m + fk_m)
    x = (pk_m * pk_c[0] + fk_m * fk_c[0]) / (pk_m + fk_m)
    FORK_EYE_W = z - (FORK_Z0 + INSERT)
    FORK_EYE_U = x - FORK_X0

PARTS = {s: layout(s) for s in SCALES}
MASS = {}
COM = {}
for bone in ("pallet", "spacers-a", "spacers-b", "sill", "lintel", "jamb-left", "jamb-right", "fork"):
    MASS[bone], COM[bone] = com_of(PARTS[50], {bone})

BONE_W = {"sill": (SILL_O[0], SILL_O[1], Z_A), "lintel": (LINTEL_O[0], LINTEL_O[1], Z_B), "jamb-left": (JL_O[0], JL_O[1], Z_A), "jamb-right": (JR_O[0], JR_O[1], Z_B)}
for b in ("sill", "lintel", "jamb-left", "jamb-right"):
    BONE_W[b] = (BONE_W[b][0], BONE_W[b][1], COM[b][2])
BONE_W["pallet"] = (0.0, 0.0, ZC)
BONE_W["yard"] = (0.0, 0.0, 0.0)
BONE_W["spacers-a"] = (BEARER_X[0], SPA_Y, COM["spacers-a"][2])
BONE_W["spacers-b"] = (BEARER_X[0], SPB_Y, COM["spacers-b"][2])
BONE_W["fork"] = (FORK_X0, 0.0, COM["fork"][2])
BONE_W["hoist"] = (0.9, STONE_BED, ZC)
BONE_W["fork-lift"] = (FORK_X0, 0.0, COM["fork"][2])
BONE_W["fork-lane"] = BONE_W["fork-lift"]
BONE_W["row-b-lane"] = BONE_W["hoist"]
BONE_W["row-b"] = BONE_W["hoist"]
BONE_W["jamb-left-carrier"] = BONE_W["jamb-left"]
BONE_W["jamb-right-carrier"] = BONE_W["jamb-right"]

FRAME = {"sill": (X_F, 0.0), "jamb-left": (X_F + HORN, FRAME_H - LINTEL_H), "lintel": (X_F, FRAME_H - LINTEL_H), "jamb-right": (X_F + SILL_L - HORN, SILL_H)}
TRAVEL = {b: (FRAME[b][0] - BONE_W[b][0], FRAME[b][1] - BONE_W[b][1]) for b in FRAME}
TRAVEL["spacers-a"] = (DROP_X, DECK_TOP - SPA_Y)
TRAVEL["spacers-b"] = (DROP_X, DECK_TOP - SPB_Y)
ROW_B = Z_A - Z_B


def prismatic(bone, travel):
    """↗️ A prismatic joint along the stored-to-reinstalled travel of a bone, with the exact offset vector."""
    dx, dy = travel
    return {"type": "prismatic", "axisDeg": math.degrees(math.atan2(dy, dx)), "minM": 0, "maxM": R(math.hypot(dx, dy) + 1e-6, 6)}


def section_props(poly_vw):
    """📏️ Area, second moment about the bone's lateral axis w and extreme fibre of a section given in (v, w)."""
    p = Polygon(poly_vw)
    a = p.area
    vc = p.centroid.x
    xs = list(p.exterior.coords)
    i = 0.0
    for (v0, w0), (v1, w1) in zip(xs, xs[1:]):
        cr = w0 * v1 - w1 * v0
        i += (v0 * v0 + v0 * v1 + v1 * v1) * cr
    i = abs(i) / 12.0 - a * vc * vc
    c = max(abs(min(v for v, _ in xs) - vc), abs(max(v for v, _ in xs) - vc))
    return a, abs(i), c, vc


def bone_sections():
    """📐️ True sections of the members: sill and lintel in their opening segment, the jambs, the three pallet runners and the fork carriage."""
    out = {}
    sv = [(t, z) for t, z in sill_section(50, "opening")]
    out["sill"] = section_props(sv)
    out["lintel"] = section_props([(t, z) for t, z in lintel_section(50, "opening")])
    out["jamb"] = section_props([(JAMB_W - s, z) for s, z in jamb_section(50)])
    out["pallet"] = (3 * RUN * RUN, 3 * RUN ** 4 / 12, RUN / 2, RUN / 2)
    a = 0.15 * 0.12 - 0.134 * 0.104
    i = (0.12 * 0.15 ** 3 - 0.104 * 0.134 ** 3) / 12
    out["fork"] = (a, i, 0.075, 0.075)
    return out


SECTIONS = bone_sections()

RIG_ORDER = ["yard", "pallet", "fork-lift", "fork-lane", "fork", "hoist", "spacers-a", "spacers-b", "sill", "jamb-left-carrier", "jamb-left", "row-b-lane", "row-b", "lintel", "jamb-right-carrier", "jamb-right"]
PARENT = {"yard": None, "pallet": "yard", "fork-lift": "yard", "fork-lane": "fork-lift", "fork": "fork-lane", "hoist": "pallet", "spacers-a": "hoist", "spacers-b": "hoist", "sill": "hoist", "jamb-left-carrier": "hoist", "jamb-left": "jamb-left-carrier", "row-b-lane": "hoist", "row-b": "row-b-lane", "lintel": "row-b", "jamb-right-carrier": "row-b", "jamb-right": "jamb-right-carrier"}


def local_of(bone):
    """📍️ The rest placement of a bone in its parent frame: every frame is world-aligned except the lateral lanes, which are the parent's own origin."""
    parent = PARENT[bone]
    if parent is None:
        return (0.0, 0.0, 0.0)
    if bone in ("fork-lane", "row-b-lane", "row-b", "fork"):
        return (0.0, 0.0, 0.0)
    pw = BONE_W[parent]
    w = BONE_W[bone]
    return (w[0] - pw[0], w[1] - pw[1], w[2] - pw[2])


def rig():
    """🦴️ The rig: pallet root, the fork chain, the hoist of the package and the travel chains of the four stones and the two spacer sets."""
    titles = {
        "yard": T("Yard floor datum on the centre plane (massless)", "Bezugspunkt Lagerboden in der Mittelebene (masselos)"),
        "pallet": T("Pallet with its two bearers (stays in the yard)", "Palette mit ihren zwei Kanthölzern (bleibt im Lager)"),
        "fork-lift": T("Crane hook travel, vertical", "Kranhakenweg, senkrecht"),
        "fork-lane": T("Crane fork lane across the pallet (massless frame)", "Gabelspur quer zur Palette (masseloser Rahmen)"),
        "fork": T("Crane fork, insertion travel", "Krangabel, Einschubweg"),
        "hoist": T("Stone package, lift travel", "Steinpaket, Hubweg"),
        "spacers-a": T("Timber spacers on the sill", "Kanthölzer auf der Sohlbank"),
        "spacers-b": T("Timber spacers on the lintel", "Kanthölzer auf dem Sturz"),
        "sill": T("Sill (stone I)", "Sohlbank (Stein I)"),
        "jamb-left-carrier": T("Left jamb travel to the frame", "Weg des linken Gewändepfostens zum Rahmen"),
        "jamb-left": T("Left jamb (stone II)", "Linker Gewändepfosten (Stein II)"),
        "row-b-lane": T("Second stack row lane (massless frame)", "Spur der zweiten Stapelreihe (masseloser Rahmen)"),
        "row-b": T("Second stack row, lateral travel into the wall plane", "Zweite Stapelreihe, seitlicher Weg in die Wandebene"),
        "lintel": T("Lintel (stone IV)", "Sturz (Stein IV)"),
        "jamb-right-carrier": T("Right jamb travel to the frame", "Weg des rechten Gewändepfostens zum Rahmen"),
        "jamb-right": T("Right jamb (stone III)", "Rechter Gewändepfosten (Stein III)"),
    }
    lengths = {"pallet": PAL_L, "fork": TINE_X[1] - TINE_X[0], "sill": SILL_L, "lintel": SILL_L, "jamb-left": OPEN_H, "jamb-right": OPEN_H}
    bones = []
    for bid in RIG_ORDER:
        x, y, z = local_of(bid)
        b = {"id": bid, "parent": PARENT[bid], "title": titles[bid], "x": R(x), "y": R(y)}
        if abs(z) > 1e-12:
            b["z"] = R(z)
        b["angleDeg"] = 0
        if bid in ("fork-lane", "row-b-lane"):
            b["abductionDeg"] = 90
        if bid in ("fork", "row-b"):
            b["abductionDeg"] = -90
        b["lengthM"] = R(lengths.get(bid, 0.0))
        if bid in ("yard", "pallet", "fork-lane", "row-b-lane"):
            b["joint"] = {"type": "fixed"}
        elif bid == "fork-lift":
            b["joint"] = {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.8}
        elif bid == "hoist":
            b["joint"] = {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.8}
        elif bid == "fork":
            b["joint"] = {"type": "prismatic", "axisDeg": 0, "minM": INSERT, "maxM": 0}
        elif bid == "row-b":
            b["joint"] = {"type": "prismatic", "axisDeg": 0, "minM": R(ROW_B), "maxM": 0}
        elif bid in ("sill", "lintel", "spacers-a", "spacers-b"):
            b["joint"] = prismatic(bid, TRAVEL[bid])
        elif bid == "jamb-left-carrier":
            b["joint"] = prismatic(bid, TRAVEL["jamb-left"])
        elif bid == "jamb-right-carrier":
            b["joint"] = prismatic(bid, TRAVEL["jamb-right"])
        elif bid == "jamb-left":
            b["joint"] = {"type": "revolute", "minDeg": -90, "maxDeg": 0}
        elif bid == "jamb-right":
            b["joint"] = {"type": "revolute", "minDeg": 0, "maxDeg": 90}
        if bid in MASS:
            c = COM[bid]
            o = BONE_W[bid]
            ref = {"pallet": "en-13698-1", "fork": "en-10219-2", "spacers-a": "en-338", "spacers-b": "en-338"}.get(bid, "grimm-1990")
            src = "authored-estimate" if bid in ("pallet", "fork", "spacers-a", "spacers-b") else "literature"
            uniform = bid in ("pallet", "sill", "lintel", "jamb-left", "jamb-right")
            b["mass"] = {"kg": R(MASS[bid], 3), "comX": R(c[0] - o[0] if not uniform else lengths[bid] / 2, 4), "comY": R(c[1] - o[1], 4), "distribution": "uniform" if uniform else "point", "source": src, "reference": ref}
        sec = {"sill": "sill", "lintel": "lintel", "jamb-left": "jamb", "jamb-right": "jamb", "pallet": "pallet", "fork": "fork"}.get(bid)
        if sec:
            a, i, c, _ = SECTIONS[sec]
            mat = {"pallet": "softwood-weathered", "fork": "steel-painted"}.get(bid, "sandstone-weathered")
            st = {"sill": T("Sill section in the opening, 160 mm with Wasserschlag and upstand", "Sohlbankquerschnitt in der Öffnung, 160 mm mit Wasserschlag und Anschlag"),
                  "lintel": T("Lintel section in the opening, 300 mm with crown band and head rebate", "Sturzquerschnitt in der Öffnung, 300 mm mit Verdachungsband und Anschlag"),
                  "jamb": T("Jamb section 220 x 200 mm with window rebate", "Gewändequerschnitt 220 x 200 mm mit Fensteranschlag"),
                  "pallet": T("Three runners 100 x 100 mm", "Drei Kufen 100 x 100 mm"),
                  "fork": T("Fork carriage RHS 150 x 120 x 8 mm", "Gabelträger Rechteckrohr 150 x 120 x 8 mm")}[sec]
            b["section"] = {"areaM2": R(a, 6), "secondMomentM4": R(i, 9), "extremeFibreM": R(c, 4), "material": mat, "title": st}
        bones.append(b)
    return {"kind": "mechanical", "root": "yard", "bones": bones}


def bone_local(bone, p):
    """🧭️ A world rest point in the frame of a bone (all frames carrying parts are world-aligned)."""
    o = BONE_W[bone]
    return (p[0] - o[0], p[1] - o[1], p[2] - o[2])


def anchors():
    """🪝️ Ground contacts of the pallet, the suspension eye of the crane fork and the two mortar beds of the reinstalled sill."""
    out = []
    for k, z in enumerate((-0.35, 0.0, 0.35)):
        out.append({"id": f"runner-{k + 1}", "bone": "pallet", "x": PAL_L / 2, "y": 0, "z": z, "kind": "contact", "title": T(f"Pallet runner {k + 1} on the yard floor", f"Palettenkufe {k + 1} auf dem Lagerboden"), "support": "contact", "footprintLengthM": PAL_L, "footprintWidthM": RUN, "frictionCoefficient": 0.5})
    e = bone_local("fork", (FORK_X0 + FORK_EYE_U, 1.15, FORK_Z0 + FORK_EYE_W))
    out.append({"id": "crane-hook", "bone": "fork", "x": R(e[0]), "y": R(e[1]), "z": R(e[2]), "kind": "contact", "title": T("Crane hook in the suspension eye of the fork", "Kranhaken im Aufhängeauge der Gabel"), "support": "pin"})
    zc = 0.0
    for k, u in enumerate((0.15, SILL_L - 0.15)):
        out.append({"id": f"sill-bed-{'left' if k == 0 else 'right'}", "bone": "sill", "x": R(u), "y": 0, "z": R(zc), "kind": "contact", "title": T(f"Mortar bed under the {'left' if k == 0 else 'right'} sill horn", f"Mörtelbett unter dem {'linken' if k == 0 else 'rechten'} Sohlbankhorn"), "support": "contact", "footprintLengthM": 0.30, "footprintWidthM": 0.25, "frictionCoefficient": 0.6})
    return out


def load(lid, kind, title, bone, p_world, fy, ref, src="authored-estimate", fz=None):
    """🧲️ A point load at a world rest point of a bone (carried with the bone), in world newtons."""
    loc = bone_local(bone, p_world)
    out = {"id": lid, "kind": kind, "title": title, "bone": bone, "x": R(loc[0]), "y": R(loc[1]), "z": R(loc[2]), "fxN": 0, "fyN": R(fy, 3), "source": src, "reference": ref}
    return out


def pair(lid, upper, lower, p_world_upper, p_world_lower, force, what):
    """🤝️ The contact force between two parts as an action-reaction pair at rest-pose points: up on the part above, down on the part below."""
    return [
        load(f"{lid}-up", "carried", T(f"{what[0]}: support force on the part above, {kn(force)} kN", f"{what[1]}: Stützkraft auf das obere Teil, {kn(force, True)} kN"), upper, p_world_upper, force, "action-reaction"),
        load(f"{lid}-down", "carried", T(f"{what[0]}: the part above presses down, {kn(force)} kN", f"{what[1]}: das obere Teil drückt nach unten, {kn(force, True)} kN"), lower, p_world_lower, -force, "action-reaction"),
    ]


def split(weight, xc, x1, x2):
    """⚖️ The two support forces of a weight at xc between supports at x1 and x2."""
    r2 = weight * (xc - x1) / (x2 - x1)
    return weight - r2, r2


def stored_loads(on_tines=False):
    """📦️ Contact pairs of the stack in the yard: jambs on spacers, spacers on sill and lintel, sill and lintel on the bearers (or on the fork tines)."""
    out = []
    w = {b: MASS[b] * G for b in MASS}
    bx = BEARER_X
    for col, low, sp, up, zlow, zup in (("a", "sill", "spacers-a", "jamb-left", Z_A, Z_A), ("b", "lintel", "spacers-b", "jamb-right", Z_B, Z_B)):
        yu = BONE_W[up][1]
        ys = BONE_W[sp][1]
        zc = COM[up][2]
        r = split(w[up], COM[up][0], bx[0], bx[1])
        for k in range(2):
            out += pair(f"{up}-on-spacer-{k + 1}", up, sp, (bx[k], yu, zc), (bx[k], yu, zc), r[k], (f"Jamb on spacer {k + 1}", f"Pfosten auf Kantholz {k + 1}"))
        s = [r[k] + w[sp] / 2 for k in range(2)]
        for k in range(2):
            out += pair(f"{sp}-on-{low}-{k + 1}", sp, low, (bx[k], ys, COM[sp][2]), (bx[k], ys, COM[sp][2]), s[k], (f"Spacer {k + 1} on the {low}", f"Kantholz {k + 1} auf {'der Sohlbank' if low == 'sill' else 'dem Sturz'}"))
        base = split(w[low], COM[low][0], bx[0], bx[1])
        tot = [base[k] + s[k] for k in range(2)]
        if not on_tines:
            for k in range(2):
                out += pair(f"{low}-on-bearer-{k + 1}", low, "pallet", (bx[k], STONE_BED, COM[low][2]), (bx[k], STONE_BED, COM[low][2]), tot[k], (f"{'Sill' if low == 'sill' else 'Lintel'} on bearer {k + 1}", f"{'Sohlbank' if low == 'sill' else 'Sturz'} auf Kantholz {k + 1}"))
        else:
            wt = w[low] + s[0] + s[1]
            m_low = w[low] * COM[low][0] + s[0] * bx[0] + s[1] * bx[1]
            xc = m_low / wt
            t = split(wt, xc, TINE_X[0], TINE_X[1])
            for k in range(2):
                out += pair(f"{low}-on-tine-{k + 1}", low, "fork", (TINE_X[k], STONE_BED, COM[low][2]), (TINE_X[k], TINE_T, COM[low][2] - INSERT), t[k], (f"{'Sill' if low == 'sill' else 'Lintel'} on fork tine {k + 1}", f"{'Sohlbank' if low == 'sill' else 'Sturz'} auf Gabelzinke {k + 1}"))
    return out


def ground_carries(bone, extra=0.0, what=("", "")):
    """🌍️ The yard ground carrying a part that stands apart from the load path of the pose, as its ground reaction at its centre of mass."""
    c = COM[bone]
    f = MASS[bone] * G + extra
    return load(f"ground-under-{bone}", "carried", T(f"Yard ground carries the {what[0]}, {kn(f)} kN", f"Lagerboden trägt {what[1]}, {kn(f, True)} kN"), bone, c, f, "action-reaction")


def frame_now(bone):
    """🏛️ Origin and turn of a bone in the reinstalled pose: the travel of its carrier, the lateral shift of the second row and the quarter turn of the jambs."""
    o = BONE_W[bone]
    if bone in TRAVEL:
        dx, dy = TRAVEL[bone]
    else:
        dx, dy = 0.0, 0.0
    dz = ROW_B if bone in ("lintel", "jamb-right") else 0.0
    turn = {"jamb-left": -90.0, "jamb-right": 90.0}.get(bone, 0.0)
    return (o[0] + dx, o[1] + dy, o[2] + dz), turn


def local_now(bone, p):
    """🧭️ A world point of the reinstalled pose in the frame of a bone."""
    o, turn = frame_now(bone)
    c, s = math.cos(math.radians(turn)), math.sin(math.radians(turn))
    d = (p[0] - o[0], p[1] - o[1], p[2] - o[2])
    return (d[0] * c + d[1] * s, -d[0] * s + d[1] * c, d[2])


def load_at(lid, title, bone, loc, fy, ref="action-reaction"):
    """🧲️ A point load at a bone-local point, in world newtons."""
    return {"id": lid, "kind": "carried", "title": title, "bone": bone, "x": R(loc[0]), "y": R(loc[1]), "z": R(loc[2]), "fxN": 0, "fyN": R(fy, 3), "source": "authored-estimate", "reference": ref}


def kn(f, de=False):
    """🔤️ A force in kN with two decimals in the reader's number format."""
    text = f"{f / 1000:.2f}"
    return text.replace(".", ",") if de else text


def reinstalled_loads():
    """🏛️ Wall load on the lintel and the contact pairs of the assembled frame: lintel on the jambs, jambs on the sill horns; pallet, spacers and fork stand on the yard ground."""
    out = []
    w = {b: MASS[b] * G for b in MASS}
    out.append({"id": "wall-load", "kind": "imposed", "title": T(f"Masonry above the lintel, 1.20 m of 18 kN/m3 brickwork over the 0.20 m stone depth, {kn(WALL_LOAD)} kN/m", f"Mauerwerk über dem Sturz, 1,20 m Ziegelmauerwerk zu 18 kN/m3 über die Steintiefe von 0,20 m, {kn(WALL_LOAD, True)} kN/m"), "fxN": 0, "fyN": 0, "distributed": {"bone": "lintel", "fromM": 0, "toM": SILL_L, "wxNPerM": 0, "wyNPerM": -WALL_LOAD}, "source": "authored-estimate", "reference": "din-1055-1"})
    lb = (HORN + JAMB_W / 2, SILL_L - HORN - JAMB_W / 2)
    lw = split(w["lintel"], COM["lintel"][0] - BONE_W["lintel"][0], lb[0], lb[1])
    ww = WALL_LOAD * SILL_L / 2
    for k, jb in enumerate(("jamb-left", "jamb-right")):
        side = ("left", "linken") if k == 0 else ("right", "rechten")
        zj = frame_now(jb)[0][2]
        head = (X_F + lb[k], FRAME_H - LINTEL_H, zj)
        foot = (X_F + lb[k], SILL_H, zj)
        for part, f, en, de in (("self", lw[k], f"Lintel self-weight on the {side[0]} jamb", f"Sturzeigengewicht auf dem {side[1]} Pfosten"), ("wall", ww, f"Wall load through the lintel on the {side[0]} jamb", f"Wandlast über den Sturz auf den {side[1]} Pfosten")):
            out.append(load_at(f"lintel-on-{jb}-{part}-up", T(f"{en}: support force on the lintel, {kn(f)} kN", f"{de}: Stützkraft auf den Sturz, {kn(f, True)} kN"), "lintel", local_now("lintel", head), f))
            out.append(load_at(f"lintel-on-{jb}-{part}-down", T(f"{en}: the lintel presses on the jamb head, {kn(f)} kN", f"{de}: der Sturz drückt auf den Pfostenkopf, {kn(f, True)} kN"), jb, local_now(jb, head), -f))
        for part, f, en, de in (("self", lw[k] + w[jb], f"{side[0].capitalize()} jamb with its share of the lintel on the sill horn", f"{side[1].capitalize()[:-1]} Pfosten mit seinem Sturzanteil auf dem Sohlbankhorn"), ("wall", ww, f"Wall load through the {side[0]} jamb on the sill horn", f"Wandlast über den {side[1]} Pfosten auf das Sohlbankhorn")):
            out.append(load_at(f"{jb}-on-sill-{part}-up", T(f"{en}: support force on the jamb foot, {kn(f)} kN", f"{de}: Stützkraft auf den Pfostenfuß, {kn(f, True)} kN"), jb, local_now(jb, foot), f))
            out.append(load_at(f"{jb}-on-sill-{part}-down", T(f"{en}: the jamb presses on the sill, {kn(f)} kN", f"{de}: der Pfosten drückt auf die Sohlbank, {kn(f, True)} kN"), "sill", local_now("sill", foot), -f))
    for sp in ("spacers-a", "spacers-b"):
        for k, x in enumerate(BEARER_X):
            f = MASS[sp] * G / 2
            out += pair(f"{sp}-on-deck-{k + 1}", sp, "pallet", (x, BONE_W[sp][1], COM[sp][2]), (x + DROP_X, DECK_TOP, COM[sp][2]), f, (f"Loose spacer {k + 1} on the pallet deck", f"Loses Kantholz {k + 1} auf dem Palettendeck"))
    out.append(ground_carries("pallet", MASS["spacers-a"] * G + MASS["spacers-b"] * G, ("empty pallet with the loose spacers", "die leere Palette mit den losen Kanthölzern")))
    out.append(ground_carries("fork", 0.0, ("parked crane fork", "die abgestellte Krangabel")))
    return out


def poses():
    """🧍️ Stored, lifted and reinstalled: the three states of a reclaimed component, with exhaustive thermal bindings."""
    regions = ["sill-stone", "jamb-stones", "lintel-stone", "pallet-timber", "spacer-timber", "crane-fork"]
    yard = {"sill-stone": {"exposedFraction": 0.85, "sunlitFraction": 0.35}, "jamb-stones": {"exposedFraction": 0.9, "sunlitFraction": 0.45}, "lintel-stone": {"exposedFraction": 0.85, "sunlitFraction": 0.4}, "pallet-timber": {"exposedFraction": 0.8, "sunlitFraction": 0.25}, "spacer-timber": {"exposedFraction": 0.7, "sunlitFraction": 0.1}, "crane-fork": {"exposedFraction": 1, "sunlitFraction": 0.5}}
    lifted = dict(yard)
    lifted.update({"sill-stone": {"exposedFraction": 0.95, "sunlitFraction": 0.35}, "lintel-stone": {"exposedFraction": 0.95, "sunlitFraction": 0.4}, "pallet-timber": {"exposedFraction": 0.9, "sunlitFraction": 0.6}})
    built = dict(yard)
    built.update({"sill-stone": {"exposedFraction": 0.45, "sunlitFraction": 0.45, "contact": {"fraction": 0.55, "tempC": 14}}, "jamb-stones": {"exposedFraction": 0.4, "sunlitFraction": 0.35, "contact": {"fraction": 0.6, "tempC": 14}}, "lintel-stone": {"exposedFraction": 0.35, "sunlitFraction": 0.4, "contact": {"fraction": 0.65, "tempC": 14}}, "pallet-timber": {"exposedFraction": 0.85, "sunlitFraction": 0.55}, "spacer-timber": {"exposedFraction": 0.9, "sunlitFraction": 0.5}})
    stored = {"id": "stored", "title": T("Stored on the pallet", "Auf der Palette gelagert"), "description": T("The four numbered stones stacked in two rows on a long pallet: sill and lintel on two bearers, the jambs on their outer side faces on spacers above them; the crane fork set down against the pallet.", "Die vier nummerierten Steine in zwei Reihen auf einer Langpalette gestapelt: Sohlbank und Sturz auf zwei Kanthölzern, die Pfosten auf ihren äußeren Seitenflächen auf Kanthölzern darüber; die Krangabel an der Palette abgestellt."), "bones": {}, "ground": "authored", "contacts": ["runner-1", "runner-2", "runner-3"], "thermal": {"regions": yard, "emitters": {}}, "loads": stored_loads(False) + [ground_carries("fork", 0.0, ("parked crane fork", "die abgestellte Krangabel"))]}
    lift = {"id": "lifted", "title": T("Lifted on the crane fork", "Mit der Krangabel angehoben"), "description": T("The crane fork has run its tines through the bearer gap under sill and lintel and lifts the stack 0.40 m; pallet and bearers stay on the ground.", "Die Krangabel hat ihre Zinken durch die Kantholzlücke unter Sohlbank und Sturz geschoben und hebt den Stapel 0,40 m an; Palette und Kanthölzer bleiben am Boden."), "bones": {"fork-lift": {"dy": R(BEAR + LIFT)}, "fork": {"dx": INSERT}, "hoist": {"dy": LIFT}}, "ground": "authored", "contacts": ["crane-hook"], "thermal": {"regions": lifted, "emitters": {}}, "loads": stored_loads(True) + [ground_carries("pallet", 0.0, ("pallet with its bearers left behind", "die zurückbleibende Palette mit ihren Kanthölzern"))]}
    re_b = {"sill": {"dx": TRAVEL["sill"][0], "dy": TRAVEL["sill"][1]}, "jamb-left-carrier": {"dx": TRAVEL["jamb-left"][0], "dy": TRAVEL["jamb-left"][1]}, "jamb-left": {"angleDeg": -90}, "row-b": {"dx": R(ROW_B, 9)}, "lintel": {"dx": TRAVEL["lintel"][0], "dy": TRAVEL["lintel"][1]}, "jamb-right-carrier": {"dx": TRAVEL["jamb-right"][0], "dy": TRAVEL["jamb-right"][1]}, "jamb-right": {"angleDeg": 90}, "spacers-a": {"dx": TRAVEL["spacers-a"][0], "dy": TRAVEL["spacers-a"][1]}, "spacers-b": {"dx": TRAVEL["spacers-b"][0], "dy": TRAVEL["spacers-b"][1]}}
    rein = {"id": "reinstalled", "title": T("Reinstalled as the window surround", "Als Fenstergewände wieder eingebaut"), "description": T("The stones set again as the assembled frame on the mortar beds of the sill horns, jambs dowelled on the sill, the lintel on the jamb heads under the masonry above; the empty pallet with the loose spacers and the fork stay in the yard.", "Die Steine wieder als zusammengesetzter Rahmen auf den Mörtelbetten der Sohlbankhörner versetzt, die Pfosten auf die Sohlbank gedübelt, der Sturz auf den Pfostenköpfen unter dem Mauerwerk darüber; die leere Palette mit den losen Kanthölzern und die Gabel bleiben im Lager."), "bones": re_b, "ground": "authored", "contacts": ["sill-bed-left", "sill-bed-right"], "thermal": {"regions": built, "emitters": {}}, "loads": reinstalled_loads()}
    for p in (stored, lift, rein):
        assert set(p["thermal"]["regions"]) == set(regions)
    return [stored, lift, rein]


def clips():
    """🎞️ The lift (fork rises, slides in, takes the stack) and the reinstalled life (the wall is bricked up over the lintel course by course)."""
    lift = {"id": "lift", "title": T("Lift off the pallet", "Vom Stapel heben"), "description": T("The crane raises the parked fork to the bearer gap, runs its tines in under sill and lintel, takes the load and lifts the stack 0.40 m off the bearers.", "Der Kran hebt die abgestellte Gabel auf Höhe der Kantholzlücke, schiebt die Zinken unter Sohlbank und Sturz, nimmt die Last auf und hebt den Stapel 0,40 m von den Kanthölzern."), "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "stored", "ease": "smooth", "bones": {"fork-lift": {"dy": R(ENTRY)}}}, {"t": 5, "pose": "stored", "ease": "smooth", "bones": {"fork-lift": {"dy": R(ENTRY)}, "fork": {"dx": INSERT}}}, {"t": 6, "pose": "stored", "ease": "smooth", "bones": {"fork-lift": {"dy": R(BEAR)}, "fork": {"dx": INSERT}}}, {"t": 9, "pose": "lifted", "ease": "smooth"}, {"t": 10, "pose": "lifted", "ease": "smooth"}],
            "contacts": [{"anchor": "runner-1", "fromS": 0, "toS": 9}, {"anchor": "runner-2", "fromS": 0, "toS": 9}, {"anchor": "runner-3", "fromS": 0, "toS": 9}, {"anchor": "crane-hook", "fromS": 6, "toS": 10}],
            "phases": [{"id": "raise-fork", "fromS": 0, "toS": 2, "title": T("Fork raised to the bearer gap", "Gabel auf Höhe der Kantholzlücke")}, {"id": "insert", "fromS": 2, "toS": 5, "title": T("Tines run in under the stones", "Zinken unter die Steine eingeschoben")}, {"id": "take-load", "fromS": 5, "toS": 6, "title": T("Tines take the load", "Zinken nehmen die Last auf")}, {"id": "lift", "fromS": 6, "toS": 9, "title": T("Stack lifted", "Stapel angehoben")}, {"id": "hold", "fromS": 9, "toS": 10, "title": T("Hold", "Halten")}],
            "events": [{"t": 6, "kind": "contact-on", "title": T("Tines bear on sill and lintel", "Zinken tragen Sohlbank und Sturz"), "target": "crane-hook"}, {"t": 9, "kind": "contact-off", "title": T("Pallet left on the ground", "Palette bleibt am Boden"), "target": "runner-2"}], "tracks": []}
    wall = {"id": "wall-built-up", "title": T("Wall built up over the lintel", "Mauerwerk über dem Sturz hochgezogen"), "description": T("After the reinstallation the masonry over the lintel is laid course by course; the wall load rises from nothing to 4.32 kN/m and the frame carries it down to the sill beds.", "Nach dem Wiedereinbau wird das Mauerwerk über dem Sturz Schicht für Schicht aufgemauert; die Wandlast steigt von null auf 4,32 kN/m, und der Rahmen leitet sie zu den Sohlbankbetten ab."), "durationS": 12, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "reinstalled", "ease": "linear"}, {"t": 12, "pose": "reinstalled", "ease": "linear"}],
            "contacts": [{"anchor": "sill-bed-left", "fromS": 0, "toS": 12}, {"anchor": "sill-bed-right", "fromS": 0, "toS": 12}],
            "phases": [{"id": "bricklaying", "fromS": 0, "toS": 10, "title": T("Courses laid over the lintel", "Schichten über dem Sturz gemauert")}, {"id": "full", "fromS": 10, "toS": 12, "title": T("Full wall load", "Volle Wandlast")}],
            "events": [{"t": 0.5, "kind": "load-on", "title": T("First course on the lintel", "Erste Schicht auf dem Sturz"), "target": "wall-load"}, {"t": 10, "kind": "load-on", "title": T("Wall complete to the next floor", "Wand bis zur nächsten Decke fertig"), "target": "wall-load"}], "tracks": []}
    courses = [(0, 0.0)] + [(0.5 + 0.5 * k, min(1.0, (k + 1) / 16)) for k in range(19) if 0.5 + 0.5 * k <= 10] + [(12, 1.0)]
    keys = [{"t": R(t, 3), "v": R(v, 4)} for t, v in courses]
    ids = ["wall-load"] + [f"lintel-on-{j}-wall-{d}" for j in ("jamb-left", "jamb-right") for d in ("up", "down")] + [f"{j}-on-sill-wall-{d}" for j in ("jamb-left", "jamb-right") for d in ("up", "down")]
    wall["tracks"] = [{"target": f"load:{i}", "ease": "linear", "keys": keys} for i in ids]
    return [lift, wall]


def world_to_view(view, poly):
    """🔭️ Nothing to change: polygons are built directly in view coordinates."""
    return poly


def projection(prism, view):
    """📐️ The exact orthographic outline of a prism in a view: elevation (x, y), front (z, y), plan (x, z)."""
    (x0, x1), (y0, y1), (z0, z1) = prism.bounds3()
    normal = {"elevation": "z", "front": "x", "plan": "y"}[view]
    if prism.axis == normal:
        ext = list(prism.poly.exterior.coords)
        ints = [] if prism.fill2d else [list(i.coords) for i in prism.poly.interiors]
        if view == "elevation":
            return Polygon(ext, ints)
        sw = lambda ring: [(b, a) for a, b in ring]
        return Polygon(sw(ext), [sw(i) for i in ints])
    if view == "elevation":
        return Polygon([(x0, y0), (x1, y0), (x1, y1), (x0, y1)])
    if view == "front":
        return Polygon([(z0, y0), (z1, y0), (z1, y1), (z0, y1)])
    return Polygon([(x0, z0), (x1, z0), (x1, z1), (x0, z1)])


def rings_of(geom):
    """🔁️ Polygons of a union, each simplified."""
    geom = geom.buffer(0)
    polys = list(geom.geoms) if isinstance(geom, MultiPolygon) else [geom]
    return [orient(p.simplify(1e-7, preserve_topology=True), 1.0) for p in polys if p.area > 1e-10]


def ring_pts(coords, dx=0.0, dy=0.0):
    """✏️ A closed ring as rounded points without the repeated end point."""
    pts = [(R(x - dx), R(y - dy)) for x, y in list(coords)[:-1]]
    out = []
    for p in pts:
        if not out or out[-1] != p:
            out.append(p)
    return [list(p) for p in out]


def shape_geom(poly, dx=0.0, dy=0.0):
    """🖋️ A polygon as a poly shape, or as an even-odd path when it has holes."""
    if len(poly.interiors) == 0:
        return {"kind": "poly", "points": ring_pts(poly.exterior.coords, dx, dy), "closed": True}
    d = []
    for ring in [poly.exterior] + list(poly.interiors):
        pts = ring_pts(ring.coords, dx, dy)
        d.append("M " + " L ".join(f"{x} {y}" for x, y in pts) + " Z")
    return {"kind": "path", "d": " ".join(d)}


ROTATING = {"jamb-left", "jamb-right"}


def depth_key(part, view):
    """🔢️ How near a part is to the viewer of a view (larger is nearer): elevation looks from -z, front from +x, plan from above."""
    bs = [p.bounds3() for p in part.prisms]
    if view == "elevation":
        return -min(b[2][0] for b in bs)
    if view == "front":
        return max(b[0][1] for b in bs)
    return max(b[1][1] for b in bs)


def solid_shapes(part, view, base):
    """🌀️ Front and plan shapes of a turning jamb: every prism as a solid swept along the bone axis, so the views stay right when the jamb is set upright."""
    o = BONE_W[part.bone]
    groups = {}
    for p in part.prisms:
        key = tuple(tuple(x) for x in ring_pts(p.poly.exterior.coords, o[1], o[2]))
        groups.setdefault(key, []).append((p.lo - o[0], p.hi - o[0]))
    out = []
    for k, (key, ranges) in enumerate(groups.items()):
        ranges.sort()
        merged = [list(ranges[0])]
        for a, b in ranges[1:]:
            if abs(a - merged[-1][1]) < 1e-9:
                merged[-1][1] = b
            else:
                merged.append([a, b])
        for j, (a, b) in enumerate(merged):
            out.append({**base, "id": base["id"] + ("" if len(groups) == 1 and len(merged) == 1 else f"-{k + 1}{'' if len(merged) == 1 else chr(97 + j)}"), "kind": "poly", "points": [list(x) for x in key], "closed": True, "solid": {"kind": "extruded", "sweep": "u", "z0": R(a), "z1": R(b)}})
    return out


def part_shapes(parts, view, scale):
    """🖼️ One filled shape per part and view, exact projection of its prisms; elevation in bone-local metres, front and plan in rest world metres."""
    ranked = sorted(parts, key=lambda p: depth_key(p, view))
    shapes = []
    for order, part in enumerate(ranked):
        base = {"id": part.id, "bone": part.bone, "role": "body", "z": order + 1, "filled": True, "tone": part.tone, "region": part.region, "material": part.material}
        if part.material == "sandstone-weathered" and scale in (50, 100) and view == "elevation":
            base["hatch"] = "sandstone"
        if view != "elevation" and part.bone in ROTATING:
            shapes += solid_shapes(part, view, base)
            continue
        polys = rings_of(unary_union([projection(p, view) for p in part.prisms]))
        o = BONE_W[part.bone]
        dx, dy = (o[0], o[1]) if view == "elevation" else (0.0, 0.0)
        for k, poly in enumerate(polys):
            shapes.append({**base, "id": part.id if len(polys) == 1 else f"{part.id}-{k + 1}", **shape_geom(poly, dx, dy)})
    return shapes


def line(sid, bone, role, region, pts, tone=0.25, extra=None):
    """➖️ An open detail or texture stroke."""
    out = {"id": sid, "bone": bone, "role": role, "z": 500, "filled": False, "tone": tone, "region": region, "kind": "poly", "points": [[R(x), R(y)] for x, y in pts], "closed": False}
    if extra:
        out.update(extra)
    return out


def numeral(sid, bone, region, text, u0, v0, h):
    """🔢️ A painted roman stone number as texture strokes: I, V and X built from straight strokes, height h."""
    strokes = []
    x = u0
    for ch in text:
        if ch == "I":
            strokes.append([(x, v0), (x, v0 + h)])
            x += 0.3 * h
        elif ch == "V":
            strokes.append([(x, v0 + h), (x + 0.25 * h, v0), (x + 0.5 * h, v0 + h)])
            x += 0.75 * h
    return [line(f"{sid}-{k + 1}", bone, "texture", region, s, 0.2) for k, s in enumerate(strokes)]


def details(scale):
    """🔍️ The arrises, painted numbers and dowel holes drawn on top of the filled parts, per view."""
    el, fr, pl = [], [], []
    so = BONE_W["sill"]
    lo = BONE_W["lintel"]
    sz = Z_A - so[2]
    lz = Z_B - lo[2]
    if scale in (50, 100, 200):
        el.append(line("sill-wasserschlag-arris", "sill", "detail", "sill-stone", [(0.30, 0.10), (1.30, 0.10)]))
        el.append(line("sill-upstand-arris", "sill", "detail", "sill-stone", [(0.30, 0.13), (1.30, 0.13)]))
        el.append(line("sill-seat-left", "sill", "detail", "sill-stone", [(0.30, 0.10), (0.30, SILL_H)]))
        el.append(line("sill-seat-right", "sill", "detail", "sill-stone", [(1.30, 0.10), (1.30, SILL_H)]))
        el.append(line("lintel-crown-arris", "lintel", "detail", "lintel-stone", [(0.0, 0.24), (SILL_L, 0.24)]))
    if scale == 50:
        for jb, o in (("jamb-left", BONE_W["jamb-left"]), ("jamb-right", BONE_W["jamb-right"])):
            el.append(line(f"{jb}-chamfer-arris", jb, "detail", "jamb-stones", [(0.0, 0.20), (OPEN_H, 0.20)]))
        el.append(line("lintel-soffit-chamfer", "lintel", "detail", "lintel-stone", [(0.30, 0.02), (1.30, 0.02)]))
    if scale in (50, 100):
        fr.append(line("lintel-spall-edge", "lintel", "detail", "lintel-stone", [(Z_B + 0.20, lo[1] + 0.26), (Z_B + 0.16, lo[1] + 0.30)]))
    if scale in (50, 100):
        el += numeral("number-sill", "sill", "sill-stone", "I", 1.42, 0.035, 0.07)
        el += numeral("number-jamb-left", "jamb-left", "jamb-stones", "II", 0.10, 0.05, 0.07)
        el += numeral("number-jamb-right", "jamb-right", "jamb-stones", "III", 1.25, 0.10, 0.07)
        el += numeral("number-lintel", "lintel", "lintel-stone", "IV", 1.42, 0.17, 0.06)
        for jb in ("jamb-left", "jamb-right"):
            o = BONE_W[jb]
            zp = (Z_A if jb == "jamb-left" else Z_B) - o[2]
            pts = [[R(0.11 + 0.0125 * math.cos(2 * math.pi * k / 12)), R(zp + 0.10 + 0.0125 * math.sin(2 * math.pi * k / 12))] for k in range(12)]
            fr.append({"id": f"{jb}-dowel-hole", "bone": jb, "role": "detail", "z": 500, "filled": False, "tone": 0.15, "region": "jamb-stones", "kind": "poly", "points": pts, "closed": True, "solid": {"kind": "extruded", "sweep": "u", "z0": R(OPEN_H - 0.04), "z1": R(OPEN_H)}})
    return el, fr, pl


def pens(scale):
    """✒️ The pen set of a scale, as the siblings: 0.35 / 0.25 / 0.18 / 0.13 mm outlines."""
    return {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}[scale]


RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DECISIONS = {
    50: {
        "silhouette": T("Side elevation of the stored stack (36 x 26 mm on paper with the fork): the long pallet of three runners and seven deck boards, two 80 mm bearers, the sill (I) and in front of it the lying left jamb (II) on 80 mm spacers, behind them the lintel (IV) and the right jamb (III), and the parked crane fork with mast, arm and eye. Front: the true sections of the four stones side by side, the jamb rebates, the sill Wasserschlag and the lintel crown, the C of the fork. Plan: deck, stone tops, spacers and the fork from above.", "Seitenansicht des gelagerten Stapels (36 x 26 mm auf Papier mit der Gabel): die Langpalette aus drei Kufen und sieben Deckbrettern, zwei Kanthölzer 80 mm, die Sohlbank (I) und davor der liegende linke Pfosten (II) auf Kanthölzern 80 mm, dahinter Sturz (IV) und rechter Pfosten (III), und die abgestellte Krangabel mit Mast, Ausleger und Auge. Vorderansicht: die wahren Querschnitte der vier Steine nebeneinander, die Fensteranschläge der Pfosten, der Wasserschlag der Sohlbank und das Verdachungsband des Sturzes, das C der Gabel. Draufsicht: Deck, Steinoberseiten, Kanthölzer und die Gabel von oben."),
        "retained": T("Every stone with its profile: 20 mm face chamfer and 55 x 100 mm rebate of the jambs, the 15 degree Wasserschlag, 40 mm upstand, 20 mm ledge and 17 mm drip groove of the sill, the 40 mm crown band with its 12 mm drip of the lintel, the head rebate and soffit chamfer, the level seats under the jambs, the 25 mm dowel holes, the 30 mm sockets in the sill and lintel beds, two cut-off pintle stubs per jamb, 12 mm mortar remnants on the backs, the spalled horn corner of the lintel; the painted stone numbers I to IV.", "Jeder Stein mit seinem Profil: 20-mm-Fase und Anschlag 55 x 100 mm der Pfosten, Wasserschlag 15 Grad, Anschlag 40 mm, Falz 20 mm und Wassernase 17 mm der Sohlbank, Verdachungsband 40 mm mit 12-mm-Wassernase des Sturzes, Sturzanschlag und Leibungsfase, die waagrechten Auflager unter den Pfosten, die Dübellöcher 25 mm, die Dübelbohrungen 30 mm in den Lagerflächen von Sohlbank und Sturz, zwei abgeschnittene Kloben je Pfosten, 12-mm-Mörtelreste auf den Rückseiten, die abgeplatzte Hornecke des Sturzes; die aufgemalten Steinnummern I bis IV."),
        "merged": T("Each stone is one shape per view; its segments (horns, opening part, end slabs with the dowel holes) are united, the arrises between them are detail strokes.", "Jeder Stein ist je Ansicht eine Form; seine Abschnitte (Hörner, Öffnungsteil, Endscheiben mit Dübellöchern) sind vereinigt, die Kanten dazwischen sind Detailstriche."),
        "omitted": T("Tooling marks of the scharriert face, the lead in the dowel holes, nails of the pallet and the crane hook above the eye.", "Scharrierspuren der Ansichtsfläche, das Blei in den Dübellöchern, die Nägel der Palette und der Kranhaken über dem Auge."),
        "thinElements": T("Pallet deck boards 22 mm, mortar remnants 12 mm, pintle stubs 15 mm and the 8 mm walls of the fork tubes at true size; the painted numbers are 70 mm texture strokes.", "Palettendeckbretter 22 mm, Mörtelreste 12 mm, Klobenstummel 15 mm und die 8-mm-Wände der Gabelrohre in wahrer Größe; die aufgemalten Nummern sind Texturstriche von 70 mm."),
        "hatching": T("Sandstone faces carry a dot stipple (0.8 mm) in the side elevation; timber and steel are plain tones.", "Sandsteinflächen tragen in der Seitenansicht eine Punktierung (0,8 mm); Holz und Stahl sind reine Töne."),
        "styles": T("Outline 0.35 mm, arrises 0.18 mm, numbers 0.1 mm; weathered sandstone mid grey, light grey timber, dark painted steel.", "Kontur 0,35 mm, Kanten 0,18 mm, Nummern 0,1 mm; verwitterter Sandstein mittelgrau, Holz hellgrau, lackierter Stahl dunkel."),
        "overlays": T("Front and plan of the jambs are solids swept along the jamb, so the views follow the jambs when they are set upright in the reinstalled pose.", "Vorderansicht und Draufsicht der Pfosten sind entlang des Pfostens gezogene Körper, so folgen die Ansichten den Pfosten, wenn sie in der Einbaupose aufgerichtet werden."),
    },
    100: {
        "silhouette": T("The same stack at half the paper size (18 x 13 mm): pallet with runners and deck boards, bearers, the four stones with their main profile, spacers and the parked fork.", "Derselbe Stapel in halber Papiergröße (18 x 13 mm): Palette mit Kufen und Deckbrettern, Kanthölzer, die vier Steine mit ihrem Hauptprofil, Kanthölzer und die abgestellte Gabel."),
        "retained": T("Rebates of jambs and lintel, Wasserschlag, upstand and ledge of the sill, the crown band, the level seats, the spalled horn corner of 40 mm; the stone numbers and the dowel holes as strokes.", "Anschläge von Pfosten und Sturz, Wasserschlag, Anschlag und Falz der Sohlbank, das Verdachungsband, die waagrechten Auflager, die abgeplatzte Hornecke von 40 mm; die Steinnummern und die Dübellöcher als Striche."),
        "merged": T("Dowel end slabs merge into the jambs, bed sockets into the horns; the fork tubes are solid.", "Dübelendscheiben gehen in den Pfosten auf, Lagerbohrungen in den Hörnern; die Gabelrohre sind voll."),
        "omitted": T("Face chamfers, drip grooves, pintle stubs and mortar remnants (below 15 mm, 0.15 mm on paper).", "Fasen, Wassernasen, Klobenstummel und Mörtelreste (unter 15 mm, 0,15 mm auf Papier)."),
        "thinElements": T("Deck boards 22 mm and the 20 mm eye plate stay at true size.", "Deckbretter 22 mm und das 20-mm-Augblech bleiben in wahrer Größe."),
        "hatching": T("Dot stipple (0.6 mm) on the sandstone faces in the side elevation.", "Punktierung (0,6 mm) auf den Sandsteinflächen in der Seitenansicht."),
        "styles": T("Outline 0.25 mm, arrises 0.13 mm, numbers 0.1 mm.", "Kontur 0,25 mm, Kanten 0,13 mm, Nummern 0,1 mm."),
        "overlays": T("Jambs as swept solids in front and plan, as at 1:50.", "Pfosten als gezogene Körper in Vorderansicht und Draufsicht, wie bei 1:50."),
    },
    200: {
        "silhouette": T("The stack as 9 x 6.5 mm: pallet as runners under one deck, bearers, four stones with rebates and crown, spacers and the fork as tines, carriage, mast, arm and eye.", "Der Stapel als 9 x 6,5 mm: Palette als Kufen unter einem Deck, Kanthölzer, vier Steine mit Anschlägen und Verdachung, Kanthölzer und die Gabel als Zinken, Träger, Mast, Ausleger und Auge."),
        "retained": T("The 55 x 100 mm rebates, the crown band, the Wasserschlag with the upstand, the level seats as arrises.", "Die Anschläge 55 x 100 mm, das Verdachungsband, der Wasserschlag mit dem Anschlag, die waagrechten Auflager als Kanten."),
        "merged": T("The seven deck boards are one 22 mm deck; the sill ledge joins the upstand; the eye plate is 30 mm.", "Die sieben Deckbretter sind ein Deck von 22 mm; der Falz der Sohlbank geht im Anschlag auf; das Augblech ist 30 mm."),
        "omitted": T("Numbers, dowel holes, the spall and the tine tapers.", "Nummern, Dübellöcher, die Abplatzung und die Zinkenanschrägungen."),
        "thinElements": T("The deck is a declared 22 mm sheet.", "Das Deck ist ein deklariertes Blech von 22 mm."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Outline 0.18 mm, arrises 0.1 mm.", "Kontur 0,18 mm, Kanten 0,1 mm."),
        "overlays": T("Jambs as swept solids in front and plan.", "Pfosten als gezogene Körper in Vorderansicht und Draufsicht."),
    },
    500: {
        "silhouette": T("The stack as 3.6 x 2.6 mm: one pallet block, two bearers, four stone blocks on their spacers and the fork as tines, carriage, mast, arm and eye block.", "Der Stapel als 3,6 x 2,6 mm: ein Palettenblock, zwei Kanthölzer, vier Steinblöcke auf ihren Kanthölzern und die Gabel als Zinken, Träger, Mast, Ausleger und Augblock."),
        "retained": T("Overall sizes of every stone (sill 1.60 x 0.16 x 0.25 m, jambs 1.50 x 0.22 x 0.20 m, lintel 1.60 x 0.30 x 0.24 m), the 80 mm timber layers, the fork.", "Gesamtmaße jedes Steins (Sohlbank 1,60 x 0,16 x 0,25 m, Pfosten 1,50 x 0,22 x 0,20 m, Sturz 1,60 x 0,30 x 0,24 m), die Holzlagen von 80 mm, die Gabel."),
        "merged": T("Runners and deck are one 122 mm pallet block; the stone profiles are their bounding sections; the eye is a 75 mm block.", "Kufen und Deck sind ein Palettenblock von 122 mm; die Steinprofile sind ihre umschreibenden Querschnitte; das Auge ist ein 75-mm-Block."),
        "omitted": T("Rebates, crown, Wasserschlag, every trace below 75 mm, all strokes.", "Anschläge, Verdachung, Wasserschlag, jede Spur unter 75 mm, alle Striche."),
        "thinElements": T("Tines drawn at the 75 mm feature instead of 50 mm.", "Zinken mit der kleinsten Einzelheit von 75 mm statt 50 mm gezeichnet."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": T("Jambs as swept solids in front and plan.", "Pfosten als gezogene Körper in Vorderansicht und Draufsicht."),
    },
}


def representation(scale):
    """🗺️ One authored representation: pens, hatch, the projected part shapes and the details of the scale."""
    parts = [p for p in PARTS[scale] if not (scale == 500 and p.bone in ("spacers-a", "spacers-b"))]
    el = part_shapes(parts, "elevation", scale)
    fr = part_shapes(parts, "front", scale)
    pl = part_shapes(parts, "plan", scale)
    de, df, dp = details(scale)
    hatches = []
    if scale in (50, 100):
        hatches = [{"id": "sandstone", "title": T("Sandstone face, stipple", "Sandsteinfläche, Punktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8 if scale == 50 else 0.6, "penMm": 0.1}]
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": pens(scale), "hatches": hatches, "geometry": {"elevation": el + de, "front": fr + df, "plan": pl + dp}, "decisions": DECISIONS[scale]}


def solid_of(prism, bone, sid, material, tone, sheet=False):
    """🧊️ A prism as an extrude solid in the frame of its bone."""
    o = BONE_W[bone]
    if prism.axis == "z":
        plane, mp, fr, to = "xy", lambda a, b: (a - o[0], b - o[1]), prism.lo - o[2], prism.hi - o[2]
    elif prism.axis == "x":
        plane, mp, fr, to = "yz", lambda a, b: (a - o[1], b - o[2]), prism.lo - o[0], prism.hi - o[0]
    else:
        plane, mp, fr, to = "zx", lambda a, b: (a - o[2], b - o[0]), prism.lo - o[1], prism.hi - o[1]
    ring = lambda coords: [[R(v) for v in mp(a, b)] for a, b in list(coords)[:-1]]
    out = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": ring(prism.poly.exterior.coords), "plane": plane, "from": R(fr), "to": R(to)}
    holes = [ring(i.coords) for i in prism.poly.interiors]
    if holes:
        out["holes"] = holes
    if sheet:
        out["sheet"] = True
    return out


MEMBER_PARTS = {"pallet": {"runner-1", "runner-2", "runner-3", "pallet", "deck", "bearer-1", "bearer-2"} | {f"deck-board-{k}" for k in range(1, 8)}, "spacers-a": {"spacer-a-1", "spacer-a-2"}, "spacers-b": {"spacer-b-1", "spacer-b-2"}, "sill": {"sill"}, "lintel": {"lintel"}, "jamb-left": {"jamb-left"}, "jamb-right": {"jamb-right"}, "fork": {"tine-1", "tine-2", "carriage", "mast", "arm", "fork"}}


def volume(scale):
    """🧊️ The handcrafted model of a scale: body solids of every part, thermal references by region, structure members, dowel joints."""
    body, thermal, structure = [], [], []
    for part in PARTS[scale]:
        ids = []
        for k, p in enumerate(part.prisms):
            sid = part.id if len(part.prisms) == 1 else f"{part.id}-{k + 1}"
            body.append(solid_of(p, part.bone, sid, part.material, part.tone, part.sheet))
            ids.append(sid)
            thermal.append({"id": f"th-{sid}", "of": sid, "region": part.region})
        for bone, names in MEMBER_PARTS.items():
            if part.id in names:
                for sid in ids:
                    structure.append({"id": f"st-{sid}", "of": sid, "member": bone, "part": "member"})
    d = {50: 0.025, 100: 0.025, 200: 0.03, 500: 0.075}[scale]
    ln = {50: 0.08, 100: 0.08, 200: 0.08, 500: 0.15}[scale]
    for jb in ("jamb-left", "jamb-right"):
        o = BONE_W[jb]
        zp = (Z_A if jb == "jamb-left" else Z_B) - o[2]
        for end, u in (("a", 0.0), ("b", OPEN_H)):
            depth = ln / 2 if scale != 500 else ln
            u0, u1 = (0.0, depth) if u == 0 else (OPEN_H - depth, OPEN_H)
            if scale == 50:
                prof = [[R(x), R(y)] for x, y in circle(0.11, zp + 0.10, d / 2)]
            else:
                prof = [[R(0.11 - d / 2), R(zp + 0.10 - d / 2)], [R(0.11 + d / 2), R(zp + 0.10 - d / 2)], [R(0.11 + d / 2), R(zp + 0.10 + d / 2)], [R(0.11 - d / 2), R(zp + 0.10 + d / 2)]]
            structure.append({"id": f"joint-dowel-{jb}-{end}", "kind": "extrude", "bone": jb, "material": "steel-painted", "role": "body", "tone": 0.3, "profile": prof, "plane": "yz", "from": R(u0), "to": R(u1), "member": jb, "part": "joint"})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[scale]}


VOLUME_DECISIONS = {
    50: [T("Body: every stone as its true profile pushed along its length in segments where the profile changes: the sill horns level (each a 120 mm block under a 40 mm bed slab with the 30 mm dowel socket), the opening part with Wasserschlag, upstand, ledge and drip groove; the jambs with face chamfer and window rebate between two 40 mm end slabs that carry the 25 mm dowel holes, two cut-off 20 x 20 x 15 mm pintle stubs in the rebate and a 12 mm mortar remnant on the back; the lintel with crown band, drip, head rebate and soffit chamfer, its bearing bed slabs with dowel sockets and a 40 mm spall at the right horn. Pallet: three 100 x 100 mm runners, seven 22 x 145 mm deck boards, two 80 mm bearers; four 80 mm spacers; the crane fork as two tapered 100 x 50 mm tines, a 150 x 120 x 8 mm carriage tube, 120 x 120 x 8 mm mast and arm tubes and a 20 mm eye plate with a 50 mm hole.", "Körper: jeder Stein als sein wahres Profil, entlang seiner Länge in Abschnitten gezogen, wo das Profil wechselt: die Sohlbankhörner waagrecht (je ein 120-mm-Block unter einer 40-mm-Lagerscheibe mit der 30-mm-Dübelbohrung), der Öffnungsteil mit Wasserschlag, Anschlag, Falz und Wassernase; die Pfosten mit Fase und Fensteranschlag zwischen zwei 40-mm-Endscheiben mit den 25-mm-Dübellöchern, zwei abgeschnittenen Klobenstummeln 20 x 20 x 15 mm im Anschlag und einem 12-mm-Mörtelrest auf der Rückseite; der Sturz mit Verdachungsband, Wassernase, Anschlag und Leibungsfase, seinen Lagerscheiben mit Dübelbohrungen und einer 40-mm-Abplatzung am rechten Horn. Palette: drei Kufen 100 x 100 mm, sieben Deckbretter 22 x 145 mm, zwei Kanthölzer 80 mm; vier Kanthölzer 80 mm; die Krangabel als zwei angeschrägte Zinken 100 x 50 mm, ein Trägerrohr 150 x 120 x 8 mm, Mast- und Auslegerrohre 120 x 120 x 8 mm und ein 20-mm-Augblech mit 50-mm-Loch."),
         T("Thermal: every body solid referenced into its region (sill, jambs, lintel, pallet timber, spacer timber, crane fork); the mortar remnants and pintle stubs belong to the stone they sit on.", "Thermik: jeder Körper als Verweis in seiner Region (Sohlbank, Pfosten, Sturz, Palettenholz, Kantholz, Krangabel); Mörtelreste und Klobenstummel gehören zum Stein, auf dem sie sitzen."),
         T("Structure: the stones, the pallet runners and the fork tines, carriage, mast and arm are the members of their bones at their true sections; each jamb end carries its dowel joint (25 mm, as the dowel holes) where it meets sill and lintel in the frame. Supports are the contacts of each pose; parts out of the load path of a pose stand on the yard ground.", "Tragwerk: Steine, Palettenkufen und Gabelzinken, Träger, Mast und Ausleger sind die Glieder ihrer Knochen in wahren Querschnitten; jedes Pfostenende trägt seine Dübelverbindung (25 mm, wie die Dübellöcher), wo es im Rahmen auf Sohlbank und Sturz trifft. Auflager sind die Kontakte jeder Pose; Teile außerhalb des Lastpfads einer Pose stehen auf dem Lagerboden.")],
    100: [T("Body at 1:100 (15 mm): the stone profiles without face chamfers, drip grooves and the 12 mm mortar remnants; dowel end slabs and bed sockets are merged, pintle stubs left out; the rebates, the Wasserschlag with upstand and ledge, the crown band and the 40 mm spall stay. Pallet runners and boards, bearers and spacers as at 1:50; the fork tubes are solid, the eye plate has no hole.", "Körper bei 1:100 (15 mm): die Steinprofile ohne Fasen, Wassernasen und 12-mm-Mörtelreste; Dübelendscheiben und Lagerbohrungen sind zusammengefasst, Klobenstummel weggelassen; Anschläge, Wasserschlag mit Anschlag und Falz, Verdachungsband und die 40-mm-Abplatzung bleiben. Palettenkufen und Bretter, Kanthölzer wie bei 1:50; die Gabelrohre sind voll, das Augblech ohne Loch."),
          T("Thermal and structure as at 1:50 on these solids; the dowel joints keep 25 mm.", "Thermik und Tragwerk wie bei 1:50 auf diesen Körpern; die Dübelverbindungen behalten 25 mm.")],
    200: [T("Body at 1:200 (30 mm): sill as level horns and an opening part with Wasserschlag and upstand (the ledge merged), lintel with crown band and the 55 mm head rebate, jambs with the rebate; the seven deck boards are one 22 mm deck declared as a sheet over the three runners; the fork tines lose their taper, the eye plate is 30 mm.", "Körper bei 1:200 (30 mm): Sohlbank als waagrechte Hörner und ein Öffnungsteil mit Wasserschlag und Anschlag (Falz zusammengefasst), Sturz mit Verdachungsband und 55-mm-Anschlag, Pfosten mit Anschlag; die sieben Deckbretter sind ein als Blech deklariertes Deck von 22 mm über den drei Kufen; die Gabelzinken verlieren ihre Anschrägung, das Augblech ist 30 mm."),
          T("Thermal by region on these solids; structure members as before, the dowel joints at the 30 mm feature.", "Thermik je Region auf diesen Körpern; Tragglieder wie zuvor, die Dübelverbindungen mit der kleinsten Einzelheit von 30 mm.")],
    500: [T("Body at 1:500 (75 mm): the pallet one 1.80 x 0.122 x 0.80 m block, the four stones as their bounding blocks (sill 160 x 250 mm, jambs 220 x 200 mm, lintel 300 x 240 mm), bearers and spacers at their 80 mm, the fork with 75 mm tines (50 mm real), the carriage, mast and arm and a 75 mm eye block.", "Körper bei 1:500 (75 mm): die Palette ein Block 1,80 x 0,122 x 0,80 m, die vier Steine als ihre umschreibenden Blöcke (Sohlbank 160 x 250 mm, Pfosten 220 x 200 mm, Sturz 300 x 240 mm), Kanthölzer in ihren 80 mm, die Gabel mit 75-mm-Zinken (50 mm wirklich), Träger, Mast und Ausleger und ein 75-mm-Augblock."),
          T("Thermal by region; structure as the stone, pallet and fork members and 75 mm dowel joints at the jamb ends.", "Thermik je Region; Tragwerk als Stein-, Paletten- und Gabelglieder und 75-mm-Dübelverbindungen an den Pfostenenden.")],
}


def contact_area(a, b):
    """🤏️ Area of the face two axis-aligned prisms share."""
    ba, bb = a.bounds3(), b.bounds3()
    total = 0.0
    for k in range(3):
        for s in (0, 1):
            if abs(ba[k][s] - bb[k][1 - s]) > 1e-9:
                continue
            c = ba[k][s]
            fa, fb = face(a, k, c), face(b, k, c)
            if fa is not None and fb is not None:
                total += fa.intersection(fb).area
    return total


def face(p, k, c):
    """🔲️ The face of a prism on the plane `axis k = c`, in the two remaining world coordinates (ascending axis order)."""
    axes = "xyz"
    if axes[k] == p.axis:
        ext = list(p.poly.exterior.coords)
        if p.axis == "x":
            return Polygon(ext, [list(i.coords) for i in p.poly.interiors])
        if p.axis == "z":
            return Polygon(ext, [list(i.coords) for i in p.poly.interiors])
        sw = lambda r: [(b, a) for a, b in r]
        return Polygon(sw(ext), [sw(list(i.coords)) for i in p.poly.interiors])
    others = [i for i in range(3) if i != k]
    along = "xyz".index(p.axis)
    pa = {"z": (0, 1), "x": (1, 2), "y": (2, 0)}[p.axis]
    idx = pa.index(k)
    coords = list(p.poly.exterior.coords)
    segs = []
    for (a0, b0), (a1, b1) in zip(coords, coords[1:]):
        q0, q1 = (a0, b0), (a1, b1)
        if abs(q0[idx] - c) < 1e-9 and abs(q1[idx] - c) < 1e-9:
            segs.append(sorted((q0[1 - idx], q1[1 - idx])))
    if not segs:
        return None
    other_profile_axis = pa[1 - idx]
    polys = []
    for s0, s1 in segs:
        rng = {other_profile_axis: (s0, s1), along: (p.lo, p.hi)}
        o0, o1 = others
        polys.append(Polygon([(rng[o0][0], rng[o1][0]), (rng[o0][1], rng[o1][0]), (rng[o0][1], rng[o1][1]), (rng[o0][0], rng[o1][1])]))
    return unary_union(polys)


def region_areas(scale):
    """📐️ Outer area per region: the surfaces of its prisms less the faces they share with each other."""
    by = {}
    for part in PARTS[scale]:
        by.setdefault(part.region, []).extend(part.prisms)
    out = {}
    for reg, ps in by.items():
        a = sum(p.area() for p in ps)
        for i in range(len(ps)):
            for j in range(i + 1, len(ps)):
                a -= 2 * contact_area(ps[i], ps[j])
        out[reg] = a
    return out


AREAS = region_areas(50)
AREAS["crane-fork"] = region_areas(100)["crane-fork"]


def thermal():
    """🌡️ Six passive regions with weathered-surface data: the three stone groups, pallet timber, spacer timber and the painted steel fork."""
    reg = lambda rid, title, kind, mat, geom, lc, svf, hc, ref, src="authored-estimate": {"id": rid, "title": title, "kind": kind, "model": "passive", "material": mat, "areaM2": R(AREAS[rid], 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [], "heatCapacityJM2K": hc, "source": src, "reference": ref}
    regions = [
        reg("sill-stone", T("Sill (stone I), weathered sandstone with mortar remnants", "Sohlbank (Stein I), verwitterter Sandstein mit Mörtelresten"), "surface", "sandstone-weathered", "horizontal-plate-up", 0.25, 0.6, 165000, "en-iso-13786"),
        reg("jamb-stones", T("Jambs (stones II and III), weathered sandstone", "Pfosten (Steine II und III), verwitterter Sandstein"), "surface", "sandstone-weathered", "vertical-plate", 0.22, 0.5, 165000, "en-iso-13786"),
        reg("lintel-stone", T("Lintel (stone IV), weathered sandstone", "Sturz (Stein IV), verwitterter Sandstein"), "surface", "sandstone-weathered", "vertical-plate", 0.30, 0.5, 165000, "en-iso-13786"),
        reg("pallet-timber", T("Pallet runners and deck boards, weathered softwood", "Palettenkufen und Deckbretter, verwittertes Nadelholz"), "surface", "softwood-weathered", "horizontal-plate-up", 0.8, 0.4, 8300, "en-iso-13786"),
        reg("spacer-timber", T("Bearers and spacers, weathered softwood", "Kanthölzer, verwittertes Nadelholz"), "surface", "softwood-weathered", "horizontal-plate-up", 0.08, 0.3, 30000, "en-iso-13786"),
        reg("crane-fork", T("Crane fork, worn painted steel", "Krangabel, abgenutzt lackierter Stahl"), "surface", "steel-painted", "vertical-plate", 0.15, 0.5, 30000, "en-iso-10456"),
    ]
    return {"environment": {"airTempC": 18, "meanRadiantTempC": 18, "skyTempC": 6, "groundTempC": 16, "relativeHumidity": 0.6, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 600, "diffuseFraction": 0.25, "pressurePa": 101325}, "referenceEnvironment": "spring-mild-sun", "passengers": [], "operating": [], "regions": regions, "emitters": [],
            "assumptions": [
                T("Sandstone faces store heat to an effective depth of about 80 mm (half the 24 h penetration depth of 174 mm for lambda 2.3 W/mK, rho 2100 kg/m3, c 1000 J/kgK, capped by the half thickness of the 0.16 to 0.30 m stones, EN ISO 13786): 165 kJ/m2K.", "Sandsteinflächen speichern Wärme bis in eine wirksame Tiefe von etwa 80 mm (die Hälfte der 24-h-Eindringtiefe von 174 mm für Lambda 2,3 W/mK, Rho 2100 kg/m3, c 1000 J/kgK, begrenzt durch die halbe Dicke der 0,16 bis 0,30 m starken Steine, EN ISO 13786): 165 kJ/m2K."),
                T("The weathered faces carry a grey-black gypsum and soot crust and a green film on the sill: solar absorptance 0.72 against about 0.55 for fresh yellow-grey sandstone; emissivity 0.93 either way (ASHRAE, typical values).", "Die verwitterten Flächen tragen eine grauschwarze Gips- und Rußkruste und auf der Sohlbank einen grünen Belag: solarer Absorptionsgrad 0,72 gegenüber etwa 0,55 für frischen gelbgrauen Sandstein; Emissionsgrad in beiden Fällen 0,93 (ASHRAE, typische Werte)."),
                T("Weathered grey softwood absorbs 0.65 against 0.45 for planed new pallet wood; the worn oxide-red paint of the fork 0.7.", "Vergrautes Nadelholz absorbiert 0,65 gegenüber 0,45 für neues gehobeltes Palettenholz; die abgenutzte oxidrote Lackierung der Gabel 0,7."),
                T("Reinstalled, about 55 to 65 per cent of each stone surface lies in the masonry (beds, back, horns) at 14 degC; nothing is a device, so the total power is 0 W.", "Eingebaut liegen etwa 55 bis 65 Prozent jeder Steinoberfläche im Mauerwerk (Lager, Rückseite, Hörner) bei 14 °C; nichts ist ein Gerät, die Gesamtleistung ist daher 0 W."),
            ]}


def materials():
    """🧱️ Weathered sandstone, lime mortar, weathered softwood, worn painted steel and the wrought iron of the pintles."""
    return [
        {"id": "sandstone-weathered", "title": T("Elbe sandstone, weathered (soot and gypsum crust)", "Elbsandstein, verwittert (Ruß- und Gipskruste)"), "emissivity": 0.93, "solarAbsorptivity": 0.72, "densityKgM3": RHO_STONE, "conductivityWmK": 2.3, "specificHeatJKgK": 1000, "elasticModulusPa": 1.5e10, "strengthPa": 5.0e6, "source": "literature", "reference": "grimm-1990"},
        {"id": "lime-mortar", "title": T("Lime mortar remnants", "Kalkmörtelreste"), "emissivity": 0.93, "solarAbsorptivity": 0.45, "densityKgM3": RHO_MORTAR, "conductivityWmK": 0.8, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
        {"id": "softwood-weathered", "title": T("Softwood, weathered grey (pine, spruce)", "Nadelholz, vergraut (Kiefer, Fichte)"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_TIMBER, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 1.1e10, "strengthPa": 2.4e7, "source": "literature", "reference": "en-338"},
        {"id": "steel-painted", "title": T("Structural steel S355, worn oxide-red paint", "Baustahl S355, abgenutzte oxidrote Lackierung"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 2.1e11, "strengthPa": 3.55e8, "source": "literature", "reference": "en-10025-2"},
        {"id": "wrought-iron", "title": T("Wrought iron, rusted (cut-off pintles)", "Schmiedeeisen, verrostet (abgeschnittene Kloben)"), "emissivity": 0.85, "solarAbsorptivity": 0.8, "densityKgM3": 7700, "conductivityWmK": 55, "specificHeatJKgK": 460, "source": "authored-estimate", "reference": "en-iso-10456"},
    ]


REFERENCES = [
    ("grimm-1990", "Grimm, W.-D. (1990): Bildatlas wichtiger Denkmalgesteine der Bundesrepublik Deutschland. Arbeitsheft 50, Bayerisches Landesamt für Denkmalpflege, München: Elbe sandstone (Posta and Cotta types) bulk density 2.0 to 2.2 g/cm3, porosity about 15 to 25 per cent; 2100 kg/m3 used."),
    ("din-en-1936", "DIN EN 1936:2007-02 Natural stone test methods, determination of real density and apparent density and of total and open porosity (the method behind the bulk density)."),
    ("din-en-12372", "DIN EN 12372:2007-02 Natural stone test methods, determination of flexural strength under concentrated load: Elbe sandstone typically 3 to 8 MPa, 5 MPa used as strengthPa; E about 15 GPa (authored estimate)."),
    ("din-18332", "DIN 18332:2019-09 VOB Part C, natural stone works: dowelling of window surrounds with non-corroding dowels, bedding in mortar, sills bedded at the horns only."),
    ("ahnert-krause-2009", "Ahnert, R., Krause, K. H. (2009): Typische Baukonstruktionen von 1860 bis 1960, Band 1, Huss-Medien, Berlin: natural stone window surrounds of the period, jambs 18 to 25 cm, sill with Wasserschlag and drip, lintel bearing 8 to 12 cm into the masonry, rebate for the casement frame."),
    ("din-1055-1", "DIN 1055-1:2002-06 Actions on structures, densities and weights: solid clay brick masonry 18 kN/m3 (wall load 0.20 m x 1.20 m x 18 kN/m3 = 4.32 kN/m on the lintel, no arching assumed)."),
    ("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties: natural stone (sandstone) 2.3 W/mK and 1000 J/kgK, lime mortar 0.8 W/mK, steel 50 W/mK and 450 J/kgK, softwood 0.13 W/mK and 1600 J/kgK."),
    ("en-iso-13786", "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); half of it, capped by the half thickness, is the effective storage depth."),
    ("en-338", "EN 338:2016 Structural timber, strength classes: C24 mean density 420 kg/m3 (470 kg/m3 used for weathered pallet timber at about 18 per cent moisture), E0,mean 11 GPa, fm,k 24 MPa."),
    ("en-13698-1", "EN 13698-1:2003 Pallet production specification, construction specification for 800 x 1200 mm flat wooden pallets: 22 mm deck boards of 145 mm; the 1800 x 800 mm long pallet here is built the same way on three 100 x 100 mm runners."),
    ("en-13155", "EN 13155:2020 Cranes, safety, non-fixed load lifting attachments: crane forks (C-hooks) with the suspension eye over the centre of gravity of fork and load."),
    ("en-10219-2", "EN 10219-2:2019 Cold formed welded structural hollow sections, tolerances, dimensions and sectional properties: RHS 120 x 120 x 8 and 150 x 120 x 8 mm (square corners assumed for the masses)."),
    ("en-10025-2", "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy structural steels: S355, yield strength 355 MPa."),
    ("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), chapter 33: emissivity and solar absorptance of building materials, typical values for stone, weathered timber and painted steel."),
    ("action-reaction", "Newton's third law as used here: where a part rests on another the contact force is written as an equal and opposite pair of loads (up on the part above, down on the part below), and a part standing apart from the load path of a pose receives its ground reaction; the archive's rigid-body solver then cuts every member along its own supports."),
]


def summary():
    """📝️ Title, summary, tags and provenance."""
    return {
        "title": T("Reclaimed Sandstone Window Surround", "Gebrauchtes Sandsteingewände"),
        "summary": T(f"A Gründerzeit window surround of Elbe sandstone for a 1.00 x 1.50 m opening, taken out stone by stone: sill with Wasserschlag and drip, two 220 x 200 mm jambs with window rebates and cut-off pintles, lintel with crown band and drip, dowel holes, mortar remnants, a spalled horn and the painted numbers I to IV. The {sum(MASS[b] for b in ('sill', 'lintel', 'jamb-left', 'jamb-right')):.0f} kg of stone lie in two rows on a long pallet with timber spacers, are lifted off by a crane fork and are set again as the assembled frame under 4.32 kN/m of masonry.", f"Ein gründerzeitliches Fenstergewände aus Elbsandstein für eine Öffnung von 1,00 x 1,50 m, Stein für Stein ausgebaut: Sohlbank mit Wasserschlag und Wassernase, zwei Pfosten 220 x 200 mm mit Fensteranschlag und abgeschnittenen Kloben, Sturz mit Verdachungsband und Wassernase, Dübellöcher, Mörtelreste, ein abgeplatztes Horn und die aufgemalten Nummern I bis IV. Die {sum(MASS[b] for b in ('sill', 'lintel', 'jamb-left', 'jamb-right')):.0f} kg Stein liegen in zwei Reihen auf einer Langpalette mit Kanthölzern, werden mit einer Krangabel abgehoben und als zusammengesetzter Rahmen unter 4,32 kN/m Mauerwerk wieder versetzt."),
        "tags": ["component", "reclaimed", "used", "natural-stone", "sandstone", "window-surround", "sill", "lintel", "jamb", "masonry", "opening", "pallet", "crane-fork", "timber", "steel"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES],
                       "notes": T("Opening 1.00 x 1.50 m clear; frame 1.60 x 1.96 m; stone masses from the modelled volumes at 2100 kg/m3; pallet, spacers and fork from their sections and densities. Every parameter is literature or authored estimate, nothing is measured.", "Lichte Öffnung 1,00 x 1,50 m; Rahmen 1,60 x 1,96 m; Steinmassen aus den modellierten Volumina mit 2100 kg/m3; Palette, Kanthölzer und Gabel aus ihren Querschnitten und Dichten. Jeder Parameter ist Literatur oder Schätzung, nichts ist gemessen.")},
    }


def dimensions():
    """📏️ The envelope of the stored pose and the total mass."""
    xs, ys, zs = [], [], []
    for part in PARTS[50]:
        for p in part.prisms:
            b = p.bounds3()
            xs += list(b[0])
            ys += list(b[1])
            zs += list(b[2])
    return {"heightM": R(max(ys) - min(ys), 3), "widthM": R(max(xs) - min(xs), 3), "depthM": R(max(zs) - min(zs), 3), "massKg": R(sum(MASS.values()), 2)}


def structure():
    """🏗️ Quasi-static, no permanent supports: each pose names its contacts."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        T("Supports are the contacts of each pose: the three pallet runners on the yard floor when stored, the crane hook in the fork eye (a pin over the centre of mass of fork and stack) when lifted, the two mortar beds under the sill horns when reinstalled.", "Auflager sind die Kontakte jeder Pose: die drei Palettenkufen auf dem Lagerboden bei der Lagerung, der Kranhaken im Gabelauge (ein Gelenk über dem Schwerpunkt von Gabel und Stapel) beim Anheben, die zwei Mörtelbetten unter den Sohlbankhörnern nach dem Wiedereinbau."),
        T("Where a part rests on another the contact force is an action-reaction pair of loads computed by hand statics (two-point supports, lever rule), so every stone is cut as the beam it is on its own supports: on its spacers or bearers in the stack, on the fork tines when lifted, on the jamb heads (lintel) or the sill horns (jambs) when reinstalled. Parts out of the load path of a pose (the parked fork, the pallet left behind, the loose spacers) stand on the yard ground through their ground reaction.", "Wo ein Teil auf einem anderen liegt, ist die Kontaktkraft ein Paar aus Last und Gegenlast nach Handstatik (Zweipunktlagerung, Hebelgesetz), so wird jeder Stein als der Balken geschnitten, der er auf seinen eigenen Auflagern ist: auf seinen Kanthölzern im Stapel, auf den Gabelzinken beim Anheben, auf den Pfostenköpfen (Sturz) oder den Sohlbankhörnern (Pfosten) nach dem Einbau. Teile außerhalb des Lastpfads einer Pose (die abgestellte Gabel, die zurückbleibende Palette, die losen Kanthölzer) stehen über ihre Bodenreaktion auf dem Lagerboden."),
        T("Wall load 4.32 kN/m over the full 1.60 m lintel (1.20 m of brickwork over the 0.20 m stone depth, no arching); the sill is bedded at the horns only, as DIN 18332 asks, so it spans the opening free.", "Wandlast 4,32 kN/m über den ganzen 1,60 m langen Sturz (1,20 m Ziegelmauerwerk über die Steintiefe von 0,20 m, ohne Gewölbewirkung); die Sohlbank liegt nur an den Hörnern auf, wie DIN 18332 es verlangt, und überspannt die Öffnung frei."),
        T("In the lift clip the loads of stored and lifted blend while the tines take the stack (5 to 9 s), so the reactions of that transition are an interpolation, not a staged analysis.", "Im Hebeclip gehen die Lasten von gelagert und angehoben ineinander über, während die Zinken den Stapel aufnehmen (5 bis 9 s); die Reaktionen dieses Übergangs sind daher eine Interpolation, keine gestufte Berechnung."),
    ]}


def document():
    """📄️ The whole asset document."""
    s = summary()
    asset = {"id": SLUG, "category": "components", **s, "dimensions": dimensions(), "materials": materials(), "rig": rig(), "anchors": anchors(),
             "representations": {str(k): representation(k) for k in SCALES}, "symbols": [], "poses": poses(), "clips": clips(), "thermal": thermal(), "structure": structure(),
             "volumes": {str(k): volume(k) for k in SCALES}, "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
             "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], "completion": "authored"}
    order = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: asset[k] for k in order}}


def report():
    """📋️ The reference numbers the report quotes."""
    print("[DEBUG] masses kg:", {b: round(m, 2) for b, m in MASS.items()}, "total", round(sum(MASS.values()), 2))
    print("[DEBUG] com:", {b: tuple(round(v, 4) for v in c) for b, c in COM.items()})
    print("[DEBUG] sections:", {k: tuple(round(v, 7) for v in s) for k, s in SECTIONS.items()})
    print("[DEBUG] eye w:", round(FORK_EYE_W, 4), "row-b:", ROW_B, "travel:", {k: (round(v[0], 4), round(v[1], 4)) for k, v in TRAVEL.items()})
    print("[DEBUG] areas 1:50:", {k: round(v, 3) for k, v in AREAS.items()})
    for s in SCALES:
        print("[DEBUG] areas 1:%d:" % s, {k: round(v, 3) for k, v in region_areas(s).items()})
    print("[DEBUG] dimensions:", dimensions())
    for b in ("sill", "lintel", "jamb-left", "jamb-right"):
        print("[DEBUG] volume", b, round(sum(p.volume() for part in PARTS[50] if part.bone == b and part.material == "sandstone-weathered" for p in part.prisms), 5))


OUT.write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n")
report()
