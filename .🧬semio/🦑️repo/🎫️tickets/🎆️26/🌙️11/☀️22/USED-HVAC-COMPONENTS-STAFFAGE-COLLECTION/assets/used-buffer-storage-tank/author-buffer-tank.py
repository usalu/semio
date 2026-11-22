"""🫙 Authoritative generator of the staffage asset `used-buffer-storage-tank`.

A used 800 l steel heating buffer tank, Ø 790 mm without insulation and 1.80 m tall on three tubular feet, drained, its eight
6/4" sleeves closed with galvanised plugs; the soft insulation jacket was taken off and lies rolled and strapped beside it.
Stored lying on two squared bearers with nailed chocks, lifted on two round slings, reinstalled upright and charged stratified.
Every hand-chosen number is stated here; running the script rewrites `🔣️.json` next to it.

@see ../../brief.md
@see ../used-steel-oil-storage-tanks/author-tanks.py
"""

import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
SLUG = "used-buffer-storage-tank"

R, T_SHELL = 0.395, 0.003
U_APEX0, U_CYL0, U_CYL1, U_APEX1 = 0.10, 0.25, 1.65, 1.80
HEAD_H = U_CYL0 - U_APEX0
ZONES = [("zone-bottom", 0.10, 0.55, 0.95), ("zone-lower", 0.55, 0.95, 0.75), ("zone-upper", 0.95, 1.35, 0.25), ("zone-top", 1.35, 1.80, 0.05)]
SLEEVE_U = [0.30, 0.70, 1.15, 1.55]
SLEEVE_R, SLEEVE_W0, SLEEVE_W1 = 0.02415, 0.38, 0.435
PLUG_R, PLUG_W1 = 0.0275, 0.447
STUD_R, STUD_W0 = 0.038, 0.371
LEG_R, LEG_TOP, PLATE_R, PLATE_T = 0.03015, 0.157, 0.04, 0.006
LEGS = [(-0.28, 0.0), (0.14, 0.242487), (0.14, -0.242487)]
STAND = [(0.0, 0.242), (LEG_TOP, 0.242), (LEG_TOP, 0.318), (0.0, 0.318), (0.0, 0.242)]
BEAR_Y, SINK = 0.10, 0.002
AX_Y = BEAR_Y + R - SINK
BEARERS = {"bearer-a": 0.55, "bearer-b": 1.35}
BEARER_SIZE, BEARER_LEN = 0.10, 0.80
CHOCK_H = 0.06
CHOCK_W0 = round(math.sqrt(R ** 2 - (R - CHOCK_H - SINK) ** 2) - 0.001, 4)
CHOCK_W1 = CHOCK_W0 + 0.12
ROLL_X, ROLL_Z, ROLL_L, ROLL_RI, ROLL_RO = 0.075, 0.80, 1.65, 0.08, 0.29
STRAPS_U = [0.35, 1.30]
STRAP_W, STRAP_IN, STRAP_OUT = 0.05, 0.004, 0.004
LIFT_DY = 0.40
RHO_STEEL, RHO_SPRUCE, RHO_FLEECE = 7850.0, 450.0, 16.0
LEG_KG_M = 4.08
SLEEVE_KG, PLUG_KG = 0.30, 0.22
SKIN_KG_M2, ZIP_KG = 0.30, 0.30
WATER_RHO = 985.0
G = 9.81


def t(en, de):
    """📝 One bilingual text."""
    return {"en": en, "de": de}


def dez(x):
    """🇩🇪 A number with a decimal comma."""
    return str(x).replace(".", ",")


def r6(x):
    """🎯 Round to micrometres."""
    return round(x, 6)


def profile(n):
    """🥚 The outer half profile (u, r) of the vessel from apex to apex: elliptical heads of depth 150 mm and the cylinder."""
    pts = [(U_APEX0, 0.0)]
    for k in range(1, n + 1):
        a = math.pi / 2 * k / n
        pts.append((U_APEX0 + HEAD_H * (1 - math.cos(a)), R * math.sin(a)))
    for k in range(n, -1, -1):
        a = math.pi / 2 * k / n
        pts.append((U_APEX1 - HEAD_H * (1 - math.cos(a)), R * math.sin(a)))
    return [(r6(u), r6(r)) for u, r in pts]


def r_at(u):
    """📏 The outer radius of the vessel at axial position u."""
    if u <= U_APEX0 or u >= U_APEX1:
        return 0.0
    if u < U_CYL0:
        c = 1 - (u - U_APEX0) / HEAD_H
        return R * math.sqrt(max(0.0, 1 - c * c))
    if u > U_CYL1:
        c = 1 - (U_APEX1 - u) / HEAD_H
        return R * math.sqrt(max(0.0, 1 - c * c))
    return R


def segment(n, u0, u1):
    """✂️ The closed half profile of the part of the vessel between u0 and u1."""
    inner = [p for p in profile(n) if u0 < p[0] < u1]
    pts = [(u0, 0.0), (u0, r_at(u0))] + inner + [(u1, r_at(u1)), (u1, 0.0)]
    out = []
    for p in pts:
        q = (r6(p[0]), r6(p[1]))
        if not out or out[-1] != q:
            out.append(q)
    return out


def wall(n, u0, u1):
    """🧱 The closed half profile of the 3 mm wall of the vessel between u0 and u1 (outer face out, inner face back)."""
    outer = [p for p in segment(n, u0, u1) if p[1] > 0 or p[0] == U_APEX0 or p[0] == U_APEX1]
    if outer[0][1] == 0 and outer[0][0] != U_APEX0:
        outer = outer[1:]
    if outer[-1][1] == 0 and outer[-1][0] != U_APEX1:
        outer = outer[:-1]
    inner = []
    for u, r in reversed(outer):
        if u < U_CYL0:
            a = math.asin(min(1.0, r / R))
            inner.append((U_APEX0 + T_SHELL + (HEAD_H - T_SHELL) * (1 - math.cos(a)), (R - T_SHELL) * math.sin(a)))
        elif u > U_CYL1:
            a = math.asin(min(1.0, r / R))
            inner.append((U_APEX1 - T_SHELL - (HEAD_H - T_SHELL) * (1 - math.cos(a)), (R - T_SHELL) * math.sin(a)))
        else:
            inner.append((u, R - T_SHELL))
    pts = outer + inner + [outer[0]]
    return [(r6(u), r6(r)) for u, r in pts]


def rev_area(pts):
    """🌀 Lateral area of a surface of revolution of a polyline (u, r)."""
    return sum(2 * math.pi * (a[1] + b[1]) / 2 * math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(pts, pts[1:]))


FINE = profile(48)
SHELL_AREA = rev_area(FINE)
ZONE_AREA = {}
for zid, u0, u1, _ in ZONES:
    ZONE_AREA[zid] = round(rev_area([p for p in segment(48, u0, u1) if p[1] > 0]), 3)
INNER_R = R - T_SHELL
VOLUME_L = round(math.pi * INNER_R ** 2 * (U_CYL1 - U_CYL0) * 1000 + 2 * (2 / 3) * math.pi * INNER_R ** 2 * (HEAD_H - T_SHELL) * 1000)
SHELL_KG = SHELL_AREA * T_SHELL * RHO_STEEL
LEG_KG = LEG_KG_M * (LEG_TOP - PLATE_T) + math.pi * PLATE_R ** 2 * PLATE_T * RHO_STEEL
TANK_KG = round(SHELL_KG + 3 * LEG_KG + 8 * (SLEEVE_KG + PLUG_KG), 2)
CHOCK_KG = 0.5 * CHOCK_H * (CHOCK_W1 - CHOCK_W0) * BEARER_SIZE * RHO_SPRUCE
BEARER_KG = round(BEARER_SIZE * BEARER_SIZE * BEARER_LEN * RHO_SPRUCE + 2 * CHOCK_KG, 2)
JACKET_AREA = math.pi * (2 * R + 0.10) * ROLL_L
ROLL_KG = round(JACKET_AREA * 0.10 * RHO_FLEECE + math.pi * (2 * R + 0.2) * ROLL_L * SKIN_KG_M2 + ZIP_KG, 2)
MASS = round(TANK_KG + 2 * BEARER_KG + ROLL_KG, 2)
WATER_KG = round(VOLUME_L / 1000 * WATER_RHO, 1)
HEIGHT = r6(AX_Y + R)
WIDTH = r6(U_APEX1)
Z_MIN, Z_MAX = -PLUG_W1, ROLL_Z + ROLL_RO
DEPTH = r6(Z_MAX - Z_MIN)

SLEEVE_AREA = round(8 * (2 * math.pi * SLEEVE_R * (SLEEVE_W1 - 0.3943) + 2 * math.pi * PLUG_R * (PLUG_W1 - SLEEVE_W1) + math.pi * PLUG_R ** 2 + math.pi * (PLUG_R ** 2 - SLEEVE_R ** 2)), 3)
LEG_AREA = round(3 * (2 * math.pi * LEG_R * (U_APEX0 + 0.045 - PLATE_T) + math.pi * (PLATE_R ** 2 - LEG_R ** 2) + 2 * math.pi * PLATE_R * PLATE_T + math.pi * PLATE_R ** 2), 3)
CHOCK_NET = 2 * 0.5 * CHOCK_H * (CHOCK_W1 - CHOCK_W0) + CHOCK_H * BEARER_SIZE + math.hypot(CHOCK_H, CHOCK_W1 - CHOCK_W0) * BEARER_SIZE - (CHOCK_W1 - CHOCK_W0) * BEARER_SIZE
BEARER_AREA = round(2 * (2 * (BEARER_SIZE * BEARER_LEN * 2 + BEARER_SIZE * BEARER_SIZE) + 2 * CHOCK_NET), 3)
ROLL_AREA = round(2 * math.pi * (ROLL_RO + ROLL_RI) * ROLL_L + 2 * math.pi * (ROLL_RO ** 2 - ROLL_RI ** 2), 3)


