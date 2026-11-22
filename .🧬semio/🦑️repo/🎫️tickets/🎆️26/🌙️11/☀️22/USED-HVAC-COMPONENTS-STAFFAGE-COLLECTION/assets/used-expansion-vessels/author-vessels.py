"""🔴 Authoritative generator of the staffage asset `used-expansion-vessels`.

Three used red diaphragm expansion vessels of 80, 50 and 35 l (6 bar, factory pre-charge 1.5 bar, R 3/4 water connection at the
bottom, gas cap valve on top, three welded feet; the 35 l one still wears the wall clip of its last installation), drained, their
water connections capped, standing in a galvanised steel cage pallet with a steel floor and a drop flap; lifted on fork tines;
reinstalled on a plant-room floor in front of the cage, filled and in service. Every hand-chosen number is stated here; running
the script rewrites `🔣️.json` next to it.

@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""

import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
SLUG = "used-expansion-vessels"
G = 9.81
RHO_STEEL, RHO_WATER = 7850.0, 990.0

CL, CW, CH = 1.24, 0.835, 0.97
FOOT_L, FOOT_H = 0.12, 0.105
RAIL_W, RAIL_TOP = 0.04, 0.17
POST_W = 0.04
TOP_Y0 = 0.93
FLOOR_T = 0.002
MESH_T = 0.001
MESH_PITCH, WIRE_D = 0.05, 0.004
ZW = CW / 2
ZI = ZW - RAIL_W
ZM = ZW - RAIL_W / 2
XI0, XI1 = RAIL_W, CL - RAIL_W
FLAP_Y0, FLAP_Y1, FLAP_BAR = 0.535, TOP_Y0, 0.032
MID_X = 0.62

T_SHELL = 0.0018
T_REMAIN = 0.0018
VESSELS = {
    "vessel-80": {"litre": 80, "R": 0.24, "hd": 0.12, "hc": 0.282, "f": 0.044, "cx": 0.29, "cz": -0.13, "membrane": 0.9},
    "vessel-50": {"litre": 50, "R": 0.2045, "hd": 0.1023, "hc": 0.244, "f": 0.055, "cx": 0.75, "cz": 0.165, "membrane": 0.6},
    "vessel-35": {"litre": 35, "R": 0.177, "hd": 0.0885, "hc": 0.238, "f": 0.035, "cx": 1.0, "cz": -0.185, "membrane": 0.45},
}
FOOT_W = 0.035
FOOT_ANGLES = (90, 210, 330)
NIP_R, CAP_R, GAS_R = 0.01345, 0.017, 0.015
P0, PA, PE = 1.5, 2.0, 2.5
CLIP_W, CLIP_T = 0.04, 0.005
BRK = (0.05, 0.10, 0.05)
TRANSFER_DX = 1.6
TRANSFER_DY = -RAIL_TOP
TRANSFER_AXIS = math.degrees(math.atan2(TRANSFER_DY, TRANSFER_DX))
TRANSFER_M = math.hypot(TRANSFER_DX, TRANSFER_DY)
LIFT = 0.3
FORK_X = (0.40, 0.84)
T_ROOM, T_SERVICE, H_COMB = 18.0, 30.0, 9.0


def t(en, de):
    """📝 One bilingual text."""
    return {"en": en, "de": de}


def dez(x):
    """🇩🇪 A number with a decimal comma."""
    return str(x).replace(".", ",")


def r6(x):
    """🎯 Round to micrometres."""
    return round(x, 6)


for v in VESSELS.values():
    v["yc"] = v["f"] + v["hd"] + v["hc"] / 2
    v["H"] = v["f"] + 2 * v["hd"] + v["hc"]
    v["rf"] = 0.72 * v["R"]


def dish_y(v, r):
    """🥣 Height above the feet of the lower dished end at radius `r`."""
    return v["f"] + v["hd"] * (1 - math.sqrt(max(0.0, 1 - (r / v["R"]) ** 2)))


def head_area(R, c):
    """🥚 Surface of one 2:1-type elliptical head (half an oblate spheroid of radii R and c)."""
    e = math.sqrt(1 - (c / R) ** 2)
    return math.pi * R * R * (1 + (1 - e * e) / e * math.atanh(e))


def volume_l(v):
    """🫙 Nominal volume of the shell in litres (cylinder and two elliptical heads)."""
    return (math.pi * v["R"] ** 2 * v["hc"] + 2 * 2 / 3 * math.pi * v["R"] ** 2 * v["hd"]) * 1000


for vid, v in VESSELS.items():
    v["area"] = 2 * math.pi * v["R"] * v["hc"] + 2 * head_area(v["R"], v["hd"])
    v["shell_kg"] = v["area"] * T_SHELL * RHO_STEEL
    v["feet_kg"] = 3 * 0.25
    v["fit_kg"] = 0.25 + (0.6 if vid == "vessel-35" else 0)
    v["kg"] = round(v["shell_kg"] + v["membrane"] + v["feet_kg"] + v["fit_kg"], 2)
    v["vol"] = volume_l(v)
    v["w_cold"] = v["litre"] * (1 - (P0 + 1) / (PA + 1)) * RHO_WATER / 1000
    v["w_hot"] = v["litre"] * (1 - (P0 + 1) / (PE + 1)) * RHO_WATER / 1000

CAGE_PARTS = {
    "feet": 4 * (4 * FOOT_L * FOOT_H + FOOT_L ** 2) * 0.003 * RHO_STEEL,
    "base frame": (2 * CL + 2 * (CW - 2 * RAIL_W) + (CW - 2 * RAIL_W)) * (0.065 + 2 * 0.04) * 0.004 * RHO_STEEL,
    "floor": (XI1 - XI0) * 2 * ZI * FLOOR_T * RHO_STEEL,
    "posts": 4 * (TOP_Y0 - RAIL_TOP) * 2.42,
    "top frame": (2 * CL + 2 * (CW - 2 * RAIL_W)) * 2.42,
    "mesh": (2 * (XI1 - XI0) + 2 * 2 * ZI) * (TOP_Y0 - RAIL_TOP) * (2 / MESH_PITCH) * math.pi * (WIRE_D / 2) ** 2 * RHO_STEEL,
    "flap": (2 * (XI1 - XI0) + 2 * (FLAP_Y1 - FLAP_Y0 - 2 * FLAP_BAR)) * 1.36 + 1.0,
}
CAGE_KG = round(sum(CAGE_PARTS.values()), 2)
MASS = round(CAGE_KG + sum(v["kg"] for v in VESSELS.values()), 2)
MESH_KG = CAGE_PARTS["mesh"]
W80, W50, W35 = (VESSELS[k]["w_hot"] for k in ("vessel-80", "vessel-50", "vessel-35"))

MATERIALS = [
    {"id": "powder-red", "title": t("Steel sheet S235 with red polyester powder coating, faded and chipped", "Stahlblech S235 mit roter Polyester-Pulverbeschichtung, verblasst und abgeplatzt"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "galvanised-weathered", "title": t("Hot-dip galvanised steel, dull grey weathered", "Feuerverzinkter Stahl, mattgrau bewittert"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "mesh-wire", "title": t("Welded galvanised wire mesh 50 x 50 mm, wire 4 mm", "Geschweißtes verzinktes Drahtgitter 50 x 50 mm, Draht 4 mm"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 400e6, "source": "literature", "reference": "din-15155"},
    {"id": "pe-cap", "title": t("Red PE protective cap", "Rote PE-Schutzkappe"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 950, "conductivityWmK": 0.4, "specificHeatJKgK": 1900, "source": "literature", "reference": "en-iso-10456"},
    {"id": "pa-cap", "title": t("Black polyamide valve cap", "Schwarze Ventilkappe aus Polyamid"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1140, "conductivityWmK": 0.25, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-iso-10456"},
    {"id": "galvanised-strap", "title": t("Galvanised steel strap of the wall clip, scratched", "Verzinktes Stahlband der Wandschelle, verkratzt"), "emissivity": 0.3, "solarAbsorptivity": 0.65, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "ashrae-2021"},
]

REFERENCES = [
    {"id": "en-13831", "citation": "EN 13831:2007 Closed expansion vessels with built in diaphragm for installation in water (design, materials, test pressure 1.43 x PS, marking of the type plate with nominal volume, maximum pressure and pre-charge)."},
    {"id": "en-12828", "citation": "EN 12828:2012+A1:2014 Heating systems in buildings, design for water-based heating systems (pressure maintenance, expansion vessel sizing, pre-charge at least static height plus 0.2 bar)."},
    {"id": "din-4807-2", "citation": "DIN 4807-2:1991 Expansion vessels, calculation (water volume of a diaphragm vessel V_w = V_n (1 - (p_0 + 1) / (p + 1)) with absolute pressures; pre-charge 1.5 bar, initial pressure 2.0 bar, final pressure 2.5 bar taken here)."},
    {"id": "vessel-datasheets", "citation": "Manufacturer data sheets of red diaphragm expansion vessels for closed heating systems, 6 bar, 120 degC, R 3/4 connection (for example Reflex N, Flamco Flexcon, Zilmet Hy-Pro): 35 l about 354 mm x 450 mm, 50 l about 409 mm x 505 mm, 80 l about 480 mm x 566 mm, empty masses about 9, 11 and 15 kg; values here are typical of the class, not of one product (authored-estimate)."},
    {"id": "din-15155", "citation": "DIN 15155:1972 Pallets, box pallet with lattice walls (outside 1240 x 835 x 970 mm, inside 1200 x 800 x 800 mm, drop flap on one long side, four corner feet for fork entry from all sides, wire mesh 50 x 50 mm; tare about 70 to 85 kg, rated load 1000 kg)."},
    {"id": "iso-2328", "citation": "ISO 2328:2011 Fork-lift trucks, hook-on type fork arms and fork arm carriages (class 2A fork arms 100 x 40 mm; tine spread of about 0.45 m used for the lifted pose)."},
    {"id": "en-10255", "citation": "EN 10255:2004+A1:2007 Non-alloy steel tubes suitable for welding and threading (R 3/4 = DN 20, outside diameter 26.9 mm, used for the water connection nipple)."},
    {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels (S235, yield strength 235 MPa) and EN 10056-1:2017 equal leg angles (40 x 40 x 4 mm, 2.42 kg/m; 30 x 30 x 3 mm, 1.36 kg/m)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (design conductivity and heat capacity: steel 50 W/mK, 450 J/kgK; PE-HD 0.4 W/mK, 1900 J/kgK; polyamide 0.25 W/mK, 1600 J/kgK)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, Table 5 emissivities and solar absorptances: painted steel 0.9 (red paint solar absorptance about 0.6 to 0.7, faded at the lower end); weathered galvanised steel about 0.28 (bright new 0.05 to 0.1)."},
    {"id": "vdi-6030", "citation": "VDI 6030 Blatt 1:2002 Auslegung von freien Raumheizflächen, and EN ISO 6946:2017 Annex C (combined surface coefficient of a free vertical metal surface in a room about 9 W/m2K: convection about 3.5, radiation about 5.5 at emissivity 0.9)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types, contents and basic principles of presentation (scales, line groups)."},
]

BONES = [
    {"id": "yard", "parent": None, "title": t("Yard ground (frame of the cage pallet)", "Lagerplatz (Bezugsrahmen der Gitterbox)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "carrier", "parent": "yard", "title": t("Lift carrier (massless, raises the cage pallet on the fork tines)", "Hubträger (masselos, hebt die Gitterbox auf den Gabelzinken)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 0.6}},
    {"id": "cage", "parent": "carrier", "title": t("Cage pallet: feet, base frame, steel floor, corner posts, top frame, mesh walls and drop flap", "Gitterbox: Füße, Bodenrahmen, Stahlboden, Eckpfosten, Oberrahmen, Gitterwände und Klappe"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": CL, "joint": {"type": "fixed"},
     "mass": {"kg": CAGE_KG, "comX": CL / 2, "comY": 0.36, "distribution": "uniform", "source": "authored-estimate", "reference": "din-15155"},
     "section": {"areaM2": 0.00058, "secondMomentM4": 3.6e-07, "extremeFibreM": 0.0325, "material": "galvanised-weathered", "title": t("Base frame channel 65 x 40 x 4 mm", "Bodenrahmen-U-Profil 65 x 40 x 4 mm")}},
    {"id": "transfer", "parent": "cage", "title": t("Transfer slide (massless, carries the vessels from the cage floor to the plant-room floor)", "Umsetzschlitten (masselos, trägt die Gefäße vom Gitterboxboden auf den Technikraumboden)"), "x": 0, "y": RAIL_TOP, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": TRANSFER_AXIS, "minM": 0, "maxM": 1.8}},
]
NAMES = {"vessel-80": ("80 l", "80 l"), "vessel-50": ("50 l", "50 l"), "vessel-35": ("35 l with wall clip", "35 l mit Wandschelle")}
for vid, v in VESSELS.items():
    BONES.append({"id": vid, "parent": "transfer", "title": t(f"Expansion vessel {NAMES[vid][0]}", f"Ausdehnungsgefäß {NAMES[vid][1]}"), "x": r6(v["cx"] - v["R"]), "y": r6(v["yc"]), "z": v["cz"], "angleDeg": 0, "lengthM": r6(2 * v["R"]), "joint": {"type": "fixed"},
                  "mass": {"kg": v["kg"], "comX": v["R"], "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "vessel-datasheets"},
                  "section": {"areaM2": r6(2 * math.pi * v["R"] * T_SHELL), "secondMomentM4": float(f"{math.pi * v['R'] ** 3 * T_SHELL:.4e}"), "extremeFibreM": v["R"], "material": "powder-red", "title": t(f"Cylindrical shell {v['R'] * 2000:.0f} x {T_SHELL * 1000:.1f} mm", f"Zylindermantel {v['R'] * 2000:.0f} x {dez(T_SHELL * 1000)} mm")}})


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
for xn, xx in (("front", CL - FOOT_L / 2), ("rear", FOOT_L / 2)):
    for side, z in (("l", ZW - FOOT_L / 2), ("r", -ZW + FOOT_L / 2)):
        ANCHORS.append(anchor(f"cage-foot-{xn}-{side}", "cage", xx, 0, "support", f"Ground under the {xn} {'left' if side == 'l' else 'right'} cage foot", f"Boden unter dem {'vorderen' if xn == 'front' else 'hinteren'} {'linken' if side == 'l' else 'rechten'} Fuß der Gitterbox", z, footprintLengthM=FOOT_L, footprintWidthM=FOOT_L, **SUP))
for k, xf in enumerate(FORK_X):
    for side, z in (("l", 0.3), ("r", -0.3)):
        ANCHORS.append(anchor(f"fork-{'ab'[k]}-{side}", "cage", xf, FOOT_H, "support", f"Fork tine {'AB'[k]} under the base frame, {'left' if side == 'l' else 'right'}", f"Gabelzinke {'AB'[k]} unter dem Bodenrahmen, {'links' if side == 'l' else 'rechts'}", z, footprintLengthM=0.1, footprintWidthM=0.1, **SUP))
FOOT_POS = {}
for vid, v in VESSELS.items():
    FOOT_POS[vid] = []
    for k, a in enumerate(FOOT_ANGLES):
        du, dw = v["rf"] * math.cos(math.radians(a)), v["rf"] * math.sin(math.radians(a))
        FOOT_POS[vid].append((du, dw))
        ANCHORS.append(anchor(f"{vid}-foot-{k + 1}", vid, v["R"] + du, -v["yc"], "support", f"Foot {k + 1} of the {NAMES[vid][0]} vessel on the floor", f"Fuß {k + 1} des Gefäßes {NAMES[vid][1]} auf dem Boden", dw, footprintLengthM=FOOT_W, footprintWidthM=FOOT_W, **SUP))
    ANCHORS.append(anchor(f"{vid}-water", vid, v["R"], -v["hc"] / 2, "load", f"Centre of the water on the water side of the {NAMES[vid][0]} vessel", f"Schwerpunkt des Wassers auf der Wasserseite des Gefäßes {NAMES[vid][1]}"))
    ANCHORS.append(anchor(f"{vid}-top", vid, v["R"], v["hc"] / 2 + v["hd"], "contact", f"Gas cap valve on top of the {NAMES[vid][0]} vessel", f"Gasfüllventil oben auf dem Gefäß {NAMES[vid][1]}"))


def solid(id_, bone, material, tone, kind, **kw):
    """🧊 A body solid."""
    s = {"id": id_, "kind": kind, "bone": bone, "material": material, "role": kw.pop("role", "body"), "tone": tone}
    s.update(kw)
    return s


def boxs(id_, bone, material, tone, x0, x1, y0, y1, z0, z1, **kw):
    """📦 An axis-aligned box from its bone-local extents."""
    return solid(id_, bone, material, tone, "box", size=[r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], transform={"translation": [r6((x0 + x1) / 2), r6((y0 + y1) / 2), r6((z0 + z1) / 2)]}, **kw)


def cyl(id_, bone, material, tone, origin, a0, a1, radius, **kw):
    """🥫 A vertical cylinder as an open two-point revolve from `a0` to `a1` above `origin`."""
    return solid(id_, bone, material, tone, "revolve", profile=[[r6(a0), r6(radius)], [r6(a1), r6(radius)]], axis=[0, 1, 0], origin=[r6(x) for x in origin], **kw)


def shell_profile(v, n, pedestal=False):
    """🥚 The [axial, radial] profile of a vessel shell about its axis, bottom pole to top pole (a filled body)."""
    R, hd, hc = v["R"], v["hd"], v["hc"]
    pts = []
    if pedestal:
        rp = 0.8 * R
        pts += [[-v["yc"], 0.0], [-v["yc"], rp], [-hc / 2 - hd * math.sqrt(1 - 0.64), rp]]
        k0 = 0
        for k in range(n + 1):
            tt = math.radians(90 * k / n)
            if R * math.sin(tt) > rp + 1e-6:
                pts.append([-hc / 2 - hd * math.cos(tt), R * math.sin(tt)])
    else:
        for k in range(n + 1):
            tt = math.radians(90 * k / n)
            pts.append([-hc / 2 - hd * math.cos(tt), R * math.sin(tt)])
    for k in range(n + 1):
        tt = math.radians(90 * k / n)
        pts.append([hc / 2 + hd * math.sin(tt), R * math.cos(tt)])
    return [[r6(a), r6(r)] for a, r in pts]


SEG = {50: 8, 100: 6, 200: 4, 500: 3}


def flap_bar(scale):
    """📏 Drawn width of the flap angles: true 32 mm, at 1:200 widened to 40 mm so the bars still print at 1:350."""
    return 0.04 if scale == 200 else FLAP_BAR


def cage_bodies(scale):
    """🧺 The body solids of the cage pallet at one scale."""
    g, w = "galvanised-weathered", "mesh-wire"
    out = []
    if scale == 500:
        out.append(boxs("plinth", "cage", g, 0.58, 0, CL, 0, RAIL_TOP, -ZW, ZW))
        pw = 0.075
        for xn, x0 in (("rear", 0.0), ("front", CL - pw)):
            for side, z0 in (("l", ZW - pw), ("r", -ZW)):
                out.append(boxs(f"post-{xn}-{side}", "cage", g, 0.62, x0, x0 + pw, RAIL_TOP, CH - pw, z0, z0 + pw))
        out.append(boxs("top-l", "cage", g, 0.62, 0, CL, CH - pw, CH, ZW - pw, ZW))
        out.append(boxs("top-r", "cage", g, 0.62, 0, CL, CH - pw, CH, -ZW, -ZW + pw))
        out.append(boxs("top-rear", "cage", g, 0.62, 0, pw, CH - pw, CH, -ZW + pw, ZW - pw))
        out.append(boxs("top-front", "cage", g, 0.62, CL - pw, CL, CH - pw, CH, -ZW + pw, ZW - pw))
        y0, y1, xa, xb, za = RAIL_TOP, CH - pw, pw, CL - pw, ZW - pw / 2
    else:
        for xn, x0 in (("rear", 0.0), ("front", CL - FOOT_L)):
            for side, z0 in (("l", ZW - FOOT_L), ("r", -ZW)):
                out.append(boxs(f"foot-{xn}-{side}", "cage", g, 0.55, x0, x0 + FOOT_L, 0, FOOT_H, z0, z0 + FOOT_L))
        out.append(boxs("rail-l", "cage", g, 0.6, 0, CL, FOOT_H, RAIL_TOP, ZI, ZW))
        out.append(boxs("rail-r", "cage", g, 0.6, 0, CL, FOOT_H, RAIL_TOP, -ZW, -ZI))
        out.append(boxs("rail-rear", "cage", g, 0.6, 0, RAIL_W, FOOT_H, RAIL_TOP, -ZI, ZI))
        out.append(boxs("rail-front", "cage", g, 0.6, CL - RAIL_W, CL, FOOT_H, RAIL_TOP, -ZI, ZI))
        out.append(boxs("bearer-mid", "cage", g, 0.6, MID_X - 0.02, MID_X + 0.02, FOOT_H, RAIL_TOP - FLOOR_T, -ZI, ZI))
        out.append(boxs("floor", "cage", g, 0.66, XI0, XI1, RAIL_TOP - FLOOR_T, RAIL_TOP, -ZI, ZI, sheet=True))
        for xn, x0 in (("rear", 0.0), ("front", CL - POST_W)):
            for side, z0 in (("l", ZI), ("r", -ZW)):
                out.append(boxs(f"post-{xn}-{side}", "cage", g, 0.62, x0, x0 + POST_W, RAIL_TOP, TOP_Y0, z0, z0 + POST_W))
        out.append(boxs("top-l", "cage", g, 0.62, 0, CL, TOP_Y0, CH, ZI, ZW))
        out.append(boxs("top-r", "cage", g, 0.62, 0, CL, TOP_Y0, CH, -ZW, -ZI))
        out.append(boxs("top-rear", "cage", g, 0.62, 0, RAIL_W, TOP_Y0, CH, -ZI, ZI))
        out.append(boxs("top-front", "cage", g, 0.62, CL - RAIL_W, CL, TOP_Y0, CH, -ZI, ZI))
        fb = flap_bar(scale)
        fz0, fz1 = -ZM - fb / 2, -ZM + fb / 2
        sl = {"slender": True} if scale in (100, 200) else {}
        out.append(boxs("flap-hinge-bar", "cage", g, 0.5, XI0, XI1, FLAP_Y0, FLAP_Y0 + fb, fz0, fz1, **sl))
        out.append(boxs("flap-top-bar", "cage", g, 0.5, XI0, XI1, FLAP_Y1 - fb, FLAP_Y1, fz0, fz1, **sl))
        if scale in (50, 100):
            out.append(boxs("flap-side-rear", "cage", g, 0.5, XI0, XI0 + FLAP_BAR, FLAP_Y0 + FLAP_BAR, FLAP_Y1 - FLAP_BAR, fz0, fz1))
            out.append(boxs("flap-side-front", "cage", g, 0.5, XI1 - FLAP_BAR, XI1, FLAP_Y0 + FLAP_BAR, FLAP_Y1 - FLAP_BAR, fz0, fz1))
        y0, y1, xa, xb, za = RAIL_TOP, TOP_Y0, XI0, XI1, ZM
    gl = {"role": "glazing"}
    out.append(boxs("mesh-l", "cage", w, 0.6, xa, xb, y0, y1, za - MESH_T / 2, za + MESH_T / 2, **gl))
    out.append(boxs("mesh-r", "cage", w, 0.6, xa, xb, y0, y1, -za - MESH_T / 2, -za + MESH_T / 2, **gl))
    xm = xa / 2 if scale != 500 else 0.0375
    out.append(boxs("mesh-rear", "cage", w, 0.6, xm - MESH_T / 2, xm + MESH_T / 2, y0, y1, -za + (0 if scale != 500 else 0.0375), za - (0 if scale != 500 else 0.0375), **gl))
    out.append(boxs("mesh-front", "cage", w, 0.6, CL - xm - MESH_T / 2, CL - xm + MESH_T / 2, y0, y1, -za + (0 if scale != 500 else 0.0375), za - (0 if scale != 500 else 0.0375), **gl))
    return out


def vessel_bodies(scale, vid, v):
    """🔴 The body solids of one vessel at one scale."""
    R = v["R"]
    out = [solid(f"{vid}-shell", vid, "powder-red", 0.42, "revolve", profile=shell_profile(v, SEG[scale], pedestal=scale == 500), axis=[0, 1, 0], origin=[R, 0, 0])]
    if scale == 500:
        return out
    base = -v["yc"]
    for k, (du, dw) in enumerate(FOOT_POS[vid]):
        top = dish_y(v, min(v["rf"] + FOOT_W * 0.71, R * 0.98)) + 0.006
        out.append(boxs(f"{vid}-foot-{k + 1}", vid, "powder-red", 0.36, R + du - FOOT_W / 2, R + du + FOOT_W / 2, base, base + top, dw - FOOT_W / 2, dw + FOOT_W / 2))
    f = v["f"]
    if scale == 200:
        out.append(cyl(f"{vid}-connection", vid, "pe-cap", 0.3, [R, 0, 0], base + f - 0.030, base + f + 0.006, CAP_R))
    else:
        out.append(cyl(f"{vid}-nipple", vid, "galvanised-weathered", 0.55, [R, 0, 0], base + f - 0.018, base + f + 0.006, NIP_R))
        out.append(cyl(f"{vid}-cap", vid, "pe-cap", 0.3, [R, 0, 0], base + f - 0.030, base + f - 0.012, CAP_R))
        top = v["hc"] / 2 + v["hd"]
        out.append(cyl(f"{vid}-gas-cap", vid, "pa-cap", 0.15, [R, 0, 0], top - 0.004, top + 0.028, GAS_R))
    if vid == "vessel-35":
        if scale in (50, 100):
            ring = [[-CLIP_W / 2, R - 0.002], [CLIP_W / 2, R - 0.002], [CLIP_W / 2, R + CLIP_T - 0.002], [-CLIP_W / 2, R + CLIP_T - 0.002], [-CLIP_W / 2, R - 0.002]]
            out.append(solid("vessel-35-clip-strap", vid, "galvanised-strap", 0.6, "revolve", profile=[[r6(a), r6(b)] for a, b in ring], axis=[0, 1, 0], origin=[R, 0, 0], slender=True))
        out.append(boxs("vessel-35-clip-bracket", vid, "galvanised-strap", 0.58, -BRK[0] + 0.007, 0.007, -BRK[1] / 2, BRK[1] / 2, -BRK[2] / 2, BRK[2] / 2))
    return out


def bodies(scale):
    """🧊 The body solids of one scale."""
    out = cage_bodies(scale)
    for vid, v in VESSELS.items():
        out += vessel_bodies(scale, vid, v)
    return out


def thermals(scale, body):
    """🌡️ The thermal solids of one scale: every non-glazing body solid in its region."""
    out = []
    for s in body:
        if s["role"] == "glazing":
            continue
        out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "cage" if s["bone"] == "cage" else s["bone"]})
    return out


def structures(scale, body):
    """🏗️ The structural solids of one scale."""
    out = []
    for s in body:
        if s["role"] == "glazing" or s["id"].endswith(("-cap", "-nipple", "-gas-cap", "-connection", "-strap", "-bracket")):
            continue
        part = "joint" if "foot" in s["id"] else "member"
        out.append({"id": f"st-{s['id']}", "of": s["id"], "member": s["bone"], "part": part})
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
    """⚪ An ellipse geometry of equal radii."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rad), "ry": r6(rad)}


