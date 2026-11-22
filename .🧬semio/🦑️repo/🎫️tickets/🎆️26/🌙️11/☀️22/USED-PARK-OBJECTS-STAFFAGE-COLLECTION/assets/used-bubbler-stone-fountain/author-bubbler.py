#!/usr/bin/env python3
"""⛲️ Authors the staffage asset `used-bubbler-stone-fountain`: a drilled granite boulder bubbler over a precast reservoir shaft with a submersible pump, in a gravel basin edged with granite setts.

The one authoritative script: `python3 author-bubbler.py` writes 🔣️.json next to it from `author_geometry` (numbers, masses) and `author_parts` (solids, drawings).
See ../../brief.md and the staffage README (🎓️teaching/🏛️architecture/🧍️staffage/README.md).
"""
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from author_geometry import (HINGE, HOSE_KG, LID_KG, LID_OPEN, LIFT, PI, PUMP_KG, R_IN, R_OUT, SHAFT_KG, SINK, SLAB_KG, SLAB_T, STONE_COM_DX, STONE_COM_H, STONE_KG, T, WATER_KG,
                             X_BORE, X_LID, X_STONE, Y_FLOOR, Y_SHAFT, Y_TOP, loc, placement, r6, super_area)
from author_parts import FEATURE, body, structure, thermal, views

SLUG = "used-bubbler-stone-fountain"
SCALES = ["50", "100", "200", "500"]
G = 9.80665

