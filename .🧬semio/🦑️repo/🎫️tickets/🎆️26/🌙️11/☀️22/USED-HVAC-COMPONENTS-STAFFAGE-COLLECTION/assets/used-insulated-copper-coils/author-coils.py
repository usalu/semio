"""🌀 Authoritative generator of the staffage asset `used-insulated-copper-coils`.

A used pre-insulated twin refrigerant line set, copper 1/4 inch liquid line and 3/8 inch suction line each in 13 mm elastomeric
foam, about 5 m long, recovered, cut off with crushed copper ends and re-coiled by hand into a loose flat coil that stands on
edge between two battens on an 800 x 600 mm half pallet; lifted on a hook through the coil eye; reinstalled as the line set
between the outdoor and the indoor unit of a split air conditioner. The coil is a chain of 39 rigid 130 mm segments on revolute
joints, so the same bones are the coil and the routed line. Every hand-chosen number is stated here; running the script
rewrites `🔣️.json` next to it.

@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""

import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
SLUG = "used-insulated-copper-coils"

S, N = 0.13, 39
R0, PITCH = 0.35, 0.046
D_L, D_S, T_CU, T_FOAM = 0.00635, 0.00952, 0.0008, 0.013
RC_L, RC_S = D_L / 2, D_S / 2
R_L, R_S = RC_L + T_FOAM, RC_S + T_FOAM
Z_L, Z_S = R_L, -R_S
STUB_L, STUB_S = 0.16, 0.20
DECK = 0.144
COIL_X = 0.40
PAL_L, PAL_W = 0.80, 0.60
BOARD_T, BLOCK_H = 0.022, 0.078
TOP_W, BLOCK_X, BLOCK_Z = 0.145, 0.145, 0.10
ROW_X = [0.0725, 0.40, 0.7275]
TOP_Z = [-0.2275, 0.0, 0.2275]
FOOT_Z = [-0.25, 0.0, 0.25]
BAT_L, BAT_W, BAT_H = 0.30, 0.04, 0.05
BAT_Z = {"batten-left": round(Z_L + R_L + BAT_W / 2, 6), "batten-right": round(Z_S - R_S - BAT_W / 2, 6)}
RHO_CU, RHO_FOAM, RHO_WOOD, RHO_R32 = 8940.0, 55.0, 450.0, 893.0
K_FOAM, K_CU = 0.036, 340.0
G = 9.81
TIES = [2, 6, 10, 14, 18, 22, 26, 30, 34]
CREASES = [4, 12, 20, 29]
HOIST_UP, LIFT_UP, TRAVERSE = 0.288, 0.5, 1.0
TAIL_BEND = 60.0
ROUTE = [0.0] * 6 + [30.0, 60.0] + [90.0] * 12 + [60.0, 30.0] + [0.0] * 17
CLAMPS = [9, 13, 17, 25, 31, 36]
TOUCHES = [(35, 22), (22, 34), (34, 21)]


def t(en, de):
    """📝 One bilingual text."""
    return {"en": en, "de": de}


def dez(x):
    """🇩🇪 A number with a decimal comma."""
    return str(x).replace(".", ",")


def r6(x):
    """🎯 Round to micrometres."""
    return round(x, 6)


def wrap(a):
    """🔄 An angle in degrees folded into (-180, 180]."""
    while a <= -180:
        a += 360
    while a > 180:
        a -= 360
    return a


def spiral():
    """🌀 Vertices of the flat coil: chords of length S on a spiral that shrinks by PITCH per turn, outer end first."""
    def at(p):
        r = R0 - PITCH * (p - p0) / (2 * math.pi)
        return (r * math.sin(p), -r * math.cos(p))
    p0 = -math.asin(S / (2 * R0))
    pts, p = [at(p0)], p0
    for _ in range(N):
        lo, hi = p, p + 1.5
        a = pts[-1]
        for _ in range(80):
            mid = (lo + hi) / 2
            b = at(mid)
            if math.dist(a, b) < S:
                lo = mid
            else:
                hi = mid
        p = (lo + hi) / 2
        pts.append(at(p))
    a0 = math.atan2(pts[1][1] - pts[0][1], pts[1][0] - pts[0][0])
    c, s_ = math.cos(-a0), math.sin(-a0)
    pts = [(x * c - y * s_, x * s_ + y * c) for x, y in pts]
    mx, my = (pts[0][0] + pts[1][0]) / 2, (pts[0][1] + pts[1][1]) / 2
    return [(x - mx + COIL_X, y - my + DECK + R_S) for x, y in pts], (COIL_X - mx, DECK + R_S - my)


VERTS, CENTRE = spiral()
DIRS = [math.degrees(math.atan2(VERTS[k + 1][1] - VERTS[k][1], VERTS[k + 1][0] - VERTS[k][0])) for k in range(N)]
REL = [DIRS[0]] + [wrap(DIRS[k] - DIRS[k - 1]) for k in range(1, N)]
ROUTE_REL = [ROUTE[0]] + [ROUTE[k] - ROUTE[k - 1] for k in range(1, N)]
OFFSETS = [r6(wrap(ROUTE_REL[k] - REL[k])) for k in range(N)]
SEG = [f"seg-{k:02d}" for k in range(N)]
ENDS = ((SEG[0], 0.0, -1.0, 0.0), (SEG[-1], S, math.cos(math.radians(TAIL_BEND)), math.sin(math.radians(TAIL_BEND))))


def touch_strip(knee, chord, width=0.008):
    """🤝 Where the knee of one turn lies against the next turn (3 mm of air or less, which the soft foam closes under the ties): a strip from the knee centre to 1 mm inside the foam of the other turn, in the frame of the knee's segment."""
    (px, py), (ax, ay), (bx, by) = VERTS[knee], VERTS[chord], VERTS[chord + 1]
    ex, ey = bx - ax, by - ay
    f = max(0.0, min(1.0, ((px - ax) * ex + (py - ay) * ey) / (ex * ex + ey * ey)))
    nx, ny = ax + f * ex - px, ay + f * ey - py
    dist = math.hypot(nx, ny)
    reach = dist - R_S + 0.001
    a = math.radians(DIRS[knee])
    ux, uy = (nx * math.cos(a) + ny * math.sin(a)) / dist, (-nx * math.sin(a) + ny * math.cos(a)) / dist
    h = width / 2
    return [(-uy * h, ux * h), (ux * reach - uy * h, uy * reach + ux * h), (ux * reach + uy * h, uy * reach - ux * h), (uy * h, -ux * h)]


def tail_pt(end, along, across):
    """📌 A point at `along` the tail of an end and `across` it, in the frame of the end segment."""
    b, a, dx, dy = end
    return (a + along * dx - across * dy, along * dy + across * dx)



def seg_min_gap():
    """📏 Smallest centre-line distance between two non-neighbouring segments of the coil (must exceed 2 R_S)."""
    def d(p, q, a, b):
        best = 1e9
        for i in range(11):
            x = (p[0] + (q[0] - p[0]) * i / 10, p[1] + (q[1] - p[1]) * i / 10)
            for j in range(11):
                y = (a[0] + (b[0] - a[0]) * j / 10, a[1] + (b[1] - a[1]) * j / 10)
                best = min(best, math.dist(x, y))
        return best
    return min(d(VERTS[i], VERTS[i + 1], VERTS[j], VERTS[j + 1]) for i in range(N) for j in range(i + 2, N))


def tube_kg_m(rc, t_):
    """⚖️ Mass per metre of a copper tube of outer radius rc and wall t_."""
    return math.pi * (rc ** 2 - (rc - t_) ** 2) * RHO_CU


def foam_kg_m(rc, rf):
    """⚖️ Mass per metre of a foam sleeve from rc to rf."""
    return math.pi * (rf ** 2 - rc ** 2) * RHO_FOAM


CU_L_KGM, CU_S_KGM = tube_kg_m(RC_L, T_CU), tube_kg_m(RC_S, T_CU)
FOAM_L_KGM, FOAM_S_KGM = foam_kg_m(RC_L, R_L), foam_kg_m(RC_S, R_S)
LINE_KGM = CU_L_KGM + CU_S_KGM + FOAM_L_KGM + FOAM_S_KGM
SEG_KG = round(LINE_KGM * S, 5)
STUB_KG = round(STUB_L * CU_L_KGM + STUB_S * CU_S_KGM, 5)
PAL_PARTS_V = 3 * PAL_L * BOARD_T * TOP_W + 3 * BOARD_T * BLOCK_X * PAL_W + 9 * BLOCK_X * BLOCK_H * BLOCK_Z + 3 * PAL_L * BOARD_T * BLOCK_Z
BAT_V = 2 * BAT_L * BAT_W * BAT_H
PAL_KG = round((PAL_PARTS_V + BAT_V) * RHO_WOOD + 0.25, 3)
END_KG = round(SEG_KG + STUB_KG, 5)
LINE_KG = round(N * SEG_KG + 2 * STUB_KG, 4)
MASS = round(PAL_KG + (N - 2) * SEG_KG + 2 * END_KG, 3)
R32_KG = round(math.pi * (RC_L - T_CU) ** 2 * N * S * RHO_R32, 4)
LENGTH = round(N * S, 3)
TOP_Y = max(y for _, y in VERTS) + R_S
HEIGHT = r6(TOP_Y)
WIDTH = PAL_L
DEPTH = PAL_W
COIL_OD = r6(2 * (R0 + R_S))
LIQ_AREA = round(2 * math.pi * R_L * LENGTH, 3)
SUC_AREA = round(2 * math.pi * R_S * LENGTH, 3)
CU_AREA = round(2 * math.pi * (RC_L * STUB_L + RC_S * STUB_S) * 2 + 4 * math.pi * RC_S ** 2, 4)


