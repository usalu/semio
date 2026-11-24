"""🏀️ Authoritative author script of `used-basketball-hoop`: states every hand-chosen number of the street basketball unit and writes `🔣️.json` next to it.

Every part is described once (bone, kind, bone-local numbers); the 3D solids and the filled 2D outlines of the three views are both derived from that description, so 2D and 3D agree by construction; texture and detail strokes are drawn by hand on top.

Run: python3 author-basketball-hoop.py  (prints the masses and the analytic areas the document is built on).

@see ../../brief.md
@see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
G = 9.80665
RHO_S = 7850.0
RHO_C = 2400.0

PL_W, PL_H = 0.40, 0.06
BP_W, BP_T = 0.34, 0.015
FL_W, FL_T = 0.20, 0.015
PIN = (0.12, 0.095)
KN_R, KN_W = 0.02, 0.06
LUG_Z = (0.062, 0.092)
PIN_R, PIN_W = 0.008, 0.10
NUT_XZ = 0.14
NUT_H, NUT_AF = 0.016, 0.03
POLE_D, POLE_T = 0.1143, 0.004
POLE_RO, POLE_RI = POLE_D / 2, POLE_D / 2 - POLE_T
POLE_Y0 = BP_T + PL_H + FL_T
POLE_L = 3.56
CAP_T, CAP_R = 0.005, 0.06
ARM_Y = 3.55
ARM_D, ARM_T = 0.0761, 0.0036
ARM_RO, ARM_RI = ARM_D / 2, ARM_D / 2 - ARM_T
ARM_U0, ARM_U1 = 0.05, 1.204
EP_T, EP_H, EP_W = 0.01, 0.25, 0.20
STRUT_D, STRUT_T = 0.0483, 0.0029
STRUT_P0 = (0.05, 2.75 - ARM_Y)
STRUT_P1 = (0.90, -0.035)
BOARD_X, BOARD_Y = 1.254, 2.90
BW, BH = 1.20, 0.90
PLATE_T = 0.003
OPEN = 0.33
FR = 0.04
FR_T = 0.002
MID_W = (0.06, 0.10)
RING_MOUNT_U = 3.041 - BOARD_Y
RING_BEND = -2.0
RING_IN = 0.151
RING_RI = 0.225
ROD_R = 0.009
RING_RC = RING_RI + ROD_R
RING_CU = RING_IN + RING_RI
MP_T, MP_H, MP_W = 0.008, 0.12, 0.10
GUS_W = (0.066, 0.074)
CHAIN_R = 0.0035
CHAIN_KGM = 0.18
TIERS = [(0.006, RING_RC), (0.14, 0.208), (0.27, 0.184), (0.40, 0.16)]
TORN = (4, 5)
TORN_DROP = 0.22
LIFT_U = 2.6


def t(en, de):
    """🗣️ One bilingual text."""
    return {"en": en, "de": de}


def r(x, n=5):
    """🎯️ Rounds a number for the document."""
    return round(x + 0.0, n)


def pts(points):
    """📍️ Rounds a 2D point list."""
    return [[r(a), r(b)] for a, b in points]


def tube_props(d, th):
    """⭕️ Area, second moment and extreme fibre of a circular hollow section."""
    di = d - 2 * th
    return math.pi / 4 * (d ** 2 - di ** 2), math.pi / 64 * (d ** 4 - di ** 4), d / 2


A_POLE, I_POLE, C_POLE = tube_props(POLE_D, POLE_T)
A_ARM, I_ARM, C_ARM = tube_props(ARM_D, ARM_T)
A_STRUT, I_STRUT, C_STRUT = tube_props(STRUT_D, STRUT_T)
A_FR = FR ** 2 - (FR - 2 * FR_T) ** 2
I_FR = (FR ** 4 - (FR - 2 * FR_T) ** 4) / 12
A_ROD = math.pi * ROD_R ** 2
I_ROD = math.pi * (2 * ROD_R) ** 4 / 64

BONES_REST = {}


def frame(bone):
    """🧭️ World rest origin and angle (degrees) of a bone."""
    return BONES_REST[bone]


def to_world(bone, p):
    """🌍️ Bone-local (u, v, w) to world (x, y, z) at rest."""
    (ox, oy), a = frame(bone)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    u, v, w = p
    return (ox + u * c - v * s, oy + u * s + v * c, w)


def local_from(parent, p):
    """↩️ World point (x, y) in the (u, v) frame of a bone."""
    (ox, oy), a = frame(parent)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy = p[0] - ox, p[1] - oy
    return (dx * c + dy * s, -dx * s + dy * c)


def place(bone, parent, world, angle):
    """📌️ Registers a bone at a world rest origin and world angle; returns its local placement in the parent."""
    if parent is None:
        BONES_REST[bone] = (world, angle)
        return (world[0], world[1], angle)
    u, v = local_from(parent, world)
    BONES_REST[bone] = (world, angle)
    return (u, v, angle - frame(parent)[1])


P_FOOT = place("footing", None, (0.0, 0.0), 0.0)
P_HINGE = place("hinge", "footing", PIN, 90.0)
P_POLE = place("pole", "hinge", (0.0, POLE_Y0), 90.0)
P_ARM = place("arm", "pole", (0.0, ARM_Y), 0.0)
P_BOARD = place("board", "arm", (BOARD_X, BOARD_Y), 90.0)
P_RING = place("ring", "board", (BOARD_X + PLATE_T, BOARD_Y + RING_MOUNT_U), RING_BEND)
RING_C_WORLD = to_world("ring", (RING_CU, 0, 0))
P_NET = place("net", "ring", (RING_C_WORLD[0], RING_C_WORLD[1]), -90.0)


def circle(cx, cy, rad, n=24, a0=0.0):
    """⚪️ A polygon circle."""
    return [(cx + rad * math.cos(a0 + 2 * math.pi * k / n), cy + rad * math.sin(a0 + 2 * math.pi * k / n)) for k in range(n)]


def hexagon(cx, cy, af):
    """🔩️ A hexagon of a given width across flats."""
    rc = af / math.sqrt(3)
    return [(cx + rc * math.cos(math.pi / 3 * k), cy + rc * math.sin(math.pi / 3 * k)) for k in range(6)]


def lug_profile(cx, cy, rad, base):
    """🪝️ A hinge lug: a rectangle from the base line up to the pin, closed by a half circle around it."""
    arc = [(cx + rad * math.cos(math.pi * k / 10), cy + rad * math.sin(math.pi * k / 10)) for k in range(11)]
    return [(cx + rad, base)] + arc + [(cx - rad, base)]


def basis(axis):
    """📐️ Two unit vectors perpendicular to an axis."""
    ax = [a / math.sqrt(sum(b * b for b in axis)) for a in axis]
    ref = (0, 0, 1) if abs(ax[2]) < 0.9 else (1, 0, 0)
    e1 = [ax[1] * ref[2] - ax[2] * ref[1], ax[2] * ref[0] - ax[0] * ref[2], ax[0] * ref[1] - ax[1] * ref[0]]
    n = math.sqrt(sum(b * b for b in e1))
    e1 = [b / n for b in e1]
    e2 = [ax[1] * e1[2] - ax[2] * e1[1], ax[2] * e1[0] - ax[0] * e1[2], ax[0] * e1[1] - ax[1] * e1[0]]
    return ax, e1, e2


def rot_about(p, axis, origin, ang):
    """🔄️ Rotates a point about an axis through an origin (Rodrigues)."""
    k = [a / math.sqrt(sum(b * b for b in axis)) for a in axis]
    q = [p[i] - origin[i] for i in range(3)]
    c, s = math.cos(ang), math.sin(ang)
    kxq = [k[1] * q[2] - k[2] * q[1], k[2] * q[0] - k[0] * q[2], k[0] * q[1] - k[1] * q[0]]
    kdq = sum(k[i] * q[i] for i in range(3))
    return [origin[i] + q[i] * c + kxq[i] * s + k[i] * kdq * (1 - c) for i in range(3)]


def samples(sol):
    """🎯️ Surface sample points of a solid in its bone frame (all copies of a repeat), used for the outlines."""
    k = sol["kind"]
    out = []
    if k == "box":
        su, sv, sw = sol["size"]
        cu, cv, cw = sol.get("transform", {}).get("translation", [0, 0, 0])
        out = [(cu + a * su / 2, cv + b * sv / 2, cw + c * sw / 2) for a in (-1, 1) for b in (-1, 1) for c in (-1, 1)]
    elif k == "extrude":
        for a, b in sol["profile"]:
            for tt in (sol["from"], sol["to"]):
                out.append({"xy": (a, b, tt), "yz": (tt, a, b), "zx": (b, tt, a)}[sol["plane"]])
        tr = sol.get("transform", {}).get("translation")
        if tr:
            out = [(p[0] + tr[0], p[1] + tr[1], p[2] + tr[2]) for p in out]
    elif k == "revolve":
        ax, e1, e2 = basis(sol["axis"])
        o = sol["origin"]
        for a, rad in sol["profile"]:
            for j in range(48):
                ph = 2 * math.pi * j / 48
                out.append(tuple(o[i] + a * ax[i] + rad * (math.cos(ph) * e1[i] + math.sin(ph) * e2[i]) for i in range(3)))
    elif k == "capsule":
        for p, rad in ((sol["p0"], sol["r0"]), (sol["p1"], sol["r1"])):
            for j in range(16):
                ph = 2 * math.pi * j / 16
                for (i1, i2) in ((0, 1), (1, 2), (0, 2)):
                    q = list(p)
                    q[i1] += rad * math.cos(ph)
                    q[i2] += rad * math.sin(ph)
                    out.append(tuple(q))
    rep = sol.get("repeat")
    if rep:
        base = list(out)
        out = []
        for n in range(rep["count"]):
            for p in base:
                q = list(p)
                if "rotationRad" in rep:
                    q = rot_about(q, rep["rotationAxis"], rep.get("origin", [0, 0, 0]), rep["rotationRad"] * n)
                if "translation" in rep:
                    q = [q[i] + rep["translation"][i] * n for i in range(3)]
                out.append(tuple(q))
    return out


def hull(points):
    """🧮️ Convex hull (monotone chain), counter-clockwise."""
    p = sorted(set((round(a, 6), round(b, 6)) for a, b in points))
    if len(p) <= 2:
        return p

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo, up = [], []
    for q in p:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], q) <= 1e-12:
            lo.pop()
        lo.append(q)
    for q in reversed(p):
        while len(up) >= 2 and cross(up[-2], up[-1], q) <= 1e-12:
            up.pop()
        up.append(q)
    return lo[:-1] + up[:-1]


def view_points(sol, view):
    """🔭️ The sample points of a solid projected into one view's 2D frame."""
    loc = samples(sol)
    if view == "elevation":
        return [(p[0], p[1]) for p in loc]
    wpts = [to_world(sol["bone"], p) for p in loc]
    if view == "front":
        return [(p[2], p[1]) for p in wpts]
    return [(p[0], p[2]) for p in wpts]


def outline(sol, view):
    """✏️ The convex outline of a solid in one view."""
    return hull(view_points(sol, view))


def axis_stroke(sol, view, copy=None):
    """〰️ The projected centre line of a capsule (one copy of a repeat) in one view."""
    p0, p1 = sol["p0"], sol["p1"]
    rep = sol.get("repeat")
    segs = []
    for n in range(rep["count"] if rep else 1):
        a, b = list(p0), list(p1)
        if rep:
            a = rot_about(a, rep["rotationAxis"], rep.get("origin", [0, 0, 0]), rep["rotationRad"] * n)
            b = rot_about(b, rep["rotationAxis"], rep.get("origin", [0, 0, 0]), rep["rotationRad"] * n)
        if view == "elevation":
            segs.append(((a[0], a[1]), (b[0], b[1])))
        else:
            wa, wb = to_world(sol["bone"], a), to_world(sol["bone"], b)
            segs.append(((wa[2], wa[1]), (wb[2], wb[1])) if view == "front" else ((wa[0], wa[2]), (wb[0], wb[2])))
    return segs


MATERIALS_SRC = [
    ("concrete-weathered", "Weathered concrete C25/30 (foundation head)", "Verwitterter Beton C25/30 (Fundamentkopf)", 0.92, 0.65, RHO_C, 2.0, 1000, 31e9, 25e6, "literature", "levinson-akbari-2002"),
    ("steel-galvanised-rusty", "Hot-dip galvanised steel, weathered with rust at the hinge", "Feuerverzinkter Stahl, verwittert, am Gelenk rostig", 0.55, 0.75, RHO_S, 50, 450, 210e9, 235e6, "authored-estimate", "vdi-waermeatlas-2013"),
    ("steel-powder-coated", "Galvanised steel, chalked moss-green powder coating", "Verzinkter Stahl, kreidende moosgrüne Pulverbeschichtung", 0.9, 0.84, RHO_S, 50, 450, 210e9, 235e6, "authored-estimate", "duffie-beckman-2013"),
    ("steel-perforated", "Perforated galvanised steel plate, dull zinc patina", "Verzinktes Stahllochblech, matte Zinkpatina", 0.28, 0.62, RHO_S, 50, 450, 210e9, 235e6, "literature", "vdi-waermeatlas-2013"),
    ("steel-ring-painted", "Solid steel ring rod, orange enamel worn to bare steel", "Stahlvollstab-Ring, orange Lackierung bis auf blanken Stahl abgenutzt", 0.88, 0.68, RHO_S, 50, 450, 210e9, 235e6, "authored-estimate", "duffie-beckman-2013"),
    ("chain-galvanised", "Galvanised steel short-link chain 3 mm", "Verzinkte Kurzgliederkette 3 mm", 0.3, 0.65, RHO_S, 50, 450, 210e9, 400e6, "literature", "vdi-waermeatlas-2013"),
]
MATERIALS = [{"id": i, "title": t(en, de), "emissivity": e, "solarAbsorptivity": a, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp, "elasticModulusPa": E, "strengthPa": fs, "source": src, "reference": ref}
             for i, en, de, e, a, rho, k, cp, E, fs, src, ref in MATERIALS_SRC]

