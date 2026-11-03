import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
raw = path.read_text(encoding="utf8", newline="")
doc = json.loads(raw)
a = doc["asset"]


def t(en, de):
    return {"en": en, "de": de}


a["summary"] = t(
    "A 0.62 m desiccant wheel seen face-on: a honeycomb silica-gel matrix on spokes turns slowly inside a casing ring; coil-heated regeneration air at 70 degC dries it through the hot sector, and the process air leaves dry and warmed by the adsorption heat.",
    "Ein 0,62 m großes Sorptionsrad in der Frontansicht: eine Silicagel-Wabenmatrix auf Speichen dreht sich langsam in einem Gehäusering; durch den heißen Sektor trocknet vom Heizregister auf 70 degC erwärmte Regenerationsluft sie aus, und die Prozessluft verlässt das Rad trocken und durch die Adsorptionswärme erwärmt.",
)

refs = a["provenance"]["references"]
refs.append({"id": "ashrae-2020-equipment", "citation": "ASHRAE (2020). Handbook - HVAC Systems and Equipment, SI edition, Atlanta (desiccant dehumidification and pressure-drying equipment)."})
refs.append({"id": "henning-2007", "citation": "Henning, H.-M. (2007). Solar assisted air conditioning of buildings - an overview. Applied Thermal Engineering 27(10), 1734-1749."})

a["provenance"]["notes"] = t(
    "A rotary desiccant dehumidifier wheel of a small sorption air handler: a 0.57 m silica-gel coated honeycomb matrix of 0.2 m depth in a 0.62 m galvanised casing ring on a stand, driven by a belt motor at 12 revolutions per hour (5 minutes per turn); a 90 degree sector is closed off for regeneration. The mass is 22 kg (casing and stand 6, matrix 14, sector plates and seals 2). Air side at the room reference (21 degC, 7.8 g/kg): the process stream of 0.4 m3/s (1440 m3/h, 2.1 m/s over the 270 degree face of 0.19 m2) loses 3.4 g/kg of moisture, which releases 0.48 kg/s x 3.4 g/kg x 2.8 MJ/kg = 4.6 kW of adsorption heat; with about 0.9 kW carried over by the matrix from the regeneration sector (14 kg x 920 J/kgK x 20 K per 300 s) the dry air leaves 11 K warmer at 4.4 g/kg. The regeneration stream of 0.14 m3/s (a third of the process flow over the 90 degree sector of 0.064 m2) is heated by the upstream heating coil to 70 degC (7.9 kW from room air), takes up 4.6 kW of desorption heat and 0.9 kW for the matrix, and leaves about 33 K above the room at 17.8 g/kg. That is a thermal coefficient of performance of 4.6 / 7.9 = 0.6, the usual range of solar-assisted desiccant cooling. The surface heats are only the free-convection and radiation losses of the matrix faces at the temperatures the air sets: 0.064 m2 at about 60 degC loses 28 W, 0.19 m2 at about 32 degC loses 18 W. The origin is the centre of the casing on the base line of the stand, x at the hub.",
    "Ein rotierendes Sorptionsrad eines kleinen Sorptionslüftungsgeräts: eine silicagelbeschichtete Wabenmatrix von 0,57 m Durchmesser und 0,2 m Tiefe in einem 0,62 m großen verzinkten Gehäusering auf einem Fuß, über einen Riemenmotor mit 12 Umdrehungen je Stunde (5 Minuten je Umdrehung) angetrieben; ein 90-Grad-Sektor ist für die Regeneration abgetrennt. Die Masse beträgt 22 kg (Gehäuse und Fuß 6, Matrix 14, Sektorbleche und Dichtungen 2). Luftseite am Raumbezug (21 degC, 7,8 g/kg): der Prozessstrom von 0,4 m3/s (1440 m3/h, 2,1 m/s über die 270-Grad-Fläche von 0,19 m2) verliert 3,4 g/kg Feuchte, das setzt 0,48 kg/s x 3,4 g/kg x 2,8 MJ/kg = 4,6 kW Adsorptionswärme frei; mit rund 0,9 kW, die die Matrix aus dem Regenerationssektor mitbringt (14 kg x 920 J/kgK x 20 K je 300 s), verlässt die trockene Luft das Rad 11 K wärmer mit 4,4 g/kg. Der Regenerationsstrom von 0,14 m3/s (ein Drittel des Prozessstroms über den 90-Grad-Sektor von 0,064 m2) wird vom vorgeschalteten Heizregister auf 70 degC erwärmt (7,9 kW aus Raumluft), nimmt 4,6 kW Desorptionswärme und 0,9 kW für die Matrix auf und verlässt das Rad rund 33 K über dem Raum mit 17,8 g/kg. Das ergibt eine thermische Leistungszahl von 4,6 / 7,9 = 0,6, den üblichen Bereich solargestützter Sorptionskühlung. Die Oberflächenwärmen sind nur die Verluste der Matrixflächen durch freie Konvektion und Strahlung bei den Temperaturen, die die Luft vorgibt: 0,064 m2 bei rund 60 degC verlieren 28 W, 0,19 m2 bei rund 32 degC verlieren 18 W. Der Ursprung liegt auf der Grundlinie des Fußes, x an der Nabe.",
)