def solid_body(id_, bone, material, tone, kind, **kw):
    """🧊 A body solid."""
    s = {"id": id_, "kind": kind, "bone": bone, "material": material, "role": kw.pop("role", "body"), "tone": tone}
    s.update(kw)
    return s


def boxs(id_, bone, material, tone, size, at, **kw):
    """📦 A box solid centred at `at`."""
    return solid_body(id_, bone, material, tone, "box", size=[r6(x) for x in size], transform={"translation": [r6(x) for x in at]}, **kw)


def cyl(id_, bone, material, tone, axis, origin, a0, a1, radius, **kw):
    """🥫 A cylinder as an open two-point revolve along `axis` from `a0` to `a1`."""
    return solid_body(id_, bone, material, tone, "revolve", profile=[[r6(a0), r6(radius)], [r6(a1), r6(radius)]], axis=axis, origin=[r6(x) for x in origin], **kw)


def revolve(id_, bone, material, tone, pts, **kw):
    """🏺 A revolve about the bone axis u through the bone origin."""
    return solid_body(id_, bone, material, tone, "revolve", profile=[[r6(a), r6(b)] for a, b in pts], axis=[1, 0, 0], origin=[0, 0, 0], **kw)


def chock_profile(side):
    """🔺 The triangular section (y, z) of a nailed chock on the given side."""
    if side > 0:
        return [[BEARER_SIZE, CHOCK_W0], [BEARER_SIZE, CHOCK_W1], [BEARER_SIZE + CHOCK_H, CHOCK_W0]]
    return [[BEARER_SIZE, -CHOCK_W1], [BEARER_SIZE, -CHOCK_W0], [BEARER_SIZE + CHOCK_H, -CHOCK_W0]]


def shape(id_, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, solid=None, pen=None):
    """✏️ One 2D shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    s.update(geom)
    if isinstance(solid, tuple):
        s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r6(solid[0]), "z1": r6(solid[1])}
    elif isinstance(solid, dict):
        s["solid"] = solid
    if pen is not None:
        s["penMm"] = pen
    return s


REVOLVED = {"kind": "revolved", "axisBone": "tank", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}


def rect(x, y, w, h, r=None):
    """▭ A rect geometry."""
    g = {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}
    if r:
        g["r"] = r
    return g


def circ(cx, cy, rad):
    """⚪ An ellipse geometry of equal radii."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rad), "ry": r6(rad)}


