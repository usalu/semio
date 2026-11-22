"""🍳 Authoritative author script of the used 50 cm electric cooker with oven on its half pallet: rig, anchors, materials, thermal and structural model, poses, clips and the assembly of the document; geometry in author_geometry.py, drawings in author_drawings.py.

Run: `python3 author-used-electric-cooker.py` (writes `🔣️.json` next to it and prints the mass and stability table of the report).

See also ../../brief.md and ../../reports/used-electric-cooker.md.
"""
import json
import os

from author_drawings import UNAVAILABLE_500, representation, symbol
from author_geometry import (
    BODY, BOX_X0, CAV_Y0, CAV_Y1, CAV_Z, COOKER_KG, DRAWER_X1, DRAWER_Y1, FASCIA_X1, FASCIA_Y0, FOOT_H, X_SIDE, DOOR_H, DOOR_T, DRAWER_X0, FEET, FOOT_R, HINGE_X, HINGE_Y, MASS, PALLET_H, PALLET_L, PALLET_X0, PALLET_Z, PLATES, PLATE_TOP,
    STRINGER_Y1, TOP_BOARDS, TOP_Y1, TOTAL, TRAY_X0, TRAY_X1, TRAY_Y0, T, ZO, box, clean, extrude, r6,
)

HERE = os.path.dirname(os.path.abspath(__file__))
G = 9.80665
LIFT, CARRY, TILT = 0.20, 0.35, 4.0
USE_SHIFT = 0.75
DOOR_OPEN = -90.0
TRAY_PULL = 0.25
CAKE_KG = 2.0
DOOR_TEST_KG = 22.5