def end_com():
    """⚖️ Centre of mass along u of the outer and inner end segments with their copper tails."""
    seg = SEG_KG * S / 2
    outer = (seg - STUB_KG * 0.09) / END_KG
    inner = (seg + STUB_KG * (S + 0.09 * math.cos(math.radians(TAIL_BEND)))) / END_KG
    return r6(outer), r6(inner), r6(STUB_KG * 0.09 * math.sin(math.radians(TAIL_BEND)) / END_KG)


COM_OUT, COM_IN, COM_IN_Y = end_com()

MATERIALS = [
    {"id": "elastomer-foam", "title": t("Flexible elastomeric foam 13 mm, black, UV-aged and soiled", "Synthesekautschuk-Dämmschlauch 13 mm, schwarz, UV-gealtert und verschmutzt"), "emissivity": 0.9, "solarAbsorptivity": 0.92, "densityKgM3": RHO_FOAM, "conductivityWmK": K_FOAM, "specificHeatJKgK": 1500, "elasticModulusPa": 0.3e6, "source": "literature", "reference": "en-14304"},
    {"id": "copper-tube", "title": t("Copper tube Cu-DHP R220, annealed, inside the insulation", "Kupferrohr Cu-DHP R220, weichgeglüht, in der Dämmung"), "emissivity": 0.05, "solarAbsorptivity": 0.3, "densityKgM3": RHO_CU, "conductivityWmK": K_CU, "specificHeatJKgK": 385, "elasticModulusPa": 120e9, "strengthPa": 220e6, "source": "literature", "reference": "en-12735-1"},
    {"id": "copper-oxidised", "title": t("Copper tube ends, oxidised and crushed", "Kupferrohrenden, oxidiert und gequetscht"), "emissivity": 0.6, "solarAbsorptivity": 0.65, "densityKgM3": RHO_CU, "conductivityWmK": K_CU, "specificHeatJKgK": 385, "elasticModulusPa": 120e9, "strengthPa": 220e6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "pallet-spruce", "title": t("Pallet spruce, rough sawn, weathered grey", "Palettenfichte, sägerau, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": RHO_WOOD, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "en-338"},
]

REFERENCES = [
    {"id": "en-12735-1", "citation": "EN 12735-1:2020 Copper and copper alloys, seamless round tubes for air conditioning and refrigeration, part 1: tubes for piping systems (Cu-DHP, R220 annealed in coils; 1/4 inch 6.35 x 0.8 mm and 3/8 inch 9.52 x 0.8 mm, tensile strength at least 220 MPa)."},
    {"id": "en-14304", "citation": "EN 14304:2015 Thermal insulation products for building equipment and industrial installations, factory made flexible elastomeric foam (FEF) products (declared conductivity about 0.033 to 0.036 W/mK at 0 degC, density 40 to 70 kg/m3)."},
    {"id": "line-set-datasheets", "citation": "Manufacturer data sheets of pre-insulated twin refrigerant line sets for split air conditioners and split heat pumps (for example Armacell Armaflex, Kaiflex, Sanha line sets): 1/4 + 3/8 inch copper, each tube in 9 to 13 mm elastomeric foam, coils of 5, 10, 15 and 25 m; the values here are typical of the class, not of one product (authored-estimate)."},
    {"id": "en-378-2", "citation": "EN 378-2:2016 Refrigerating systems and heat pumps, safety and environmental requirements, part 2: design, construction, testing, marking and documentation (piping, flare joints, support of refrigerant lines)."},
    {"id": "f-gas-2024", "citation": "Regulation (EU) 2024/573 on fluorinated greenhouse gases (recovery of the refrigerant by a certified person before a line is opened or dismantled)."},
    {"id": "vdi-2055-1", "citation": "VDI 2055 Blatt 1:2019 Thermal insulation of heated and refrigerated operational installations in the industry and the building services (thermal resistance of a cylindrical insulation layer r_o ln(r_o / r_i) / lambda referred to the outer surface)."},
    {"id": "refprop-r32", "citation": "Lemmon, Bell, Huber, McLinden: NIST REFPROP 10 (2018), R32 (difluoromethane) saturated liquid density about 893 kg/m3 at 40 degC; cooling design of a small split unit about 8 to 10 degC evaporating and 42 to 45 degC condensing."},
    {"id": "en-13698-1", "citation": "EN 13698-1:2003 Pallet production specification, part 1: construction specification for 800 mm x 1200 mm flat wooden pallets (boards 22 mm, blocks 145 x 78 x 100 mm, height 144 mm); the 800 x 600 mm half pallet used here is built from the same boards and blocks (authored-estimate)."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes (C24: f_m,k 24 MPa, E0,mean 11 GPa; 450 kg/m3 taken for pallet spruce at about 18 % moisture)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (softwood 0.13 W/mK, 1600 J/kgK)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, Table 5 emissivities and solar absorptances: oxidised copper about 0.6, polished copper about 0.05, black rubber about 0.9; weathered and soiled values differ from new surfaces."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types, contents and basic principles of presentation (scales, line groups)."},
]


def bone_seg(k):
    """🦴 One rigid segment of the line set."""
    first, last = k == 0, k == N - 1
    en = "outer end with the outdoor-unit tails" if first else ("inner end with the indoor-unit tails" if last else f"segment {k + 1} of {N}")
    de = "äußeres Ende mit den Außengeräte-Enden" if first else ("inneres Ende mit den Innengeräte-Enden" if last else f"Abschnitt {k + 1} von {N}")
    lim = max(30.0, math.ceil(abs(OFFSETS[k]) + 10))
    b = {"id": SEG[k], "parent": "traverse" if first else SEG[k - 1], "title": t(f"Line set, {en}", f"Leitungspaar, {de}"),
         "x": r6(VERTS[0][0]) if first else S, "y": r6(VERTS[0][1]) if first else 0, "angleDeg": r6(REL[k]), "lengthM": S,
         "joint": {"type": "revolute", "minDeg": -min(lim, 180), "maxDeg": min(lim, 180)},
         "mass": {"kg": END_KG if first or last else SEG_KG, "comX": COM_OUT if first else (COM_IN if last else r6(S / 2)), "comY": COM_IN_Y if last else 0, "distribution": "point" if first or last else "uniform", "source": "literature", "reference": "en-12735-1"},
         "section": SECTION}
    return b


SECTION = {"areaM2": r6(math.pi * (RC_L ** 2 - (RC_L - T_CU) ** 2) + math.pi * (RC_S ** 2 - (RC_S - T_CU) ** 2)),
           "secondMomentM4": float(f"{math.pi / 4 * (RC_L ** 4 - (RC_L - T_CU) ** 4) + math.pi / 4 * (RC_S ** 4 - (RC_S - T_CU) ** 4):.4e}"),
           "extremeFibreM": r6(RC_S), "material": "copper-tube", "title": t("Two copper tubes 6.35 x 0.8 and 9.52 x 0.8 mm", "Zwei Kupferrohre 6,35 x 0,8 und 9,52 x 0,8 mm")}

BONES = [
    {"id": "yard", "parent": None, "title": t("Store floor (frame of the lot)", "Lagerboden (Bezugsrahmen des Postens)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "pallet", "parent": "yard", "title": t("Half pallet 800 x 600 mm with the two battens", "Halbpalette 800 x 600 mm mit den beiden Leisten"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": PAL_L, "joint": {"type": "fixed"},
     "mass": {"kg": PAL_KG, "comX": 0.4, "comY": 0.08, "distribution": "point", "source": "authored-estimate", "reference": "en-13698-1"},
     "section": {"areaM2": r6(3 * TOP_W * BOARD_T), "secondMomentM4": float(f"{3 * TOP_W * BOARD_T ** 3 / 12:.4e}"), "extremeFibreM": BOARD_T / 2, "material": "pallet-spruce", "title": t("Three top boards 145 x 22 mm", "Drei Deckbretter 145 x 22 mm")}},
    {"id": "hoist", "parent": "yard", "title": t("Hoist (massless: the hook or the installer's hands)", "Hub (masselos: Haken oder Hände des Monteurs)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 0.6}},
    {"id": "traverse", "parent": "hoist", "title": t("Traverse (massless: carries the line set to its new place)", "Verfahrweg (masselos: trägt das Leitungspaar an seinen neuen Ort)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.2, "maxM": 1.2}},
] + [bone_seg(k) for k in range(N)]


def anchor(id_, bone, x, y, kind, en, de, z=None, **kw):
    """📍 One anchor."""
    a = {"id": id_, "bone": bone, "x": r6(x), "y": r6(y)}
    if z is not None:
        a["z"] = r6(z)
    a.update({"kind": kind, "title": t(en, de)})
    a.update(kw)
    return a


def hook_segment():
    """🪝 The segment of the innermost turn whose top lies nearest above the centre of the coil."""
    inner = range(N - 14, N)
    tops = [k for k in inner if 60 < wrap(DIRS[k]) % 360 and abs(wrap(DIRS[k]) - 180) < 30]
    return min(tops, key=lambda k: abs((VERTS[k][0] + VERTS[k + 1][0]) / 2 - COIL_X))


HOOK = hook_segment()
CONTACT = {"support": "contact", "normalDeg": 90, "frictionCoefficient": 0.5}
ANCHORS = []
for name, x in (("rear", 0.05), ("front", 0.75)):
    for side, z in (("left", 0.25), ("right", -0.25)):
        ANCHORS.append(anchor(f"ground-{name}-{side}", "pallet", x, 0, "support", f"Bottom board end on the floor, {name} {side}", f"Bodenbrettende auf dem Boden, {'hinten' if name == 'rear' else 'vorn'} {'links' if side == 'left' else 'rechts'}", z, footprintLengthM=0.1, footprintWidthM=0.1, **CONTACT))
ANCHORS.append(anchor("hook", SEG[HOOK], S / 2, R_S, "support", "Padded hook under the top of the innermost turn", "Gepolsterter Haken unter dem Scheitel der innersten Windung", 0, support="pin"))
ANCHORS.append(anchor("flare-outdoor", SEG[0], -STUB_S, 0, "support", "Flare nuts on the service valves of the outdoor unit", "Bördelmuttern an den Absperrventilen des Außengeräts", 0, support="pin"))
ANCHORS.append(anchor("flare-indoor", SEG[-1], *tail_pt(ENDS[1], STUB_S, 0),  "support", "Flare nuts on the connection tails of the indoor unit", "Bördelmuttern an den Anschlussenden des Innengeräts", 0, support="pin"))
for k in CLAMPS:
    ANCHORS.append(anchor(f"clamp-{k:02d}", SEG[k], S / 2, 0, "support", f"Line-set clamp on the wall at segment {k + 1}", f"Leitungsschelle an der Wand bei Abschnitt {k + 1}", 0, support="pin"))
GROUND = [a["id"] for a in ANCHORS if a["id"].startswith("ground")]
SERVICE = ["flare-outdoor", "flare-indoor"] + [f"clamp-{k:02d}" for k in CLAMPS]


def solid(id_, bone, material, tone, kind, **kw):
    """🧊 A solid."""
    s = {"id": id_, "kind": kind, "bone": bone, "material": material, "role": kw.pop("role", "body"), "tone": tone}
    s.update(kw)
    return s


def boxs(id_, bone, material, tone, size, at, **kw):
    """📦 A box solid centred at `at`."""
    return solid(id_, bone, material, tone, "box", size=[r6(x) for x in size], transform={"translation": [r6(x) for x in at]}, **kw)


def cyl(id_, bone, material, tone, a0, a1, radius, z, axis=(1, 0, 0), origin=None, **kw):
    """🥫 A filled cylinder as an open two-point revolve."""
    return solid(id_, bone, material, tone, "revolve", profile=[[r6(a0), r6(radius)], [r6(a1), r6(radius)]], axis=list(axis), origin=origin or [0, 0, r6(z)], **kw)


def annulus(id_, bone, material, tone, a0, a1, ri, ro, z, **kw):
    """⭕ A hollow tube as a closed-loop revolve."""
    return solid(id_, bone, material, tone, "revolve", profile=[[r6(a0), r6(ri)], [r6(a1), r6(ri)], [r6(a1), r6(ro)], [r6(a0), r6(ro)], [r6(a0), r6(ri)]], axis=[1, 0, 0], origin=[0, 0, r6(z)], **kw)


def ball(id_, bone, material, tone, radius, z, n=12, **kw):
    """🔵 A sphere as a revolved half circle."""
    prof = [[r6(-radius * math.cos(math.pi * i / n)), r6(radius * math.sin(math.pi * i / n))] for i in range(n + 1)]
    return solid(id_, bone, material, tone, "revolve", profile=prof, axis=[1, 0, 0], origin=[0, 0, r6(z)], **kw)


def ngon(rad, n, zc):
    """🔷 A regular n-gon of circumradius rad in the (v, w) plane about (0, zc)."""
    return [[r6(rad * math.cos(2 * math.pi * i / n)), r6(zc + rad * math.sin(2 * math.pi * i / n))] for i in range(n)]


TUBES = (("liquid", R_L, Z_L, RC_L, STUB_L, 0.2), ("suction", R_S, Z_S, RC_S, STUB_S, 0.16))
REGION = {"liquid": "liquid-line", "suction": "suction-line"}


DECK_HOLES = [(z0, z1, x0, x1) for z0, z1 in ((-0.155, -0.0725), (0.0725, 0.155)) for x0, x1 in ((ROW_X[0] + BLOCK_X / 2, ROW_X[1] - BLOCK_X / 2), (ROW_X[1] + BLOCK_X / 2, ROW_X[2] - BLOCK_X / 2))]


def pallet_bodies(scale):
    """🪵 The pallet and its battens at one scale."""
    out, w = [], "pallet-spruce"
    if scale in (50, 100):
        for i, z in enumerate(TOP_Z):
            out.append(boxs(f"top-board-{i + 1}", "pallet", w, 0.62, [PAL_L, BOARD_T, TOP_W], [PAL_L / 2, DECK - BOARD_T / 2, z], **({"cornerRadius": 0.002} if scale == 50 else {})))
        for i, z in enumerate(FOOT_Z):
            out.append(boxs(f"bottom-board-{i + 1}", "pallet", w, 0.55, [PAL_L, BOARD_T, BLOCK_Z], [PAL_L / 2, BOARD_T / 2, z]))
    if scale == 50:
        for i, x in enumerate(ROW_X):
            out.append(boxs(f"cross-board-{i + 1}", "pallet", w, 0.58, [BLOCK_X, BOARD_T, PAL_W], [x, BOARD_T + BLOCK_H + BOARD_T / 2, 0]))
            for j, z in enumerate(FOOT_Z):
                out.append(boxs(f"block-{i + 1}-{j + 1}", "pallet", w, 0.5, [BLOCK_X, BLOCK_H, BLOCK_Z], [x, BOARD_T + BLOCK_H / 2, z], cornerRadius=0.003))
    if scale == 100:
        for i, x in enumerate(ROW_X):
            y0, y1, y2 = BOARD_T, BOARD_T + BLOCK_H, BOARD_T + BLOCK_H + BOARD_T
            prof = [[y0, -0.3], [y0, -0.2], [y1, -0.2], [y1, -0.05], [y0, -0.05], [y0, 0.05], [y1, 0.05], [y1, 0.2], [y0, 0.2], [y0, 0.3], [y2, 0.3], [y2, -0.3]]
            out.append(solid(f"stringer-{i + 1}", "pallet", w, 0.55, "extrude", profile=[[r6(a), r6(b)] for a, b in prof], plane="yz", **{"from": r6(x - BLOCK_X / 2), "to": r6(x + BLOCK_X / 2)}))
    if scale == 200:
        out.append(solid("deck", "pallet", w, 0.62, "extrude", profile=[[-0.3, 0], [0.3, 0], [0.3, PAL_L], [-0.3, PAL_L]], holes=[[[r6(z0), r6(x0)], [r6(z1), r6(x0)], [r6(z1), r6(x1)], [r6(z0), r6(x1)]] for z0, z1, x0, x1 in DECK_HOLES], plane="zx", **{"from": r6(DECK - 2 * BOARD_T), "to": DECK}))
        for j, z in enumerate(FOOT_Z):
            y1 = BOARD_T + BLOCK_H
            prof = [[0, 0], [PAL_L, 0], [PAL_L, y1], [ROW_X[2] - BLOCK_X / 2, y1], [ROW_X[2] - BLOCK_X / 2, BOARD_T], [ROW_X[1] + BLOCK_X / 2, BOARD_T], [ROW_X[1] + BLOCK_X / 2, y1], [ROW_X[1] - BLOCK_X / 2, y1], [ROW_X[1] - BLOCK_X / 2, BOARD_T], [ROW_X[0] + BLOCK_X / 2, BOARD_T], [ROW_X[0] + BLOCK_X / 2, y1], [0, y1]]
            out.append(solid(f"runner-{j + 1}", "pallet", w, 0.55, "extrude", profile=[[r6(a), r6(b)] for a, b in prof], plane="xy", **{"from": r6(z - BLOCK_Z / 2), "to": r6(z + BLOCK_Z / 2)}))
    for bid, z in BAT_Z.items():
        out.append(boxs(bid, "pallet", w, 0.66, [BAT_L, BAT_H, BAT_W], [COIL_X, DECK + BAT_H / 2, z], **({"cornerRadius": 0.002} if scale == 50 else {})))
    return out


def line_bodies(scale):
    """🌀 The line set solids of one scale."""
    out = []
    for k in range(N):
        b = SEG[k]
        for name, rf, zc, rc, ln, tone in TUBES:
            if scale == 50:
                out.append(cyl(f"{b}-{name}", b, "elastomer-foam", tone, 0, S, rf, zc))
                if k > 0:
                    out.append(ball(f"{b}-{name}-knee", b, "elastomer-foam", tone, rf, zc))
            elif scale == 100:
                out.append(solid(f"{b}-{name}", b, "elastomer-foam", tone, "extrude", profile=ngon(rf / math.cos(math.pi / 16), 16, zc), plane="yz", **{"from": 0, "to": S}))
                if k > 0:
                    out.append(solid(f"{b}-{name}-knee", b, "elastomer-foam", tone, "ellipsoid", radii=[r6(rf), r6(rf), r6(rf)], transform={"translation": [0, 0, r6(zc)]}))
            else:
                out.append(boxs(f"{b}-{name}", b, "elastomer-foam", tone, [S, 2 * rf, 2 * rf], [S / 2, 0, zc]))
                if k > 0:
                    out.append(cyl(f"{b}-{name}-knee", b, "elastomer-foam", tone, zc - rf, zc + rf, rf, 0, axis=(0, 0, 1), origin=[0, 0, 0]))
    for b, a, dx, dy in ENDS:
        for name, rf, zc, rc, ln, tone in TUBES:
            kw = {} if (scale == 50 and name == "suction") else {"slender": True}
            out.append(cyl(f"{b}-{name}-tail", b, "copper-oxidised", 0.55, 0, ln, rc, zc, axis=(r6(dx), r6(dy), 0), origin=[r6(a), 0, r6(zc)], **kw))
    return out


def bodies(scale):
    """🧊 The body solids of one scale."""
    return pallet_bodies(scale) + line_bodies(scale)


def thermals(scale, body):
    """🌡️ The thermal solids of one scale."""
    out = []
    for s in body:
        sid = s["id"]
        if s["bone"] == "pallet":
            out.append({"id": f"th-{sid}", "of": sid, "region": "pallet"})
        elif sid.endswith("-tail"):
            out.append({"id": f"th-{sid}", "of": sid, "region": "copper-ends"})
        elif scale != 50:
            out.append({"id": f"th-{sid}", "of": sid, "region": REGION[sid.split("-")[2]]})
    if scale == 50:
        for k in range(N):
            b = SEG[k]
            for name, rf, zc, rc, ln, tone in TUBES:
                reg = REGION[name]
                out.append(annulus(f"th-{b}-{name}-foam", b, "elastomer-foam", tone, 0, S, rc, rf, zc, region=reg, layer=f"{name}-foam"))
                out.append(annulus(f"th-{b}-{name}-copper", b, "copper-tube", 0.55, 0, S, rc - T_CU, rc, zc, region=reg, layer=f"{name}-copper", slender=True))
                if k > 0:
                    out.append(dict(ball(f"th-{b}-{name}-knee", b, "elastomer-foam", tone, rf, zc), region=reg, layer=f"{name}-foam"))
    return out


def structures(scale, body):
    """🏗️ The structural solids of one scale."""
    out = []
    for s in body:
        sid = s["id"]
        if s["bone"] == "pallet":
            out.append({"id": f"st-{sid}", "of": sid, "member": "pallet", "part": "member"})
        elif sid.endswith("-knee"):
            out.append({"id": f"st-{sid}", "of": sid, "member": s["bone"], "part": "joint"})
        else:
            out.append({"id": f"st-{sid}", "of": sid, "member": s["bone"], "part": "member"})
    return out


def shape(id_, bone, role, z, filled, tone, geom, region=None, material=None, pen=None):
    """✏️ One 2D shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    s.update(geom)
    if pen is not None:
        s["penMm"] = pen
    return s


def rect(x, y, w, h):
    """▭ A rect geometry."""
    return {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}


def circ(cx, cy, rad):
    """⚪ A circle geometry."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rad), "ry": r6(rad)}


def poly(points, closed=False):
    """〰️ A polyline geometry."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in points], "closed": closed}


def member(x0, x1, z, d=None, w=None, h=None):
    """📊 A member geometry along the bone at lateral z."""
    sec = {"kind": "circle", "d": r6(d)} if d is not None else {"kind": "rect", "w": r6(w), "h": r6(h)}
    return {"kind": "member", "from": {"x": r6(x0), "y": 0, "z": r6(z)}, "to": {"x": r6(x1), "y": 0, "z": r6(z)}, "section": sec}


def tail_member(end, ln, z, **sec):
    """🔧 A member along the tail of an end."""
    m = member(0, 0, z, **sec)
    x1, y1 = tail_pt(end, ln, 0)
    m["from"] = {"x": r6(end[1]), "y": 0, "z": r6(z)}
    m["to"] = {"x": r6(x1), "y": r6(y1), "z": r6(z)}
    return m


def line_shapes(scale, view):
    """🌀 The line set in one view: one member per tube and segment, knee discs in the side elevation, the copper tails."""
    out = []
    for k in range(N):
        b = SEG[k]
        for name, rf, zc, rc, ln, tone in TUBES:
            zz = (2 if name == "suction" else 1) + k * 0.001
            geom = member(0, S, zc, w=2 * rf, h=2 * rf) if scale == 200 else member(0, S, zc, d=2 * rf)
            out.append(shape(f"{b}-{name}", b, "body", zz, True, tone, geom, region=REGION[name], material="elastomer-foam"))
            if view == "elevation" and k > 0:
                out.append(shape(f"{b}-{name}-knee", b, "body", zz + 0.0005, True, tone, circ(0, 0, rf), region=REGION[name], material="elastomer-foam"))
    for end in ENDS:
        b, a = end[0], end[1]
        if view != "elevation" and scale != 50:
            z0, z1 = Z_S - RC_S, Z_L + RC_L
            out.append(shape(f"{b}-tails", b, "body", 3, True, 0.55, tail_member(end, STUB_S, (z0 + z1) / 2, w=z1 - z0, h=D_S), region="copper-ends", material="copper-oxidised"))
            continue
        for name, rf, zc, rc, ln, tone in TUBES:
            if view == "elevation" and name == "liquid":
                continue
            out.append(shape(f"{b}-{name}-tail", b, "body", 3, True, 0.55, tail_member(end, ln, zc, d=2 * rc), region="copper-ends", material="copper-oxidised"))
    if view == "elevation" and scale == 50:
        for end in ENDS:
            b, a = end[0], end[1]
            out.append(shape(f"{b}-crushed-end", b, "detail", 3.2, False, 0.35, poly([tail_pt(end, STUB_S - 0.03, RC_S), tail_pt(end, STUB_S - 0.012, RC_S * 0.5), tail_pt(end, STUB_S - 0.012, -RC_S * 0.5), tail_pt(end, STUB_S - 0.03, -RC_S)]), region="copper-ends"))
            out.append(shape(f"{b}-foam-cut", b, "detail", 3.1, False, 0.3, poly([(a, -R_S), (a + 0.004, -R_S * 0.3), (a - 0.002, R_S * 0.4), (a, R_S)]), region="suction-line"))
        for k in TIES:
            b = SEG[k]
            out.append(shape(f"{b}-cable-tie", b, "detail", 4, True, 0.1, rect(S / 2 - 0.0024, -R_S - 0.0012, 0.0048, 2 * R_S + 0.0024), region="suction-line"))
            out.append(shape(f"{b}-tie-head", b, "detail", 4.1, True, 0.1, rect(S / 2 - 0.004, -R_S - 0.006, 0.008, 0.006), region="suction-line"))
        for k in CREASES:
            b = SEG[k]
            out.append(shape(f"{b}-crease", b, "texture", 4.2, False, 0.35, poly([(0.03, -0.01), (0.05, -0.004), (0.07, 0.002), (0.095, 0.006)]), region="suction-line"))
        lb = SEG[HOOK - 3]
        out.append(shape(f"{lb}-label", lb, "detail", 4.3, True, 0.92, rect(0.035, -0.012, 0.06, 0.024), region="suction-line"))
        out.append(shape(f"{lb}-label-text", lb, "texture", 4.4, False, 0.4, poly([(0.042, 0.002), (0.088, 0.002)]), region="suction-line"))
    if view == "elevation" and scale == 100:
        for k in TIES[::2]:
            b = SEG[k]
            out.append(shape(f"{b}-cable-tie", b, "texture", 4, False, 0.1, poly([(S / 2, -R_S), (S / 2, R_S)]), region="suction-line"))
        for knee, chord in TOUCHES:
            out.append(shape(f"{SEG[knee]}-turn-contact", SEG[knee], "body", 2 + knee * 0.001 + 0.0007, True, 0.16, poly(touch_strip(knee, chord), closed=True), region="suction-line", material="elastomer-foam"))
    return out


def pallet_elevation(scale):
    """🪵 Pallet and battens in the side elevation (seen from the right)."""
    w, out = "pallet-spruce", []
    if scale == 200:
        out.append(shape("deck", "pallet", "body", 0.5, True, 0.62, rect(0, DECK - 2 * BOARD_T, PAL_L, 2 * BOARD_T), region="pallet", material=w))
        out.append(shape("runner-1", "pallet", "body", 0.4, True, 0.55, poly([(0, 0), (PAL_L, 0), (PAL_L, BOARD_T + BLOCK_H), (ROW_X[2] - BLOCK_X / 2, BOARD_T + BLOCK_H), (ROW_X[2] - BLOCK_X / 2, BOARD_T), (ROW_X[1] + BLOCK_X / 2, BOARD_T), (ROW_X[1] + BLOCK_X / 2, BOARD_T + BLOCK_H), (ROW_X[1] - BLOCK_X / 2, BOARD_T + BLOCK_H), (ROW_X[1] - BLOCK_X / 2, BOARD_T), (ROW_X[0] + BLOCK_X / 2, BOARD_T), (ROW_X[0] + BLOCK_X / 2, BOARD_T + BLOCK_H), (0, BOARD_T + BLOCK_H)], True), region="pallet", material=w))
    else:
        out.append(shape("bottom-board-1", "pallet", "body", 0.4, True, 0.55, rect(0, 0, PAL_L, BOARD_T), region="pallet", material=w))
        for i, x in enumerate(ROW_X):
            out.append(shape(f"block-{i + 1}-1" if scale == 50 else f"stringer-{i + 1}", "pallet", "body", 0.45, True, 0.5, rect(x - BLOCK_X / 2, BOARD_T, BLOCK_X, BLOCK_H + (0 if scale == 50 else BOARD_T)), region="pallet", material=w))
            if scale == 50:
                out.append(shape(f"cross-board-{i + 1}", "pallet", "body", 0.46, True, 0.58, rect(x - BLOCK_X / 2, BOARD_T + BLOCK_H, BLOCK_X, BOARD_T), region="pallet", material=w))
        out.append(shape("top-board-1", "pallet", "body", 0.5, True, 0.62, rect(0, DECK - BOARD_T, PAL_L, BOARD_T), region="pallet", material=w))
        if scale == 50:
            for i, x in enumerate(ROW_X):
                out.append(shape(f"block-{i + 1}-grain", "pallet", "texture", 0.47, False, 0.35, poly([(x - 0.05, 0.035), (x - 0.01, 0.05), (x + 0.04, 0.045), (x + 0.06, 0.07)]), region="pallet"))
            out.append(shape("top-board-nails", "pallet", "detail", 0.52, False, 0.3, poly([(0.06, DECK - 0.006), (0.085, DECK - 0.006)]), region="pallet"))
            out.append(shape("pallet-stamp", "pallet", "detail", 0.48, False, 0.3, rect(0.37, 0.035, 0.06, 0.03), region="pallet"))
    out.append(shape("batten-right", "pallet", "body", 2.5, True, 0.66, rect(COIL_X - BAT_L / 2, DECK, BAT_L, BAT_H), region="pallet", material=w))
    return out


def pallet_front(scale):
    """🪵 Pallet and battens in the front elevation (lateral by height)."""
    w, out = "pallet-spruce", []
    if scale == 200:
        out.append(shape("deck", "pallet", "body", 0.5, True, 0.62, rect(-PAL_W / 2, DECK - 2 * BOARD_T, PAL_W, 2 * BOARD_T), region="pallet", material=w))
        for j, z in enumerate(FOOT_Z):
            out.append(shape(f"runner-{j + 1}", "pallet", "body", 0.4, True, 0.55, rect(z - BLOCK_Z / 2, 0, BLOCK_Z, BOARD_T + BLOCK_H), region="pallet", material=w))
    else:
        for j, z in enumerate(FOOT_Z):
            out.append(shape(f"bottom-board-{j + 1}", "pallet", "body", 0.4, True, 0.55, rect(z - BLOCK_Z / 2, 0, BLOCK_Z, BOARD_T), region="pallet", material=w))
            out.append(shape(f"block-1-{j + 1}" if scale == 50 else f"stringer-1-foot-{j + 1}", "pallet", "body", 0.45, True, 0.5, rect(z - BLOCK_Z / 2, BOARD_T, BLOCK_Z, BLOCK_H), region="pallet", material=w))
        out.append(shape("cross-board-3" if scale == 50 else "stringer-3", "pallet", "body", 0.46, True, 0.58, rect(-PAL_W / 2, BOARD_T + BLOCK_H, PAL_W, BOARD_T), region="pallet", material=w))
        for i, z in enumerate(TOP_Z):
            out.append(shape(f"top-board-{i + 1}", "pallet", "body", 0.5, True, 0.62, rect(z - TOP_W / 2, DECK - BOARD_T, TOP_W, BOARD_T), region="pallet", material=w))
    for bid, z in BAT_Z.items():
        out.append(shape(bid, "pallet", "body", 2.5, True, 0.66, rect(z - BAT_W / 2, DECK, BAT_W, BAT_H), region="pallet", material=w))
    return out


def pallet_plan(scale):
    """🪵 Pallet and battens in the plan (ahead by lateral)."""
    w, out = "pallet-spruce", []
    if scale == 200:
        d = f"M 0 -0.3 L {PAL_L} -0.3 L {PAL_L} 0.3 L 0 0.3 Z " + " ".join(f"M {r6(x0)} {r6(z0)} L {r6(x1)} {r6(z0)} L {r6(x1)} {r6(z1)} L {r6(x0)} {r6(z1)} Z" for z0, z1, x0, x1 in DECK_HOLES)
        out.append(shape("deck", "pallet", "body", 0.5, True, 0.62, {"kind": "path", "d": d}, region="pallet", material=w))
    else:
        for i, x in enumerate(ROW_X):
            out.append(shape(f"cross-board-{i + 1}" if scale == 50 else f"stringer-{i + 1}", "pallet", "body", 0.46, True, 0.58, rect(x - BLOCK_X / 2, -PAL_W / 2, BLOCK_X, PAL_W), region="pallet", material=w))
        for i, z in enumerate(TOP_Z):
            out.append(shape(f"top-board-{i + 1}", "pallet", "body", 0.5, True, 0.62, rect(0, z - TOP_W / 2, PAL_L, TOP_W), region="pallet", material=w))
        if scale == 50:
            for i, z in enumerate(TOP_Z):
                out.append(shape(f"top-board-{i + 1}-grain", "pallet", "texture", 0.51, False, 0.35, poly([(0.02, z - 0.03), (0.3, z - 0.025), (0.55, z - 0.034), (0.78, z - 0.028)]), region="pallet"))
    for bid, z in BAT_Z.items():
        out.append(shape(bid, "pallet", "body", 2.5, True, 0.66, rect(COIL_X - BAT_L / 2, z - BAT_W / 2, BAT_L, BAT_W), region="pallet", material=w))
    return out


def elevation(scale):
    """📐 Side elevation: the coil seen face on, standing on edge between the battens."""
    return pallet_elevation(scale) + line_shapes(scale, "elevation")


def front(scale):
    """🎯 Front elevation: the coil seen edge on, 68 mm wide, between the battens on the pallet."""
    return pallet_front(scale) + line_shapes(scale, "front")


def plan(scale):
    """🗺️ Plan: the top of the coil as a 68 mm band across the pallet."""
    return pallet_plan(scale) + line_shapes(scale, "plan")


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}
TURNS = round((sum(abs(r) for r in REL[1:]) + abs(REL[0])) / 360, 2)

DECISIONS = {
    50: {
        "silhouette": t(f"Side elevation: the coil face on, {COIL_OD * 1000:.0f} mm across (14.7 mm on paper), standing on edge between two battens on the 800 x 600 x 144 mm half pallet; {N} straight segments of 130 mm form {TURNS} loose turns of the 35.5 mm suction line with the 32.4 mm liquid line behind it, the knees closed by discs of the tube diameter, and the copper tails at both ends. Front view: the coil edge on as a band of 68 mm (the two insulated tubes side by side) between the battens. Plan: the top of the coil across the pallet boards.", f"Seitenansicht: die Rolle von vorn, {dez(f'{COIL_OD * 1000:.0f}')} mm im Durchmesser (14,7 mm auf Papier), hochkant zwischen zwei Leisten auf der Halbpalette 800 x 600 x 144 mm; {N} gerade Abschnitte von 130 mm bilden {dez(TURNS)} lockere Windungen der Saugleitung von 35,5 mm mit der Flüssigkeitsleitung von 32,4 mm dahinter, die Knicke durch Scheiben vom Rohrdurchmesser geschlossen, und die Kupferenden an beiden Enden. Vorderansicht: die Rolle von der Kante als 68 mm breites Band (die beiden gedämmten Rohre nebeneinander) zwischen den Leisten. Draufsicht: der Scheitel der Rolle quer über den Palettenbrettern."),
        "retained": t("The two copper tails of each end (6.35 and 9.52 mm, 160 and 200 mm out of the foam) with their crushed tips, the ragged knife cut of the foam, nine cable ties with their heads binding the pair, compression creases of the foam, the paper tag of the dealer, the stamp, the nails and the grain of the pallet.", "Die beiden Kupferenden jedes Endes (6,35 und 9,52 mm, 160 und 200 mm aus der Dämmung) mit ihren gequetschten Spitzen, der ausgefranste Messerschnitt der Dämmung, neun Kabelbinder mit ihren Köpfen, die das Paar binden, Druckfalten der Dämmung, der Papieranhänger des Händlers, der Stempel, die Nägel und die Maserung der Palette."),
        "merged": t("The liquid line lies hidden behind the suction line in the side elevation and is drawn behind it; the 0.8 mm copper wall inside the foam is not drawn.", "Die Flüssigkeitsleitung liegt in der Seitenansicht verdeckt hinter der Saugleitung und ist dahinter gezeichnet; die Kupferwand von 0,8 mm in der Dämmung ist nicht gezeichnet."),
        "omitted": t("Flare nut threads, the felt of the hook and the 2 mm teeth of the cable ties (below 7.5 mm).", "Gewinde der Bördelmuttern, der Filz des Hakens und die 2-mm-Zähne der Kabelbinder (unter 7,5 mm)."),
        "thinElements": t("The copper tails keep their true diameters of 6.35 and 9.52 mm (0.13 and 0.19 mm on paper).", "Die Kupferenden behalten ihre wahren Durchmesser von 6,35 und 9,52 mm (0,13 und 0,19 mm auf Papier)."),
        "hatching": t("No hatch: a view of black foam, oxidised copper and weathered spruce; tone separates them.", "Keine Schraffur: Ansicht von schwarzem Schaum, oxidiertem Kupfer und vergrauter Fichte; der Ton trennt sie."),
        "styles": t("Outline 0.35 mm, ties, tag, cuts and crushed tips 0.18 mm, creases and grain 0.1 mm.", "Kontur 0,35 mm, Kabelbinder, Anhänger, Schnitte und gequetschte Spitzen 0,18 mm, Falten und Maserung 0,1 mm."),
        "overlays": t("Every foam member is a cell of the liquid-line or suction-line region, the tails cells of the copper-ends region, the boards, blocks and battens cells of the pallet region.", "Jedes Schaumelement ist eine Zelle der Region Flüssigkeitsleitung oder Saugleitung, die Enden Zellen der Region Kupferenden, Bretter, Klötze und Leisten Zellen der Region Palette."),
    },
    100: {
        "silhouette": t("The same coil at 7.4 mm across: the 39 segments of both insulated tubes with their knees, the copper tails at both ends drawn at their true diameter, the pallet with its three block rows drawn as stringers.", "Dieselbe Rolle mit 7,4 mm Durchmesser: die 39 Abschnitte beider gedämmten Rohre mit ihren Knicken, die Kupferenden an beiden Enden in wahrem Durchmesser, die Palette mit ihren drei Klotzreihen als Kufen gezeichnet."),
        "retained": t("Every second cable tie as a stroke across the pair.", "Jeder zweite Kabelbinder als Strich quer über das Paar."),
        "merged": t("Each cross board and its three blocks are one stringer; the knees of the tubes are spheres of the tube diameter; at three knees by the hook that lie 3 to 4 mm from the next turn (0.03 mm on paper) the foam of the two turns is drawn touching.", "Jedes Querbrett und seine drei Klötze sind eine Kufe; die Knicke der Rohre sind Kugeln vom Rohrdurchmesser; an drei Knicken beim Haken, die 3 bis 4 mm vor der nächsten Windung liegen (0,03 mm auf Papier), ist die Dämmung beider Windungen berührend gezeichnet."),
        "omitted": t("Tie heads, crushed tips, knife cuts, creases, the tag and the pallet grain, stamp and nails.", "Binderköpfe, gequetschte Spitzen, Messerschnitte, Falten, der Anhänger sowie Maserung, Stempel und Nägel der Palette."),
        "thinElements": t("The copper tails keep their true diameters as slender parts (0.06 and 0.1 mm on paper).", "Die Kupferenden behalten als schlanke Teile ihre wahren Durchmesser (0,06 und 0,1 mm auf Papier)."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm, ties 0.13 mm.", "Kontur 0,25 mm, Kabelbinder 0,13 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    200: {
        "silhouette": t("A coil of 3.7 mm across on a pallet of 4 x 0.7 mm: the 39 segments of both tubes as square bars of their insulated diameter, the knees as discs, the tails as slender strokes; the pallet as one deck slab on three runners.", "Eine Rolle von 3,7 mm Durchmesser auf einer Palette von 4 x 0,7 mm: die 39 Abschnitte beider Rohre als Vierkantstäbe ihres gedämmten Durchmessers, die Knicke als Scheiben, die Enden als schlanke Striche; die Palette als eine Deckplatte auf drei Kufen."),
        "retained": t("The open eye of the coil and the gaps between the turns, the battens beside the coil.", "Das offene Auge der Rolle und die Lücken zwischen den Windungen, die Leisten neben der Rolle."),
        "merged": t("Top boards and cross boards are one deck slab (44 mm), each row of bottom board and blocks one runner.", "Deckbretter und Querbretter sind eine Deckplatte (44 mm), jede Reihe aus Bodenbrett und Klötzen eine Kufe."),
        "omitted": t("Cable ties and all traces of use.", "Kabelbinder und alle Gebrauchsspuren."),
        "thinElements": t("The copper tails as slender parts at their true diameter.", "Die Kupferenden als schlanke Teile in wahrem Durchmesser."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm.", "Kontur 0,18 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
}

VOLUME_DECISIONS = {
    50: [
        t("Body: each of the 39 segments carries the two insulated tubes side by side as turned cylinders (1/4 inch liquid line 32.35 mm, 3/8 inch suction line 35.52 mm, touching along their length), every knee is closed by a sphere of each tube, the copper tails 160 and 200 mm long stand out of the foam at both ends; the half pallet is built board by board: three top boards 145 x 22 mm, three cross boards, nine blocks 145 x 78 x 100 mm, three bottom boards and the two battens 40 x 50 x 300 mm that keep the coil upright.", "Körper: jeder der 39 Abschnitte trägt die beiden gedämmten Rohre nebeneinander als gedrehte Zylinder (1/4-Zoll-Flüssigkeitsleitung 32,35 mm, 3/8-Zoll-Saugleitung 35,52 mm, der Länge nach aneinander), jeder Knick ist durch eine Kugel jedes Rohrs geschlossen, die Kupferenden von 160 und 200 mm stehen an beiden Enden aus der Dämmung; die Halbpalette ist Brett für Brett gebaut: drei Deckbretter 145 x 22 mm, drei Querbretter, neun Klötze 145 x 78 x 100 mm, drei Bodenbretter und die beiden Leisten 40 x 50 x 300 mm, die die Rolle aufrecht halten."),
        t("Thermal: each tube as two nested shells, the copper tube 0.8 mm (slender) inside the foam sleeve 13 mm, with foam spheres at the knees; tails and pallet reuse their body solids.", "Thermisch: jedes Rohr als zwei ineinanderliegende Schalen, das Kupferrohr 0,8 mm (schlank) in der Dämmhülse 13 mm, mit Schaumkugeln an den Knicken; Enden und Palette verwenden ihre Körper."),
        t("Structure: the two tube cylinders of every segment are the members of its bone, the knee spheres its joints to the previous segment, the tails members of the end segments; the pallet parts are the members of the pallet bone.", "Tragwerk: die beiden Rohrzylinder jedes Abschnitts sind die Stäbe seines Knochens, die Knickkugeln seine Gelenke zum vorigen Abschnitt, die Enden Stäbe der Endabschnitte; die Palettenteile sind die Stäbe des Palettenknochens."),
    ],
    100: [
        t("Body: each tube segment is a 16-sided prism of the insulated diameter, the knees are ellipsoid spheres; each cross board with its three blocks is one comb-shaped stringer; top boards, bottom boards and battens as boxes; the tails as slender cylinders at their true diameter.", "Körper: jeder Rohrabschnitt ist ein 16-seitiges Prisma vom gedämmten Durchmesser, die Knicke sind Kugeln; jedes Querbrett mit seinen drei Klötzen ist eine kammförmige Kufe; Deckbretter, Bodenbretter und Leisten als Quader; die Enden als schlanke Zylinder in wahrem Durchmesser."),
        t("Thermal: every solid reuses its body solid; the copper wall and the nested shells of 1:50 are below the 15 mm feature.", "Thermisch: jeder Körper verwendet seinen Körper; Kupferwand und Schalen von 1:50 liegen unter der kleinsten Einzelheit von 15 mm."),
        t("Structure: as 1:50 on these solids.", "Tragwerk: wie 1:50 auf diesen Körpern."),
    ],
    200: [
        t("Body: each tube segment is a square bar of its insulated diameter (32.35 and 35.52 mm, above the 30 mm feature), the knees are discs along the joint axis that close the bend of a square bar exactly; the pallet is one deck slab 800 x 44 x 600 mm on three runners of bottom board and blocks; the battens as boxes; the tails slender.", "Körper: jeder Rohrabschnitt ist ein Vierkantstab seines gedämmten Durchmessers (32,35 und 35,52 mm, über der kleinsten Einzelheit von 30 mm), die Knicke sind Scheiben entlang der Gelenkachse, die den Knick eines Vierkantstabs genau schließen; die Palette ist eine Deckplatte 800 x 44 x 600 mm auf drei Kufen aus Bodenbrett und Klötzen; die Leisten als Quader; die Enden schlank."),
        t("Thermal: every solid reuses its body solid.", "Thermisch: jeder Körper verwendet seinen Körper."),
        t("Structure: as 1:100 on these solids.", "Tragwerk: wie 1:100 auf diesen Körpern."),
    ],
}


def representation(scale):
    """🖨️ The authored representation of one scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": DECISIONS[scale]}


UNAVAILABLE_500 = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
                   "reason": t(f"The whole lot is {PAL_L * 2:.1f} x {HEIGHT * 2:.1f} mm at 1:500 and the line set itself 0.07 mm thick: its 35.5 mm tubes, the turns of the coil and the 130 mm segments that carry it cannot be drawn or modelled at the 75 mm smallest feature, so the 1:200 drawing stands in.", f"Der ganze Posten misst bei 1:500 {dez(f'{PAL_L * 2:.1f}')} x {dez(f'{HEIGHT * 2:.1f}')} mm und das Leitungspaar selbst ist 0,07 mm dick: seine Rohre von 35,5 mm, die Windungen der Rolle und die Abschnitte von 130 mm, die sie tragen, lassen sich mit der kleinsten Einzelheit von 75 mm weder zeichnen noch modellieren; daher tritt die Zeichnung 1:200 an ihre Stelle."),
                   "limitingFeature": t("Insulated suction line (35.5 mm)", "Gedämmte Saugleitung (35,5 mm)"), "limitingSizeM": r6(2 * R_S),
                   "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}


def volume(scale):
    """🧊 The authored volume of one scale."""
    body = bodies(scale)
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermals(scale, body), "structure": structures(scale, body), "decisions": VOLUME_DECISIONS[scale]}


def bind(exp, sun, deck=None, pal_sun=0.3):
    """🌬️ Region bindings of one pose."""
    reg = {"liquid-line": {"exposedFraction": exp, "sunlitFraction": sun}, "suction-line": {"exposedFraction": exp, "sunlitFraction": sun}, "copper-ends": {"exposedFraction": 1, "sunlitFraction": sun}, "pallet": {"exposedFraction": 0.85, "sunlitFraction": pal_sun, "contact": {"fraction": 0.1, "tempC": 9}}}
    if deck:
        reg["suction-line"]["contact"] = {"fraction": deck, "tempC": 9}
    return reg


REINSTALLED = {"hoist": {"dy": HOIST_UP}, "traverse": {"dx": TRAVERSE}}
REINSTALLED.update({SEG[k]: {"angleDeg": OFFSETS[k]} for k in range(N) if abs(OFFSETS[k]) > 1e-9})
R32_LOAD = [{"id": "refrigerant-charge", "kind": "carried", "title": t(f"Liquid R32 in the liquid line, {R32_KG * 1000:.0f} g", f"Flüssiges R32 in der Flüssigkeitsleitung, {dez(f'{R32_KG * 1000:.0f}')} g"), "bone": SEG[13], "x": r6(S / 2), "y": 0, "z": r6(Z_L), "fxN": 0, "fyN": r6(-R32_KG * G), "source": "literature", "reference": "refprop-r32"}]

POSES = [
    {"id": "stored", "title": t("Stored on the half pallet", "Auf der Halbpalette gelagert"), "description": t("Recovered and cut free, the line set is re-coiled by hand into a loose flat coil held by cable ties and stands on edge between two battens on a half pallet in the component exchange, its crushed copper ends pointing out of the coil.", "Abgesaugt und freigeschnitten ist das Leitungspaar von Hand zu einer lockeren flachen Rolle gewickelt, von Kabelbindern gehalten, und steht hochkant zwischen zwei Leisten auf einer Halbpalette in der Bauteilbörse, die gequetschten Kupferenden ragen aus der Rolle."), "bones": {}, "ground": "authored", "contacts": GROUND, "thermal": {"operating": {"refrigerant": 0}, "regions": bind(0.82, 0.3, deck=0.01), "emitters": {}}, "loads": []},
    {"id": "lifted", "title": t("Lifted on a hook", "Am Haken angehoben"), "description": t("A padded hook through the eye of the coil takes it by the top of the innermost turn and lifts it 500 mm out of the battens; the pallet stays behind.", "Ein gepolsterter Haken durch das Auge der Rolle fasst sie am Scheitel der innersten Windung und hebt sie 500 mm aus den Leisten; die Palette bleibt zurück."), "bones": {"hoist": {"dy": LIFT_UP}}, "ground": "authored", "contacts": ["hook"], "thermal": {"operating": {"refrigerant": 0}, "regions": bind(0.82, 0.3), "emitters": {}}, "loads": []},
    {"id": "reinstalled", "title": t("Reinstalled, cooling", "Wieder eingebaut, im Kühlbetrieb"), "description": t("Uncoiled, its ends cut back and re-flared, evacuated and charged, the line set runs again between the outdoor unit and the indoor unit of a split air conditioner: 0.8 m along the base of the wall, 1.5 m up and 2.2 m to the indoor unit, held by six clamps; in cooling the suction line carries cold gas at 10 to 14 degC, the liquid line warm liquid at 40 to 38 degC.", "Abgewickelt, die Enden zurückgeschnitten und neu gebördelt, evakuiert und befüllt, verbindet das Leitungspaar wieder Außen- und Innengerät eines Split-Klimageräts: 0,8 m am Wandfuß entlang, 1,5 m hoch und 2,2 m zum Innengerät, von sechs Schellen gehalten; im Kühlbetrieb führt die Saugleitung kaltes Gas mit 10 bis 14 °C, die Flüssigkeitsleitung warme Flüssigkeit mit 40 bis 38 °C."), "bones": REINSTALLED, "ground": "authored", "contacts": SERVICE, "thermal": {"operating": {"refrigerant": 1}, "regions": bind(0.92, 0.45), "emitters": {}}, "loads": R32_LOAD},
    {"id": "reinstalled-off", "title": t("Reinstalled, unit off", "Wieder eingebaut, Gerät aus"), "description": t("The same route with the unit switched off: the charge rests in the line set at the temperature of the facade.", "Derselbe Verlauf bei ausgeschaltetem Gerät: die Füllung ruht im Leitungspaar mit der Temperatur der Fassade."), "bones": REINSTALLED, "ground": "authored", "contacts": SERVICE, "thermal": {"operating": {"refrigerant": 0}, "regions": bind(0.92, 0.45), "emitters": {}}, "loads": R32_LOAD},
]

CLIPS = [
    {"id": "lift", "title": t("Lift out of the battens", "Aus den Leisten heben"), "description": t("The hook takes the coil at the top of its innermost turn and lifts it 500 mm in four seconds; the pallet and the battens stay on the floor.", "Der Haken fasst die Rolle am Scheitel ihrer innersten Windung und hebt sie in vier Sekunden um 500 mm; Palette und Leisten bleiben am Boden."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1, "pose": "stored", "ease": "smooth"}, {"t": 5, "pose": "lifted", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 1} for a in GROUND] + [{"anchor": "hook", "fromS": 1, "toS": 6}],
     "phases": [{"id": "engage", "fromS": 0, "toS": 1, "title": t("Hook engages", "Haken greift")}, {"id": "raise", "fromS": 1, "toS": 5, "title": t("Raise", "Anheben")}, {"id": "hold", "fromS": 5, "toS": 6, "title": t("Hold", "Halten")}],
     "events": [{"t": 1, "kind": "contact-off", "title": t("Coil leaves the pallet", "Rolle verlässt die Palette")}], "tracks": []},
    {"id": "start-up", "title": t("Start-up in cooling", "Anlauf im Kühlbetrieb"), "description": t("The unit starts after the charge has settled: within four minutes the suction line cools from the facade temperature to 10 to 14 degC while the liquid line warms to 40 to 38 degC, and the pair holds its design temperatures.", "Das Gerät läuft an, nachdem sich die Füllung gesetzt hat: innerhalb von vier Minuten kühlt die Saugleitung von der Fassadentemperatur auf 10 bis 14 °C ab, während sich die Flüssigkeitsleitung auf 40 bis 38 °C erwärmt, und das Paar hält seine Auslegungstemperaturen."), "durationS": 600, "fps": 1, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 30, "pose": "reinstalled", "ease": "step"}, {"t": 600, "pose": "reinstalled", "ease": "step"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 600} for a in SERVICE],
     "phases": [{"id": "standstill", "fromS": 0, "toS": 30, "title": t("Standstill", "Stillstand")}, {"id": "pull-down", "fromS": 30, "toS": 270, "title": t("Pull-down", "Herunterkühlen")}, {"id": "design", "fromS": 270, "toS": 600, "title": t("Design temperatures", "Auslegungstemperaturen")}],
     "events": [{"t": 30, "kind": "operating", "title": t("Compressor starts", "Verdichter läuft an"), "target": "refrigerant"}, {"t": 270, "kind": "operating", "title": t("Design temperatures reached", "Auslegungstemperaturen erreicht"), "target": "refrigerant"}],
     "tracks": [{"target": "operating:refrigerant", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 270, "v": 1}, {"t": 600, "v": 1}]}]},
]


