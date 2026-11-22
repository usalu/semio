#!/usr/bin/env python3
"""🧱️ Authors the staffage asset `reclaimed-concrete-wall-panel`: a sawn-out WBS 70 three-layer outer wall panel 6.0 x 2.8 m in an A-frame rack.

Every number of the document is chosen here by hand and written once to `🔣️.json` next to this file; run `python3 author-panel.py`.

@see ../../brief.md — the asset contract of the collection
@see ../../reports/reclaimed-concrete-wall-panel.md — reference table and measurements
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

L, H = 6.0, 2.8
Z_OUT, Z_SI, Z_IL, Z_IN = -0.13, -0.07, -0.01, 0.13
OX0, OX1, OY0, OY1 = 1.8, 4.2, 0.8, 2.4
RIB = 0.06
HB = 0.20
LEAN = 3.5
RHO_C, RHO_EPS, RHO_ST = 2300.0, 20.0, 7850.0
LOOPS = (1.5, 4.5)
LOOP_R_OUT, LOOP_BAR, LOOP_LEG = 0.07, 0.02, 0.05
STUB_Y = (0.5, 1.4, 2.3)
STUB_LEN, STUB_D, STUB_Z = 0.06, 0.016, 0.06
TRESTLES = (0.75, 5.25)
LEG, LEG_TOP, RUN_H, RUN_HALF = 0.08, 2.40, 0.10, 0.85
RIDGE_H = 0.08
BEARER_Z0, BEARER_Z1 = -0.20, 0.12
TRAVEL = 7.0
LIFT = 1.5
QP, CPE_D, CPE_A = 650.0, 0.8, -1.2
TAN = math.tan(math.radians(LEAN))
COS = math.cos(math.radians(LEAN))
D_LEG = LEG / COS


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r(x, n=5):
    """🎯️ A number rounded for the document."""
    return round(x + 0.0, n)


def area(points):
    """📐️ Signed area of a polygon (shoelace)."""
    return 0.5 * sum(points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1] for i in range(len(points)))


def centroid(points):
    """⚖️ Centroid of a simple polygon."""
    a = area(points)
    cx = sum((points[i][0] + points[(i + 1) % len(points)][0]) * (points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1]) for i in range(len(points))) / (6 * a)
    cy = sum((points[i][1] + points[(i + 1) % len(points)][1]) * (points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1]) for i in range(len(points))) / (6 * a)
    return cx, cy


def rect_pts(x0, y0, x1, y1):
    """⬛️ Counter-clockwise corners of an axis-aligned rectangle."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


SHELL_FINE = [(0.14, 0.0), (L, 0.0), (L, H), (5.2, H), (5.15, 2.758), (4.95, 2.755), (4.85, H), (0.0, H), (0.0, 0.11), (0.03, 0.06), (0.08, 0.025)]
SHELL_MID = [(0.12, 0.0), (L, 0.0), (L, H), (0.0, H), (0.0, 0.10)]
PANEL = rect_pts(0.0, 0.0, L, H)
OPENING = rect_pts(OX0, OY0, OX1, OY1)
GROWN = rect_pts(OX0 - RIB, OY0 - RIB, OX1 + RIB, OY1 + RIB)


def loop_outline(xc, bar=LOOP_BAR, steps=8):
    """➰️ The U of a lifting loop in panel coordinates: legs from the top edge, a semicircular crown, the bar width `bar`."""
    ro, ri = LOOP_R_OUT, LOOP_R_OUT - bar
    base = H + LOOP_LEG
    outer = [(xc - ro, H), (xc - ro, base)] + [(xc - ro * math.cos(math.pi * k / steps), base + ro * math.sin(math.pi * k / steps)) for k in range(1, steps)] + [(xc + ro, base), (xc + ro, H)]
    inner = [(xc + ri, H), (xc + ri, base)] + [(xc + ri * math.cos(math.pi * k / steps), base + ri * math.sin(math.pi * k / steps)) for k in range(1, steps)] + [(xc - ri, base), (xc - ri, H)]
    return outer + inner


LOOP_TOP = H + LOOP_LEG + LOOP_R_OUT
LOOP_RC = LOOP_R_OUT - LOOP_BAR / 2
LOOP_DEV = math.pi * LOOP_RC + 2 * LOOP_LEG
LOOP_VOL = math.pi * (LOOP_BAR / 2) ** 2 * LOOP_DEV
STUB_VOL = math.pi * (STUB_D / 2) ** 2 * STUB_LEN

parts = [
    ("weather-shell", RHO_C, abs(area(SHELL_FINE)) - abs(area(OPENING)), Z_SI - Z_OUT, (Z_OUT + Z_SI) / 2, SHELL_FINE, [OPENING]),
    ("insulation", RHO_EPS, abs(area(PANEL)) - abs(area(GROWN)), Z_IL - Z_SI, (Z_SI + Z_IL) / 2, PANEL, [GROWN]),
    ("reveal-rib", RHO_C, abs(area(GROWN)) - abs(area(OPENING)), Z_IL - Z_SI, (Z_SI + Z_IL) / 2, GROWN, [OPENING]),
    ("load-bearing-leaf", RHO_C, abs(area(PANEL)) - abs(area(OPENING)), Z_IN - Z_IL, (Z_IL + Z_IN) / 2, PANEL, [OPENING]),
]
mass = {}
moment = [0.0, 0.0, 0.0]
total = 0.0
for name, rho, a, t, zc_, outer, holes in parts:
    m = rho * a * t
    ocx, ocy = centroid(outer)
    oa = abs(area(outer))
    cx, cy = ocx * oa, ocy * oa
    for hole in holes:
        hcx, hcy = centroid(hole)
        ha = abs(area(hole))
        cx -= hcx * ha
        cy -= hcy * ha
    cx /= a
    cy /= a
    mass[name] = m
    moment[0] += m * cx
    moment[1] += m * cy
    moment[2] += m * zc_
    total += m
for xc in LOOPS:
    m = RHO_ST * LOOP_VOL
    mass[f"lifting-loop-{xc}"] = m
    moment[0] += m * xc
    moment[1] += m * (H + 0.07)
    moment[2] += m * STUB_Z
    total += m
for side in (-STUB_LEN / 2, L + STUB_LEN / 2):
    for y in STUB_Y:
        m = RHO_ST * STUB_VOL
        moment[0] += m * side
        moment[1] += m * y
        moment[2] += m * STUB_Z
        total += m
X_COM, Y_COM, Z_COM = moment[0] / total, moment[1] / total, moment[2] / total
MASS = total
ZC = Z_COM
PY0 = H / 2


def u(px):
    """📏️ Panel coordinate along the panel to bone-local u."""
    return r(px)


def v(py):
    """📏️ Panel height from the bottom edge to bone-local v."""
    return r(py - PY0)


def w(pz):
    """📏️ Rest world z of the panel to bone-local w."""
    return r(pz - ZC)


def uv(points):
    """🔁️ Panel points to bone-local (u, v)."""
    return [[u(p[0]), v(p[1])] for p in points]


A_L, A_C = LOOP_TOP - LOOP_BAR / 2, Y_COM
B_L, B_C = STUB_Z - Z_IN, ZC - Z_IN
HANG = math.degrees(math.atan2(B_L - B_C, A_L - A_C))


def z_face(y):
    """📐️ World z of the rack leg face against the leaning panel at height y."""
    return Z_IN + (y - HB) * TAN


ZR = z_face(LEG_TOP) + D_LEG
LEG_NEAR = [(0.10, z_face(0.10)), (LEG_TOP, z_face(LEG_TOP)), (LEG_TOP, ZR), (0.10, z_face(0.10) + D_LEG)]
LEG_FAR = [(y, 2 * ZR - z) for (y, z) in reversed(LEG_NEAR)]
RUN_Z0, RUN_Z1 = ZR - RUN_HALF, ZR + RUN_HALF
TIE_NEAR, TIE_FAR = ZR - 0.80, ZR + 0.80

SEC_AREA = (OY0 + (H - OY1)) * (Z_IN - Z_IL)
_yc = (OY0 * OY0 / 2 + (H - OY1) * (OY1 + H) / 2) / (OY0 + (H - OY1))
SEC_I = (Z_IN - Z_IL) * (OY0 ** 3 / 12 + (H - OY1) ** 3 / 12 + OY0 * (OY0 / 2 - _yc) ** 2 + (H - OY1) * ((OY1 + H) / 2 - _yc) ** 2)
SEC_E = max(_yc, H - _yc)

AREA_FACE = abs(area(SHELL_FINE)) - abs(area(OPENING))
AREA_INNER = abs(area(PANEL)) - abs(area(OPENING))
AREA_EDGES = 2 * (L + H) * (Z_IN - Z_OUT)
AREA_REVEALS = 2 * ((OX1 - OX0) + (OY1 - OY0)) * (Z_IN - Z_OUT)
AREA_STEEL = 2 * math.pi * LOOP_BAR * LOOP_DEV + 6 * (math.pi * STUB_D * STUB_LEN + math.pi * STUB_D ** 2 / 4)
RACK_AREA = 8.5