def poly(points, closed=False):
    """〰️ A polyline geometry."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in points], "closed": closed}


def ring(cx, cy, ro, ri, n=32):
    """⭕ A closed keyhole polygon of a ring (outer counter-clockwise, inner clockwise), filled by the non-zero rule."""
    outer = [(cx + ro * math.cos(2 * math.pi * k / n), cy + ro * math.sin(2 * math.pi * k / n)) for k in range(n + 1)]
    inner = [(cx + ri * math.cos(-2 * math.pi * k / n), cy + ri * math.sin(-2 * math.pi * k / n)) for k in range(n + 1)]
    return poly(outer + inner, True)


MATERIALS = [
    {"id": "primed-steel", "title": t("Steel S235JR with red-brown primer, weathered and scuffed", "Stahl S235JR mit rotbrauner Grundierung, gealtert und verschrammt"), "emissivity": 0.9, "solarAbsorptivity": 0.72, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-10025-2"},
    {"id": "galvanised-iron", "title": t("Galvanised malleable iron plug EN 10242, dull", "Verzinkter Temperguss-Stopfen EN 10242, matt"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7300, "conductivityWmK": 45, "specificHeatJKgK": 480, "elasticModulusPa": 170e9, "strengthPa": 200e6, "source": "literature", "reference": "en-10242"},
    {"id": "fleece-jacket", "title": t("Soft polyester fleece insulation 100 mm with grey polystyrene skin, rolled", "Weichschaum-Vlies-Dämmung 100 mm mit grauer Polystyrol-Haut, gerollt"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": RHO_FLEECE, "conductivityWmK": 0.04, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "buffer-tank-datasheets"},
    {"id": "polyester-strap", "title": t("Polyester lashing strap 50 mm", "Polyester-Spanngurt 50 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 1380, "conductivityWmK": 0.15, "specificHeatJKgK": 1200, "source": "literature", "reference": "en-iso-10456"},
    {"id": "spruce-weathered", "title": t("Squared spruce C24, weathered grey", "Kantholz Fichte C24, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_SPRUCE, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "en-338"},
]

REFERENCES = [
    {"id": "buffer-tank-datasheets", "citation": "Manufacturer data sheets of 800 l steel heating buffer tanks without heat exchanger (class of the Austria Email PSM, Sunex PSB, Wolf BSP, Reflex Storatherm H): typical diameter 790 mm without and 990 mm with a 100 mm soft fleece jacket, height 1790 to 1820 mm, empty mass 105 to 140 kg, eight 6/4 inch sleeves in two vertical rows, sensor sleeves 1/2 inch, test pressure 4.5 bar, operating pressure 3 bar, maximum 95 degC; values here are typical of the class, not of one product (authored-estimate)."},
    {"id": "en-12828", "citation": "EN 12828:2012+A1:2014 Heating systems in buildings, design for water-based heating systems (buffer storage, hydraulic connections, flow and return temperatures of the heat generator circuit)."},
    {"id": "en-12977-3", "citation": "EN 12977-3:2018 Thermal solar systems and components, custom built systems, part 3: performance test methods for solar water heater stores (thermal stratification, store heat loss, charge and discharge tests)."},
    {"id": "geg-2024", "citation": "Gebäudeenergiegesetz (GEG) 2024, paragraph 69 and annex 8: heat distribution and hot water pipework and fittings must be insulated; a buffer store is refitted with its jacket after commissioning."},
    {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy structural steels (S235JR: yield strength 235 MPa for thicknesses up to 16 mm)."},
    {"id": "iso-7-1", "citation": "ISO 7-1:1994 Pipe threads where pressure-tight joints are made on the threads (Rp 1 1/2: major diameter 47.8 mm; welded sleeve 6/4 inch with outer diameter about 48 mm)."},
    {"id": "en-10242", "citation": "EN 10242:1994+A2:2003 Threaded pipe fittings in malleable cast iron (plug with hexagon head, symbol T9 / 291, size 1 1/2 inch, about 0.22 kg)."},
    {"id": "en-10255", "citation": "EN 10255:2004 Non-alloy steel tubes suitable for welding and threading (DN 50 light series 60.3 x 2.9 mm, 4.08 kg/m, used for the three feet)."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes (C24: f_m,k 24 MPa, E0,mean 11 GPa, rho_mean 420 kg/m3; 450 kg/m3 taken for weathered timber at about 18 % moisture)."},
    {"id": "en-1492-2", "citation": "EN 1492-2:2000+A1:2008 Textile slings, round slings made of man-made fibres for general purpose use (two round slings in basket hitch for lifting a lying vessel)."},
    {"id": "vdi-2035", "citation": "VDI 2035 part 1 and 2 (2021) Prevention of damage in water heating installations: scale formation and water-side corrosion (the limescale and rust tones at the sleeves and the drain sleeve)."},
    {"id": "iapws-if97", "citation": "IAPWS-IF97 (2007) Industrial formulation for the thermodynamic properties of water and steam (density of water at 35 degC 994 kg/m3, at 70 degC 978 kg/m3; 985 kg/m3 taken for the stratified filling)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (design conductivity and heat capacity: steel 50 W/mK, 450 J/kgK; softwood 0.13 W/mK, 1600 J/kgK; polyester 0.15 W/mK)."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics (periodic penetration depth; about half of it taken as the effective storage depth of an exposed timber face and of the fleece roll)."},
    {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed. (2018), chapter 9: free convection of water on the inside of a vertical wall, h of 300 to 1000 W/m2K (film resistance 0.0015 m2K/W taken)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, Table 5 emissivities and solar absorptances: red oxide primer 0.9 and about 0.7; oxidised galvanised steel about 0.28; weathered values differ from new metal (bright galvanised 0.05 to 0.1)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types, contents and basic principles of presentation (scales, line groups)."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on cuts and sections."},
]

TANK_SECTION = {"areaM2": r6(math.pi * (2 * R - T_SHELL) * T_SHELL), "secondMomentM4": r6(math.pi / 8 * (2 * R - T_SHELL) ** 3 * T_SHELL), "extremeFibreM": R, "material": "primed-steel", "title": t("Steel shell as thin-walled tube 790 x 3 mm", "Stahlmantel als dünnwandiges Rohr 790 x 3 mm")}
BEARER_SECTION = {"areaM2": 0.01, "secondMomentM4": 8.333e-06, "extremeFibreM": 0.05, "material": "spruce-weathered", "title": t("Squared timber C24 100 x 100 mm", "Kantholz C24 100 x 100 mm")}
BONES = [
    {"id": "yard", "parent": None, "title": t("Yard ground (frame of the stored tank)", "Lagerplatz (Bezugsrahmen des gelagerten Speichers)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "bearer-a", "parent": "yard", "title": t("Bearer with two chocks under the lower end", "Kantholz mit zwei Keilen unter dem unteren Ende"), "x": BEARERS["bearer-a"], "y": 0, "angleDeg": 0, "lengthM": BEARER_SIZE, "joint": {"type": "fixed"}, "mass": {"kg": BEARER_KG, "comX": 0.05, "comY": 0.055, "distribution": "point", "source": "literature", "reference": "en-338"}, "section": BEARER_SECTION},
    {"id": "bearer-b", "parent": "yard", "title": t("Bearer with two chocks under the upper end", "Kantholz mit zwei Keilen unter dem oberen Ende"), "x": BEARERS["bearer-b"], "y": 0, "angleDeg": 0, "lengthM": BEARER_SIZE, "joint": {"type": "fixed"}, "mass": {"kg": BEARER_KG, "comX": 0.05, "comY": 0.055, "distribution": "point", "source": "literature", "reference": "en-338"}, "section": BEARER_SECTION},
    {"id": "jacket-roll", "parent": "yard", "title": t("Removed insulation jacket, rolled and strapped beside the tank", "Abgenommener Dämmmantel, gerollt und gegurtet neben dem Speicher"), "x": ROLL_X, "y": ROLL_RO, "z": ROLL_Z, "angleDeg": 0, "lengthM": ROLL_L, "joint": {"type": "fixed"}, "mass": {"kg": ROLL_KG, "comX": ROLL_L / 2, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "buffer-tank-datasheets"}},
    {"id": "carrier", "parent": "yard", "title": t("Lift carrier (massless, the crane hook raising the slings)", "Hubträger (masselos, der Kranhaken, der die Rundschlingen hebt)"), "x": 0, "y": AX_Y, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.6, "maxM": 0.6}},
    {"id": "tank", "parent": "carrier", "title": t("Buffer tank (axis from the feet to the top head; pivot at the feet end tilts it upright)", "Pufferspeicher (Achse von den Füßen zum oberen Boden; der Drehpunkt am Fußende richtet ihn auf)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": U_APEX1, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 90}, "mass": {"kg": TANK_KG, "comX": 0.93, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "buffer-tank-datasheets"}, "section": TANK_SECTION},
]


def anchor(id_, bone, x, y, kind, en, de, z=None, **kw):
    """📍 One anchor."""
    a = {"id": id_, "bone": bone, "x": r6(x), "y": r6(y)}
    if z is not None:
        a["z"] = r6(z)
    a.update({"kind": kind, "title": t(en, de)})
    a.update(kw)
    return a


SUP = {"support": "contact", "normalDeg": 90, "frictionCoefficient": 0.5}
ANCHORS = []
for bid, nm_en, nm_de in [("bearer-a", "lower-end bearer", "Kantholz am unteren Ende"), ("bearer-b", "upper-end bearer", "Kantholz am oberen Ende")]:
    for side, z in (("l", 0.35), ("r", -0.35)):
        ANCHORS.append(anchor(f"{bid}-ground-{side}", bid, 0.05, 0, "support", f"Ground under the {nm_en}, {'left' if side == 'l' else 'right'}", f"Boden unter dem {nm_de}, {'links' if side == 'l' else 'rechts'}", z, footprintLengthM=0.1, footprintWidthM=0.1, **SUP))
for sid, u in (("sling-a", 0.55), ("sling-b", 1.45)):
    ANCHORS.append(anchor(sid, "tank", u, R, "support", f"Round sling {sid[-1].upper()} over the shell (basket hitch to the crane hook)", f"Rundschlinge {sid[-1].upper()} über dem Mantel (Korbanschlag zum Kranhaken)", 0, footprintLengthM=0.05, footprintWidthM=0.3, **SUP))
for i, (v, w) in enumerate(LEGS, 1):
    ANCHORS.append(anchor(f"foot-{i}", "tank", 0, v, "support", f"Foot plate {i} on the boiler-room floor", f"Fußplatte {i} auf dem Heizraumboden", w, footprintLengthM=0.08, footprintWidthM=0.08, **SUP))
ANCHORS.append(anchor("water-centre", "tank", 0.95, 0, "load", "Centre of the water filling", "Schwerpunkt der Wasserfüllung"))
ANCHORS.append(anchor("tank-top", "tank", U_APEX1, 0, "emitter", "Crown of the top head (warm plume when charged)", "Scheitel des oberen Bodens (warme Fahne im geladenen Zustand)"))


def bodies(scale):
    """🧊 The body solids of one scale."""
    out = []
    n = {50: 8, 100: 6, 200: 4, 500: 3}[scale]
    for bid in BEARERS:
        out.append(boxs(f"{bid}-timber", bid, "spruce-weathered", 0.5, [BEARER_SIZE, BEARER_SIZE, BEARER_LEN], [0.05, 0.05, 0]))
        if scale != 500:
            for side, nm in ((1, "l"), (-1, "r")):
                out.append(solid_body(f"{bid}-chock-{nm}", bid, "spruce-weathered", 0.45, "extrude", profile=chock_profile(side), plane="yz", **{"from": 0, "to": BEARER_SIZE}))
    out.append(revolve("tank-shell", "tank", "primed-steel", 0.4, profile(n)))
    if scale == 500:
        out.append(revolve("stand", "tank", "primed-steel", 0.35, STAND))
    for i, (v, w) in enumerate(LEGS if scale != 500 else [], 1):
        out.append(cyl(f"leg-{i}", "tank", "primed-steel", 0.35, [1, 0, 0], [0, v, w], PLATE_T if scale == 50 else 0, LEG_TOP, LEG_R))
        if scale == 50:
            out.append(cyl(f"foot-plate-{i}", "tank", "primed-steel", 0.3, [1, 0, 0], [0, v, w], 0, PLATE_T, PLATE_R))
    for side, nm in ((1, "l"), (-1, "r")):
        for k, u in enumerate(SLEEVE_U, 1):
            if scale == 500:
                out.append(cyl(f"sleeve-{nm}{k}", "tank", "galvanised-iron", 0.5, [0, 0, 1], [u, 0, 0], STUD_W0 if side > 0 else -PLUG_W1, PLUG_W1 if side > 0 else -STUD_W0, STUD_R))
            elif scale == 50:
                out.append(cyl(f"sleeve-{nm}{k}", "tank", "primed-steel", 0.38, [0, 0, 1], [u, 0, 0], side * SLEEVE_W0 if side > 0 else -SLEEVE_W1, SLEEVE_W1 if side > 0 else -SLEEVE_W0, SLEEVE_R))
                out.append(cyl(f"plug-{nm}{k}", "tank", "galvanised-iron", 0.6, [0, 0, 1], [u, 0, 0], SLEEVE_W1 if side > 0 else -PLUG_W1, PLUG_W1 if side > 0 else -SLEEVE_W1, PLUG_R))
            else:
                out.append(cyl(f"sleeve-{nm}{k}", "tank", "galvanised-iron", 0.5, [0, 0, 1], [u, 0, 0], SLEEVE_W0 if side > 0 else -PLUG_W1, PLUG_W1 if side > 0 else -SLEEVE_W0, PLUG_R))
    out.append(revolve("jacket-roll", "jacket-roll", "fleece-jacket", 0.75, [(0, ROLL_RI), (ROLL_L, ROLL_RI), (ROLL_L, ROLL_RO), (0, ROLL_RO), (0, ROLL_RI)]))
    if scale == 50:
        for k, u in enumerate(STRAPS_U, 1):
            out.append(revolve(f"strap-{k}", "jacket-roll", "polyester-strap", 0.25, [(u, ROLL_RO - STRAP_IN), (u + STRAP_W, ROLL_RO - STRAP_IN), (u + STRAP_W, ROLL_RO + STRAP_OUT), (u, ROLL_RO + STRAP_OUT), (u, ROLL_RO - STRAP_IN)]))
    return out


def thermals(scale, body):
    """🌡️ The thermal solids of one scale."""
    out = []
    n = {50: 8, 100: 6, 200: 4, 500: 3}[scale]
    for zid, u0, u1, _ in ZONES:
        out.append(dict(revolve(f"th-{zid}", "tank", "primed-steel", 0.4, wall(n, u0, u1)), region=zid, layer="shell-steel"))
    for s in body:
        sid = s["id"]
        if sid.startswith(("sleeve", "plug")):
            out.append({"id": f"th-{sid}", "of": sid, "region": "sleeves"})
        elif sid.startswith(("leg", "foot", "stand")):
            out.append({"id": f"th-{sid}", "of": sid, "region": "feet"})
        elif s["bone"] in BEARERS:
            out.append({"id": f"th-{sid}", "of": sid, "region": "bearers"})
        elif s["bone"] == "jacket-roll":
            out.append({"id": f"th-{sid}", "of": sid, "region": "jacket-roll"})
    pr, pl = {50: (0.12, 0.03), 100: (0.12, 0.04), 200: (0.14, 0.06), 500: (0.15, 0.08)}[scale]
    out.append(dict(cyl("th-top-plume", "tank", "primed-steel", 0.6, [1, 0, 0], [0, 0, 0], U_APEX1 - pl / 2, U_APEX1 + pl / 2, pr, role="detail"), region="zone-top", tags=["top-plume"]))
    return out


def structures(scale, body):
    """🏗️ The structural solids of one scale."""
    out = []
    for s in body:
        sid = s["id"]
        if sid == "tank-shell" or sid.startswith(("leg", "stand")):
            out.append({"id": f"st-{sid}", "of": sid, "member": "tank", "part": "member"})
        elif sid.startswith(("sleeve", "foot")):
            out.append({"id": f"st-{sid}", "of": sid, "member": "tank", "part": "joint"})
        elif s["bone"] in BEARERS:
            out.append({"id": f"st-{sid}", "of": sid, "member": s["bone"], "part": "member" if "timber" in sid else "joint"})
        elif sid == "jacket-roll":
            out.append({"id": f"st-{sid}", "of": sid, "member": "jacket-roll", "part": "member"})
    return out


def tank_shapes(scale, view):
    """🫙 The tank in one view: four zones of the shell, three feet, eight sleeves (solids, so every pose reads right)."""
    n = {50: 8, 100: 6, 200: 4, 500: 3}[scale]
    out = []
    pen = None
    for k, (zid, u0, u1, _) in enumerate(ZONES):
        seg = segment(n, u0, u1)
        if view == "elevation":
            top = [p for p in seg if p[1] > 0 or p[0] in (U_APEX0, U_APEX1)]
            pts = top + [(u, -r) for u, r in reversed(top) if r > 0]
            out.append(shape(f"shell-{zid}", "tank", "body", 2 + 0.01 * k, True, 0.4, poly(pts, True), region=zid, material="primed-steel"))
        else:
            out.append(shape(f"shell-{zid}", "tank", "body", 2 + 0.01 * k, True, 0.4, poly(seg, True), region=zid, material="primed-steel", solid=REVOLVED))
    if scale == 500:
        ro = STAND[2][1]
        out.append(shape("stand", "tank", "body", 1.8, True, 0.35, rect(0, -ro, LEG_TOP, 2 * ro) if view == "elevation" else poly([(0, 0), (0, ro), (LEG_TOP, ro), (LEG_TOP, 0)], True), region="feet", material="primed-steel", solid=None if view == "elevation" else REVOLVED))
    lr = LEG_R
    for i, (v, w) in enumerate(LEGS if scale != 500 else [], 1):
        if view == "elevation" and i == 3:
            continue
        u0 = PLATE_T if scale == 50 else 0
        out.append(shape(f"leg-{i}", "tank", "body", 1.8, True, 0.35, rect(u0, v - lr, LEG_TOP - u0, 2 * lr), region="feet", material="primed-steel", solid=None if view == "elevation" else (w - lr, w + lr)))
        if scale == 50:
            out.append(shape(f"foot-plate-{i}", "tank", "body", 1.85, True, 0.3, rect(0, v - PLATE_R, PLATE_T, 2 * PLATE_R), region="feet", material="primed-steel", solid=None if view == "elevation" else (w - PLATE_R, w + PLATE_R)))
    for side, nm in ((1, "l"), (-1, "r")):
        for k, u in enumerate(SLEEVE_U, 1):
            if scale == 500:
                if view == "elevation":
                    if side < 0:
                        out.append(shape(f"sleeve-{nm}{k}", "tank", "body", 3, True, 0.5, circ(u, 0, STUD_R), region="sleeves", material="galvanised-iron"))
                    continue
                w0, w1 = (STUD_W0, PLUG_W1) if side > 0 else (-PLUG_W1, -STUD_W0)
                out.append(shape(f"sleeve-{nm}{k}", "tank", "body", 3, True, 0.5, rect(u - STUD_R, -STUD_R, 2 * STUD_R, 2 * STUD_R), region="sleeves", material="galvanised-iron", solid=(w0, w1)))
                continue
            if view == "elevation":
                if side > 0:
                    continue
                out.append(shape(f"plug-{nm}{k}" if scale == 50 else f"sleeve-{nm}{k}", "tank", "body", 3, True, 0.6, circ(u, 0, PLUG_R), region="sleeves", material="galvanised-iron"))
                if scale == 50:
                    out.append(shape(f"plug-{nm}{k}-socket", "tank", "detail", 3.1, False, 0.3, rect(u - 0.009, -0.009, 0.018, 0.018), region="sleeves"))
                continue
            if scale == 50:
                w0, w1 = (SLEEVE_W0, SLEEVE_W1) if side > 0 else (-SLEEVE_W1, -SLEEVE_W0)
                out.append(shape(f"sleeve-{nm}{k}", "tank", "body", 3, True, 0.38, rect(u - SLEEVE_R, -SLEEVE_R, 2 * SLEEVE_R, 2 * SLEEVE_R), region="sleeves", material="primed-steel", solid=(w0, w1)))
                p0, p1 = (SLEEVE_W1, PLUG_W1) if side > 0 else (-PLUG_W1, -SLEEVE_W1)
                out.append(shape(f"plug-{nm}{k}", "tank", "body", 3.1, True, 0.6, rect(u - PLUG_R, -PLUG_R, 2 * PLUG_R, 2 * PLUG_R), region="sleeves", material="galvanised-iron", solid=(p0, p1)))
            else:
                w0, w1 = (SLEEVE_W0, PLUG_W1) if side > 0 else (-PLUG_W1, -SLEEVE_W0)
                out.append(shape(f"sleeve-{nm}{k}", "tank", "body", 3, True, 0.5, rect(u - PLUG_R, -PLUG_R, 2 * PLUG_R, 2 * PLUG_R), region="sleeves", material="galvanised-iron", solid=(w0, w1)))
    return out


def tank_details(scale):
    """🔎 Traces of use and dismantling on the near (right) side of the shell in the side elevation."""
    out = []
    if scale in (50, 100):
        for k, u in enumerate((U_CYL0, U_CYL1), 1):
            out.append(shape(f"head-seam-{k}", "tank", "detail", 4, False, 0.3, poly([(u, -R), (u, R)]), region="zone-bottom" if k == 1 else "zone-top"))
        out.append(shape("type-plate", "tank", "detail", 4.2, True, 0.25, rect(1.30, 0.16, 0.12, 0.07), region="zone-upper"))
        for k, u in enumerate(SLEEVE_U, 1):
            pts = [(u - 0.004, -0.03), (u + 0.006, -0.11), (u - 0.003, -0.2), (u + 0.004, -0.3)]
            out.append(shape(f"rust-run-{k}", "tank", "texture", 4.1, False, 0.25, poly(pts if scale == 50 else [pts[0], pts[-1]]), region="zone-bottom" if u < 0.55 else "zone-lower" if u < 0.95 else "zone-upper" if u < 1.35 else "zone-top"))
    if scale == 50:
        out.append(shape("tape-remnant", "tank", "detail", 4.15, True, 0.7, poly([(0.62, 0.33), (1.02, 0.335), (1.02, 0.355), (0.62, 0.35)], True), region="zone-lower"))
        out.append(shape("glue-dabs", "tank", "texture", 4.1, False, 0.55, poly([(1.40, 0.30), (1.46, 0.31), (1.52, 0.295), (1.58, 0.305)]), region="zone-top"))
        out.append(shape("chalk-mark", "tank", "texture", 4.1, False, 0.9, poly([(0.85, -0.22), (0.95, -0.12), (0.9, -0.17), (0.85, -0.12), (0.95, -0.22)]), region="zone-lower"))
        out.append(shape("scale-crust", "tank", "texture", 4.1, False, 0.8, poly([(0.27, -0.05), (0.29, -0.07), (0.33, -0.07), (0.35, -0.05)]), region="zone-bottom"))
        out.append(shape("scuff-1", "tank", "texture", 4.1, False, 0.55, poly([(0.48, -0.36), (0.62, -0.372), (0.76, -0.365)]), region="zone-bottom"))
        out.append(shape("scuff-2", "tank", "texture", 4.1, False, 0.55, poly([(1.25, -0.37), (1.42, -0.378)]), region="zone-upper"))
    return out


def roll_elevation(scale):
    """🧻 The rolled jacket in the side elevation (behind the tank)."""
    out = [shape("jacket-roll", "jacket-roll", "body", 0.5, True, 0.75, rect(0, -ROLL_RO, ROLL_L, 2 * ROLL_RO), region="jacket-roll", material="fleece-jacket")]
    if scale == 50:
        for k, u in enumerate(STRAPS_U, 1):
            out.append(shape(f"strap-{k}", "jacket-roll", "body", 0.6, True, 0.25, rect(u, -ROLL_RO - STRAP_OUT, STRAP_W, 2 * (ROLL_RO + STRAP_OUT)), region="jacket-roll", material="polyester-strap"))
        out.append(shape("roll-zip", "jacket-roll", "texture", 0.7, False, 0.4, poly([(0.0, 0.12), (0.5, 0.125), (1.1, 0.118), (1.65, 0.122)]), region="jacket-roll"))
    return out


def elevation(scale):
    """📐 Side elevation (seen from the right): the lying tank on its bearers, the plugs of the near sleeve row, the roll behind."""
    out = roll_elevation(scale)
    for bid in BEARERS:
        out.append(shape(f"{bid}-timber", bid, "body", 1, True, 0.5, rect(0, 0, BEARER_SIZE, BEARER_SIZE), region="bearers", material="spruce-weathered"))
        if scale != 500:
            out.append(shape(f"{bid}-chock-r", bid, "body", 1.1, True, 0.45, rect(0, BEARER_SIZE, BEARER_SIZE, CHOCK_H), region="bearers", material="spruce-weathered"))
        if scale == 50:
            out.append(shape(f"{bid}-end-grain", bid, "texture", 1.2, False, 0.3, poly([(0.012, 0.0), (0.03, 0.028), (0.032, 0.06), (0.018, 0.1)]), region="bearers"))
            out.append(shape(f"{bid}-nail", bid, "detail", 1.3, True, 0.2, circ(0.05, 0.13, 0.006), region="bearers"))
    out += tank_shapes(scale, "elevation")
    out += tank_details(scale)
    return out


def front(scale):
    """🎯 Front elevation (seen from ahead, along the lying axis): the top head as a circle, sleeves both sides, bearers, roll."""
    out = []
    for k, bid in enumerate(BEARERS):
        out.append(shape(f"{bid}-timber", bid, "body", 1 + k * 0.1, True, 0.5, rect(-BEARER_LEN / 2, 0, BEARER_LEN, BEARER_SIZE), region="bearers", material="spruce-weathered"))
        if scale != 500:
            for side, nm in ((1, "l"), (-1, "r")):
                pr = chock_profile(side)
                out.append(shape(f"{bid}-chock-{nm}", bid, "body", 1.2 + k * 0.1, True, 0.45, poly([(p[1], p[0]) for p in pr], True), region="bearers", material="spruce-weathered"))
    out.append(shape("jacket-roll", "jacket-roll", "body", 1.5, True, 0.75, ring(ROLL_Z, ROLL_RO, ROLL_RO, ROLL_RI, 32 if scale <= 100 else 20), region="jacket-roll", material="fleece-jacket"))
    if scale == 50:
        out.append(shape("roll-spiral", "jacket-roll", "texture", 1.6, False, 0.4, poly([(ROLL_Z + 0.08 * math.cos(a / 8) * (1 + a / 40), ROLL_RO + 0.08 * math.sin(a / 8) * (1 + a / 40)) for a in range(0, 100, 4)]), region="jacket-roll"))
    if scale == 50:
        out.append(shape("bearer-b-grain", "bearer-b", "texture", 1.3, False, 0.3, poly([(-0.4, 0.036), (-0.1, 0.04), (0.2, 0.034), (0.4, 0.038)]), region="bearers"))
    out += tank_shapes(scale, "front")
    return out


def plan(scale):
    """🗺️ Plan (seen from above): the lying tank with both sleeve rows, the bearer ends, the roll beside it."""
    out = []
    for bid, x in BEARERS.items():
        out.append(shape(f"{bid}-timber", bid, "body", 1, True, 0.5, rect(x, -BEARER_LEN / 2, BEARER_SIZE, BEARER_LEN), region="bearers", material="spruce-weathered"))
    out.append(shape("jacket-roll", "jacket-roll", "body", 1.5, True, 0.75, rect(ROLL_X, ROLL_Z - ROLL_RO, ROLL_L, 2 * ROLL_RO), region="jacket-roll", material="fleece-jacket"))
    if scale == 50:
        for k, u in enumerate(STRAPS_U, 1):
            out.append(shape(f"strap-{k}", "jacket-roll", "body", 1.6, True, 0.25, rect(ROLL_X + u, ROLL_Z - ROLL_RO - STRAP_OUT, STRAP_W, 2 * (ROLL_RO + STRAP_OUT)), region="jacket-roll", material="polyester-strap"))
    out += tank_shapes(scale, "plan")
    return out


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DECISIONS = {
    50: {
        "silhouette": t("Side elevation from the right: the lying tank 1800 x 790 mm (36 x 15.8 mm on paper) with its elliptical heads, the three tubular feet with foot plates at the lower end, the four plugged sleeves of the near row, the two bearers end on with their chocks and the strapped jacket roll behind. Front view: the top head as a 790 mm circle with the plugged sleeves standing out 52 mm on both sides, the chocks and the roll as a ring with its open core. Plan: the tank outline, both sleeve rows, the bearers standing out beyond the shell and the roll beside it.", "Seitenansicht von rechts: der liegende Speicher 1800 x 790 mm (36 x 15,8 mm auf Papier) mit seinen elliptischen Böden, den drei Rohrfüßen mit Fußplatten am unteren Ende, den vier verschlossenen Muffen der nahen Reihe, den beiden Kanthölzern in Stirnansicht mit ihren Keilen und der gegurteten Dämmrolle dahinter. Vorderansicht: der obere Boden als Kreis von 790 mm mit den verschlossenen Muffen, die auf beiden Seiten 52 mm vorstehen, die Keile und die Rolle als Ring mit offenem Kern. Draufsicht: die Speicherkontur, beide Muffenreihen, die über den Mantel vorstehenden Kanthölzer und die Rolle daneben."),
        "retained": t("Plugs with their hexagon socket, the 6 mm foot plates, the two head welds, the type plate, rust runs below the sleeves, the scale crust at the lowest sleeve, a strip of jacket tape and glue dabs left on the shell, a chalk cross of the exchange, scuffs from dragging, the nails of the chocks, the two 50 mm straps and the zip of the rolled jacket.", "Stopfen mit Innensechskant, die Fußplatten 6 mm, die beiden Bodennähte, das Typenschild, Rostläufer unter den Muffen, die Kalkkruste an der untersten Muffe, ein Streifen Mantelklebeband und Klebepunkte auf dem Mantel, ein Kreidekreuz der Börse, Schleifspuren vom Ziehen, die Nägel der Keile, die beiden Gurte 50 mm und der Reißverschluss des gerollten Mantels."),
        "merged": t("The shell is drawn as four bands, the four thermal zones of the stratified store; sensor clamp rail and 1/2 inch sensor sleeves (21 mm) are part of the shell.", "Der Mantel ist als vier Bänder gezeichnet, die vier thermischen Zonen des geschichteten Speichers; Fühlerklemmleiste und Fühlermuffen 1/2 Zoll (21 mm) sind Teil des Mantels."),
        "omitted": t("The far sleeve row in the side elevation (hidden behind the shell), thread turns and the weld beads of the sleeves (below 7.5 mm).", "Die ferne Muffenreihe in der Seitenansicht (hinter dem Mantel verdeckt), Gewindegänge und Schweißraupen der Muffen (unter 7,5 mm)."),
        "thinElements": t("None below the 7.5 mm feature: plugs 55 mm, feet 60.3 mm, straps 50 mm wide.", "Keine unter der kleinsten Einzelheit von 7,5 mm: Stopfen 55 mm, Füße 60,3 mm, Gurte 50 mm breit."),
        "hatching": t("No hatch: a view of primed sheet steel, fleece and weathered timber; tone separates steel, galvanised plugs, fleece, straps and timber.", "Keine Schraffur: Ansicht von grundiertem Stahlblech, Vlies und vergrautem Holz; der Ton trennt Stahl, verzinkte Stopfen, Vlies, Gurte und Holz."),
        "styles": t("Outline 0.35 mm, seams, plate, tape and nails 0.18 mm, rust runs, chalk, scuffs and zip 0.1 mm.", "Kontur 0,35 mm, Nähte, Schild, Klebeband und Nägel 0,18 mm, Rostläufer, Kreide, Schleifspuren und Reißverschluss 0,1 mm."),
        "overlays": t("Each shell band is one cell of its zone region; sleeves and plugs are cells of the sleeve region, feet of the feet region, bearers and chocks of the bearer region, roll and straps of the jacket region.", "Jedes Mantelband ist eine Zelle seiner Zonenregion; Muffen und Stopfen sind Zellen der Muffenregion, die Füße der Fußregion, Kanthölzer und Keile der Kantholzregion, Rolle und Gurte der Mantelregion."),
    },
    100: {
        "silhouette": t("The same tank at 18 x 7.9 mm: heads, feet, the plugged sleeves as one turned part each, bearers with chocks and the roll; front and plan as at 1:50.", "Derselbe Speicher mit 18 x 7,9 mm: Böden, Füße, die verschlossenen Muffen als je ein gedrehtes Teil, Kanthölzer mit Keilen und die Rolle; Vorder- und Draufsicht wie 1:50."),
        "retained": t("Head welds, type plate and the rust runs below the sleeves.", "Bodennähte, Typenschild und die Rostläufer unter den Muffen."),
        "merged": t("Sleeve and plug are one part (55 mm across, 67 mm long); the 6 mm foot plates merge into the feet.", "Muffe und Stopfen sind ein Teil (55 mm Durchmesser, 67 mm lang); die Fußplatten 6 mm gehen in den Füßen auf."),
        "omitted": t("Straps, zip, tape, glue dabs, chalk mark, scuffs and the plug sockets (below the 15 mm feature or unreadable at 0.15 mm).", "Gurte, Reißverschluss, Klebeband, Klebepunkte, Kreidezeichen, Schleifspuren und die Stopfen-Innensechskante (unter der kleinsten Einzelheit von 15 mm oder bei 0,15 mm nicht lesbar)."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm, seams and plate 0.13 mm, rust runs 0.1 mm.", "Kontur 0,25 mm, Nähte und Schild 0,13 mm, Rostläufer 0,1 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    200: {
        "silhouette": t("The tank outline of 9 x 4 mm with its four zone bands, feet, the sleeves as 0.28 mm studs, bearers with chocks and the roll ring in the front view.", "Die Speicherkontur von 9 x 4 mm mit ihren vier Zonenbändern, Füße, die Muffen als Zapfen von 0,28 mm, Kanthölzer mit Keilen und der Rollenring in der Vorderansicht."),
        "retained": t("The eight sleeves standing out of the shell and the chocks that keep the cylinder from rolling.", "Die acht Muffen, die aus dem Mantel ragen, und die Keile, die den Zylinder am Rollen hindern."),
        "merged": t("Heads drawn with four facets per quarter.", "Böden mit vier Facetten je Viertel gezeichnet."),
        "omitted": t("All traces of use (below 30 mm or unreadable at 0.1 mm pen).", "Alle Gebrauchsspuren (unter 30 mm oder bei 0,1-mm-Feder nicht lesbar)."),
        "thinElements": t("None below the 30 mm feature.", "Keine unter der kleinsten Einzelheit von 30 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm.", "Kontur 0,18 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    500: {
        "silhouette": t("The tank as a 3.6 x 1.6 mm outline with three facets per head, the three feet merged into one ring stand 636 mm across, the eight sleeve studs thickened to 76 mm (0.15 mm), two bearers and the roll with its 160 mm core.", "Der Speicher als Kontur 3,6 x 1,6 mm mit drei Facetten je Boden, die drei Füße zu einem Ringfuß von 636 mm Durchmesser zusammengefasst, die acht Muffenzapfen auf 76 mm verdickt (0,15 mm), zwei Kanthölzer und die Rolle mit ihrem Kern von 160 mm."),
        "retained": t("Overall size of the stored set, the tank, its feet, the bearers and the roll beside it.", "Gesamtgröße des gelagerten Satzes, der Speicher, seine Füße, die Kanthölzer und die Rolle daneben."),
        "merged": t("Sleeve and plug are one stud of 76 mm, 52 mm out of the shell, so that the two connection rows still read.", "Muffe und Stopfen sind ein Zapfen von 76 mm, 52 mm aus dem Mantel, damit die beiden Anschlussreihen lesbar bleiben."),
        "omitted": t("Chocks (60 mm high) and every trace of use.", "Keile (60 mm hoch) und alle Gebrauchsspuren."),
        "thinElements": t("The three feet (60.3 mm, 166 mm apart in the front view of the upright tank, 0.04 mm of white at 1:1000 under the 0.13 mm pen) are one ring stand 76 mm wide through their centres on the 280 mm circle, so the upright tank keeps its stand without a gap that fills in; the sleeves are thickened from 55 to 76 mm.", "Die drei Füße (60,3 mm, in der Vorderansicht des aufrechten Speichers 166 mm auseinander, bei 1:1000 unter der Feder 0,13 mm nur 0,04 mm Weiß) sind ein Ringfuß von 76 mm Breite durch ihre Mitten auf dem Kreis von 280 mm, damit der aufrechte Speicher seinen Stand ohne zulaufende Lücke behält; die Muffen sind von 55 auf 76 mm verdickt."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
}

VOLUME_DECISIONS = {
    50: [
        t(f"Body: the shell is one turned solid of the true half profile, Ø 790 mm, cylinder 1400 mm between two elliptical heads 150 mm deep (eight facets per quarter), crown to crown {U_APEX1 - U_APEX0:.2f} m; three feet DN 50 (60.3 mm) with 80 mm foot plates at 120 degrees on a 280 mm circle; eight 6/4 inch sleeves (48.3 mm, 55 mm out of the shell) in two rows at 300, 700, 1150 and 1550 mm with galvanised hexagon plugs 55 x 12 mm; two bearers 100 x 100 x 800 mm with two nailed triangular chocks each touching the shell; the jacket roll Ø 580 mm with a 160 mm core, 1650 mm long, with two 50 mm straps pressed 4 mm into the fleece.", f"Körper: der Mantel ist ein gedrehter Körper des wahren Halbprofils, Ø 790 mm, Zylinder 1400 mm zwischen zwei elliptischen Böden von 150 mm Tiefe (acht Facetten je Viertel), Scheitel zu Scheitel {dez(f'{U_APEX1 - U_APEX0:.2f}')} m; drei Füße DN 50 (60,3 mm) mit Fußplatten 80 mm unter 120 Grad auf einem Kreis von 280 mm; acht Muffen 6/4 Zoll (48,3 mm, 55 mm aus dem Mantel) in zwei Reihen auf 300, 700, 1150 und 1550 mm mit verzinkten Sechskantstopfen 55 x 12 mm; zwei Kanthölzer 100 x 100 x 800 mm mit je zwei genagelten Dreieckkeilen, die den Mantel berühren; die Dämmrolle Ø 580 mm mit Kern 160 mm, 1650 mm lang, mit zwei Gurten 50 mm, 4 mm ins Vlies gedrückt."),
        t("Thermal: the shell as four turned zone solids cut at 550, 950 and 1350 mm along the axis (bottom, lower, upper and top zone of the stratified store), meeting on closed seams; sleeves, plugs, feet, bearers, chocks, roll and straps reuse their body solids; a 30 mm disc over the top crown carries the warm plume.", "Thermisch: der Mantel als vier gedrehte Zonenkörper, entlang der Achse bei 550, 950 und 1350 mm geschnitten (untere, mittlere untere, mittlere obere und obere Zone des geschichteten Speichers), die sich an geschlossenen Fugen treffen; Muffen, Stopfen, Füße, Kanthölzer, Keile, Rolle und Gurte verwenden ihre Körper; eine Scheibe von 30 mm über dem oberen Scheitel trägt die warme Fahne."),
        t("Structure: the shell and the three feet are the members of the tank bone, the sleeves its joints; the bearers are members of theirs with the chocks as joints; the roll is the member of its own bone and lies on the ground.", "Tragwerk: Mantel und die drei Füße sind die Stäbe des Speicherknochens, die Muffen seine Gelenke; die Kanthölzer sind Stäbe der ihren mit den Keilen als Gelenken; die Rolle ist der Stab ihres eigenen Knochens und liegt am Boden."),
    ],
    100: [
        t("Body: as 1:50 with six facets per head quarter; sleeve and plug are one cylinder Ø 55 mm from inside the shell to the plug face, the foot plates (6 mm) and the straps (4 mm proud) are below the 15 mm feature and left out.", "Körper: wie 1:50 mit sechs Facetten je Bodenviertel; Muffe und Stopfen sind ein Zylinder Ø 55 mm von innerhalb des Mantels bis zur Stopfenfläche, die Fußplatten (6 mm) und die Gurte (4 mm Überstand) liegen unter der kleinsten Einzelheit von 15 mm und entfallen."),
        t("Thermal: four zone solids as at 1:50; the other regions reuse their body solids.", "Thermisch: vier Zonenkörper wie 1:50; die anderen Regionen verwenden ihre Körper."),
        t("Structure: as 1:50 without the foot plates.", "Tragwerk: wie 1:50 ohne die Fußplatten."),
    ],
    200: [
        t("Body: four facets per head quarter; sleeves as 55 mm studs, chocks (60 mm) kept as they hold the cylinder.", "Körper: vier Facetten je Bodenviertel; Muffen als Zapfen 55 mm, die Keile (60 mm) bleiben, weil sie den Zylinder halten."),
        t("Thermal: as 1:100.", "Thermisch: wie 1:100."),
        t("Structure: as 1:100.", "Tragwerk: wie 1:100."),
    ],
    500: [
        t("Body: three facets per head quarter; the chocks (60 mm) are below the 75 mm feature and left out (the cylinder lies on the bearers alone); the three feet are one turned ring stand 76 mm wide and 157 mm high on their 280 mm circle and the eight sleeve studs are thickened to 76 mm.", "Körper: drei Facetten je Bodenviertel; die Keile (60 mm) liegen unter der kleinsten Einzelheit von 75 mm und entfallen (der Zylinder liegt allein auf den Kanthölzern); die drei Füße sind ein gedrehter Ringfuß von 76 mm Breite und 157 mm Höhe auf ihrem Kreis von 280 mm, und die acht Muffenzapfen sind auf 76 mm verdickt."),
        t("Thermal: four zone solids; sleeves, feet, bearers and roll reuse their body solids.", "Thermisch: vier Zonenkörper; Muffen, Füße, Kanthölzer und Rolle verwenden ihre Körper."),
        t("Structure: shell and ring stand as members, bearers and roll as members.", "Tragwerk: Mantel und Ringfuß als Stäbe, Kanthölzer und Rolle als Stäbe."),
    ],
}


def representation(scale):
    """🖨️ The authored representation of one scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": DECISIONS[scale]}