def r_cyl(ro, ri, k):
    """🧮 Thermal resistance of a cylindrical layer referred to its outer surface (VDI 2055-1)."""
    return r6(ro * math.log(ro / ri) / k)


def layers(name, rc, rf):
    """🧶 Copper wall and foam sleeve of one tube, innermost first."""
    return [
        {"id": f"{name}-copper", "title": t("Copper tube wall 0.8 mm", "Kupferrohrwand 0,8 mm"), "kind": "metal", "material": "copper-tube", "thicknessM": T_CU, "resistanceM2KW": r_cyl(rc, rc - T_CU, K_CU), "heatCapacityJM2K": round(T_CU * RHO_CU * 385 * rc / rf, 1), "source": "literature", "reference": "en-12735-1"},
        {"id": f"{name}-foam", "title": t("Elastomeric foam sleeve 13 mm", "Synthesekautschuk-Dämmhülse 13 mm"), "kind": "insulation", "material": "elastomer-foam", "thicknessM": T_FOAM, "resistanceM2KW": r_cyl(rf, rc, K_FOAM), "heatCapacityJM2K": round((rf ** 2 - rc ** 2) / (2 * rf) * RHO_FOAM * 1500, 1), "source": "literature", "reference": "en-14304"},
    ]


LAY_L, LAY_S = layers("liquid", RC_L, R_L), layers("suction", RC_S, R_S)
THERMAL = {
    "environment": {"airTempC": 22, "meanRadiantTempC": 24, "skyTempC": 8, "groundTempC": 20, "relativeHumidity": 0.55, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 450, "diffuseFraction": 0.4, "pressurePa": 101325},
    "passengers": [],
    "operating": [{"id": "refrigerant", "title": t("Refrigerant circuit of the split unit", "Kältekreis des Split-Geräts"), "levels": [{"v": 0, "title": t("Recovered, or unit off", "Abgesaugt oder Gerät aus")}, {"v": 1, "title": t("Cooling at design: suction gas 10/14 degC, liquid 40/38 degC", "Kühlen in Auslegung: Sauggas 10/14 °C, Flüssigkeit 40/38 °C")}]}],
    "regions": [
        {"id": "liquid-line", "title": t("Liquid line 1/4 inch in 13 mm foam", "Flüssigkeitsleitung 1/4 Zoll in 13 mm Dämmung"), "kind": "surface", "model": "hydronic", "material": "elastomer-foam", "areaM2": LIQ_AREA, "characteristicLengthM": r6(2 * R_L), "geometry": "cylinder", "skyViewFactor": 0.4, "layers": LAY_L, "heatCapacityJM2K": round(sum(x["heatCapacityJM2K"] for x in LAY_L), 1), "hydronic": {"key": "refrigerant", "rows": [{"v": 1, "supplyTempC": 40, "returnTempC": 38}], "flowPosition": 0.5, "filmM2KW": 0.002}, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "line-set-datasheets"},
        {"id": "suction-line", "title": t("Suction line 3/8 inch in 13 mm foam", "Saugleitung 3/8 Zoll in 13 mm Dämmung"), "kind": "surface", "model": "hydronic", "material": "elastomer-foam", "areaM2": SUC_AREA, "characteristicLengthM": r6(2 * R_S), "geometry": "cylinder", "skyViewFactor": 0.4, "layers": LAY_S, "heatCapacityJM2K": round(sum(x["heatCapacityJM2K"] for x in LAY_S), 1), "hydronic": {"key": "refrigerant", "rows": [{"v": 1, "supplyTempC": 14, "returnTempC": 10}], "flowPosition": 0.5, "filmM2KW": 0.004}, "source": "authored-estimate", "reference": "line-set-datasheets"},
        {"id": "copper-ends", "title": t("Bare copper tails with crushed tips", "Blanke Kupferenden mit gequetschten Spitzen"), "kind": "surface", "model": "passive", "material": "copper-oxidised", "areaM2": CU_AREA, "characteristicLengthM": D_S, "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": round(T_CU * RHO_CU * 385, 1), "source": "literature", "reference": "en-12735-1"},
        {"id": "pallet", "title": t("Half pallet and battens", "Halbpalette und Leisten"), "kind": "surface", "model": "passive", "material": "pallet-spruce", "areaM2": 1.8, "characteristicLengthM": 0.145, "geometry": "horizontal-plate-up", "skyViewFactor": 0.4, "layers": [{"id": "pallet-timber", "title": t("Spruce, thermally active 11 mm", "Fichte, thermisch wirksam 11 mm"), "kind": "timber", "material": "pallet-spruce", "thicknessM": 0.011, "resistanceM2KW": r6(0.011 / 0.13), "heatCapacityJM2K": round(0.011 * RHO_WOOD * 1600, 1), "source": "literature", "reference": "en-iso-10456"}], "heatCapacityJM2K": round(0.011 * RHO_WOOD * 1600, 1), "source": "literature", "reference": "en-iso-10456"},
    ],
    "emitters": [],
    "assumptions": [
        t("No element draws power: the line set only carries the refrigerant of the split unit (0 W, passive, stated on the liquid line); the pallet is passive too.", "Kein Element nimmt Leistung auf: das Leitungspaar führt nur das Kältemittel des Split-Geräts (0 W, passiv, an der Flüssigkeitsleitung angegeben); auch die Palette ist passiv."),
        t(f"Each tube is two layers, innermost first: copper 0.8 mm and elastomeric foam 13 mm with lambda {K_FOAM} W/mK; the resistances are referred to the outer surface of the sleeve, r_o ln(r_o / r_i) / lambda (VDI 2055-1): {LAY_L[1]['resistanceM2KW']} m2K/W on the liquid line, {LAY_S[1]['resistanceM2KW']} m2K/W on the suction line.", f"Jedes Rohr besteht aus zwei Schichten, innerste zuerst: Kupfer 0,8 mm und Synthesekautschuk 13 mm mit Lambda {dez(K_FOAM)} W/mK; die Widerstände sind auf die Außenfläche der Hülse bezogen, r_a ln(r_a / r_i) / Lambda (VDI 2055-1): {dez(LAY_L[1]['resistanceM2KW'])} m²K/W an der Flüssigkeitsleitung, {dez(LAY_S[1]['resistanceM2KW'])} m²K/W an der Saugleitung."),
        t("The refrigerant is entered as the fluid of a circuit (model hydronic) with the key refrigerant: at level 1 the suction gas is 10 degC at the indoor unit and 14 degC at the outdoor unit, the liquid 40 degC at the outdoor unit and 38 degC at the indoor unit (R32, about 8 degC evaporating and 43 degC condensing); at level 0 (recovered, or unit off) the fluid stands at the air temperature.", "Das Kältemittel ist als Fluid eines Kreises (Modell hydronic) mit dem Schlüssel refrigerant eingetragen: auf Stufe 1 hat das Sauggas 10 °C am Innengerät und 14 °C am Außengerät, die Flüssigkeit 40 °C am Außengerät und 38 °C am Innengerät (R32, etwa 8 °C Verdampfung und 43 °C Verflüssigung); auf Stufe 0 (abgesaugt oder Gerät aus) steht das Fluid auf Lufttemperatur."),
        t("Weathered surfaces: the black foam is UV-aged and soiled (emissivity 0.9, solar absorptance 0.92), the bare copper tails are oxidised brown (0.6 instead of about 0.05 polished), the spruce of the pallet is weathered grey (solar absorptance 0.62).", "Bewitterte Oberflächen: der schwarze Schaum ist UV-gealtert und verschmutzt (Emissionsgrad 0,9, solare Absorption 0,92), die blanken Kupferenden sind braun oxidiert (0,6 statt etwa 0,05 blank), die Fichte der Palette ist vergraut (solare Absorption 0,62)."),
        t("Reference case: a warm summer afternoon (22 degC air, 450 W/m2 sun); stored and lifted the coil stands in the open store, reinstalled the line set runs on a sunlit facade.", "Referenzfall: ein warmer Sommernachmittag (22 °C Luft, 450 W/m² Sonne); gelagert und gehoben steht die Rolle im offenen Lager, eingebaut verläuft das Leitungspaar an einer besonnten Fassade."),
    ],
}