MATERIALS = [
    {"id": "granite-boulder", "title": T("Grey granite boulder, drilled, weathered with limescale crust and algae on the shaded flank", "Grauer Granitfindling, gebohrt, verwittert, mit Kalkkruste und Algen an der Schattenseite"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": 2650, "conductivityWmK": 2.8, "specificHeatJKgK": 820, "elasticModulusPa": 50e9, "strengthPa": 150e6, "source": "literature", "reference": "en-1936"},
    {"id": "granite-setts", "title": T("Granite small setts 9/11 in concrete bedding", "Granit-Kleinpflaster 9/11 im Betonbett"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 2650, "conductivityWmK": 2.8, "specificHeatJKgK": 820, "source": "literature", "reference": "en-1342"},
    {"id": "concrete", "title": T("Precast reinforced concrete (shaft ring, cover slab) and C12/15 haunch", "Stahlbetonfertigteil (Schachtring, Abdeckplatte) und Rückenstütze C12/15"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 34e9, "strengthPa": 45e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "mortar", "title": T("Trass-cement mortar bed under the boulder", "Trasszement-Mörtelbett unter dem Findling"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 1900, "conductivityWmK": 1.0, "specificHeatJKgK": 1000, "elasticModulusPa": 10e9, "strengthPa": 10e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "gravel", "title": T("Rounded granite gravel 16/32, bulk, with algae film and silt", "Granit-Rundkies 16/32, geschüttet, mit Algenfilm und Schlamm"), "emissivity": 0.9, "solarAbsorptivity": 0.68, "densityKgM3": 1650, "conductivityWmK": 0.7, "specificHeatJKgK": 840, "source": "literature", "reference": "en-12620"},
    {"id": "steel-galv", "title": T("Hot-dip galvanised chequer plate and frame, dull grey patina with rust spots", "Feuerverzinktes Tränenblech mit Rahmen, stumpfgraue Patina mit Rostpunkten"), "emissivity": 0.6, "solarAbsorptivity": 0.72, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "stainless", "title": T("Stainless steel 1.4301 pump housing, scratched", "Pumpengehäuse aus Edelstahl 1.4301, verkratzt"), "emissivity": 0.35, "solarAbsorptivity": 0.5, "densityKgM3": 7900, "conductivityWmK": 16, "specificHeatJKgK": 500, "elasticModulusPa": 200e9, "strengthPa": 230e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "pe-hose", "title": T("Black PE 100 pipe 25 x 2.3 mm (SDR 11)", "Schwarzes PE-100-Rohr 25 x 2,3 mm (SDR 11)"), "emissivity": 0.94, "solarAbsorptivity": 0.95, "densityKgM3": 960, "conductivityWmK": 0.4, "specificHeatJKgK": 1900, "elasticModulusPa": 1.1e9, "source": "literature", "reference": "en-12201-2"},
    {"id": "water", "title": T("Circulating fountain water, hard (about 18 degrees dH)", "Umlaufendes Brunnenwasser, hart (etwa 18 Grad dH)"), "emissivity": 0.96, "solarAbsorptivity": 0.1, "densityKgM3": 1000, "conductivityWmK": 0.6, "specificHeatJKgK": 4180, "source": "literature", "reference": "incropera-2018"},
    {"id": "limescale", "title": T("Calcite crust (limescale) deposited by evaporation", "Kalkkruste (Calcit), durch Verdunstung abgeschieden"), "emissivity": 0.92, "solarAbsorptivity": 0.3, "densityKgM3": 2500, "conductivityWmK": 1.5, "specificHeatJKgK": 840, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "leaf-litter", "title": T("Fallen lime leaves, wet", "Gefallenes Lindenlaub, nass"), "emissivity": 0.95, "solarAbsorptivity": 0.7, "densityKgM3": 700, "conductivityWmK": 0.3, "specificHeatJKgK": 3000, "source": "authored-estimate", "reference": "incropera-2018"},
]

REF = {
    "boulder-catalogue-typical": "Catalogue-typical data of drilled granite bubbler boulders (Quellsteine, Sprudelsteine) of German natural stone suppliers for public squares and parks (no brand): grey granite, 0.5 to 1.0 m high, 200 to 900 kg, a core bore of 30 to 50 mm through the stone with a ground flat outlet cup on top, a sawn base set in mortar on a cover slab over the water reservoir, service access to the pump beside the stone.",
    "pump-catalogue-typical": "Catalogue-typical data of submersible fountain pumps (no brand): 50 to 100 W electrical input, 2500 to 5000 l/h, maximum head 2.5 to 3.5 m, stainless or plastic housing with inlet strainer, IPX8, 5 to 9 kg, a lifting eye for the service rope; at about 1.5 m of head an 80 W pump delivers about 2000 l/h, which lifts a dome of 50 to 70 mm out of a 20 mm bore.",
    "en-1917": "EN 1917:2002 and DIN 4034-1:2004 Concrete manholes and inspection chambers, unreinforced, steel fibre and reinforced (DN 1200 shaft rings with 120 mm walls, base units, cover slabs of 150 to 200 mm with an eccentric opening).",
    "en-124": "EN 124-1:2015 Gully tops and manhole tops for vehicular and pedestrian areas (class A 15 for areas used only by pedestrians and cyclists: 15 kN test load; hinged covers are held open by a stay).",
    "din-59220": "DIN 59220:2000 Hot-rolled floor plates (chequer plate 8/10 mm with teardrop pattern, 63.6 kg/m2 at 8 mm) and EN ISO 1461:2022 hot-dip galvanised coatings (zinc patina, white rust and red rust spots where the coating is worn).",
    "en-1936": "EN 1936:2006 Natural stone test methods, determination of real and apparent density (granite 2600 to 2700 kg/m3); EN 1926 compressive strength (granite 120 to 250 MPa); thermal values after ISO 10456 (2.8 W/mK, 820 J/kgK).",
    "en-1342": "EN 1342:2012 Setts of natural stone for external paving (small setts 9/11 cm) and DIN 18318 / ZTV Wegebau: setts and edgings set in C12/15 concrete with a back haunch.",
    "en-12620": "EN 12620:2013 Aggregates for concrete (rounded gravel 16/32, bulk density 1550 to 1750 kg/m3; thermal conductivity of dry to moist gravel beds 0.5 to 1.0 W/mK).",
    "en-12201-2": "EN 12201-2:2011 Plastics piping systems for water supply, polyethylene (PE 100 SDR 11, 25 x 2.3 mm, 0.17 kg/m).",
    "iec-60364-7-702": "IEC 60364-7-702:2010 / DIN VDE 0100-702:2012 Low-voltage electrical installations, requirements for special installations: swimming pools and fountains (equipment in zone 0 of a fountain supplied by SELV or protected by a 30 mA residual current device; pumps IPX8).",
    "en-60335-2-41": "EN 60335-2-41:2003+A2:2010 Household and similar electrical appliances, particular requirements for pumps (submersible pumps, rating plate with electrical input power).",
    "en-1991-1-1": "EN 1991-1-1:2002 Densities, self-weight, imposed loads on buildings (persons on street furniture: concentrated loads of 1 to 1.5 kN; used as the basis of the climbing child and the seated adult).",
    "en-1991-1-4": "EN 1991-1-4:2005 + NA Germany: wind actions (peak velocity pressure about 0.5 kN/m2 at 1 m in a park; on the 0.6 m2 face of the boulder about 0.3 kN, which its 8.5 kN of weight carries by friction and overturning stability).",
    "iso-10456-2007": "ISO 10456:2007 Building materials and products, hygrothermal properties, tabulated design values (concrete 2400 kg/m3, 2.0 W/mK, 1000 J/kgK; mortar 1900 kg/m3; stainless steel 7900 kg/m3, 16 W/mK).",
    "incropera-2018": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity and solar absorptance of stone, weathered galvanised and stainless steel, water; evaporation from wetted surfaces by the heat and mass transfer analogy).",
    "winkler-1997": "Winkler, E. M. (1997). Stone in Architecture: Properties, Durability, 3rd ed., Springer (weathering of granite surfaces outdoors: darkening by biofilms and algae on shaded flanks, calcite crusts where hard water evaporates; solar absorptance of weathered grey granite 0.55 to 0.7 against about 0.5 fresh).",
}

PENS = {"50": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.18, "ground": 0.5}, "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.13, "ground": 0.35},
        "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, "500": {"body": 0.14, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
HATCHES = {
    "50": [{"id": "concrete", "title": T("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13},
           {"id": "granite", "title": T("Natural stone, stipple", "Naturstein, Punktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.9, "penMm": 0.1},
           {"id": "setts", "title": T("Setts, cross hatch", "Pflaster, Kreuzschraffur"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1},
           {"id": "gravel", "title": T("Gravel stipple", "Kiespunktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1},
           {"id": "chequer", "title": T("Chequer plate", "Tränenblech"), "pattern": "cross", "angleDeg": 30, "spacingMm": 0.7, "penMm": 0.1}] ,
    "100": [{"id": "concrete", "title": T("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1},
            {"id": "granite", "title": T("Natural stone, stipple", "Naturstein, Punktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1},
            {"id": "setts", "title": T("Setts, cross hatch", "Pflaster, Kreuzschraffur"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.7, "penMm": 0.1},
            {"id": "gravel", "title": T("Gravel stipple", "Kiespunktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.1},
            {"id": "chequer", "title": T("Chequer plate", "Tränenblech"), "pattern": "cross", "angleDeg": 30, "spacingMm": 0.6, "penMm": 0.1}],
    "200": [], "500": []}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}


def mass_centres():
    """⚖️ Local centres of mass of the three massed bones."""
    lid_u = (PI * 0.186 ** 2 * 0.008 * 7850 * 0.195 + PI * (0.16 ** 2 - 0.15 ** 2) * 0.03 * 7850 * 0.195) / LID_KG
    lid_v = PI * (0.16 ** 2 - 0.15 ** 2) * 0.03 * 7850 * 0.019 / LID_KG
    return {"stone": (r6(STONE_COM_H), r6(-STONE_COM_DX)), "lid": (r6(lid_u), r6(lid_v)), "pump": (0.075, 0.0)}


def rig():
    """🦴️ The rig: shaft root in the soil, slab and basin on it, boulder on the slab with the bubble, the hinged lid and the liftable pump."""
    def bone(id, en, de, length, joint, mass=None, section=None):
        x, y, a = placement(id)
        parent = {"shaft": None, "slab": "shaft", "basin": "shaft", "stone": "slab", "bubble": "stone", "lid": "slab", "pump": "shaft"}[id]
        b = {"id": id, "parent": parent, "title": T(en, de), "x": x, "y": y, "angleDeg": a, "lengthM": length, "joint": joint}
        if mass:
            b["mass"] = mass
        if section:
            b["section"] = section
        return b
    c = mass_centres()
    a_ring = PI * (R_OUT ** 2 - R_IN ** 2)
    i_ring = PI / 4 * (R_OUT ** 4 - R_IN ** 4)
    a_stone = super_area(0.43, 0.36, 2.54)
    return {"kind": "mechanical", "root": "shaft", "bones": [
        bone("shaft", "Precast reservoir shaft DN 1200 with base (stays in the ground)", "Schachtfertigteil DN 1200 mit Boden (bleibt im Boden)", 0.78, {"type": "fixed"},
             section={"areaM2": r6(a_ring), "secondMomentM4": r6(i_ring), "extremeFibreM": R_OUT, "material": "concrete", "title": T("Ring DN 1200, wall 120 mm", "Ring DN 1200, Wand 120 mm")}),
        bone("slab", "Precast cover slab 150 mm with eccentric service opening", "Abdeckplatte 150 mm mit außermittiger Revisionsöffnung", 2 * R_OUT, {"type": "fixed"},
             section={"areaM2": r6(2 * R_OUT * SLAB_T), "secondMomentM4": r6(2 * R_OUT * SLAB_T ** 3 / 12), "extremeFibreM": SLAB_T / 2, "material": "concrete", "title": T("Slab 1440 x 150 mm", "Platte 1440 x 150 mm")}),
        bone("basin", "Gravel basin with granite sett edging and concrete haunch", "Kiesbecken mit Granitpflaster-Einfassung und Betonrückenstütze", 0.05, {"type": "fixed"}),
        bone("stone", "Drilled granite boulder bedded on the slab", "Gebohrter Granitfindling, auf der Platte vermörtelt", 0.80, {"type": "fixed"},
             mass={"kg": r6(STONE_KG + HOSE_KG), "comX": c["stone"][0], "comY": c["stone"][1], "distribution": "point", "source": "literature", "reference": "en-1936"},
             section={"areaM2": r6(a_stone), "secondMomentM4": r6(PI / 4 * 0.43 ** 3 * 0.36), "extremeFibreM": 0.43, "material": "granite-boulder", "title": T("Boulder at its girth, 860 x 720 mm", "Findling am größten Umfang, 860 x 720 mm")}),
        bone("bubble", "Water dome above the bore (sinks into the bore when the pump stops)", "Wasserkuppe über der Bohrung (sinkt in die Bohrung, wenn die Pumpe steht)", 0.056, {"type": "prismatic", "minM": -SINK, "maxM": 0.0, "axisDeg": 0}),
        bone("lid", "Hinged galvanised chequer plate lid of the service opening", "Klappdeckel aus verzinktem Tränenblech über der Revisionsöffnung", 0.38, {"type": "revolute", "minDeg": LID_OPEN, "maxDeg": 0},
             mass={"kg": r6(LID_KG), "comX": c["lid"][0], "comY": c["lid"][1], "distribution": "point", "source": "literature", "reference": "din-59220"},
             section={"areaM2": 0.003576, "secondMomentM4": 2.41e-07, "extremeFibreM": 0.0308, "material": "steel-galv", "title": T("Plate 372 x 8 mm with 10 x 30 mm rib", "Blech 372 x 8 mm mit Rippe 10 x 30 mm")}),
        bone("pump", "Submersible fountain pump on the shaft floor (lifted out for service)", "Unterwasser-Brunnenpumpe auf dem Schachtboden (zur Wartung herausgehoben)", 0.21, {"type": "prismatic", "minM": 0.0, "maxM": 1.0, "axisDeg": 0},
             mass={"kg": r6(PUMP_KG), "comX": c["pump"][0], "comY": c["pump"][1], "distribution": "point", "source": "authored-estimate", "reference": "pump-catalogue-typical"},
             section={"areaM2": r6(2 * (0.24 + 0.16) * 0.002), "secondMomentM4": r6(2 * 0.002 * 0.24 ** 3 / 12 + 2 * 0.16 * 0.002 * 0.12 ** 2), "extremeFibreM": 0.12, "material": "stainless", "title": T("Housing shell 240 x 160 x 2 mm", "Gehäusemantel 240 x 160 x 2 mm")}),
    ]}


def anchors():
    """⚓️ Soil bearing, the evaporation outlet, the pump lifting eye and the visitor loads."""
    top_u = Y_TOP - (-0.05)
    knee = loc("stone", -0.02, 0.62)
    seat = loc("stone", -0.60, 0.42)
    return [
        {"id": "shaft-soil", "bone": "shaft", "x": 0, "y": 0, "kind": "support", "title": T("Shaft base bearing on the compacted gravel sub-base", "Schachtboden auf dem verdichteten Kiesplanum"), "support": "contact", "footprintLengthM": 2 * R_OUT, "footprintWidthM": 2 * R_OUT, "frictionCoefficient": 0.6},
        {"id": "outlet", "bone": "stone", "x": r6(top_u), "y": r6(-(X_BORE - X_STONE)), "kind": "emitter", "title": T("Wet boulder top where the water evaporates", "Nasser Findlingsscheitel, an dem das Wasser verdunstet")},
        {"id": "pump-eye", "bone": "pump", "x": 0.21, "y": 0, "kind": "load", "title": T("Lifting eye of the pump on the service rope", "Hebeöse der Pumpe am Wartungsseil")},
        {"id": "child-knee", "bone": "stone", "x": r6(knee[0]), "y": r6(knee[1]), "kind": "load", "title": T("Knee of a child climbing onto the boulder", "Knie eines Kindes, das auf den Findling klettert")},
        {"id": "seat", "bone": "stone", "x": r6(seat[0]), "y": r6(seat[1]), "kind": "load", "title": T("Seat of an adult on the low rear flank", "Sitz eines Erwachsenen auf der niedrigen hinteren Flanke")},
    ]


SUN = {"granite-stone": 0.5, "bubble-water": 0.8, "gravel-bed": 0.7, "edging-setts": 0.8, "lid-steel": 0.85, "civil-concrete": 0.0, "pump-motor": 0.0, "reservoir-water": 0.0, "riser-hose": 0.0}


def regions(running, lifted=False):
    """🌡️ Region bindings of a pose."""
    out = {}
    for rid, sun in SUN.items():
        b = {"exposedFraction": 1.0, "sunlitFraction": sun}
        if rid == "granite-stone":
            b["wettedness"] = 0.4 if running else 0.05
            b["contact"] = {"fraction": 0.1, "tempC": 17}
        if rid == "bubble-water":
            b["exposedFraction"] = 1.0 if running else 0.0
            b["wettedness"] = 1.0
            b["contact"] = {"fraction": 0.8, "tempC": 16}
        if rid == "gravel-bed":
            b["wettedness"] = 0.5 if running else 0.2
            b["contact"] = {"fraction": 0.45, "tempC": 17}
        if rid == "edging-setts":
            b["contact"] = {"fraction": 0.6, "tempC": 16}
        if rid == "civil-concrete":
            b["contact"] = {"fraction": 0.9, "tempC": 12}
        if rid == "reservoir-water":
            b["wettedness"] = 1.0
            b["contact"] = {"fraction": 0.7, "tempC": 13}
        if rid == "riser-hose":
            b["contact"] = {"fraction": 0.9, "tempC": 16}
        if rid == "pump-motor":
            if lifted:
                b.update({"sunlitFraction": 0.5, "wettedness": 0.6})
            else:
                b["contact"] = {"fraction": 1.0, "tempC": 16}
        out[rid] = b
    return out


def pose(id, en, de, den, dde, running, bones=None, loads=None, lifted=False, key=False):
    """🧍️ A pose."""
    p = {"id": id, "title": T(en, de), "description": T(den, dde), "bones": bones or {}, "ground": "authored", "contacts": ["shaft-soil"],
         "thermal": {"regions": regions(running, lifted), "emitters": {"evaporation": 1 if running else 0}, "operating": {"pump": 1 if running else 0}}, "loads": loads or []}
    if key:
        p["key"] = True
    return p


def poses():
    """🎬️ The poses: running in service, switched off, lid opened, pump lifted for service, visitors on the boulder."""
    off = {"bubble": {"dx": -SINK}}
    return [
        pose("in-service", "In service, bubbling on a summer afternoon", "Im Betrieb, sprudelnd an einem Sommernachmittag",
             "The pump in the shaft lifts about 2000 l/h through the bore; a 56 mm water dome wells out of the cup on the boulder top, runs down the front flank and seeps through the gravel back into the reservoir; the wet stone and gravel evaporate in the sun while the dry galvanised lid heats up.",
             "Die Pumpe im Schacht fördert etwa 2000 l/h durch die Bohrung; eine Wasserkuppe von 56 mm quillt aus der Mulde auf dem Findlingsscheitel, läuft über die vordere Flanke und versickert durch den Kies zurück in den Speicher; nasser Stein und Kies verdunsten in der Sonne, während sich der trockene verzinkte Deckel aufheizt.", True),
        pose("pump-off", "Switched off on a hot afternoon", "An einem heißen Nachmittag abgeschaltet",
             "The timer has stopped the pump (water restriction during a drought): the dome sinks back into the bore, the film on the boulder dries within half an hour and the dark granite heats up in the sun like the lid.",
             "Die Zeitschaltuhr hat die Pumpe abgeschaltet (Wassersparen in einer Trockenperiode): die Kuppe sinkt in die Bohrung zurück, der Film auf dem Findling trocknet binnen einer halben Stunde und der dunkle Granit heizt sich in der Sonne auf wie der Deckel.", False, bones=off),
        pose("lid-open", "Service lid opened", "Revisionsdeckel geöffnet",
             "The depot worker has switched the pump off at the RCD box, raked the gravel off the frame and swung the hinged lid 90 degrees up onto its stay.",
             "Der Bauhofmitarbeiter hat die Pumpe am FI-Kasten abgeschaltet, den Kies vom Rahmen gerecht und den Klappdeckel um 90 Grad auf seine Stütze hochgeschwenkt.", False, bones={"bubble": {"dx": -SINK}, "lid": {"angleDeg": LID_OPEN}}, key=True),
        pose("maintenance", "Maintenance, pump lifted out", "Wartung, Pumpe herausgehoben",
             "With the lid on its stay, the worker pulls the 8.6 kg pump 0.96 m up on its service rope through the opening after uncoupling the hose; it hangs dripping above the frame for the strainer to be cleaned of leaves and limescale. Boulder, slab and shaft stay where they are.",
             "Bei aufgestelltem Deckel zieht der Mitarbeiter die Pumpe von 8,6 kg nach dem Abkuppeln des Schlauchs am Wartungsseil 0,96 m durch die Öffnung hoch; sie hängt tropfend über dem Rahmen, damit der Ansaugkorb von Laub und Kalk gereinigt wird. Findling, Platte und Schacht bleiben an Ort.", False,
             bones={"bubble": {"dx": -SINK}, "lid": {"angleDeg": LID_OPEN}, "pump": {"dx": LIFT}}, lifted=True,
             loads=[{"id": "rope", "kind": "equipment", "title": T("Service rope held by the worker", "Vom Mitarbeiter gehaltenes Wartungsseil"), "anchor": "pump-eye", "fxN": 0, "fyN": r6(round(PUMP_KG * G + 2.0, 1)), "source": "authored-estimate", "reference": "pump-catalogue-typical"}]),
        pose("in-use", "In use, children at the bubble", "In Benutzung, Kinder am Sprudel",
             "While it bubbles, a child kneels on the front shoulder of the boulder and holds both hands into the dome (0.30 kN down, 0.05 kN pushed back), and a parent sits on the low rear flank (0.75 kN down, 0.08 kN pushed forward).",
             "Während es sprudelt, kniet ein Kind auf der vorderen Schulter des Findlings und hält beide Hände in die Kuppe (0,30 kN nach unten, 0,05 kN nach hinten geschoben), und ein Elternteil sitzt auf der niedrigen hinteren Flanke (0,75 kN nach unten, 0,08 kN nach vorn geschoben).", True,
             loads=[{"id": "child", "kind": "occupant", "title": T("Kneeling child", "Kniendes Kind"), "anchor": "child-knee", "fxN": -50, "fyN": -300, "source": "authored-estimate", "reference": "en-1991-1-1"},
                    {"id": "sitter", "kind": "occupant", "title": T("Seated parent", "Sitzender Elternteil"), "anchor": "seat", "fxN": 80, "fyN": -750, "source": "authored-estimate", "reference": "en-1991-1-1"}]),
    ]


def clips():
    """🎞️ Maintenance lift and the start-up of the bubble."""
    return [
        {"id": "maintenance", "title": T("Pump service", "Pumpenwartung"),
         "description": T("The pump is switched off and the dome sinks into the bore; the lid is swung up onto its stay, then the pump is pulled 0.96 m up on its rope through the opening.", "Die Pumpe wird abgeschaltet und die Kuppe sinkt in die Bohrung; der Deckel wird auf seine Stütze hochgeschwenkt, dann wird die Pumpe am Seil 0,96 m durch die Öffnung hochgezogen."),
         "durationS": 40, "fps": 10, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "linear"}, {"t": 1, "pose": "in-service", "ease": "linear"}, {"t": 4, "pose": "pump-off", "ease": "smooth"}, {"t": 8, "pose": "pump-off", "ease": "smooth"},
                  {"t": 14, "pose": "lid-open", "ease": "smooth"}, {"t": 17, "pose": "lid-open", "ease": "smooth"}, {"t": 36, "pose": "maintenance", "ease": "smooth"}, {"t": 40, "pose": "maintenance", "ease": "linear"}],
         "contacts": [{"anchor": "shaft-soil", "fromS": 0, "toS": 40}],
         "phases": [{"id": "switch-off", "fromS": 0, "toS": 8, "title": T("Pump off, dome sinks", "Pumpe aus, Kuppe sinkt")}, {"id": "open", "fromS": 8, "toS": 17, "title": T("Lid swung up", "Deckel hochgeschwenkt")}, {"id": "lift", "fromS": 17, "toS": 40, "title": T("Pump pulled out", "Pumpe herausgezogen")}],
         "events": [{"t": 1, "kind": "shutdown", "title": T("Pump switched off at the RCD box", "Pumpe am FI-Kasten abgeschaltet"), "target": "pump"}, {"t": 8, "kind": "motion", "title": T("Lid unlocked and lifted", "Deckel entriegelt und angehoben")},
                    {"t": 14, "kind": "motion", "title": T("Lid on its stay", "Deckel auf der Stütze")}, {"t": 17, "kind": "load-on", "title": T("Rope takes the pump", "Seil übernimmt die Pumpe"), "target": "rope"}, {"t": 36, "kind": "motion", "title": T("Pump clear of the frame", "Pumpe über dem Rahmen")}],
         "tracks": [{"target": "operating:pump", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1, "v": 0}]}, {"target": "emitter:evaporation", "ease": "linear", "keys": [{"t": 0, "v": 1}, {"t": 1, "v": 1}, {"t": 30, "v": 0.2}, {"t": 40, "v": 0.1}]},
                    {"target": "region:granite-stone:wettedness", "ease": "linear", "keys": [{"t": 0, "v": 0.4}, {"t": 40, "v": 0.3}]}]},
        {"id": "start-up", "title": T("Bubbling up", "Aufsprudeln"),
         "description": T("The timer starts the pump: after a second and a half the water rises out of the bore into a dome that pulses with the air the pump draws in, and the boulder flank and the gravel wet again.", "Die Zeitschaltuhr startet die Pumpe: nach anderthalb Sekunden steigt das Wasser aus der Bohrung zu einer Kuppe, die mit der angesaugten Luft pulsiert, und Findlingsflanke und Kies werden wieder nass."),
         "durationS": 14, "fps": 20, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "pump-off", "ease": "linear"}, {"t": 1.5, "pose": "pump-off", "ease": "out"}, {"t": 4, "pose": "in-service", "ease": "smooth"},
                  {"t": 5, "pose": "in-service", "ease": "smooth", "bones": {"bubble": {"dx": -0.010}}}, {"t": 6, "pose": "in-service", "ease": "smooth"},
                  {"t": 7, "pose": "in-service", "ease": "smooth", "bones": {"bubble": {"dx": -0.012}}}, {"t": 8, "pose": "in-service", "ease": "smooth"},
                  {"t": 9.5, "pose": "in-service", "ease": "smooth", "bones": {"bubble": {"dx": -0.008}}}, {"t": 11, "pose": "in-service", "ease": "smooth"},
                  {"t": 12.5, "pose": "in-service", "ease": "smooth", "bones": {"bubble": {"dx": -0.011}}}, {"t": 14, "pose": "in-service", "ease": "smooth"}],
         "contacts": [{"anchor": "shaft-soil", "fromS": 0, "toS": 14}],
         "phases": [{"id": "still", "fromS": 0, "toS": 1.5, "title": T("Still, dry top", "Ruhig, trockener Scheitel")}, {"id": "rise", "fromS": 1.5, "toS": 4, "title": T("Water rises out of the bore", "Wasser steigt aus der Bohrung")}, {"id": "bubbling", "fromS": 4, "toS": 14, "title": T("Pulsing dome", "Pulsierende Kuppe")}],
         "events": [{"t": 1.5, "kind": "ignition", "title": T("Timer starts the pump", "Zeitschaltuhr startet die Pumpe"), "target": "pump"}, {"t": 4, "kind": "motion", "title": T("Dome at full height", "Kuppe in voller Höhe")}],
         "tracks": [{"target": "operating:pump", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.5, "v": 1}]}, {"target": "emitter:evaporation", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 1.5, "v": 0}, {"t": 14, "v": 1}]},
                    {"target": "region:granite-stone:wettedness", "ease": "linear", "keys": [{"t": 0, "v": 0.05}, {"t": 1.5, "v": 0.05}, {"t": 14, "v": 0.4}]}]},
    ]


AREAS = {"granite-stone": 2.3, "bubble-water": 0.013, "gravel-bed": 4.17, "edging-setts": 2.26, "civil-concrete": 9.0, "lid-steel": 0.361, "pump-motor": 0.261, "reservoir-water": 3.6, "riser-hose": 0.15}


def thermal_model():
    """🌡️ The thermal model: environment, the pump key, nine regions, the evaporation vent."""
    def region(id, en, de, kind, material, length, geom, sky, layer, extra=None):
        lid_, len_, lde, lk, lm, t = layer
        m = next(x for x in MATERIALS if x["id"] == lm)
        cap = r6(m["densityKgM3"] * m["specificHeatJKgK"] * t)
        r = {"id": id, "title": T(en, de), "kind": kind, "model": "passive", "material": material, "areaM2": AREAS[id], "characteristicLengthM": length, "geometry": geom, "skyViewFactor": sky,
             "layers": [{"id": lid_, "title": T(len_, lde), "kind": lk, "material": lm, "thicknessM": t, "resistanceM2KW": r6(t / m["conductivityWmK"]), "heatCapacityJM2K": cap, "source": "literature", "reference": "iso-10456-2007"}],
             "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "incropera-2018"}
        if extra:
            r.update(extra)
        return r
    hid = lambda en, de: {"hidden": T(en, de)}
    return {
        "environment": {"airTempC": 28, "meanRadiantTempC": 33, "skyTempC": 12, "groundTempC": 34, "relativeHumidity": 0.4, "windMS": 1.0, "windDirectionDeg": 0, "solarWM2": 800, "diffuseFraction": 0.2, "pressurePa": 101325},
        "passengers": [],
        "operating": [{"id": "pump", "title": T("Submersible fountain pump", "Unterwasser-Brunnenpumpe"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Running (80 W input, about 2000 l/h)", "Läuft (80 W Aufnahme, etwa 2000 l/h)")}]}],
        "regions": [
            region("granite-stone", "Drilled granite boulder, wet film where it runs, algae on the shaded rear", "Gebohrter Granitfindling, nasser Film, wo es läuft, Algen an der schattigen Rückseite", "surface", "granite-boulder", 0.8, "sphere", 0.7,
                   ("granite", "Thermally active granite 100 mm", "Thermisch aktiver Granit 100 mm", "masonry", "granite-boulder", 0.1)),
            region("bubble-water", "Water dome over the bore", "Wasserkuppe über der Bohrung", "water", "water", 0.08, "sphere", 0.9, ("dome", "Water 30 mm", "Wasser 30 mm", "polymer", "water", 0.03)),
            region("gravel-bed", "Gravel bed of the basin, wet near the boulder", "Kiesbett des Beckens, nahe am Findling nass", "surface", "gravel", 1.8, "horizontal-plate-up", 0.9, ("gravel", "Gravel 50 mm", "Kies 50 mm", "masonry", "gravel", 0.05)),
            region("edging-setts", "Granite sett edging", "Granitpflaster-Einfassung", "surface", "granite-setts", 0.1, "horizontal-plate-up", 0.95, ("sett", "Granite sett 100 mm", "Granitstein 100 mm", "masonry", "granite-setts", 0.1)),
            region("civil-concrete", "Shaft ring, cover slab and sett haunch in the ground", "Schachtring, Abdeckplatte und Rückenstütze im Boden", "surface", "concrete", 1.2, "vertical-plate", 0.0, ("wall", "Concrete 120 mm", "Beton 120 mm", "masonry", "concrete", 0.12)),
            region("lid-steel", "Galvanised chequer plate lid and frame", "Verzinkter Tränenblechdeckel mit Rahmen", "surface", "steel-galv", 0.37, "horizontal-plate-up", 0.9, ("plate", "Steel 8 mm", "Stahl 8 mm", "metal", "steel-galv", 0.008)),
            region("pump-motor", "Submersible pump housing (motor losses released into the water)", "Gehäuse der Unterwasserpumpe (Motorverluste ins Wasser abgegeben)", "surface", "stainless", 0.14, "cylinder", 0.0, ("shell", "Stainless steel 2 mm", "Edelstahl 2 mm", "metal", "stainless", 0.002),
                   dict(hid("Submerged on the floor of the closed shaft in service; seen only when it is lifted out.", "Im Betrieb auf dem Boden des geschlossenen Schachts untergetaucht; nur herausgehoben sichtbar."), model="operating", operating={"key": "pump", "heatW": 56}, ratedPowerW=80, ratedPowerBasis="electrical-input", source="authored-estimate", reference="pump-catalogue-typical")),
            region("reservoir-water", "Reservoir water in the shaft (about 400 l)", "Speicherwasser im Schacht (etwa 400 l)", "water", "water", 1.2, "horizontal-plate-up", 0.0, ("water", "Water 100 mm", "Wasser 100 mm", "polymer", "water", 0.1),
                   hid("Inside the closed shaft under the cover slab and the lid.", "Im geschlossenen Schacht unter Abdeckplatte und Deckel.")),
            region("riser-hose", "PE riser hose in the bore and the shaft", "PE-Steigleitung in der Bohrung und im Schacht", "surface", "pe-hose", 0.025, "cylinder", 0.0, ("pipe", "PE 2.3 mm", "PE 2,3 mm", "polymer", "pe-hose", 0.0023),
                   hid("Grouted in the bore of the boulder and submerged in the shaft.", "In der Bohrung des Findlings vergossen und im Schacht untergetaucht.")),
        ],
        "emitters": [{"id": "evaporation", "kind": "vent", "title": T("Vapour evaporating from the wet boulder", "Vom nassen Findling verdunstender Wasserdampf"), "anchor": "outlet", "directionDeg": 90, "diameterM": 0.3, "operatingKey": "pump", "volumeFlowM3S": 0.004, "temperatureRiseK": -1.5, "vapourKgPerKg": 0.004, "region": "granite-stone", "source": "authored-estimate", "reference": "incropera-2018"}],
        "assumptions": [
            T("Reference case: a clear summer afternoon in a German city park, 28 degC air at 40 percent relative humidity, 800 W/m2 of sun, 1 m/s of wind. No surface temperature is authored; every surface follows its heat balance, the wetted ones including evaporation.", "Referenzfall: ein klarer Sommernachmittag in einem deutschen Stadtpark, Luft 28 degC bei 40 Prozent relativer Feuchte, 800 W/m2 Sonne, 1 m/s Wind. Keine Oberflächentemperatur ist festgelegt; jede Oberfläche folgt ihrer Wärmebilanz, die benetzten einschließlich der Verdunstung."),
            T("Water: the reservoir and the dome are held towards the reservoir temperature (16 degC) by contact; while the pump runs the boulder is wet over about 40 percent of its surface and the gravel over half, so evaporation cools them below the air, while the dry galvanised lid beside them is the hot surface. Switched off, the film dries and the granite heats up. Evaporation in summer is about 3 l/m2 a day, about 5 l a day from the wet surfaces, topped up by the float valve of the reservoir.", "Wasser: Speicher und Kuppe werden über Kontakt zur Speichertemperatur (16 degC) gezogen; bei laufender Pumpe ist der Findling auf etwa 40 Prozent seiner Oberfläche nass und der Kies zur Hälfte, so dass die Verdunstung sie unter die Lufttemperatur kühlt, während der trockene verzinkte Deckel daneben die heiße Fläche ist. Abgeschaltet trocknet der Film und der Granit heizt sich auf. Die Verdunstung beträgt im Sommer etwa 3 l/m2 am Tag, etwa 5 l am Tag von den nassen Flächen, nachgespeist über das Schwimmerventil des Speichers."),
            T("Weathering: the grey granite has darkened by biofilm and algae on the shaded rear flank, so its solar absorptance is 0.62 against about 0.5 fresh (Winkler 1997); the galvanised lid has a dull patina with emissivity 0.6 instead of about 0.25 new; limescale crusts the outlet and the runs of the water.", "Verwitterung: der graue Granit ist durch Biofilm und Algen an der schattigen Rückseite nachgedunkelt, sein solarer Absorptionsgrad ist 0,62 statt etwa 0,5 frisch (Winkler 1997); der verzinkte Deckel hat eine stumpfe Patina mit Emissionsgrad 0,6 statt etwa 0,25 neu; Kalk verkrustet den Auslauf und die Laufspuren des Wassers."),
            T("Power: the nameplate of the asset is the 80 W electrical input of the submersible pump (catalogue-typical, EN 60335-2-41); about 56 W of motor and hydraulic losses are released at its housing into the reservoir water. Every other element is passive.", "Leistung: die Nennleistung des Objekts ist die elektrische Aufnahme der Unterwasserpumpe von 80 W (katalogtypisch, EN 60335-2-41); etwa 56 W Motor- und Hydraulikverluste werden an ihrem Gehäuse an das Speicherwasser abgegeben. Alle anderen Elemente sind passiv."),
            T("Ground: shaft, slab and haunch exchange with the soil at 12 degC through their contact fraction; the hidden regions (pump, reservoir water, riser hose) lie in the closed shaft or the bore.", "Boden: Schacht, Platte und Rückenstütze tauschen über ihren Kontaktanteil mit dem Erdreich bei 12 degC; die verdeckten Regionen (Pumpe, Speicherwasser, Steigleitung) liegen im geschlossenen Schacht oder in der Bohrung."),
        ],
    }


def structure_model():
    """🏗️ Structure assumptions."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
        T("Load path in service: the 8.5 kN boulder bears through its 30 mm trass mortar bed on the precast cover slab (150 mm, spanning the DN 1200 ring), the slab on the 120 mm ring wall and the ring through its base on the compacted sub-base (support shaft-soil, a contact with a 1.44 m footprint and friction 0.6). The slab sees about 8.5 kN eccentric by 0.25 m on a 1.2 m span, roughly 2.5 kNm, far below the A 15 / pedestrian design of a standard cover slab.", "Lastpfad im Betrieb: der Findling von 8,5 kN lastet über sein Trassmörtelbett von 30 mm auf der Abdeckplatte (150 mm, über den Ring DN 1200 gespannt), die Platte auf der Ringwand von 120 mm und der Ring über seinen Boden auf dem verdichteten Planum (Auflager shaft-soil, ein Kontakt mit 1,44 m Aufstandsfläche und Reibung 0,6). Die Platte erhält etwa 8,5 kN mit 0,25 m Ausmitte auf 1,2 m Spannweite, rund 2,5 kNm, weit unter der Bemessung A 15 / Fußgänger einer Standardplatte."),
        T("Masses: boulder 864 kg from its lofted volume of 0.326 m3 at 2650 kg/m3 plus 0.9 kg of riser hose with its water; lid 9.3 kg from 8 mm chequer plate, rib and hinge; pump 8.6 kg (2 mm stainless shell, motor core, base). The shaft (about 1.26 t), the slab (about 0.56 t), the gravel, setts, haunch and the 400 l of water are civil works in the ground and carry no mass in this asset; dimensions.massKg is what the depot can move.", "Massen: Findling 864 kg aus seinem gelofteten Volumen von 0,326 m3 bei 2650 kg/m3 plus 0,9 kg Steigleitung mit Wasser; Deckel 9,3 kg aus Tränenblech 8 mm, Rippe und Scharnier; Pumpe 8,6 kg (Edelstahlmantel 2 mm, Motorkern, Fuß). Schacht (etwa 1,26 t), Platte (etwa 0,56 t), Kies, Pflaster, Rückenstütze und die 400 l Wasser sind Tiefbau im Boden und tragen in diesem Objekt keine Masse; dimensions.massKg ist, was der Bauhof bewegen kann."),
        T("Maintenance: the rope of the worker is applied as an upward load of the pump's weight at its lifting eye, not as a second support: the structural solver treats the whole rig as one rigid body on supports of equal stiffness and would share the boulder's weight with the rope (see requests.md). The lid on its stay hangs on its hinge into the frame.", "Wartung: das Seil des Mitarbeiters ist als aufwärts gerichtete Last vom Gewicht der Pumpe an ihrer Hebeöse angesetzt, nicht als zweites Auflager: der Tragwerkslöser behandelt das ganze Skelett als einen starren Körper auf Auflagern gleicher Steifigkeit und würde das Gewicht des Findlings mit dem Seil teilen (siehe requests.md). Der Deckel auf seiner Stütze hängt über das Scharnier im Rahmen."),
        T("In use: a kneeling child (0.30 kN, with a dynamic share for climbing) and a seated adult (0.75 kN) after EN 1991-1-1 for persons on street furniture; the boulder is no play device under EN 1176, its stability comes from its mass and its mortar bed with two stainless dowels. Wind on the boulder (about 0.3 kN after EN 1991-1-4) is not posed.", "In Benutzung: ein kniendes Kind (0,30 kN, mit dynamischem Anteil für das Klettern) und ein sitzender Erwachsener (0,75 kN) nach EN 1991-1-1 für Personen an Stadtmobiliar; der Findling ist kein Spielgerät nach EN 1176, seine Standsicherheit kommt aus seiner Masse und seinem Mörtelbett mit zwei Edelstahldübeln. Wind auf den Findling (etwa 0,3 kN nach EN 1991-1-4) ist nicht als Pose geführt."),
    ], "clinical": False}


from author_texts import REP_DECISIONS, VOL_DECISIONS, SUMMARY, NOTES


def build():
    """🏗️ Assembles the asset document."""
    reps, vols = {}, {}
    masses = mass_centres()
    section = {"plane": 0.0, "label": T("A-A", "A-A")}
    for s in SCALES:
        v = views(s, PENS[s])
        b = body(s)
        reps[s] = {"status": "authored", "denominator": int(s), "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": HATCHES[s],
                   "geometry": {"elevation": v.e, "front": v.f, "plan": v.p}, "sections": {"elevation": section}, "decisions": REP_DECISIONS[s]}
        vols[s] = {"status": "authored", "denominator": int(s), "body": b, "thermal": thermal(s, b), "structure": structure(s, b, masses), "decisions": VOL_DECISIONS[s]}
    total = round(STONE_KG + HOSE_KG + LID_KG + PUMP_KG, 3)
    asset = {
        "id": SLUG, "category": "objects",
        "title": T("Used Park Bubbler Fountain", "Gebrauchter Sprudelstein"),
        "summary": SUMMARY,
        "tags": ["used", "park", "public-space", "water", "fountain", "bubbler-stone", "granite", "boulder", "submersible-pump", "gravel", "limescale", "reservoir-shaft", "moisture-source"],
        "provenance": {"author": "Semio staffage content (used park objects wave, row 26)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": k, "citation": v} for k, v in REF.items()], "notes": NOTES},
        "dimensions": {"heightM": r6(Y_TOP + 0.056 - Y_SHAFT), "widthM": 2.0, "depthM": 2.0, "massKg": total},
        "materials": MATERIALS, "rig": rig(), "anchors": anchors(), "representations": reps, "symbols": [], "poses": poses(), "clips": clips(),
        "thermal": thermal_model(), "structure": structure_model(), "volumes": vols,
        "presentation": {"defaultPose": "in-service", "defaultClip": "start-up", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = build()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print("[DEBUG] stone", round(STONE_KG, 1), "hose", round(HOSE_KG, 2), "lid", round(LID_KG, 2), "pump", round(PUMP_KG, 2), "total", doc["asset"]["dimensions"]["massKg"], "ground works kg", round(SHAFT_KG), round(SLAB_KG), round(WATER_KG))
