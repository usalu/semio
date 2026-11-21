"""🧰 Round 3 propane patio heater: root fixes outside `volumes` (rig, anchors, poses, loads, materials, thermal regions, structure assumptions, provenance).

`apply(asset, areas)` changes the asset in place; `areas` maps region id to the measured `areaM2` (or None to keep the values).

@see r5-patio-heater-kit.py — the geometry these numbers come from
"""
import copy
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r5-patio-heater-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def rig(asset):
    """🦴 Upright bones at their real joints: pan, bottle on the pan, post in the lid socket, head on the post top, dome on the emitter cap, knob on the valve stem."""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    spec = {
        "base": (0.0, 0.0, 90, K.POLE_Y, {"kg": 8.7, "comX": 0.22, "comY": 0.0}),
        "bottle": (K.BOTTLE_Y, 0.0, 0, 0.58, {"kg": 24.5, "comX": 0.27, "comY": 0.0}),
        "pole": (K.POLE_Y, 0.0, 0, K.r6(K.HEAD_Y - K.POLE_Y), {"kg": 4.5, "comX": 0.465, "comY": 0.0}),
        "burner-head": (K.r6(K.HEAD_Y - K.POLE_Y), 0.0, 0, K.CAP_TOP_U, {"kg": 3.5, "comX": 0.2, "comY": 0.0}),
        "reflector": (K.CAP_TOP_U, 0.0, 0, K.r6(K.TOP_Y - K.REFLECTOR_Y), {"kg": 3.5, "comX": 0.1, "comY": 0.0}),
        "knob": (K.r6(K.KNOB_Y - K.HEAD_Y), -K.KNOB_X, -90, 0.027, {"kg": 0.3, "comX": 0.012, "comY": 0.0}),
    }
    for bid, (x, y, angle, length, mass) in spec.items():
        b = bones[bid]
        b["x"], b["y"], b["angleDeg"], b["lengthM"] = x, y, angle, length
        b["mass"].update(mass)
    bones["knob"]["parent"] = "burner-head"
    bones["knob"]["title"] = text("Burner control knob on the valve under the emitter", "Brennerregelknopf am Ventil unter dem Strahler")
    bones["knob"]["joint"] = {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 120}
    bones["burner-head"]["title"] = text("Head assembly: valve housing, burner, emitter screen and cap", "Kopfbaugruppe: Ventilgehäuse, Brenner, Strahlergitter und Deckel")
    bones["reflector"]["title"] = text("Dome reflector on three spacer studs", "Reflektorhaube auf drei Abstandsbolzen")
    bones["base"]["title"] = text("Base pan and bottle housing", "Bodenwanne und Flaschengehäuse")
    order = ["base", "bottle", "pole", "burner-head", "knob", "reflector"]
    asset["rig"]["bones"] = [bones[i] for i in order]


def anchors(asset):
    """⚓ Ground pad of the pan, flue outlet on the emitter cap, crown of the dome."""
    a = {x["id"]: x for x in asset["anchors"]}
    a["base-contact"].update({"x": 0.0, "y": 0.0, "footprintLengthM": 0.47, "footprintWidthM": 0.47})
    a["base-contact"]["title"] = text("Base pan on the ground", "Bodenwanne auf dem Boden")
    a["flue-vent"].update({"x": K.CAP_TOP_U, "y": 0.0})
    a["flue-vent"]["title"] = text("Flue gas outlet on the emitter cap, under the dome", "Abgasaustritt auf dem Strahlerdeckel, unter der Haube")
    a["reflector-top"].update({"x": K.r6(K.RIM_U + K.DOME_H), "y": 0.0})


def poses(asset):
    """🎚️ Knob turns about its own axis (twist), the bottle region joins every pose, wind loads follow the real areas."""
    for p in asset["poses"]:
        knob = p["bones"]["knob"]
        p["bones"]["knob"] = {"twistDeg": knob.get("angleDeg", knob.get("twistDeg", 0))}
        regions = p["thermal"]["regions"]
        new = {}
        for k, v in regions.items():
            new[k] = v
            if k == "base-housing":
                new["gas-bottle"] = {"exposedFraction": 0.25, "sunlitFraction": 0.0}
        p["thermal"]["regions"] = new
        if p["id"] == "off-afternoon":
            p["description"] = text("Cold heater, knob at off; the sun shines on the dome and the post.", "Kalter Heizpilz, Knopf auf Aus; die Sonne scheint auf Haube und Mast.")
        if p["id"] == "high-in-breeze":
            p["description"] = text("Full output with an 8 m/s breeze (about 40 Pa) pushing on dome, head, post and housing.", "Volle Leistung bei 8 m/s Wind (etwa 40 Pa) auf Haube, Kopf, Mast und Gehäuse.")
            loads = {x["id"]: x for x in p["loads"]}
            loads["wind-dome"].update({"fxN": 4, "x": 0.1, "y": 0})
            loads["wind-pole"].update({"fxN": 3, "x": 0.465, "y": 0})
            loads["wind-base"].update({"fxN": 10, "x": 0.36, "y": 0})
            head = copy.deepcopy(loads["wind-dome"])
            head.update({"id": "wind-head", "title": text("Breeze on the head", "Wind auf den Kopf"), "fxN": 3, "bone": "burner-head", "x": 0.25, "y": 0})
            p["loads"] = [loads["wind-dome"], head, loads["wind-pole"], loads["wind-base"]]


