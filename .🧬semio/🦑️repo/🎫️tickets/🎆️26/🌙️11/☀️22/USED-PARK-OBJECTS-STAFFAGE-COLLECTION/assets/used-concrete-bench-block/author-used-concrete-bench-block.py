"""🧱️ Authoritative author script of `used-concrete-bench-block`: states every hand-chosen number and writes `🔣️.json` next to it.

Run: python3 author-used-concrete-bench-block.py  (prints the measured areas, volumes and masses the document is built on).

@see ../../brief.md
@see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))

L = 2.0
D = 0.5
HW = D / 2
HC = 0.405
TB = 0.045
H = HC + TB
C = 0.015
BW = 0.08
GAP = 0.012
NB = 5
B0 = 0.2
BL = 1.6
DECK = NB * BW + (NB - 1) * GAP
BZ0 = -DECK / 2
PITCH = BW + GAP
BZC = [round(BZ0 + BW / 2 + k * PITCH, 6) for k in range(NB)]
EASE = 0.005
SOCKET_X = (0.1, 1.9)
FIX_U = (0.1, 0.8, 1.5)
RHO_C = 2500.0
RHO_T = 760.0
RHO_S = 7850.0
G = 9.80665
SITTER_N = round(75 * G, 1)
SKATE_N = round(80 * G * 2.5, 1)


def t(en, de):
    """🗣️ One bilingual text."""
    return {"en": en, "de": de}


def r(x, n=5):
    """🎯️ Rounds a number for the document."""
    return round(x + 0.0, n)


def pts(points):
    """📍️ Rounds a point list."""
    return [[r(a), r(b)] for a, b in points]


WEAR_50 = [(0.0, C, 0.5), (0.25, C, 0.5), (0.31, 0.024, 0.32), (0.52, 0.028, 0.3), (0.63, 0.04, 0.3), (0.69, 0.029, 0.3), (0.98, 0.031, 0.3), (1.24, 0.027, 0.3), (1.39, 0.042, 0.3), (1.44, 0.03, 0.3), (1.68, 0.026, 0.32), (1.75, C, 0.5), (L, C, 0.5)]
WEAR_100 = [(0.0, C, 0.5), (0.25, C, 0.5), (0.35, 0.028, 0.3), (1.65, 0.028, 0.3), (1.75, C, 0.5), (L, C, 0.5)]


def wear_at(table, x):
    """🛹️ Chamfer size and rounding factor of the worn north arris at station x, interpolated in the table."""
    for (x0, c0, m0), (x1, c1, m1) in zip(table, table[1:]):
        if x0 <= x <= x1:
            f = 0.0 if x1 == x0 else (x - x0) / (x1 - x0)
            return c0 + f * (c1 - c0), m0 + f * (m1 - m0)
    return table[-1][1], table[-1][2]


def section(cw, m, inset):
    """🔷️ The nine-vertex cross-section (v, w) of the block: chamfer C on every long edge, the worn north top arris with chamfer cw and rounding factor m; `inset` shrinks it for the chamfered end faces."""
    d = inset
    c = C - d * (2 - math.sqrt(2)) if d > 0 else C
    cw = c if d > 0 else cw
    lo, hi, wl, wr = d, HC - d, -HW + d, HW - d
    return [(lo, wl + c), (lo, wr - c), (lo + c, wr), (hi - c, wr), (hi, wr - c), (hi, wl + cw), (hi - m * cw, wl + m * cw), (hi - cw, wl), (lo + c, wl)]


def block_mesh(table, extra):
    """🧊️ The block as a closed polyhedron: stations along u with chamfered end faces and the worn north arris; returns vertices (u, v, w), triangles, area and volume."""
    xs = sorted(set([C, L - C] + [x for x, _, _ in table if C < x < L - C] + extra))
    rings = [[(0.0, v, w) for v, w in section(C, 0.5, C)]]
    for x in xs:
        cw, m = wear_at(table, x)
        rings.append([(x, v, w) for v, w in section(cw, m, 0.0)])
    rings.append([(L, v, w) for v, w in section(C, 0.5, C)])
    n = len(rings[0])
    verts = [p for ring in rings for p in ring]
    tris = []
    for i in range(len(rings) - 1):
        for k in range(n):
            a, b = i * n + k, i * n + (k + 1) % n
            c2, d2 = (i + 1) * n + (k + 1) % n, (i + 1) * n + k
            tris += [[a, c2, b], [a, d2, c2]]
    last = (len(rings) - 1) * n
    for k in range(1, n - 1):
        tris.append([0, k, k + 1])
        tris.append([last, last + k + 1, last + k])
    vol = 0.0
    for a, b, c2 in tris:
        p, q, s = verts[a], verts[b], verts[c2]
        vol += (p[0] * (q[1] * s[2] - q[2] * s[1]) - p[1] * (q[0] * s[2] - q[2] * s[0]) + p[2] * (q[0] * s[1] - q[1] * s[0])) / 6
    if vol < 0:
        tris = [[a, c2, b] for a, b, c2 in tris]
        vol = -vol
    area = 0.0
    cy = 0.0
    for a, b, c2 in tris:
        p, q, s = verts[a], verts[b], verts[c2]
        u1 = [q[j] - p[j] for j in range(3)]
        u2 = [s[j] - p[j] for j in range(3)]
        cr = [u1[1] * u2[2] - u1[2] * u2[1], u1[2] * u2[0] - u1[0] * u2[2], u1[0] * u2[1] - u1[1] * u2[0]]
        area += math.sqrt(sum(x * x for x in cr)) / 2
    for a, b, c2 in tris:
        p, q, s = verts[a], verts[b], verts[c2]
        v6 = (p[0] * (q[1] * s[2] - q[2] * s[1]) - p[1] * (q[0] * s[2] - q[2] * s[0]) + p[2] * (q[0] * s[1] - q[1] * s[0])) / 6
        cy += v6 * (p[1] + q[1] + s[1]) / 4
    cy = abs(cy / vol) if vol else 0
    return [[r(a), r(b), r(c2)] for a, b, c2 in verts], tris, area, vol, cy


def poly_area(p):
    """📐️ Area of a simple polygon."""
    return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2


def perimeter(p):
    """📏️ Perimeter of a closed polygon."""
    return sum(math.dist(p[i], p[(i + 1) % len(p)]) for i in range(len(p)))


def prism_area(profile, length):
    """📦️ Outer area of a prism of a profile and a length."""
    return perimeter(profile) * length + 2 * poly_area(profile)


def batten_profile(ease):
    """🪵️ The (v, w) section of the first (north) batten in the batten frame, top edges eased by `ease`."""
    w0, w1 = BZ0, BZ0 + BW
    if ease <= 0:
        return [(0.0, w0), (0.0, w1), (TB, w1), (TB, w0)]
    return [(0.0, w0), (0.0, w1), (TB - ease, w1), (TB, w1 - ease), (TB, w0 + ease), (TB - ease, w0)]


ALGAE_FACE = {
    50: [(0.06, 0.016), (1.94, 0.016), (1.94, 0.2), (1.81, 0.262), (1.63, 0.238), (1.46, 0.302), (1.21, 0.281), (1.0, 0.312), (0.79, 0.27), (0.56, 0.292), (0.35, 0.243), (0.18, 0.252), (0.06, 0.181)],
    100: [(0.06, 0.016), (1.94, 0.016), (1.94, 0.21), (1.62, 0.25), (1.3, 0.29), (1.0, 0.3), (0.65, 0.28), (0.3, 0.25), (0.06, 0.19)],
    200: [(0.06, 0.016), (1.94, 0.016), (1.94, 0.22), (1.4, 0.29), (0.7, 0.28), (0.06, 0.2)],
    500: [(0.06, 0.016), (1.94, 0.016), (1.94, 0.26), (0.06, 0.26)],
}
ALGAE_END = {
    50: [(0.02, 0.225), (0.02, 0.04), (0.068, 0.03), (0.139, 0.062), (0.198, 0.088), (0.258, 0.142), (0.272, 0.19), (0.252, 0.225)],
    100: [(0.02, 0.225), (0.02, 0.035), (0.11, 0.045), (0.21, 0.09), (0.27, 0.16), (0.262, 0.225)],
    200: [(0.02, 0.225), (0.02, 0.03), (0.17, 0.06), (0.27, 0.14), (0.27, 0.225)],
    500: [(0.02, 0.225), (0.02, 0.02), (0.27, 0.02), (0.27, 0.225)],
}
SHEET = 0.002


def algae_face_area(s):
    """🦠️ Outer area of the face film: its outer face and its edges (the face against the concrete is a section face)."""
    p = ALGAE_FACE[s]
    return poly_area(p) + perimeter(p) * SHEET


def algae_end_area(s):
    """🦠️ Outer area of the end-face film."""
    p = ALGAE_END[s]
    return poly_area(p) + perimeter(p) * SHEET


B50 = block_mesh(WEAR_50, [])
B100 = block_mesh(WEAR_100, [])
BOX_AREA = 2 * (L * D + L * HC + D * HC)
BAT50 = batten_profile(EASE)
BAT100 = batten_profile(0.0)
AREA = {
    "concrete-block": {50: B50[2], 100: B100[2], 200: BOX_AREA, 500: 2 * (L * D + L * 0.375 + D * 0.375)},
    "seat-battens": {50: NB * prism_area([(w, v) for v, w in BAT50], BL), 100: NB * prism_area([(w, v) for v, w in BAT100], BL), 200: prism_area([(0, 0), (DECK, 0), (DECK, TB), (0, TB)], BL), 500: prism_area([(0, 0), (D, 0), (D, 0.075), (0, 0.075)], BL)},
    "algae-film": {s: algae_face_area(s) + algae_end_area(s) for s in (50, 100, 200, 500)},
}
VOL_BLOCK = B50[3]
VOL_BATTENS = NB * poly_area(BAT50) * BL
M_SOCKETS = 2 * math.pi * 0.016 ** 2 * 0.1 * RHO_S
M_SCREWS = 15 * math.pi * 0.004 ** 2 * 0.08 * RHO_S
M_BLOCK = VOL_BLOCK * RHO_C + M_SOCKETS
M_BATTENS = VOL_BATTENS * RHO_T + M_SCREWS
M_TOTAL = M_BLOCK + M_BATTENS
COM_Y = B50[4]

CONCRETE_AREA = round(AREA["concrete-block"][50], 3)
BATTEN_AREA = round(AREA["seat-battens"][50], 3)
ALGAE_AREA = round(AREA["algae-film"][50], 3)

REFS = [
    ("en-1991-1-1-2002", "EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: General actions, densities, self-weight, imposed loads for buildings (Table A.1: normal-weight concrete 24 kN/m3, reinforced 25 kN/m3; Table A.3 timber densities)."),
    ("en-1992-1-1-2004", "EN 1992-1-1:2004 Eurocode 2: Design of concrete structures, Part 1-1 (Table 3.1: C35/45, fck 35 MPa, Ecm 34 GPa)."),
    ("din-en-206-din-1045-2", "DIN EN 206:2021 and DIN 1045-2:2023 Concrete: specification, performance, production and conformity (exposure classes XC4, XF4 and XM for unprotected precast street furniture exposed to rain, frost, de-icing salt and abrasion)."),
    ("din-en-13198-2003", "DIN EN 13198:2003 Precast concrete products - Street furniture and garden products (requirements and tests for precast seating, planters and bollards: dimensional tolerances, durability, stability)."),
    ("vdi-bv-bs-6205-2012", "VDI/BV-BS 6205:2012 Lifting inserts and lifting insert systems for precast concrete elements (threaded lifting sockets, load directions, dynamic factor 1.3 for crane lifts, edge distances)."),
    ("din-18040-3-2014", "DIN 18040-3:2014 Construction of accessible buildings - Public traffic and open space (resting places: seat heights of about 0.46-0.48 m, seating with and without backrest along paths)."),
    ("en-16139-2013", "EN 16139:2013 Furniture - Strength, durability and safety - Requirements for non-domestic seating (test levels for public use) with the test methods of EN 1728:2012."),
    ("en-338-2016", "EN 338:2016 Structural timber - Strength classes (hardwood class D40: E0,mean 13 GPa)."),
    ("din-en-350-2016", "DIN EN 350:2016 Durability of wood and wood-based products (Robinia pseudoacacia heartwood durability class 1-2 against fungi, used untreated in exterior street furniture)."),
    ("wagenfuehr-holzatlas-2007", "Wagenfuehr, R. (2007). Holzatlas, 6th ed., Fachbuchverlag Leipzig (Robinia pseudoacacia: density 0.73-0.80 g/cm3 at 12-15 % moisture content)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (reinforced concrete 2500 kg/m3, 2.3 W/mK, 1000 J/kgK; hardwood 0.18 W/mK, 1600 J/kgK; steel 50 W/mK, 450 J/kgK)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total hemispherical emissivity of concrete 0.88-0.93, wood 0.82-0.92, galvanised steel)."),
    ("levinson-akbari-2002", "Levinson, R., Akbari, H. (2002). Effects of composition and exposure on the solar reflectance of Portland cement concrete. Cement and Concrete Research 32(11), 1679-1698 (new grey concrete reflects about 0.35-0.5; weathering and soiling lower the reflectance, so weathered grey concrete absorbs about 0.65)."),
    ("feist-hon-1984", "Feist, W. C., Hon, D. N.-S. (1984). Chemistry of weathering and protection. In: Rowell, R. (ed.), The Chemistry of Solid Wood, Advances in Chemistry 207, American Chemical Society, 401-451 (photodegradation and leaching turn exposed wood silver-grey with surface checks)."),
    ("gaylarde-morton-1999", "Gaylarde, C. C., Morton, L. H. G. (1999). Deteriogenic biofilms on buildings and their control: a review. Biofouling 14(1), 59-74 (green algal films on shaded, damp masonry and concrete faces)."),
    ("woolley-johns-2001", "Woolley, H., Johns, R. (2001). Skateboarding: The city as a playground. Journal of Urban Design 6(2), 211-230 (skaters grind the arrises of benches and blocks in public squares; wax, metal streaks and chipped edges)."),
    ("municipal-seating-block-catalogues", "Municipal street-furniture catalogues of German precast suppliers (catalogue-typical data, no brand): precast concrete seating blocks 1.5-2.5 m long, 0.4-0.6 m deep, seat 0.42-0.46 m high, 0.8-1.4 t, set freestanding by mass on a compacted gravel base and a chippings bed, hardwood or robinia battens screwed down along the seat, two cast-in threaded lifting sockets, edges chamfered 10-20 mm."),
]


def mat(i, en, de, eps, alpha, rho, k, cp, e=None, fs=None, src="literature", ref="iso-10456-2007"):
    """🧱️ One material."""
    m = {"id": i, "title": t(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp}
    if e:
        m["elasticModulusPa"] = e
    if fs:
        m["strengthPa"] = fs
    m["source"] = src
    m["reference"] = ref
    return m


MATERIALS = [
    mat("concrete-weathered", "Weathered precast concrete C35/45", "Verwitterter Fertigteilbeton C35/45", 0.92, 0.65, RHO_C, 2.3, 1000, 34e9, 35e6, "literature", "levinson-akbari-2002"),
    mat("robinia-greyed", "Greyed robinia hardwood", "Vergraute Robinie", 0.9, 0.6, RHO_T, 0.18, 1600, 13e9, None, "authored-estimate", "feist-hon-1984"),
    mat("algae-biofilm", "Green algae biofilm", "Grüner Algenbelag", 0.95, 0.8, 1050, 0.6, 3500, None, None, "authored-estimate", "gaylarde-morton-1999"),
    mat("steel-galvanised", "Galvanised steel lifting socket", "Verzinkte Transportankerhülse", 0.28, 0.65, RHO_S, 50, 450, 210e9, None, "literature", "incropera-2018"),
    mat("stainless-a4", "Stainless steel A4 screw", "Edelstahlschraube A4", 0.35, 0.6, 7950, 15, 500, 200e9, None, "literature", "iso-10456-2007"),
]

RIG = {
    "kind": "structural",
    "root": "ground",
    "bones": [
        {"id": "ground", "parent": None, "title": t("Bed in the water-bound path (stays behind)", "Bett im wassergebundenen Weg (bleibt liegen)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "block", "parent": "ground", "title": t("Precast concrete block (crane lift along its vertical axis)", "Betonfertigteil-Block (Kranhub entlang der Hochachse)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": L, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.2},
         "mass": {"kg": r(M_BLOCK, 1), "comX": 1.0, "comY": r(COM_Y, 4), "distribution": "uniform", "source": "literature", "reference": "en-1991-1-1-2002"},
         "section": {"areaM2": r(VOL_BLOCK / L, 5), "secondMomentM4": r(D * HC ** 3 / 12, 7), "extremeFibreM": r(HC / 2, 4), "material": "concrete-weathered", "title": t("Concrete section 500 x 405 mm, chamfered", "Betonquerschnitt 500 x 405 mm, gefast")}},
        {"id": "battens", "parent": "block", "title": t("Robinia seat battens", "Sitzleisten aus Robinie"), "x": B0, "y": HC, "angleDeg": 0, "lengthM": BL, "joint": {"type": "fixed"},
         "mass": {"kg": r(M_BATTENS, 1), "comX": BL / 2, "comY": r(TB / 2, 4), "distribution": "uniform", "source": "literature", "reference": "wagenfuehr-holzatlas-2007"},
         "section": {"areaM2": r(VOL_BATTENS / BL, 5), "secondMomentM4": r(NB * BW * TB ** 3 / 12, 9), "extremeFibreM": r(TB / 2, 4), "material": "robinia-greyed", "title": t("Five battens 80 x 45 mm", "Fünf Leisten 80 x 45 mm")}},
    ],
}

ANCHORS = [
    {"id": "bed-west", "bone": "block", "x": 0.5, "y": 0, "kind": "contact", "title": t("West half of the base on the chippings bed", "Westliche Hälfte der Aufstandsfläche auf dem Splittbett"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.0, "footprintWidthM": D, "frictionCoefficient": 0.6},
    {"id": "bed-east", "bone": "block", "x": 1.5, "y": 0, "kind": "contact", "title": t("East half of the base on the chippings bed", "Östliche Hälfte der Aufstandsfläche auf dem Splittbett"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.0, "footprintWidthM": D, "frictionCoefficient": 0.6},
    {"id": "socket-west", "bone": "block", "x": SOCKET_X[0], "y": HC, "kind": "support", "title": t("West threaded lifting socket Rd 24 with the sling leg", "Westliche Transportankerhülse Rd 24 mit Kettenstrang"), "support": "pin"},
    {"id": "socket-east", "bone": "block", "x": SOCKET_X[1], "y": HC, "kind": "support", "title": t("East threaded lifting socket Rd 24 with the sling leg", "Östliche Transportankerhülse Rd 24 mit Kettenstrang"), "support": "pin"},
    {"id": "north-arris", "bone": "block", "x": 1.0, "y": HC, "z": -HW, "kind": "load", "title": t("North top arris where skateboards grind", "Nördliche Oberkante, an der Skateboards grinden")},
]

CONCRETE_REGION = "concrete-block"
REGION_IDS = [CONCRETE_REGION, "seat-battens", "algae-film"]


def bind(conc, bat, alg):
    """🌡️ The region bindings of one pose."""
    return {"regions": {CONCRETE_REGION: conc, "seat-battens": bat, "algae-film": alg}, "emitters": {}}


SERVICE_CONC = {"exposedFraction": 0.45, "sunlitFraction": 0.32, "contact": {"fraction": 0.25, "tempC": 24}}
SERVICE_BAT = {"exposedFraction": 0.69, "sunlitFraction": 0.36, "contact": {"fraction": 0.31, "tempC": 34}}
ALGAE = {"exposedFraction": 1.0, "sunlitFraction": 0.05, "wettedness": 0.4}

SITTERS = [("sitter-south-west", 0.3, 0.1, t("Adult sitting on the south side, west (75 kg)", "Erwachsene Person auf der Südseite, West (75 kg)")),
           ("sitter-south-east", 1.15, 0.1, t("Adult sitting on the south side, east (75 kg)", "Erwachsene Person auf der Südseite, Ost (75 kg)")),
           ("sitter-north", 0.75, -0.1, t("Adult sitting on the north side facing the lawn (75 kg)", "Erwachsene Person auf der Nordseite zur Wiese (75 kg)"))]

POSES = [
    {"id": "in-service", "title": t("In service", "Im Einsatz"), "description": t("The block stands freestanding by its own weight on its chippings bed in the water-bound path, nobody seated, afternoon sun on the south face and the battens.", "Der Block steht frei durch sein Eigengewicht auf seinem Splittbett im wassergebundenen Weg, niemand sitzt, Nachmittagssonne auf der Südseite und den Leisten."),
     "bones": {}, "ground": "auto", "contacts": ["bed-west", "bed-east"], "thermal": bind(SERVICE_CONC, SERVICE_BAT, ALGAE), "loads": []},
    {"id": "maintenance", "title": t("Lifted by the depot crane", "Vom Bauhofkran gehoben"), "description": t("The depot's truck-mounted crane holds the block 0.6 m above its bed on a two-leg chain sling screwed into the two lifting sockets; the bed stays behind with the pale imprint of the base.", "Der Ladekran des Bauhofs hält den Block an einem in die beiden Transportankerhülsen geschraubten Zweistrang-Kettengehänge 0,6 m über seinem Bett; das Bett bleibt mit dem hellen Abdruck der Aufstandsfläche zurück."),
     "bones": {"block": {"dy": 0.6}}, "ground": "auto", "contacts": ["socket-west", "socket-east"], "thermal": bind({"exposedFraction": 0.7, "sunlitFraction": 0.3}, SERVICE_BAT, ALGAE), "loads": []},
    {"id": "in-use", "title": t("Three people sitting in the sun", "Drei Personen in der Sonne"), "description": t("Two adults sit on the south side and one on the north side of the backless block; the sun-heated battens and concrete warm the sitters from below.", "Zwei Erwachsene sitzen auf der Süd- und eine Person auf der Nordseite des lehnenlosen Blocks; die sonnenwarmen Leisten und der Beton wärmen die Sitzenden von unten."),
     "bones": {}, "ground": "auto", "contacts": ["bed-west", "bed-east"], "thermal": bind({"exposedFraction": 0.45, "sunlitFraction": 0.25, "contact": {"fraction": 0.25, "tempC": 24}}, {"exposedFraction": 0.42, "sunlitFraction": 0.2, "contact": {"fraction": 0.27, "tempC": 31}}, ALGAE),
     "loads": [{"id": i, "kind": "occupant", "title": tt, "bone": "battens", "x": x, "y": TB, "z": z, "fxN": 0, "fyN": -SITTER_N, "source": "authored-estimate", "reference": "en-16139-2013"} for i, x, z, tt in SITTERS]},
    {"id": "skate-grind", "title": t("Skateboard grinding the north arris", "Skateboard grindet die Nordkante"), "description": t("A skateboarder slides the trucks of the board along the north top arris, the smooth side along the path: the impact of landing on it and the friction along the edge are what wore it round and chipped it.", "Eine Person fährt mit den Achsen des Boards die nördliche Oberkante entlang, die glatte Seite am Weg: der Aufprall beim Aufspringen und die Reibung entlang der Kante haben sie rund geschliffen und abgeplatzt."),
     "bones": {}, "ground": "auto", "contacts": ["bed-west", "bed-east"], "thermal": bind(SERVICE_CONC, SERVICE_BAT, ALGAE),
     "loads": [{"id": "skate-truck", "kind": "live", "title": t("Board trucks landing and sliding on the arris (80 kg, dynamic factor 2.5, friction 0.3)", "Achsen des Boards landen und gleiten auf der Kante (80 kg, dynamischer Faktor 2,5, Reibung 0,3)"), "anchor": "north-arris", "fxN": r(0.3 * SKATE_N, 1), "fyN": -SKATE_N, "source": "authored-estimate", "reference": "woolley-johns-2001"}]},
]

CLIPS = [
    {"id": "crane-lift", "title": t("Crane lift for maintenance", "Kranhub zur Wartung"), "description": t("The depot crew screws the lifting loops into the two sockets, the crane tensions the chain sling, the block leaves its bed and is held 0.6 m up so the bed can be re-levelled.", "Der Bauhof schraubt die Abhebeschlaufen in die beiden Hülsen, der Kran spannt das Kettengehänge, der Block verlässt sein Bett und wird 0,6 m hoch gehalten, damit das Bett neu abgezogen werden kann."),
     "durationS": 10, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 2, "pose": "in-service", "ease": "smooth"}, {"t": 8, "pose": "maintenance", "ease": "smooth"}, {"t": 10, "pose": "maintenance", "ease": "smooth"}],
     "contacts": [{"anchor": "bed-west", "fromS": 0, "toS": 2.3}, {"anchor": "bed-east", "fromS": 0, "toS": 2.3}, {"anchor": "socket-west", "fromS": 1.6, "toS": 10}, {"anchor": "socket-east", "fromS": 1.6, "toS": 10}],
     "phases": [{"id": "sling-up", "fromS": 0, "toS": 2, "title": t("Sling screwed in and tensioned", "Gehänge eingeschraubt und gespannt")}, {"id": "lift", "fromS": 2, "toS": 8, "title": t("Lift off the bed", "Abheben vom Bett")}, {"id": "hold", "fromS": 8, "toS": 10, "title": t("Held above the bed", "Über dem Bett gehalten")}],
     "events": [{"t": 1.6, "kind": "contact-on", "title": t("Chain sling takes the load", "Kettengehänge übernimmt die Last"), "target": "socket-west"}, {"t": 2.3, "kind": "contact-off", "title": t("Block leaves its bed", "Block verlässt sein Bett"), "target": "bed-west"}],
     "tracks": []},
    {"id": "sitting-in-sun", "title": t("Sitting down on the sun-heated block", "Hinsetzen auf den sonnenwarmen Block"), "description": t("On a summer afternoon three people sit down one after the other; their bodies shade and cover the battens while the concrete keeps the heat of the day.", "An einem Sommernachmittag setzen sich drei Personen nacheinander; ihre Körper beschatten und bedecken die Leisten, während der Beton die Wärme des Tages hält."),
     "durationS": 60, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 6, "pose": "in-use", "ease": "smooth"}, {"t": 60, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": "bed-west", "fromS": 0, "toS": 60}, {"anchor": "bed-east", "fromS": 0, "toS": 60}],
     "phases": [{"id": "sitting-down", "fromS": 0, "toS": 6, "title": t("Sitting down", "Hinsetzen")}, {"id": "seated", "fromS": 6, "toS": 60, "title": t("Seated in the sun", "Sitzen in der Sonne")}],
     "events": [{"t": 2, "kind": "load-on", "title": t("First person sits down", "Erste Person setzt sich"), "target": "sitter-south-west"}, {"t": 4, "kind": "load-on", "title": t("Second person sits down", "Zweite Person setzt sich"), "target": "sitter-north"}, {"t": 6, "kind": "load-on", "title": t("Third person sits down", "Dritte Person setzt sich"), "target": "sitter-south-east"}],
     "tracks": [{"target": "load:sitter-south-west", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 2, "v": 1}, {"t": 60, "v": 1}]},
                {"target": "load:sitter-north", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 2, "v": 0}, {"t": 4, "v": 1}, {"t": 60, "v": 1}]},
                {"target": "load:sitter-south-east", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 0}, {"t": 6, "v": 1}, {"t": 60, "v": 1}]},
                {"target": "region:concrete-block:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.32}, {"t": 60, "v": 0.25}]}]},
    {"id": "skate-grind", "title": t("Skateboard grind along the arris", "Skateboard-Grind entlang der Kante"), "description": t("A skater ollies onto the north arris, grinds along it and rolls off; the block does not move on its bed.", "Eine Person springt mit dem Board auf die Südkante, grindet sie entlang und rollt ab; der Block bewegt sich auf seinem Bett nicht."),
     "durationS": 1.5, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "in"}, {"t": 0.4, "pose": "skate-grind", "ease": "linear"}, {"t": 1.1, "pose": "skate-grind", "ease": "out"}, {"t": 1.5, "pose": "in-service", "ease": "smooth"}],
     "contacts": [{"anchor": "bed-west", "fromS": 0, "toS": 1.5}, {"anchor": "bed-east", "fromS": 0, "toS": 1.5}],
     "phases": [{"id": "ollie", "fromS": 0, "toS": 0.4, "title": t("Ollie onto the arris", "Ollie auf die Kante")}, {"id": "grind", "fromS": 0.4, "toS": 1.1, "title": t("Grind", "Grind")}, {"id": "roll-off", "fromS": 1.1, "toS": 1.5, "title": t("Roll off", "Abrollen")}],
     "events": [{"t": 0.4, "kind": "load-on", "title": t("Trucks land on the arris", "Achsen landen auf der Kante"), "target": "skate-truck"}, {"t": 1.1, "kind": "load-off", "title": t("Board leaves the arris", "Board verlässt die Kante"), "target": "skate-truck"}],
     "tracks": []},
]


def layer(i, en, de, kind, m, thick, k, rho, cp):
    """🧶️ One region layer with its one-hour areal heat capacity rho*cp*min(d, sqrt(alpha*3600))."""
    a = k / (rho * cp)
    dpen = min(thick, math.sqrt(a * 3600))
    return {"id": i, "title": t(en, de), "kind": kind, "material": m, "thicknessM": thick, "resistanceM2KW": r(thick / k, 5), "heatCapacityJM2K": round(rho * cp * dpen), "source": "literature", "reference": "iso-10456-2007"}


L_CONC = layer("concrete-mass", "Solid precast concrete", "Massiver Fertigteilbeton", "masonry", "concrete-weathered", 0.25, 2.3, RHO_C, 1000)
L_BAT = layer("robinia-batten", "Robinia batten", "Robinienleiste", "timber", "robinia-greyed", TB, 0.18, RHO_T, 1600)
L_ALG = layer("algae-biofilm", "Wet algal biofilm", "Nasser Algenbelag", "shell", "algae-biofilm", SHEET, 0.6, 1050, 3500)

THERMAL = {
    "environment": {"airTempC": 28, "meanRadiantTempC": 31, "skyTempC": 15, "groundTempC": 38, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 90, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325},
    "referenceEnvironment": "summer-hot-sun",
    "passengers": [],
    "operating": [],
    "regions": [
        {"id": CONCRETE_REGION, "title": t("Weathered concrete block", "Verwitterter Betonblock"), "kind": "surface", "model": "passive", "material": "concrete-weathered", "areaM2": CONCRETE_AREA, "characteristicLengthM": HC, "geometry": "vertical-plate", "skyViewFactor": 0.55, "layers": [L_CONC], "heatCapacityJM2K": L_CONC["heatCapacityJM2K"], "source": "literature", "reference": "levinson-akbari-2002"},
        {"id": "seat-battens", "title": t("Greyed robinia seat battens", "Vergraute Robinien-Sitzleisten"), "kind": "surface", "model": "passive", "material": "robinia-greyed", "areaM2": BATTEN_AREA, "characteristicLengthM": BW, "geometry": "horizontal-plate-up", "skyViewFactor": 0.95, "layers": [L_BAT], "heatCapacityJM2K": L_BAT["heatCapacityJM2K"], "source": "authored-estimate", "reference": "feist-hon-1984"},
        {"id": "algae-film", "title": t("Algae film on the shaded north face", "Algenbelag auf der schattigen Nordseite"), "kind": "surface", "model": "passive", "material": "algae-biofilm", "areaM2": ALGAE_AREA, "characteristicLengthM": 0.3, "geometry": "vertical-plate", "skyViewFactor": 0.45, "layers": [L_ALG], "heatCapacityJM2K": L_ALG["heatCapacityJM2K"], "source": "authored-estimate", "reference": "gaylarde-morton-1999"},
    ],
    "emitters": [],
    "assumptions": [
        t("Passive surfaces only: absorbed sun, long-wave exchange with sky and ground, convection and heat capacity; the block draws no power (0 W) and has no emitters.", "Nur passive Flächen: absorbierte Sonne, Langwellenaustausch mit Himmel und Boden, Konvektion und Wärmekapazität; der Block nimmt keine Leistung auf (0 W) und hat keine Emitter."),
        t("Weathered surfaces differ from new ones: soiled, weathered grey concrete absorbs about 0.65 of the sun instead of about 0.55 new (Levinson and Akbari 2002); robinia greyed by UV and leaching absorbs about 0.6 instead of about 0.7 for the fresh yellow-brown wood (Feist and Hon 1984); the wet algal film on the shaded north face absorbs about 0.8 and is kept moist (wettedness 0.4).", "Verwitterte Flächen unterscheiden sich von neuen: verschmutzter, verwitterter grauer Beton absorbiert etwa 0,65 der Sonne statt etwa 0,55 im Neuzustand (Levinson und Akbari 2002); durch UV und Auswaschung vergraute Robinie absorbiert etwa 0,6 statt etwa 0,7 beim frischen gelbbraunen Holz (Feist und Hon 1984); der nasse Algenbelag auf der schattigen Nordseite absorbiert etwa 0,8 und bleibt feucht (Benetzung 0,4)."),
        t("Areal heat capacity is rho*cp*min(thickness, sqrt(alpha*3600 s)): concrete takes part with 58 mm within an hour, so the 1 t block lags hours behind the sun and stays warm into the evening, while the 45 mm battens follow it within the hour (23 mm).", "Die flächenbezogene Wärmekapazität ist rho*cp*min(Dicke, sqrt(alpha*3600 s)): Beton nimmt binnen einer Stunde mit 58 mm teil, so dass der 1-t-Block der Sonne um Stunden nachläuft und bis in den Abend warm bleibt, während die 45-mm-Leisten ihr binnen einer Stunde folgen (23 mm)."),
        t("areaM2 is the total outer surface of each region's 1:50 solids: the concrete including its base on the bed and the top under the battens, the battens including their undersides, the algae film without its face against the concrete. Contact fractions: the base on the chippings bed at 24 degC, the batten undersides on the sun-warmed concrete at 34 degC, sitters' clothing at 31 degC (ISO 7730 range).", "areaM2 ist die gesamte Außenfläche der 1:50-Körper jeder Region: der Beton einschließlich seiner Aufstandsfläche auf dem Bett und der Oberseite unter den Leisten, die Leisten einschließlich ihrer Unterseiten, der Algenbelag ohne seine Fläche am Beton. Kontaktanteile: die Aufstandsfläche auf dem Splittbett bei 24 degC, die Leistenunterseiten auf dem sonnenwarmen Beton bei 34 degC, die Kleidung Sitzender bei 31 degC (Bereich nach ISO 7730)."),
        t("Reference case: a German summer afternoon, 28 degC air, 800 W/m2 sun from the south-west, 1.5 m/s breeze, hot water-bound gravel around it at 38 degC; sunlit fractions are scenario data of each pose and clip.", "Referenzfall: deutscher Sommernachmittag, 28 degC Luft, 800 W/m2 Sonne aus Südwest, 1,5 m/s Wind, heiße wassergebundene Decke ringsum bei 38 degC; besonnte Anteile sind Szenariodaten jeder Pose und jedes Clips."),
    ],
}

STRUCTURE = {
    "gravityMS2": G,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        t("In service the block is a rigid body freestanding by its own weight (about 1 t) on its chippings bed, modelled as two unilateral contacts of 1.0 x 0.5 m each with friction 0.6 (concrete on chippings); it has no anchorage, so only its mass resists sliding and overturning.", "Im Einsatz ist der Block ein starrer Körper, der frei durch sein Eigengewicht (etwa 1 t) auf seinem Splittbett steht, abgebildet als zwei einseitige Kontakte von je 1,0 x 0,5 m mit Reibung 0,6 (Beton auf Splitt); er ist nicht verankert, nur seine Masse hält ihn gegen Gleiten und Kippen."),
        t("Maintenance: the crane holds the block on two cast-in threaded lifting sockets Rd 24, 0.1 m from each end (pins at the socket heads); the two-leg chain sling and the crane belong to the depot, not to the asset. Each socket carries half the weight, about 5 kN, times the dynamic factor 1.3 of VDI/BV-BS 6205 for a crane lift; the block spans 1.8 m between them as a beam under its own weight.", "Wartung: Der Kran hält den Block an zwei einbetonierten Transportankerhülsen Rd 24, 0,1 m von jedem Ende (Gelenke an den Hülsenköpfen); Zweistrang-Kettengehänge und Kran gehören dem Bauhof, nicht zum Asset. Jede Hülse trägt die Hälfte des Gewichts, etwa 5 kN, mal dem dynamischen Faktor 1,3 nach VDI/BV-BS 6205 für einen Kranhub; der Block spannt dazwischen 1,8 m als Balken unter seinem Eigengewicht."),
        t("In use: seated adults are 75 kg point loads on the battens (EN 16139 tests public seating at higher static loads; this is the service case, not the test case); the skateboard grind is an authored estimate of 80 kg times a dynamic factor 2.5 with friction 0.3 at the south arris.", "In Benutzung: sitzende Erwachsene sind Einzellasten von 75 kg auf den Leisten (EN 16139 prüft öffentliche Sitzmöbel mit höheren statischen Lasten; dies ist der Gebrauchsfall, nicht der Prüffall); der Skateboard-Grind ist eine Schätzung von 80 kg mal dynamischem Faktor 2,5 mit Reibung 0,3 an der Südkante."),
        t("Members: the concrete block (C35/45, section 500 x 405 mm) and the five robinia battens (80 x 45 mm, D40), screwed to the concrete by stainless A4 screws in plastic dowels on three lines; masses are section times density (concrete 2500, robinia 760, steel 7850 kg/m3).", "Glieder: der Betonblock (C35/45, Querschnitt 500 x 405 mm) und die fünf Robinienleisten (80 x 45 mm, D40), mit Edelstahlschrauben A4 in Kunststoffdübeln auf drei Linien am Beton verschraubt; Massen sind Querschnitt mal Dichte (Beton 2500, Robinie 760, Stahl 7850 kg/m3)."),
    ],
    "clinical": False,
}

TAGS = ["used", "park", "public-space", "seating", "bench", "street-furniture", "precast-concrete", "concrete", "timber", "robinia", "seating-block", "skateboarding", "weathered"]

SUMMARY = t(
    "A precast concrete seating block of the municipal catalogue type, 2.0 x 0.5 m and 0.45 m high to the seat, about 1 t, freestanding by its own weight on a chippings bed in a water-bound park path, with five greyed and checked robinia battens screwed along its top. Years of service left the north top arris ground round and chipped by skateboards, a green algae film on the shaded north face, rust halos at the two lifting sockets, sticker residue and chewing gum; the depot lifts it by crane on those sockets, and in summer it is a warm seat for people from both sides.",
    "Ein Betonsitzblock aus Fertigteil vom Typ der kommunalen Kataloge, 2,0 x 0,5 m und 0,45 m Sitzhöhe, etwa 1 t, frei durch sein Eigengewicht auf einem Splittbett in einem wassergebundenen Parkweg stehend, mit fünf vergrauten, rissigen Robinienleisten oben aufgeschraubt. Die Jahre im Einsatz haben die nördliche Oberkante von Skateboards rund geschliffen und abgeplatzt, einen grünen Algenbelag auf der schattigen Nordseite, Rosthöfe um die beiden Transportankerhülsen, Aufkleberreste und Kaugummi hinterlassen; der Bauhof hebt ihn an diesen Hülsen mit dem Kran, und im Sommer ist er ein warmer Sitz für Menschen von beiden Seiten.")

PROVENANCE = {
    "author": "Semio staffage content (used park objects wave)",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": [{"id": i, "citation": c} for i, c in REFS],
    "notes": t(
        f"Catalogue type of German municipal precast seating blocks (no brand): concrete C35/45 XC4/XF4 body 2000 x 500 x 405 mm with 15 mm chamfers on every edge, five robinia battens 80 x 45 mm with 12 mm gaps on 1.6 m of the seat, leaving the 0.2 m concrete ends bare with one threaded lifting socket Rd 24 each; seat 0.45 m (DIN 18040-3). Masses from the modelled 1:50 volumes: concrete {VOL_BLOCK:.4f} m3 x 2500 = {VOL_BLOCK * RHO_C:.1f} kg plus sockets {M_SOCKETS:.1f} kg, battens {VOL_BATTENS:.4f} m3 x 760 = {VOL_BATTENS * RHO_T:.1f} kg plus screws {M_SCREWS:.2f} kg, total {M_TOTAL:.1f} kg. Traces of use: the south arris worn from the 15 mm chamfer to 24-42 mm rounds with two chips over 1.5 m by skateboard grinds, a 2 mm algae film over the lower 0.2-0.3 m of the north face and round the north-east corner, battens greyed with checks and a split end, rust halos, gum and stickers as texture. All numbers are literature or authored estimates; nothing is measured.",
        f"Katalogtyp der deutschen kommunalen Betonsitzblöcke aus Fertigteil (ohne Marke): Betonkörper C35/45 XC4/XF4 2000 x 500 x 405 mm mit 15-mm-Fasen an allen Kanten, fünf Robinienleisten 80 x 45 mm mit 12-mm-Fugen auf 1,6 m der Sitzfläche, so dass die 0,2 m langen Betonenden mit je einer Transportankerhülse Rd 24 frei bleiben; Sitzhöhe 0,45 m (DIN 18040-3). Massen aus den modellierten 1:50-Volumen: Beton {VOL_BLOCK:.4f} m3 x 2500 = {VOL_BLOCK * RHO_C:.1f} kg plus Hülsen {M_SOCKETS:.1f} kg, Leisten {VOL_BATTENS:.4f} m3 x 760 = {VOL_BATTENS * RHO_T:.1f} kg plus Schrauben {M_SCREWS:.2f} kg, zusammen {M_TOTAL:.1f} kg. Gebrauchsspuren: die Südkante auf 1,5 m von der 15-mm-Fase zu 24-42 mm Rundungen mit zwei Abplatzungen von Skateboard-Grinds abgeschliffen, ein 2 mm Algenbelag über die unteren 0,2-0,3 m der Nordseite und um die Nordostecke, Leisten vergraut mit Rissen und einem gespaltenen Ende, Rosthöfe, Kaugummi und Aufkleber als Textur. Alle Zahlen sind Literatur- oder Schätzwerte; nichts ist gemessen."),
}

T_CONC, T_BAT, T_ALG, T_RUST, T_GUM, T_STICK, T_STEEL, T_WEAR = 0.58, 0.66, 0.36, 0.42, 0.25, 0.85, 0.2, 0.44


def shape(i, bone, role, z, filled, tone, geom, region=CONCRETE_REGION, material="concrete-weathered", **kw):
    """✏️ One 2D shape."""
    s = {"id": i, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    s["region"] = region
    s["material"] = material
    s.update(kw)
    return s


def poly(points, closed=True):
    """🔺️ Poly geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rect(x, y, w, h, rr=None):
    """⬛️ Rect geometry."""
    g = {"kind": "rect", "x": r(x), "y": r(y), "w": r(w), "h": r(h)}
    if rr:
        g["r"] = rr
    return g