REG = {"concrete-weathered": "foundation-head", "steel-galvanised-rusty": "base-hinge", "steel-powder-coated": "pole-coating", "steel-perforated": "backboard", "steel-ring-painted": "ring", "chain-galvanised": "chain-net"}
TONE = {"concrete-weathered": 0.62, "steel-galvanised-rusty": 0.36, "steel-powder-coated": 0.3, "steel-perforated": 0.66, "steel-ring-painted": 0.48, "chain-galvanised": 0.5}


def solid(i, bone, material, kind, role="body", tone=None, **geom):
    """🧊️ One body solid."""
    s = {"id": i, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}
    s.update(geom)
    return s


def box(i, bone, material, size, at, **kw):
    """📦️ A box solid centred at a bone-local point."""
    return solid(i, bone, material, "box", size=[r(x) for x in size], transform={"translation": [r(x) for x in at]}, **kw)


def box_span(i, bone, material, u, v, w, **kw):
    """📦️ A box solid given by its bone-local ranges."""
    return box(i, bone, material, (u[1] - u[0], v[1] - v[0], w[1] - w[0]), ((u[0] + u[1]) / 2, (v[0] + v[1]) / 2, (w[0] + w[1]) / 2), **kw)


def extrude(i, bone, material, profile, plane, a, b, **kw):
    """🧊️ An extrude solid."""
    return solid(i, bone, material, "extrude", profile=pts(profile), plane=plane, **{"from": r(a), "to": r(b)}, **kw)


def revolve(i, bone, material, profile, axis, origin, **kw):
    """🌀️ A revolve solid."""
    return solid(i, bone, material, "revolve", profile=pts(profile), axis=axis, origin=[r(x) for x in origin], **kw)


def capsule(i, bone, material, p0, p1, rad, **kw):
    """💊️ A capsule solid."""
    return solid(i, bone, material, "capsule", p0=[r(x) for x in p0], p1=[r(x) for x in p1], r0=rad, r1=rad, **kw)


def tube(i, bone, material, u0, u1, ro, ri, closed_top=False, **kw):
    """🔘️ A circular hollow section along the bone axis as a closed-loop revolve; `closed_top` caps it (a solid end plate of the wall thickness)."""
    if closed_top:
        prof = [(u0, ro), (u1, ro), (u1, 0.0), (u1 - (ro - ri), 0.0), (u1 - (ro - ri), ri), (u0, ri), (u0, ro)]
    else:
        prof = [(u0, ro), (u1, ro), (u1, ri), (u0, ri), (u0, ro)]
    return revolve(i, bone, material, prof, [1, 0, 0], [0, 0, 0], **kw)


def rod(i, bone, material, u0, u1, rad, **kw):
    """🥢️ A solid round bar along the bone axis."""
    return revolve(i, bone, material, [(u0, 0), (u0, rad), (u1, rad), (u1, 0)], [1, 0, 0], [0, 0, 0], **kw)


def torus(i, bone, material, centre_u, rc, rr, n=16, **kw):
    """🍩️ The ring: a closed circular loop of radius rr revolved at radius rc about the bone normal v through (centre_u, 0, 0)."""
    loop = [(rr * math.sin(2 * math.pi * k / n), rc + rr * math.cos(2 * math.pi * k / n)) for k in range(n)]
    loop.append(loop[0])
    return revolve(i, bone, material, loop, [0, 1, 0], [centre_u, 0, 0], **kw)


def net_point(tier, k, tiers):
    """🕸️ A knot of the chain net: tier index and strand index (net frame: u down, v ahead, w left)."""
    u, rad = tiers[tier]
    th = math.radians(30 * k + (15 if tier % 2 else 0))
    return (u, rad * math.cos(th), rad * math.sin(th))


def net_solids(tiers, rad, torn, prefix="chain"):
    """⛓️ The diamond chain net: every knot joined to the two nearest knots of the next tier; tier pair 2-3 is listed strand by strand so the torn sector can be left out, and the torn chain end dangles."""
    out = []
    rep = {"count": 12, "rotationAxis": [1, 0, 0], "rotationRad": r(math.pi / 6, 7), "origin": [0, 0, 0]}
    for tier in range(len(tiers) - 2):
        a = net_point(tier, 0, tiers)
        for side in (0, -1) if tier % 2 == 0 else (0, 1):
            b = net_point(tier + 1, side, tiers)
            out.append(capsule(f"{prefix}-{tier}{tier + 1}{'a' if side == 0 else 'b'}", "net", "chain-galvanised", a, b, rad, slender=True, repeat=dict(rep)))
    last = len(tiers) - 2
    for k in range(12):
        a = net_point(last, k, tiers)
        for side in (0, -1) if last % 2 == 0 else (0, 1):
            if k in torn:
                continue
            b = net_point(last + 1, k + side, tiers)
            out.append(capsule(f"{prefix}-{last}{last + 1}-{k}{'a' if side == 0 else 'b'}", "net", "chain-galvanised", a, b, rad, slender=True))
    a = net_point(last, torn[1], tiers)
    out.append(capsule(f"{prefix}-torn-end", "net", "chain-galvanised", a, (a[0] + TORN_DROP, a[1] * 0.92, a[2] * 0.92), rad, slender=True))
    return out


def chain_length(sols):
    """📏️ Total chain length of a list of capsules (with repeats)."""
    tot = 0.0
    for s in sols:
        n = s.get("repeat", {}).get("count", 1)
        tot += n * math.dist(s["p0"], s["p1"])
    return tot


def parts50():
    """🧱️ The 1:50 parts: every component of the real unit above the 7.5 mm feature."""
    p = [
        box("plinth", "footing", "concrete-weathered", (PL_W, PL_H, PL_W), (0, PL_H / 2, 0), cornerRadius=0.008),
        box("base-plate", "footing", "steel-galvanised-rusty", (BP_W, BP_T, BP_W), (0, PL_H + BP_T / 2, 0)),
        extrude("anchor-nut", "footing", "steel-galvanised-rusty", hexagon(NUT_XZ, NUT_XZ, NUT_AF), "zx", PL_H + BP_T, PL_H + BP_T + NUT_H, role="detail", repeat={"count": 2, "translation": [0, 0, -2 * NUT_XZ]}),
        extrude("anchor-nut-back", "footing", "steel-galvanised-rusty", hexagon(NUT_XZ, -NUT_XZ, NUT_AF), "zx", PL_H + BP_T, PL_H + BP_T + NUT_H, role="detail", repeat={"count": 2, "translation": [0, 0, -2 * NUT_XZ]}),
        extrude("hinge-lug", "footing", "steel-galvanised-rusty", lug_profile(PIN[0], PIN[1], KN_R, PL_H + BP_T), "xy", LUG_Z[0], LUG_Z[1], role="detail", repeat={"count": 2, "translation": [0, 0, -(LUG_Z[0] + LUG_Z[1])]}),
        extrude("hinge-pin", "hinge", "steel-galvanised-rusty", circle(0, 0, PIN_R, 16), "xy", -PIN_W, PIN_W, role="detail"),
        box_span("foot-flange", "pole", "steel-galvanised-rusty", (-FL_T, 0), (-FL_W / 2, FL_W / 2), (-FL_W / 2, FL_W / 2)),
        extrude("hinge-knuckle", "pole", "steel-galvanised-rusty", circle(PIN[1] - POLE_Y0, -PIN[0], KN_R, 20), "xy", -KN_W, KN_W, role="detail"),
        extrude("foot-rib", "pole", "steel-galvanised-rusty", [(0, POLE_RO - 0.001), (0, FL_W / 2), (0.012, FL_W / 2), (0.08, POLE_RO - 0.001)], "xy", -0.004, 0.004, role="detail", repeat={"count": 4, "rotationAxis": [1, 0, 0], "rotationRad": r(math.pi / 2, 7), "origin": [0, 0, 0]}),
        tube("pole-tube", "pole", "steel-powder-coated", 0, POLE_L, POLE_RO, POLE_RI),
        rod("pole-cap", "pole", "steel-powder-coated", POLE_L, POLE_L + CAP_T, CAP_R, sheet=True),
        tube("arm-tube", "arm", "steel-powder-coated", ARM_U0, ARM_U1, ARM_RO, ARM_RI),
        box_span("arm-end-plate", "arm", "steel-powder-coated", (ARM_U1, ARM_U1 + EP_T), (-EP_H / 2, EP_H / 2), (-EP_W / 2, EP_W / 2), sheet=True),
        capsule("arm-strut", "arm", "steel-powder-coated", (STRUT_P0[0], STRUT_P0[1], 0), (STRUT_P1[0], STRUT_P1[1], 0), STRUT_D / 2),
        extrude("backboard-plate", "board", "steel-perforated", [(-PLATE_T, -BW / 2), (0, -BW / 2), (0, BW / 2), (-PLATE_T, BW / 2)], "yz", 0, BH, sheet=True),
        box_span("frame-bottom", "board", "steel-powder-coated", (0, FR), (0, FR), (-BW / 2, BW / 2)),
        box_span("frame-top", "board", "steel-powder-coated", (BH - FR, BH), (0, FR), (-BW / 2, BW / 2)),
        box_span("frame-side", "board", "steel-powder-coated", (FR, BH - FR), (0, FR), (BW / 2 - FR, BW / 2), repeat={"count": 2, "translation": [0, 0, -(BW - FR)]}),
        box_span("frame-mid", "board", "steel-powder-coated", (FR, BH - FR), (0, FR), MID_W, repeat={"count": 2, "translation": [0, 0, -(MID_W[0] + MID_W[1])]}),
        box_span("ring-mount", "ring", "steel-ring-painted", (0, MP_T), (-MP_H / 2, MP_H / 2), (-MP_W, MP_W)),
        extrude("ring-gusset", "ring", "steel-ring-painted", [(MP_T, -0.07), (MP_T, -0.004), (RING_IN + 0.02, -0.004), (RING_IN + 0.02, -0.016)], "xy", GUS_W[0], GUS_W[1], repeat={"count": 2, "translation": [0, 0, -(GUS_W[0] + GUS_W[1])]}),
        torus("ring-rod", "ring", "steel-ring-painted", RING_CU, RING_RC, ROD_R, slender=True),
    ]
    return p + net_solids(TIERS, CHAIN_R, TORN)


def parts100():
    """🧱️ The 1:100 parts, authored for the 15 mm feature: nuts, pin, ribs, cap and gusset pair merged or left out."""
    p = [
        box("plinth", "footing", "concrete-weathered", (PL_W, PL_H, PL_W), (0, PL_H / 2, 0)),
        box("base-plate", "footing", "steel-galvanised-rusty", (BP_W, BP_T, BP_W), (0, PL_H + BP_T / 2, 0)),
        extrude("hinge-lug", "footing", "steel-galvanised-rusty", lug_profile(PIN[0], PIN[1], KN_R, PL_H + BP_T), "xy", KN_W, KN_W + 0.03, role="detail", repeat={"count": 2, "translation": [0, 0, -(2 * KN_W + 0.03)]}),
        box_span("foot-flange", "pole", "steel-galvanised-rusty", (-FL_T, 0), (-FL_W / 2, FL_W / 2), (-FL_W / 2, FL_W / 2)),
        extrude("hinge-knuckle", "pole", "steel-galvanised-rusty", circle(PIN[1] - POLE_Y0, -PIN[0], KN_R, 16), "xy", -KN_W, KN_W, role="detail"),
        tube("pole-tube", "pole", "steel-powder-coated", 0, POLE_L + CAP_T, POLE_RO, POLE_RI, closed_top=True),
        tube("arm-tube", "arm", "steel-powder-coated", ARM_U0, ARM_U1, ARM_RO, ARM_RI),
        box_span("arm-end-plate", "arm", "steel-powder-coated", (ARM_U1, ARM_U1 + EP_T), (-EP_H / 2, EP_H / 2), (-EP_W / 2, EP_W / 2), sheet=True),
        capsule("arm-strut", "arm", "steel-powder-coated", (STRUT_P0[0], STRUT_P0[1], 0), (STRUT_P1[0], STRUT_P1[1], 0), STRUT_D / 2),
        extrude("backboard-plate", "board", "steel-perforated", [(-PLATE_T, -BW / 2), (0, -BW / 2), (0, BW / 2), (-PLATE_T, BW / 2)], "yz", 0, BH, sheet=True),
        box_span("frame-bottom", "board", "steel-powder-coated", (0, FR), (0, FR), (-BW / 2, BW / 2)),
        box_span("frame-top", "board", "steel-powder-coated", (BH - FR, BH), (0, FR), (-BW / 2, BW / 2)),
        box_span("frame-side", "board", "steel-powder-coated", (FR, BH - FR), (0, FR), (BW / 2 - FR, BW / 2), repeat={"count": 2, "translation": [0, 0, -(BW - FR)]}),
        box_span("frame-mid", "board", "steel-powder-coated", (FR, BH - FR), (0, FR), MID_W, repeat={"count": 2, "translation": [0, 0, -(MID_W[0] + MID_W[1])]}),
        extrude("ring-bracket", "ring", "steel-ring-painted", [(0, -0.07), (0, 0.05), (0.016, 0.05), (0.016, -0.004), (RING_IN + 0.02, -0.004), (RING_IN + 0.02, -0.02)], "xy", -0.08, 0.08),
        torus("ring-rod", "ring", "steel-ring-painted", RING_CU, RING_RC, ROD_R, n=12, slender=True),
    ]
    return p + net_solids(TIERS, CHAIN_R, TORN)