def materials(asset):
    """🎨 Adds the painted steel of the gas cylinder."""
    if not any(m["id"] == "bottle-steel" for m in asset["materials"]):
        asset["materials"].append({
            "id": "bottle-steel",
            "title": text("Painted steel gas cylinder", "Lackierte Stahl-Gasflasche"),
            "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000.0,
            "source": "literature", "reference": "iso-10456-2007",
        })


def thermal(asset, areas):
    """🌡️ The hidden bottle region, the pole region with its valve housing and the areas of the honest model."""
    regions = asset["thermal"]["regions"]
    if not any(r["id"] == "gas-bottle" for r in regions):
        bottle = {
            "id": "gas-bottle",
            "title": text("Propane bottle (11 kg) in the housing", "Propanflasche (11 kg) im Gehäuse"),
            "kind": "surface", "model": "passive", "material": "bottle-steel", "areaM2": 0.7, "characteristicLengthM": 0.3, "geometry": "cylinder", "skyViewFactor": 0.0,
            "layers": [{"id": "shell", "title": text("Welded steel shell 2.5 mm", "Geschweißter Stahlmantel 2,5 mm"), "kind": "metal", "material": "bottle-steel", "thicknessM": 0.0025,
                        "resistanceM2KW": 5e-05, "heatCapacityJM2K": 8831, "source": "authored-estimate", "reference": "en-1442"}],
            "heatCapacityJM2K": 49900,
            "hidden": text("The bottle stands on the base pan inside the closed housing behind the access panel; no view shows it.",
                           "Die Flasche steht auf der Bodenwanne im geschlossenen Gehäuse hinter der Zugangsklappe; keine Ansicht zeigt sie."),
            "source": "authored-estimate", "reference": "en-1442",
        }
        i = next(i for i, r in enumerate(regions) if r["id"] == "base-housing")
        regions.insert(i + 1, bottle)
    by = {r["id"]: r for r in regions}
    by["pole-stainless"]["title"] = text("Stainless post and valve housing", "Edelstahlmast und Ventilgehäuse")
    by["base-housing"]["title"] = text("Bottle housing, base pan and wheels", "Flaschengehäuse, Bodenwanne und Räder")
    by["emitter-head"]["title"] = text("Emitter screen, guard rods and cap (hot head)", "Strahlergitter, Schutzstäbe und Deckel (heißer Kopf)")
    by["reflector"]["title"] = text("Dome reflector with spacers", "Reflektorhaube mit Abstandshaltern")
    if areas:
        for rid, value in areas.items():
            by[rid]["areaM2"] = value
    asset["thermal"]["assumptions"][1] = text(
        "The propane bottle is a hidden passive region inside the closed housing: its 2.5 mm steel shell carries the heat capacity of the shell and of the 11 kg liquid propane (2.5 kJ/kgK). The cooling of the bottle by evaporation (0.28 g/s at high, about 0.12 kW of latent heat) is neglected.",
        "Die Propanflasche ist eine verborgene passive Region im geschlossenen Gehäuse: ihr 2,5-mm-Stahlmantel trägt die Wärmekapazität des Mantels und der 11 kg flüssigen Propans (2,5 kJ/kgK). Die Abkühlung der Flasche durch Verdampfung (0,28 g/s auf Groß, etwa 0,12 kW latente Wärme) wird vernachlässigt.")


def structure(asset):
    """🏗️ The load path in words."""
    asset["structure"]["assumptions"] = [
        text("A free-standing rigid body on its base pan: the post stands in the socket of the pressed housing lid, the 1.2 mm cylinder wall of the housing carries post, head and dome down to the pan, and the bottle stands on the pan; the pan transmits a centre-of-pressure moment up to N times its radius, which carries the breeze.",
             "Ein freistehender starrer Körper auf seiner Bodenwanne: der Mast steht in der Muffe des gepressten Gehäusedeckels, die 1,2-mm-Zylinderwand des Gehäuses trägt Mast, Kopf und Haube zur Wanne, und die Flasche steht auf der Wanne; die Wanne überträgt ein Druckmittelpunktmoment bis N mal ihren Radius, das den Wind aufnimmt."),
        text("Wind pressure 0.5 rho v^2 = 40 Pa at 8 m/s on the projected areas (dome 0.08 m2, head 0.07 m2, post 0.056 m2, housing 0.30 m2) with drag coefficients 1.2, 1.0, 1.2 and 0.8 (EN 1991-1-4 basis).",
             "Winddruck 0,5 rho v^2 = 40 Pa bei 8 m/s auf die projizierten Flächen (Haube 0,08 m2, Kopf 0,07 m2, Mast 0,056 m2, Gehäuse 0,30 m2) mit Widerstandsbeiwerten 1,2, 1,0, 1,2 und 0,8 (Grundlage EN 1991-1-4)."),
    ]