def ell(cx, cy, rx, ry):
    """⚪️ Ellipse geometry."""
    return {"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(rx), "ry": r(ry)}


def bat_kw():
    """🪵️ Region and material of a batten shape."""
    return {"region": "seat-battens", "material": "robinia-greyed"}


def alg_kw():
    """🦠️ Region and material of an algae shape."""
    return {"region": "algae-film", "material": "algae-biofilm"}


ELEV_BLOCK = [(0, C), (C, 0), (L - C, 0), (L, C), (L, HC - C), (L - C, HC), (C, HC), (0, HC - C)]
FRONT_BLOCK = [(-HW + C, 0), (HW - C, 0), (HW, C), (HW, HC - C), (HW - C, HC), (-HW + C, HC), (-HW, HC - C), (-HW, C)]
PLAN_BLOCK = [(0, -HW + C), (C, -HW), (L - C, -HW), (L, -HW + C), (L, HW - C), (L - C, HW), (C, HW), (0, HW - C)]
INSET = C
CI = C - INSET * (2 - math.sqrt(2))
FRONT_INNER = [(-HW + INSET + CI, INSET), (HW - INSET - CI, INSET), (HW - INSET, INSET + CI), (HW - INSET, HC - INSET - CI), (HW - INSET - CI, HC - INSET), (-HW + INSET + CI, HC - INSET), (-HW + INSET, HC - INSET - CI), (-HW + INSET, INSET + CI)]


def wear_line(table):
    """🛹️ The lower edge of the worn north arris seen in elevation, at the stations of the table between 0.015 and 1.985 m."""
    xs = sorted(set([C, L - C] + [x for x, _, _ in table if C < x < L - C]))
    return [(x, HC - wear_at(table, x)[0]) for x in xs]


def plan_wear_line(table):
    """🛹️ The inner edge of the top face on the worn north side seen in plan."""
    xs = sorted(set([C, L - C] + [x for x, _, _ in table if C < x < L - C]))
    return [(x, -HW + wear_at(table, x)[0]) for x in xs]


def front_batten(k, ease):
    """🪵️ The front outline of batten k (world z, height)."""
    z0 = BZ0 + k * PITCH
    z1 = z0 + BW
    if ease <= 0:
        return [(z0, HC), (z1, HC), (z1, H), (z0, H)]
    return [(z0, HC), (z1, HC), (z1, H - ease), (z1 - ease, H), (z0 + ease, H), (z0, H - ease)]


GUM = [(0.47, BZC[0] + 0.02), (0.92, BZC[1] - 0.015), (1.38, BZC[3] + 0.01), (1.63, BZC[0] - 0.02), (0.71, BZC[4] + 0.02), (1.12, BZC[2] - 0.02)]
STREAKS = [[(0.34, 0.392), (0.52, 0.388)], [(0.58, 0.39), (0.84, 0.385)], [(0.9, 0.392), (1.18, 0.389)], [(1.22, 0.39), (1.37, 0.386)], [(1.46, 0.391), (1.66, 0.388)], [(0.42, 0.383), (0.66, 0.38)], [(1.02, 0.384), (1.31, 0.381)]]


def decisions(silhouette, retained, merged, omitted, thin, hatching, styles):
    """📝️ The eight decision texts of a representation."""
    return {"silhouette": silhouette, "retained": retained, "merged": merged, "omitted": omitted, "thinElements": thin, "hatching": hatching, "styles": styles,
            "overlays": t("Every shape carries one region (concrete-block, seat-battens or algae-film) and its material, so the thermal view paints the concrete, the battens and the algae film apart.", "Jede Form trägt eine Region (concrete-block, seat-battens oder algae-film) und ihr Material, so dass die thermische Ansicht Beton, Leisten und Algenbelag getrennt färbt.")}


HATCHES_50 = [{"id": "concrete-dots", "title": t("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.9, "penMm": 0.1},
              {"id": "grain", "title": t("Weathered grain", "Verwitterte Maserung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.1}]
HATCHES_100 = [{"id": "grain", "title": t("Weathered grain", "Verwitterte Maserung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.45, "penMm": 0.1}]


def rep50():
    """🖨️ The 1:50 drawings."""
    elev = [
        shape("block", "block", "body", 1, True, T_CONC, poly(ELEV_BLOCK), hatch="concrete-dots"),
        shape("algae-north", "block", "detail", 1.1, True, T_ALG, poly(ALGAE_FACE[50]), **alg_kw()),
        shape("end-chamfers", "block", "detail", 1.2, False, 0.3, {"kind": "path", "d": f"M {C} {C} L {C} {HC - C} M {L - C} {C} L {L - C} {HC - C} M {C} {C} L {L - C} {C}"}),
        shape("arris-wear", "block", "detail", 1.3, False, 0.25, poly(wear_line(WEAR_50), closed=False)),
        shape("grind-streaks", "block", "texture", 1.4, False, 0.12, {"kind": "path", "d": " ".join(f"M {r(a[0])} {r(a[1])} L {r(b[0])} {r(b[1])}" for a, b in STREAKS)}),
        shape("sticker-residue", "block", "texture", 1.5, True, T_STICK, rect(1.53, 0.17, 0.075, 0.095)),
        shape("battens", "battens", "body", 2, True, T_BAT, rect(0, 0, BL, TB), hatch="grain", **bat_kw()),
        shape("batten-ease", "battens", "detail", 2.1, False, 0.35, poly([(0, TB - EASE), (BL, TB - EASE)], closed=False), **bat_kw()),
        shape("batten-checks", "battens", "texture", 2.2, False, 0.2, {"kind": "path", "d": "M 0.12 0.026 L 0.43 0.024 M 0.71 0.017 L 0.98 0.02 M 1.24 0.03 L 1.51 0.027 M 1.45 0.012 L 1.6 0.015"}, **bat_kw()),
    ]
    front = [
        shape("block", "block", "body", 1, True, T_CONC, poly(FRONT_BLOCK), hatch="concrete-dots"),
        shape("end-face-edge", "block", "detail", 1.2, False, 0.3, poly(FRONT_INNER)),
        shape("algae-end", "block", "detail", 1.5, True, T_ALG, poly([(-w, v) for v, w in ALGAE_END[50]]), **alg_kw()),
        shape("end-sticker", "block", "texture", 1.6, True, T_STICK, rect(0.08, 0.2, 0.09, 0.06)),
    ] + [shape(f"batten-{k + 1}", "battens", "body", 2, True, T_BAT, poly(front_batten(k, EASE)), penMm=0.25, **bat_kw()) for k in range(NB)]
    plan = [
        shape("block", "block", "body", 1, True, T_CONC, poly(PLAN_BLOCK), hatch="concrete-dots"),
        shape("top-face-edge", "block", "detail", 1.2, False, 0.3, poly([(C, HW - C)] + plan_wear_line(WEAR_50) + [(L - C, HW - C)])),
    ] + [shape(f"rust-halo-{i + 1}", "block", "texture", 1.3, True, T_RUST, ell(x, 0.012, 0.045, 0.034)) for i, x in enumerate(SOCKET_X)] \
      + [shape(f"socket-{i + 1}", "block", "detail", 1.4, True, T_STEEL, ell(x, 0, 0.02, 0.02), material="steel-galvanised") for i, x in enumerate(SOCKET_X)] \
      + [shape(f"batten-{k + 1}", "battens", "body", 2, True, T_BAT, rect(B0, BZ0 + k * PITCH, BL, BW), hatch="grain", penMm=0.25, **bat_kw()) for k in range(NB)] \
      + [shape("screw-heads", "battens", "texture", 2.3, False, 0.15, {"kind": "path", "d": " ".join(f"M {r(B0 + u - 0.006)} {r(z)} L {r(B0 + u + 0.006)} {r(z)}" for u in FIX_U for z in BZC)}, **bat_kw())] \
      + [shape(f"gum-{i + 1}", "battens", "texture", 2.4, True, T_GUM, ell(x, z, 0.009, 0.008), **bat_kw()) for i, (x, z) in enumerate(GUM)] \
      + [shape("batten-split", "battens", "texture", 2.5, False, 0.15, poly([(B0 + BL, BZC[0] + 0.004), (B0 + BL - 0.09, BZC[0] + 0.001), (B0 + BL - 0.17, BZC[0] - 0.003)], closed=False), **bat_kw())]
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75},
            "pens": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, "hatches": HATCHES_50,
            "geometry": {"elevation": elev, "front": front, "plan": plan},
            "decisions": decisions(
                t("Block 40 x 9 mm on paper with its whole construction: in elevation (the shaded north face) the 15 mm chamfers of every edge, the arris ground round over 1.5 m with its two chips as a line wandering 24-42 mm under the top, the grind streaks, a sticker and the five battens seen as one 45 mm strip with the eased edge and checks. Front (east end): the chamfered end face, the five eased batten ends with their 12 mm gaps and the algae film spreading round the north-east corner. Plan: the 15 mm top chamfers widening on the north side where the arris is worn, both lifting sockets in the bare ends with their rust halos, five battens with screw heads on three lines, gum spots and a split batten end.", "Block 40 x 9 mm auf dem Papier mit seiner ganzen Konstruktion: in der Ansicht (die schattige Nordseite) die 15-mm-Fasen aller Kanten, die auf 1,5 m rund geschliffene Kante mit ihren zwei Abplatzungen als Linie, die 24-42 mm unter der Oberseite wandert, die Grind-Striemen, ein Aufkleber und die fünf Leisten als ein 45-mm-Streifen mit gebrochener Kante und Rissen. Vorderansicht (Ostende): die gefaste Stirnfläche, die fünf gebrochenen Leistenenden mit ihren 12-mm-Fugen und der Algenbelag, der um die Nordostecke wächst. Draufsicht: die 15-mm-Oberkantenfasen, die auf der Nordseite breiter werden, wo die Kante abgeschliffen ist, beide Transportankerhülsen in den freien Enden mit ihren Rosthöfen, fünf Leisten mit Schraubenköpfen auf drei Linien, Kaugummiflecken und ein gespaltenes Leistenende."),
                t("Every chamfer, the worn arris and its chips, the batten gaps and eased edges, the sockets, the algae edge.", "Jede Fase, die abgeschliffene Kante mit ihren Abplatzungen, die Leistenfugen und gebrochenen Kanten, die Hülsen, der Rand des Algenbelags."),
                t("The five battens coincide in the side elevation; the 2 mm algae film is drawn as a patch on the north face in elevation and where it turns onto the east end in the front view; in the plan it lies edge-on in the outline.", "Die fünf Leisten decken sich in der Seitenansicht; der 2-mm-Algenbelag ist in der Ansicht als Fleck auf der Nordseite und in der Vorderansicht dort gezeichnet, wo er auf die Ostseite übergeht; in der Draufsicht liegt er auf der Kante im Umriss."),
                t("The plastic dowels, the screw shanks and the cast-in socket bodies inside the concrete.", "Die Kunststoffdübel, die Schraubenschäfte und die einbetonierten Hülsenkörper im Beton."),
                t("The 12 mm batten gaps (0.24 mm) stay open between filled batten ends; the arris wear, the grind streaks, the checks and the screw heads are strokes, the sticker, rust halos and gum spots small filled texture.", "Die 12-mm-Leistenfugen (0,24 mm) bleiben zwischen den gefüllten Leistenenden offen; Kantenverschleiß, Grind-Striemen, Risse und Schraubenköpfe sind Striche, Aufkleber, Rosthöfe und Kaugummiflecken kleine gefüllte Textur."),
                t("Concrete stipple of dots every 0.9 mm; weathered grain lines every 0.5 mm along the battens.", "Betonpunktierung alle 0,9 mm; verwitterte Maserungslinien alle 0,5 mm entlang der Leisten."),
                t("Concrete mid grey 0.58, greyed robinia lighter 0.66, algae dark 0.36, rust 0.42, gum near black, sticker residue pale.", "Beton mittelgrau 0,58, vergraute Robinie heller 0,66, Algen dunkel 0,36, Rost 0,42, Kaugummi fast schwarz, Aufkleberreste hell."))}


def rep100():
    """🖨️ The 1:100 drawings."""
    elev = [
        shape("block", "block", "body", 1, True, T_CONC, poly(ELEV_BLOCK)),
        shape("algae-north", "block", "detail", 1.1, True, T_ALG, poly(ALGAE_FACE[100]), **alg_kw()),
        shape("arris-wear", "block", "detail", 1.3, False, 0.25, poly(wear_line(WEAR_100), closed=False)),
        shape("battens", "battens", "body", 2, True, T_BAT, rect(0, 0, BL, TB), hatch="grain", **bat_kw()),
    ]
    front = [
        shape("block", "block", "body", 1, True, T_CONC, poly(FRONT_BLOCK)),
        shape("algae-end", "block", "detail", 1.5, True, T_ALG, poly([(-w, v) for v, w in ALGAE_END[100]]), **alg_kw()),
    ] + [shape(f"batten-{k + 1}", "battens", "body", 2, True, T_BAT, poly(front_batten(k, 0)), **bat_kw()) for k in range(NB)]
    plan = [
        shape("block", "block", "body", 1, True, T_CONC, poly(PLAN_BLOCK)),
        shape("top-face-edge", "block", "detail", 1.2, False, 0.3, poly([(C, HW - C)] + plan_wear_line(WEAR_100) + [(L - C, HW - C)])),
    ] + [shape(f"socket-{i + 1}", "block", "detail", 1.4, True, T_STEEL, ell(x, 0, 0.02, 0.02), material="steel-galvanised") for i, x in enumerate(SOCKET_X)] \
      + [shape(f"batten-{k + 1}", "battens", "body", 2, True, T_BAT, rect(B0, BZ0 + k * PITCH, BL, BW), **bat_kw()) for k in range(NB)]
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150},
            "pens": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, "hatches": HATCHES_100,
            "geometry": {"elevation": elev, "front": front, "plan": plan},
            "decisions": decisions(
                t("Block 20 x 4.5 mm: the chamfered outline, the worn arris as one line generalised to a 28 mm round over 1.3 m, the battens as one strip; front with the five square batten ends and the algae patch; plan with the battens, the generalised wear on the top face edge and both sockets.", "Block 20 x 4,5 mm: der gefaste Umriss, die abgeschliffene Kante als eine Linie, verallgemeinert zu einer 28-mm-Rundung über 1,3 m, die Leisten als ein Streifen; Vorderansicht mit den fünf rechtwinkligen Leistenenden und dem Algenfleck; Draufsicht mit den Leisten, dem verallgemeinerten Verschleiß an der Oberkante und beiden Hülsen."),
                t("The 15 mm chamfers (0.15 mm), the 12 mm batten gaps, the sockets, the wear line.", "Die 15-mm-Fasen (0,15 mm), die 12-mm-Leistenfugen, die Hülsen, die Verschleißlinie."),
                t("The two chips merge into the generalised wear; the eased batten edges become square.", "Die zwei Abplatzungen gehen im verallgemeinerten Verschleiß auf; die gebrochenen Leistenkanten werden rechtwinklig."),
                t("Grind streaks, checks, screw heads, gum, sticker and rust halos (below 0.15 mm or meaningless at this size); the end-face chamfer line.", "Grind-Striemen, Risse, Schraubenköpfe, Kaugummi, Aufkleber und Rosthöfe (unter 0,15 mm oder bei dieser Größe bedeutungslos); die Fasenlinie der Stirnfläche."),
                t("Batten gaps of 0.12 mm stay open between the filled batten ends and plan strips.", "Leistenfugen von 0,12 mm bleiben zwischen den gefüllten Leistenenden und Draufsichtstreifen offen."),
                t("Grain lines along the battens in elevation only; concrete as plain tone.", "Maserungslinien entlang der Leisten nur in der Ansicht; Beton als glatter Ton."),
                t("Same tones as 1:50.", "Gleiche Töne wie 1:50."))}


def merged_elev(top_c):
    """🧱️ The elevation outline of block and seat as one mass for the coarse scales (concrete ends at top_c, seat to H between 0.2 and 1.8 m)."""
    return [(0, 0), (L, 0), (L, top_c), (B0 + BL, top_c), (B0 + BL, H), (B0, H), (B0, top_c), (0, top_c)]


def merged_front(top_c, half):
    """🧱️ The front outline of block and seat as one mass for the coarse scales."""
    pts_ = [(-HW, 0), (HW, 0), (HW, top_c)]
    if half < HW:
        pts_ += [(half, top_c)]
    pts_ += [(half, H), (-half, H)]
    if half < HW:
        pts_ += [(-half, top_c)]
    pts_ += [(-HW, top_c)]
    return pts_


def rep200():
    """🖨️ The 1:200 drawings."""
    both = {"regions": [CONCRETE_REGION, "seat-battens"]}
    elev = [
        {"id": "block", "bone": "block", "role": "body", "z": 1, "filled": True, "tone": T_CONC, **poly(merged_elev(HC)), **both, "material": "concrete-weathered"},
        shape("seat-joint", "block", "detail", 1.2, False, 0.3, poly([(B0, HC), (B0 + BL, HC)], closed=False), **bat_kw()),
        shape("algae-north", "block", "detail", 1.1, True, T_ALG, poly(ALGAE_FACE[200]), **alg_kw()),
    ]
    front = [
        {"id": "block", "bone": "block", "role": "body", "z": 1, "filled": True, "tone": T_CONC, **poly(merged_front(HC, DECK / 2)), **both, "material": "concrete-weathered"},
        shape("seat-joint", "block", "detail", 1.2, False, 0.3, {"kind": "path", "d": f"M {r(-DECK / 2)} {HC} L {r(DECK / 2)} {HC}" + "".join(f" M {r(BZ0 + k * PITCH - GAP / 2)} {HC} L {r(BZ0 + k * PITCH - GAP / 2)} {H}" for k in range(1, NB))}, **bat_kw()),
        shape("algae-end", "block", "detail", 1.5, True, T_ALG, poly([(-w, v) for v, w in ALGAE_END[200]]), **alg_kw()),
    ]
    plan = [
        shape("block", "block", "body", 1, True, T_CONC, rect(0, -HW, L, D)),
        shape("deck", "battens", "body", 2, True, T_BAT, rect(B0, BZ0, BL, DECK), **bat_kw()),
        shape("deck-joints", "battens", "detail", 2.1, False, 0.3, {"kind": "path", "d": " ".join(f"M {B0} {r(BZ0 + k * PITCH - GAP / 2)} L {B0 + BL} {r(BZ0 + k * PITCH - GAP / 2)}" for k in range(1, NB))}, **bat_kw()),
    ]
    return {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350},
            "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [],
            "geometry": {"elevation": elev, "front": front, "plan": plan},
            "decisions": decisions(
                t("Block 10 x 2.25 mm: one stepped outline of concrete and seat in elevation and front (the 45 mm seat alone would print 0.13 mm at 1:350), the seat joint as a line, the batten joints as lines; plan with the bare concrete ends and the seat deck.", "Block 10 x 2,25 mm: ein abgestufter Umriss aus Beton und Sitz in Ansicht und Vorderansicht (der 45-mm-Sitz allein würde bei 1:350 mit 0,13 mm gedruckt), die Sitzfuge als Linie, die Leistenfugen als Linien; Draufsicht mit den freien Betonenden und dem Sitzdeck."),
                t("The 45 mm seat step at the bare ends and along the 26 mm side bands, the batten joints, the algae patch on the east end.", "Die 45-mm-Sitzstufe an den freien Enden und entlang der 26-mm-Randstreifen, die Leistenfugen, der Algenfleck an der Ostseite."),
                t("Concrete and seat are one shape in elevation and front; the five battens one deck with joint lines.", "Beton und Sitz sind in Ansicht und Vorderansicht eine Form; die fünf Leisten ein Deck mit Fugenlinien."),
                t("The 15 mm chamfers (0.075 mm), the arris wear, the sockets, every texture.", "Die 15-mm-Fasen (0,075 mm), der Kantenverschleiß, die Hülsen, jede Textur."),
                t("The batten gaps are drawn as 0.13 mm joint lines, not as openings.", "Die Leistenfugen werden als 0,13-mm-Fugenlinien gezeichnet, nicht als Öffnungen."),
                t("No hatches.", "Keine Schraffuren."),
                t("Concrete 0.58 for the merged mass, seat deck 0.66 in plan, algae 0.36.", "Beton 0,58 für die zusammengefasste Masse, Sitzdeck 0,66 in der Draufsicht, Algen 0,36."))}


TOP500 = 0.375


def rep500():
    """🖨️ The 1:500 drawings."""
    both = {"regions": [CONCRETE_REGION, "seat-battens"]}
    elev = [{"id": "block", "bone": "block", "role": "body", "z": 1, "filled": True, "tone": T_CONC, **poly(merged_elev(TOP500)), **both, "material": "concrete-weathered"}, shape("algae-north", "block", "body", 1.1, True, T_ALG, poly(ALGAE_FACE[500]), **alg_kw())]
    front = [
        {"id": "block", "bone": "block", "role": "body", "z": 1, "filled": True, "tone": T_CONC, **rect(-HW, 0, D, H), **both, "material": "concrete-weathered"},
        shape("algae-end", "block", "body", 1.5, True, T_ALG, poly([(-w, v) for v, w in ALGAE_END[500]]), **alg_kw()),
    ]
    plan = [
        shape("block", "block", "body", 1, True, T_CONC, rect(0, -HW, L, D)),
        shape("seat", "battens", "body", 2, True, T_BAT, rect(B0, -HW, BL, D), **bat_kw()),
    ]
    return {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000},
            "pens": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [],
            "geometry": {"elevation": elev, "front": front, "plan": plan},
            "decisions": decisions(
                t("Block 4 x 0.9 mm: the stepped long outline of the seat over the bare ends (generalised to a 75 mm step), the end as one rectangle with the algae patch, the plan as the concrete with the seat over its middle 1.6 m.", "Block 4 x 0,9 mm: der abgestufte Längsumriss des Sitzes über den freien Enden (verallgemeinert zu einer 75-mm-Stufe), das Ende als ein Rechteck mit dem Algenfleck, die Draufsicht als Beton mit dem Sitz über den mittleren 1,6 m."),
                t("Length, depth, seat height, the bare concrete ends and the algae on the shaded corner.", "Länge, Tiefe, Sitzhöhe, die freien Betonenden und die Algen an der schattigen Ecke."),
                t("The battens and the 26 mm concrete side bands merge into one seat course over the full depth.", "Die Leisten und die 26-mm-Betonrandstreifen gehen in einer Sitzlage über die ganze Tiefe auf."),
                t("Chamfers, joints, wear, sockets and texture (all below 0.15 mm).", "Fasen, Fugen, Verschleiß, Hülsen und Textur (alle unter 0,15 mm)."),
                t("No thin elements remain; the algae patch keeps 0.25 x 0.2 m so it prints 0.2 mm at 1:1000.", "Es bleiben keine dünnen Elemente; der Algenfleck behält 0,25 x 0,2 m, so dass er bei 1:1000 mit 0,2 mm gedruckt wird."),
                t("No hatches.", "Keine Schraffuren."),
                t("Concrete 0.58, seat 0.66, algae 0.36.", "Beton 0,58, Sitz 0,66, Algen 0,36."))}


def extrude(i, bone, material, tone, profile, plane, a, b, **kw):
    """🧊️ An extrude solid."""
    s = {"id": i, "kind": "extrude", "bone": bone, "material": material, "role": kw.pop("role", "body"), "tone": tone, "profile": pts(profile), "plane": plane, "from": r(a), "to": r(b)}
    s.update(kw)
    return s


def boxs(i, bone, material, tone, size, at, **kw):
    """📦️ A box solid."""
    s = {"id": i, "kind": "box", "bone": bone, "material": material, "role": kw.pop("role", "body"), "tone": tone, "size": [r(x) for x in size], "transform": {"translation": [r(x) for x in at]}}
    s.update(kw)
    return s


def algae_solids(s):
    """🦠️ The two algae film sheets of scale s: the north face and the east end."""
    return [extrude("algae-north", "block", "algae-biofilm", T_ALG, ALGAE_FACE[s], "xy", -HW - SHEET, -HW, sheet=True),
            extrude("algae-east", "block", "algae-biofilm", T_ALG, [(v, -w) for v, w in ALGAE_END[s]], "yz", L, L + SHEET, sheet=True)]


def algae_thermal():
    """🌡️ The algae thermal references with their faces against the concrete as section faces."""
    return [{"id": "th-algae-north", "of": "algae-north", "region": "algae-film", "sectionFaces": ["+w"]},
            {"id": "th-algae-east", "of": "algae-east", "region": "algae-film", "sectionFaces": ["-u"]}]


def socket_tex(x):
    """🔩️ A lifting socket head 3 mm proud of the concrete top (texture)."""
    return {"id": f"socket-{'west' if x < 1 else 'east'}", "kind": "revolve", "bone": "block", "material": "steel-galvanised", "role": "detail", "tone": T_STEEL, "profile": [[0, 0.02], [0.003, 0.018], [0.003, 0.0]], "axis": [0, 1, 0], "origin": [x, HC, 0], "texture": True}


def joint_capsule(i, u, rad, v0, v1, count=NB):
    """🔩️ A line of screw joints through the battens into the concrete."""
    s = {"id": i, "kind": "capsule", "bone": "battens", "material": "stainless-a4", "role": "detail", "tone": 0.2, "p0": [r(u), r(v0), r(BZC[0])], "p1": [r(u), r(v1), r(BZC[0])], "r0": rad, "r1": rad, "member": "battens", "part": "joint"}
    if count > 1:
        s["repeat"] = {"count": count, "translation": [0, 0, r(PITCH)]}
    return s


def socket_joint(i, x, rad, depth):
    """🪝️ A cast-in lifting socket as the joint the crane load enters by."""
    return {"id": i, "kind": "capsule", "bone": "block", "material": "steel-galvanised", "role": "detail", "tone": 0.2, "p0": [x, r(HC - depth), 0], "p1": [x, r(HC - rad), 0], "r0": rad, "r1": rad, "member": "block", "part": "joint"}


def mass(i, bone, rad, at, member):
    """⚖️ A mass marker."""
    return {"id": i, "kind": "ellipsoid", "bone": bone, "material": "concrete-weathered" if bone == "block" else "robinia-greyed", "role": "detail", "tone": 0.3, "radii": [rad, rad, rad], "transform": {"translation": [r(x) for x in at]}, "member": member, "part": "mass"}


def vol50():
    """🧊️ The 1:50 model."""
    body = [{"id": "block", "kind": "polyhedron", "bone": "block", "material": "concrete-weathered", "role": "body", "tone": T_CONC, "vertices": B50[0], "triangles": B50[1]},
            extrude("batten", "battens", "robinia-greyed", T_BAT, BAT50, "yz", 0, BL, repeat={"count": NB, "translation": [0, 0, r(PITCH)]})] + algae_solids(50) + [socket_tex(x) for x in SOCKET_X]
    thermal = [{"id": "th-block", "of": "block", "region": CONCRETE_REGION}, {"id": "th-batten", "of": "batten", "region": "seat-battens"}] + algae_thermal()
    structure = [{"id": "m-block", "of": "block", "member": "block", "part": "member"}, {"id": "m-batten", "of": "batten", "member": "battens", "part": "member"}] \
        + [joint_capsule(f"j-screws-{k + 1}", u, 0.004, -0.035, TB - 0.004) for k, u in enumerate(FIX_U)] \
        + [socket_joint(f"j-socket-{n}", x, 0.016, 0.1) for n, x in zip(("west", "east"), SOCKET_X)] \
        + [mass("mass-block", "block", 0.03, (1.0, COM_Y, 0), "block"), mass("mass-battens", "battens", 0.02, (BL / 2, TB / 2, 0), "battens")]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": [
        t(f"Body, authored for 7.5 mm: the concrete block as one closed polyhedron of {len(B50[0])} vertices: 15 mm chamfers on all twelve edges (chamfered end faces), and the north top arris ground by skateboards over 1.5 m from the 15 mm chamfer into 24-31 mm rounds with chips of 40 and 42 mm at 0.63 m and 1.39 m (nine-point sections at {len(B50[0]) // 9} stations); five robinia battens 80 x 45 mm with 5 mm eased top edges and 12 mm gaps on 1.6 m, set 26 mm in from the long faces and leaving the 0.2 m ends bare; the algae film as two declared 2 mm sheets on the north face and round the north-east corner onto the east end; the two lifting socket heads as texture.",
          f"Körper, für 7,5 mm gebaut: der Betonblock als ein geschlossenes Polyeder aus {len(B50[0])} Ecken: 15-mm-Fasen an allen zwölf Kanten (gefaste Stirnflächen) und die von Skateboards auf 1,5 m abgeschliffene nördliche Oberkante, von der 15-mm-Fase zu 24-31-mm-Rundungen mit Abplatzungen von 40 und 42 mm bei 0,63 m und 1,39 m (neunpunktige Querschnitte an {len(B50[0]) // 9} Stationen); fünf Robinienleisten 80 x 45 mm mit 5 mm gebrochenen Oberkanten und 12-mm-Fugen auf 1,6 m, 26 mm von den Längsseiten eingerückt, die 0,2 m langen Enden frei; der Algenbelag als zwei deklarierte 2-mm-Häute auf der Nordseite und um die Nordostecke auf die Ostseite; die zwei Hülsenköpfe als Textur."),
        t("Thermal: the concrete, the battens and the algae film are each the surface of their body solids (the faces of the film against the concrete are section faces); no hidden regions, no emitters. Structure: the block and the battens are members along their bones; the three lines of stainless screws (8 mm) are the joints of battens and block, the two cast-in sockets (32 mm) the joints the crane load enters by; supports are the pose contacts on the bed or at the sockets; each bone carries its mass at its centre.",
          "Thermik: Beton, Leisten und Algenbelag sind jeweils die Oberfläche ihrer Körper (die Flächen des Belags am Beton sind Schnittflächen); keine verdeckten Regionen, keine Emitter. Tragwerk: Block und Leisten sind Glieder entlang ihrer Knochen; die drei Reihen Edelstahlschrauben (8 mm) sind die Gelenke zwischen Leisten und Block, die zwei einbetonierten Hülsen (32 mm) die Gelenke, über die die Kranlast eingeleitet wird; Auflager sind die Posenkontakte auf dem Bett oder an den Hülsen; jeder Knochen trägt seine Masse in seinem Schwerpunkt.")]}


def vol100():
    """🧊️ The 1:100 model."""
    body = [{"id": "block", "kind": "polyhedron", "bone": "block", "material": "concrete-weathered", "role": "body", "tone": T_CONC, "vertices": B100[0], "triangles": B100[1]},
            extrude("batten", "battens", "robinia-greyed", T_BAT, BAT100, "yz", 0, BL, repeat={"count": NB, "translation": [0, 0, r(PITCH)]})] + algae_solids(100) + [socket_tex(x) for x in SOCKET_X]
    thermal = [{"id": "th-block", "of": "block", "region": CONCRETE_REGION}, {"id": "th-batten", "of": "batten", "region": "seat-battens"}] + algae_thermal()
    structure = [{"id": "m-block", "of": "block", "member": "block", "part": "member"}, {"id": "m-batten", "of": "batten", "member": "battens", "part": "member"}] \
        + [joint_capsule(f"j-screws-{k + 1}", u, 0.0075, -0.035, TB - 0.0075) for k, u in enumerate(FIX_U)] \
        + [socket_joint(f"j-socket-{n}", x, 0.016, 0.1) for n, x in zip(("west", "east"), SOCKET_X)] \
        + [mass("mass-block", "block", 0.03, (1.0, COM_Y, 0), "block"), mass("mass-battens", "battens", 0.02, (BL / 2, TB / 2, 0), "battens")]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": [
        t(f"Body, authored anew for 15 mm: the block polyhedron keeps its 15 mm chamfers (one feature) but the arris wear is generalised to one 28 mm round over 1.3 m with short ramps ({len(B100[0]) // 9} stations instead of {len(B50[0]) // 9}; the two chips merged into it); the five battens square-edged (the 5 mm easing is below the feature) with their 12 mm gaps, which stay open to air (more than half a feature); algae sheets with generalised outlines; socket heads as texture because the plan still draws them.",
          f"Körper, neu für 15 mm gebaut: das Blockpolyeder behält seine 15-mm-Fasen (ein Merkmal), der Kantenverschleiß ist zu einer 28-mm-Rundung über 1,3 m mit kurzen Übergängen verallgemeinert ({len(B100[0]) // 9} statt {len(B50[0]) // 9} Stationen; die zwei Abplatzungen gehen darin auf); die fünf Leisten rechtwinklig (die 5-mm-Kantenbrechung liegt unter dem Merkmal) mit ihren 12-mm-Fugen, die für Luft offen bleiben (mehr als ein halbes Merkmal); Algenhäute mit verallgemeinerten Umrissen; Hülsenköpfe als Textur, weil die Draufsicht sie noch zeichnet."),
        t("Thermal: one surface per region as at 1:50. Structure: block and battens members; the screw lines as 15 mm joints, the sockets as 32 mm joints; masses at the centres.",
          "Thermik: eine Fläche je Region wie bei 1:50. Tragwerk: Block und Leisten als Glieder; die Schraubenreihen als 15-mm-Gelenke, die Hülsen als 32-mm-Gelenke; Massen in den Schwerpunkten.")]}