def volume(scale):
    """🧊 The authored volume of one scale."""
    body = bodies(scale)
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermals(scale, body), "structure": structures(scale, body), "decisions": VOLUME_DECISIONS[scale]}


ZONE_IDS = [z[0] for z in ZONES]


def bind(sun_shell, sun_low, exp_bear):
    """🌬️ Region bindings of one pose."""
    regions = {z: {"exposedFraction": 0.97, "sunlitFraction": sun_shell} for z in ZONE_IDS}
    regions["sleeves"] = {"exposedFraction": 1, "sunlitFraction": sun_shell}
    regions["feet"] = {"exposedFraction": 1, "sunlitFraction": sun_low}
    regions["bearers"] = {"exposedFraction": exp_bear, "sunlitFraction": sun_low, "contact": {"fraction": 0.12, "tempC": 10}}
    regions["jacket-roll"] = {"exposedFraction": 0.97, "sunlitFraction": sun_low, "contact": {"fraction": 0.03, "tempC": 10}}
    return regions


BEARER_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("bearer")]
SLING_CONTACTS = ["sling-a", "sling-b"]
FOOT_CONTACTS = ["foot-1", "foot-2", "foot-3"]
UPRIGHT = {"carrier": {"dy": r6(-AX_Y)}, "tank": {"angleDeg": 90}}
WATER_LOAD = [{"id": "water", "kind": "carried", "title": t(f"Heating water, {VOLUME_L} l, {WATER_KG * G / 1000:.2f} kN", f"Heizwasser, {VOLUME_L} l, {dez(f'{WATER_KG * G / 1000:.2f}')} kN"), "anchor": "water-centre", "fxN": 0, "fyN": r6(-WATER_KG * G), "source": "literature", "reference": "iapws-if97"}]