def strands(tiers, rad, torn_k, prefix="strand"):
    """⛓️ The net generalised to twelve straight strands from the ring to the lower rim, one of them torn short."""
    top, bot = tiers[0], tiers[-1]
    out = []
    keep = [k for k in range(12) if k != torn_k]
    for k in keep:
        th = math.radians(30 * k)
        out.append(capsule(f"{prefix}-{k}", "net", "chain-galvanised", (top[0], top[1] * math.cos(th), top[1] * math.sin(th)), (bot[0], bot[1] * math.cos(th), bot[1] * math.sin(th)), rad, slender=True))
    th = math.radians(30 * torn_k)
    mid = tiers[2]
    out.append(capsule(f"{prefix}-torn", "net", "chain-galvanised", (top[0], top[1] * math.cos(th), top[1] * math.sin(th)), (mid[0] + TORN_DROP, mid[1] * 0.92 * math.cos(th), mid[1] * 0.92 * math.sin(th)), rad, slender=True))
    return out


def parts200():
    """🧱️ The 1:200 parts, authored for the 30 mm feature."""
    return [
        box("footing-block", "footing", "concrete-weathered", (PL_W, PL_H + BP_T, PL_W), (0, (PL_H + BP_T) / 2, 0)),
        box_span("pole-foot", "pole", "steel-galvanised-rusty", (-FL_T, FL_T), (-FL_W / 2 - 0.04, FL_W / 2), (-FL_W / 2, FL_W / 2)),
        rod("pole-tube", "pole", "steel-powder-coated", FL_T, POLE_L + CAP_T, POLE_RO),
        rod("arm-tube", "arm", "steel-powder-coated", ARM_U0, ARM_U1 + EP_T, ARM_RO),
        capsule("arm-strut", "arm", "steel-powder-coated", (STRUT_P0[0], STRUT_P0[1], 0), (STRUT_P1[0], STRUT_P1[1], 0), STRUT_D / 2),
        extrude("backboard-plate", "board", "steel-perforated", [(-PLATE_T, -BW / 2), (0, -BW / 2), (0, BW / 2), (-PLATE_T, BW / 2)], "yz", 0, BH, sheet=True),
        box_span("board-frame", "board", "steel-powder-coated", (0, BH), (0, FR), (-BW / 2, BW / 2)),
        box_span("ring-bracket", "ring", "steel-ring-painted", (0, RING_IN + 0.02), (-0.04, 0.0), (-0.08, 0.08)),
        torus("ring-rod", "ring", "steel-ring-painted", RING_CU, RING_RC, ROD_R, n=10, slender=True),
    ] + strands(TIERS, CHAIN_R, TORN[1])


def parts500():
    """🧱️ The 1:500 parts, authored for the 75 mm feature."""
    return [
        box("footing-block", "footing", "concrete-weathered", (PL_W, PL_H + BP_T, PL_W), (0, (PL_H + BP_T) / 2, 0)),
        box_span("pole-foot", "pole", "steel-galvanised-rusty", (-FL_T, 0.06), (-FL_W / 2, FL_W / 2), (-FL_W / 2, FL_W / 2)),
        rod("pole-tube", "pole", "steel-powder-coated", 0.06, POLE_L + CAP_T, POLE_RO),
        rod("arm-tube", "arm", "steel-powder-coated", ARM_U0, ARM_U1 + EP_T, ARM_RO),
        capsule("arm-strut", "arm", "steel-powder-coated", (STRUT_P0[0], STRUT_P0[1], 0), (STRUT_P1[0], STRUT_P1[1], 0), STRUT_D / 2, slender=True),
        box_span("backboard", "board", "steel-perforated", (0, BH), (-PLATE_T, FR), (-BW / 2, BW / 2), sheet=True),
        box_span("ring-bracket", "ring", "steel-ring-painted", (0, RING_IN + 0.02), (-0.075, 0.0), (-0.0375, 0.0375)),
        torus("ring-rod", "ring", "steel-ring-painted", RING_CU, RING_RC, ROD_R, n=8, slender=True),
        revolve("net-cone", "net", "chain-galvanised", [(TIERS[0][0], 0.0), (TIERS[0][0], TIERS[0][1]), (TIERS[-1][0], TIERS[-1][1]), (TIERS[-1][0], 0.0)], [1, 0, 0], [0, 0, 0], envelope=True),
    ]


PARTS = {50: parts50(), 100: parts100(), 200: parts200(), 500: parts500()}


def mass_of(sol):
    """⚖️ Mass of a 1:50 part from its true section and density (hollow sections by their wall, the perforated plate by its solid share)."""
    i = sol["id"]
    n = sol.get("repeat", {}).get("count", 1)
    if i == "plinth":
        return PL_W * PL_W * PL_H * RHO_C
    if i == "base-plate":
        return BP_W * BP_W * BP_T * RHO_S
    if i.startswith("anchor-nut"):
        return n * (3 * math.sqrt(3) / 2 * (NUT_AF / math.sqrt(3)) ** 2 - math.pi * 0.0105 ** 2) * NUT_H * RHO_S
    if i == "hinge-lug":
        return n * (0.04 * 0.02 + math.pi * KN_R ** 2 / 2 - math.pi * PIN_R ** 2) * (LUG_Z[1] - LUG_Z[0]) * RHO_S
    if i == "hinge-pin":
        return math.pi * PIN_R ** 2 * 2 * PIN_W * RHO_S
    if i == "foot-flange":
        return (FL_W ** 2 - math.pi * POLE_RI ** 2) * FL_T * RHO_S
    if i == "hinge-knuckle":
        return (math.pi * (KN_R ** 2 - PIN_R ** 2)) * 2 * KN_W * RHO_S
    if i == "foot-rib":
        return n * 0.5 * (FL_W / 2 - POLE_RO) * (0.08 + 0.012) * 0.008 * RHO_S
    if i == "pole-tube":
        return A_POLE * POLE_L * RHO_S
    if i == "pole-cap":
        return math.pi * CAP_R ** 2 * CAP_T * RHO_S
    if i == "arm-tube":
        return A_ARM * (ARM_U1 - ARM_U0) * RHO_S
    if i == "arm-end-plate":
        return EP_T * EP_H * EP_W * RHO_S
    if i == "arm-strut":
        return A_STRUT * math.dist(STRUT_P0, STRUT_P1) * RHO_S
    if i == "backboard-plate":
        return BW * BH * PLATE_T * (1 - OPEN) * RHO_S
    if i in ("frame-bottom", "frame-top"):
        return A_FR * BW * RHO_S
    if i in ("frame-side", "frame-mid"):
        return n * A_FR * (BH - 2 * FR) * RHO_S
    if i == "ring-mount":
        return MP_T * MP_H * 2 * MP_W * RHO_S
    if i == "ring-gusset":
        return n * 0.5 * ((0.066) + (0.012)) * (RING_IN + 0.02 - MP_T) * 0.008 * RHO_S
    if i == "ring-rod":
        return A_ROD * 2 * math.pi * RING_RC * RHO_S
    if i.startswith("chain"):
        return chain_length([sol]) * CHAIN_KGM
    raise KeyError(i)


BONE_MASS = {}
for _s in PARTS[50]:
    BONE_MASS[_s["bone"]] = BONE_MASS.get(_s["bone"], 0.0) + mass_of(_s)
M_TOTAL = sum(BONE_MASS.values())
NET_LEN = chain_length([s for s in PARTS[50] if s["bone"] == "net"])


def com_of(bone):
    """🎯️ Mass-weighted centre (u, v) of the 1:50 parts of a bone, from the centroids of their sample points."""
    mu = mv = mt = 0.0
    for s in PARTS[50]:
        if s["bone"] != bone:
            continue
        sp = samples(s)
        cu = sum(p[0] for p in sp) / len(sp)
        cv = sum(p[1] for p in sp) / len(sp)
        m = mass_of(s)
        mu += m * cu
        mv += m * cv
        mt += m
    return mu / mt, mv / mt


COM = {b: com_of(b) for b in BONE_MASS}

REFS = [
    ("din-en-1270-2005", "DIN EN 1270:2005 Playing field equipment - Basketball equipment - Functional and safety requirements, test methods (ring inner diameter 450-459 mm, top of the ring 3.05 m above the playing surface, ring rod 16-20 mm, inner edge of the ring 151 mm in front of the backboard, static strength tests of ring and backboard at the front of the ring)."),
    ("din-en-15312-2010", "DIN EN 15312:2010 Free access multi-sports equipment - Requirements, including safety and test methods (basketball units in public spaces: ring and backboard strength under hanging and dunking, finger entrapment at the ring mount and in the net fixing, overhang of the post)."),
    ("fiba-basketball-equipment-2022", "FIBA Official Basketball Rules 2022 - Basketball Equipment (backboard 1.80 x 1.05 m with the inner rectangle 0.59 x 0.45 m whose lower edge is level with the top of the ring; ring height 3.05 m)."),
    ("din-18035-1-2018", "DIN 18035-1:2018 Sports grounds - Part 1: Outdoor play and athletics areas, planning and dimensions (streetball and basketball courts: asphalt surface, run-out zones around the post)."),
    ("din-en-10219-2-2019", "DIN EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (CHS 114.3 x 4.0, 76.1 x 3.6, 48.3 x 2.9; square 40 x 40 x 2.0)."),
    ("din-en-iso-1461-2022", "DIN EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (zinc coating of post, base, chain and perforated plate; duplex system with powder coating on the post and frame)."),
    ("din-en-1991-1-1-2010", "DIN EN 1991-1-1:2010 Eurocode 1: Actions on structures - Densities, self-weight (steel 78.5 kN/m3, plain concrete 24 kN/m3)."),
    ("din-en-1991-1-4-2010", "DIN EN 1991-1-4:2010 with DIN EN 1991-1-4/NA:2010 Eurocode 1: Wind actions (simplified peak velocity pressure 0.65 kN/m2 for wind zone 2 inland up to 10 m; force coefficient of a plate 1.8, reduced for a perforated plate by its solidity ratio)."),
    ("din-en-1993-1-1-2010", "DIN EN 1993-1-1:2010 Eurocode 3: Design of steel structures - General rules (S235: fy 235 MPa, E 210 GPa)."),
    ("din-en-206-2021", "DIN EN 206:2021 and DIN 1045-2:2023 Concrete (C25/30 XC4/XF1 for the point foundation of street and sports equipment)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel 7800 kg/m3, 50 W/mK, 450 J/kgK; concrete 2400 kg/m3, 2.0 W/mK, 1000 J/kgK)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Appendix A.11: total hemispherical emissivities of metals, paints and concrete)."),
    ("vdi-waermeatlas-2013", "VDI-Wärmeatlas (2013), 11th ed., Springer Vieweg, section K1 (emissivities: bright galvanised steel about 0.05-0.1, weathered dull galvanised steel 0.25-0.3, oxidised and rusty steel 0.6-0.8)."),
    ("duffie-beckman-2013", "Duffie, J. A., Beckman, W. A. (2013). Solar Engineering of Thermal Processes, 4th ed., Wiley (solar absorptance of paints: dark green about 0.88 new, chalking and soiling lower it; orange enamel about 0.55-0.6, bare rusty steel 0.75-0.8)."),
    ("levinson-akbari-2002", "Levinson, R., Akbari, H. (2002). Effects of composition and exposure on the solar reflectance of Portland cement concrete. Cement and Concrete Research 32(11), 1679-1698 (weathered grey concrete absorbs about 0.65 of the sun)."),
    ("streetball-catalogue-typical", "Municipal sports and playground equipment catalogues of German manufacturers (catalogue-typical data, no brand): streetball units with a galvanised and powder-coated steel post 114.3 mm, 1.2 m overhang, perforated steel backboard 1.2 x 0.9 m on a 40 x 40 mm frame, solid ring 18 mm, galvanised chain net with 12 hooks, set on a base plate over a concrete point foundation, optionally as a hinged post that the depot can lower, about 120-160 kg."),
]