def vol200():
    """🧊️ The 1:200 model."""
    body = [boxs("block", "block", "concrete-weathered", T_CONC, (L, HC, D), (L / 2, HC / 2, 0)),
            extrude("deck", "battens", "robinia-greyed", T_BAT, [(0, BZ0), (0, -BZ0), (TB, -BZ0), (TB, BZ0)], "yz", 0, BL)] + algae_solids(200)
    thermal = [{"id": "th-block", "of": "block", "region": CONCRETE_REGION}, {"id": "th-deck", "of": "deck", "region": "seat-battens"}] + algae_thermal()
    structure = [{"id": "m-block", "of": "block", "member": "block", "part": "member"}, {"id": "m-deck", "of": "deck", "member": "battens", "part": "member"}] \
        + [boxs(f"j-screws-{k + 1}", "battens", "stainless-a4", 0.2, (0.03, 0.07, DECK - 0.04), (u, 0.01, 0), role="detail", member="battens", part="joint") for k, u in enumerate(FIX_U)] \
        + [socket_joint(f"j-socket-{n}", x, 0.016, 0.1) for n, x in zip(("west", "east"), SOCKET_X)] \
        + [mass("mass-block", "block", 0.035, (1.0, COM_Y, 0), "block"), mass("mass-battens", "battens", 0.02, (BL / 2, TB / 2, 0), "battens")]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": [
        t("Body, authored anew for 30 mm: the block as a plain box 2.0 x 0.405 x 0.5 m (the 15 mm chamfers and the 28 mm arris wear are below the feature), the five battens as one 45 mm seat deck 1.6 x 0.448 m (the 12 mm gaps are below half a feature and are drawn as joint lines in 2D), the algae sheets with generalised outlines; sockets left out.",
          "Körper, neu für 30 mm gebaut: der Block als einfacher Quader 2,0 x 0,405 x 0,5 m (die 15-mm-Fasen und der 28-mm-Kantenverschleiß liegen unter dem Merkmal), die fünf Leisten als ein 45-mm-Sitzdeck 1,6 x 0,448 m (die 12-mm-Fugen liegen unter einem halben Merkmal und sind in 2D als Fugenlinien gezeichnet), die Algenhäute mit verallgemeinerten Umrissen; Hülsen weggelassen."),
        t("Thermal: one surface per region. Structure: block and deck members; each screw line as one 30 mm joint bar across the deck, the sockets as 32 mm joints; masses at the centres.",
          "Thermik: eine Fläche je Region. Tragwerk: Block und Deck als Glieder; jede Schraubenreihe als ein 30-mm-Gelenkriegel quer zum Deck, die Hülsen als 32-mm-Gelenke; Massen in den Schwerpunkten.")]}