POSES = [
    {"id": "stored", "title": t("Stored lying on bearers", "Liegend auf Kanthölzern gelagert"), "description": t("Drained and plugged, the tank lies on two squared bearers with nailed chocks in the yard of the component exchange; its insulation jacket lies rolled and strapped beside it.", "Entleert und verschlossen liegt der Speicher auf zwei Kanthölzern mit genagelten Keilen auf dem Hof der Bauteilbörse; sein Dämmmantel liegt gerollt und gegurtet daneben."), "bones": {}, "ground": "authored", "contacts": BEARER_CONTACTS, "thermal": {"operating": {"charge": 0}, "regions": bind(0.35, 0.2, 0.6), "emitters": {"top-plume": 0}}, "loads": []},
    {"id": "lifted", "title": t("Lifted on two round slings", "An zwei Rundschlingen angehoben"), "description": t("Two round slings in basket hitch under the shell take the tank 400 mm off the bearers on the crane hook; bearers and roll stay on the ground.", "Zwei Rundschlingen im Korbanschlag unter dem Mantel heben den Speicher am Kranhaken 400 mm von den Kanthölzern; Kanthölzer und Rolle bleiben am Boden."), "bones": {"carrier": {"dy": LIFT_DY}}, "ground": "authored", "contacts": SLING_CONTACTS, "thermal": {"operating": {"charge": 0}, "regions": bind(0.4, 0.3, 0.8), "emitters": {"top-plume": 0}}, "loads": []},
    {"id": "reinstalled", "title": t("Reinstalled upright, charged and stratified", "Aufrecht wieder eingebaut, geladen und geschichtet"), "description": t(f"Back in a boiler room the tank stands on its three feet, filled with {VOLUME_L} l of heating water and charged by the heat generator: 68 degC under the top head, a thermocline in the middle and 37 degC at the bottom; the commissioning run is done before the jacket is refitted.", f"Wieder in einem Heizraum steht der Speicher auf seinen drei Füßen, mit {VOLUME_L} l Heizwasser gefüllt und vom Wärmeerzeuger geladen: 68 °C unter dem oberen Boden, eine Sprungschicht in der Mitte und 37 °C unten; der Inbetriebnahmelauf erfolgt, bevor der Mantel wieder angelegt wird."), "bones": UPRIGHT, "ground": "authored", "contacts": FOOT_CONTACTS, "thermal": {"operating": {"charge": 1}, "regions": bind(0, 0, 0.6), "emitters": {"top-plume": 1}}, "loads": WATER_LOAD},
    {"id": "reinstalled-cold", "title": t("Reinstalled, filled and cold", "Wieder eingebaut, gefüllt und kalt"), "description": t("The same place before the first charge: filled and vented, the water at room temperature.", "Derselbe Ort vor der ersten Ladung: gefüllt und entlüftet, das Wasser auf Raumtemperatur."), "bones": UPRIGHT, "ground": "authored", "contacts": FOOT_CONTACTS, "thermal": {"operating": {"charge": 0}, "regions": bind(0, 0, 0.6), "emitters": {"top-plume": 0}}, "loads": WATER_LOAD},
]