RIG = {"kind": "mechanical", "root": "footing", "bones": [
    {"id": "footing", "parent": None, "title": t("Concrete foundation head, base plate and lower hinge lugs (stay behind)", "Betonfundamentkopf, Fußplatte und untere Gelenklaschen (bleiben stehen)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": PL_W / 2, "joint": {"type": "fixed"},
     "mass": {"kg": r(BONE_MASS["footing"], 2), "comX": r(COM["footing"][0], 4), "comY": r(COM["footing"][1], 4), "distribution": "point", "source": "literature", "reference": "din-en-1991-1-1-2010"},
     "section": {"areaM2": r(PL_W * PL_H, 5), "secondMomentM4": r(PL_W * PL_H ** 3 / 12, 9), "extremeFibreM": r(PL_H / 2, 4), "material": "concrete-weathered", "title": t("Foundation head 400 x 60 mm", "Fundamentkopf 400 x 60 mm")}},
    {"id": "hinge", "parent": "footing", "title": t("Hinge pin of the lowering base (locked in service)", "Gelenkbolzen des Klappfußes (im Einsatz verriegelt)"), "x": r(P_HINGE[0]), "y": r(P_HINGE[1]), "angleDeg": r(P_HINGE[2], 4), "lengthM": 0.04, "joint": {"type": "revolute", "minDeg": -60, "maxDeg": 1},
     "mass": {"kg": r(BONE_MASS["hinge"], 3), "comX": 0, "comY": 0, "distribution": "point", "source": "literature", "reference": "din-en-1991-1-1-2010"}},
    {"id": "pole", "parent": "hinge", "title": t("Post CHS 114.3 x 4.0 with foot flange and knuckle", "Pfosten Rundrohr 114,3 x 4,0 mit Fußflansch und Gelenkauge"), "x": r(P_POLE[0]), "y": r(P_POLE[1]), "angleDeg": r(P_POLE[2], 4), "lengthM": POLE_L, "joint": {"type": "fixed"},
     "mass": {"kg": r(BONE_MASS["pole"], 2), "comX": r(COM["pole"][0], 4), "comY": r(COM["pole"][1], 4), "distribution": "point", "source": "literature", "reference": "din-en-10219-2-2019"},
     "section": {"areaM2": r(A_POLE, 7), "secondMomentM4": r(I_POLE, 10), "extremeFibreM": r(C_POLE, 5), "material": "steel-powder-coated", "title": t("CHS 114.3 x 4.0 S235", "Rundrohr 114,3 x 4,0 S235")}},
    {"id": "arm", "parent": "pole", "title": t("Offset arm CHS 76.1 x 3.6 with strut 48.3 x 2.9 and end plate (1.2 m overhang)", "Ausleger Rundrohr 76,1 x 3,6 mit Strebe 48,3 x 2,9 und Kopfplatte (1,2 m Ausladung)"), "x": r(P_ARM[0]), "y": r(P_ARM[1]), "angleDeg": r(P_ARM[2], 4), "lengthM": r(ARM_U1 + EP_T), "joint": {"type": "fixed"},
     "mass": {"kg": r(BONE_MASS["arm"], 2), "comX": r(COM["arm"][0], 4), "comY": r(COM["arm"][1], 4), "distribution": "point", "source": "literature", "reference": "din-en-10219-2-2019"},
     "section": {"areaM2": r(A_ARM, 7), "secondMomentM4": r(I_ARM, 10), "extremeFibreM": r(C_ARM, 5), "material": "steel-powder-coated", "title": t("CHS 76.1 x 3.6 S235", "Rundrohr 76,1 x 3,6 S235")}},
    {"id": "board", "parent": "arm", "title": t("Perforated steel backboard 1.2 x 0.9 m on its 40 x 40 frame", "Stahllochblech-Korbbrett 1,2 x 0,9 m auf seinem Rahmen 40 x 40"), "x": r(P_BOARD[0]), "y": r(P_BOARD[1]), "angleDeg": r(P_BOARD[2], 4), "lengthM": BH, "joint": {"type": "fixed"},
     "mass": {"kg": r(BONE_MASS["board"], 2), "comX": r(COM["board"][0], 4), "comY": r(COM["board"][1], 4), "distribution": "point", "source": "literature", "reference": "streetball-catalogue-typical"},
     "section": {"areaM2": r(2 * A_FR, 7), "secondMomentM4": r(2 * I_FR, 10), "extremeFibreM": FR / 2, "material": "steel-powder-coated", "title": t("Two frame posts 40 x 40 x 2", "Zwei Rahmenpfosten 40 x 40 x 2")}},
    {"id": "ring", "parent": "board", "title": t("Solid ring 18 mm on its mount, bent 2 degrees down by dunking", "Vollstabring 18 mm auf seiner Halterung, durch Dunks 2 Grad nach unten gebogen"), "x": r(P_RING[0]), "y": r(P_RING[1]), "angleDeg": r(P_RING[2], 4), "lengthM": r(RING_IN + 2 * RING_RI + 2 * ROD_R), "joint": {"type": "revolute", "minDeg": -6, "maxDeg": 2},
     "mass": {"kg": r(BONE_MASS["ring"], 3), "comX": r(COM["ring"][0], 4), "comY": r(COM["ring"][1], 4), "distribution": "point", "source": "literature", "reference": "din-en-1270-2005"},
     "section": {"areaM2": r(A_ROD, 8), "secondMomentM4": r(I_ROD, 12), "extremeFibreM": ROD_R, "material": "steel-ring-painted", "title": t("Round bar 18 mm", "Rundstab 18 mm")}},
    {"id": "net", "parent": "ring", "title": t("Galvanised chain net, 12 hooks, torn in one sector", "Verzinktes Kettennetz, 12 Haken, in einem Feld gerissen"), "x": r(P_NET[0]), "y": r(P_NET[1]), "angleDeg": r(P_NET[2], 4), "lengthM": TIERS[-1][0], "joint": {"type": "revolute", "minDeg": -35, "maxDeg": 60},
     "mass": {"kg": r(BONE_MASS["net"], 3), "comX": r(COM["net"][0], 4), "comY": r(COM["net"][1], 4), "distribution": "point", "source": "literature", "reference": "streetball-catalogue-typical"},
     "section": {"areaM2": r(12 * 2 * math.pi * 0.0015 ** 2, 9), "secondMomentM4": 1e-12, "extremeFibreM": 0.0015, "material": "chain-galvanised", "title": t("Twelve chains of 3 mm wire", "Zwölf Ketten aus 3-mm-Draht")}},
]}

RING_FRONT_U = RING_IN + 2 * RING_RI + 2 * ROD_R - ROD_R
ANCHORS = [
    {"id": "foundation", "bone": "footing", "x": 0, "y": 0, "kind": "support", "title": t("Point foundation 0.6 x 0.6 x 0.9 m below the asphalt (fixed)", "Punktfundament 0,6 x 0,6 x 0,9 m unter dem Asphalt (eingespannt)"), "support": "fixed", "footprintLengthM": PL_W, "footprintWidthM": PL_W},
    {"id": "hinge-pin", "bone": "hinge", "x": 0, "y": 0, "kind": "support", "title": t("Hinge pin, carrying the post while it is lowered", "Gelenkbolzen, trägt den Pfosten beim Umlegen"), "support": "pin"},
    {"id": "crane-sling", "bone": "pole", "x": LIFT_U, "y": 0, "kind": "support", "title": t("Round sling of the depot's loader crane around the post", "Rundschlinge des Bauhof-Ladekrans um den Pfosten"), "support": "pin"},
    {"id": "ring-front", "bone": "ring", "x": r(RING_FRONT_U), "y": ROD_R, "kind": "load", "title": t("Front of the ring where players hang after a dunk", "Vorderkante des Rings, an der Spieler nach einem Dunk hängen")},
    {"id": "board-centre", "bone": "board", "x": BH / 2, "y": -PLATE_T, "kind": "load", "title": t("Centre of pressure of the backboard", "Druckmittelpunkt des Korbbretts")},
]

REGIONS = ["foundation-head", "base-hinge", "pole-coating", "backboard", "ring", "chain-net"]
SUN_SERVICE = {"foundation-head": 0.55, "base-hinge": 0.5, "pole-coating": 0.45, "backboard": 0.5, "ring": 0.5, "chain-net": 0.4}


def bind(sun, contact=None):
    """🌡️ The region bindings of one pose."""
    regions = {}
    for k in REGIONS:
        b = {"exposedFraction": 1.0 if k != "foundation-head" else 0.62, "sunlitFraction": sun[k]}
        if k == "foundation-head":
            b["contact"] = {"fraction": 0.38, "tempC": 22}
        regions[k] = b
    return {"regions": regions, "emitters": {}}


SUN_LOWERED = {"foundation-head": 0.55, "base-hinge": 0.5, "pole-coating": 0.5, "backboard": 0.45, "ring": 0.6, "chain-net": 0.55}
DUNK_N = r(95 * G * 2.0, 1)
WIND_N = r(650 * 1.8 * (1 - OPEN) * BW * BH, 1)

POSES = [
    {"id": "in-service", "title": t("In service", "Im Einsatz"), "description": t("The post stands bolted on its hinged base over the point foundation at the edge of the asphalt court, ring 3.05 m up, nobody playing, afternoon sun on the dark green post and the grey perforated board.", "Der Pfosten steht verschraubt auf seinem Klappfuß über dem Punktfundament am Rand des Asphaltplatzes, der Ring 3,05 m hoch, niemand spielt, Nachmittagssonne auf dem dunkelgrünen Pfosten und dem grauen Lochblechbrett."),
     "bones": {}, "ground": "auto", "contacts": ["foundation"], "thermal": bind(SUN_SERVICE), "loads": []},
    {"id": "maintenance", "title": t("Lowered for a new net", "Zum Netzwechsel umgelegt"), "description": t("The depot crew has removed the locking bolts of the hinged base; the loader crane holds the post on a round sling 2.6 m up and has lowered it 50 degrees forward about the hinge pin, so the ring comes down to about 0.8 m and the torn chain net can be replaced; the foundation head stays behind.", "Der Bauhof hat die Sicherungsschrauben des Klappfußes gelöst; der Ladekran hält den Pfosten an einer Rundschlinge in 2,6 m Höhe und hat ihn um 50 Grad nach vorn um den Gelenkbolzen umgelegt, so dass der Ring auf etwa 0,8 m herunterkommt und das gerissene Kettennetz getauscht werden kann; der Fundamentkopf bleibt stehen."),
     "bones": {"hinge": {"angleDeg": -50}}, "ground": "auto", "contacts": ["hinge-pin", "crane-sling"], "thermal": bind(SUN_LOWERED), "loads": []},
    {"id": "in-use", "title": t("Player hanging on the ring after a dunk", "Spieler hängt nach einem Dunk am Ring"), "description": t("A player of about 95 kg has dunked and hangs on the front of the ring: the ring flexes a further 3 degrees down on its mount, the post leans a few millimetres, the chain net follows the ring.", "Ein Spieler von etwa 95 kg hat gedunkt und hängt an der Vorderkante des Rings: der Ring federt auf seiner Halterung weitere 3 Grad nach unten, der Pfosten neigt sich wenige Millimeter, das Kettennetz folgt dem Ring."),
     "bones": {"hinge": {"angleDeg": -0.2}, "ring": {"angleDeg": -3}}, "ground": "auto", "contacts": ["foundation"], "thermal": bind(SUN_SERVICE),
     "loads": [{"id": "dunk-hang", "kind": "live", "title": t("Player hanging on the ring front (95 kg, dynamic factor 2.0)", "Spieler hängt an der Ringvorderkante (95 kg, dynamischer Faktor 2,0)"), "anchor": "ring-front", "fxN": 0, "fyN": -DUNK_N, "source": "authored-estimate", "reference": "din-en-15312-2010"}]},
    {"id": "storm", "title": t("Storm gust on the backboard", "Sturmböe auf das Korbbrett"), "description": t("A gust from behind presses on the perforated backboard; the post sways a little and the chain net swings forward on its hooks.", "Eine Böe von hinten drückt auf das Lochblechbrett; der Pfosten schwankt ein wenig und das Kettennetz schwingt an seinen Haken nach vorn."),
     "bones": {"hinge": {"angleDeg": -0.4}, "net": {"angleDeg": 3}}, "ground": "auto", "contacts": ["foundation"], "thermal": bind({k: 0.0 for k in REGIONS}),
     "loads": [{"id": "wind-board", "kind": "wind", "title": t("Wind on the perforated backboard (0.65 kN/m2, cf 1.8, solidity 0.67)", "Wind auf das Lochblechbrett (0,65 kN/m2, cf 1,8, Völligkeit 0,67)"), "anchor": "board-centre", "fxN": WIND_N, "fyN": 0, "source": "literature", "reference": "din-en-1991-1-4-2010"}]},
]

CLIPS = [
    {"id": "lowering", "title": t("Lowering the post for maintenance", "Umlegen des Pfostens zur Wartung"), "description": t("The crane takes the post on the round sling, the crew pulls the two locking bolts, and the post turns 50 degrees forward about the hinge pin until the ring is about 0.8 m above the court; the chain net turns with the ring.", "Der Kran nimmt den Pfosten an der Rundschlinge, die Mannschaft zieht die beiden Sicherungsschrauben, und der Pfosten dreht sich um 50 Grad nach vorn um den Gelenkbolzen, bis der Ring etwa 0,8 m über dem Platz ist; das Kettennetz dreht sich mit dem Ring."),
     "durationS": 16, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 3, "pose": "in-service", "ease": "smooth"}, {"t": 13, "pose": "maintenance", "ease": "smooth"}, {"t": 16, "pose": "maintenance", "ease": "smooth"}],
     "contacts": [{"anchor": "foundation", "fromS": 0, "toS": 3}, {"anchor": "hinge-pin", "fromS": 2.5, "toS": 16}, {"anchor": "crane-sling", "fromS": 2.5, "toS": 16}],
     "phases": [{"id": "sling-on", "fromS": 0, "toS": 3, "title": t("Sling tensioned, locking bolts pulled", "Schlinge gespannt, Sicherungsschrauben gezogen")}, {"id": "lower", "fromS": 3, "toS": 13, "title": t("Lowering about the hinge", "Umlegen um das Gelenk")}, {"id": "hold", "fromS": 13, "toS": 16, "title": t("Held lowered", "Umgelegt gehalten")}],
     "events": [{"t": 2.5, "kind": "contact-on", "title": t("Crane sling takes the post", "Kranschlinge übernimmt den Pfosten"), "target": "crane-sling"}, {"t": 3, "kind": "contact-off", "title": t("Base unlocked", "Fuß entriegelt"), "target": "foundation"}],
     "tracks": []},
    {"id": "dunk", "title": t("Dunk and hang", "Dunk und Hängen"), "description": t("A player dunks, grabs the front of the ring and hangs for half a second; the ring springs down and back, the chain net swings and settles.", "Ein Spieler dunkt, greift die Vorderkante des Rings und hängt eine halbe Sekunde; der Ring federt nach unten und zurück, das Kettennetz schwingt und beruhigt sich."),
     "durationS": 2.4, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "in"}, {"t": 0.3, "pose": "in-use", "ease": "linear"}, {"t": 0.9, "pose": "in-use", "ease": "out"},
              {"t": 1.1, "pose": "in-service", "ease": "smooth", "bones": {"ring": {"angleDeg": 1}, "net": {"angleDeg": -3}}},
              {"t": 1.5, "pose": "in-service", "ease": "smooth", "bones": {"net": {"angleDeg": 2}}},
              {"t": 1.9, "pose": "in-service", "ease": "smooth", "bones": {"net": {"angleDeg": -1}}},
              {"t": 2.4, "pose": "in-service", "ease": "smooth"}],
     "contacts": [{"anchor": "foundation", "fromS": 0, "toS": 2.4}],
     "phases": [{"id": "dunk", "fromS": 0, "toS": 0.3, "title": t("Ball through, hands on the ring", "Ball durch, Hände am Ring")}, {"id": "hang", "fromS": 0.3, "toS": 0.9, "title": t("Hanging", "Hängen")}, {"id": "release", "fromS": 0.9, "toS": 2.4, "title": t("Release, net swings out", "Loslassen, Netz schwingt aus")}],
     "events": [{"t": 0.2, "kind": "load-on", "title": t("Player grabs the ring", "Spieler greift den Ring"), "target": "dunk-hang"}, {"t": 0.95, "kind": "load-off", "title": t("Player lets go", "Spieler lässt los"), "target": "dunk-hang"}],
     "tracks": [{"target": "load:dunk-hang", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.3, "v": 1}, {"t": 0.9, "v": 1}, {"t": 1.0, "v": 0}, {"t": 2.4, "v": 0}]}]},
]