MATERIALS = [
    {"id": "enamel-white-aged", "title": T("White vitreous enamel on 0.7 mm steel sheet, yellowed and dulled by about 20 years of kitchen use", "Weiße Emaille auf 0,7-mm-Stahlblech, durch etwa 20 Jahre Küchengebrauch vergilbt und matt"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 200000000000.0, "strengthPa": 180000000.0, "source": "authored-estimate", "reference": "en-10209-2013"},
    {"id": "enamel-hob-stained", "title": T("Enamelled hob plate with burnt-on stains and a chipped front edge", "Emailliertes Muldenblech mit eingebrannten Flecken und abgeplatzter Vorderkante"), "emissivity": 0.92, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 200000000000.0, "strengthPa": 180000000.0, "source": "authored-estimate", "reference": "en-10209-2013"},
    {"id": "enamel-oven-dark", "title": T("Dark blue-grey oven enamel on 1 mm steel (muffle, rails, baking tray)", "Dunkle blaugraue Backofenemaille auf 1-mm-Stahl (Muffel, Einschubleisten, Backblech)"), "emissivity": 0.9, "solarAbsorptivity": 0.85, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 200000000000.0, "strengthPa": 180000000.0, "source": "literature", "reference": "incropera-2018"},
    {"id": "steel-galvanised", "title": T("Galvanised steel sheet 0.6 to 0.8 mm (back, base, drawer box, runners)", "Verzinktes Stahlblech 0,6 bis 0,8 mm (Rückwand, Boden, Schubkasten, Laufschienen)"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 200000000000.0, "strengthPa": 220000000.0, "source": "literature", "reference": "incropera-2018"},
    {"id": "cast-iron-aged", "title": T("Cast-iron hotplate with a worn black protective coating and rust spots", "Gusseiserne Kochplatte mit abgenutztem schwarzem Schutzanstrich und Rostflecken"), "emissivity": 0.88, "solarAbsorptivity": 0.9, "densityKgM3": 7200, "conductivityWmK": 50, "specificHeatJKgK": 500, "elasticModulusPa": 110000000000.0, "strengthPa": 200000000.0, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "stainless-dulled", "title": T("Stainless trim ring, dulled and heat-tinted", "Edelstahl-Abdeckring, matt und angelaufen"), "emissivity": 0.35, "solarAbsorptivity": 0.5, "densityKgM3": 7900, "conductivityWmK": 15, "specificHeatJKgK": 500, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "door-glass", "title": T("Toughened soda-lime oven door glass 4 mm, scratched and with a grease film", "Vorgespanntes Kalk-Natron-Türglas 4 mm, verkratzt und mit Fettfilm"), "emissivity": 0.9, "solarAbsorptivity": 0.15, "densityKgM3": 2500, "conductivityWmK": 1.0, "specificHeatJKgK": 840, "elasticModulusPa": 70000000000.0, "strengthPa": 120000000.0, "source": "literature", "reference": "incropera-2018"},
    {"id": "chrome-steel", "title": T("Chromed steel tube handle 20 x 1 mm", "Verchromter Stahlrohrgriff 20 x 1 mm"), "emissivity": 0.1, "solarAbsorptivity": 0.4, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "incropera-2018"},
    {"id": "abs-knob-aged", "title": T("White ABS control knob, yellowed", "Weißer ABS-Bedienknebel, vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "pp-black", "title": T("Black polypropylene (replacement knob, adjustable feet)", "Schwarzes Polypropylen (Ersatzknebel, Stellfüße)"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 900, "conductivityWmK": 0.22, "specificHeatJKgK": 1800, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "mineral-wool", "title": T("Mineral wool oven insulation 40 mm with aluminium foil", "Mineralwolle-Backofendämmung 40 mm mit Aluminiumfolie"), "emissivity": 0.1, "densityKgM3": 60, "conductivityWmK": 0.04, "specificHeatJKgK": 840, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "switchgear", "title": T("Energy regulators, oven thermostat, terminal block and wiring as one bulk", "Energieregler, Backofenthermostat, Anschlussklemme und Verdrahtung als ein Block"), "emissivity": 0.9, "densityKgM3": 1000, "source": "authored-estimate", "reference": "cooker-catalogue"},
    {"id": "tubular-element", "title": T("Tubular heating element, Incoloy sheath on magnesia, 8 mm", "Rohrheizkörper, Incoloy-Mantel auf Magnesia, 8 mm"), "emissivity": 0.8, "densityKgM3": 5500, "conductivityWmK": 15, "specificHeatJKgK": 500, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "pvc-cable", "title": T("PVC sheathed mains cable H05VV-F 5G1.5, coiled and taped", "PVC-Mantelleitung H05VV-F 5G1,5, aufgerollt und abgeklebt"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 1000, "conductivityWmK": 0.17, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "spruce-pallet", "title": T("Sawn spruce of a used half pallet, greyed and dirty", "Sägeraue Fichte einer gebrauchten Halbpalette, vergraut und verschmutzt"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "literature", "reference": "en-338-2016"},
]

REFERENCES = [
    {"id": "en-60335-2-6", "citation": "EN 60335-2-6:2015 + A11:2020 Household and similar electrical appliances - Safety - Part 2-6: Particular requirements for stationary cooking ranges, hobs, ovens and similar appliances (clause 20: stability of free-standing cookers, also with the oven door open and loaded)."},
    {"id": "en-60350-1", "citation": "EN 60350-1:2016 Household electric cooking appliances - Part 1: Ranges, ovens, steam ovens and grills - Methods for measuring performance (usable cavity volume, preheating time, energy consumption)."},
    {"id": "en-60350-2", "citation": "EN 60350-2:2018 Household electric cooking appliances - Part 2: Hobs - Methods for measuring performance (cooking zones and their diameters)."},
    {"id": "din-18022-2007", "citation": "DIN 18022:2007-11 Küche, Bad und WC im Wohnungsbau - Planungsgrundlagen (cooker placing area 50 or 60 cm wide and 60 cm deep, work height 85 to 92 cm)."},
    {"id": "en-10209-2013", "citation": "EN 10209:2013 Cold rolled low carbon steel flat products for vitreous enamelling (DC04EK, yield strength 140-220 MPa, sheet 0.6-1.0 mm)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018) Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11: emissivities of oxidised cast iron about 0.9, galvanised steel about 0.28, polished chrome 0.1, glass 0.9; Table A.1: steel and cast iron)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (mineral wool 0.04 W/mK, softwood 0.13 W/mK and 1600 J/kgK, polymers)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (spruce C24: bending strength 24 MPa, mean modulus 11 GPa, mean density 420-450 kg/m3)."},
    {"id": "en-13698-1-2003", "citation": "EN 13698-1:2003 Pallet production specification - Part 1: Construction specification for 800 mm x 1200 mm flat wooden pallets (22 mm boards, 78 mm blocks, 144 mm height); the 600 x 800 mm half pallet follows the same build."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - General actions, densities, self-weight, imposed loads for buildings (gravity 9.80665 m/s2)."},
    {"id": "cooker-catalogue", "citation": "Catalogue-typical 50 cm free-standing electric cookers with four cast-iron hotplates (Standherd mit Gusskochplatten) of German mid-range makers 1990-2010, taken as the type without brand: 50 x 60 x 85 cm, hotplates 2 x 145 mm (1.0 and 1.5 kW rapid) and 2 x 180 mm (1.5 and 2.0 kW), conventional oven of about 55 l with top and bottom heat 2.0 kW, connected load about 8 kW at 400 V 3N, appliance drawer, 38-45 kg."},
    {"id": "half-pallet", "citation": "Catalogue-typical exchange half pallet (Halbpalette) 600 x 800 x 144 mm of spruce, 9.5-12 kg, as used by reuse depots for white goods."},
    {"id": "semio-used-household", "citation": "Semio staffage content, used household objects collection (2026-11-22): traces of use, storage state and carrying of a reuse-depot object are authored estimates of the typical case; nothing is measured."},
]

PLATE_KW = {p[0]: p[5] for p in PLATES}
HOB_W = sum(PLATE_KW.values())
OVEN_W = 2000
OVEN_HEAT = {"casing": 760, "hob-top": 250, "oven-door": 450, "door-window": 150, "fascia": 40, "drawer": 60}
VENT_FLOW, VENT_DT = 0.004, 60.0
VENT_W = 1.2 * 1005 * VENT_FLOW * VENT_DT

BONES = [
    {"id": "floor", "parent": None, "title": T("Floor of the depot hall", "Boden der Lagerhalle"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "pallet", "parent": "floor", "title": T("Half pallet (stays on the floor)", "Halbpalette (bleibt am Boden)"), "x": PALLET_X0, "y": 0, "angleDeg": 0, "lengthM": PALLET_L, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.0, "maxM": 1.0}, "mass": {"kg": r6(MASS["pallet"][0]), "comX": r6(MASS["pallet"][1]), "comY": r6(MASS["pallet"][2]), "distribution": "point", "source": "authored-estimate", "reference": "half-pallet"}, "section": {"areaM2": 0.0066, "secondMomentM4": 2.662e-07, "extremeFibreM": 0.011, "material": "spruce-pallet", "title": T("Three stringer boards 100 x 22 mm", "Drei Querbretter 100 x 22 mm")}},
    {"id": "pallet-deck", "parent": "pallet", "title": T("Top deck boards of the half pallet", "Deckbretter der Halbpalette"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": PALLET_L, "joint": {"type": "fixed"}, "mass": {"kg": r6(MASS["pallet-deck"][0]), "comX": r6(MASS["pallet-deck"][1]), "comY": r6(MASS["pallet-deck"][2]), "distribution": "point", "source": "authored-estimate", "reference": "half-pallet"}, "section": {"areaM2": 0.0106, "secondMomentM4": 4.28e-07, "extremeFibreM": 0.011, "material": "spruce-pallet", "title": T("Four top boards 22 mm, 480 mm wide together", "Vier Deckbretter 22 mm, zusammen 480 mm breit")}},
    {"id": "cooker", "parent": "floor", "title": T("Standing level of the cooker (on the pallet or on the floor)", "Standhöhe des Herds (auf der Palette oder am Boden)"), "x": 0, "y": PALLET_H, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -PALLET_H, "maxM": 0.5}},
    {"id": "cooker-carry", "parent": "cooker", "title": T("Carrying travel of the cooker", "Tragweg des Herds"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.2, "maxM": 1.0}},
    {"id": "carcass", "parent": "cooker-carry", "title": T("Carcass: side panels, back, base, hob, fascia, oven muffle, hotplates, knobs", "Korpus: Seitenwände, Rückwand, Boden, Mulde, Blende, Backofenmuffel, Kochplatten, Knebel"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": DRAWER_X0, "joint": {"type": "revolute", "minDeg": -15, "maxDeg": 15}, "mass": {"kg": r6(MASS["carcass"][0]), "comX": r6(MASS["carcass"][1]), "comY": r6(MASS["carcass"][2]), "distribution": "point", "source": "authored-estimate", "reference": "cooker-catalogue"}, "section": {"areaM2": 0.000784, "secondMomentM4": 2.0487e-05, "extremeFibreM": 0.28, "material": "enamel-white-aged", "title": T("Two enamelled side panels 0.7 x 560 mm", "Zwei emaillierte Seitenwände 0,7 x 560 mm")}},
    {"id": "door", "parent": "carcass", "title": T("Oven door on its bottom hinges", "Backofentür an ihren unteren Scharnieren"), "x": HINGE_X, "y": HINGE_Y, "angleDeg": 90, "lengthM": DOOR_H, "joint": {"type": "revolute", "minDeg": -90, "maxDeg": 0}, "mass": {"kg": r6(MASS["door"][0]), "comX": r6(MASS["door"][1]), "comY": r6(MASS["door"][2]), "distribution": "point", "source": "authored-estimate", "reference": "cooker-catalogue"}, "section": {"areaM2": 0.00079, "secondMomentM4": 3.16e-07, "extremeFibreM": 0.02, "material": "enamel-white-aged", "title": T("Outer and inner door sheet 0.8 mm, 40 mm apart", "Äußeres und inneres Türblech 0,8 mm, 40 mm auseinander")}},
    {"id": "drawer", "parent": "carcass", "title": T("Appliance drawer on its runners", "Geräteschublade auf ihren Laufschienen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": DRAWER_X0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.40}, "mass": {"kg": r6(MASS["drawer"][0]), "comX": r6(MASS["drawer"][1]), "comY": r6(MASS["drawer"][2]), "distribution": "point", "source": "authored-estimate", "reference": "cooker-catalogue"}, "section": {"areaM2": 0.000144, "secondMomentM4": 1.728e-07, "extremeFibreM": 0.06, "material": "steel-galvanised", "title": T("Two drawer sides 0.6 x 120 mm", "Zwei Schubkastenseiten 0,6 x 120 mm")}},
    {"id": "tray", "parent": "carcass", "title": T("Baking tray on the middle oven rails", "Backblech auf den mittleren Einschubleisten"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": TRAY_X1, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.30}, "mass": {"kg": r6(MASS["tray"][0]), "comX": r6(MASS["tray"][1]), "comY": r6(MASS["tray"][2]), "distribution": "point", "source": "authored-estimate", "reference": "cooker-catalogue"}, "section": {"areaM2": 0.00035, "secondMomentM4": 2.08e-09, "extremeFibreM": 0.0125, "material": "enamel-oven-dark", "title": T("Tray bottom with two 25 mm rims, 0.8 mm", "Blechboden mit zwei 25-mm-Rändern, 0,8 mm")}},
]

ANCHORS = [
    {"id": "pallet-floor", "bone": "pallet", "x": 0.30, "y": 0.0, "kind": "contact", "title": T("Half pallet on the floor", "Halbpalette auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": PALLET_L, "footprintWidthM": 2 * PALLET_Z, "frictionCoefficient": 0.5},
]
FOOT_IDS = []
for x, z in FEET:
    fid = f"foot-{'front' if x > 0.3 else 'back'}-{'l' if z > 0 else 'r'}"
    FOOT_IDS.append(fid)
    ANCHORS.append({"id": fid, "bone": "carcass", "x": x, "y": 0.0, "z": z, "kind": "contact", "title": T(f"Adjustable foot, {'front' if x > 0.3 else 'back'} {'left' if z > 0 else 'right'}", f"Stellfuß, {'vorn' if x > 0.3 else 'hinten'} {'links' if z > 0 else 'rechts'}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 2 * FOOT_R, "footprintWidthM": 2 * FOOT_R, "frictionCoefficient": 0.5})
GRIPS = []
for x, where in ((0.08, ("back", "hinten")), (0.48, ("front", "vorn"))):
    for z, side in ((-ZO, ("right", "rechts")), (ZO, ("left", "links"))):
        gid = f"grip-{where[0]}-{side[0][0]}"
        GRIPS.append(gid)
        ANCHORS.append({"id": gid, "bone": "carcass", "x": x, "y": 0.015, "z": z, "kind": "support", "title": T(f"Carrier {side[0]}: hand under the base, {where[0]}", f"Träger {side[1]}: Hand unter dem Boden, {where[1]}"), "support": "pin"})
ANCHORS += [
    {"id": "anti-tilt", "bone": "carcass", "x": 0.0, "y": 0.04, "kind": "support", "title": T("Anti-tilt bracket of the kitchen holding the rear base", "Kippschutzwinkel der Küche, der den hinteren Boden hält"), "support": "pin"},
    {"id": "oven-vent", "bone": "carcass", "x": 0.035, "y": 0.852, "kind": "emitter", "title": T("Oven vapour outlet at the rear of the hob", "Wrasenauslass des Backofens hinten an der Mulde"), "normalDeg": 90},
    {"id": "hob-centre", "bone": "carcass", "x": 0.41, "y": 0.859, "z": 0.12, "kind": "emitter", "title": T("Above the front left hotplate", "Über der Kochplatte vorn links"), "normalDeg": 90},
]


def region_of(scale, sid):
    """🌡️ The thermal region of a body solid at a scale, None for the inside parts."""
    if sid.startswith("pallet"):
        return "pallet"
    if sid.startswith(("plate-", "ring-")):
        return "hotplates"
    if sid in ("top-plate", "hob-slab"):
        return "hob-top"
    if sid == "fascia" or sid.startswith("knob-"):
        return "fascia"
    if sid in ("door-frame", "door-slab", "door-face", "handle-bar", "handle-block") or sid.startswith("standoff"):
        return "oven-door"
    if sid == "glass-outer":
        return "door-window"
    if sid == "drawer-front" or (scale == "200" and sid == "drawer-block"):
        return "drawer"
    if sid in ("side-r", "side-l", "back", "base", "front-frame", "cable-coil") or sid.startswith(("foot-", "carcass-")):
        return "casing"
    return None


EMIT = {"50": (0.008, 0.08), "100": (0.015, 0.09), "200": (0.03, 0.10)}


def emitter_solids(scale):
    """💨 The outlet patches of the two emitters, one smallest feature thick and set half into their surface: the oven vapour slot at the rear of the hob and a patch on the front left plate for its plume."""
    h, a = EMIT[scale]
    vx = (0.02, 0.05) if scale != "200" else (0.015, 0.055)
    vent = box("th-oven-vent", "carcass", "enamel-hob-stained", 0.3, vx[0], vx[1], TOP_Y1 - h / 2, TOP_Y1 + h / 2, -0.15, 0.15, role="detail")
    plume = box("th-hob-plume", "carcass", "cast-iron-aged", 0.12, 0.41 - a / 2, 0.41 + a / 2, PLATE_TOP - h / 2, PLATE_TOP + h / 2, 0.12 - a / 2, 0.12 + a / 2, role="detail")
    return [
        {**{k: v for k, v in vent.items() if not k.startswith("_")}, "region": "hob-top", "tags": ["oven-vent"]},
        {**{k: v for k, v in plume.items() if not k.startswith("_")}, "region": "hotplates", "tags": ["hob-plume"]},
    ]


def shell_solids(scale):
    """🫙 The casing of a coarse scale as the thermal model sees it: a closed shell of side, top and bottom walls, a back wall and the front plate round the oven opening, one smallest feature thick, with the fascia and (1:200) the drawer front set between its walls, so the faces inside the closed casing are no outer area although the body list models the casing as solid masses."""
    t = {"100": 0.015, "200": 0.03}[scale]
    y0 = FOOT_H if scale == "100" else 0.0
    top = {"100": 0.835, "200": 0.82}[scale]
    zi = ZO - t
    ring = lambda a0, a1, b0, b1: [(b, a) for a, b in [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]]
    out = [
        extrude("th-shell-walls", "carcass", "enamel-white-aged", 0.88, ring(-ZO, ZO, y0, top), "yz", t, X_SIDE, holes=[ring(-zi, zi, y0 + t, top - t)]),
        box("th-shell-back", "carcass", "enamel-white-aged", 0.88, 0.0, t, y0, top, -ZO, ZO),
        extrude("th-shell-front", "carcass", "enamel-white-aged", 0.88, ring(-zi, zi, HINGE_Y, FASCIA_Y0), "yz", X_SIDE - t, X_SIDE, holes=[ring(-CAV_Z, CAV_Z, CAV_Y0, CAV_Y1)]),
        box("th-shell-fascia", "carcass", "enamel-white-aged", 0.86, FASCIA_X1 - (0.02 if scale == "100" else 0.03), FASCIA_X1, FASCIA_Y0, top - t, -zi, zi),
    ]
    regs = ["casing", "casing", "casing", "fascia"]
    if scale == "200":
        out.append(box("th-shell-drawer", "drawer", "enamel-white-aged", 0.86, 0.55, DRAWER_X1, t, DRAWER_Y1, -zi, zi))
        regs.append("drawer")
    return [{**{k: v for k, v in s.items() if not k.startswith("_")}, "region": r} for s, r in zip(out, regs)]


SHELL_REPLACES = {"100": {"carcass-back", "carcass-rear", "carcass-bay", "carcass-oven", "carcass-upper", "fascia"}, "200": {"carcass-low", "carcass-back", "carcass-oven", "carcass-upper", "fascia", "drawer-front", "drawer-block"}}


def thermal_list(scale):
    """🔥 Thermal list: references of every exposed body solid with its region, so at 1:50 the sheet casing, the closed oven and the door close themselves; at 1:100 and 1:200 the solid casing masses are replaced by a closed shell of the same outline; the outlet patches of the emitters."""
    out = []
    skip = SHELL_REPLACES.get(scale, set())
    for s in BODY[scale]:
        reg = region_of(scale, s["id"])
        if reg and s["id"] not in skip:
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": reg})
    if scale in SHELL_REPLACES:
        out += shell_solids(scale)
    if scale == "100":
        out = [s for s in out if not s.get("of", "").startswith("pallet-top")]
        out += [{**{k: v for k, v in box(f"th-pallet-board-{i + 1}", "pallet-deck", "spruce-pallet", 0.6, x0, x1, STRINGER_Y1, PALLET_H, -PALLET_Z, PALLET_Z).items() if not k.startswith("_")}, "region": "pallet"} for i, (x0, x1) in enumerate(TOP_BOARDS)]
    return out + emitter_solids(scale)


HINGE_JOINT = {"50": 0.012, "100": 0.015, "200": 0.03}
MASS_R = {"50": 0.02, "100": 0.025, "200": 0.03}
STRUCTURAL = {
    "50": {"carcass": ["side-r", "side-l", "back", "base", "top-plate", "front-frame", "fascia", "muffle", "muffle-back", "foot-1", "foot-2", "foot-3", "foot-4"], "drawer": ["drawer-front", "drawer-bottom", "drawer-side-r", "drawer-side-l", "drawer-back"], "tray": ["tray-bottom", "tray-rim"], "door": ["door-frame"]},
    "100": {"carcass": ["carcass-back", "carcass-rear", "carcass-bay", "carcass-oven", "carcass-upper", "fascia", "hob-slab", "foot-1", "foot-2", "foot-3", "foot-4"], "drawer": ["drawer-front", "drawer-box"], "tray": ["tray"], "door": ["door-frame"]},
    "200": {"carcass": ["carcass-low", "carcass-back", "carcass-oven", "carcass-upper", "fascia", "hob-slab"], "drawer": ["drawer-block", "drawer-front"], "tray": ["tray"], "door": ["door-slab", "door-face"]},
}
JOINTS = {"50": {"drawer": ["runner-r", "runner-l"], "tray": ["rail-2-r", "rail-2-l"]}, "100": {"tray": ["rail-r", "rail-l"]}, "200": {}}


def structure_list(scale):
    """🏗️ Structure list: the sheet carcass and the muffle are the members of the carcass, the drawer box, the tray and the door frame members of their bones, runners, rails and the two door hinges the joints, the pallet boards and blocks its members, mass markers at the computed centres of mass; supports are the pose contacts."""
    out = []
    for bone, ids in STRUCTURAL[scale].items():
        out += [{"id": f"m-{sid}", "of": sid, "member": bone, "part": "member"} for sid in ids]
    for bone, ids in JOINTS[scale].items():
        out += [{"id": f"j-{sid}", "of": sid, "member": bone, "part": "joint"} for sid in ids]
    h = HINGE_JOINT[scale]
    for side, sgn in (("r", -1), ("l", 1)):
        z0, z1 = sorted((sgn * 0.215, sgn * (0.215 + max(0.02, h))))
        s = box(f"j-hinge-{side}", "door", "chrome-steel", 0.5, 0.0, h, 0.0, h, z0, z1, role="detail")
        out.append({**{k: v for k, v in s.items() if not k.startswith("_")}, "member": "door", "part": "joint"})
    out += [{"id": f"m-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"} for s in BODY[scale] if s["bone"].startswith("pallet")]
    r = MASS_R[scale]
    for bone in ("carcass", "door", "drawer", "tray", "pallet"):
        kg, cx, cy = MASS[bone]
        out.append({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": "chrome-steel", "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [r6(cx), r6(cy), 0.0]}, "member": bone, "part": "mass"})
    return out


VOL_DECISIONS = {
    "50": [
        T("Body 1:50 is the cooker as built in true gauges: 0.7 mm enamelled side panels, galvanised back and base, the 1 mm enamelled hob plate with four holes in which two 180 mm and two 145 mm cast-iron hotplates sit in pressed stainless trim rings, the 20 mm fascia with five original knobs and the black replacement, the front frame round the oven opening, the 1 mm oven muffle with three pairs of pressed rails and the top heating element in its 40 mm mineral wool jacket, the switchgear block behind the fascia, four adjustable feet and the taped cable coil on the back. The drawer is a pressed front on a 0.6 mm sheet box on two runners, the baking tray an 0.8 mm pan on the middle rails, the door an enamel frame with two 4 mm panes and a 20 mm bar handle on stand-offs. The spruce half pallet has its bottom boards, nine blocks, stringers and four top boards.", "Body 1:50 ist der Herd wie gebaut in wahren Blechstärken: 0,7-mm-Seitenwände aus Emaille, verzinkte Rückwand und Boden, das 1-mm-Muldenblech mit vier Löchern, in denen zwei 180-mm- und zwei 145-mm-Gusskochplatten in gepressten Edelstahlringen sitzen, die 20-mm-Blende mit fünf originalen Knebeln und dem schwarzen Ersatz, der Frontrahmen um die Backofenöffnung, die 1-mm-Backofenmuffel mit drei Paaren gepresster Einschubleisten und dem Oberhitzeheizkörper in ihrem 40-mm-Mineralwollmantel, der Schaltblock hinter der Blende, vier Stellfüße und das abgeklebte Kabelbündel an der Rückwand. Die Schublade ist eine gepresste Front an einem 0,6-mm-Blechkasten auf zwei Laufschienen, das Backblech eine 0,8-mm-Wanne auf den mittleren Leisten, die Tür ein Emaillerahmen mit zwei 4-mm-Scheiben und einem 20-mm-Stangengriff auf Stützen. Die Fichten-Halbpalette hat ihre Bodenbretter, neun Klötze, Querbretter und vier Deckbretter."),
        T("Thermal 1:50 references the exposed solids: the casing region closes itself with sides, back, base, front frame, feet and cable coil, the hob plate and the plates close the top, fascia and knobs, door frame and handle, the two panes, the drawer front; the 1.5 mm seam between drawer and door and the 0.3 mm side gaps of the drawer are under a quarter feature and closed, so the casing interior, the oven cavity and the space between the panes are no outer area. Outlet patches carry the oven vent and the hob plume. Structure 1:50: the sheet carcass and the muffle are members of the carcass bone, drawer box, tray and door frame members of their bones, runners, middle rails and two hinges the joints, the pallet parts its members, mass markers at the computed centres.", "Thermik 1:50 verweist auf die außenliegenden Körper: die Gehäuseregion schließt sich mit Seiten, Rückwand, Boden, Frontrahmen, Füßen und Kabelbündel, Muldenblech und Platten schließen oben, Blende und Knebel, Türrahmen und Griff, die zwei Scheiben, die Schubladenfront; die 1,5-mm-Fuge zwischen Schublade und Tür und die 0,3-mm-Seitenfugen der Schublade liegen unter einem Viertelmerkmal und sind geschlossen, so sind Gehäuseinneres, Backraum und der Raum zwischen den Scheiben keine Außenfläche. Auslassflächen tragen den Wrasenauslass und die Fahne über der Mulde. Struktur 1:50: Blechkorpus und Muffel sind Glieder des Korpusknochens, Schubkasten, Blech und Türrahmen Glieder ihrer Knochen, Laufschienen, mittlere Leisten und zwei Scharniere die Gelenke, die Palettenteile ihre Glieder, Massenmarken an den berechneten Schwerpunkten."),
    ],
    "100": [
        T("Body 1:100: the sheet casing becomes solid masses (a hollow rear shell with its back wall where the real cooker is empty behind the muffle, the drawer bay as a U, the oven block round the true 0.39 x 0.34 x 0.43 m cavity, the block behind the fascia) under a 15 mm hob slab; plate and trim ring are one revolved profile per plate in the slab holes; knobs are three-point profiles; the drawer is a closed box behind its front; one pair of 15 mm rails carries the tray as a 16 mm plate; the door keeps its frame, one pane and the handle on stand-offs; the 0.7 mm sheets, the muffle, the insulation, the switchgear and the heating element are inside the masses; feet are 30 mm blocks. The pallet keeps its bottom boards, blocks and stringers; its four top boards are typified as three on their own deck bone (envelope solids), because the 40 mm gaps would print under the office-laser minimum at 1:150.", "Body 1:100: das Blechgehäuse wird zu Vollmassen (eine hohle hintere Schale mit ihrer Rückwand, wo der echte Herd hinter der Muffel leer ist, das Schubkastenfach als U, der Ofenblock um den wahren Backraum von 0,39 x 0,34 x 0,43 m, der Block hinter der Blende) unter einer 15-mm-Muldenplatte; Platte und Abdeckring sind je Platte ein Drehprofil in den Löchern der Platte; Knebel sind Dreipunktprofile; die Schublade ist ein geschlossener Kasten hinter ihrer Front; ein Paar 15-mm-Leisten trägt das Blech als 16-mm-Platte; die Tür behält Rahmen, eine Scheibe und den Griff auf Stützen; die 0,7-mm-Bleche, Muffel, Dämmung, Schaltblock und Heizkörper liegen in den Massen; Füße sind 30-mm-Klötze. Die Palette behält Bodenbretter, Klötze und Querbretter; ihre vier Deckbretter sind auf eigenem Deckknochen als drei typisiert (Hüllkörper), weil die 40-mm-Fugen bei 1:150 unter dem Mindestmaß des Büro-Laserdrucks lägen."),
        T("Thermal 1:100 references the hob slab, plates, knobs, door, pane, handle, drawer front, feet, cable coil and pallet; the solid casing masses are replaced by a closed 15 mm shell of the same outline with the fascia between its walls, so the faces inside the closed casing are no outer area and the oven cavity is closed by the door; the pallet deck is thermally its four real top boards, because a typified envelope board has no surface of its own. Structure 1:100: the masses and the slab as carcass members with the feet, drawer box and front, tray and door frame as members of their bones, the rails and two 15 mm hinges as joints, the pallet parts, mass markers.", "Thermik 1:100 verweist auf Muldenplatte, Kochplatten, Knebel, Tür, Scheibe, Griff, Schubladenfront, Füße, Kabelbündel und Palette; die vollen Gehäusemassen sind durch eine geschlossene 15-mm-Schale desselben Umrisses mit der Blende zwischen ihren Wänden ersetzt, so sind die Flächen im geschlossenen Gehäuse keine Außenfläche und der Backraum ist durch die Tür geschlossen; das Palettendeck ist thermisch seine vier echten Deckbretter, weil ein typisiertes Hüllbrett keine eigene Oberfläche hat. Struktur 1:100: Massen und Platte als Korpusglieder mit den Füßen, Schubkasten und Front, Blech und Türrahmen als Glieder ihrer Knochen, die Leisten und zwei 15-mm-Scharniere als Gelenke, die Palettenteile, Massenmarken."),
    ],
    "200": [
        T("Body 1:200: the lower casing is one U from the floor (feet and base merged) round a drawer block that fills the bay, the oven block keeps the cavity, back and upper blocks, a 30 mm fascia and a 30 mm hob slab with the four plates as 37 mm discs that stand 7 mm proud; the door is a slab with its glass sheet in the face and a 30 x 45 mm grip block; the tray is a 30 mm plate in the cavity; knobs, feet, the cable coil, rails and stand-offs are under 30 mm and left out; the pallet is a 44 mm deck on nine blocks that reach the floor.", "Body 1:200: das untere Gehäuse ist ein U vom Boden an (Füße und Boden verschmolzen) um einen Schubkastenblock, der das Fach füllt, der Ofenblock behält den Backraum, Rück- und Oberblock, eine 30-mm-Blende und eine 30-mm-Muldenplatte mit den vier Kochplatten als 37-mm-Scheiben, die 7 mm vorstehen; die Tür ist eine Platte mit der Glasscheibe in der Front und einem Griffblock von 30 x 45 mm; das Blech ist eine 30-mm-Platte im Backraum; Knebel, Füße, Kabelbündel, Leisten und Griffstützen liegen unter 30 mm und entfallen; die Palette ist ein 44-mm-Deck auf neun Klötzen, die bis zum Boden reichen."),
        T("Thermal 1:200 references hob slab, plate discs, door slab and face, glass, grip block and the pallet; the casing masses, fascia and drawer are replaced by a closed 30 mm shell of the same outline with fascia and drawer front set between its walls. Structure 1:200: the masses and the slab as carcass members, drawer block and front, tray, door slab and face as members of their bones, two 30 mm hinges as joints, the pallet deck and blocks, mass markers.", "Thermik 1:200 verweist auf Muldenplatte, Plattenscheiben, Türplatte und -front, Glas, Griffblock und Palette; Gehäusemassen, Blende und Schublade sind durch eine geschlossene 30-mm-Schale desselben Umrisses ersetzt, Blende und Schubladenfront sitzen zwischen ihren Wänden. Struktur 1:200: die Massen und die Platte als Korpusglieder, Schubkastenblock und Front, Blech, Türplatte und -front als Glieder ihrer Knochen, zwei 30-mm-Scharniere als Gelenke, Palettendeck und Klötze, Massenmarken."),
    ],
}


def volume(scale):
    """🧊 One authored volume."""
    return {"status": "authored", "denominator": int(scale), "body": clean(BODY[scale]), "thermal": thermal_list(scale), "structure": structure_list(scale), "decisions": VOL_DECISIONS[scale]}


AREAS = {"casing": 1.72, "hob-top": 0.205, "hotplates": 0.115, "fascia": 0.095, "oven-door": 0.47, "door-window": 0.08, "drawer": 0.115, "pallet": 3.1}
AREA_FILE = os.path.join(HERE, "areas.json")
if os.path.exists(AREA_FILE):
    AREAS.update(json.load(open(AREA_FILE)))
MAT = {m["id"]: m for m in MATERIALS}


def layer(lid, title, kind, mat, d, src="literature", ref="iso-10456-2007"):
    """🧶 One layer: thickness, resistance d/k, areal heat capacity rho c d."""
    m = MAT[mat]
    return {"id": lid, "title": title, "kind": kind, "material": mat, "thicknessM": d, "resistanceM2KW": r6(d / m["conductivityWmK"]), "heatCapacityJM2K": round(m["densityKgM3"] * m["specificHeatJKgK"] * d), "source": src, "reference": ref}


def region(rid, title, kind, model, mat, length, geometry, sky, lay, ref, **kw):
    """🌡️ One region."""
    r = {"id": rid, "title": title, "kind": kind, "model": model, "material": mat, "areaM2": AREAS[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"]}
    r.update(kw)
    r.update({"source": "authored-estimate", "reference": ref})
    return r


REGIONS = [
    region("casing", T("Enamelled side panels, back, base, front frame, feet and cable coil", "Emaillierte Seitenwände, Rückwand, Boden, Frontrahmen, Füße und Kabelbündel"), "surface", "operating", "enamel-white-aged", 0.85, "vertical-plate", 0.5, layer("casing-sheet", T("Enamelled steel sheet 0.7 mm", "Emailliertes Stahlblech 0,7 mm"), "metal", "enamel-white-aged", 0.0007, ref="en-10209-2013"), "en-60335-2-6", operating={"key": "oven", "heatW": OVEN_HEAT["casing"]}),
    region("hob-top", T("Enamelled hob plate with burnt-on stains", "Emailliertes Muldenblech mit eingebrannten Flecken"), "surface", "operating", "enamel-hob-stained", 0.5, "horizontal-plate-up", 0.9, layer("hob-sheet", T("Enamelled steel 1 mm", "Emaillierter Stahl 1 mm"), "metal", "enamel-hob-stained", 0.001, ref="en-10209-2013"), "en-60335-2-6", operating={"key": "oven", "heatW": OVEN_HEAT["hob-top"]}),
    region("hotplates", T("Four cast-iron hotplates and their trim rings", "Vier Gusskochplatten und ihre Abdeckringe"), "heater", "operating", "cast-iron-aged", 0.16, "horizontal-plate-up", 0.9, layer("plate-iron", T("Cast iron 12 mm", "Gusseisen 12 mm"), "metal", "cast-iron-aged", 0.012, ref="incropera-2018"), "cooker-catalogue", operating={"key": "hob", "heatW": HOB_W}, ratedPowerW=HOB_W, ratedPowerBasis="electrical-input"),
    region("fascia", T("Control fascia with six knobs and two pilot lamps", "Schalterblende mit sechs Knebeln und zwei Kontrolllampen"), "surface", "operating", "enamel-white-aged", 0.12, "vertical-plate", 0.5, layer("fascia-sheet", T("Enamelled steel 1 mm", "Emaillierter Stahl 1 mm"), "metal", "enamel-white-aged", 0.001, ref="en-10209-2013"), "en-60335-2-6", operating={"key": "oven", "heatW": OVEN_HEAT["fascia"]}),
    region("oven-door", T("Oven door frame and handle", "Backofentürrahmen und Griff"), "surface", "operating", "enamel-white-aged", 0.52, "vertical-plate", 0.5, layer("door-sheet", T("Enamelled steel 0.8 mm", "Emaillierter Stahl 0,8 mm"), "metal", "enamel-white-aged", 0.0008, ref="en-10209-2013"), "cooker-catalogue", operating={"key": "oven", "heatW": OVEN_HEAT["oven-door"]}, ratedPowerW=OVEN_W, ratedPowerBasis="electrical-input"),
    region("door-window", T("Double-glazed door window", "Zweifach verglastes Türfenster"), "glazing", "operating", "door-glass", 0.24, "vertical-plate", 0.5, layer("door-pane", T("Toughened glass 4 mm", "Vorgespanntes Glas 4 mm"), "glass", "door-glass", 0.004, ref="incropera-2018"), "incropera-2018", operating={"key": "oven", "heatW": OVEN_HEAT["door-window"]}),
    region("drawer", T("Drawer front", "Schubladenfront"), "surface", "operating", "enamel-white-aged", 0.18, "vertical-plate", 0.5, layer("drawer-sheet", T("Enamelled steel 0.8 mm", "Emaillierter Stahl 0,8 mm"), "metal", "enamel-white-aged", 0.0008, ref="en-10209-2013"), "en-60335-2-6", operating={"key": "oven", "heatW": OVEN_HEAT["drawer"]}),
    region("pallet", T("Spruce half pallet", "Fichten-Halbpalette"), "surface", "passive", "spruce-pallet", 0.6, "horizontal-plate-up", 0.5, layer("pallet-board", T("Spruce board 22 mm", "Fichtenbrett 22 mm"), "timber", "spruce-pallet", 0.022), "half-pallet", ratedPowerW=0, ratedPowerBasis="passive"),
]
REGION_IDS = [r["id"] for r in REGIONS]

OPERATING = [
    {"id": "oven", "title": T("Oven, top and bottom heat", "Backofen, Ober- und Unterhitze"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.4, "title": T("Holding 200 degC, thermostat cycling (0.8 kW mean)", "Hält 200 degC, Thermostat taktet (0,8 kW im Mittel)")}, {"v": 1, "title": T("Preheating at full top and bottom heat (2.0 kW)", "Vorheizen mit voller Ober- und Unterhitze (2,0 kW)")}]},
    {"id": "hob", "title": T("Hotplates", "Kochplatten"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.333, "title": T("The 2.0 kW front left plate at full", "Die 2,0-kW-Platte vorn links auf voller Stufe")}, {"v": 1, "title": T("All four plates at full (6.0 kW)", "Alle vier Platten auf voller Stufe (6,0 kW)")}]},
]
EMITTERS = [
    {"id": "oven-vent", "kind": "vent", "title": T("Hot moist air from the oven vent at the rear of the hob", "Heiße feuchte Luft aus dem Wrasenauslass hinten an der Mulde"), "anchor": "oven-vent", "directionDeg": 90, "diameterM": 0.03, "operatingKey": "oven", "volumeFlowM3S": VENT_FLOW, "temperatureRiseK": VENT_DT, "vapourKgPerKg": 0.01, "source": "authored-estimate", "reference": "en-60350-1"},
    {"id": "hob-plume", "kind": "plume", "title": T("Plume above the hot plate", "Fahne über der heißen Platte"), "anchor": "hob-centre", "directionDeg": 90, "diameterM": 0.18, "region": "hotplates", "source": "authored-estimate", "reference": "incropera-2018"},
]


EXPOSED = {"casing": 0.77, "hob-top": 0.86, "hotplates": 0.97, "fascia": 0.86, "oven-door": 0.96, "door-window": 0.94, "drawer": 0.97, "pallet": 0.78}


def binding(oven, hob, vent, plume, pallet_contact=0.15, casing_contact=0.0):
    """🌬️ The thermal state of one pose: operating levels, every region in air without sun, the pallet and feet on the floor."""
    regs = {rid: {"exposedFraction": EXPOSED[rid], "sunlitFraction": 0.0} for rid in REGION_IDS}
    regs["pallet"]["contact"] = {"fraction": pallet_contact, "tempC": 18}
    if casing_contact:
        regs["casing"]["contact"] = {"fraction": casing_contact, "tempC": 18}
    return {"operating": {"oven": oven, "hob": hob}, "regions": regs, "emitters": {"oven-vent": vent, "hob-plume": plume}}


def cake_load():
    """🎂 A tray cake of 2 kg spread over the baking tray."""
    return {"id": "tray-cake", "kind": "live", "title": T("Tray cake on the baking tray (2 kg)", "Blechkuchen auf dem Backblech (2 kg)"), "distributed": {"bone": "tray", "fromM": TRAY_X0, "toM": TRAY_X1, "wxNPerM": 0, "wyNPerM": r6(-CAKE_KG * G / (TRAY_X1 - TRAY_X0))}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-60350-1"}


def stability():
    """🧮 Centre of mass of the cooker with the door open flat, the tray pulled 0.25 m with its cake and the 22.5 kg test mass on the door centre, against the front feet."""
    m = mx = 0.0
    for bone in ("carcass", "drawer"):
        kg, cx, _ = MASS[bone]
        m += kg
        mx += kg * cx
    kg, cu, _ = MASS["door"]
    m += kg
    mx += kg * (HINGE_X + cu)
    kg, cx, _ = MASS["tray"]
    m += kg + CAKE_KG
    mx += kg * (cx + TRAY_PULL) + CAKE_KG * ((TRAY_X0 + TRAY_X1) / 2 + TRAY_PULL)
    com = mx / m
    edge = max(x for x, _ in FEET) + FOOT_R
    restoring = m * G * (edge - com)
    overturning = DOOR_TEST_KG * G * (HINGE_X + DOOR_H / 2 - edge)
    return m, com, edge, restoring, overturning


M_OPEN, COM_OPEN, EDGE, RESTORING, OVERTURN = stability()

STORED_BONES = {}
CARRIED_BONES = {"cooker": {"dy": LIFT}, "cooker-carry": {"dx": CARRY}, "carcass": {"angleDeg": TILT}}
IN_USE_BONES = {"cooker": {"dy": -PALLET_H}, "cooker-carry": {"dx": USE_SHIFT}}
OPEN_BONES = {**IN_USE_BONES, "door": {"angleDeg": DOOR_OPEN}, "tray": {"dx": TRAY_PULL}}

POSES = [
    {"id": "stored", "title": T("Stored in the reuse depot", "Gelagert im Gebrauchtwarenlager"), "description": T(f"Cleaned, tested and switched off, the cooker stands on its four feet on a spruce half pallet among the white goods of the depot, the baking tray inside on the middle rails, the drawer closed, the mains cable coiled and taped to the back, the price tag on the hob: the pallet carries {TOTAL:.0f} kg on 0.48 m2.", f"Gereinigt, geprüft und ausgeschaltet steht der Herd auf seinen vier Füßen auf einer Fichten-Halbpalette zwischen den Weißgeräten des Lagers, das Backblech innen auf den mittleren Leisten, die Schublade geschlossen, die Anschlussleitung aufgerollt und an der Rückwand abgeklebt, das Preisschild auf der Mulde: die Palette trägt {TOTAL:.0f} kg auf 0,48 m2."), "bones": STORED_BONES, "ground": "auto", "contacts": ["pallet-floor"], "thermal": binding(0, 0, 0, 0), "loads": []},
    {"id": "carried", "title": T("Carried by two people", "Von zwei Personen getragen"), "description": T(f"Two carriers lift the {COOKER_KG:.1f} kg cooker 0.20 m off the pallet with both hands under the base at the side edges, tilt it 4 degrees back so the door stays shut and carry it 0.35 m forward; the pallet stays on the floor.", f"Zwei Träger heben den {COOKER_KG:.1f} kg schweren Herd mit beiden Händen unter dem Boden an den Seitenkanten 0,20 m von der Palette, kippen ihn 4 Grad nach hinten, damit die Tür zu bleibt, und tragen ihn 0,35 m nach vorn; die Palette bleibt am Boden."), "bones": CARRIED_BONES, "ground": "authored", "contacts": GRIPS + ["pallet-floor"], "thermal": binding(0, 0, 0, 0), "loads": []},
    {"id": "in-use", "title": T("Baking in a kitchen", "Backen in einer Küche"), "description": T("Back in a kitchen the cooker stands on its feet on the floor at the 85 cm work height, the oven holds 200 degC with top and bottom heat while a 2 kg tray cake bakes on the middle rails; the hotplates are off and the pallet it came on stays behind.", "Zurück in einer Küche steht der Herd auf seinen Füßen am Boden in 85 cm Arbeitshöhe, der Backofen hält 200 degC mit Ober- und Unterhitze, während ein Blechkuchen von 2 kg auf den mittleren Leisten backt; die Kochplatten sind aus, die Palette, auf der er kam, bleibt zurück."), "bones": IN_USE_BONES, "ground": "auto", "contacts": FOOT_IDS + ["pallet-floor"], "thermal": binding(0.4, 0, 1, 0), "loads": [cake_load()]},
    {"id": "door-open", "title": T("Door open, tray pulled out", "Tür offen, Blech herausgezogen"), "description": T(f"The oven is off, the door lies open flat on its hinges and the tray with the cake is pulled 0.25 m onto its stops; a 22.5 kg mass on the middle of the door, as in the stability test of EN 60335-2-6, turns the cooker forward with {OVERTURN:.0f} Nm against {RESTORING:.0f} Nm of its own weight about the front feet: free-standing it would tip, so the anti-tilt bracket screwed behind it in the kitchen holds the rear base down.", f"Der Backofen ist aus, die Tür liegt flach offen auf ihren Scharnieren und das Blech mit dem Kuchen ist 0,25 m bis zu den Anschlägen herausgezogen; eine Masse von 22,5 kg auf der Türmitte wie bei der Standsicherheitsprüfung nach EN 60335-2-6 dreht den Herd mit {OVERTURN:.0f} Nm nach vorn gegen {RESTORING:.0f} Nm aus seinem Eigengewicht um die Vorderfüße: frei stehend kippte er, deshalb hält der hinter ihm in der Küche verschraubte Kippschutzwinkel den hinteren Boden nieder."), "bones": OPEN_BONES, "ground": "auto", "contacts": FOOT_IDS + ["anti-tilt", "pallet-floor"], "thermal": binding(0, 0, 0, 0), "loads": [cake_load(), {"id": "door-test-mass", "kind": "imposed", "title": T("Test mass of 22.5 kg on the middle of the open door", "Prüfmasse von 22,5 kg auf der Mitte der offenen Tür"), "bone": "door", "x": r6(DOOR_H / 2), "y": DOOR_T, "fxN": 0, "fyN": r6(-DOOR_TEST_KG * G), "source": "authored-estimate", "reference": "en-60335-2-6"}]},
]


def carry_clip():
    """🧺 Clip stored to carried: the carriers take hold under the base, lift the cooker off the pallet, tilt it back and carry it forward."""
    keys = [
        {"t": 0, "pose": "stored", "ease": "linear"},
        {"t": 1.0, "pose": "stored", "ease": "in"},
        {"t": 2.2, "pose": "stored", "ease": "linear", "bones": {"cooker": {"dy": LIFT}, "carcass": {"angleDeg": TILT}}},
        {"t": 4.4, "pose": "carried", "ease": "linear"},
    ]
    return {"id": "carry", "title": T("Lift the cooker off the pallet and carry it", "Herd von der Palette heben und tragen"), "description": T("Two carriers grip under the base at both side edges, lift the cooker 0.20 m off the pallet while tilting it 4 degrees back, and carry it 0.35 m forward; the pallet stays where it was.", "Zwei Träger greifen an beiden Seitenkanten unter den Boden, heben den Herd 0,20 m von der Palette, kippen ihn dabei 4 Grad nach hinten und tragen ihn 0,35 m nach vorn; die Palette bleibt, wo sie war."), "durationS": 4.4, "fps": 25, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [{"anchor": "pallet-floor", "fromS": 0, "toS": 4.4}] + [{"anchor": g, "fromS": 1.0, "toS": 4.4} for g in GRIPS], "phases": [{"id": "grip", "fromS": 0, "toS": 1.0, "title": T("Taking hold", "Zugreifen")}, {"id": "lift", "fromS": 1.0, "toS": 2.2, "title": T("Lifting and tilting", "Anheben und Kippen")}, {"id": "carry", "fromS": 2.2, "toS": 4.4, "title": T("Carrying", "Tragen")}], "events": [{"t": 1.0, "kind": "contact-on", "title": T("Carriers take hold under the base", "Träger fassen unter den Boden"), "target": "grip-front-l"}, {"t": 2.2, "kind": "motion", "title": T("Cooker lifted 0.20 m and tilted back", "Herd 0,20 m angehoben und nach hinten gekippt"), "target": "carcass"}], "tracks": []}


def heat_clip():
    """♨️ Clip in use: the oven preheats at full top and bottom heat, the thermostat starts cycling at 200 degC, then the large front plate is switched on."""
    return {"id": "oven-heating", "title": T("Preheat the oven and switch on a plate", "Backofen vorheizen und eine Platte einschalten"), "description": T("The oven is switched to 200 degC top and bottom heat: it preheats at the full 2.0 kW for about seven minutes, then the thermostat cycles at a mean 0.8 kW; after ten minutes the 2.0 kW front left plate is switched on at full; door, casing and hob warm up and the vapour leaves at the rear vent.", "Der Backofen wird auf 200 degC Ober- und Unterhitze gestellt: er heizt etwa sieben Minuten mit vollen 2,0 kW vor, dann taktet der Thermostat mit 0,8 kW im Mittel; nach zehn Minuten wird die 2,0-kW-Platte vorn links auf volle Stufe geschaltet; Tür, Gehäuse und Mulde erwärmen sich, der Wrasen tritt hinten am Auslass aus."), "durationS": 900, "fps": 1, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "in-use", "ease": "step"}, {"t": 900, "pose": "in-use", "ease": "step"}], "contacts": [{"anchor": a, "fromS": 0, "toS": 900} for a in FOOT_IDS + ["pallet-floor"]], "phases": [{"id": "off", "fromS": 0, "toS": 10, "title": T("Cold", "Kalt")}, {"id": "preheat", "fromS": 10, "toS": 420, "title": T("Preheating at 2.0 kW", "Vorheizen mit 2,0 kW")}, {"id": "hold", "fromS": 420, "toS": 900, "title": T("Holding 200 degC", "Hält 200 degC")}], "events": [{"t": 10, "kind": "operating", "title": T("Oven switched on", "Backofen eingeschaltet"), "target": "oven"}, {"t": 420, "kind": "thermal", "title": T("200 degC reached, thermostat cycles", "200 degC erreicht, Thermostat taktet"), "target": "oven"}, {"t": 600, "kind": "operating", "title": T("Front left plate on at full", "Platte vorn links auf voller Stufe"), "target": "hob"}], "tracks": [{"target": "operating:oven", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 10, "v": 1}, {"t": 420, "v": 0.4}, {"t": 900, "v": 0.4}]}, {"target": "operating:hob", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 600, "v": 0.333}, {"t": 900, "v": 0.333}]}, {"target": "emitter:oven-vent", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 10, "v": 1}, {"t": 900, "v": 1}]}, {"target": "emitter:hob-plume", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 600, "v": 1}, {"t": 900, "v": 1}]}]}


def door_clip():
    """🚪 Clip in use: the door is opened flat, the tray pulled to its stops and pushed back, the door closed again."""
    o = {"door": {"angleDeg": DOOR_OPEN}}
    ot = {"door": {"angleDeg": DOOR_OPEN}, "tray": {"dx": TRAY_PULL}}
    keys = [
        {"t": 0, "pose": "in-use", "ease": "out"},
        {"t": 1.4, "pose": "in-use", "ease": "linear", "bones": o},
        {"t": 2.4, "pose": "in-use", "ease": "linear", "bones": ot},
        {"t": 3.4, "pose": "in-use", "ease": "out", "bones": ot},
        {"t": 4.4, "pose": "in-use", "ease": "in", "bones": o},
        {"t": 6.0, "pose": "in-use", "ease": "linear"},
    ]
    return {"id": "open-door", "title": T("Open the door and pull the tray", "Tür öffnen und Blech herausziehen"), "description": T("The oven door is let down flat on its hinges, the baking tray with the cake is pulled 0.25 m to its stops to check the cake and pushed back, and the door is closed again.", "Die Backofentür wird flach auf ihre Scharniere herabgelassen, das Backblech mit dem Kuchen wird 0,25 m bis zu den Anschlägen gezogen, um nach dem Kuchen zu sehen, und wieder eingeschoben, dann wird die Tür geschlossen."), "durationS": 6.0, "fps": 25, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [{"anchor": a, "fromS": 0, "toS": 6.0} for a in FOOT_IDS + ["pallet-floor"]], "phases": [{"id": "open", "fromS": 0, "toS": 1.4, "title": T("Door opening", "Tür öffnen")}, {"id": "tray-out", "fromS": 1.4, "toS": 3.4, "title": T("Tray out, looking at the cake", "Blech heraus, Blick auf den Kuchen")}, {"id": "tray-in", "fromS": 3.4, "toS": 4.4, "title": T("Tray pushed back", "Blech eingeschoben")}, {"id": "close", "fromS": 4.4, "toS": 6.0, "title": T("Door closing", "Tür schließen")}], "events": [{"t": 1.4, "kind": "motion", "title": T("Door open flat", "Tür flach offen"), "target": "door"}, {"t": 2.4, "kind": "motion", "title": T("Tray at its stops", "Blech an den Anschlägen"), "target": "tray"}, {"t": 6.0, "kind": "motion", "title": T("Door closed", "Tür geschlossen"), "target": "door"}], "tracks": []}


THERMAL = {
    "environment": {"airTempC": 21, "meanRadiantTempC": 20, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1.0, "pressurePa": 101325},
    "operating": OPERATING,
    "regions": REGIONS,
    "emitters": EMITTERS,
    "passengers": [],
    "assumptions": [
        T(f"Nameplates (catalogue-typical, electrical input): hotplates {HOB_W / 1000:.1f} kW on the hotplates region (2.0 + 1.0 + 1.5 + 1.5 kW), oven top and bottom heat {OVEN_W / 1000:.1f} kW on the oven-door region; total {(HOB_W + OVEN_W) / 1000:.1f} kW, which is the connected load of such a cooker at 400 V 3N without the oven lamp; the pallet states 0 W passive.", f"Typenschilder (katalogtypisch, elektrische Aufnahme): Kochplatten {HOB_W / 1000:.1f} kW an der Region der Kochplatten (2,0 + 1,0 + 1,5 + 1,5 kW), Ober- und Unterhitze {OVEN_W / 1000:.1f} kW an der Region der Backofentür; zusammen {(HOB_W + OVEN_W) / 1000:.1f} kW, der Anschlusswert eines solchen Herds an 400 V 3N ohne Backofenlampe; die Palette gibt 0 W passiv an."),
        T(f"At full oven level the 2.0 kW leave as released heat of the casing ({OVEN_HEAT['casing']} W), the door frame ({OVEN_HEAT['oven-door']} W) and window ({OVEN_HEAT['door-window']} W), the hob plate ({OVEN_HEAT['hob-top']} W), drawer ({OVEN_HEAT['drawer']} W), fascia ({OVEN_HEAT['fascia']} W) and the vent air (4 l/s heated 60 K = {VENT_W:.0f} W); the hotplates release their electrical input at their level, so a bare plate at full glows as in reality.", f"Bei voller Backofenstufe verlassen die 2,0 kW als abgegebene Wärme das Gehäuse ({OVEN_HEAT['casing']} W), den Türrahmen ({OVEN_HEAT['oven-door']} W) und das Fenster ({OVEN_HEAT['door-window']} W), das Muldenblech ({OVEN_HEAT['hob-top']} W), die Schublade ({OVEN_HEAT['drawer']} W), die Blende ({OVEN_HEAT['fascia']} W) und die Wrasenluft (4 l/s um 60 K erwärmt = {VENT_W:.0f} W); die Kochplatten geben ihre elektrische Aufnahme auf ihrer Stufe ab, so glüht eine leere Platte auf voller Stufe wie in Wirklichkeit."),
        T("Aged surfaces differ from new ones: the yellowed white enamel is taken with a solar absorptance of 0.30 against about 0.20 new, the stained hob plate 0.50, the cast-iron plates with their worn coating and rust an emissivity of 0.88 against 0.95 freshly coated, the dulled stainless rings 0.35 against about 0.15 polished, the greyed pallet 0.65 against about 0.45 for fresh spruce (authored estimates from the colour, after Incropera Table A.11).", "Gealterte Oberflächen unterscheiden sich von neuen: die vergilbte weiße Emaille wird mit einem solaren Absorptionsgrad von 0,30 statt etwa 0,20 neu angesetzt, das fleckige Muldenblech mit 0,50, die Gussplatten mit abgenutztem Anstrich und Rost mit einem Emissionsgrad von 0,88 statt 0,95 frisch beschichtet, die matten Edelstahlringe mit 0,35 statt etwa 0,15 poliert, die vergraute Palette mit 0,65 statt etwa 0,45 für frische Fichte (Schätzwerte aus der Farbe, nach Incropera Tabelle A.11)."),
        T("Reference case: a kitchen at 21 degC without sun; the inner door pane, the muffle, its mineral wool, the switchgear, the drawer box and the baking tray lie inside the closed casing and oven and belong to no region; the pallet rests with 15 % of its surface on the 18 degC floor.", "Referenzfall: eine Küche mit 21 degC ohne Sonne; die innere Türscheibe, Muffel, Mineralwolle, Schaltblock, Schubkasten und Backblech liegen im geschlossenen Gehäuse und Backofen und gehören zu keiner Region; die Palette liegt mit 15 % ihrer Fläche auf dem 18 degC kalten Boden."),
    ],
}

STRUCTURE = {
    "gravityMS2": G,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T(f"Masses from the 1:50 solids, section x density (enamelled and galvanised steel 7850, cast iron 7200, glass 2500, mineral wool 60, spruce 450 kg/m3); hollow pressed parts by their developed sheet area x gauge x density (fascia 0.76 kg, door frame 3.07 kg, drawer front 0.68 kg, handle tube 0.21 kg) and the switchgear as a stated 2.0 kg: carcass {MASS['carcass'][0]:.2f} kg, door {MASS['door'][0]:.2f} kg, drawer {MASS['drawer'][0]:.2f} kg, tray {MASS['tray'][0]:.2f} kg, cooker {COOKER_KG:.2f} kg (catalogue 38-45 kg), pallet {MASS['pallet'][0] + MASS['pallet-deck'][0]:.2f} kg, total {TOTAL:.2f} kg.", f"Massen aus den 1:50-Körpern, Querschnitt x Dichte (emaillierter und verzinkter Stahl 7850, Gusseisen 7200, Glas 2500, Mineralwolle 60, Fichte 450 kg/m3); hohle gepresste Teile über abgewickelte Blechfläche x Stärke x Dichte (Blende 0,76 kg, Türrahmen 3,07 kg, Schubladenfront 0,68 kg, Griffrohr 0,21 kg) und der Schaltblock mit angegebenen 2,0 kg: Korpus {MASS['carcass'][0]:.2f} kg, Tür {MASS['door'][0]:.2f} kg, Schublade {MASS['drawer'][0]:.2f} kg, Blech {MASS['tray'][0]:.2f} kg, Herd {COOKER_KG:.2f} kg (Katalog 38-45 kg), Palette {MASS['pallet'][0] + MASS['pallet-deck'][0]:.2f} kg, gesamt {TOTAL:.2f} kg."),
        T(f"Load paths: stored, the four feet stand on the top boards and the pallet on the floor (one floor contact of 0.60 x 0.80 m); in use, the four feet stand on the floor; carried, two carriers hold the cooker at four pins under the base at the side edges (about {COOKER_KG * G / 4:.0f} N per hand) while the pallet rests on the floor. The product solves the whole asset as one rigid body on all contacts of a pose, so in the carried pose the shares between the hands and the pallet contact are the solver's elastic split, not separate equilibria.", f"Lastpfade: gelagert stehen die vier Füße auf den Deckbrettern und die Palette auf dem Boden (ein Bodenkontakt 0,60 x 0,80 m); in Gebrauch stehen die vier Füße am Boden; getragen halten zwei Träger den Herd an vier Gelenkpunkten unter dem Boden an den Seitenkanten (etwa {COOKER_KG * G / 4:.0f} N je Hand), während die Palette am Boden liegt. Das Produkt löst das ganze Objekt als einen starren Körper auf allen Kontakten einer Pose, daher sind die Anteile von Händen und Palettenkontakt in der getragenen Pose die elastische Aufteilung des Lösers, keine getrennten Gleichgewichte."),
        T(f"Stability (EN 60335-2-6 clause 20): with the door open flat and the tray with its cake pulled 0.25 m, the {M_OPEN:.1f} kg cooker has its centre of mass at x = {COM_OPEN:.3f} m, {1000 * (EDGE - COM_OPEN):.0f} mm behind the front edge of the front feet (restoring {RESTORING:.1f} Nm); a 22.5 kg mass on the middle of the door, {HINGE_X + DOOR_H / 2 - EDGE:.3f} m beyond that edge, adds {OVERTURN:.1f} Nm, {OVERTURN - RESTORING:.1f} Nm more than the weight holds: a free-standing 40 kg cooker fails this case, which is why the standard asks for an anti-tilt device, and the door-open pose carries the reaction at the anti-tilt bracket pin at the rear base (in use it is mounted but unloaded); the test mass is an authored estimate of the standard's door load.", f"Standsicherheit (EN 60335-2-6 Abschnitt 20): mit flach offener Tür und 0,25 m herausgezogenem Blech mit Kuchen liegt der Schwerpunkt des {M_OPEN:.1f} kg schweren Herds bei x = {COM_OPEN:.3f} m, {1000 * (EDGE - COM_OPEN):.0f} mm hinter der Vorderkante der Vorderfüße (Rückstellmoment {RESTORING:.1f} Nm); eine Masse von 22,5 kg auf der Türmitte, {HINGE_X + DOOR_H / 2 - EDGE:.3f} m vor dieser Kante, fügt {OVERTURN:.1f} Nm hinzu, {OVERTURN - RESTORING:.1f} Nm mehr als das Eigengewicht hält: ein frei stehender 40-kg-Herd besteht diesen Fall nicht, deshalb verlangt die Norm eine Kippsicherung, und die Pose mit offener Tür nimmt die Reaktion am Gelenkpunkt des Kippschutzwinkels am hinteren Boden auf (in Gebrauch ist er montiert, aber unbelastet); die Prüfmasse ist eine Schätzung der Türlast der Norm."),
        T("Sections: the carcass is a sheet monocoque whose side panels (two 0.7 x 560 mm) carry the hob and oven loads to the feet; the door is a sandwich of two 0.8 mm sheets 40 mm apart on two hinges; drawer and tray are thin pressed steel; enamelling steel DC04EK after EN 10209, the spruce of the pallet C24 after EN 338.", "Querschnitte: der Korpus ist ein Blechmonocoque, dessen Seitenwände (zwei 0,7 x 560 mm) die Lasten von Mulde und Backofen in die Füße tragen; die Tür ist ein Sandwich aus zwei 0,8-mm-Blechen im Abstand von 40 mm an zwei Scharnieren; Schublade und Blech sind dünner gepresster Stahl; Emaillierstahl DC04EK nach EN 10209, die Fichte der Palette C24 nach EN 338."),
    ],
    "clinical": False,
}

ASSET = {
    "id": "used-electric-cooker",
    "category": "appliances",
    "title": T("Used Electric Cooker with Oven", "Gebrauchter Elektroherd mit Backofen"),
    "summary": T(f"A second-hand free-standing 50 cm electric cooker, 0.50 x 0.60 x 0.85 m and {COOKER_KG:.0f} kg, with four cast-iron hotplates in stainless trim rings, a 55 l oven with top and bottom heat behind a double-glazed door, a baking tray and an appliance drawer, {(HOB_W + OVEN_W) / 1000:.0f} kW connected load: the hob shows burnt-on stains and a rust ring, one knob is a black replacement, the door glass is scratched. It is stored on a half pallet in the reuse depot, carried by two people with their hands under the base, and back in use in a kitchen baking a tray cake, with the door let down and the tray pulled out.", f"Ein gebrauchter Stand-Elektroherd mit 50 cm Breite, 0,50 x 0,60 x 0,85 m und {COOKER_KG:.0f} kg, mit vier Gusskochplatten in Edelstahlringen, einem 55-l-Backofen mit Ober- und Unterhitze hinter einer doppelt verglasten Tür, einem Backblech und einer Geräteschublade, {(HOB_W + OVEN_W) / 1000:.0f} kW Anschlusswert: die Mulde zeigt eingebrannte Flecken und einen Rostring, ein Knebel ist ein schwarzer Ersatz, das Türglas ist verkratzt. Er lagert auf einer Halbpalette im Gebrauchtwarenlager, wird von zwei Personen mit den Händen unter dem Boden getragen und backt wieder in einer Küche einen Blechkuchen, mit herabgelassener Tür und herausgezogenem Blech."),
    "tags": ["used", "household", "second-hand", "kitchen", "cooker", "electric-cooker", "oven", "hob", "hotplates", "cast-iron", "enamel", "white-goods", "pallet", "appliance", "interior"],
    "provenance": {"author": "Semio staffage content (used household objects)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T("Built to the catalogue-typical 50 cm free-standing electric cooker with cast-iron hotplates of 1990-2010 (no brand); every parameter is literature or an authored estimate, nothing is measured.", "Gebaut nach dem katalogtypischen 50-cm-Standherd mit Gusskochplatten der Jahre 1990-2010 (ohne Marke); jeder Parameter ist Literatur oder Schätzung, nichts ist gemessen.")},
    "dimensions": {"heightM": r6(PALLET_H + PLATE_TOP), "widthM": r6(HINGE_X + 0.045 + 0.024), "depthM": r6(2 * PALLET_Z), "massKg": round(TOTAL, 3)},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "floor", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {"500": UNAVAILABLE_500, **{s: representation(s) for s in ("200", "100", "50")}},
    "symbols": [symbol()],
    "poses": POSES,
    "clips": [carry_clip(), heat_clip(), door_clip()],
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {"50": volume("50"), "100": volume("100"), "200": volume("200"), "500": {"status": "unavailable", "denominator": 500}},
    "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

if __name__ == "__main__":
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(DOC, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for b, (kg, cx, cy) in MASS.items():
        print(f"[DEBUG] mass {b}: {kg:.3f} kg at ({cx:.3f}, {cy:.3f})")
    print(f"[DEBUG] cooker {COOKER_KG:.2f} kg, total {TOTAL:.2f} kg; door-open {M_OPEN:.2f} kg com x {COM_OPEN:.4f} edge {EDGE:.3f} restoring {RESTORING:.1f} Nm overturning {OVERTURN:.1f} Nm; vent {VENT_W:.0f} W")
    print({s: (len(BODY[s]), len(thermal_list(s)), len(structure_list(s))) for s in BODY})