a["anchors"] += [
    {"id": "process-collar", "bone": "casing", "x": 0.2, "y": 0.12, "kind": "emitter", "title": t("Dry process air leaving the lower right of the disc", "Trockene Prozessluft unten rechts an der Scheibe")},
    {"id": "regen-collar", "bone": "casing", "x": 0.27, "y": 0.31, "kind": "emitter", "title": t("Moist regeneration air leaving the hot sector", "Feuchte Regenerationsluft am heißen Sektor")},
]

th = a["thermal"]
th["operating"] = [
    {"id": "rotation", "title": t("Wheel drive", "Radantrieb"), "levels": [
        {"v": 0, "title": t("Off", "Aus")},
        {"v": 0.5, "title": t("Slow (6 rev/h)", "Langsam (6 U/h)")},
        {"v": 1, "title": t("Rated (12 rev/h)", "Nenndrehzahl (12 U/h)")},
    ]},
    {"id": "regeneration", "title": t("Regeneration heating coil", "Heizregister der Regeneration"), "levels": [
        {"v": 0, "title": t("Coil off (room air)", "Register aus (Raumluft)")},
        {"v": 0.5, "title": t("Coil half (about 45 degC)", "Register halb (rund 45 degC)")},
        {"v": 1, "title": t("Coil rated (70 degC)", "Register Nennleistung (70 degC)")},
    ]},
]

regions = {r["id"]: r for r in th["regions"]}
regions["disc"]["areaM2"] = 0.19
regions["disc"]["operating"] = {"key": "regeneration", "heatW": 18}
regions["regen"]["areaM2"] = 0.064
regions["regen"]["operating"] = {"key": "regeneration", "heatW": 28}
regions["hub"]["operating"] = {"key": "rotation", "heatW": 4}
ref = "ashrae-2020-equipment"
th["emitters"] = [
    {"id": "process-air", "kind": "vent", "title": t("Dry process air leaving the disc", "Trockene Prozessluft am Rad"), "anchor": "process-collar", "directionDeg": -10, "diameterM": 0.4, "operatingKey": "regeneration", "volumeFlowM3S": 0.4, "temperatureRiseK": 11, "vapourKgPerKg": 0.0044, "source": "authored-estimate", "reference": ref},
    {"id": "regen-air", "kind": "vent", "title": t("Moist regeneration air leaving the sector", "Feuchte Regenerationsluft am Sektor"), "anchor": "regen-collar", "directionDeg": 15, "diameterM": 0.25, "operatingKey": "regeneration", "volumeFlowM3S": 0.14, "temperatureRiseK": 33, "vapourKgPerKg": 0.0178, "source": "authored-estimate", "reference": ref},
]
th["assumptions"] = [
    t(
        "Both air streams leave as vent emitters scaled linearly by the heating-coil level: the process air dry and 11 K warmer through the adsorption heat, the regeneration air 33 K above the room and moist after the coil heated it to 70 degC; the coil itself is another asset, and a half-loaded coil is a linear approximation.",
        "Beide Luftströme verlassen das Rad als Auslass-Emitter, linear mit der Stufe des Heizregisters skaliert: die Prozessluft trocken und durch die Adsorptionswärme 11 K wärmer, die Regenerationsluft nach dem Heizregister auf 70 degC 33 K über dem Raum und feucht; das Heizregister ist ein eigenes Asset, und ein halb belastetes Register ist eine lineare Näherung.",
    ),
    t(
        "The region heats are the surface losses of the matrix faces at the temperatures the air sets (about 32 degC process face, about 60 degC regeneration face), not the heat of the air streams.",
        "Die Regionswärmen sind die Oberflächenverluste der Matrixflächen bei den von der Luft vorgegebenen Temperaturen (rund 32 degC Prozessseite, rund 60 degC Regenerationsseite), nicht die Wärme der Luftströme.",
    ),
]