MATERIALS = [
    {"id": "load-bearing-concrete", "title": T("Reinforced concrete of the load-bearing leaf (grade B 300, about C20/25)", "Stahlbeton der Tragschicht (Güte B 300, etwa C20/25)"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": RHO_C, "conductivityWmK": 2.3, "specificHeatJKgK": 1000, "elasticModulusPa": 30e9, "strengthPa": 20e6, "source": "authored-estimate", "reference": "en-1992-1-1"},
    {"id": "exposed-aggregate-concrete", "title": T("Exposed-aggregate concrete of the weather shell, washed gravel face, weathered", "Waschbeton der Wetterschale, ausgewaschene Kiesoberfläche, verwittert"), "emissivity": 0.93, "solarAbsorptivity": 0.72, "densityKgM3": RHO_C, "conductivityWmK": 2.3, "specificHeatJKgK": 1000, "elasticModulusPa": 30e9, "source": "authored-estimate", "reference": "ashrae-fundamentals"},
    {"id": "eps-insulation", "title": T("Expanded polystyrene of the period, aged", "Polystyrol-Hartschaum der Bauzeit, gealtert"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": RHO_EPS, "conductivityWmK": 0.04, "specificHeatJKgK": 1450, "source": "authored-estimate", "reference": "en-iso-10456"},
    {"id": "sawn-concrete", "title": T("Diamond-sawn concrete face, light grey and dusty", "Diamantgesägte Betonfläche, hellgrau und staubig"), "emissivity": 0.93, "solarAbsorptivity": 0.55, "densityKgM3": RHO_C, "conductivityWmK": 2.3, "specificHeatJKgK": 1000, "source": "authored-estimate", "reference": "ashrae-fundamentals"},
    {"id": "reveal-concrete", "title": T("Cast window reveal with sealant and dowel remnants", "Geschalte Fensterleibung mit Dichtstoff- und Dübelresten"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": RHO_C, "conductivityWmK": 2.3, "specificHeatJKgK": 1000, "source": "authored-estimate", "reference": "ashrae-fundamentals"},
    {"id": "interior-finish", "title": T("Filler, paint and wallpaper remnants of the former room face", "Spachtel-, Farb- und Tapetenreste der früheren Raumseite"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 1100, "conductivityWmK": 0.5, "specificHeatJKgK": 1100, "source": "authored-estimate", "reference": "ashrae-fundamentals"},
    {"id": "rusted-steel", "title": T("Reinforcing steel with rust film (lifting loops, cut bars)", "Betonstahl mit Rostfilm (Transportschlaufen, abgetrennte Stäbe)"), "emissivity": 0.85, "solarAbsorptivity": 0.75, "densityKgM3": RHO_ST, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 220e6, "source": "authored-estimate", "reference": "vdi-bv-bs-6205"},
    {"id": "rack-steel", "title": T("Painted structural steel S235 of the A-frame rack", "Lackierter Baustahl S235 des A-Bocks"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_ST, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-10025-2"},
    {"id": "bearer-softwood", "title": T("Weathered softwood squared timber", "Verwittertes Nadelholz-Kantholz"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-iso-10456"},
]

REFERENCES = [
    {"id": "hannemann-die-platte", "citation": "Hannemann, C.: Die Platte. Industrialisierter Wohnungsbau in der DDR. 3rd ed., Berlin: Verlag Hans Schiler, 2005 (WBS 70: 6.0 m grid, 2.8 m storey height, 6.3 t crane class, three-layer outer wall panels with exposed-aggregate weather shell)."},
    {"id": "en-1992-1-1", "citation": "EN 1992-1-1:2004 Eurocode 2, design of concrete structures, Table 3.1 (C20/25: fck = 20 MPa, Ecm = 30 GPa); the TGL grade B 300 of the period is taken as about C20/25 (authored estimate)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties, Table 3 (reinforced concrete with 1 % steel: 2300 kg/m3, 2.3 W/mK, 1000 J/kgK; expanded polystyrene 0.035 to 0.040 W/mK, 1450 J/kgK; softwood 450 kg/m3, 0.13 W/mK, 1600 J/kgK)."},
    {"id": "en-iso-6946", "citation": "EN ISO 6946:2017 Building components and building elements, thermal resistance and thermal transmittance (R_si 0.13 and R_se 0.04 m2K/W for horizontal heat flow)."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); about half of it is taken as the effective storage depth of an exposed face."},
    {"id": "ashrae-fundamentals", "citation": "ASHRAE Handbook Fundamentals (2021), Chapter 33 physical properties of materials: emissivity and solar absorptance of building surfaces (concrete 0.85 to 0.95; weathered and soiled surfaces absorb more than new ones; typical rounded values, not table look-ups)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, Part 1-1, Table A.1: normal-weight concrete 24 kN/m3, plus 1 kN/m3 for the normal percentage of reinforcement."},
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Part 1-4 wind actions, Table 7.1 external pressure coefficients of vertical walls (cpe,10 zone D +0.8, zone A -1.2)."},
    {"id": "din-en-1991-1-4-na", "citation": "DIN EN 1991-1-4/NA:2010-12 German National Annex, Table NA.B.3: wind zone 2, inland, building height up to 10 m, peak velocity pressure qp = 0.65 kN/m2."},
    {"id": "vdi-bv-bs-6205", "citation": "VDI/BV-BS 6205:2012 Lifting inserts and lifting insert systems for precast concrete elements (cast-in steel loops, lifting angles, dynamic factor)."},
    {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy structural steels (S235 yield strength 235 MPa, E = 210 GPa)."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on sections and cuts (hatching)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."},
]

BONES = [
    {"id": "rack", "parent": None, "title": T("A-frame rack of the component exchange, steel S235 (yard equipment, stays on the ground; about 0.25 t, not counted in the component mass)", "A-Bock der Bauteilbörse, Stahl S235 (Hofausrüstung, bleibt am Boden; etwa 0,25 t, nicht in der Bauteilmasse gezählt)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": L, "joint": {"type": "fixed"}},
    {"id": "bearer-a", "parent": "rack", "title": T("Squared-timber bearer under the left end (stays on the rack)", "Kantholzauflager unter dem linken Ende (bleibt auf dem Bock)"), "x": r(TRESTLES[0] - 0.05), "y": RUN_H, "angleDeg": 0, "lengthM": 0.1, "joint": {"type": "fixed"}},
    {"id": "bearer-b", "parent": "rack", "title": T("Squared-timber bearer under the right end (stays on the rack)", "Kantholzauflager unter dem rechten Ende (bleibt auf dem Bock)"), "x": r(TRESTLES[1] - 0.05), "y": RUN_H, "angleDeg": 0, "lengthM": 0.1, "joint": {"type": "fixed"}},
    {"id": "travel", "parent": "rack", "title": T("Crane travel carrier (massless, moves the panel along the yard)", "Kranfahrt-Träger (masselos, verfährt die Platte über den Hof)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": TRAVEL}},
    {"id": "hoist", "parent": "travel", "title": T("Sling carrier of the crane hook (massless, lifts and lowers the panel)", "Anschlagträger des Kranhakens (masselos, hebt und senkt die Platte)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -HB, "maxM": 2.0}},
    {"id": "pivot", "parent": "hoist", "title": T("Tilt axis along the inner bottom edge of the panel (lean against the rack, out of plumb when hanging)", "Kippachse entlang der inneren Unterkante der Platte (Anlehnen an den Bock, Schiefhang am Haken)"), "x": 0, "y": HB, "z": Z_IN, "angleDeg": 0, "lengthM": 0, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -LEAN, "twistMaxDeg": 3.0}},
    {"id": "panel", "parent": "pivot", "title": T("WBS 70 outer wall panel, three layers 140/60/60 mm (bone in the plane of its centre of mass)", "WBS-70-Außenwandplatte, dreischichtig 140/60/60 mm (Knochen in der Ebene ihres Schwerpunkts)"), "x": 0, "y": PY0, "z": r(ZC - Z_IN), "angleDeg": 0, "lengthM": L, "joint": {"type": "fixed"}, "mass": {"kg": r(MASS, 2), "comX": r(X_COM, 4), "comY": r(Y_COM - PY0, 4), "distribution": "uniform", "source": "authored-estimate", "reference": "en-iso-10456"}, "section": {"areaM2": r(SEC_AREA, 5), "secondMomentM4": r(SEC_I, 5), "extremeFibreM": r(SEC_E, 4), "material": "load-bearing-concrete", "title": T("Load-bearing leaf 140 mm through the window: parapet 0.80 m and lintel 0.40 m", "Tragschicht 140 mm im Fensterschnitt: Brüstung 0,80 m und Sturz 0,40 m")}},
]

ANCHORS = [
    {"id": "bearer-contact-a", "bone": "panel", "x": TRESTLES[0], "y": v(0.0), "z": w(-0.005), "kind": "contact", "title": T("Bottom edge on the left bearer", "Unterkante auf dem linken Kantholz"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.1, "footprintWidthM": 0.25, "frictionCoefficient": 0.6},
    {"id": "bearer-contact-b", "bone": "panel", "x": TRESTLES[1], "y": v(0.0), "z": w(-0.005), "kind": "contact", "title": T("Bottom edge on the right bearer", "Unterkante auf dem rechten Kantholz"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.1, "footprintWidthM": 0.25, "frictionCoefficient": 0.6},
    {"id": "rack-lean-a", "bone": "panel", "x": TRESTLES[0], "y": v(LEG_TOP - HB - 0.1), "z": w(Z_IN), "kind": "contact", "title": T("Inner face leaning on the left rack leg", "Innenseite am linken Bockschenkel angelehnt"), "support": "contact", "normalDeg": 90, "tiltDeg": r(-(90 - LEAN), 3), "footprintLengthM": LEG, "footprintWidthM": 0.2, "frictionCoefficient": 0.4},
    {"id": "rack-lean-b", "bone": "panel", "x": TRESTLES[1], "y": v(LEG_TOP - HB - 0.1), "z": w(Z_IN), "kind": "contact", "title": T("Inner face leaning on the right rack leg", "Innenseite am rechten Bockschenkel angelehnt"), "support": "contact", "normalDeg": 90, "tiltDeg": r(-(90 - LEAN), 3), "footprintLengthM": LEG, "footprintWidthM": 0.2, "frictionCoefficient": 0.4},
    {"id": "lifting-loop-a", "bone": "panel", "x": LOOPS[0], "y": v(A_L), "z": w(STUB_Z), "kind": "support", "title": T("Left lifting loop on the hook of the two-leg sling", "Linke Transportschlaufe am Haken des Zweistranggehänges"), "support": "pin"},
    {"id": "lifting-loop-b", "bone": "panel", "x": LOOPS[1], "y": v(A_L), "z": w(STUB_Z), "kind": "support", "title": T("Right lifting loop on the hook of the two-leg sling", "Rechte Transportschlaufe am Haken des Zweistranggehänges"), "support": "pin"},
    {"id": "base-bed-a", "bone": "panel", "x": 0.75, "y": v(0.0), "z": 0, "kind": "contact", "title": T("Mortar bed under the left half of the reinstalled panel", "Mörtelbett unter der linken Hälfte der wieder eingebauten Platte"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.5, "footprintWidthM": 0.26, "frictionCoefficient": 0.6},
    {"id": "base-bed-b", "bone": "panel", "x": 5.25, "y": v(0.0), "z": 0, "kind": "contact", "title": T("Mortar bed under the right half of the reinstalled panel", "Mörtelbett unter der rechten Hälfte der wieder eingebauten Platte"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.5, "footprintWidthM": 0.26, "frictionCoefficient": 0.6},
    {"id": "head-tie-a", "bone": "panel", "x": 0.45, "y": v(H - 0.15), "z": w(Z_IN), "kind": "support", "title": T("Left steel angle tie to the new floor slab (holds the head across the wall)", "Linker Stahlwinkel zur neuen Decke (hält den Kopf quer zur Wand)"), "support": "roller", "normalDeg": 90, "tiltDeg": 90},
    {"id": "head-tie-b", "bone": "panel", "x": 5.55, "y": v(H - 0.15), "z": w(Z_IN), "kind": "support", "title": T("Right steel angle tie to the new floor slab (holds the head across the wall)", "Rechter Stahlwinkel zur neuen Decke (hält den Kopf quer zur Wand)"), "support": "roller", "normalDeg": 90, "tiltDeg": 90},
    {"id": "wind-resultant-a", "bone": "panel", "x": 1.5, "y": v(H / 2), "z": w(Z_OUT), "kind": "load", "title": T("Wind resultant on the left half of the facade face", "Windresultierende auf der linken Hälfte der Fassadenfläche")},
    {"id": "wind-resultant-b", "bone": "panel", "x": 4.5, "y": v(H / 2), "z": w(Z_OUT), "kind": "load", "title": T("Wind resultant on the right half of the facade face", "Windresultierende auf der rechten Hälfte der Fassadenfläche")},
]

REGION_IDS = ["weather-face", "inner-face", "sawn-edges", "window-reveals", "exposed-steel", "rack-frame", "bearer-timber"]


def binding(**over):
    """🌡️ The thermal binding of a pose: every region, defaults of the yard, overrides by region id."""
    base = {
        "weather-face": {"exposedFraction": 1, "sunlitFraction": 0.6},
        "inner-face": {"exposedFraction": 1, "sunlitFraction": 0.1},
        "sawn-edges": {"exposedFraction": 0.95, "sunlitFraction": 0.3},
        "window-reveals": {"exposedFraction": 1, "sunlitFraction": 0.25},
        "exposed-steel": {"exposedFraction": 1, "sunlitFraction": 0.5},
        "rack-frame": {"exposedFraction": 1, "sunlitFraction": 0.4},
        "bearer-timber": {"exposedFraction": 0.6, "sunlitFraction": 0.1},
    }
    for key, value in over.items():
        base[key.replace("_", "-")] = value
    return {"regions": base, "emitters": {}}


REINSTALLED_THERMAL = dict(
    weather_face={"exposedFraction": 1, "sunlitFraction": 0.7},
    inner_face={"exposedFraction": 0, "sunlitFraction": 0, "contact": {"fraction": 1, "tempC": 20}},
    sawn_edges={"exposedFraction": 0, "sunlitFraction": 0, "contact": {"fraction": 1, "tempC": 12}},
    window_reveals={"exposedFraction": 0.5, "sunlitFraction": 0.2},
    exposed_steel={"exposedFraction": 0, "sunlitFraction": 0, "contact": {"fraction": 1, "tempC": 12}},
)

WIND_P = QP * CPE_D * L * H / 2
WIND_S = QP * CPE_A * L * H / 2
REINSTALLED_BONES = {"travel": {"dx": TRAVEL}, "hoist": {"dy": -HB}}
STORED_BONES = {"pivot": {"twistDeg": -LEAN}}
LIFTED_BONES = {"hoist": {"dy": LIFT}, "pivot": {"twistDeg": r(HANG, 3)}}
REINSTALLED_CONTACTS = ["base-bed-a", "base-bed-b", "head-tie-a", "head-tie-b"]
POSES = [
    {"id": "stored", "title": T("Stored in the A-frame rack", "Im A-Bock gelagert"), "description": T(f"The sawn-out panel stands on two squared-timber bearers on the base runners of the A-frame rack and leans {LEAN} degrees with its inner face against the rack legs, exposed-aggregate face outwards, as the component exchange keeps it.", f"Die herausgesägte Platte steht auf zwei Kanthölzern auf den Fußholmen des A-Bocks und lehnt sich um {LEAN} Grad mit ihrer Innenseite an die Bockschenkel, die Waschbetonseite nach außen, so wie die Bauteilbörse sie lagert."), "bones": STORED_BONES, "ground": "authored", "contacts": ["bearer-contact-a", "bearer-contact-b", "rack-lean-a", "rack-lean-b"], "thermal": binding(), "loads": []},
    {"id": "lifted", "title": T("Lifted on the crane", "Am Kran angehoben"), "description": T(f"The two-leg sling of the crane hangs the panel from its two cast-in lifting loops {LIFT} m above the bearers; because the loops sit in the inner leaf, the panel hangs {HANG:.1f} degrees out of plumb until its centre of mass is below them. Rack and bearers stay on the ground.", f"Das Zweistranggehänge des Krans hängt die Platte an ihren beiden einbetonierten Transportschlaufen {LIFT} m über die Kanthölzer; weil die Schlaufen in der Tragschicht sitzen, hängt die Platte {HANG:.1f} Grad schief, bis ihr Schwerpunkt unter ihnen liegt. Bock und Kanthölzer bleiben am Boden."), "bones": LIFTED_BONES, "ground": "authored", "contacts": ["lifting-loop-a", "lifting-loop-b"], "thermal": binding(sawn_edges={"exposedFraction": 1, "sunlitFraction": 0.3}), "loads": []},
    {"id": "reinstalled", "title": T("Reinstalled as a facade panel", "Als Fassadenplatte wieder eingebaut"), "description": T(f"The panel is set {TRAVEL} m along the yard plumb on a mortar bed on its new floor slab and tied at the head with two steel angles; its self-weight goes down through the bed. The room behind its inner face is at 20 degC.", f"Die Platte ist {TRAVEL} m weiter lotrecht auf ein Mörtelbett ihrer neuen Decke versetzt und am Kopf mit zwei Stahlwinkeln angeschlossen; ihr Eigengewicht geht über das Bett nach unten. Der Raum hinter ihrer Innenseite hat 20 °C."), "bones": REINSTALLED_BONES, "ground": "authored", "contacts": REINSTALLED_CONTACTS, "thermal": binding(**REINSTALLED_THERMAL), "loads": []},
    {"id": "reinstalled-wind-pressure", "title": T("Reinstalled under wind pressure", "Wieder eingebaut unter Winddruck"), "description": T(f"Wind pressure {QP * CPE_D / 1000:.2f} kN/m2 (qp {QP / 1000:.2f} kN/m2, cpe +0.8) on the whole 6.0 x 2.8 m face, the opening included because the new window passes its load to the reveals; mortar bed and head ties carry it.", f"Winddruck {QP * CPE_D / 1000:.2f} kN/m2 (qp {QP / 1000:.2f} kN/m2, cpe +0,8) auf die ganze Fläche 6,0 x 2,8 m, die Öffnung eingeschlossen, weil das neue Fenster seine Last an die Leibungen abgibt; Mörtelbett und Kopfwinkel tragen ihn."), "bones": REINSTALLED_BONES, "ground": "authored", "contacts": REINSTALLED_CONTACTS, "thermal": binding(**REINSTALLED_THERMAL), "loads": [
        {"id": "wind-pressure-a", "kind": "wind", "title": T(f"Wind pressure on the left half, {WIND_P / 1000:.2f} kN", f"Winddruck auf die linke Hälfte, {WIND_P / 1000:.2f} kN"), "anchor": "wind-resultant-a", "fxN": 0, "fyN": 0, "fzN": r(WIND_P, 1), "source": "literature", "reference": "en-1991-1-4"},
        {"id": "wind-pressure-b", "kind": "wind", "title": T(f"Wind pressure on the right half, {WIND_P / 1000:.2f} kN", f"Winddruck auf die rechte Hälfte, {WIND_P / 1000:.2f} kN"), "anchor": "wind-resultant-b", "fxN": 0, "fyN": 0, "fzN": r(WIND_P, 1), "source": "literature", "reference": "en-1991-1-4"}]},
    {"id": "reinstalled-wind-suction", "title": T("Reinstalled under wind suction at the building edge", "Wieder eingebaut unter Windsog an der Gebäudeecke"), "description": T(f"Wind suction {-QP * CPE_A / 1000:.2f} kN/m2 (cpe -1.2, edge zone A) pulls the face outwards; the head ties and the friction of the bed hold it.", f"Windsog {-QP * CPE_A / 1000:.2f} kN/m2 (cpe -1,2, Randbereich A) zieht die Fläche nach außen; Kopfwinkel und Reibung des Betts halten sie."), "bones": REINSTALLED_BONES, "ground": "authored", "contacts": REINSTALLED_CONTACTS, "thermal": binding(**REINSTALLED_THERMAL), "loads": [
        {"id": "wind-suction-a", "kind": "wind", "title": T(f"Wind suction on the left half, {-WIND_S / 1000:.2f} kN", f"Windsog auf die linke Hälfte, {-WIND_S / 1000:.2f} kN"), "anchor": "wind-resultant-a", "fxN": 0, "fyN": 0, "fzN": r(WIND_S, 1), "source": "literature", "reference": "en-1991-1-4"},
        {"id": "wind-suction-b", "kind": "wind", "title": T(f"Wind suction on the right half, {-WIND_S / 1000:.2f} kN", f"Windsog auf die rechte Hälfte, {-WIND_S / 1000:.2f} kN"), "anchor": "wind-resultant-b", "fxN": 0, "fyN": 0, "fzN": r(WIND_S, 1), "source": "literature", "reference": "en-1991-1-4"}]},
]

CLIPS = [
    {"id": "lift", "title": T("Lift out of the rack", "Aus dem Bock heben"), "description": T(f"The crane takes the load at the two loops, pulls the panel upright off the rack legs while its inner bottom edge still rests on the bearers, lets it swing {HANG:.1f} degrees out of plumb as it leaves them and hoists it {LIFT} m; rack and bearers stay where they are.", f"Der Kran nimmt die Last an den beiden Schlaufen auf, zieht die Platte von den Bockschenkeln aufrecht, während ihre innere Unterkante noch auf den Kanthölzern ruht, lässt sie beim Abheben {HANG:.1f} Grad schief pendeln und hebt sie {LIFT} m an; Bock und Kanthölzer bleiben stehen."), "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none", "keys": [
        {"t": 0, "pose": "stored", "ease": "smooth"},
        {"t": 3, "pose": "stored", "ease": "smooth", "bones": {"pivot": {"twistDeg": 0}}},
        {"t": 4.5, "pose": "lifted", "ease": "smooth", "bones": {"hoist": {"dy": 0.1}}},
        {"t": 10, "pose": "lifted", "ease": "smooth"}],
        "contacts": [{"anchor": "bearer-contact-a", "fromS": 0, "toS": 4.5}, {"anchor": "bearer-contact-b", "fromS": 0, "toS": 4.5}, {"anchor": "rack-lean-a", "fromS": 0, "toS": 1}, {"anchor": "rack-lean-b", "fromS": 0, "toS": 1}, {"anchor": "lifting-loop-a", "fromS": 1, "toS": 10}, {"anchor": "lifting-loop-b", "fromS": 1, "toS": 10}],
        "phases": [{"id": "upright", "fromS": 0, "toS": 3, "title": T("Pull upright", "Aufrichten")}, {"id": "lift-off", "fromS": 3, "toS": 4.5, "title": T("Lift off the bearers", "Von den Kanthölzern abheben")}, {"id": "hoist", "fromS": 4.5, "toS": 10, "title": T("Hoist", "Heben")}],
        "events": [{"t": 1, "kind": "contact-off", "title": T("Panel leaves the rack legs", "Platte verlässt die Bockschenkel"), "target": "rack-lean-a"}, {"t": 4.5, "kind": "contact-off", "title": T("Panel leaves the bearers", "Platte verlässt die Kanthölzer"), "target": "bearer-contact-a"}], "tracks": []},
    {"id": "wind-gust", "title": T("Wind suction gust on the reinstalled panel", "Windsogböe an der wieder eingebauten Platte"), "description": T("The suction at the building edge rises to its design value and falls again; the head ties take the outward pull.", "Der Sog an der Gebäudeecke steigt auf seinen Bemessungswert und fällt wieder; die Kopfwinkel nehmen den Zug nach außen auf."), "durationS": 6, "fps": 25, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "reinstalled-wind-suction", "ease": "smooth"}, {"t": 6, "pose": "reinstalled-wind-suction", "ease": "smooth"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in REINSTALLED_CONTACTS],
        "phases": [{"id": "gust", "fromS": 0, "toS": 6, "title": T("Gust", "Böe")}], "events": [{"t": 3, "kind": "load-on", "title": T("Peak suction", "Sogspitze")}],
        "tracks": [{"target": "load:wind-suction-a", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 3, "v": 1}, {"t": 6, "v": 0}]}, {"target": "load:wind-suction-b", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 3, "v": 1}, {"t": 6, "v": 0}]}]},
    {"id": "sun-passing", "title": T("Sun passes over the reinstalled facade", "Die Sonne zieht über die wieder eingebaute Fassade"), "description": T("Over a looped day the sunlit share of the exposed-aggregate face rises from shade to full sun and falls again while the room side stays at 20 degC.", "Über einen geschlossenen Tag steigt der besonnte Anteil der Waschbetonfläche vom Schatten zur vollen Sonne und fällt wieder, während die Raumseite bei 20 °C bleibt."), "durationS": 12, "fps": 25, "loop": True, "rootMotion": "none", "keys": [{"t": 0, "pose": "reinstalled", "ease": "smooth"}, {"t": 12, "pose": "reinstalled", "ease": "smooth"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 12} for a in REINSTALLED_CONTACTS],
        "phases": [{"id": "day", "fromS": 0, "toS": 12, "title": T("Day", "Tag")}], "events": [{"t": 6, "kind": "thermal", "title": T("Face in full sun", "Fläche in voller Sonne")}],
        "tracks": [{"target": "region:weather-face:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 6, "v": 1}, {"t": 12, "v": 0}]}, {"target": "region:window-reveals:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 6, "v": 0.4}, {"t": 12, "v": 0}]}]},
]

HC_FACE = (Z_SI - Z_OUT) * RHO_C * 1000
DELTA = math.sqrt(2.3 * 86400 / (math.pi * RHO_C * 1000))
HC_LEAF = DELTA / 2 * RHO_C * 1000


def layer(id_, title, kind, material, t, k, rho, c, source="literature", ref="en-iso-10456"):
    """🧅️ One thermal layer with its resistance and heat capacity from thickness and material."""
    return {"id": id_, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r(t / k, 5), "heatCapacityJM2K": r(t * rho * c, 1), "source": source, "reference": ref}


REGIONS = [
    {"id": "weather-face", "title": T("Exposed-aggregate face of the weather shell, sandwich of three layers", "Waschbetonfläche der Wetterschale, Sandwich aus drei Schichten"), "kind": "wall", "model": "passive", "material": "exposed-aggregate-concrete", "areaM2": r(AREA_FACE, 3), "characteristicLengthM": H, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [
        layer("inner-leaf", T("Load-bearing leaf, reinforced concrete 140 mm", "Tragschicht, Stahlbeton 140 mm"), "masonry", "load-bearing-concrete", Z_IN - Z_IL, 2.3, RHO_C, 1000),
        layer("insulation", T("Polystyrene insulation 60 mm", "Polystyrol-Dämmschicht 60 mm"), "insulation", "eps-insulation", Z_IL - Z_SI, 0.04, RHO_EPS, 1450, "authored-estimate"),
        layer("weather-shell", T("Exposed-aggregate weather shell 60 mm", "Waschbeton-Wetterschale 60 mm"), "masonry", "exposed-aggregate-concrete", Z_SI - Z_OUT, 2.3, RHO_C, 1000)],
     "heatCapacityJM2K": r(HC_FACE, 0), "source": "authored-estimate", "reference": "en-iso-13786"},
    {"id": "inner-face", "title": T("Former room face of the load-bearing leaf with filler and wallpaper remnants", "Frühere Raumseite der Tragschicht mit Spachtel- und Tapetenresten"), "kind": "wall", "model": "passive", "material": "interior-finish", "areaM2": r(AREA_INNER, 3), "characteristicLengthM": H, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": r(HC_LEAF, 0), "source": "authored-estimate", "reference": "en-iso-13786"},
    {"id": "sawn-edges", "title": T("Diamond-sawn perimeter edges through all three layers", "Diamantgesägte Randflächen durch alle drei Schichten"), "kind": "surface", "model": "passive", "material": "sawn-concrete", "areaM2": r(AREA_EDGES, 3), "characteristicLengthM": Z_IN - Z_OUT, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": r(HC_LEAF, 0), "source": "authored-estimate", "reference": "en-iso-13786"},
    {"id": "window-reveals", "title": T("Cast window reveals with sealant and dowel remnants", "Geschalte Fensterleibungen mit Dichtstoff- und Dübelresten"), "kind": "surface", "model": "passive", "material": "reveal-concrete", "areaM2": r(AREA_REVEALS, 3), "characteristicLengthM": Z_IN - Z_OUT, "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [], "heatCapacityJM2K": r(HC_LEAF, 0), "source": "authored-estimate", "reference": "en-iso-13786"},
    {"id": "exposed-steel", "title": T("Lifting loops and cut reinforcement stubs, rusted", "Transportschlaufen und abgetrennte Bewehrungsstummel, angerostet"), "kind": "surface", "model": "passive", "material": "rusted-steel", "areaM2": r(AREA_STEEL, 4), "characteristicLengthM": LOOP_BAR, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": r(LOOP_BAR / 4 * RHO_ST * 450, 0), "source": "authored-estimate", "reference": "en-iso-10456"},
    {"id": "rack-frame", "title": T("Painted steel tubes of the A-frame rack", "Lackierte Stahlrohre des A-Bocks"), "kind": "surface", "model": "passive", "material": "rack-steel", "areaM2": RACK_AREA, "characteristicLengthM": LEG, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": r(0.004 * RHO_ST * 450, 0), "source": "authored-estimate", "reference": "en-iso-10456"},
    {"id": "bearer-timber", "title": T("Squared-timber bearers", "Kanthölzer"), "kind": "surface", "model": "passive", "material": "bearer-softwood", "areaM2": r(2 * (2 * 0.1 * 0.1 + 4 * 0.1 * (BEARER_Z1 - BEARER_Z0)), 4), "characteristicLengthM": 0.1, "geometry": "horizontal-plate-up", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": r(0.02 * 450 * 1600, 0), "source": "authored-estimate", "reference": "en-iso-10456"},
]

THERMAL = {
    "environment": {"airTempC": 15, "meanRadiantTempC": 15, "skyTempC": 2, "groundTempC": 14, "relativeHumidity": 0.65, "windMS": 2, "windDirectionDeg": 90, "solarWM2": 600, "diffuseFraction": 0.25, "pressurePa": 101325},
    "passengers": [], "operating": [], "regions": REGIONS, "emitters": [],
    "assumptions": [
        T(f"Reference case: a mild sunny day in the yard, air 15 degC, sky 2 degC, wind 2 m/s, sun 600 W/m2; all values are authored estimates. The panel is a passive element: it draws no power and releases no heat.", f"Referenzfall: ein milder sonniger Tag auf dem Hof, Luft 15 °C, Himmel 2 °C, Wind 2 m/s, Sonne 600 W/m2; alle Werte sind Schätzwerte. Die Platte ist ein passives Bauteil: sie nimmt keine Leistung auf und gibt keine Wärme ab."),
        T(f"The sandwich (140 mm reinforced concrete, 60 mm polystyrene, 60 mm exposed-aggregate shell) has U = {1 / (0.13 + (Z_IN - Z_IL) / 2.3 + (Z_IL - Z_SI) / 0.04 + (Z_SI - Z_OUT) / 2.3 + 0.04):.2f} W/(m2 K) over R_si + sum d/lambda + R_se (EN ISO 6946) with aged polystyrene at 0.040 W/mK; the concrete reveal ribs and the shell anchors are thermal bridges the passive regions do not resolve.", f"Das Sandwich (140 mm Stahlbeton, 60 mm Polystyrol, 60 mm Waschbetonschale) hat U = {1 / (0.13 + (Z_IN - Z_IL) / 2.3 + (Z_IL - Z_SI) / 0.04 + (Z_SI - Z_OUT) / 2.3 + 0.04):.2f} W/(m2 K) aus R_si + Summe d/lambda + R_se (EN ISO 6946) mit gealtertem Polystyrol bei 0,040 W/mK; die Betonrippen der Leibungen und die Schalenanker sind Wärmebrücken, die die passiven Regionen nicht auflösen."),
        T(f"The weathered exposed-aggregate face absorbs 0.72 of the sun (new washed concrete about 0.6), the fresh saw cuts 0.55, the rusted steel 0.75; heat is stored in the 60 mm shell in front of the insulation ({HC_FACE / 1000:.0f} kJ/(m2 K)) and in about {DELTA / 2 * 1000:.0f} mm of the leaf and edges (half the 24 h penetration depth of {DELTA * 1000:.0f} mm, EN ISO 13786).", f"Die verwitterte Waschbetonfläche absorbiert 0,72 der Sonne (neuer Waschbeton etwa 0,6), die frischen Sägeschnitte 0,55, der angerostete Stahl 0,75; Wärme wird in der 60-mm-Schale vor der Dämmung ({HC_FACE / 1000:.0f} kJ/(m2 K)) und in etwa {DELTA / 2 * 1000:.0f} mm der Tragschicht und der Ränder gespeichert (die Hälfte der 24-h-Eindringtiefe von {DELTA * 1000:.0f} mm, EN ISO 13786)."),
        T("In the reinstalled poses the room face is bound as a contact with the 20 degC room air (exposed 0, contact 1), the sawn edges and the loops sit in mortar joints against the neighbouring panels at about 12 degC, and half of each reveal faces the outside air beside the new window.", "In den Einbauposen ist die Raumseite als Kontakt mit der 20 °C warmen Raumluft gebunden (frei 0, Kontakt 1), die Sägekanten und die Schlaufen liegen in Mörtelfugen an den Nachbarplatten bei etwa 12 °C, und je die Hälfte der Leibungen liegt neben dem neuen Fenster an der Außenluft."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
    T(f"Each pose names its own supports as contacts: stored, the bottom edge on the two bearers (friction 0.6) and the inner face against the two rack legs (unilateral, normal {LEAN} degrees off the horizontal); lifted, the two loops as pins of the two-leg sling; reinstalled, the mortar bed in two halves (friction 0.6) and two steel angle ties at the head that hold the panel across the wall.", f"Jede Pose nennt ihre eigenen Auflager als Kontakte: gelagert die Unterkante auf den beiden Kanthölzern (Reibung 0,6) und die Innenseite an den beiden Bockschenkeln (einseitig, Normale {LEAN} Grad gegen die Waagerechte); angehoben die beiden Schlaufen als Gelenke des Zweistranggehänges; wieder eingebaut das Mörtelbett in zwei Hälften (Reibung 0,6) und zwei Stahlwinkel am Kopf, die die Platte quer zur Wand halten."),
    T(f"The rack and the bearers are yard equipment, not part of the reclaimed component: the archive solves one rigid body per pose, so they carry no segment mass and every pose shows the load path of the panel alone. Their own masses (rack about 0.25 t, bearers about 7 kg each) stay on the ground in every pose.", f"Bock und Kanthölzer sind Hofausrüstung, nicht Teil des gebrauchten Bauteils: das Archiv löst je Pose einen starren Körper, daher tragen sie keine Segmentmasse, und jede Pose zeigt den Lastpfad der Platte allein. Ihre eigenen Massen (Bock etwa 0,25 t, Kanthölzer je etwa 7 kg) bleiben in jeder Pose am Boden."),
    T(f"Panel mass {MASS:.0f} kg from section x density: 140 mm leaf, 60 mm reveal rib and 60 mm shell at 2300 kg/m3 (reinforced concrete with 1 % steel, EN ISO 10456), 60 mm polystyrene at 20 kg/m3, steel 7850 kg/m3; spread uniformly along the panel, centre of mass {Z_COM * 1000:.1f} mm on the room side of the mid-plane, so the bone lies in that plane.", f"Plattenmasse {MASS:.0f} kg aus Querschnitt x Dichte: 140 mm Tragschicht, 60 mm Leibungsrippe und 60 mm Schale mit 2300 kg/m3 (Stahlbeton mit 1 % Stahl, EN ISO 10456), 60 mm Polystyrol mit 20 kg/m3, Stahl 7850 kg/m3; gleichmäßig entlang der Platte verteilt, Schwerpunkt {Z_COM * 1000:.1f} mm raumseitig der Mittelebene, daher liegt der Knochen in dieser Ebene."),
    T(f"The weather shell hangs on stainless anchors from the load-bearing leaf and carries nothing; the member section is the 140 mm leaf through the window, parapet 0.80 m plus lintel 0.40 m acting together (A = {SEC_AREA:.3f} m2, I = {SEC_I:.4f} m4), an upper bound for the frame action around the opening. Concrete B 300 is taken as C20/25 (Ecm 30 GPa); the 6.3 t crane class of WBS 70 holds the {MASS / 1000:.2f} t panel.", f"Die Wetterschale hängt an Edelstahlankern an der Tragschicht und trägt nichts; der Stabquerschnitt ist die 140-mm-Tragschicht im Fensterschnitt, Brüstung 0,80 m und Sturz 0,40 m gemeinsam (A = {SEC_AREA:.3f} m2, I = {SEC_I:.4f} m4), eine obere Schranke der Rahmenwirkung um die Öffnung. Beton B 300 wird als C20/25 angesetzt (Ecm 30 GPa); die 6,3-t-Kranklasse der WBS 70 hält die {MASS / 1000:.2f}-t-Platte."),
    T(f"Wind on the reinstalled panel after EN 1991-1-4 with qp = 0.65 kN/m2 (DIN NA wind zone 2, inland, up to 10 m): pressure cpe +0.8, suction cpe -1.2 at the building edge, each applied as two resultants at mid-height of the two halves of the 16.8 m2 face (authored estimate, not a code check).", f"Wind auf die wieder eingebaute Platte nach EN 1991-1-4 mit qp = 0,65 kN/m2 (DIN NA Windzone 2, Binnenland, bis 10 m): Druck cpe +0,8, Sog cpe -1,2 am Gebäuderand, jeweils als zwei Resultierende in halber Höhe der beiden Hälften der 16,8-m2-Fläche (Schätzwert, kein Normnachweis)."),
]}


def shape(id_, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None, solid=None):
    """✏️ A shape of a representation with its role, order, fill, tone and binding."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if hatch is not None:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    if solid is not None:
        s["solid"] = solid
    s.update(geom)
    return s


def rect(x0, y0, x1, y1):
    """⬛️ A rect geometry from two corners."""
    return {"kind": "rect", "x": r(min(x0, x1)), "y": r(min(y0, y1)), "w": r(abs(x1 - x0)), "h": r(abs(y1 - y0))}


def poly(points, closed=True):
    """🔷️ A poly geometry."""
    return {"kind": "poly", "points": [[r(p[0]), r(p[1])] for p in points], "closed": closed}


def path(rings):
    """〰️ A path geometry of closed rings (even-odd)."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r(p[0])} {r(p[1])}" for p in ring) + " Z" for ring in rings)}


def ext(z0, z1):
    """🌀️ A lateral extruded solid hint between two rest world z positions."""
    return {"kind": "extruded", "sweep": "w", "z0": r(z0), "z1": r(z1)}


def ellipse(cx, cy, rr):
    """⭕️ A circle geometry."""
    return {"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": rr, "ry": rr}


def panel_full():
    """⬛️ The panel rectangle in bone-local coordinates."""
    return uv(PANEL)




ZN0 = z_face(0.10)
ZF0 = 2 * ZR - ZN0
A_OPEN = [(ZN0, RUN_H), (ZR - D_LEG, LEG_TOP), (ZR + D_LEG, LEG_TOP), (ZF0, RUN_H), (ZF0 - D_LEG, RUN_H), (ZR, LEG_TOP - 0.05), (ZN0 + D_LEG, RUN_H)]
NOTCH = [(ZN0 + D_LEG, RUN_H), (ZF0 - D_LEG, RUN_H), (ZR, LEG_TOP - 0.05)]
A_ON_RUNNER = [(RUN_Z0, 0.0), (RUN_Z1, 0.0), (RUN_Z1, RUN_H), (ZF0, RUN_H), (ZR + D_LEG, LEG_TOP), (ZR - D_LEG, LEG_TOP), (ZN0, RUN_H), (RUN_Z0, RUN_H)]
SHELLS = {50: SHELL_FINE, 100: SHELL_MID, 200: PANEL, 500: PANEL}
PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
    500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
}
HATCHES = {
    50: [{"id": "exposed-aggregate", "title": T("Exposed aggregate (washed gravel)", "Waschbeton (ausgewaschener Kies)"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1},
         {"id": "concrete-cut", "title": T("Sawn concrete face", "Gesägte Betonfläche"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1},
         {"id": "insulation", "title": T("Polystyrene insulation", "Polystyrol-Dämmung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.4, "penMm": 0.1}],
    100: [{"id": "exposed-aggregate", "title": T("Exposed aggregate (washed gravel)", "Waschbeton (ausgewaschener Kies)"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
    200: [],
    500: [],
}
PANEL_REGIONS_500 = ["weather-face", "sawn-edges", "window-reveals", "exposed-steel"]


def inset(sc):
    """📏️ Half the detail pen of a scale in metres: strokes along an edge keep their centre line inside the body."""
    return PENS[sc]["detail"] * sc / 2000


def panel_elevation(sc):
    """🧱️ The panel in the side elevation at a scale: back cell, exposed-aggregate face with the opening, reveal and sawn-edge strokes, loops, stubs and traces."""
    shell = SHELLS[sc]
    face_binding = {"regions": PANEL_REGIONS_500} if sc == 500 else {"region": "weather-face"}
    out = [
        shape("inner-face", "panel", "body", 0.5, True, 0.48, path([uv(PANEL), uv(OPENING)]), region="inner-face", material="load-bearing-concrete"),
        shape("weather-face", "panel", "body", 2, True, 0.62 if sc <= 100 else 0.55, path([uv(shell), uv(OPENING)]), material="exposed-aggregate-concrete", hatch="exposed-aggregate" if sc <= 100 else None, **face_binding),
    ]
    if sc <= 200:
        e = inset(sc)
        out.append(shape("reveal-arris", "panel", "detail", 3, False, 0.3, poly(uv(rect_pts(OX0 - e, OY0 - e, OX1 + e, OY1 + e))), region="window-reveals"))
        low = {50: 0.13, 100: 0.12, 200: e}[sc]
        left_low = {50: 0.15, 100: 0.14, 200: e}[sc]
        out.append(shape("sawn-edge-left", "panel", "detail", 3, False, 0.3, poly(uv([(e, low), (e, H - e)]), closed=False), region="sawn-edges"))
        out.append(shape("sawn-edge-right", "panel", "detail", 3, False, 0.3, poly(uv([(L - e, e), (L - e, H - e)]), closed=False), region="sawn-edges"))
        if sc <= 100:
            out.append(shape("sawn-edge-top", "panel", "detail", 3, False, 0.3, poly(uv([(e, H - e), (L - e, H - e)]), closed=False), region="sawn-edges"))
            out.append(shape("sawn-edge-bottom", "panel", "detail", 3, False, 0.3, poly(uv([(left_low, e), (L - e, e)]), closed=False), region="sawn-edges"))
    if sc == 50:
        out.append(shape("sealant-residue-left", "panel", "texture", 3.2, False, 0.3, poly(uv([(0.035, 0.3), (0.035, 1.25)]), closed=False), region="weather-face"))
        out.append(shape("sealant-residue-right", "panel", "texture", 3.2, False, 0.3, poly(uv([(L - 0.035, 1.5), (L - 0.035, 2.6)]), closed=False), region="weather-face"))
        for k, y in enumerate((0.7, 2.1)):
            out.append(shape(f"bracket-hole-{k + 1}", "panel", "detail", 3.3, False, 0.3, ellipse(5.7, v(y), 0.012), region="weather-face"))
        out.append(shape("spall-fracture", "panel", "texture", 3.2, False, 0.3, poly(uv([(4.88, 2.79), (4.96, 2.77), (5.05, 2.782), (5.14, 2.77)]), closed=False), region="weather-face"))
    for k, xc in zip("ab", LOOPS):
        if sc <= 100:
            out.append(shape(f"lifting-loop-{k}", "panel", "body", 4, True, 0.22, poly(uv(loop_outline(xc))), region="exposed-steel", material="rusted-steel"))
        else:
            out.append(shape(f"lifting-loop-{k}", "panel", "body", 4, True, 0.22, rect(u(xc - LOOP_R_OUT), v(H), u(xc + LOOP_R_OUT), v(LOOP_TOP)), region="exposed-steel", material="rusted-steel"))
    if sc <= 100:
        for side, x0 in (("left", -STUB_LEN), ("right", L)):
            for k, y in enumerate(STUB_Y):
                out.append(shape(f"rebar-stub-{side}-{k + 1}", "panel", "body", 4, True, 0.22, rect(x0, v(y) - STUB_D / 2, x0 + STUB_LEN, v(y) + STUB_D / 2), region="exposed-steel", material="rusted-steel"))
    return out


def panel_front(sc):
    """🧱️ The panel in the front elevation: the sawn end face with its layers as lateral solids, loops and stubs, cut bar ends."""
    full = uv(PANEL)
    out = []
    if sc == 500:
        out.append(shape("panel-edge", "panel", "body", 2, True, 0.5, poly(full), regions=PANEL_REGIONS_500, material="load-bearing-concrete", solid=ext(Z_OUT, Z_IN)))
    else:
        layers = [("shell-edge", Z_OUT, Z_SI, 0.58, "exposed-aggregate-concrete", "concrete-cut", 2.0), ("insulation-edge", Z_SI, Z_IL, 0.92, "eps-insulation", "insulation", 2.1), ("leaf-edge", Z_IL, Z_IN, 0.5, "load-bearing-concrete", "concrete-cut", 2.2)]
        for id_, z0, z1, tone, material, hatch, order in layers:
            out.append(shape(id_, "panel", "body", order, True, tone, poly(full), region="sawn-edges", material=material, hatch=hatch if sc == 50 else None, solid=ext(z0, z1)))
    for k, xc in zip("ab", LOOPS):
        if sc <= 100:
            out.append(shape(f"lifting-loop-{k}", "panel", "body", 3, True, 0.22, poly(uv(loop_outline(xc))), region="exposed-steel", material="rusted-steel", solid=ext(STUB_Z - LOOP_BAR / 2, STUB_Z + LOOP_BAR / 2)))
        else:
            half = 0.015 if sc == 200 else 0.0375
            out.append(shape(f"lifting-loop-{k}", "panel", "body", 3, True, 0.22, poly(uv(rect_pts(xc - LOOP_R_OUT, H, xc + LOOP_R_OUT, LOOP_TOP))), region="exposed-steel", material="rusted-steel", solid=ext(STUB_Z - half, STUB_Z + half)))
    if sc <= 100:
        for k, y in enumerate(STUB_Y):
            out.append(shape(f"rebar-stub-right-{k + 1}", "panel", "body", 3, True, 0.22, poly(uv(rect_pts(L, y - STUB_D / 2, L + STUB_LEN, y + STUB_D / 2))), region="exposed-steel", material="rusted-steel", solid=ext(STUB_Z - STUB_D / 2, STUB_Z + STUB_D / 2)))
    if sc == 50:
        n = 0
        for zz in (0.03, 0.10):
            for yy in (0.3, 0.9, 1.5, 2.1, 2.6):
                n += 1
                out.append(shape(f"cut-bar-end-{n}", "panel", "texture", 2.5, False, 0.3, ellipse(zz, HB + yy, 0.005), region="sawn-edges"))
        for k, yy in enumerate((0.7, 2.1)):
            out.append(shape(f"shell-anchor-{k + 1}", "panel", "texture", 2.5, False, 0.3, poly([(Z_SI + 0.005, HB + yy), (Z_IL - 0.005, HB + yy)], closed=False), region="sawn-edges"))
    return out


def panel_plan(sc):
    """🧱️ The panel from above: the leaning prism as a lateral solid, the sawn top edge, loops and stubs."""
    full = uv(PANEL)
    top = uv(rect_pts(0.0, H - 0.02, L, H))
    out = [shape("panel-plan", "panel", "body", 2, True, 0.55, poly(full), material="exposed-aggregate-concrete", solid=ext(Z_OUT, Z_IN), **({"regions": PANEL_REGIONS_500} if sc == 500 else {"region": "weather-face"}))]
    if sc == 50:
        for id_, z0, z1, tone, material in (("top-edge-shell", Z_OUT, Z_SI, 0.6, "exposed-aggregate-concrete"), ("top-edge-insulation", Z_SI, Z_IL, 0.92, "eps-insulation"), ("top-edge-leaf", Z_IL, Z_IN, 0.5, "load-bearing-concrete")):
            out.append(shape(id_, "panel", "body", 3, True, tone, poly(top), region="sawn-edges", material=material, solid=ext(z0, z1)))
    elif sc <= 200:
        out.append(shape("top-edge", "panel", "body", 3, True, 0.6, poly(top), region="sawn-edges", material="sawn-concrete", solid=ext(Z_OUT, Z_IN)))
    for k, xc in zip("ab", LOOPS):
        if sc <= 100:
            out.append(shape(f"lifting-loop-{k}", "panel", "body", 4, True, 0.22, poly(uv(loop_outline(xc))), region="exposed-steel", material="rusted-steel", solid=ext(STUB_Z - LOOP_BAR / 2, STUB_Z + LOOP_BAR / 2)))
        else:
            half = 0.015 if sc == 200 else 0.0375
            out.append(shape(f"lifting-loop-{k}", "panel", "body", 4, True, 0.22, poly(uv(rect_pts(xc - LOOP_R_OUT, H, xc + LOOP_R_OUT, LOOP_TOP))), region="exposed-steel", material="rusted-steel", solid=ext(STUB_Z - half, STUB_Z + half)))
    if sc <= 100:
        for side, x0 in (("left", -STUB_LEN), ("right", L)):
            y = STUB_Y[-1]
            out.append(shape(f"rebar-stub-{side}-3", "panel", "body", 4, True, 0.22, poly(uv(rect_pts(x0, y - STUB_D / 2, x0 + STUB_LEN, y + STUB_D / 2))), region="exposed-steel", material="rusted-steel", solid=ext(STUB_Z - STUB_D / 2, STUB_Z + STUB_D / 2)))
    return out


def bearers_2d(sc, view):
    """🪵️ The two squared-timber bearers in one view."""
    out = []
    tone = 0.55 if sc <= 100 else 0.5
    for k, xt in zip("ab", TRESTLES):
        g = {"elevation": rect(0, 0, 0.1, 0.1), "front": rect(BEARER_Z0, RUN_H, BEARER_Z1, HB), "plan": rect(xt - 0.05, BEARER_Z0, xt + 0.05, BEARER_Z1)}[view]
        out.append(shape(f"bearer-{k}", f"bearer-{k}", "body", 1.5, True, tone, g, region="bearer-timber", material="bearer-softwood"))
        if view == "elevation" and sc <= 100:
            out.append(shape(f"bearer-grain-{k}", f"bearer-{k}", "texture", 1.6, False, 0.3, poly([(0.015, 0.03), (0.05, 0.072), (0.085, 0.03)], closed=False), region="bearer-timber"))
    return out


def rack_2d(sc, view):
    """🪜️ The A-frame rack in one view at a scale: legs, runners, ridge and base ties, merged into trestles from 1:100 (front) and 1:200 (elevation, plan)."""
    st = {"region": "rack-frame", "material": "rack-steel"}
    out = []
    if view == "elevation":
        out.append(shape("ridge", "rack", "body", 0.2, True, 0.35, rect(TRESTLES[0] - LEG / 2, LEG_TOP, TRESTLES[1] + LEG / 2, LEG_TOP + RIDGE_H), **st))
        for k, xt in zip("ab", TRESTLES):
            if sc <= 100:
                out.append(shape(f"leg-{k}", "rack", "body", 0.1, True, 0.35, rect(xt - LEG / 2, RUN_H, xt + LEG / 2, LEG_TOP), **st))
                out.append(shape(f"runner-{k}", "rack", "body", 0.3, True, 0.3, rect(xt - 0.05, 0, xt + 0.05, RUN_H), **st))
            else:
                out.append(shape(f"trestle-{k}", "rack", "body", 0.1, True, 0.32, rect(xt - 0.05, 0, xt + 0.05, LEG_TOP), **st))
        out.append(shape("base-tie", "rack", "body", 0.4, True, 0.3, rect(TRESTLES[0] + 0.05, 0, TRESTLES[1] - 0.05, LEG), **st))
    elif view == "front":
        if sc == 50:
            out.append(shape("leg-near", "rack", "body", 0.2, True, 0.35, poly([(p[1], p[0]) for p in LEG_NEAR]), **st))
            out.append(shape("leg-far", "rack", "body", 0.2, True, 0.35, poly([(p[1], p[0]) for p in LEG_FAR]), **st))
            out.append(shape("runner", "rack", "body", 0.3, True, 0.3, rect(RUN_Z0, 0, RUN_Z1, RUN_H), **st))
        elif sc == 100:
            out.append(shape("trestle", "rack", "body", 0.2, True, 0.33, poly(A_OPEN), **st))
            out.append(shape("runner", "rack", "body", 0.3, True, 0.3, rect(RUN_Z0, 0, RUN_Z1, RUN_H), **st))
        elif sc == 200:
            out.append(shape("trestle", "rack", "body", 0.2, True, 0.32, path([A_ON_RUNNER, NOTCH]), **st))
        else:
            out.append(shape("trestle", "rack", "body", 0.2, True, 0.32, poly(A_ON_RUNNER), **st))
        out.append(shape("ridge-end", "rack", "body", 0.25, True, 0.3, rect(ZR - LEG / 2, LEG_TOP, ZR + LEG / 2, LEG_TOP + RIDGE_H), **st))
    else:
        out.append(shape("ridge", "rack", "body", 0.6, True, 0.35, rect(TRESTLES[0] - LEG / 2, ZR - LEG / 2, TRESTLES[1] + LEG / 2, ZR + LEG / 2), **st))
        for k, xt in zip("ab", TRESTLES):
            if sc == 50:
                out.append(shape(f"runner-{k}", "rack", "body", 0.2, True, 0.3, rect(xt - 0.05, RUN_Z0, xt + 0.05, RUN_Z1), **st))
                out.append(shape(f"leg-near-{k}", "rack", "body", 0.5, True, 0.35, rect(xt - LEG / 2, ZN0, xt + LEG / 2, ZR), **st))
                out.append(shape(f"leg-far-{k}", "rack", "body", 0.5, True, 0.35, rect(xt - LEG / 2, ZR, xt + LEG / 2, ZF0), **st))
            elif sc == 100:
                out.append(shape(f"runner-{k}", "rack", "body", 0.2, True, 0.3, rect(xt - 0.05, RUN_Z0, xt + 0.05, RUN_Z1), **st))
                out.append(shape(f"trestle-{k}", "rack", "body", 0.5, True, 0.34, rect(xt - LEG / 2, ZN0, xt + LEG / 2, ZF0), **st))
            else:
                out.append(shape(f"trestle-{k}", "rack", "body", 0.5, True, 0.32, rect(xt - 0.05, RUN_Z0, xt + 0.05, RUN_Z1), **st))
        for id_, zt in (("base-tie-near", TIE_NEAR), ("base-tie-far", TIE_FAR)):
            out.append(shape(id_, "rack", "body", 0.3, True, 0.3, rect(TRESTLES[0] + 0.05, zt - LEG / 2, TRESTLES[1] - 0.05, zt + LEG / 2), **st))
    return out


def geometry(sc):
    """🗺️ The three views of one scale."""
    return {
        "elevation": rack_2d(sc, "elevation") + bearers_2d(sc, "elevation") + panel_elevation(sc),
        "front": rack_2d(sc, "front") + bearers_2d(sc, "front") + panel_front(sc),
        "plan": rack_2d(sc, "plan") + bearers_2d(sc, "plan") + panel_plan(sc),
    }


def solid(id_, bone, material, tone, kind, role="body", **fields):
    """🧊️ A volume solid of a bone in its rest frame."""
    s = {"id": id_, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(fields)
    return s


def plate(id_, material, tone, outer, holes, z0, z1):
    """🟫️ A panel layer: the outline with holes in the (u, v) plane, extruded across the thickness."""
    return solid(id_, "panel", material, tone, "extrude", profile=uv(outer), holes=[uv(h) for h in holes], plane="xy", **{"from": w(z0), "to": w(z1)})


def box(id_, bone, material, tone, size, at, **extra):
    """📦️ A centred box."""
    return solid(id_, bone, material, tone, "box", size=[r(s_) for s_ in size], transform={"translation": [r(a) for a in at]}, **extra)


def yz_prism(id_, material, tone, zy_points, x0, x1, holes=None, repeat=None):
    """🔺️ A rack part: an outline in the (height, lateral) plane extruded along the rack."""
    fields = {"profile": [[r(y), r(z)] for (z, y) in zy_points], "plane": "yz", "from": r(x0), "to": r(x1)}
    if holes:
        fields["holes"] = [[[r(y), r(z)] for (z, y) in h] for h in holes]
    if repeat:
        fields["repeat"] = repeat
    return solid(id_, "rack", material, tone, "extrude", **fields)


def loop_solids():
    """➰️ The two cast-in lifting loops as round bars: a half-torus crown and two legs each."""
    circle = [[r(0.01 * math.cos(2 * math.pi * k / 12)), r(LOOP_RC + 0.01 * math.sin(2 * math.pi * k / 12))] for k in range(12)]
    circle.append(circle[0])
    step = {"count": 2, "translation": [LOOPS[1] - LOOPS[0], 0, 0]}
    crown = solid("loop-crowns", "panel", "rusted-steel", 0.22, "revolve", profile=circle, axis=[0, 0, 1], origin=[u(LOOPS[0]), v(H + LOOP_LEG), w(STUB_Z)], sweepRad=r(math.pi, 6), repeat=step)
    legs = [solid(f"loop-legs-{side}", "panel", "rusted-steel", 0.22, "capsule", p0=[u(LOOPS[0] + sgn * LOOP_RC), v(H + LOOP_BAR / 2), w(STUB_Z)], p1=[u(LOOPS[0] + sgn * LOOP_RC), v(H + LOOP_LEG), w(STUB_Z)], r0=LOOP_BAR / 2, r1=LOOP_BAR / 2, repeat=step) for side, sgn in (("left", -1), ("right", 1))]
    return [crown] + legs


def stub_solids():
    """✂️ The cut connection bars at both sawn vertical edges, three per edge."""
    step = {"count": 3, "translation": [0, STUB_Y[1] - STUB_Y[0], 0]}
    return [solid(f"rebar-stubs-{side}", "panel", "rusted-steel", 0.22, "revolve", profile=[[0, STUB_D / 2], [STUB_LEN, STUB_D / 2]], axis=[1, 0, 0], origin=[r(x0), v(STUB_Y[0]), w(STUB_Z)], repeat=step) for side, x0 in (("left", -STUB_LEN), ("right", L))]


def rack_solids(sc):
    """🪜️ The rack of a scale: legs, runners, ridge, ties at 1:50; trestles from 1:100, standing on their runners from 1:200."""
    steel, t = "rack-steel", 0.34
    out = [box("ridge", "rack", steel, t, [TRESTLES[1] - TRESTLES[0] + LEG, RIDGE_H, LEG], [3.0, LEG_TOP + RIDGE_H / 2, ZR])]
    if sc == 50:
        for k, xt in zip("ab", TRESTLES):
            out.append(box(f"runner-{k}", "rack", steel, 0.3, [0.1, RUN_H, RUN_Z1 - RUN_Z0], [xt, RUN_H / 2, ZR]))
            out.append(yz_prism(f"leg-near-{k}", steel, t, [(z, y) for (y, z) in LEG_NEAR], xt - LEG / 2, xt + LEG / 2))
            out.append(yz_prism(f"leg-far-{k}", steel, t, [(z, y) for (y, z) in LEG_FAR], xt - LEG / 2, xt + LEG / 2))
        for id_, zt in (("base-tie-near", TIE_NEAR), ("base-tie-far", TIE_FAR)):
            out.append(box(id_, "rack", steel, 0.3, [TRESTLES[1] - TRESTLES[0] - 0.1, LEG, LEG], [3.0, LEG / 2, zt]))
        return out
    pair = {"count": 2, "translation": [TRESTLES[1] - TRESTLES[0], 0, 0]}
    ties = box("base-ties", "rack", steel, 0.3, [TRESTLES[1] - TRESTLES[0] - 0.1, LEG, LEG], [3.0, LEG / 2, TIE_NEAR], repeat={"count": 2, "translation": [0, 0, r(TIE_FAR - TIE_NEAR)]})
    if sc == 100:
        out.append(box("runners", "rack", steel, 0.3, [0.1, RUN_H, RUN_Z1 - RUN_Z0], [TRESTLES[0], RUN_H / 2, ZR], repeat=pair))
        out.append(yz_prism("trestles", steel, t, A_OPEN, TRESTLES[0] - LEG / 2, TRESTLES[0] + LEG / 2, repeat=pair))
    elif sc == 200:
        out.append(yz_prism("trestles", steel, t, A_ON_RUNNER, TRESTLES[0] - 0.05, TRESTLES[0] + 0.05, holes=[NOTCH], repeat=pair))
    else:
        out.append(yz_prism("trestles", steel, t, A_ON_RUNNER, TRESTLES[0] - 0.05, TRESTLES[0] + 0.05, repeat=pair))
    out.append(ties)
    return out


def bearer_solids():
    """🪵️ The two squared-timber bearers on their own bones."""
    return [box(f"bearer-{k}", f"bearer-{k}", "bearer-softwood", 0.55, [0.1, 0.1, BEARER_Z1 - BEARER_Z0], [0.05, 0.05, (BEARER_Z0 + BEARER_Z1) / 2]) for k in "ab"]


def panel_solids(sc):
    """🧱️ The panel body of a scale: three layers and the reveal rib with loops and stubs, generalised towards 1:500."""
    if sc == 500:
        return [plate("panel-plate", "exposed-aggregate-concrete", 0.55, PANEL, [OPENING], Z_OUT, Z_IN), loop_blocks(sc)]
    out = [
        plate("weather-shell", "exposed-aggregate-concrete", 0.62, SHELLS[sc], [OPENING], Z_OUT, Z_SI),
        plate("insulation", "eps-insulation", 0.92, PANEL, [GROWN], Z_SI, Z_IL),
        plate("reveal-rib", "load-bearing-concrete", 0.5, GROWN, [OPENING], Z_SI, Z_IL),
        plate("load-bearing-leaf", "load-bearing-concrete", 0.5, PANEL, [OPENING], Z_IL, Z_IN),
    ]
    if sc <= 100:
        out += loop_solids() + stub_solids()
    else:
        out.append(loop_blocks(sc))
    return out


def loop_blocks(sc):
    """➰️ The lifting loops of 1:200 and 1:500 as blocks of their outline, as thick as the smallest feature."""
    t = 0.03 if sc == 200 else 0.075
    return box("lifting-loops", "panel", "rusted-steel", 0.22, [2 * LOOP_R_OUT, LOOP_TOP - H, t], [u(LOOPS[0]), v(H + (LOOP_TOP - H) / 2), w(STUB_Z)], repeat={"count": 2, "translation": [LOOPS[1] - LOOPS[0], 0, 0]})


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
JOINT_R = {50: 0.015, 100: 0.02, 200: 0.03, 500: 0.04}


def zones(sc):
    """🩹️ The surface zones of the room face, the four sawn edges and the four reveals, inset into the panel as deep as the smallest feature of the scale; only their own surface faces count, their back and side faces lie against the core or in the faces of the other regions."""
    f = FEATURE[sc]
    thick = Z_IN - Z_OUT
    mid = w((Z_OUT + Z_IN) / 2)
    out = [solid("th-room-face", "panel", "interior-finish", 0.7, "extrude", profile=uv(PANEL), holes=[uv(OPENING)], plane="xy", sectionFaces=["-w", "-u", "+u", "-v", "+v"], region="inner-face", **{"from": w(Z_IN - f), "to": w(Z_IN)})]
    edges = [("top", [L, f, thick], [L / 2, v(H - f / 2), mid], "-v"), ("bottom", [L, f, thick], [L / 2, v(f / 2), mid], "+v"), ("left", [f, H, thick], [f / 2, v(H / 2), mid], "+u"), ("right", [f, H, thick], [L - f / 2, v(H / 2), mid], "-u")]
    out += [box(f"th-sawn-{n}", "panel", "sawn-concrete", 0.7, s_, a_, region="sawn-edges", sectionFaces=[b_, "-w", "+w"]) for n, s_, a_, b_ in edges]
    cx, cy = (OX0 + OX1) / 2, (OY0 + OY1) / 2
    reveals = [("head", [OX1 - OX0, f, thick], [cx, v(OY1 + f / 2), mid], "+v"), ("sill", [OX1 - OX0, f, thick], [cx, v(OY0 - f / 2), mid], "-v"), ("jamb-left", [f, OY1 - OY0, thick], [OX0 - f / 2, v(cy), mid], "-u"), ("jamb-right", [f, OY1 - OY0, thick], [OX1 + f / 2, v(cy), mid], "+u")]
    out += [box(f"th-reveal-{n}", "panel", "reveal-concrete", 0.7, s_, a_, region="window-reveals", sectionFaces=[b_, "-w", "+w"]) for n, s_, a_, b_ in reveals]
    return out


def ref(id_, of, **label):
    """🔗️ A thermal or structural solid that reuses a body solid of the same scale."""
    s = {"id": id_, "of": of}
    s.update(label)
    return s


def thermal_solids(sc, body):
    """🌡️ The thermal list of a scale: the sandwich as nested layers at 1:50 and as one plate from 1:100, the surface zones, the steel, rack and bearers."""
    ids = [s["id"] for s in body]
    out = []
    if sc == 50:
        out += [ref("th-load-bearing-leaf", "load-bearing-leaf", region="weather-face", layer="inner-leaf"), ref("th-reveal-rib", "reveal-rib", region="weather-face", layer="inner-leaf"), ref("th-insulation", "insulation", region="weather-face", layer="insulation"), ref("th-weather-shell", "weather-shell", region="weather-face", layer="weather-shell", sectionFaces=["-u", "+u", "-v", "+v"])]
    elif sc == 500:
        out.append(ref("th-sandwich", "panel-plate", region="weather-face", sectionFaces=["+w", "-u", "+u", "-v", "+v"]))
    else:
        out.append(plate("th-sandwich", "exposed-aggregate-concrete", 0.62, PANEL, [OPENING], Z_OUT, Z_IN) | {"region": "weather-face", "sectionFaces": ["+w", "-u", "+u", "-v", "+v"]})
    out += zones(sc)
    out += [ref(f"th-{i}", i, region="exposed-steel") for i in ids if i.startswith(("loop-", "rebar-", "lifting-"))]
    out += [ref(f"th-{i}", i, region="rack-frame") for i in ids if i.startswith(("ridge", "runner", "leg-", "base-tie", "trestle"))]
    out += [ref(f"th-{i}", i, region="bearer-timber") for i in ids if i.startswith("bearer-")]
    return out


def structure_solids(sc, body):
    """🏗️ The structural list of a scale: the load-bearing leaf with its rib, the loops, the tilt joint of the panel (its mass is spread uniformly, so no concentrated mass marker; the centre of mass lies in the window void), the rack and the bearers."""
    ids = [s["id"] for s in body]
    out = []
    if sc == 500:
        out.append(plate("st-load-bearing-leaf", "load-bearing-concrete", 0.5, PANEL, [OPENING], Z_IL, Z_IN) | {"member": "panel", "part": "member"})
    else:
        out += [ref("st-load-bearing-leaf", "load-bearing-leaf", member="panel", part="member"), ref("st-reveal-rib", "reveal-rib", member="panel", part="member")]
    out += [ref(f"st-{i}", i, member="panel", part="member") for i in ids if i.startswith(("loop-", "lifting-"))]
    out.append(solid("st-tilt-joint", "panel", "load-bearing-concrete", 0.4, "capsule", p0=[0.3, v(JOINT_R[sc]), w(Z_IN - JOINT_R[sc])], p1=[L - 0.3, v(JOINT_R[sc]), w(Z_IN - JOINT_R[sc])], r0=JOINT_R[sc], r1=JOINT_R[sc], slender=True, member="panel", part="joint"))
    out += [ref(f"st-{i}", i, member="rack", part="member") for i in ids if i.startswith(("ridge", "runner", "leg-", "base-tie", "trestle"))]
    out += [ref(f"st-{i}", i, member=i, part="member") for i in ids if i.startswith("bearer-")]
    return out


VOLUME_DECISIONS = {
    50: [T("Body: the three layers of the sandwich as extrusions of their own outlines with the 2.40 x 1.60 m opening: the 60 mm exposed-aggregate shell with the chipped bottom corner and the 45 mm spall at the top edge, the 60 mm polystyrene stopped 60 mm short of the opening by the concrete reveal rib, the 140 mm load-bearing leaf; the two lifting loops as round 20 mm bars (half-torus crown and two legs), the six cut connection bars of 16 mm at the sawn edges.", "Körper: die drei Schichten des Sandwichs als Extrusionen ihrer eigenen Umrisse mit der Öffnung 2,40 x 1,60 m: die 60-mm-Waschbetonschale mit der abgeplatzten unteren Ecke und der 45-mm-Abplatzung an der Oberkante, das 60-mm-Polystyrol, das die Beton-Leibungsrippe 60 mm vor der Öffnung abschließt, die 140-mm-Tragschicht; die beiden Transportschlaufen als runde 20-mm-Stäbe (Halbtorus-Scheitel und zwei Schenkel), die sechs abgetrennten 16-mm-Anschlussstäbe an den Sägekanten."),
         T("Rack: two base runners, four 80 mm legs leaning 3.5 degrees in the plane of the panel face, the ridge tube and two base ties, all on the rack bone; the bearers on their own bones. Thermal: the sandwich as nested layers innermost first (leaf with rib, polystyrene, shell), the room face, the sawn edges and the reveals as 2 mm surface zones inset into the panel. Structure: the leaf and its rib, the loops, the tilt joint along the inner bottom edge, rack and bearers by reference; the panel mass is spread uniformly along its bone, and its centre of mass lies in the window void, so there is no concentrated mass marker.", "Bock: zwei Fußholme, vier 80-mm-Schenkel mit 3,5 Grad Neigung in der Ebene der Plattenfläche, das Firstrohr und zwei Fußstreben, alle am Bockknochen; die Kanthölzer an eigenen Knochen. Thermik: das Sandwich als verschachtelte Schichten von innen nach außen (Tragschicht mit Rippe, Polystyrol, Schale), Raumseite, Sägekanten und Leibungen als 2-mm-Oberflächenzonen in der Platte. Tragwerk: Tragschicht und Rippe, die Schlaufen, das Kippgelenk an der inneren Unterkante, Bock und Kanthölzer als Verweise; die Plattenmasse ist gleichmäßig entlang ihres Knochens verteilt und ihr Schwerpunkt liegt in der Fensteröffnung, daher gibt es keine konzentrierte Masse.")],
    100: [T("Body: the shell keeps only the chipped corner as one chamfer (the 45 mm spall is 0.45 mm on paper and is left out), layers, rib, loops and the 16 mm stubs as at 1:50 because they stay above the 15 mm feature. Rack: each trestle is one A-shaped extrusion of both legs, runners and ties as repeats.", "Körper: die Schale behält nur die abgeplatzte Ecke als eine Fase (die 45-mm-Abplatzung misst 0,45 mm auf dem Papier und entfällt), Schichten, Rippe, Schlaufen und die 16-mm-Stummel wie bei 1:50, weil sie über dem kleinsten Merkmal von 15 mm bleiben. Bock: jeder Bock ist eine A-förmige Extrusion beider Schenkel, Holme und Streben als Wiederholungen."),
          T("Thermal: the sandwich is one plate whose outer face is the weather face (its room face and edges are glued to the surface zones), the zones and the steel as at 1:50. Structure: as at 1:50 with the merged trestles.", "Thermik: das Sandwich ist eine Platte, deren Außenfläche die Wetterseite ist (Raumseite und Ränder liegen an den Oberflächenzonen an), Zonen und Stahl wie bei 1:50. Tragwerk: wie bei 1:50 mit den zusammengefassten Böcken.")],
    200: [T("Body: the shell outline is the plain rectangle with the opening (chips and spall are under the 30 mm feature), layers and rib stay because they are 60 mm; the loops are 140 x 120 x 30 mm blocks of their outline, the stubs are left out. Rack: each trestle is one extrusion of the A standing on its runner, 100 mm deep, with the opening between the legs.", "Körper: der Schalenumriss ist das einfache Rechteck mit der Öffnung (Abplatzungen liegen unter dem Merkmal von 30 mm), Schichten und Rippe bleiben, weil sie 60 mm messen; die Schlaufen sind Blöcke 140 x 120 x 30 mm ihres Umrisses, die Stummel entfallen. Bock: jeder Bock ist eine Extrusion des A auf seinem Holm, 100 mm tief, mit der Öffnung zwischen den Schenkeln."),
          T("Thermal: one sandwich plate and the surface zones; the loop blocks carry the steel region. Structure: leaf, rib, loop blocks, tilt joint, trestles, ridge, ties and bearers.", "Thermik: eine Sandwichplatte und die Oberflächenzonen; die Schlaufenblöcke tragen die Stahlregion. Tragwerk: Tragschicht, Rippe, Schlaufenblöcke, Kippgelenk, Böcke, First, Streben und Kanthölzer.")],
    500: [T("Body: the panel is one 260 mm plate with the opening (the 60 mm layers and the rib are below the 75 mm feature and merge), the loops are two 140 x 120 x 75 mm blocks so that the height of 3.12 m stays; each trestle is one solid A on its runner, the 80 mm ridge and ties remain.", "Körper: die Platte ist eine 260-mm-Platte mit der Öffnung (die 60-mm-Schichten und die Rippe liegen unter dem Merkmal von 75 mm und gehen auf), die Schlaufen sind zwei Blöcke 140 x 120 x 75 mm, damit die Höhe von 3,12 m bleibt; jeder Bock ist ein volles A auf seinem Holm, First und Streben mit 80 mm bleiben."),
          T("Thermal: the plate carries the weather face, the surface zones keep their 2 mm gauge. Structure: the 140 mm leaf as its own plate inside the merged body (the body is not the member), loop blocks, tilt joint, rack and bearers. Minimal: no further merge keeps the opening, the loops and the rack apart.", "Thermik: die Platte trägt die Wetterseite, die Oberflächenzonen behalten ihre 2 mm. Tragwerk: die 140-mm-Tragschicht als eigene Platte im zusammengefassten Körper (der Körper ist nicht der Stab), Schlaufenblöcke, Kippgelenk, Bock und Kanthölzer. Minimal: keine weitere Zusammenfassung hält Öffnung, Schlaufen und Bock auseinander.")],
}


def volume(sc):
    """🧊️ The authored volume of one scale."""
    body = rack_solids(sc) + bearer_solids() + panel_solids(sc)
    return {"status": "authored", "denominator": sc, "body": body, "thermal": thermal_solids(sc, body), "structure": structure_solids(sc, body), "decisions": VOLUME_DECISIONS[sc]}


REP_DECISIONS = {
    50: {
        "silhouette": T("Side elevation of the exposed-aggregate face 6.00 x 2.80 m (120 x 56 mm on paper) leaning 3.5 degrees in the rack, with the 2.40 x 1.60 m window opening at 0.80 m, through which the ridge tube of the rack shows, the two lifting loops on the top edge, three cut connection bars at each sawn edge, the bearers and runners below. Front elevation: the sawn end face with its three layers 60/60/140 mm leaning against the A of the near trestle. Plan: the leaning panel with its sawn top edge in three bands over the runners, legs, ridge and base ties.", "Seitenansicht der Waschbetonfläche 6,00 x 2,80 m (120 x 56 mm auf dem Papier) mit 3,5 Grad Neigung im Bock, mit der Fensteröffnung 2,40 x 1,60 m bei 0,80 m, durch die das Firstrohr des Bocks zu sehen ist, den beiden Transportschlaufen an der Oberkante, drei abgetrennten Anschlussstäben je Sägekante, darunter Kanthölzer und Holme. Vorderansicht: die gesägte Stirnfläche mit ihren drei Schichten 60/60/140 mm, an das A des vorderen Bocks gelehnt. Draufsicht: die geneigte Platte mit ihrer gesägten Oberkante in drei Bändern über Holmen, Schenkeln, First und Fußstreben."),
        "retained": T("The chipped bottom corner and the spall at the top edge of the shell (the leaf behind shows through), the reveal arris, all four sawn edges, sealant residue at the old vertical joints, two drill holes of a removed downpipe bracket, the cut bar ends and the shell anchors in the end face, the grain of the bearers.", "Die abgeplatzte untere Ecke und die Abplatzung an der Oberkante der Schale (die Tragschicht dahinter scheint durch), die Leibungskante, alle vier Sägekanten, Dichtstoffreste an den alten senkrechten Fugen, zwei Bohrlöcher einer entfernten Fallrohrschelle, die abgetrennten Stabenden und die Schalenanker in der Stirnfläche, die Maserung der Kanthölzer."),
        "merged": T("The room face is drawn as the hidden back cell of the panel; the rack legs behind the panel are drawn and covered by it, so they show when the panel is lifted.", "Die Raumseite ist als verdeckte Rückzelle der Platte gezeichnet; die Bockschenkel hinter der Platte sind gezeichnet und von ihr verdeckt, damit sie beim Anheben erscheinen."),
        "omitted": T("Individual aggregate grains (hatch instead), the 2 mm surface zones of the thermal model, the reinforcement mesh inside the leaf.", "Einzelne Kieskörner (stattdessen Schraffur), die 2-mm-Oberflächenzonen des Thermikmodells, das Bewehrungsnetz in der Tragschicht."),
        "thinElements": T("Loops (20 mm bar, 0.4 mm) and stubs (16 mm, 0.32 mm) are filled shapes at their true size; the strokes along the edges sit half a pen inside the body.", "Schlaufen (20-mm-Stab, 0,4 mm) und Stummel (16 mm, 0,32 mm) sind gefüllte Formen in wahrer Größe; die Striche entlang der Kanten liegen eine halbe Feder innerhalb des Körpers."),
        "hatching": T("Dots of 0.8 mm for the exposed aggregate, 45-degree lines of 0.5 mm for the sawn concrete of the end face, stipple of 0.4 mm for the polystyrene (DIN ISO 128-50).", "Punkte mit 0,8 mm für den Waschbeton, 45-Grad-Linien mit 0,5 mm für den gesägten Beton der Stirnfläche, Punktierung mit 0,4 mm für das Polystyrol (DIN ISO 128-50)."),
        "styles": T("Outline 0.35 mm, details 0.18 mm, texture 0.1 mm; steel dark, concrete mid grey, insulation light, timber warm grey.", "Kontur 0,35 mm, Details 0,18 mm, Textur 0,1 mm; Stahl dunkel, Beton mittelgrau, Dämmung hell, Holz warmgrau."),
        "overlays": T("Each thermal region has its own cell: weather face, room face (back cell), sawn edges and reveals (strokes), steel, rack and bearers.", "Jede thermische Region hat ihre eigene Zelle: Wetterseite, Raumseite (Rückzelle), Sägekanten und Leibungen (Striche), Stahl, Bock und Kanthölzer."),
    },
    100: {
        "silhouette": T("The panel face 60 x 28 mm on paper with the opening, the loops and the six stubs; the chipped corner as one chamfer. Front: the three layers of the sawn end face (0.6 / 0.6 / 1.4 mm) against one A-shaped trestle on its runner. Plan: leaning panel with one sawn top band, runners, trestles, ridge, ties.", "Die Plattenfläche 60 x 28 mm auf dem Papier mit der Öffnung, den Schlaufen und den sechs Stummeln; die abgeplatzte Ecke als eine Fase. Vorderansicht: die drei Schichten der gesägten Stirnfläche (0,6 / 0,6 / 1,4 mm) vor einem A-förmigen Bock auf seinem Holm. Draufsicht: geneigte Platte mit einem gesägten Oberkantenband, Holme, Böcke, First, Streben."),
        "retained": T("Opening, loops, stubs, the chipped corner, the reveal arris and the four sawn edges, the bearer grain.", "Öffnung, Schlaufen, Stummel, die abgeplatzte Ecke, die Leibungskante und die vier Sägekanten, die Maserung der Kanthölzer."),
        "merged": T("The two legs of a trestle are one A in the front view and one bar in the plan; the three top bands of the plan are one.", "Die beiden Schenkel eines Bocks sind in der Vorderansicht ein A und in der Draufsicht ein Balken; die drei Oberkantenbänder der Draufsicht sind eines."),
        "omitted": T("The spall at the top edge, sealant residue, bracket holes, bar ends and anchors of the end face.", "Die Abplatzung an der Oberkante, Dichtstoffreste, Schellenbohrungen, Stabenden und Anker der Stirnfläche."),
        "thinElements": T("Loops 0.2 mm and stubs 0.16 mm stay filled shapes at true size.", "Schlaufen 0,2 mm und Stummel 0,16 mm bleiben gefüllte Formen in wahrer Größe."),
        "hatching": T("Exposed aggregate as dots of 0.6 mm; the end face is shown by tone only.", "Waschbeton als Punkte mit 0,6 mm; die Stirnfläche nur durch Tonwert."),
        "styles": T("Outline 0.25 mm, details 0.13 mm, texture 0.1 mm.", "Kontur 0,25 mm, Details 0,13 mm, Textur 0,1 mm."),
        "overlays": T("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": T("The panel face 30 x 14 mm on paper as a rectangle with the opening and two loop blocks; each trestle seen from the side as one 100 mm post; front: three layers (0.3 mm each) against the A standing on its runner with the opening between the legs; plan: panel, top band, loop blocks, trestles, ridge, ties.", "Die Plattenfläche 30 x 14 mm auf dem Papier als Rechteck mit der Öffnung und zwei Schlaufenblöcken; jeder Bock von der Seite als ein 100-mm-Pfosten; Vorderansicht: drei Schichten (je 0,3 mm) vor dem A auf seinem Holm mit der Öffnung zwischen den Schenkeln; Draufsicht: Platte, Oberkantenband, Schlaufenblöcke, Böcke, First, Streben."),
        "retained": T("Opening, loops as blocks, the reveal arris and the two vertical sawn edges, the layers in the end face.", "Öffnung, Schlaufen als Blöcke, die Leibungskante und die beiden senkrechten Sägekanten, die Schichten in der Stirnfläche."),
        "merged": T("Runner and legs of each trestle are one shape in every view; loops are blocks of their outline.", "Holm und Schenkel jedes Bocks sind in jeder Ansicht eine Form; Schlaufen sind Blöcke ihres Umrisses."),
        "omitted": T("Chipped corner and spall (under 30 mm), stubs, top and bottom sawn-edge strokes, the hatch, the bearer grain.", "Abgeplatzte Ecke und Abplatzung (unter 30 mm), Stummel, Striche der oberen und unteren Sägekante, die Schraffur, die Maserung der Kanthölzer."),
        "thinElements": T("The 80 mm ridge and ties and the 100 mm bearers are 0.4 to 0.5 mm wide on paper and stay true.", "First und Streben mit 80 mm und die Kanthölzer mit 100 mm sind auf dem Papier 0,4 bis 0,5 mm breit und bleiben wahr."),
        "hatching": T("No hatch: the face reads by its tone.", "Keine Schraffur: die Fläche liest sich über ihren Tonwert."),
        "styles": T("Outline 0.18 mm, details 0.1 mm.", "Kontur 0,18 mm, Details 0,1 mm."),
        "overlays": T("Regions as at 1:50; the sawn edges are drawn by the two vertical strokes.", "Regionen wie bei 1:50; die Sägekanten zeichnen die beiden senkrechten Striche."),
    },
    500: {
        "silhouette": T("The panel 12 x 5.6 mm on paper as one face with the opening and two loop blocks over two trestle posts, the base tie and the bearers; front: the 0.52 mm end face as one strip against the solid A of the trestle; plan: the leaning panel over the trestles, ridge and ties.", "Die Platte 12 x 5,6 mm auf dem Papier als eine Fläche mit der Öffnung und zwei Schlaufenblöcken über zwei Bockpfosten, der Fußstrebe und den Kanthölzern; Vorderansicht: die 0,52-mm-Stirnfläche als ein Streifen vor dem vollen A des Bocks; Draufsicht: die geneigte Platte über Böcken, First und Streben."),
        "retained": T("The opening (4.8 x 3.2 mm), the loops, the rack posts, ridge, tie and bearers at true size (80 to 100 mm, at least 0.2 mm with the pen at 1:1000).", "Die Öffnung (4,8 x 3,2 mm), die Schlaufen, Bockpfosten, First, Strebe und Kanthölzer in wahrer Größe (80 bis 100 mm, mit der Feder bei 1:1000 mindestens 0,2 mm)."),
        "merged": T("Weather face, sawn edges, reveals and steel are one face cell; the three layers are one strip in the front view; the A of each trestle is filled.", "Wetterseite, Sägekanten, Leibungen und Stahl sind eine Flächenzelle; die drei Schichten sind in der Vorderansicht ein Streifen; das A jedes Bocks ist gefüllt."),
        "omitted": T("Every stroke, every trace of use and the stubs.", "Jeder Strich, jede Gebrauchsspur und die Stummel."),
        "thinElements": T("The loop blocks are 75 mm thick in front and plan, the smallest feature, instead of the 20 mm bar.", "Die Schlaufenblöcke sind in Vorderansicht und Draufsicht 75 mm dick, das kleinste Merkmal, statt des 20-mm-Stabs."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": T("Overlays colour the face cell with the area-weighted value of weather face, sawn edges, reveals and steel (labelled merged).", "Überlagerungen färben die Flächenzelle mit dem flächengewichteten Wert von Wetterseite, Sägekanten, Leibungen und Stahl (als zusammengefasst gekennzeichnet)."),
    },
}

RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def representation(sc):
    """🖨️ The authored representation of one scale."""
    lo, hi = RANGES[sc]
    return {"status": "authored", "denominator": sc, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[sc], "hatches": HATCHES[sc], "geometry": geometry(sc), "decisions": REP_DECISIONS[sc]}


def height_stored():
    """📏️ Height of the stored pose: the crown of the loops on the leaning panel."""
    a, b = LOOP_TOP, STUB_Z - Z_IN
    t = -math.radians(LEAN)
    return HB + a * math.cos(t) + b * math.sin(t)


ASSET = {
    "id": "reclaimed-concrete-wall-panel",
    "category": "components",
    "title": T("Reclaimed Concrete Wall Panel", "Gebrauchte Betonwandplatte"),
    "summary": T(f"A sawn-out WBS 70 three-layer outer wall panel of 6.00 x 2.80 m and 260 mm (140 mm load-bearing leaf, 60 mm polystyrene, 60 mm exposed-aggregate weather shell) with a 2.40 x 1.60 m window opening, {MASS / 1000:.2f} t, standing on two squared-timber bearers and leaning against an A-frame rack of a component exchange. Its use shows in the chipped corner and spalled top edge of the shell, the sawn edges with cut connection bars, sealant residue, bracket holes and the two cast-in lifting loops by which it is lifted and reinstalled as a facade panel under wind.", f"Eine herausgesägte dreischichtige WBS-70-Außenwandplatte von 6,00 x 2,80 m und 260 mm (140 mm Tragschicht, 60 mm Polystyrol, 60 mm Waschbeton-Wetterschale) mit einer Fensteröffnung 2,40 x 1,60 m, {MASS / 1000:.2f} t, auf zwei Kanthölzern stehend und an einen A-Bock einer Bauteilbörse gelehnt. Ihr Gebrauch zeigt sich an der abgeplatzten Ecke und der abgeplatzten Oberkante der Schale, den Sägekanten mit abgetrennten Anschlussstäben, Dichtstoffresten, Schellenbohrungen und den beiden einbetonierten Transportschlaufen, an denen sie gehoben und als Fassadenplatte unter Wind wieder eingebaut wird."),
    "tags": ["component", "reclaimed", "concrete", "precast", "sandwich-panel", "wall-panel", "wbs-70", "exposed-aggregate", "facade", "a-frame-rack"],
    "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T("WBS 70 outer wall panel on the 6.0 m grid and the 2.8 m storey: three layers 140/60/60 mm, opening 2.40 x 1.60 m at a 0.80 m parapet with a 0.40 m lintel band, lifting loops at the quarter points; layer thicknesses, opening and loop positions are authored estimates of the typical panel guided by the WBS 70 literature, nothing is measured.", "WBS-70-Außenwandplatte im 6,0-m-Raster und der 2,8-m-Geschosshöhe: drei Schichten 140/60/60 mm, Öffnung 2,40 x 1,60 m über einer 0,80-m-Brüstung mit einem 0,40-m-Sturzband, Transportschlaufen in den Viertelspunkten; Schichtdicken, Öffnung und Schlaufenlagen sind Schätzwerte der typischen Platte nach der WBS-70-Literatur, nichts ist gemessen.")},
    "dimensions": {"heightM": r(height_stored(), 3), "widthM": r(L + 2 * STUB_LEN, 3), "depthM": r(RUN_Z1 - RUN_Z0, 3), "massKg": r(MASS, 2)},
    "materials": MATERIALS,
    "rig": {"kind": "structural", "root": "rack", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {str(sc): representation(sc) for sc in (50, 100, 200, 500)},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {str(sc): volume(sc) for sc in (50, 100, 200, 500)},
    "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

if __name__ == "__main__":
    with open(OUT, "w", encoding="utf-8") as handle:
        json.dump({"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print(f"[DEBUG] wrote {OUT}: mass {MASS:.1f} kg, com ({X_COM:.4f}, {Y_COM:.4f}, {Z_COM:.4f}), hang {HANG:.3f} deg, height {height_stored():.3f} m")