def layer(i, en, de, kind, m, thick, k, rho, cp, ref="iso-10456-2007"):
    """🧶️ One region layer with its one-hour areal heat capacity rho*cp*min(d, sqrt(alpha*3600))."""
    a = k / (rho * cp)
    dpen = min(thick, math.sqrt(a * 3600))
    return {"id": i, "title": t(en, de), "kind": kind, "material": m, "thicknessM": thick, "resistanceM2KW": r(thick / k, 6), "heatCapacityJM2K": round(rho * cp * dpen), "source": "literature", "reference": ref}


LAYERS = {
    "foundation-head": layer("concrete", "Concrete foundation head", "Beton des Fundamentkopfs", "masonry", "concrete-weathered", PL_H, 2.0, RHO_C, 1000),
    "base-hinge": layer("plate", "Galvanised base and flange plates 15 mm", "Verzinkte Fuß- und Flanschplatten 15 mm", "metal", "steel-galvanised-rusty", 0.015, 50, RHO_S, 450),
    "pole-coating": layer("tube-wall", "Coated tube wall 4 mm", "Beschichtete Rohrwand 4 mm", "metal", "steel-powder-coated", 0.004, 50, RHO_S, 450),
    "backboard": layer("perforated-plate", "Perforated plate 3 mm", "Lochblech 3 mm", "metal", "steel-perforated", PLATE_T, 50, RHO_S * (1 - OPEN), 450),
    "ring": layer("rod", "Solid ring rod 18 mm", "Vollstab 18 mm", "metal", "steel-ring-painted", 2 * ROD_R, 50, RHO_S, 450),
    "chain-net": layer("chain-wire", "Chain wire 3 mm", "Kettendraht 3 mm", "metal", "chain-galvanised", 0.003, 50, RHO_S, 450),
}

AREA = {"foundation-head": 0.2560, "base-hinge": 0.4000, "pole-coating": 2.7400, "backboard": 2.1700, "ring": 0.1520, "chain-net": 0.2400}
AREA_FILE = os.path.join(HERE, "author-areas.json")
if os.path.exists(AREA_FILE):
    with open(AREA_FILE, encoding="utf-8") as fh:
        AREA.update(json.load(fh))

REGION_DEF = [
    ("foundation-head", "Weathered concrete foundation head", "Verwitterter Betonfundamentkopf", "concrete-weathered", PL_H, "horizontal-plate-up", 0.9, "levinson-akbari-2002"),
    ("base-hinge", "Rusty galvanised base plate, hinge and foot flange", "Rostige verzinkte Fußplatte, Gelenk und Fußflansch", "steel-galvanised-rusty", 0.1, "horizontal-plate-up", 0.85, "vdi-waermeatlas-2013"),
    ("pole-coating", "Moss-green powder-coated post, arm and board frame", "Moosgrün pulverbeschichteter Pfosten, Ausleger und Brettrahmen", "steel-powder-coated", POLE_D, "cylinder", 0.5, "duffie-beckman-2013"),
    ("backboard", "Perforated galvanised backboard", "Verzinktes Lochblech-Korbbrett", "steel-perforated", BH, "vertical-plate", 0.5, "vdi-waermeatlas-2013"),
    ("ring", "Worn orange ring and mount", "Abgenutzter oranger Ring mit Halterung", "steel-ring-painted", 2 * ROD_R, "cylinder", 0.6, "duffie-beckman-2013"),
    ("chain-net", "Galvanised chain net", "Verzinktes Kettennetz", "chain-galvanised", 0.006, "cylinder", 0.5, "vdi-waermeatlas-2013"),
]

THERMAL = {
    "environment": {"airTempC": 29, "meanRadiantTempC": 33, "skyTempC": 16, "groundTempC": 45, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 90, "solarWM2": 820, "diffuseFraction": 0.15, "pressurePa": 101325},
    "referenceEnvironment": "summer-hot-sun",
    "passengers": [],
    "operating": [],
    "regions": [{"id": i, "title": t(en, de), "kind": "surface", "model": "passive", "material": m, "areaM2": r(AREA[i], 4), "characteristicLengthM": r(lc, 4), "geometry": geo, "skyViewFactor": svf, "layers": [LAYERS[i]], "heatCapacityJM2K": LAYERS[i]["heatCapacityJM2K"], "source": "authored-estimate" if i != "backboard" else "literature", "reference": ref}
                for i, en, de, m, lc, geo, svf, ref in REGION_DEF],
    "emitters": [],
    "assumptions": [
        t("Passive surfaces only: absorbed sun, long-wave exchange with sky and the hot asphalt court, convection and heat capacity; the unit draws no power (0 W) and has no emitters.", "Nur passive Flächen: absorbierte Sonne, Langwellenaustausch mit Himmel und heißem Asphaltplatz, Konvektion und Wärmekapazität; die Anlage nimmt keine Leistung auf (0 W) und hat keine Emitter."),
        t("Weathered surfaces differ from new ones: the moss-green powder coating has chalked and absorbs about 0.84 of the sun instead of about 0.88 new, so the thin 4 mm post wall is the hot part on a summer afternoon; the perforated plate has lost its bright zinc (emissivity 0.05-0.1 new) to a dull patina of about 0.28, absorbing 0.62; the orange ring enamel is worn to bare rusty steel at the front, raising its absorptance from about 0.58 to 0.68; the hinge and base plate are rusty (emissivity 0.55).", "Verwitterte Flächen unterscheiden sich von neuen: die moosgrüne Pulverbeschichtung kreidet und absorbiert etwa 0,84 der Sonne statt etwa 0,88 im Neuzustand, so dass die dünne 4-mm-Pfostenwand an einem Sommernachmittag der heiße Teil ist; das Lochblech hat sein blankes Zink (Emissionsgrad neu 0,05-0,1) gegen eine matte Patina von etwa 0,28 getauscht und absorbiert 0,62; die orange Ringlackierung ist vorn bis auf rostigen blanken Stahl abgenutzt, was die Absorption von etwa 0,58 auf 0,68 hebt; Gelenk und Fußplatte sind rostig (Emissionsgrad 0,55)."),
        t("areaM2 is the total outer surface of each region's 1:50 solids; the perforated board is modelled as a closed plate: its 33 % open area removes about a third of the two faces but the hole walls add about the same back, so the plate area stands for both. The foundation head continues into the ground below the asphalt (its underside is a section face); its sides are 38 % in contact with the asphalt at 22 degC under the surface.", "areaM2 ist die gesamte Außenfläche der 1:50-Körper jeder Region; das Lochblech ist als geschlossene Platte modelliert: seine 33 % offene Fläche nimmt den beiden Seiten etwa ein Drittel, die Lochwandungen geben etwa ebenso viel zurück, so dass die Plattenfläche beides vertritt. Der Fundamentkopf setzt sich unter dem Asphalt im Boden fort (seine Unterseite ist eine Schnittfläche); seine Seiten berühren zu 38 % den Asphalt bei 22 degC unter der Oberfläche."),
        t("Reference case: a German summer afternoon, 29 degC air, 820 W/m2 sun, 1.5 m/s breeze, the asphalt court at 45 degC; sunlit fractions are scenario data of each pose.", "Referenzfall: deutscher Sommernachmittag, 29 degC Luft, 820 W/m2 Sonne, 1,5 m/s Wind, Asphaltplatz bei 45 degC; besonnte Anteile sind Szenariodaten jeder Pose."),
    ],
}

STRUCTURE = {
    "gravityMS2": G, "dynamics": "quasi-static", "supports": [],
    "assumptions": [
        t("In service the post is a cantilever fixed in its point foundation (0.6 x 0.6 x 0.9 m C25/30 below the asphalt, cut at the court surface: only its head is modelled); the hinged base is locked by two bolts and acts as a rigid joint; the 1.2 m arm with its strut carries the backboard and ring as a rigid bracket.", "Im Einsatz ist der Pfosten ein Kragarm, eingespannt im Punktfundament (0,6 x 0,6 x 0,9 m C25/30 unter dem Asphalt, an der Platzoberfläche geschnitten: nur sein Kopf ist modelliert); der Klappfuß ist mit zwei Schrauben verriegelt und wirkt als starre Verbindung; der 1,2-m-Ausleger mit seiner Strebe trägt Brett und Ring als starre Konsole."),
        t("Maintenance: with the locking bolts pulled, the post turns about the hinge pin and is held by the loader crane on a round sling 2.6 m up the post; the pin and the sling are the two pin supports, the crane and sling belong to the depot and are not modelled; the foundation head stays behind (its own small weight is reported at the pin).", "Wartung: Mit gezogenen Sicherungsschrauben dreht sich der Pfosten um den Gelenkbolzen und wird vom Ladekran an einer Rundschlinge 2,6 m am Pfosten gehalten; Bolzen und Schlinge sind die zwei gelenkigen Auflager, Kran und Schlinge gehören dem Bauhof und sind nicht modelliert; der Fundamentkopf bleibt stehen (sein kleines Eigengewicht erscheint am Bolzen)."),
        t("In use: a player of 95 kg hanging on the ring front after a dunk with a dynamic factor of 2.0 (1.86 kN); DIN EN 1270 and DIN EN 15312 test ring and backboard statically at the front of the ring with higher loads, this is the service case, not the test. Storm: 0.65 kN/m2 peak pressure (DIN EN 1991-1-4/NA, wind zone 2 inland) times cf 1.8 on the 1.08 m2 board reduced by its solidity 0.67 (0.85 kN).", "In Benutzung: ein Spieler von 95 kg hängt nach einem Dunk mit dynamischem Faktor 2,0 an der Ringvorderkante (1,86 kN); DIN EN 1270 und DIN EN 15312 prüfen Ring und Brett statisch an der Ringvorderkante mit höheren Lasten, dies ist der Gebrauchsfall, nicht die Prüfung. Sturm: 0,65 kN/m2 Böengeschwindigkeitsdruck (DIN EN 1991-1-4/NA, Windzone 2 Binnenland) mal cf 1,8 auf das 1,08-m2-Brett, vermindert um seine Völligkeit 0,67 (0,85 kN)."),
        t("Members with their true sections (DIN EN 10219): post CHS 114.3 x 4.0, arm CHS 76.1 x 3.6, strut CHS 48.3 x 2.9, board frame 40 x 40 x 2, ring round bar 18 mm, all S235; masses are section times density (steel 7850 kg/m3, the perforated plate at its 67 % solid share, chain 0.18 kg/m, concrete head 2400 kg/m3).", "Glieder mit ihren wahren Querschnitten (DIN EN 10219): Pfosten Rundrohr 114,3 x 4,0, Ausleger 76,1 x 3,6, Strebe 48,3 x 2,9, Brettrahmen 40 x 40 x 2, Ring Rundstab 18 mm, alle S235; Massen sind Querschnitt mal Dichte (Stahl 7850 kg/m3, das Lochblech mit seinem Vollanteil von 67 %, Kette 0,18 kg/m, Betonkopf 2400 kg/m3)."),
    ],
    "clinical": False,
}