STRUCTURE = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
    t("No support is permanent: each pose names its contacts. Stored, the four bottom-board ends of the pallet on the floor (contacts, friction 0.5); lifted, one padded hook under the top of the innermost turn (pin); reinstalled, the flare nuts on the outdoor and the indoor unit and six wall clamps along the route (pins).", "Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert die vier Bodenbrettenden der Palette auf dem Boden (Kontakte, Reibung 0,5); gehoben ein gepolsterter Haken unter dem Scheitel der innersten Windung (Gelenk); eingebaut die Bördelmuttern an Außen- und Innengerät und sechs Wandschellen entlang des Verlaufs (Gelenke)."),
    t(f"The solver treats the whole asset as one rigid body: lifted and reinstalled, the {dez(PAL_KG)} kg pallet left on the floor is counted in the hook and clamp reactions ({PAL_KG / MASS * 100:.0f} % of the weight).", f"Der Löser behandelt das ganze Asset als einen starren Körper: gehoben und eingebaut geht die am Boden zurückgelassene Palette von {dez(PAL_KG)} kg in die Reaktionen von Haken und Schellen ein ({PAL_KG / MASS * 100:.0f} % des Gewichts)."),
    t(f"Masses from sections and densities: copper 1/4 inch {CU_L_KGM:.4f} kg/m and 3/8 inch {CU_S_KGM:.4f} kg/m (EN 12735-1, 8940 kg/m3), foam sleeves {FOAM_L_KGM:.4f} and {FOAM_S_KGM:.4f} kg/m (55 kg/m3), together {LINE_KGM:.4f} kg/m, {SEG_KG} kg per 130 mm segment; tails {STUB_KG} kg per end; pallet with battens and nails {PAL_KG} kg (450 kg/m3); total {MASS} kg. The segments carry the bending as two copper tubes; the foam carries nothing.", f"Massen aus Querschnitten und Dichten: Kupfer 1/4 Zoll {dez(f'{CU_L_KGM:.4f}')} kg/m und 3/8 Zoll {dez(f'{CU_S_KGM:.4f}')} kg/m (EN 12735-1, 8940 kg/m³), Dämmhülsen {dez(f'{FOAM_L_KGM:.4f}')} und {dez(f'{FOAM_S_KGM:.4f}')} kg/m (55 kg/m³), zusammen {dez(f'{LINE_KGM:.4f}')} kg/m, {dez(SEG_KG)} kg je Abschnitt von 130 mm; Enden {dez(STUB_KG)} kg je Ende; Palette mit Leisten und Nägeln {dez(PAL_KG)} kg (450 kg/m³); zusammen {dez(MASS)} kg. Die Abschnitte tragen die Biegung als zwei Kupferrohre; der Schaum trägt nichts."),
    t(f"Reinstalled, the liquid line holds {R32_KG * 1000:.0f} g of liquid R32 (893 kg/m3 at 40 degC in the 4.75 mm bore over {LENGTH} m), drawn as one point load in the middle of the riser; the gas in the suction line weighs a few grams and is left out.", f"Eingebaut enthält die Flüssigkeitsleitung {dez(f'{R32_KG * 1000:.0f}')} g flüssiges R32 (893 kg/m³ bei 40 °C in der Bohrung von 4,75 mm über {dez(LENGTH)} m), als eine Einzellast in der Mitte der Steigleitung dargestellt; das Gas in der Saugleitung wiegt wenige Gramm und ist weggelassen."),
], "clinical": False}