poses = {p["id"]: p for p in a["poses"]}
poses["off"]["thermal"]["operating"] = {"rotation": 0, "regeneration": 0}
poses["off"]["thermal"]["emitters"] = {"process-air": 1, "regen-air": 1}
poses["slow"]["thermal"]["operating"] = {"rotation": 0.5, "regeneration": 0.5}
poses["slow"]["thermal"]["emitters"] = {"process-air": 1, "regen-air": 1}
poses["slow"]["description"] = t(
    "The wheel turns at 6 revolutions per hour and the heating coil warms the regeneration air to about 45 degC.",
    "Das Rad dreht mit 6 Umdrehungen je Stunde, und das Heizregister erwärmt die Regenerationsluft auf rund 45 degC.",
)
poses["rated"]["thermal"]["operating"] = {"rotation": 1, "regeneration": 1}
poses["rated"]["thermal"]["emitters"] = {"process-air": 1, "regen-air": 1}
poses["rated"]["description"] = t(
    "The wheel turns at 12 revolutions per hour: the coil heats the regeneration air to 70 degC, the sector runs hot and the process air leaves dry and warmed by the adsorption heat.",
    "Das Rad dreht mit 12 Umdrehungen je Stunde: das Register erwärmt die Regenerationsluft auf 70 degC, der Sektor läuft heiß, und die Prozessluft verlässt das Rad trocken und durch die Adsorptionswärme erwärmt.",
)

clips = {c["id"]: c for c in a["clips"]}
spin = clips["spin"]
spin["description"] = t(
    "The matrix turns once in 5 minutes (12 revolutions per hour, real time) through the process air and the hot regeneration sector; play it faster to see the motion.",
    "Die Matrix dreht sich einmal in 5 Minuten (12 Umdrehungen je Stunde, Echtzeit) durch die Prozessluft und den heißen Regenerationssektor; zum Sichtbarmachen schneller abspielen.",
)
spin["durationS"] = 300
spin["fps"] = 1
spin["keys"][1]["t"] = 300
spin["contacts"][0]["toS"] = 300
spin["phases"][0]["toS"] = 300

run = clips["start-and-run"]
run["description"] = t(
    "The drive starts, the heating coil brings the regeneration air to 70 degC within a minute and the matrix faces warm over five minutes.",
    "Der Antrieb startet, das Heizregister bringt die Regenerationsluft innerhalb einer Minute auf 70 degC, und die Matrixflächen erwärmen sich in fünf Minuten.",
)
run["events"] = [
    {"t": 5, "kind": "operating", "title": t("Drive started", "Antrieb gestartet"), "target": "rotation"},
    {"t": 5, "kind": "operating", "title": t("Heating coil switched on", "Heizregister eingeschaltet"), "target": "regeneration"},
]
run["tracks"] = [
    {"target": "operating:rotation", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}, {"t": 10, "v": 1}]},
    {"target": "operating:regeneration", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}, {"t": 65, "v": 1}]},
]
run["phases"][1]["title"] = t("Starting", "Anlaufen")

out = json.dumps(doc, indent=2, ensure_ascii=False).replace("3e-05", "0.00003").replace("8e-05", "0.00008")
path.write_text(out, encoding="utf8", newline="")