CLIPS = [
    {"id": "lift", "title": t("Lift off the bearers", "Von den Kanthölzern abheben"), "description": t("The crane takes up the two round slings and raises the tank 400 mm in four seconds; bearers and roll stay on the ground.", "Der Kran strafft die beiden Rundschlingen und hebt den Speicher in vier Sekunden um 400 mm an; Kanthölzer und Rolle bleiben am Boden."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1, "pose": "stored", "ease": "smooth"}, {"t": 5, "pose": "lifted", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 1} for a in BEARER_CONTACTS] + [{"anchor": a, "fromS": 1, "toS": 6} for a in SLING_CONTACTS],
     "phases": [{"id": "take-up", "fromS": 0, "toS": 1, "title": t("Slings taken up", "Schlingen gestrafft")}, {"id": "raise", "fromS": 1, "toS": 5, "title": t("Raise", "Anheben")}, {"id": "hold", "fromS": 5, "toS": 6, "title": t("Hold", "Halten")}],
     "events": [{"t": 1, "kind": "contact-off", "title": t("Tank leaves the bearers", "Speicher verlässt die Kanthölzer")}], "tracks": []},
    {"id": "charge", "title": t("Stratified charge after reinstallation", "Geschichtete Ladung nach dem Wiedereinbau"), "description": t("Time lapse of about three hours in 60 s: the heat generator feeds 70 degC at the top sleeve; the hot layer grows downwards, the thermocline passes the upper and then the lower zone, and the bottom stays coolest at 37 degC; the warm plume over the bare top head grows with it.", "Zeitraffer von etwa drei Stunden in 60 s: der Wärmeerzeuger speist 70 °C an der oberen Muffe ein; die heiße Schicht wächst nach unten, die Sprungschicht durchläuft die obere und dann die untere Zone, und der Boden bleibt mit 37 °C am kühlsten; die warme Fahne über dem blanken oberen Boden wächst mit."), "durationS": 60, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-cold", "ease": "linear"}, {"t": 60, "pose": "reinstalled", "ease": "linear"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 60} for a in FOOT_CONTACTS],
     "phases": [{"id": "hot-layer", "fromS": 0, "toS": 20, "title": t("Hot layer forms under the top head", "Heiße Schicht bildet sich unter dem oberen Boden")}, {"id": "thermocline", "fromS": 20, "toS": 50, "title": t("Thermocline moves down", "Sprungschicht wandert nach unten")}, {"id": "charged", "fromS": 50, "toS": 60, "title": t("Charged", "Geladen")}],
     "events": [{"t": 0, "kind": "operating", "title": t("Charge pump starts", "Ladepumpe läuft an"), "target": "charge"}, {"t": 50, "kind": "operating", "title": t("Bottom sensor reached, charge ends", "Unterer Fühler erreicht, Ladung endet"), "target": "charge"}],
     "tracks": [{"target": "operating:charge", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 20, "v": 0.5}, {"t": 50, "v": 1}, {"t": 60, "v": 1}]}, {"target": "emitter:top-plume", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 20, "v": 0.6}, {"t": 50, "v": 1}, {"t": 60, "v": 1}]}]},
]