TAGS = ["used", "park", "public-space", "sport", "basketball", "streetball", "steel", "galvanised-steel", "perforated-steel", "chain-net", "street-furniture", "weathered"]

SUMMARY = t(
    f"A street basketball unit of the municipal catalogue type at the edge of an asphalt court: a 114.3 mm galvanised and moss-green powder-coated steel post on a hinged base plate over a concrete point foundation, a 1.2 m offset arm with strut, a 1.2 x 0.9 m perforated steel backboard and an 18 mm solid ring 3.05 m up with a galvanised chain net, about {M_TOTAL:.0f} kg above the foundation. Years of play left the ring bent 2 degrees down by dunks and worn to bare steel, the chain net torn in one sector with a dangling end, ball marks, tags and stickers on the board and post, flaking coating and rust at the foot; the depot lowers it forward about its hinge to change the net, and players hang on the ring.",
    f"Eine Streetball-Korbanlage vom Typ der kommunalen Kataloge am Rand eines Asphaltplatzes: ein verzinkter und moosgrün pulverbeschichteter Stahlpfosten 114,3 mm auf einem Klappfuß über einem Betonpunktfundament, ein 1,2 m Ausleger mit Strebe, ein Stahllochblech-Korbbrett 1,2 x 0,9 m und ein 18-mm-Vollstabring in 3,05 m Höhe mit verzinktem Kettennetz, etwa {M_TOTAL:.0f} kg über dem Fundament. Die Jahre des Spielens haben den Ring durch Dunks 2 Grad nach unten gebogen und bis auf blanken Stahl abgenutzt, das Kettennetz in einem Feld mit herabhängendem Ende zerrissen, Ballabdrücke, Tags und Aufkleber auf Brett und Pfosten, abblätternde Beschichtung und Rost am Fuß hinterlassen; der Bauhof legt ihn zum Netzwechsel um sein Gelenk nach vorn um, und Spieler hängen am Ring.")

PROVENANCE = {
    "author": "Semio staffage content (used park objects wave)",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": [{"id": i, "citation": c} for i, c in REFS],
    "notes": t(
        f"Catalogue type of German municipal streetball units (no brand): post CHS 114.3 x 4.0 S235 hot-dip galvanised and powder coated RAL 6005, hinged base (base plate 340 x 340 x 15 on four M20 anchors, flange 200 x 200 x 15 with four ribs, pin 16 mm), offset arm CHS 76.1 x 3.6 at 3.55 m with strut CHS 48.3 x 2.9 and end plate, 1.2 m overhang from the post face to the board, perforated plate 1.2 x 0.9 x 3 mm (33 % open) on a 40 x 40 x 2 frame with two mid posts, lower board edge 2.90 m, ring 18 mm solid bar, 450 mm inside, inner edge 151 mm in front of the board, top 3.05 m when new (DIN EN 1270), galvanised chain net with 12 hooks in three diamond rows, 0.40 m long. Masses from the 1:50 parts: footing {BONE_MASS['footing']:.1f} kg, hinge pin {BONE_MASS['hinge']:.2f} kg, post {BONE_MASS['pole']:.1f} kg, arm {BONE_MASS['arm']:.1f} kg, board {BONE_MASS['board']:.1f} kg, ring {BONE_MASS['ring']:.1f} kg, net {BONE_MASS['net']:.2f} kg ({NET_LEN:.1f} m chain), total {M_TOTAL:.1f} kg. Traces of use: ring bent 2 degrees down and its enamel worn, two chain diamonds torn out with one chain end dangling 0.22 m, ball marks, a painted shooting square, tags and stickers as texture, coating flaking and rust at the post foot and the hinge. All numbers are literature or authored estimates; nothing is measured.",
        f"Katalogtyp der deutschen kommunalen Streetball-Korbanlagen (ohne Marke): Pfosten Rundrohr 114,3 x 4,0 S235 feuerverzinkt und pulverbeschichtet RAL 6005, Klappfuß (Fußplatte 340 x 340 x 15 auf vier Ankern M20, Flansch 200 x 200 x 15 mit vier Rippen, Bolzen 16 mm), Ausleger Rundrohr 76,1 x 3,6 in 3,55 m mit Strebe 48,3 x 2,9 und Kopfplatte, 1,2 m Ausladung von der Pfostenvorderseite bis zum Brett, Lochblech 1,2 x 0,9 x 3 mm (33 % offen) auf einem Rahmen 40 x 40 x 2 mit zwei Mittelpfosten, Brettunterkante 2,90 m, Ring Vollstab 18 mm, 450 mm innen, Innenkante 151 mm vor dem Brett, Oberkante 3,05 m im Neuzustand (DIN EN 1270), verzinktes Kettennetz mit 12 Haken in drei Rautenreihen, 0,40 m lang. Massen aus den 1:50-Teilen: Fuß {BONE_MASS['footing']:.1f} kg, Gelenkbolzen {BONE_MASS['hinge']:.2f} kg, Pfosten {BONE_MASS['pole']:.1f} kg, Ausleger {BONE_MASS['arm']:.1f} kg, Brett {BONE_MASS['board']:.1f} kg, Ring {BONE_MASS['ring']:.1f} kg, Netz {BONE_MASS['net']:.2f} kg ({NET_LEN:.1f} m Kette), zusammen {M_TOTAL:.1f} kg. Gebrauchsspuren: Ring 2 Grad nach unten gebogen und seine Lackierung abgenutzt, zwei Kettenrauten ausgerissen mit einem 0,22 m herabhängenden Kettenende, Ballabdrücke, ein aufgemaltes Zielquadrat, Tags und Aufkleber als Textur, abblätternde Beschichtung und Rost am Pfostenfuß und am Gelenk. Alle Zahlen sind Literatur- oder Schätzwerte; nichts ist gemessen."),
}

Z_ORDER = {"elevation": {}, "front": {}, "plan": {}}


def base_z(sol, view):
    """🧮️ Painting order of a solid's outline in one view: nearer parts are drawn later."""
    sp = samples(sol)
    w = [to_world(sol["bone"], p) for p in sp]
    if view == "elevation":
        d = max(p[2] for p in w)
    elif view == "front":
        d = max(p[0] for p in w)
    else:
        d = max(p[1] for p in w)
    return round(5 + d, 4)


def shape_of(sol, view, scale, pen=None, tone=None, role=None, sid=None, z=None):
    """✏️ The filled outline of a body solid in one view, bound to its bone, region and material."""
    if sol["kind"] == "capsule" and sol.get("slender"):
        return None
    pts_ = outline(sol, view)
    s = {"id": sid or sol["id"], "bone": sol["bone"], "role": role or ("body" if sol["role"] == "body" else "detail"), "z": z if z is not None else base_z(sol, view), "filled": True, "tone": sol["tone"] if tone is None else tone,
         "kind": "poly", "points": pts(pts_), "closed": True, "region": REG[sol["material"]], "material": sol["material"]}
    if pen:
        s["penMm"] = pen
    return s


def chain_strokes(sols, view, pen, sid, z):
    """⛓️ The chains of the net as one path of their projected centre lines."""
    d = []
    for s in sols:
        if s["kind"] != "capsule":
            continue
        for a, b in axis_stroke(s, view):
            d.append(f"M {r(a[0])} {r(a[1])} L {r(b[0])} {r(b[1])}")
    return {"id": sid, "bone": "net", "role": "detail", "z": z, "filled": False, "tone": TONE["chain-galvanised"], "kind": "path", "d": " ".join(d), "penMm": pen, "region": "chain-net", "material": "chain-galvanised"}


def strut_stroke(sol, view, pen, z):
    """〰️ A slender strut drawn by its axis."""
    (a, b), = axis_stroke(sol, view)
    return {"id": sol["id"], "bone": sol["bone"], "role": "body", "z": z, "filled": False, "tone": sol["tone"], "kind": "poly", "points": pts([a, b]), "closed": False, "penMm": pen, "region": REG[sol["material"]], "material": sol["material"]}


def ring_plan(sol, pen, z):
    """⭕️ The ring seen from above: an outline around its void, drawn on the rod's centre line at the rod's width."""
    c = to_world("ring", (RING_CU, 0, 0))
    ca = math.cos(math.radians(RING_BEND))
    return {"id": "ring-rod", "bone": "ring", "role": "body", "z": z, "filled": False, "tone": sol["tone"], "kind": "ellipse", "cx": r(c[0]), "cy": 0, "rx": r(RING_RC * ca), "ry": r(RING_RC), "penMm": pen, "region": "ring", "material": "steel-ring-painted"}


def ring_band(sol, view, half, z):
    """⭕️ The ring generalised for 1:500 as a band of a printable height on its centre plane (elevation strip, front ellipse)."""
    base = {"id": "ring-rod", "bone": "ring", "role": "body", "z": z, "filled": True, "tone": sol["tone"], "region": "ring", "material": "steel-ring-painted"}
    if view == "elevation":
        base.update({"kind": "rect", "x": r(RING_IN - ROD_R), "y": -half, "w": r(2 * RING_RI + 4 * ROD_R), "h": 2 * half})
    else:
        c = to_world("ring", (RING_CU, 0, 0))
        base.update({"kind": "ellipse", "cx": 0, "cy": r(c[1]), "rx": r(RING_RC + ROD_R), "ry": half})
    return base