def poly(points, closed=False):
    """〰️ A polyline geometry."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in points], "closed": closed}


def cage_side(scale, view):
    """🧺 The cage seen from the side (elevation, x along) or from ahead (front, lateral z along)."""
    g, gr = "galvanised-weathered", "cage"
    lo, hi = (0.0, CL) if view == "elevation" else (-ZW, ZW)
    fw = FOOT_L
    out = []
    pw = 0.075 if scale == 500 else POST_W
    ty = CH - pw if scale == 500 else TOP_Y0
    if scale == 500:
        out.append(shape(f"plinth-{view}", "cage", "body", 1, True, 0.58, rect(lo, 0, hi - lo, RAIL_TOP), gr, g))
    else:
        for k, x0 in enumerate((lo, hi - fw)):
            out.append(shape(f"foot-{view}-{k + 1}", "cage", "body", 1, True, 0.55, rect(x0, 0, fw, FOOT_H), gr, g))
        out.append(shape(f"base-{view}", "cage", "body", 1.1, True, 0.6, rect(lo, FOOT_H, hi - lo, RAIL_TOP - FOOT_H), gr, g))
    out.append(shape(f"mesh-{view}", "cage", "glazing", 6, True, 0.6, rect(lo + pw, RAIL_TOP, hi - lo - 2 * pw, ty - RAIL_TOP), gr, "mesh-wire"))
    for k, x0 in enumerate((lo, hi - pw)):
        out.append(shape(f"post-{view}-{k + 1}", "cage", "body", 6.1, True, 0.62, rect(x0, RAIL_TOP, pw, ty - RAIL_TOP), gr, g))
    out.append(shape(f"top-{view}", "cage", "body", 6.2, True, 0.62, rect(lo, ty, hi - lo, CH - ty), gr, g))
    if scale in (50, 100):
        pitch = MESH_PITCH * (1 if scale == 50 else 2)
        n = int(round((hi - lo - 2 * pw) / pitch))
        for k in range(1, n):
            x = lo + pw + k * (hi - lo - 2 * pw) / n
            out.append(shape(f"wire-{view}-v{k}", "cage", "texture", 6.05, False, 0.35, poly([(x, RAIL_TOP), (x, ty)]), gr))
        m = int(round((ty - RAIL_TOP) / pitch))
        for k in range(1, m):
            y = RAIL_TOP + k * (ty - RAIL_TOP) / m
            out.append(shape(f"wire-{view}-h{k}", "cage", "texture", 6.05, False, 0.35, poly([(lo + pw, y), (hi - pw, y)]), gr))
    if view == "elevation" and scale != 500:
        fb = flap_bar(scale)
        out.append(shape("flap-hinge-bar", "cage", "detail", 6.3, True, 0.5, rect(XI0, FLAP_Y0, XI1 - XI0, fb), gr, g))
        out.append(shape("flap-top-bar", "cage", "detail", 6.3, True, 0.5, rect(XI0, FLAP_Y1 - fb, XI1 - XI0, fb), gr, g))
        if scale in (50, 100):
            out.append(shape("flap-side-rear", "cage", "detail", 6.3, True, 0.5, rect(XI0, FLAP_Y0 + FLAP_BAR, FLAP_BAR, FLAP_Y1 - FLAP_Y0 - 2 * FLAP_BAR), gr, g))
            out.append(shape("flap-side-front", "cage", "detail", 6.3, True, 0.5, rect(XI1 - FLAP_BAR, FLAP_Y0 + FLAP_BAR, FLAP_BAR, FLAP_Y1 - FLAP_Y0 - 2 * FLAP_BAR), gr, g))
        if scale == 50:
            for k, xh in enumerate((0.16, 0.62, 1.08)):
                out.append(shape(f"flap-hinge-{k + 1}", "cage", "detail", 6.4, True, 0.4, rect(xh - 0.03, FLAP_Y0 - 0.012, 0.06, 0.012 + FLAP_BAR), gr, g))
            for k, xl in enumerate((XI0 + FLAP_BAR, XI1 - FLAP_BAR - 0.05)):
                out.append(shape(f"flap-latch-{k + 1}", "cage", "detail", 6.4, True, 0.4, rect(xl, FLAP_Y1 - 0.07, 0.05, 0.025), gr, g))
            out.append(shape("mesh-dent", "cage", "texture", 6.45, False, 0.3, poly([(0.86, 0.3), (0.89, 0.27), (0.94, 0.265), (0.98, 0.29)]), gr))
            out.append(shape("rail-rust-1", "cage", "texture", 1.2, False, 0.3, poly([(0.2, 0.16), (0.24, 0.135), (0.31, 0.142)]), gr))
            out.append(shape("rail-rust-2", "cage", "texture", 1.2, False, 0.3, poly([(0.9, 0.12), (0.95, 0.15), (1.02, 0.138)]), gr))
            out.append(shape("foot-scrape", "cage", "texture", 1.2, False, 0.3, poly([(1.135, 0.02), (1.17, 0.05), (1.225, 0.06)]), gr))
    return out


def vessel_outline(v, n, pedestal=False):
    """🥚 The closed outline (u, v pairs about the axis at u = R) of a vessel shell."""
    prof = shell_profile(v, n, pedestal)
    right = [(v["R"] + r, a) for a, r in prof]
    left = [(v["R"] - r, a) for a, r in reversed(prof) if r > 0]
    return right + left


def vessel_side(scale, vid, v, view, zbase):
    """🔴 One vessel seen from the right (elevation, bone-local) or from ahead (front, rest coordinates)."""
    R, mat, gr = v["R"], "powder-red", vid
    if view == "elevation":
        X = lambda u: u
        Y = lambda w: w
    else:
        X = lambda u: v["cz"] + (u - R)
        Y = lambda w: RAIL_TOP + v["yc"] + w
    out = []
    base = -v["yc"]
    if scale != 500:
        feet = [(du, dw) for du, dw in FOOT_POS[vid]]
        for k, (du, dw) in enumerate(feet):
            off = du if view == "elevation" else -dw
            if (view == "front" and k == 2) or (view == "elevation" and scale == 200 and k == 0):
                continue
            top = dish_y(v, v["rf"]) + 0.004
            out.append(shape(f"{vid}-foot-{k + 1}", vid, "body", zbase, True, 0.36, rect(X(R + off - FOOT_W / 2), Y(base), FOOT_W, top), gr, mat))
        f = v["f"]
        if scale == 200:
            out.append(shape(f"{vid}-connection", vid, "body", zbase + 0.01, True, 0.3, rect(X(R - CAP_R), Y(base + f - 0.030), 2 * CAP_R, 0.034), gr, "pe-cap"))
        else:
            out.append(shape(f"{vid}-nipple", vid, "body", zbase + 0.01, True, 0.55, rect(X(R - NIP_R), Y(base + f - 0.018), 2 * NIP_R, 0.02), gr, "galvanised-weathered"))
            out.append(shape(f"{vid}-cap", vid, "body", zbase + 0.02, True, 0.3, rect(X(R - CAP_R), Y(base + f - 0.030), 2 * CAP_R, 0.018), gr, "pe-cap"))
            top = v["hc"] / 2 + v["hd"]
            out.append(shape(f"{vid}-gas-cap", vid, "body", zbase + 0.01, True, 0.15, rect(X(R - GAS_R), Y(top - 0.004), 2 * GAS_R, 0.032), gr, "pa-cap"))
    out.append(shape(f"{vid}-shell", vid, "body", zbase + 0.1, True, 0.42, poly([(X(a), Y(b)) for a, b in vessel_outline(v, SEG[scale], scale == 500)], True), gr, mat))
    if scale in (50, 100):
        for k, s in enumerate((-1, 1)):
            out.append(shape(f"{vid}-seam-{k + 1}", vid, "detail", zbase + 0.2, False, 0.3, poly([(X(0.004), Y(s * v["hc"] / 2)), (X(2 * R - 0.004), Y(s * v["hc"] / 2))]), gr))
    if view == "elevation" and scale in (50, 100):
        out.append(shape(f"{vid}-label", vid, "detail", zbase + 0.3, True, 0.92, rect(R - 0.07, -0.03, 0.14, 0.09), gr))
    if view == "elevation" and scale == 50:
        out.append(shape(f"{vid}-type-plate", vid, "detail", zbase + 0.3, True, 0.75, rect(R + 0.09, -v["hc"] / 2 + 0.03, 0.06, 0.035), gr))
        out.append(shape(f"{vid}-label-text-1", vid, "texture", zbase + 0.35, False, 0.4, poly([(R - 0.055, 0.035), (R + 0.03, 0.035)]), gr))
        out.append(shape(f"{vid}-label-text-2", vid, "texture", zbase + 0.35, False, 0.4, poly([(R - 0.055, 0.012), (R + 0.055, 0.012)]), gr))
        out.append(shape(f"{vid}-rust-1", vid, "texture", zbase + 0.25, False, 0.25, poly([(0.03, -v["hc"] / 2 - 0.02), (0.06, -v["hc"] / 2 - 0.045), (0.1, -v["hc"] / 2 - 0.06)]), gr))
        out.append(shape(f"{vid}-rust-2", vid, "texture", zbase + 0.25, False, 0.25, poly([(2 * R - 0.05, -v["hc"] / 2 + 0.01), (2 * R - 0.03, -v["hc"] / 2 - 0.02)]), gr))
        out.append(shape(f"{vid}-scuff", vid, "texture", zbase + 0.25, False, 0.3, poly([(0.06, v["hc"] / 2 - 0.05), (0.11, v["hc"] / 2 - 0.03), (0.17, v["hc"] / 2 - 0.035)]), gr))
    if vid == "vessel-35":
        if scale in (50, 100):
            out.append(shape("vessel-35-clip-strap", vid, "detail", zbase + 0.4, True, 0.6, rect(X(-0.003), Y(-CLIP_W / 2), 2 * R + 0.006, CLIP_W), gr, "galvanised-strap"))
        if scale != 500:
            if view == "elevation":
                out.append(shape("vessel-35-clip-bracket", vid, "body", zbase + 0.45, True, 0.58, rect(-BRK[0] + 0.007, -BRK[1] / 2, BRK[0], BRK[1]), gr, "galvanised-strap"))
                if scale == 50:
                    out.append(shape("vessel-35-keyhole", vid, "detail", zbase + 0.46, False, 0.3, poly([(-0.03, 0.02), (-0.03, -0.01)]), gr))
            else:
                out.append(shape("vessel-35-clip-bracket", vid, "body", zbase + 0.05, True, 0.58, rect(X(R - BRK[2] / 2), Y(-BRK[1] / 2), BRK[2], BRK[1]), gr, "galvanised-strap"))
    return out


def elevation(scale):
    """📐 Side elevation (seen from the right: the near long wall with the drop flap, the vessels behind its mesh)."""
    out = cage_side(scale, "elevation")
    order = sorted(VESSELS, key=lambda k: -VESSELS[k]["cz"])
    for k, vid in enumerate(order):
        out += vessel_side(scale, vid, VESSELS[vid], "elevation", 2 + k)
    return out


def front(scale):
    """🎯 Front elevation (seen from ahead: the front end wall, the vessels behind it)."""
    out = cage_side(scale, "front")
    order = sorted(VESSELS, key=lambda k: VESSELS[k]["cx"])
    for k, vid in enumerate(order):
        out += vessel_side(scale, vid, VESSELS[vid], "front", 2 + k)
    return out


def plan(scale):
    """🗺️ Plan (seen from above into the open cage: floor, vessel tops, top frame)."""
    g, gr = "galvanised-weathered", "cage"
    pw = 0.075 if scale == 500 else RAIL_W
    out = [shape("floor-plan", "cage", "body", 1, True, 0.66, rect(0, -ZW, CL, CW), gr, g)]
    out.append(shape("top-plan-l", "cage", "body", 8, True, 0.62, rect(0, ZW - pw, CL, pw), gr, g))
    out.append(shape("top-plan-r", "cage", "body", 8, True, 0.62, rect(0, -ZW, CL, pw), gr, g))
    out.append(shape("top-plan-rear", "cage", "body", 8, True, 0.62, rect(0, -ZW + pw, pw, CW - 2 * pw), gr, g))
    out.append(shape("top-plan-front", "cage", "body", 8, True, 0.62, rect(CL - pw, -ZW + pw, pw, CW - 2 * pw), gr, g))
    if scale == 50:
        out.append(shape("floor-corrugation-1", "cage", "texture", 1.1, False, 0.4, poly([(XI0, -0.2), (XI1, -0.2)]), gr))
        out.append(shape("floor-corrugation-2", "cage", "texture", 1.1, False, 0.4, poly([(XI0, 0.0), (XI1, 0.0)]), gr))
        out.append(shape("floor-corrugation-3", "cage", "texture", 1.1, False, 0.4, poly([(XI0, 0.2), (XI1, 0.2)]), gr))
        out.append(shape("floor-water-stain", "cage", "texture", 1.2, False, 0.3, poly([(0.55, -0.33), (0.6, -0.3), (0.66, -0.31), (0.7, -0.345)]), gr))
    for k, (vid, v) in enumerate(VESSELS.items()):
        zb = 2 + k
        if vid == "vessel-35" and scale != 500:
            if scale in (50, 100):
                out.append(shape("vessel-35-clip-strap", vid, "detail", zb - 0.05, True, 0.6, circ(v["cx"], v["cz"], v["R"] + CLIP_T - 0.002), vid, "galvanised-strap"))
            out.append(shape("vessel-35-clip-bracket", vid, "body", zb - 0.04, True, 0.58, rect(v["cx"] - v["R"] - BRK[0] + 0.007, v["cz"] - BRK[2] / 2, BRK[0], BRK[2]), vid, "galvanised-strap"))
        out.append(shape(f"{vid}-shell", vid, "body", zb, True, 0.42, circ(v["cx"], v["cz"], v["R"]), vid, "powder-red"))
        if scale in (50, 100):
            out.append(shape(f"{vid}-head-seam", vid, "detail", zb + 0.1, False, 0.3, circ(v["cx"], v["cz"], v["R"] * 0.9), vid))
            out.append(shape(f"{vid}-gas-cap", vid, "body", zb + 0.2, True, 0.15, circ(v["cx"], v["cz"], GAS_R), vid, "pa-cap"))
    return out


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": 0.1, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DECISIONS = {
    50: {
        "silhouette": t("Side elevation from the right: the near long wall of the cage pallet 1240 x 970 mm (24.8 x 19.4 mm on paper) with its four corner feet, the base frame, corner posts, top frame and the drop flap framed by 30 mm angles, the welded mesh drawn wire by wire at 50 mm, and behind it the three red vessels with their dished ends, three feet, capped R 3/4 nipples and gas caps; the 35 l vessel shows its wall clip bracket. Front: the end wall with the vessels behind it; plan: the open top with floor, top frame and the three vessel tops.", "Seitenansicht von rechts: die nahe Längswand der Gitterbox 1240 x 970 mm (24,8 x 19,4 mm auf Papier) mit ihren vier Eckfüßen, Bodenrahmen, Eckpfosten, Oberrahmen und der von 30-mm-Winkeln gerahmten Klappe, das geschweißte Gitter Draht für Draht im Raster von 50 mm, dahinter die drei roten Gefäße mit ihren Klöpperböden, drei Füßen, verschlossenen R-3/4-Nippeln und Gaskappen; das 35-l-Gefäß zeigt den Halter seiner Wandschelle. Vorderansicht: die Stirnwand mit den Gefäßen dahinter; Draufsicht: die offene Oberseite mit Boden, Oberrahmen und den drei Gefäßköpfen."),
        "retained": t("Pressure labels and type plates, the circumferential welds of the heads, rust at the lower welds, a scuff on the coating, the clip strap and its keyhole bracket, the flap hinges and latches, a dent in the mesh, rust on the base frame and a scraped foot, the corrugation and a water stain on the steel floor.", "Druckaufkleber und Typenschilder, die Rundnähte der Böden, Rost an den unteren Nähten, eine Schramme in der Beschichtung, Schellenband und Halter mit Schlüsselloch, Scharniere und Riegel der Klappe, eine Delle im Gitter, Rost am Bodenrahmen und ein abgeschabter Fuß, die Sicken und ein Wasserfleck auf dem Stahlboden."),
        "merged": t("Each corner foot is one pressed box; the 65 mm base frame channel and the floor are one band in the side views.", "Jeder Eckfuß ist ein gepresster Kasten; das 65-mm-U-Profil des Bodenrahmens und der Boden sind in den Seitenansichten ein Band."),
        "omitted": t("Thread turns, the Schrader valve core under the cap, the inner diaphragm (hidden) and the far mesh wall, which lies behind the near one.", "Gewindegänge, der Ventileinsatz unter der Kappe, die innere Membran (verdeckt) und die ferne Gitterwand, die hinter der nahen liegt."),
        "thinElements": t("The 4 mm mesh wires are texture strokes at 0.1 mm over the translucent mesh panel; the 2 mm floor sheet is shown in plan only.", "Die 4-mm-Gitterdrähte sind Texturstriche mit 0,1 mm über der durchscheinenden Gitterfläche; das 2-mm-Bodenblech erscheint nur in der Draufsicht."),
        "hatching": t("No hatch: a view of coated and galvanised steel; tone separates red coating, galvanised frame, caps and labels.", "Keine Schraffur: Ansicht von beschichtetem und verzinktem Stahl; der Ton trennt rote Beschichtung, verzinkten Rahmen, Kappen und Etiketten."),
        "styles": t("Outline 0.35 mm, flap frame, welds and labels 0.18 mm, wires, rust and scuffs 0.1 mm, mesh panel 0.13 mm.", "Kontur 0,35 mm, Klappenrahmen, Nähte und Etiketten 0,18 mm, Drähte, Rost und Schrammen 0,1 mm, Gitterfläche 0,13 mm."),
        "overlays": t("The cage parts are cells of the cage region; each vessel with its feet, nipple, caps and clip is the cell of its own region.", "Die Teile der Gitterbox sind Zellen der Gitterbox-Region; jedes Gefäß mit Füßen, Nippel, Kappen und Schelle ist die Zelle seiner eigenen Region."),
    },
    100: {
        "silhouette": t("The same cage at 12.4 x 9.7 mm with feet, frame, posts, the flap frame and the mesh drawn every second wire (100 mm); the three vessel outlines with feet, capped nipples, gas caps, head welds and labels.", "Dieselbe Gitterbox mit 12,4 x 9,7 mm mit Füßen, Rahmen, Pfosten, Klappenrahmen und dem Gitter mit jedem zweiten Draht (100 mm); die drei Gefäßkonturen mit Füßen, verschlossenen Nippeln, Gaskappen, Bodennähten und Etiketten."),
        "retained": t("Clip strap and bracket of the 35 l vessel, the labels, the flap frame.", "Schellenband und Halter des 35-l-Gefäßes, die Etiketten, der Klappenrahmen."),
        "merged": t("Two mesh wires of 1:50 are drawn as one stroke (100 mm pitch).", "Zwei Gitterdrähte von 1:50 sind als ein Strich gezeichnet (Raster 100 mm)."),
        "omitted": t("Type plates, rust, scuffs, hinges, latches, the mesh dent and the floor texture.", "Typenschilder, Rost, Schrammen, Scharniere, Riegel, die Gitterdelle und die Bodentextur."),
        "thinElements": t("The flap angles keep their 30 mm width; the wires are texture at 0.1 mm.", "Die Klappenwinkel behalten ihre Breite von 30 mm; die Drähte sind Textur mit 0,1 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm, details 0.13 mm, wires 0.1 mm.", "Kontur 0,25 mm, Details 0,13 mm, Drähte 0,1 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    200: {
        "silhouette": t("Cage outline of 6.2 x 4.85 mm with feet, base band, posts, top frame and the two long bars of the flap over the translucent mesh panel; the vessels as their outlines with feet and a capped connection.", "Gitterbox-Kontur von 6,2 x 4,85 mm mit Füßen, Bodenband, Pfosten, Oberrahmen und den beiden langen Stäben der Klappe über der durchscheinenden Gitterfläche; die Gefäße als Konturen mit Füßen und verschlossenem Anschluss."),
        "retained": t("The fork openings between the feet, the flap line and the 35 l wall clip bracket.", "Die Gabelöffnungen zwischen den Füßen, die Klappenlinie und der Halter der 35-l-Wandschelle."),
        "merged": t("Nipple and cap are one 34 mm block per vessel; the mesh is the panel tone without wires.", "Nippel und Kappe sind je Gefäß ein Block von 34 mm; das Gitter ist der Flächenton ohne Drähte."),
        "omitted": t("Gas caps (30 mm), head welds, labels, the flap side angles and the clip strap.", "Gaskappen (30 mm), Bodennähte, Etiketten, die seitlichen Klappenwinkel und das Schellenband."),
        "thinElements": t("None below the 30 mm feature.", "Keine unter der kleinsten Einzelheit von 30 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm, flap bars 0.1 mm.", "Kontur 0,18 mm, Klappenstäbe 0,1 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    500: {
        "silhouette": t("The cage as a box of 2.5 x 1.9 mm with four feet, a 75 mm base band, 75 mm corner posts and top frame and the translucent mesh panel; the vessels as three red outlines standing on their merged feet.", "Die Gitterbox als Kasten von 2,5 x 1,9 mm mit vier Füßen, einem Bodenband von 75 mm, Eckpfosten und Oberrahmen von 75 mm und der durchscheinenden Gitterfläche; die Gefäße als drei rote Konturen auf ihren zusammengefassten Füßen."),
        "retained": t("Overall size of the cage, the fork openings and the three vessel sizes.", "Gesamtgröße der Gitterbox, die Gabelöffnungen und die drei Gefäßgrößen."),
        "merged": t("Frame angles of 40 mm are drawn as 75 mm bands, the floor and base frame as one band, the three feet of each vessel as a pedestal of 0.8 times its diameter.", "Rahmenwinkel von 40 mm sind als Bänder von 75 mm gezeichnet, Boden und Bodenrahmen als ein Band, die drei Füße jedes Gefäßes als Sockel von 0,8-fachem Durchmesser."),
        "omitted": t("Flap, caps, connections, the wall clip and all traces of use.", "Klappe, Kappen, Anschlüsse, die Wandschelle und alle Gebrauchsspuren."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.13 mm, mesh panel 0.1 mm.", "Kontur 0,13 mm, Gitterfläche 0,1 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
}

VOLUME_DECISIONS = {
    50: [
        t("Body: the cage pallet after DIN 15155: four pressed corner feet 120 x 105 x 120 mm, the base frame of 65 x 40 mm channels with a middle bearer, a 2 mm galvanised floor sheet, corner posts and top frame of 40 mm angles, the drop flap framed by 30 mm angles on the near long side and the four mesh walls as 1 mm translucent glazing planes (the 4 mm wires are below the 7.5 mm feature); each vessel a revolved shell of cylinder and two elliptical heads, three 35 mm feet welded to the lower head, the R 3/4 nipple (26.9 mm) with its red PE cap and the black gas cap on top; the 35 l vessel keeps its 40 mm clip strap (a slender hoop) and the clip bracket.", "Körper: die Gitterbox nach DIN 15155: vier gepresste Eckfüße 120 x 105 x 120 mm, der Bodenrahmen aus U-Profilen 65 x 40 mm mit Mittelträger, ein verzinktes Bodenblech von 2 mm, Eckpfosten und Oberrahmen aus Winkeln 40 mm, die von 30-mm-Winkeln gerahmte Klappe auf der nahen Längsseite und die vier Gitterwände als durchscheinende Glasflächen von 1 mm (die 4-mm-Drähte liegen unter der kleinsten Einzelheit von 7,5 mm); jedes Gefäß ein gedrehter Mantel aus Zylinder und zwei elliptischen Böden, drei Füße von 35 mm am unteren Boden, der R-3/4-Nippel (26,9 mm) mit roter PE-Kappe und die schwarze Gaskappe oben; das 35-l-Gefäß behält sein Schellenband von 40 mm (ein schlanker Reif) und den Schellenhalter."),
        t("Thermal: the steel parts of the cage are one region, each vessel with its feet, nipple, caps and clip is its own region; the mesh planes are no thermal solids, so the vessels stay open to the air.", "Thermisch: die Stahlteile der Gitterbox sind eine Region, jedes Gefäß mit Füßen, Nippel, Kappen und Schelle ist eine eigene Region; die Gitterflächen sind keine thermischen Körper, damit die Gefäße der Luft offen bleiben."),
        t("Structure: the frame parts and the floor are members of the cage, the feet its joints; each vessel shell is the member of its bone and its feet the joints that stand on the floor sheet.", "Tragwerk: die Rahmenteile und der Boden sind Stäbe der Gitterbox, die Füße ihre Gelenke; jeder Gefäßmantel ist der Stab seines Knochens und seine Füße die Gelenke, die auf dem Bodenblech stehen."),
    ],
    100: [
        t("Body: as 1:50; the flap angles (32 x 32 mm, 1.16 m long) are slender, all other parts are above the 15 mm feature.", "Körper: wie 1:50; die Klappenwinkel (32 x 32 mm, 1,16 m lang) sind schlank, alle anderen Teile liegen über der kleinsten Einzelheit von 15 mm."),
        t("Thermal: as 1:50.", "Thermisch: wie 1:50."),
        t("Structure: as 1:50.", "Tragwerk: wie 1:50."),
    ],
    200: [
        t("Body: cage as 1:100 without the short flap side angles (335 mm, not slender); each vessel keeps shell, feet and one 34 mm connection block (nipple and cap merged); the gas caps (30 mm) and the clip strap are left out, the clip bracket stays.", "Körper: Gitterbox wie 1:100 ohne die kurzen seitlichen Klappenwinkel (335 mm, nicht schlank); jedes Gefäß behält Mantel, Füße und einen Anschlussblock von 34 mm (Nippel und Kappe zusammengefasst); Gaskappen (30 mm) und Schellenband entfallen, der Schellenhalter bleibt."),
        t("Thermal: as 1:50 with the merged parts.", "Thermisch: wie 1:50 mit den zusammengefassten Teilen."),
        t("Structure: as 1:50.", "Tragwerk: wie 1:50."),
    ],
    500: [
        t("Body: four feet 120 x 95 x 120 mm, the base frame and floor as one 75 mm plate, corner posts and top frame generalised to 75 mm bars, the four mesh planes; each vessel one revolved shell standing on a pedestal of 0.8 times its diameter that merges the three feet and the gap under the lower head.", "Körper: vier Füße 120 x 95 x 120 mm, Bodenrahmen und Boden als eine Platte von 75 mm, Eckpfosten und Oberrahmen auf Stäbe von 75 mm verallgemeinert, die vier Gitterflächen; jedes Gefäß ein gedrehter Mantel auf einem Sockel von 0,8-fachem Durchmesser, der die drei Füße und den Spalt unter dem unteren Boden zusammenfasst."),
        t("Thermal: cage parts and the three shells reuse their body solids.", "Thermisch: Gitterboxteile und die drei Mäntel verwenden ihre Körper."),
        t("Structure: the base plate, posts and top frame are the members of the cage, each shell the member of its vessel.", "Tragwerk: Bodenplatte, Pfosten und Oberrahmen sind die Stäbe der Gitterbox, jeder Mantel der Stab seines Gefäßes."),
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


REGION_IDS = ["cage"] + list(VESSELS)


def bind(sun, cage_exp, vessel_contact=None):
    """🌬️ Region bindings of one pose."""
    regions = {"cage": {"exposedFraction": cage_exp, "sunlitFraction": sun}}
    for vid in VESSELS:
        regions[vid] = {"exposedFraction": 0.98, "sunlitFraction": sun * 0.6}
        if vessel_contact:
            regions[vid]["contact"] = vessel_contact
    return regions


def water_loads(key):
    """💧 The weight of the water on the water side of every vessel, cold-filled or at service temperature."""
    out = []
    for vid, v in VESSELS.items():
        kg = v["w_hot"] if key == "hot" else v["w_cold"]
        out.append({"id": f"water-{vid}", "kind": "carried", "title": t(f"Water in the {NAMES[vid][0]} vessel, {kg:.1f} kg ({'hot' if key == 'hot' else 'cold'})", f"Wasser im Gefäß {NAMES[vid][1]}, {dez(f'{kg:.1f}')} kg ({'warm' if key == 'hot' else 'kalt'})"), "anchor": f"{vid}-water", "fxN": 0, "fyN": r6(-kg * G), "source": "literature", "reference": "din-4807-2"})
    return out


CAGE_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("cage-foot")]
FORK_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("fork")]
VESSEL_CONTACTS = [a["id"] for a in ANCHORS if "-foot-" in a["id"] and a["id"].startswith("vessel")]
OUT = {"transfer": {"dx": r6(TRANSFER_DX), "dy": r6(TRANSFER_DY)}}

POSES = [
    {"id": "stored", "title": t("Stored in the cage pallet", "In der Gitterbox gelagert"), "description": t("Drained, their water connections capped, the three vessels stand on their feet on the steel floor of a cage pallet in the covered yard of the component exchange.", "Entleert und mit verschlossenen Wasseranschlüssen stehen die drei Gefäße auf ihren Füßen auf dem Stahlboden einer Gitterbox im überdachten Hof der Bauteilbörse."), "bones": {}, "ground": "authored", "contacts": CAGE_CONTACTS, "thermal": {"operating": {"service": 0}, "regions": bind(0.2, 0.9), "emitters": {}}, "loads": []},
    {"id": "lifted", "title": t("Lifted on fork tines", "Auf Gabelzinken angehoben"), "description": t("A forklift has run its tines between the feet under the base frame and raised the cage pallet 300 mm with the vessels in it.", "Ein Gabelstapler hat seine Zinken zwischen den Füßen unter den Bodenrahmen geschoben und die Gitterbox mit den Gefäßen 300 mm angehoben."), "bones": {"carrier": {"dy": LIFT}}, "ground": "authored", "contacts": FORK_CONTACTS, "thermal": {"operating": {"service": 0}, "regions": bind(0.2, 0.95), "emitters": {}}, "loads": []},
    {"id": "reinstalled-cold", "title": t("Reinstalled, filled and cold", "Wieder eingebaut, befüllt und kalt"), "description": t("Taken out of the cage, the three vessels stand on a plant-room floor in front of it, reconnected and filled at 2.0 bar with the system still cold.", "Aus der Gitterbox genommen stehen die drei Gefäße davor auf einem Technikraumboden, wieder angeschlossen und bei kalter Anlage mit 2,0 bar befüllt."), "bones": OUT, "ground": "authored", "contacts": CAGE_CONTACTS + VESSEL_CONTACTS, "thermal": {"operating": {"service": 0}, "regions": bind(0, 0.9, {"fraction": 0.01, "tempC": 18}), "emitters": {}}, "loads": water_loads("cold")},
    {"id": "reinstalled", "title": t("Reinstalled and in service", "Wieder eingebaut und in Betrieb"), "description": t("The vessels in service: the heating water has expanded into them, the system stands at 2.5 bar and each holds its hot water reserve at about 30 degC on the water side.", "Die Gefäße im Betrieb: das Heizwasser hat sich in sie ausgedehnt, die Anlage steht bei 2,5 bar und jedes hält seine warme Wasservorlage von etwa 30 °C auf der Wasserseite."), "bones": OUT, "ground": "authored", "contacts": CAGE_CONTACTS + VESSEL_CONTACTS, "thermal": {"operating": {"service": 1}, "regions": bind(0, 0.9, {"fraction": 0.01, "tempC": 18}), "emitters": {}}, "loads": water_loads("hot")},
]

COLD_FRACTION = r6(VESSELS["vessel-80"]["w_cold"] / VESSELS["vessel-80"]["w_hot"])
CLIPS = [
    {"id": "lift", "title": t("Lift the cage pallet", "Gitterbox anheben"), "description": t("The fork tines take the cage pallet off the ground and raise it 300 mm in four seconds with the three vessels in it.", "Die Gabelzinken nehmen die Gitterbox vom Boden und heben sie mit den drei Gefäßen in vier Sekunden um 300 mm an."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1, "pose": "stored", "ease": "smooth"}, {"t": 5, "pose": "lifted", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 1} for a in CAGE_CONTACTS] + [{"anchor": a, "fromS": 1, "toS": 6} for a in FORK_CONTACTS],
     "phases": [{"id": "engage", "fromS": 0, "toS": 1, "title": t("Tines engage", "Zinken greifen")}, {"id": "raise", "fromS": 1, "toS": 5, "title": t("Raise", "Anheben")}, {"id": "hold", "fromS": 5, "toS": 6, "title": t("Hold", "Halten")}],
     "events": [{"t": 1, "kind": "contact-off", "title": t("Cage pallet leaves the ground", "Gitterbox verlässt den Boden")}], "tracks": []},
    {"id": "heat-up", "title": t("Heating up after reinstallation", "Aufheizen nach dem Wiedereinbau"), "description": t("The boiler heats the system from cold: the heating water expands, the system pressure rises from 2.0 to 2.5 bar and the water on the water side of every vessel grows to its hot reserve while the shells warm to about 30 degC.", "Der Kessel heizt die Anlage aus dem kalten Zustand auf: das Heizwasser dehnt sich aus, der Anlagendruck steigt von 2,0 auf 2,5 bar, das Wasser auf der Wasserseite jedes Gefäßes wächst auf seine warme Vorlage und die Mäntel erwärmen sich auf etwa 30 °C."), "durationS": 60, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled", "ease": "linear"}, {"t": 60, "pose": "reinstalled", "ease": "linear"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 60} for a in CAGE_CONTACTS + VESSEL_CONTACTS],
     "phases": [{"id": "heat", "fromS": 0, "toS": 50, "title": t("Heating up", "Aufheizen")}, {"id": "steady", "fromS": 50, "toS": 60, "title": t("Steady service", "Beharrungsbetrieb")}],
     "events": [{"t": 0, "kind": "operating", "title": t("Burner starts", "Brenner startet"), "target": "service"}, {"t": 50, "kind": "operating", "title": t("Design temperature reached", "Auslegungstemperatur erreicht"), "target": "service"}],
     "tracks": [{"target": f"load:water-{vid}", "ease": "linear", "keys": [{"t": 0, "v": COLD_FRACTION}, {"t": 50, "v": 1}, {"t": 60, "v": 1}]} for vid in VESSELS]
     + [{"target": "operating:service", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 50, "v": 1}, {"t": 60, "v": 1}]}]},
]

AREAS = {"cage": 4.6, "vessel-80": 0.96, "vessel-50": 0.71, "vessel-35": 0.56}


def vessel_region(vid, v):
    """🔴 The thermal region of one vessel: its coated shell with feet, nipple, caps and clip."""
    heat = round(H_COMB * (T_SERVICE - T_ROOM) * AREAS[vid], 1)
    r = {"id": vid, "title": t(f"Expansion vessel {NAMES[vid][0]} (coated shell, feet, capped nipple, gas cap)", f"Ausdehnungsgefäß {NAMES[vid][1]} (beschichteter Mantel, Füße, verschlossener Nippel, Gaskappe)"), "kind": "surface", "model": "operating", "material": "powder-red", "areaM2": AREAS[vid], "characteristicLengthM": r6(2 * v["R"]), "geometry": "cylinder", "skyViewFactor": 0.4,
         "layers": [{"id": f"{vid}-shell", "title": t(f"Steel shell {dez(T_SHELL * 1000)} mm, powder coated", f"Stahlmantel {dez(T_SHELL * 1000)} mm, pulverbeschichtet"), "kind": "metal", "material": "powder-red", "thicknessM": T_SHELL, "resistanceM2KW": r6(T_SHELL / 50), "heatCapacityJM2K": round(T_SHELL * RHO_STEEL * 450), "source": "literature", "reference": "en-iso-10456"}],
         "heatCapacityJM2K": round(T_SHELL * RHO_STEEL * 450), "operating": {"key": "service", "heatW": heat}, "source": "authored-estimate", "reference": "vdi-6030"}
    if vid == "vessel-80":
        r["ratedPowerW"] = 0
        r["ratedPowerBasis"] = "passive"
    return r


THERMAL = {
    "environment": {"airTempC": 10, "meanRadiantTempC": 8, "skyTempC": -4, "groundTempC": 9, "relativeHumidity": 0.75, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 200, "diffuseFraction": 0.7, "pressurePa": 101325},
    "passengers": [],
    "operating": [{"id": "service", "title": t("Heating system in service", "Heizungsanlage in Betrieb"), "levels": [{"v": 0, "title": t("Cold, filled at 2.0 bar", "Kalt, mit 2,0 bar befüllt")}, {"v": 0.5, "title": t("Heating up", "Aufheizen")}, {"v": 1, "title": t("At service temperature, 2.5 bar, shells about 30 degC", "Bei Betriebstemperatur, 2,5 bar, Mäntel etwa 30 °C")}]}],
    "regions": [
        {"id": "cage", "title": t("Cage pallet: galvanised frame, feet and floor", "Gitterbox: verzinkter Rahmen, Füße und Boden"), "kind": "surface", "model": "passive", "material": "galvanised-weathered", "areaM2": AREAS["cage"], "characteristicLengthM": 0.04, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(0.003 * RHO_STEEL * 450), "source": "authored-estimate", "reference": "din-15155"},
    ] + [vessel_region(vid, v) for vid, v in VESSELS.items()],
    "emitters": [],
    "assumptions": [
        t("An expansion vessel is fed from the heating system and draws no power: every region is passive with 0 W (stated on the 80 l region); the operating vessels only give off heat from their warm shells.", "Ein Ausdehnungsgefäß wird von der Heizungsanlage gespeist und nimmt keine Leistung auf: jede Region ist passiv mit 0 W (an der 80-l-Region angegeben); die Gefäße in Betrieb geben nur Wärme von ihren warmen Mänteln ab."),
        t(f"In service the water side of a vessel on the return line stands at about {T_SERVICE:.0f} degC (the stagnant water warms far less than the return); in an {T_ROOM:.0f} degC plant room the shell loses {H_COMB:.0f} W/m2K x {T_SERVICE - T_ROOM:.0f} K, that is {round(H_COMB * (T_SERVICE - T_ROOM) * AREAS['vessel-80'])} W, {round(H_COMB * (T_SERVICE - T_ROOM) * AREAS['vessel-50'])} W and {round(H_COMB * (T_SERVICE - T_ROOM) * AREAS['vessel-35'])} W for the 80, 50 and 35 l vessel.", f"Im Betrieb steht die Wasserseite eines Gefäßes im Rücklauf bei etwa {T_SERVICE:.0f} °C (das stehende Wasser erwärmt sich weit weniger als der Rücklauf); in einem Technikraum von {T_ROOM:.0f} °C verliert der Mantel {H_COMB:.0f} W/m²K x {T_SERVICE - T_ROOM:.0f} K, also {round(H_COMB * (T_SERVICE - T_ROOM) * AREAS['vessel-80'])} W, {round(H_COMB * (T_SERVICE - T_ROOM) * AREAS['vessel-50'])} W und {round(H_COMB * (T_SERVICE - T_ROOM) * AREAS['vessel-35'])} W beim 80-, 50- und 35-l-Gefäß."),
        t("Weathered surfaces: the red powder coating has faded (solar absorptance 0.62 instead of about 0.7 new, emissivity 0.9), the galvanised cage is dull grey (emissivity 0.28 instead of 0.05 to 0.1 when new).", "Bewitterte Oberflächen: die rote Pulverbeschichtung ist verblasst (solare Absorption 0,62 statt etwa 0,7 neu, Emissionsgrad 0,9), die verzinkte Gitterbox ist mattgrau (Emissionsgrad 0,28 statt 0,05 bis 0,1 im Neuzustand)."),
        t(f"The 4 mm wires of the mesh walls ({CAGE_PARTS['mesh']:.1f} kg, about 1.8 m2 of wire surface) are not part of the cage region: they are below the smallest feature at every scale and stand at air temperature; the inner EPDM diaphragm of each vessel is hidden and not a region.", f"Die 4-mm-Drähte der Gitterwände ({dez(f'{MESH_KG:.1f}')} kg, etwa 1,8 m² Drahtoberfläche) gehören nicht zur Gitterbox-Region: sie liegen bei jedem Maßstab unter der kleinsten Einzelheit und stehen auf Lufttemperatur; die innere EPDM-Membran jedes Gefäßes ist verdeckt und keine Region."),
        t("Reference case: an overcast autumn day in the covered yard; the reinstalled poses bind no sun and an 18 degC plant-room floor under the feet.", "Referenzfall: ein bedeckter Herbsttag im überdachten Hof; die eingebauten Posen binden keine Sonne und einen Technikraumboden von 18 °C unter den Füßen."),
    ],
}

STRUCTURE = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
    t("Supports per pose: stored, the four cage feet on the ground; lifted, four contacts on the two fork tines under the base frame (ISO 2328 class 2A, 440 mm apart); reinstalled, the four cage feet and the three feet of every vessel on the floor. Each pose is solved as one rigid body.", "Auflager je Pose: gelagert die vier Füße der Gitterbox auf dem Boden; angehoben vier Kontakte auf den beiden Gabelzinken unter dem Bodenrahmen (ISO 2328 Klasse 2A, 440 mm Abstand); eingebaut die vier Füße der Gitterbox und die drei Füße jedes Gefäßes auf dem Boden. Jede Pose wird als ein starrer Körper gelöst."),
    t(f"Masses from sheet areas, section tables and densities: cage pallet {CAGE_KG} kg (DIN 15155 tare 70 to 85 kg); vessels {VESSELS['vessel-80']['kg']}, {VESSELS['vessel-50']['kg']} and {VESSELS['vessel-35']['kg']} kg (shell {dez(T_SHELL * 1000)} mm steel, EPDM diaphragm, feet and fittings); total {MASS} kg drained.", f"Massen aus Blechflächen, Profiltabellen und Dichten: Gitterbox {dez(CAGE_KG)} kg (DIN 15155 Eigengewicht 70 bis 85 kg); Gefäße {dez(VESSELS['vessel-80']['kg'])}, {dez(VESSELS['vessel-50']['kg'])} und {dez(VESSELS['vessel-35']['kg'])} kg (Mantel {dez(T_SHELL * 1000)} mm Stahl, EPDM-Membran, Füße und Anschlüsse); zusammen {dez(MASS)} kg entleert."),
    t(f"Water side after DIN 4807-2 with pre-charge {P0} bar: filled cold at {PA} bar each vessel holds {100 * (1 - (P0 + 1) / (PA + 1)):.1f} % of its volume as water, in service at {PE} bar {100 * (1 - (P0 + 1) / (PE + 1)):.1f} % ({VESSELS['vessel-80']['w_hot']:.1f}, {VESSELS['vessel-50']['w_hot']:.1f} and {VESSELS['vessel-35']['w_hot']:.1f} kg); the weight is a point load at the centre of the lower half.", f"Wasserseite nach DIN 4807-2 mit Vordruck {dez(P0)} bar: kalt mit {dez(PA)} bar befüllt hält jedes Gefäß {dez(f'{100 * (1 - (P0 + 1) / (PA + 1)):.1f}')} % seines Volumens als Wasser, im Betrieb bei {dez(PE)} bar {dez(f'{100 * (1 - (P0 + 1) / (PE + 1)):.1f}')} % ({dez(f'{W80:.1f}')}, {dez(f'{W50:.1f}')} und {dez(f'{W35:.1f}')} kg); das Gewicht ist eine Einzellast in der Mitte der unteren Hälfte."),
], "clinical": False}

HEIGHT, WIDTH, DEPTH = CH, CL, CW

ASSET = {
    "id": SLUG,
    "category": "components",
    "title": t("Used Diaphragm Expansion Vessels", "Gebrauchte Membran-Ausdehnungsgefäße"),
    "summary": t("Three used red diaphragm expansion vessels for closed heating systems, 80, 50 and 35 l, 6 bar, pre-charged to 1.5 bar, with dished ends, three welded feet, a capped R 3/4 water connection and a gas cap valve, the smallest still wearing its wall clip; drained, they stand in a galvanised DIN 15155 cage pallet, are lifted with it on fork tines and stand reinstalled on a plant-room floor, filled and in service.", "Drei gebrauchte rote Membran-Ausdehnungsgefäße für geschlossene Heizungsanlagen, 80, 50 und 35 l, 6 bar, mit 1,5 bar vorgespannt, mit Klöpperböden, drei angeschweißten Füßen, verschlossenem R-3/4-Wasseranschluss und Gasfüllventil, das kleinste noch mit seiner Wandschelle; entleert stehen sie in einer verzinkten Gitterbox nach DIN 15155, werden mit ihr auf Gabelzinken angehoben und stehen wieder eingebaut, befüllt und in Betrieb auf einem Technikraumboden."),
    "tags": ["component", "reclaimed", "used", "hvac", "heating", "plumbing", "steel", "pressure-vessel", "expansion-vessel", "diaphragm", "cage-pallet", "storage"],
    "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
        f"Typical red heating expansion vessels of 6 bar: 80 l {VESSELS['vessel-80']['R'] * 2000:.0f} x {VESSELS['vessel-80']['H'] * 1000:.0f} mm, 50 l {VESSELS['vessel-50']['R'] * 2000:.0f} x {VESSELS['vessel-50']['H'] * 1000:.0f} mm, 35 l {VESSELS['vessel-35']['R'] * 2000:.0f} x {VESSELS['vessel-35']['H'] * 1000:.0f} mm, in a DIN 15155 cage pallet 1240 x 835 x 970 mm; drained mass {MASS} kg. All dimensions and masses are catalogue-typical (authored-estimate), nothing is measured.",
        f"Typische rote Heizungs-Ausdehnungsgefäße für 6 bar: 80 l {VESSELS['vessel-80']['R'] * 2000:.0f} x {VESSELS['vessel-80']['H'] * 1000:.0f} mm, 50 l {VESSELS['vessel-50']['R'] * 2000:.0f} x {VESSELS['vessel-50']['H'] * 1000:.0f} mm, 35 l {VESSELS['vessel-35']['R'] * 2000:.0f} x {VESSELS['vessel-35']['H'] * 1000:.0f} mm, in einer Gitterbox nach DIN 15155 von 1240 x 835 x 970 mm; Masse entleert {dez(MASS)} kg. Alle Maße und Massen sind katalogtypisch (authored-estimate), nichts ist gemessen.")},
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
    print(f"[DEBUG] wrote {SLUG}: mass {MASS} kg, cage {CAGE_KG} kg, " + ", ".join(f"{k} {v['kg']} kg {v['vol']:.1f} l H {v['H']:.3f} A {v['area']:.3f} hot {v['w_hot']:.1f}" for k, v in VESSELS.items()))
    print("[DEBUG] cage parts " + ", ".join(f"{k} {x:.2f}" for k, x in CAGE_PARTS.items()))