SHELL_LAYER = {"id": "shell-steel", "title": t("Steel shell 3 mm, primed, bare (jacket removed)", "Stahlmantel 3 mm, grundiert, blank (Mantel abgenommen)"), "kind": "metal", "material": "primed-steel", "thicknessM": T_SHELL, "resistanceM2KW": r6(T_SHELL / 50), "heatCapacityJM2K": round(T_SHELL * RHO_STEEL * 450), "source": "literature", "reference": "en-iso-10456"}
ROWS = [{"v": 0.5, "supplyTempC": 55, "returnTempC": 30}, {"v": 1, "supplyTempC": 70, "returnTempC": 35}]
ZONE_TITLES = {"zone-bottom": t("Bottom zone with the lower head (return side)", "Untere Zone mit dem unteren Boden (Rücklaufseite)"), "zone-lower": t("Lower middle zone", "Untere mittlere Zone"), "zone-upper": t("Upper middle zone", "Obere mittlere Zone"), "zone-top": t("Top zone with the upper head (flow side)", "Obere Zone mit dem oberen Boden (Vorlaufseite)")}


def zone_region(zid, pos):
    """🌡️ One stratified zone of the shell: the inside is the store water at its height in the charge profile."""
    r = {"id": zid, "title": ZONE_TITLES[zid], "kind": "surface", "model": "hydronic", "material": "primed-steel", "areaM2": ZONE_AREA[zid], "characteristicLengthM": 2 * R, "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [SHELL_LAYER], "heatCapacityJM2K": SHELL_LAYER["heatCapacityJM2K"], "hydronic": {"key": "charge", "rows": ROWS, "flowPosition": pos, "filmM2KW": 0.0015}, "source": "authored-estimate", "reference": "en-12977-3"}
    if zid == "zone-top":
        r["ratedPowerW"] = 0
        r["ratedPowerBasis"] = "passive"
    return r


THERMAL = {
    "environment": {"airTempC": 12, "meanRadiantTempC": 11, "skyTempC": 2, "groundTempC": 10, "relativeHumidity": 0.7, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 250, "diffuseFraction": 0.6, "pressurePa": 101325},
    "passengers": [],
    "operating": [{"id": "charge", "title": t("Charge state of the store", "Ladezustand des Speichers"), "levels": [{"v": 0, "title": t("Drained or cold (room temperature)", "Entleert oder kalt (Raumtemperatur)")}, {"v": 0.5, "title": t("Half charged, 55 degC at the top, 30 degC at the bottom", "Halb geladen, oben 55 °C, unten 30 °C")}, {"v": 1, "title": t("Charged, 70 degC flow, 35 degC return, stratified", "Geladen, Vorlauf 70 °C, Rücklauf 35 °C, geschichtet")}]}],
    "regions": [zone_region(z, pos) for z, _, _, pos in ZONES] + [
        {"id": "sleeves", "title": t("Eight 6/4 inch sleeves with galvanised plugs", "Acht Muffen 6/4 Zoll mit verzinkten Stopfen"), "kind": "surface", "model": "hydronic", "material": "galvanised-iron", "areaM2": SLEEVE_AREA, "characteristicLengthM": 0.055, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 16000, "hydronic": {"key": "charge", "rows": ROWS, "flowPosition": 0.5, "filmM2KW": 0.02}, "source": "authored-estimate", "reference": "en-10242"},
        {"id": "feet", "title": t("Three tubular feet with foot plates", "Drei Rohrfüße mit Fußplatten"), "kind": "surface", "model": "passive", "material": "primed-steel", "areaM2": LEG_AREA, "characteristicLengthM": 0.06, "geometry": "cylinder", "skyViewFactor": 0.2, "layers": [], "heatCapacityJM2K": round(0.0029 * RHO_STEEL * 450), "source": "authored-estimate", "reference": "en-10255"},
        {"id": "bearers", "title": t("Squared timber bearers with chocks", "Kanthölzer mit Keilen"), "kind": "surface", "model": "passive", "material": "spruce-weathered", "areaM2": BEARER_AREA, "characteristicLengthM": 0.1, "geometry": "horizontal-plate-up", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 24000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "jacket-roll", "title": t("Rolled insulation jacket with straps", "Gerollter Dämmmantel mit Gurten"), "kind": "surface", "model": "passive", "material": "fleece-jacket", "areaM2": ROLL_AREA, "characteristicLengthM": 2 * ROLL_RO, "geometry": "cylinder", "skyViewFactor": 0.45, "layers": [], "heatCapacityJM2K": 1000, "source": "authored-estimate", "reference": "en-iso-13786"},
    ],
    "emitters": [{"id": "top-plume", "kind": "plume", "title": t("Warm air rising over the bare top head", "Warme Luft über dem blanken oberen Boden"), "anchor": "tank-top", "directionDeg": 90, "diameterM": 0.5, "region": "zone-top", "source": "authored-estimate", "reference": "incropera-2018"}],
    "assumptions": [
        t("The tank draws no power: it is charged from a heat generator elsewhere (0 W passive, stated on the top zone).", "Der Speicher nimmt keine Leistung auf: er wird von einem Wärmeerzeuger an anderer Stelle geladen (0 W passiv, an der oberen Zone angegeben)."),
        t("Stratification: the shell is four hydronic zones on one charge circuit; each zone sees the store water at its height, from the 70 degC flow at the top (flow position 0.05) to the 35 degC return at the bottom (0.95), with a thermocline between the upper (0.25) and the lower middle zone (0.75); half charged the circuit runs 55/30 degC.", "Schichtung: der Mantel besteht aus vier hydronischen Zonen an einem Ladekreis; jede Zone sieht das Speicherwasser auf ihrer Höhe, vom Vorlauf 70 °C oben (Strömungsposition 0,05) bis zum Rücklauf 35 °C unten (0,95), mit einer Sprungschicht zwischen der oberen (0,25) und der unteren mittleren Zone (0,75); halb geladen läuft der Kreis mit 55/30 °C."),
        t("The jacket is off: the shell is bare primed steel 3 mm with the water-side free-convection film of 0.0015 m2K/W, so the reinstalled commissioning run shows the full surface temperature of the store; refitted, the 100 mm fleece (0.04 W/mK) would cut the loss by about 95 %.", "Der Mantel ist abgenommen: der Mantel ist blanker grundierter Stahl 3 mm mit dem wasserseitigen Film freier Konvektion von 0,0015 m²K/W, sodass der Inbetriebnahmelauf im Wiedereinbau die volle Oberflächentemperatur des Speichers zeigt; wieder angelegt würde das Vlies von 100 mm (0,04 W/mK) den Verlust um etwa 95 % senken."),
        t("Weathered surfaces: red-brown primer scuffed and dull (emissivity 0.9, solar absorptance 0.72), galvanised plugs dull grey (0.28 instead of 0.05 to 0.1 when new), spruce weathered grey (0.65), the polystyrene skin of the jacket faded light grey (0.45).", "Gealterte Oberflächen: rotbraune Grundierung verschrammt und matt (Emissionsgrad 0,9, solare Absorption 0,72), verzinkte Stopfen mattgrau (0,28 statt 0,05 bis 0,1 im Neuzustand), Fichte vergraut (0,65), die Polystyrolhaut des Mantels zu Hellgrau verblichen (0,45)."),
        t("Reference case: a cool covered yard or boiler room at 12 degC; stored and lifted the tank is drained and at ambient temperature; the feet, bearers and roll are passive.", "Referenzfall: ein kühler überdachter Hof oder Heizraum bei 12 °C; gelagert und angehoben ist der Speicher entleert und auf Umgebungstemperatur; Füße, Kanthölzer und Rolle sind passiv."),
    ],
}

STRUCTURE = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
    t("Supports per pose: stored, four ground contacts under the two bearers (the chocks hold the cylinder against rolling); lifted, two round slings (EN 1492-2) over the shell at 550 and 1450 mm from the feet end; reinstalled, the three foot plates on the boiler-room floor. The whole set is one rigid body per pose, so in the lifted pose the bearers and the roll left on the ground are counted with the tank (17 kg, 13 % of the weight).", "Auflager je Pose: gelagert die vier Bodenkontakte unter den beiden Kanthölzern (die Keile halten den Zylinder gegen Rollen); angehoben zwei Rundschlingen (EN 1492-2) über dem Mantel bei 550 und 1450 mm vom Fußende; eingebaut die drei Fußplatten auf dem Heizraumboden. Der ganze Satz ist je Pose ein starrer Körper; in der angehobenen Pose werden Kanthölzer und Rolle, die am Boden bleiben, daher mit dem Speicher gezählt (17 kg, 13 % des Gewichts)."),
    t(f"Masses from areas, lengths and densities: shell {SHELL_AREA:.2f} m2 x 3 mm = {SHELL_KG:.1f} kg, three feet {3 * LEG_KG:.1f} kg, eight sleeves and plugs {8 * (SLEEVE_KG + PLUG_KG):.1f} kg (tank {TANK_KG} kg), bearers with chocks {BEARER_KG} kg each, jacket roll {ROLL_KG} kg; total {MASS} kg.", f"Massen aus Flächen, Längen und Dichten: Mantel {dez(f'{SHELL_AREA:.2f}')} m² x 3 mm = {dez(f'{SHELL_KG:.1f}')} kg, drei Füße {dez(f'{3 * LEG_KG:.1f}')} kg, acht Muffen und Stopfen {dez(f'{8 * (SLEEVE_KG + PLUG_KG):.1f}')} kg (Speicher {dez(TANK_KG)} kg), Kanthölzer mit Keilen je {dez(BEARER_KG)} kg, Dämmrolle {dez(ROLL_KG)} kg; zusammen {dez(MASS)} kg."),
    t(f"Reinstalled, the tank holds {VOLUME_L} l of heating water at a mean 985 kg/m3 ({WATER_KG} kg, {WATER_KG * G / 1000:.2f} kN) at the centre of the filling, carried by the shell into the three feet; the 3 bar operating pressure is a membrane stress of the shell (sigma = p r / t = 39 MPa against 235 MPa) and not drawn.", f"Eingebaut enthält der Speicher {VOLUME_L} l Heizwasser mit im Mittel 985 kg/m³ ({dez(WATER_KG)} kg, {dez(f'{WATER_KG * G / 1000:.2f}')} kN) im Schwerpunkt der Füllung, vom Mantel in die drei Füße getragen; der Betriebsdruck von 3 bar ist eine Membranspannung des Mantels (sigma = p r / t = 39 MPa gegen 235 MPa) und nicht dargestellt."),
], "clinical": False}