def tex(i, bone, view_pts, tone, region, material, z, closed=True, filled=True, role="texture", pen=None, kind="poly"):
    """🎨️ A hand-drawn texture or detail shape."""
    s = {"id": i, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": pts(view_pts), "closed": closed, "region": region, "material": material}
    if pen:
        s["penMm"] = pen
    return s


def path(i, bone, d, tone, region, material, z, role="texture", pen=None):
    """🖊️ A hand-drawn stroke path."""
    s = {"id": i, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "kind": "path", "d": d, "region": region, "material": material}
    if pen:
        s["penMm"] = pen
    return s


def seg_path(segs):
    """🖊️ Path data of a list of segments."""
    return " ".join(f"M {r(a[0])} {r(a[1])} L {r(b[0])} {r(b[1])}" for a, b in segs)


def board_front_pt(u, w):
    """🏀️ A point on the playing face of the board in the front view (world z, height)."""
    return (w, BOARD_Y + u)


def textures50(view):
    """🎨️ The 1:50 traces of use and fine details of one view."""
    out = []
    if view == "elevation":
        out.append(tex("rust-foot", "pole", [(0.0, -POLE_RO), (0.0, POLE_RO), (0.21, POLE_RO), (0.17, 0.02), (0.24, -0.01), (0.13, -POLE_RO)], 0.42, "base-hinge", "steel-galvanised-rusty", 8.0))
        out.append(tex("coating-flake", "pole", [(0.32, -POLE_RO), (0.36, -0.03), (0.41, -0.045), (0.44, -POLE_RO)], 0.5, "pole-coating", "steel-powder-coated", 8.1))
        out.append(tex("sticker-pole", "pole", [(1.38, -0.035), (1.48, -0.035), (1.48, 0.03), (1.38, 0.03)], 0.85, "pole-coating", "steel-powder-coated", 8.2))
        out.append(path("ball-scuffs-pole", "pole", "M 0.9 -0.05 L 0.96 -0.05 M 1.1 -0.054 L 1.14 -0.054 M 2.05 -0.05 L 2.12 -0.05", 0.55, "pole-coating", "steel-powder-coated", 8.3))
        out.append(path("ring-wear", "ring", f"M {r(RING_FRONT_U - 0.08)} 0 L {r(RING_FRONT_U + 0.004)} 0", 0.75, "ring", "steel-ring-painted", 9.6))
    if view == "front":
        out.append(path("shooting-square", "board", "M -0.295 3.059 L 0.295 3.059 L 0.295 3.509 L -0.295 3.509 Z M -0.57 2.93 L 0.57 2.93 L 0.57 3.77 L -0.57 3.77 Z", 0.92, "backboard", "steel-perforated", 9.0, pen=0.25))
        out.append(tex("tag-board", "board", [(0.34, 3.6), (0.5, 3.66), (0.52, 3.58), (0.4, 3.52), (0.55, 3.47), (0.36, 3.43)], 0.22, "backboard", "steel-perforated", 9.1))
        out.append(tex("sticker-board", "board", [(-0.5, 3.0), (-0.42, 3.0), (-0.42, 3.07), (-0.5, 3.07)], 0.88, "backboard", "steel-perforated", 9.2))
        out.append(path("ball-marks", "board", "M -0.12 3.42 L -0.06 3.43 M 0.08 3.36 L 0.15 3.38 M -0.02 3.6 L 0.04 3.59 M 0.18 3.52 L 0.24 3.54 M -0.2 3.3 L -0.15 3.29", 0.4, "backboard", "steel-perforated", 9.3))
        out.append(tex("rust-foot", "pole", [(-POLE_RO, POLE_Y0), (POLE_RO, POLE_Y0), (POLE_RO, POLE_Y0 + 0.19), (0.01, POLE_Y0 + 0.24), (-0.03, POLE_Y0 + 0.15), (-POLE_RO, POLE_Y0 + 0.2)], 0.42, "base-hinge", "steel-galvanised-rusty", 8.0))
        out.append(tex("sticker-pole", "pole", [(-0.04, POLE_Y0 + 1.36), (0.02, POLE_Y0 + 1.36), (0.02, POLE_Y0 + 1.45), (-0.04, POLE_Y0 + 1.45)], 0.85, "pole-coating", "steel-powder-coated", 8.2))
    if view == "plan":
        c = to_world("ring", (RING_CU, 0, 0))
        out.append(path("ring-wear", "ring", f"M {r(c[0] + RING_RC - 0.002)} -0.06 L {r(c[0] + RING_RC - 0.002)} 0.06", 0.75, "ring", "steel-ring-painted", 9.7))
        out.append(path("plinth-crack", "footing", "M -0.2 0.12 L -0.16 0.13 L -0.175 0.17 M 0.17 -0.2 L 0.18 -0.16", 0.4, "foundation-head", "concrete-weathered", 8.0, pen=0.13))
    return out


HATCH50 = [{"id": "perforation", "title": t("Perforation Rv 10-15 (typified)", "Lochung Rv 10-15 (typisiert)"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1},
           {"id": "concrete-dots", "title": t("Concrete stipple", "Betonpunktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1}]
HATCH100 = [{"id": "perforation", "title": t("Perforation (typified)", "Lochung (typisiert)"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}]

PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7},
        100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
        200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
        500: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}}
CHAIN_PEN = {50: 0.18, 100: 0.13, 200: 0.13, 500: 0.13}
RING_PEN = {50: 0.36, 100: 0.18, 200: 0.13, 500: 0.13}


def geometry(scale):
    """🖨️ The three views of one scale from its parts plus the hand-drawn traces."""
    parts = PARTS[scale]
    views = {}
    for view in ("elevation", "front", "plan"):
        shapes = []
        for s in parts:
            if s["bone"] == "net" and s["kind"] == "capsule":
                continue
            if s["kind"] == "capsule" and s.get("slender"):
                shapes.append(strut_stroke(s, view, PENS[scale]["body"] if scale == 500 else 0.25, base_z(s, view)))
                continue
            if view == "plan" and s["id"] == "ring-rod":
                shapes.append(ring_plan(s, RING_PEN[scale], base_z(s, view)))
                continue
            if scale == 500 and s["id"] == "ring-rod":
                shapes.append(ring_band(s, view, 0.025, base_z(s, view)))
                continue
            hatch = None
            if s["id"] in ("backboard-plate", "backboard") and view == "front" and scale in (50, 100):
                hatch = "perforation"
            if s["id"] == "plinth" and view != "plan" and scale == 50:
                hatch = "concrete-dots"
            sh = shape_of(s, view, scale)
            if s["id"] == "net-cone":
                sh["filled"] = True
                sh["tone"] = 0.8
            if hatch:
                sh["hatch"] = hatch
            shapes.append(sh)
        chains = [s for s in parts if s["bone"] == "net" and s["kind"] == "capsule"]
        if chains:
            shapes.append(chain_strokes(chains, view, CHAIN_PEN[scale], "chain-net", 12.0 if view != "plan" else 4.0))
        if scale == 50:
            shapes += textures50(view)
        if scale == 100 and view == "front":
            shapes.append(path("shooting-square", "board", "M -0.295 3.059 L 0.295 3.059 L 0.295 3.509 L -0.295 3.509 Z", 0.92, "backboard", "steel-perforated", 9.0, pen=0.18))
        if scale == 100 and view == "elevation":
            shapes.append(tex("rust-foot", "pole", [(0.0, -POLE_RO), (0.0, POLE_RO), (0.22, POLE_RO), (0.22, -POLE_RO)], 0.42, "base-hinge", "steel-galvanised-rusty", 8.0, role="detail"))
        views[view] = shapes
    return views


def decisions(silhouette, retained, merged, omitted, thin, hatching, styles):
    """📝️ The eight decision texts of a representation."""
    return {"silhouette": silhouette, "retained": retained, "merged": merged, "omitted": omitted, "thinElements": thin, "hatching": hatching, "styles": styles,
            "overlays": t("Every shape carries one region (foundation-head, base-hinge, pole-coating, backboard, ring or chain-net) and its material, so the thermal view paints the hot coated post, the grey board, the ring and the net apart.", "Jede Form trägt eine Region (foundation-head, base-hinge, pole-coating, backboard, ring oder chain-net) und ihr Material, so dass die thermische Ansicht den heißen beschichteten Pfosten, das graue Brett, den Ring und das Netz getrennt färbt.")}


REP_DECISIONS = {
    50: decisions(
        t("Unit 41 x 76 mm on paper with its whole construction: the concrete head with base plate, four M20 nuts, the hinge with lugs, knuckle and pin, the foot flange with four ribs, the post with its cap, the offset arm with strut and end plate, the backboard edge-on with its frame in elevation and full face with frame-line square, tags and ball marks in front, the ring on its mount with two gussets, and the chain net in three diamond rows with the torn sector and the dangling chain end.", "Anlage 41 x 76 mm auf dem Papier mit ihrer ganzen Konstruktion: der Betonkopf mit Fußplatte, vier Muttern M20, das Gelenk mit Laschen, Gelenkauge und Bolzen, der Fußflansch mit vier Rippen, der Pfosten mit Kappe, der Ausleger mit Strebe und Kopfplatte, das Brett in der Ansicht von der Kante mit seinem Rahmen und in der Vorderansicht ganz mit Zielquadrat, Tags und Ballabdrücken, der Ring auf seiner Halterung mit zwei Knotenblechen und das Kettennetz in drei Rautenreihen mit dem gerissenen Feld und dem herabhängenden Kettenende."),
        t("Every plate thickness from 8 mm, the 18 mm ring rod at its width, the 2 degree bend of the ring, every chain of the net, the torn sector.", "Jede Blechdicke ab 8 mm, der 18-mm-Ringstab in seiner Breite, die 2-Grad-Biegung des Rings, jede Kette des Netzes, das gerissene Feld."),
        t("The frame bars and the arm end plate lie behind the board plate in the front view and are hidden; the strut is hidden in the front view by the post and in the plan by the arm.", "Rahmenstäbe und Kopfplatte liegen in der Vorderansicht hinter dem Blech und sind verdeckt; die Strebe ist in der Vorderansicht vom Pfosten und in der Draufsicht vom Ausleger verdeckt."),
        t("The individual 10 mm perforations (typified by a dot hatch), the chain links (each chain is one stroke), bolt threads and the 12 net hooks.", "Die einzelnen 10-mm-Löcher (durch eine Punktschraffur typisiert), die Kettenglieder (jede Kette ist ein Strich), Gewinde und die 12 Netzhaken."),
        t("The chains are 0.18 mm strokes on their centre lines (8 mm chain), the ring in plan an outline 0.36 mm wide on the rod's centre line, the strut a solid outline; the 3 mm plate is drawn edge-on as part of the board strip.", "Die Ketten sind 0,18-mm-Striche auf ihren Mittellinien (8-mm-Kette), der Ring in der Draufsicht ein 0,36 mm breiter Umriss auf der Stabmittellinie, die Strebe ein voller Umriss; das 3-mm-Blech ist von der Kante als Teil des Brettstreifens gezeichnet."),
        t("Perforation as dots every 0.6 mm on the board face; concrete stipple on the foundation head.", "Lochung als Punkte alle 0,6 mm auf der Brettfläche; Betonpunktierung auf dem Fundamentkopf."),
        t("Coated post dark 0.30, rusty hinge 0.36, board light 0.66, ring 0.48, chains 0.50, concrete 0.62; stickers pale, tags near black.", "Beschichteter Pfosten dunkel 0,30, rostiges Gelenk 0,36, Brett hell 0,66, Ring 0,48, Ketten 0,50, Beton 0,62; Aufkleber hell, Tags fast schwarz.")),
    100: decisions(
        t("Unit 21 x 38 mm: the head with base plate and hinge lugs, the foot flange with the knuckle, the capped post, arm, strut and end plate, the board with frame posts, the ring on one bracket and the diamond chain net with its torn sector.", "Anlage 21 x 38 mm: der Kopf mit Fußplatte und Gelenklaschen, der Fußflansch mit Gelenkauge, der geschlossene Pfosten, Ausleger, Strebe und Kopfplatte, das Brett mit Rahmenpfosten, der Ring auf einer Konsole und das Rauten-Kettennetz mit seinem gerissenen Feld."),
        t("Hinge, overhang, the ring's bend, the net's diamonds and the torn sector.", "Gelenk, Ausladung, die Biegung des Rings, die Rauten des Netzes und das gerissene Feld."),
        t("Mount plate and the two 8 mm gussets become one ring bracket; the 5 mm cap merges into the closed post top.", "Halterungsplatte und die zwei 8-mm-Knotenbleche werden eine Ringkonsole; die 5-mm-Kappe geht im geschlossenen Pfostenkopf auf."),
        t("Nuts, ribs, pin, stickers, tags and ball marks (below 0.15 mm or meaningless at this size).", "Muttern, Rippen, Bolzen, Aufkleber, Tags und Ballabdrücke (unter 0,15 mm oder bei dieser Größe bedeutungslos)."),
        t("Chains 0.13 mm strokes, the ring in plan a 0.18 mm outline.", "Ketten als 0,13-mm-Striche, der Ring in der Draufsicht als 0,18-mm-Umriss."),
        t("Perforation dots on the board face only.", "Lochungspunkte nur auf der Brettfläche."),
        t("Same tones as 1:50.", "Gleiche Töne wie 1:50.")),
    200: decisions(
        t("Unit 10 x 19 mm: one footing block, the post foot with the hinge knuckle merged, the post, arm and strut, the board as plate and frame, the ring on its bracket and the net as twelve strands with one torn short.", "Anlage 10 x 19 mm: ein Fußblock, der Pfostenfuß mit eingeschlossenem Gelenkauge, Pfosten, Ausleger und Strebe, das Brett als Blech und Rahmen, der Ring auf seiner Konsole und das Netz als zwölf Stränge, einer kurz abgerissen."),
        t("Post, overhang, board, ring height and bend, net length and its torn strand.", "Pfosten, Ausladung, Brett, Ringhöhe und Biegung, Netzlänge und sein abgerissener Strang."),
        t("Concrete head and base plate one block; flange and knuckle one foot block; the board frame one block behind the plate; the diamonds of the net straightened into twelve strands.", "Betonkopf und Fußplatte ein Block; Flansch und Gelenkauge ein Fußblock; der Brettrahmen ein Block hinter dem Blech; die Rauten des Netzes zu zwölf Strängen gestreckt."),
        t("Lugs, pin, nuts, end plate, the separate frame posts and every texture.", "Laschen, Bolzen, Muttern, Kopfplatte, die einzelnen Rahmenpfosten und jede Textur."),
        t("Strands and ring outline 0.13 mm.", "Stränge und Ringumriss 0,13 mm."),
        t("No hatches.", "Keine Schraffuren."),
        t("Same tones as 1:50.", "Gleiche Töne wie 1:50.")),
    500: decisions(
        t("Unit 4 x 7.6 mm: footing block, post, arm with the strut as a line, the board as one slab, the ring on its bracket and the net as a pale cone.", "Anlage 4 x 7,6 mm: Fußblock, Pfosten, Ausleger mit der Strebe als Linie, das Brett als eine Platte, der Ring auf seiner Konsole und das Netz als heller Kegel."),
        t("Height, overhang, board size, ring height and the net.", "Höhe, Ausladung, Brettgröße, Ringhöhe und das Netz."),
        t("Board plate and frame one 43 mm slab; the twelve chains one cone envelope.", "Brettblech und Rahmen eine 43-mm-Platte; die zwölf Ketten eine Kegelhülle."),
        t("Hinge, flange, plates and all traces of use (below 0.15 mm).", "Gelenk, Flansch, Bleche und alle Gebrauchsspuren (unter 0,15 mm)."),
        t("Strut and ring are 0.13-0.18 mm lines.", "Strebe und Ring sind 0,13-0,18-mm-Linien."),
        t("No hatches.", "Keine Schraffuren."),
        t("Same tones as 1:50, the net cone pale 0.8.", "Gleiche Töne wie 1:50, der Netzkegel hell 0,8.")),
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def representation(scale):
    """🖨️ One authored representation."""
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": RANGES[scale][0], "maxDenominator": RANGES[scale][1]},
            "pens": PENS[scale], "hatches": HATCH50 if scale == 50 else HATCH100 if scale == 100 else [],
            "geometry": geometry(scale), "decisions": REP_DECISIONS[scale]}


def th_refs(parts, scale):
    """🌡️ The thermal list: every body solid as the surface of its region (the foundation head's underside continues into the ground)."""
    out = []
    for s in parts:
        ref = {"id": f"th-{s['id']}", "of": s["id"], "region": REG[s["material"]]}
        if s["id"] in ("plinth", "footing-block"):
            ref["sectionFaces"] = ["-v"]
        out.append(ref)
    return out


MEMBER_IDS = {"plinth": "footing", "footing-block": "footing", "pole-tube": "pole", "arm-tube": "arm", "arm-strut": "arm", "frame-bottom": "board", "frame-top": "board", "frame-side": "board", "frame-mid": "board", "board-frame": "board", "backboard": "board", "ring-rod": "ring"}
JOINT_IDS = {"base-plate": "footing", "ring-gusset": "ring", "hinge-pin": "pole", "hinge-knuckle": "pole", "hinge-lug": "footing", "foot-flange": "pole", "pole-foot": "pole", "arm-end-plate": "board", "ring-mount": "ring", "ring-bracket": "ring", "anchor-nut": "footing", "anchor-nut-back": "footing"}
MASS_R = {50: 0.03, 100: 0.03, 200: 0.035, 500: 0.04}


def structure_list(parts, scale):
    """🏗️ The structural list: members along their bones, joints at the hinge, plates and mounts, the chains as the net member, masses at the centres."""
    out = []
    for s in parts:
        if s["id"] in MEMBER_IDS:
            out.append({"id": f"m-{s['id']}", "of": s["id"], "member": MEMBER_IDS[s["id"]], "part": "member"})
        elif s["id"] in JOINT_IDS:
            out.append({"id": f"j-{s['id']}", "of": s["id"], "member": JOINT_IDS[s["id"]], "part": "joint"})
        elif s["bone"] == "net":
            out.append({"id": f"m-{s['id']}", "of": s["id"], "member": "net", "part": "member"})
    weld_r = ARM_RO + 0.006
    if scale in (50, 100):
        out.append(revolve("j-arm-weld", "arm", "steel-powder-coated", [(POLE_RO - 0.004, ARM_RO), (POLE_RO + 0.012, ARM_RO), (POLE_RO - 0.004, weld_r), (POLE_RO - 0.004, ARM_RO)] if scale == 50 else [(POLE_RO - 0.004, ARM_RO - 0.001), (POLE_RO + 0.02, ARM_RO - 0.001), (POLE_RO + 0.02, ARM_RO + 0.004), (POLE_RO - 0.004, ARM_RO + 0.004), (POLE_RO - 0.004, ARM_RO - 0.001)], [1, 0, 0], [0, 0, 0], role="detail", tone=0.25, member="arm", part="joint"))
    for b, (cu, cv) in COM.items():
        if b == "hinge" or (scale == 500 and b == "net"):
            continue
        (mu, mv, mw), rad = marker_at(b, scale)
        out.append(solid(f"mass-{b}", b, {"footing": "concrete-weathered", "pole": "steel-powder-coated", "arm": "steel-powder-coated", "board": "steel-perforated", "ring": "steel-ring-painted", "net": "chain-galvanised"}[b], "ellipsoid", role="detail", tone=0.3,
                         radii=[rad, rad, rad], transform={"translation": [r(mu), r(mv), r(mw)]}, member=b, part="mass"))
    return out


def marker_at(bone, scale):
    """⚖️ Where the mass marker of a bone sits and how large it is: at the true centre where that lies in material; on the arm tube, the ring rod and a net strand at the height and reach of the centre where the centre lies in the open (between tube and strut, in the ring's void, inside the net)."""
    cu, cv = COM[bone]
    big = MASS_R[scale]
    if bone == "arm":
        return (cu, 0.0, 0.0), min(big, 0.03) if scale < 500 else 0.0375
    if bone == "board":
        return (cu, min(cv, FR / 2), 0.0), {50: 0.018, 100: 0.018, 200: 0.02, 500: 0.03}[scale]
    if bone == "ring":
        return (RING_CU, 0.0, RING_RC), {50: 0.008, 100: 0.008, 200: 0.015, 500: 0.0375}[scale]
    if bone == "net":
        u = TIERS[1][0] if scale in (50, 100) else cu
        rad_at = TIERS[1][1] if scale in (50, 100) else TIERS[0][1] + (TIERS[-1][1] - TIERS[0][1]) * (cu - TIERS[0][0]) / (TIERS[-1][0] - TIERS[0][0])
        th = math.radians(15) if scale in (50, 100) else 0.0
        return (u, rad_at * math.cos(th), rad_at * math.sin(th)), {50: 0.008, 100: 0.008, 200: 0.015}[scale]
    return (cu, cv, 0.0), big


VOL_DECISIONS = {
    50: [t("Body, authored for 7.5 mm: the concrete head 400 x 60 mm with eased edges, base plate 340 x 340 x 15 with four hexagon nuts M20, the hinge (two lugs on the base plate, the knuckle on the foot flange, pin 16 mm), flange 200 x 200 x 15 with four ribs, the post as a hollow CHS 114.3 x 4.0 closed by flange and a 5 mm cap sheet, arm CHS 76.1 x 3.6 hollow with a 10 mm end plate sheet, the strut 48.3 mm, the 3 mm perforated plate declared as a sheet on its frame of 40 x 40 bars with two mid posts, the ring mount plate with two 8 mm gussets, the ring as an 18 mm hoop (slender), bent 2 degrees down, and the net as 64 chains of 8 mm (slender capsules) in three diamond rows with two diamonds of the lowest row torn out and one chain end dangling.", "Körper, für 7,5 mm gebaut: der Betonkopf 400 x 60 mm mit gebrochenen Kanten, Fußplatte 340 x 340 x 15 mit vier Sechskantmuttern M20, das Gelenk (zwei Laschen auf der Fußplatte, das Gelenkauge am Fußflansch, Bolzen 16 mm), Flansch 200 x 200 x 15 mit vier Rippen, der Pfosten als hohles Rundrohr 114,3 x 4,0, geschlossen durch Flansch und eine 5-mm-Kappe als Blech, Ausleger Rundrohr 76,1 x 3,6 hohl mit einer 10-mm-Kopfplatte als Blech, die Strebe 48,3 mm, das 3-mm-Lochblech als Blech deklariert auf seinem Rahmen aus Stäben 40 x 40 mit zwei Mittelpfosten, die Ringhalterung mit zwei 8-mm-Knotenblechen, der Ring als 18-mm-Reif (schlank), 2 Grad nach unten gebogen, und das Netz als 64 Ketten von 8 mm (schlanke Kapseln) in drei Rautenreihen, zwei Rauten der untersten Reihe ausgerissen und ein Kettenende herabhängend."),
         t("Thermal: every body solid is the surface of its region (the foundation head's underside continues into the ground as a section face); no hidden regions, no emitters. Structure: footing, post, arm with strut, board frame, ring and net are members along their bones with their true sections; hinge pin, knuckle, lugs, flange, nuts, end plate, ring mount and the arm weld are the joints; supports are the pose contacts (foundation in service, hinge pin and crane sling when lowered); each bone carries its mass at its centre.", "Thermik: jeder Körper ist die Oberfläche seiner Region (die Unterseite des Fundamentkopfs setzt sich als Schnittfläche in den Boden fort); keine verdeckten Regionen, keine Emitter. Tragwerk: Fuß, Pfosten, Ausleger mit Strebe, Brettrahmen, Ring und Netz sind Glieder entlang ihrer Knochen mit ihren wahren Querschnitten; Gelenkbolzen, Gelenkauge, Laschen, Flansch, Muttern, Kopfplatte, Ringhalterung und die Auslegernaht sind die Gelenke; Auflager sind die Posenkontakte (Fundament im Einsatz, Gelenkbolzen und Kranschlinge beim Umlegen); jeder Knochen trägt seine Masse in seinem Schwerpunkt.")],
    100: [t("Body, authored anew for 15 mm: nuts, ribs and pin left out (the pin is inside the knuckle and lugs, which now touch it), the lugs 30 mm wide against the knuckle, the post closed at the top by its own wall instead of the 5 mm cap, the mount plate and gussets merged into one 160 mm ring bracket, the ring hoop with fewer segments; the chain net keeps its diamonds because each chain is a slender part at its true 8 mm.", "Körper, neu für 15 mm gebaut: Muttern, Rippen und Bolzen weggelassen (der Bolzen liegt in Gelenkauge und Laschen, die ihn nun berühren), die Laschen 30 mm breit am Gelenkauge, der Pfosten oben durch seine eigene Wand statt der 5-mm-Kappe geschlossen, Halterung und Knotenbleche zu einer 160 mm breiten Ringkonsole zusammengefasst, der Ringreif mit weniger Segmenten; das Kettennetz behält seine Rauten, weil jede Kette ein schlankes Teil in ihrer wahren Stärke von 8 mm ist."),
          t("Thermal: one surface per region as at 1:50. Structure: members as at 1:50; knuckle, lugs, flange, end plate and ring bracket as joints; masses at the centres.", "Thermik: eine Fläche je Region wie bei 1:50. Tragwerk: Glieder wie bei 1:50; Gelenkauge, Laschen, Flansch, Kopfplatte und Ringkonsole als Gelenke; Massen in den Schwerpunkten.")],
    200: [t("Body, authored anew for 30 mm: concrete head and base plate as one 75 mm block, flange and knuckle as one 30 mm foot block on the post, post, arm and end plate as solid round bars (the bores are not shown at this scale), the board as the 3 mm plate sheet and one 40 mm frame block, the ring bracket as one block, and the net as twelve straight slender strands from the ring to the lower rim with one strand torn short.", "Körper, neu für 30 mm gebaut: Betonkopf und Fußplatte als ein 75-mm-Block, Flansch und Gelenkauge als ein 30-mm-Fußblock am Pfosten, Pfosten, Ausleger und Kopfplatte als volle Rundstäbe (die Bohrungen zeigt dieser Maßstab nicht), das Brett als 3-mm-Blech und ein 40-mm-Rahmenblock, die Ringkonsole als ein Block und das Netz als zwölf gerade schlanke Stränge vom Ring zum unteren Rand, einer kurz abgerissen."),
          t("Thermal: one surface per region. Structure: footing block, post, arm and strut, frame block, ring and strands as members; foot block and ring bracket as joints; masses at the centres.", "Thermik: eine Fläche je Region. Tragwerk: Fußblock, Pfosten, Ausleger und Strebe, Rahmenblock, Ring und Stränge als Glieder; Fußblock und Ringkonsole als Gelenke; Massen in den Schwerpunkten.")],
    500: [t("Body, authored anew for 75 mm: the footing block, the post as one bar down to the base plate, the arm bar with the strut as a slender rod, the board as one 43 mm slab declared as a sheet (plate and frame merged), the ring bracket 75 mm, the ring hoop, and the net as one frustum marked as envelope of the twelve chains (the open mesh seen as a mass).", "Körper, neu für 75 mm gebaut: der Fußblock, der Pfosten als ein Stab bis zur Fußplatte, der Auslegerstab mit der Strebe als schlanker Stab, das Brett als eine 43-mm-Platte, als Blech deklariert (Blech und Rahmen zusammengefasst), die Ringkonsole 75 mm, der Ringreif und das Netz als ein Kegelstumpf, als Hülle der zwölf Ketten markiert (das offene Geflecht als Masse gesehen)."),
          t("Thermal: one surface per region (the net cone is an envelope and counts no area; only the total area is compared at this scale). Structure: footing, post, arm and strut, board slab, ring and the net envelope as members, the bracket as joint; masses at the centres of the heavy bones.", "Thermik: eine Fläche je Region (der Netzkegel ist eine Hülle und zählt keine Fläche; bei diesem Maßstab wird nur die Gesamtfläche verglichen). Tragwerk: Fuß, Pfosten, Ausleger und Strebe, Brettplatte, Ring und die Netzhülle als Glieder, die Konsole als Gelenk; Massen in den Schwerpunkten der schweren Knochen.")],
}


def volume(scale):
    """🧊️ One authored volume."""
    parts = PARTS[scale]
    return {"status": "authored", "denominator": scale, "body": parts, "thermal": th_refs(parts, scale), "structure": structure_list(parts, scale), "decisions": VOL_DECISIONS[scale]}


def bounds50():
    """📏️ World extents of the 1:50 body at rest."""
    xs, ys, zs = [], [], []
    for s in PARTS[50]:
        for p in samples(s):
            w = to_world(s["bone"], p)
            xs.append(w[0])
            ys.append(w[1])
            zs.append(w[2])
    return min(xs), max(xs), min(ys), max(ys), min(zs), max(zs)


B = bounds50()

ASSET = {
    "id": "used-basketball-hoop",
    "category": "objects",
    "title": t("Used Basketball Hoop", "Gebrauchter Basketballkorb"),
    "summary": SUMMARY,
    "tags": TAGS,
    "provenance": PROVENANCE,
    "dimensions": {"heightM": r(B[3] - B[2], 3), "widthM": r(B[1] - B[0], 3), "depthM": r(B[5] - B[4], 3), "massKg": r(M_TOTAL, 1)},
    "materials": MATERIALS,
    "rig": RIG,
    "anchors": ANCHORS,
    "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
    "presentation": {"defaultPose": "in-service", "defaultClip": "dunk", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

if __name__ == "__main__":
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("[DEBUG] bones", {k: (tuple(round(c, 4) for c in v[0]), v[1]) for k, v in BONES_REST.items()})
    print("[DEBUG] masses", {k: round(v, 2) for k, v in BONE_MASS.items()}, "total", round(M_TOTAL, 2), "chain m", round(NET_LEN, 2))
    print("[DEBUG] bounds x %.3f..%.3f y %.3f..%.3f z %.3f..%.3f" % B)
    print("[DEBUG] ring centre", RING_C_WORLD, "dunk N", DUNK_N, "wind N", WIND_N)


if __name__ == "__main__":
    # the silhouette repair of batch R2-O, the last step: ops/used-basketball-hoop.json of the umbrella ticket of 2026-11-22
    import importlib.util as _util
    from pathlib import Path as _Path
    _spec = _util.spec_from_file_location("silhouette_r2o_lib", "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️silhouette/R2-O/lib.py")
    _r2o = _util.module_from_spec(_spec)
    _spec.loader.exec_module(_r2o)
    _r2o.repair_file(_Path(__file__).resolve().parent / "🔣️.json", "used-basketball-hoop")