def vol500():
    """🧊️ The 1:500 model."""
    body = [boxs("block", "block", "concrete-weathered", T_CONC, (L, TOP500, D), (L / 2, TOP500 / 2, 0)),
            extrude("seat", "battens", "robinia-greyed", T_BAT, [(TOP500 - HC, -HW), (TOP500 - HC, HW), (TB, HW), (TB, -HW)], "yz", 0, BL)] + algae_solids(500)
    thermal = [{"id": "th-block", "of": "block", "region": CONCRETE_REGION}, {"id": "th-seat", "of": "seat", "region": "seat-battens"}] + algae_thermal()
    structure = [{"id": "m-block", "of": "block", "member": "block", "part": "member"}, {"id": "m-seat", "of": "seat", "member": "battens", "part": "member"},
                 boxs("j-screws", "battens", "stainless-a4", 0.2, (0.075, 0.075, 0.3), (BL / 2, 0.0075, 0), role="detail", member="battens", part="joint")] \
        + [socket_joint(f"j-socket-{n}", x, 0.0375, 0.15) for n, x in zip(("west", "east"), SOCKET_X)] \
        + [mass("mass-block", "block", 0.04, (1.0, COM_Y, 0), "block")]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": [
        t("Body, authored anew for 75 mm: the concrete as a box 2.0 x 0.375 x 0.5 m and the seat as one 75 mm course 1.6 x 0.5 m over it (battens, gaps and the 26 mm side bands merged; the 45 mm seat would be below the feature, so the seat step over the bare ends is generalised from 45 to 75 mm while the seat height 0.45 m stays true); the algae sheets as two rectangles; nothing else remains above 75 mm.",
          "Körper, neu für 75 mm gebaut: der Beton als Quader 2,0 x 0,375 x 0,5 m und der Sitz als eine 75-mm-Lage 1,6 x 0,5 m darüber (Leisten, Fugen und die 26-mm-Randstreifen zusammengefasst; der 45-mm-Sitz läge unter dem Merkmal, daher ist die Sitzstufe über den freien Enden von 45 auf 75 mm verallgemeinert, während die Sitzhöhe 0,45 m stimmt); die Algenhäute als zwei Rechtecke; sonst bleibt nichts über 75 mm."),
        t("Thermal: one surface per region (only the total area is compared at this scale). Structure: block and seat members; the screw lines as one 75 mm joint, the sockets as 75 mm joints; the block mass marker (the 22 kg seat mass is below what this scale shows apart and is carried by its member).",
          "Thermik: eine Fläche je Region (bei diesem Maßstab wird nur die Gesamtfläche verglichen). Tragwerk: Block und Sitz als Glieder; die Schraubenreihen als ein 75-mm-Gelenk, die Hülsen als 75-mm-Gelenke; der Massenmarker des Blocks (die 22 kg des Sitzes liegen unter dem, was dieser Maßstab getrennt zeigt, und werden von seinem Glied getragen).")]}