ASSET = {
    "id": SLUG,
    "category": "components",
    "title": t("Used Heating Buffer Tank", "Gebrauchter Heizungspufferspeicher"),
    "summary": t(f"A used 800 l steel heating buffer tank, Ø 790 mm without insulation and 1.80 m tall on three tubular feet, {TANK_KG:.0f} kg empty, drained and its eight 6/4 inch sleeves closed with galvanised plugs; it lies on two squared bearers with nailed chocks beside its rolled and strapped soft insulation jacket, hangs from two round slings when lifted and stands upright, filled and stratified from 70 to 35 degC when reinstalled.", f"Ein gebrauchter Heizungspufferspeicher aus Stahl mit 800 l, Ø 790 mm ohne Dämmung und 1,80 m hoch auf drei Rohrfüßen, leer {TANK_KG:.0f} kg, entleert und seine acht Muffen 6/4 Zoll mit verzinkten Stopfen verschlossen; er liegt auf zwei Kanthölzern mit genagelten Keilen neben seinem gerollten und gegurteten Weichschaum-Dämmmantel, hängt beim Anheben an zwei Rundschlingen und steht im Wiedereinbau aufrecht, gefüllt und von 70 bis 35 °C geschichtet."),
    "tags": ["component", "reclaimed", "used", "hvac", "heating", "plumbing", "steel", "tank", "buffer-tank", "storage", "insulation"],
    "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
        f"Typical 800 l buffer tank without heat exchanger: shell Ø 790 x 3 mm S235JR, elliptical heads 150 mm deep, overall height 1800 mm on three DN 50 feet, actual water volume {VOLUME_L} l, eight 6/4 inch sleeves in two rows; jacket 100 mm soft fleece with polystyrene skin. Empty tank {TANK_KG} kg, set with bearers and roll {MASS} kg. All dimensions and masses are catalogue-typical (authored-estimate), nothing is measured.",
        f"Typischer Pufferspeicher 800 l ohne Wärmetauscher: Mantel Ø 790 x 3 mm S235JR, elliptische Böden 150 mm tief, Gesamthöhe 1800 mm auf drei Füßen DN 50, wahres Wasservolumen {VOLUME_L} l, acht Muffen 6/4 Zoll in zwei Reihen; Mantel 100 mm Weichschaum-Vlies mit Polystyrolhaut. Leerer Speicher {dez(TANK_KG)} kg, Satz mit Kanthölzern und Rolle {dez(MASS)} kg. Alle Maße und Massen sind katalogtypisch (authored-estimate), nichts ist gemessen.")},
    "dimensions": {"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH, "massKg": MASS},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "yard", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
    "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"[DEBUG] wrote {SLUG}: mass {MASS} kg, tank {TANK_KG} kg (shell {SHELL_KG:.1f}), bearer {BEARER_KG}, roll {ROLL_KG}, volume {VOLUME_L} l, h {HEIGHT} w {WIDTH} d {DEPTH}")
    print(f"[DEBUG] areas zones {ZONE_AREA} sleeves {SLEEVE_AREA} feet {LEG_AREA} bearers {BEARER_AREA} roll {ROLL_AREA}")