ASSET = {
    "id": SLUG,
    "category": "components",
    "title": t("Used Insulated Copper Pipe Coils", "Gebrauchte isolierte Kupferrohrringe"),
    "summary": t(f"A used pre-insulated twin refrigerant line set of a split air conditioner, copper 1/4 and 3/8 inch each in 13 mm black elastomeric foam, {LENGTH} m long and {LINE_KG} kg; recovered and cut free with crushed copper ends, it is re-coiled by hand into a loose flat coil of {COIL_OD * 1000:.0f} mm held by cable ties and stands on edge between two battens on a half pallet, is lifted on a hook through its eye and runs again as the line set between outdoor and indoor unit when reinstalled.", f"Ein gebrauchtes vorgedämmtes Kältemittel-Leitungspaar eines Split-Klimageräts, Kupfer 1/4 und 3/8 Zoll in je 13 mm schwarzer Synthesekautschuk-Dämmung, {dez(LENGTH)} m lang und {dez(LINE_KG)} kg schwer; abgesaugt und mit gequetschten Kupferenden freigeschnitten, ist es von Hand zu einer lockeren flachen Rolle von {dez(f'{COIL_OD * 1000:.0f}')} mm gewickelt, von Kabelbindern gehalten, und steht hochkant zwischen zwei Leisten auf einer Halbpalette, wird an einem Haken durch sein Auge gehoben und verbindet im Wiedereinbau wieder Außen- und Innengerät."),
    "tags": ["component", "reclaimed", "used", "hvac", "cooling", "plumbing", "copper", "insulation", "elastomeric-foam", "refrigerant-line", "line-set", "pipe", "coil", "pallet", "timber"],
    "provenance": {"author": "Semio staffage component authors (used HVAC wave, author 18)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
        f"Typical pre-insulated twin line set for a 2.5 to 3.5 kW split unit: copper 6.35 x 0.8 and 9.52 x 0.8 mm (EN 12735-1, R220 annealed), each in 13 mm elastomeric foam, {LENGTH} m in {N} segments of 130 mm, coiled flat with an outer diameter of {COIL_OD * 1000:.0f} mm and a pitch of 46 mm per turn; half pallet 800 x 600 x 144 mm built from EN 13698-1 boards and blocks; total {MASS} kg. All dimensions and masses are catalogue-typical (authored-estimate or literature), nothing is measured.",
        f"Typisches vorgedämmtes Leitungspaar für ein Split-Gerät von 2,5 bis 3,5 kW: Kupfer 6,35 x 0,8 und 9,52 x 0,8 mm (EN 12735-1, R220 weich), je in 13 mm Synthesekautschuk, {dez(LENGTH)} m in {N} Abschnitten von 130 mm, flach gewickelt mit einem Außendurchmesser von {dez(f'{COIL_OD * 1000:.0f}')} mm und einer Steigung von 46 mm je Windung; Halbpalette 800 x 600 x 144 mm aus Brettern und Klötzen nach EN 13698-1; zusammen {dez(MASS)} kg. Alle Maße und Massen sind katalogtypisch (authored-estimate oder literature), nichts ist gemessen.")},
    "dimensions": {"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH, "massKg": MASS},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "yard", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {**{str(s): representation(s) for s in (50, 100, 200)}, "500": UNAVAILABLE_500},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {**{str(s): volume(s) for s in (50, 100, 200)}, "500": {"status": "unavailable", "denominator": 500}},
    "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"[DEBUG] wrote {SLUG}: mass {MASS} kg (pallet {PAL_KG}, line {LINE_KG}), length {LENGTH} m, turns {TURNS}, h {HEIGHT} w {WIDTH} d {DEPTH}")
    print(f"[DEBUG] coil OD {COIL_OD}, min centre gap {seg_min_gap():.4f} (need {2 * R_S:.4f}), hook seg {HOOK}, offsets min {min(OFFSETS):.1f} max {max(OFFSETS):.1f}")
    print(f"[DEBUG] areas liquid {LIQ_AREA} suction {SUC_AREA} copper {CU_AREA}; inner vertex {VERTS[-1]}")