ASSET = {
    "id": "used-concrete-bench-block",
    "category": "furniture",
    "title": t("Used Concrete Seating Block", "Gebrauchter Betonsitzblock"),
    "summary": SUMMARY,
    "tags": TAGS,
    "provenance": PROVENANCE,
    "dimensions": {"heightM": H, "widthM": L, "depthM": D, "massKg": r(M_TOTAL, 1)},
    "materials": MATERIALS,
    "rig": RIG,
    "anchors": ANCHORS,
    "representations": {"50": rep50(), "100": rep100(), "200": rep200(), "500": rep500()},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {"50": vol50(), "100": vol100(), "200": vol200(), "500": vol500()},
    "presentation": {"defaultPose": "in-service", "defaultClip": "sitting-in-sun", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

if __name__ == "__main__":
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(f"[DEBUG] block 1:50 volume {VOL_BLOCK:.5f} m3 area {B50[2]:.4f} m2 comY {COM_Y:.4f}; 1:100 area {B100[2]:.4f}")
    print(f"[DEBUG] masses block {M_BLOCK:.2f} battens {M_BATTENS:.2f} total {M_TOTAL:.2f} kg")
    for k, v in AREA.items():
        print(f"[DEBUG] area {k}: " + ", ".join(f"1:{s} {a:.4f}" for s, a in v.items()))