def provenance(asset):
    """📚 Product references and rewritten notes."""
    refs = asset["provenance"]["references"]
    ids = {r["id"] for r in refs}
    if "amazonbasics-46k" not in ids:
        refs.append({"id": "amazonbasics-46k", "citation": "Amazon Basics 46,000 BTU commercial outdoor propane patio heater with wheels, retailer product data (32.1 x 32.1 x 91.3 in, base 18.4 in wide and 28.76 in high, 37 lb), retrieved 2026-11-20."})
    if "crown-verity-cv3050" not in ids:
        refs.append({"id": "crown-verity-cv3050", "citation": "Crown Verity CV3050 patio heater owner's manual (parts: reflector, head assembly with burner screen and control knob, gas hose inside the three-part post, base with weight box, three reflector spacers on studs with wing nuts), webstaurantstore.com, retrieved 2026-11-20."})
    asset["provenance"]["notes"] = text(
        "Commercial mushroom heater of the 13 kW class (46,000 BTU/h = 13.5 kW): a 0.47 m base pan with two rear wheels, a 0.42 m cylindrical bottle housing 0.73 m high with an access panel for the 11 kg propane bottle (EN 1442, 0.30 m x 0.58 m), a 60 mm stainless post in three sections with the gas hose inside, the head assembly with the valve housing, control knob and igniter, the burner deck, a 0.24 m emitter screen 0.23 m high with eight guard rods and a cap, and a one-piece 0.80 m aluminised dome on three spacer studs (Amazon Basics and Crown Verity product data). Mass 45 kg with a full bottle. Heat input 13 kW at high and 6 kW at low are authored estimates for the class (EN 14543 product family). The release is split into 8.2 kW at the emitter screen and 2.2 kW at the dome, the flue gas sensible heat (0.0098 m3/s at 280 K rise = 1.9 kW) makes up the rest; the flue vapour share 0.078 kg/kg follows from C3H8 + 5 O2 -> 3 CO2 + 4 H2O (1.64 kg water per kg propane) with 21 kg flue gas per kg propane at excess air 1.3.",
        "Gewerblicher Heizpilz der 13-kW-Klasse (46.000 BTU/h = 13,5 kW): eine Bodenwanne von 0,47 m mit zwei hinteren Rädern, ein zylindrisches Flaschengehäuse von 0,42 m und 0,73 m Höhe mit Zugangsklappe für die 11-kg-Propanflasche (EN 1442, 0,30 m x 0,58 m), ein 60-mm-Edelstahlmast aus drei Teilen mit dem Gasschlauch innen, die Kopfbaugruppe mit Ventilgehäuse, Regelknopf und Zünder, das Brennerdeck, ein Strahlergitter von 0,24 m und 0,23 m Höhe mit acht Schutzstäben und Deckel sowie eine einteilige aluminierte Haube von 0,80 m auf drei Abstandsbolzen (Produktdaten Amazon Basics und Crown Verity). Masse 45 kg mit voller Flasche. Wärmebelastung 13 kW auf Groß und 6 kW auf Klein sind Schätzwerte für die Klasse (Produktfamilie EN 14543). Die Freisetzung teilt sich in 8,2 kW am Strahlergitter und 2,2 kW an der Haube, die fühlbare Abgaswärme (0,0098 m3/s bei 280 K Anstieg = 1,9 kW) bildet den Rest; der Dampfanteil 0,078 kg/kg folgt aus C3H8 + 5 O2 -> 3 CO2 + 4 H2O (1,64 kg Wasser je kg Propan) mit 21 kg Abgas je kg Propan bei Luftüberschuss 1,3.")
    asset["summary"] = text(
        "A free-standing 13 kW propane mushroom heater, 2.25 m high with a 0.80 m dome: the bottle stands in the base housing, the control knob sits on the head under the emitter screen; operating levels off, low (6 kW) and high (13 kW) heat the emitter screen and the dome and send a hot flue plume up under the dome.",
        "Ein freistehender Propan-Heizpilz von 13 kW, 2,25 m hoch mit 0,80 m Haube: die Flasche steht im Sockelgehäuse, der Regelknopf sitzt am Kopf unter dem Strahlergitter; die Betriebsstufen Aus, Klein (6 kW) und Groß (13 kW) heizen Strahlergitter und Haube und schicken eine heiße Abgasfahne unter der Haube nach oben.")


def apply(asset, areas=None):
    """🛠️ Every root fix in order."""
    rig(asset)
    anchors(asset)
    poses(asset)
    materials(asset)
    thermal(asset, areas)
    structure(asset)
    provenance(asset)
