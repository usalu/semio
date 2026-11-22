"""📜 Texts, materials, rig, anchors, poses, clips, thermal and structural model of the used monobloc heat pump; loaded by `author-asset.py` into its namespace (needs `T`).

@see ./author-asset.py
"""

SUMMARY = T(
    "A used 8 kW air-to-water monobloc heat pump outdoor unit as a component exchange offers it: casing 1.12 m wide, 0.56 m deep and 1.30 m high with one 0.60 m axial fan behind a black wire grille on the front, the L-shaped evaporator coil on the back and the right side with a patch of crushed fins, the compressor and hydraulics compartment on the left with the faded type plate on its front, and the flow and return stubs cut at the back and closed with red caps. The refrigerant (about 1 kg of propane) was recovered and the water drained; the unit stays bolted to its galvanised ground console, whose anchor bolts were cut at the foundation, and stands strapped on a Euro pallet. Lifted from the pallet by two round slings under the console, and reinstalled on the ground it heats at the A7/W35 point: the fan blows air 5 K colder out of the front, and every 20 minutes or so a defrost reverses the cycle, the fan stops and vapour and melt water leave the coil and the condensate spigot.",
    "Eine gebrauchte Luft-Wasser-Monoblock-Wärmepumpe von 8 kW als Außengerät, wie eine Bauteilbörse sie anbietet: Gehäuse 1,12 m breit, 0,56 m tief und 1,30 m hoch mit einem Axiallüfter von 0,60 m hinter einem schwarzen Drahtgitter auf der Front, dem L-förmigen Verdampfer an Rückseite und rechter Seite mit einer Stelle zerdrückter Lamellen, dem Verdichter- und Hydraulikraum links mit dem verblassten Typenschild auf seiner Front und den hinten abgeschnittenen, mit roten Kappen verschlossenen Vor- und Rücklaufstutzen. Das Kältemittel (etwa 1 kg Propan) wurde abgesaugt und das Wasser abgelassen; das Gerät bleibt auf seiner verzinkten Bodenkonsole verschraubt, deren Ankerbolzen am Fundament abgetrennt wurden, und steht verzurrt auf einer Europalette. Mit zwei Rundschlingen unter der Konsole von der Palette gehoben und am Boden wieder aufgestellt, heizt es im Prüfpunkt A7/W35: Der Lüfter bläst um 5 K kältere Luft aus der Front, und etwa alle 20 Minuten kehrt ein Abtauvorgang den Kreislauf um, der Lüfter steht, und Dampf und Schmelzwasser verlassen den Verdampfer und den Kondensatstutzen.",
)

PROVENANCE = {
    "author": "Semio staffage component author 05 of the used HVAC components wave",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": [
        {"id": "class-data-note", "citation": "Manufacturer class data of single-fan 7 to 9 kW air-to-water monobloc outdoor units with R290 (public data sheet families such as Viessmann Vitocal 250-A, Vaillant aroTHERM plus, Stiebel Eltron WPL-A, quoted as orders of magnitude from memory of the data sheets, not retrieved for this asset): casing 1.10 to 1.15 m wide, 1.30 to 1.40 m high, 0.55 to 0.60 m deep, 130 to 170 kg, one axial fan of 0.55 to 0.65 m, 2,500 to 3,500 m3/h, about 1 kg of R290, maximum electrical input about 3 kW. Every number derived from it is tagged authored-estimate."},
        {"id": "en-14511", "citation": "EN 14511-2:2022 Air conditioners, liquid chilling packages and heat pumps for space heating and cooling, Part 2: Test conditions (standard rating point A7/W35 heating, outdoor air 7 C dry bulb, 6 C wet bulb, water 30/35 C)."},
        {"id": "en-14825", "citation": "EN 14825:2022 Air conditioners, liquid chilling packages and heat pumps, testing and rating at part load conditions and calculation of seasonal performance (part-load points, defrost penalty below +7 C)."},
        {"id": "en-378-4", "citation": "EN 378-4:2016+A1:2019 Refrigerating systems and heat pumps, safety and environmental requirements, Part 4: Operation, maintenance, repair and recovery (recovery of the refrigerant before a system is opened or dismantled)."},
        {"id": "vdi-4645", "citation": "VDI 4645:2018 Heating plants with heat pumps in single-family and multi-family houses, planning, construction, operation (outdoor installation on a ground console, condensate drainage, clearances)."},
        {"id": "en-13698-1", "citation": "EN 13698-1:2003 Pallet production specification, Part 1: Construction specification for 800 mm x 1200 mm flat wooden pallets (EUR pallet: 1200 x 800 x 144 mm, top deck 3 x 145 + 2 x 100 mm boards of 22 mm, three 145 mm stringer boards, nine blocks 78 mm high, bottom 2 x 100 + 1 x 145 mm boards)."},
        {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions (peak velocity pressure and force coefficients of blocks on the ground)."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (softwood 500 kg/m3, 0.13 W/mK, 1600 J/kgK)."},
        {"id": "incropera-2011", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley. Appendix A (properties of steel, aluminium, copper, polymers, rubber) and Table A.11 (emissivities)."},
        {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), Chapter 4 Heat Transfer and Chapter 25 (emissivity and solar absorptance of new and weathered galvanised steel, painted and powder-coated sheet)."},
    ],
    "notes": T(
        "Class data of a single-fan 8 kW monobloc as it comes back from a dismantled installation: drained, refrigerant recovered, the hydraulic connections cut 60 mm behind the casing and capped, the console unbolted from the foundation with the anchor bolts cut 25 mm above the foot plates, the casing faded and the side coil crushed over a hand-sized patch. Nothing is measured; dimensions follow the class, the pallet follows EN 13698-1, the operating point EN 14511 A7/W35 (8 kW heating, COP about 4.7, electrical input about 1.7 kW) and the nameplate the maximum input of the class (3.0 kW compressor and controls, 0.12 kW fan, 0.1 kW base pan heater).",
        "Klassendaten eines 8-kW-Monoblocks mit einem Lüfter, wie er aus einer rückgebauten Anlage zurückkommt: entleert, Kältemittel abgesaugt, die Hydraulikanschlüsse 60 mm hinter dem Gehäuse abgetrennt und verschlossen, die Konsole vom Fundament gelöst und die Ankerbolzen 25 mm über den Fußplatten abgetrennt, das Gehäuse verblasst und der seitliche Verdampfer auf einer handgroßen Stelle eingedrückt. Nichts ist gemessen; die Maße folgen der Klasse, die Palette EN 13698-1, der Betriebspunkt EN 14511 A7/W35 (8 kW Heizleistung, COP etwa 4,7, elektrische Aufnahme etwa 1,7 kW) und das Typenschild der Höchstaufnahme der Klasse (3,0 kW Verdichter und Regelung, 0,12 kW Lüfter, 0,1 kW Bodenwannenheizung).",
    ),
}

MATERIALS = [
    {"id": "pallet-softwood", "title": T("Weathered pallet softwood (spruce, pine)", "Verwittertes Palettenholz (Fichte, Kiefer)"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 520, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 10000000000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "galvanised-steel", "title": T("Weathered galvanised steel", "Verwitterter verzinkter Stahl"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 280000000, "source": "literature", "reference": "ashrae-2021"},
    {"id": "powder-coat-faded", "title": T("Faded light-grey powder-coated sheet steel", "Verblasstes hellgrau pulverbeschichtetes Stahlblech"), "emissivity": 0.9, "solarAbsorptivity": 0.48, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "aluminium-fins", "title": T("Weathered aluminium fins on copper tubes", "Verwitterte Aluminiumlamellen auf Kupferrohren"), "emissivity": 0.2, "solarAbsorptivity": 0.5, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2011"},
    {"id": "coated-wire", "title": T("Black plastic-coated steel wire", "Schwarz kunststoffbeschichteter Stahldraht"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "incropera-2011"},
    {"id": "painted-steel", "title": T("Black painted motor steel", "Schwarz lackierter Motorstahl"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "incropera-2011"},
    {"id": "copper-oxidised", "title": T("Oxidised copper tube", "Oxidiertes Kupferrohr"), "emissivity": 0.6, "solarAbsorptivity": 0.65, "densityKgM3": 8930, "conductivityWmK": 390, "specificHeatJKgK": 385, "source": "literature", "reference": "incropera-2011"},
    {"id": "pe-cap", "title": T("Red polyethylene protective cap", "Rote Polyethylen-Schutzkappe"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 950, "conductivityWmK": 0.4, "specificHeatJKgK": 1900, "source": "literature", "reference": "incropera-2011"},
    {"id": "epdm-rubber", "title": T("EPDM rubber of the anti-vibration mounts", "EPDM-Gummi der Schwingungsdämpfer"), "emissivity": 0.93, "solarAbsorptivity": 0.9, "densityKgM3": 1100, "conductivityWmK": 0.15, "specificHeatJKgK": 1700, "source": "literature", "reference": "incropera-2011"},
    {"id": "pp-gf", "title": T("Glass-fibre reinforced polypropylene (fan)", "Glasfaserverstärktes Polypropylen (Lüfter)"), "emissivity": 0.92, "solarAbsorptivity": 0.85, "densityKgM3": 1120, "conductivityWmK": 0.3, "specificHeatJKgK": 1500, "source": "literature", "reference": "incropera-2011"},
]

RIG = {
    "kind": "mechanical",
    "root": "yard",
    "bones": [
        {"id": "yard", "parent": None, "title": T("Yard floor at the pallet (massless reference)", "Lagerboden an der Palette (masseloser Bezug)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "pallet", "parent": "yard", "title": T("Euro pallet 1200 x 800 x 144 mm, stays behind when the unit is lifted and is pushed aside for the reinstallation", "Europalette 1200 x 800 x 144 mm, bleibt beim Anheben liegen und wird für den Wiedereinbau beiseitegeschoben"), "x": -0.40, "y": 0, "angleDeg": 0, "lengthM": 0.80, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.4, "maxM": 0}, "mass": {"kg": 23.6, "comX": 0.40, "comY": 0.075, "distribution": "point", "source": "literature", "reference": "en-13698-1"}},
        {"id": "unit", "parent": "yard", "title": T("Heat pump unit: casing, coil, compressor compartment, fan motor and cover (lifted and set down vertically)", "Wärmepumpengerät: Gehäuse, Verdampfer, Verdichterraum, Lüftermotor und Haube (senkrecht gehoben und abgesetzt)"), "x": -0.30, "y": 0.444, "angleDeg": 0, "lengthM": 0.56, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.144, "maxM": 0.6}, "mass": {"kg": 145, "comX": 0.26, "comY": 0.52, "distribution": "point", "source": "authored-estimate", "reference": "class-data-note"}},
        {"id": "console", "parent": "unit", "title": T("Galvanised ground console, two stands bolted under the base pan", "Verzinkte Bodenkonsole, zwei Böcke unter der Bodenwanne verschraubt"), "x": 0, "y": -0.30, "angleDeg": 0, "lengthM": 0.60, "joint": {"type": "fixed"}, "mass": {"kg": 16, "comX": 0.30, "comY": 0.05, "distribution": "point", "source": "authored-estimate", "reference": "vdi-4645"}},
        {"id": "fan", "parent": "unit", "title": T("Axial fan impeller on the motor shaft (its 2.5 kg counted in the unit)", "Axiallüfterrad auf der Motorwelle (seine 2,5 kg im Gerät enthalten)"), "x": 0.45, "y": 0.70, "z": -0.17, "angleDeg": 0, "lengthM": 0.07, "joint": {"type": "revolute", "continuousTwist": True}},
    ],
}

ANCHORS = [
    {"id": "pallet-contact-rear", "bone": "pallet", "x": 0.05, "y": 0, "kind": "contact", "title": T("Rear bottom board of the pallet on the yard floor", "Hinteres Bodenbrett der Palette auf dem Lagerboden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 1.2, "frictionCoefficient": 0.5},
    {"id": "pallet-contact-mid", "bone": "pallet", "x": 0.40, "y": 0, "kind": "contact", "title": T("Middle bottom board of the pallet on the yard floor", "Mittleres Bodenbrett der Palette auf dem Lagerboden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.145, "footprintWidthM": 1.2, "frictionCoefficient": 0.5},
    {"id": "pallet-contact-front", "bone": "pallet", "x": 0.75, "y": 0, "kind": "contact", "title": T("Front bottom board of the pallet on the yard floor", "Vorderes Bodenbrett der Palette auf dem Lagerboden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 1.2, "frictionCoefficient": 0.5},
]
for nm, x in (("rear", 0.10), ("front", 0.50)):
    for side, z in (("r", -0.40), ("l", 0.40)):
        ANCHORS.append({"id": f"sling-{nm}-{side}", "bone": "console", "x": x, "y": 0.27, "z": z, "kind": "support", "title": T(f"Round sling under the {nm} end of the {'right' if side == 'r' else 'left'} console stand", f"Rundschlinge unter dem {'hinteren' if nm == 'rear' else 'vorderen'} Ende des {'rechten' if side == 'r' else 'linken'} Konsolbocks"), "support": "pin"})
for nm, x in (("rear", 0.03), ("front", 0.57)):
    for side, z in (("r", -0.40), ("l", 0.40)):
        ANCHORS.append({"id": f"bolt-{nm}-{side}", "bone": "console", "x": x, "y": 0, "z": z, "kind": "support", "title": T(f"New anchor bolt at the {nm} foot of the {'right' if side == 'r' else 'left'} stand", f"Neuer Ankerbolzen am {'hinteren' if nm == 'rear' else 'vorderen'} Fuß des {'rechten' if side == 'r' else 'linken'} Bocks"), "support": "pin"})
ANCHORS += [
    {"id": "fan-outlet", "bone": "unit", "x": 0.60, "y": 0.70, "z": -0.17, "kind": "emitter", "title": T("Centre of the fan grille", "Mitte des Lüftergitters")},
    {"id": "coil-rear", "bone": "unit", "x": 0.0, "y": 1.0, "z": -0.15, "kind": "emitter", "title": T("Upper middle of the rear coil face", "Obere Mitte der hinteren Verdampferfläche")},
    {"id": "condensate-outlet", "bone": "unit", "x": 0.30, "y": -0.03, "z": -0.20, "kind": "emitter", "title": T("Mouth of the condensate spigot under the base pan", "Mündung des Kondensatstutzens unter der Bodenwanne")},
    {"id": "wind-face", "bone": "unit", "x": 0.56, "y": 0.65, "kind": "load", "title": T("Centre of the front face (wind resultant)", "Mitte der Frontfläche (Windresultierende)")},
]

PALLET_CONTACTS = ["pallet-contact-rear", "pallet-contact-mid", "pallet-contact-front"]
SLINGS = ["sling-rear-r", "sling-rear-l", "sling-front-r", "sling-front-l"]
BOLTS = ["bolt-rear-r", "bolt-rear-l", "bolt-front-r", "bolt-front-l"]
REGION_IDS = ["pallet", "console", "base-pan", "compressor-compartment", "coil", "casing", "top-cover", "fan", "water-connections"]
SUN = {"pallet": 0.3, "console": 0.2, "base-pan": 0.0, "compressor-compartment": 0.5, "coil": 0.3, "casing": 0.5, "top-cover": 1.0, "fan": 0.0, "water-connections": 0.0}


def bind(contact_pallet, contact_console):
    """🌬️ The region bindings of a pose: everything in free air, the pallet and the console feet on the ground by the given shares."""
    out = {}
    for r in REGION_IDS:
        b = {"exposedFraction": 1, "sunlitFraction": SUN[r]}
        if r == "pallet" and contact_pallet:
            b["contact"] = {"fraction": contact_pallet, "tempC": 8}
        if r == "console" and contact_console:
            b["contact"] = {"fraction": contact_console, "tempC": 8}
        out[r] = b
    return out


def ops(c, f, d, df, h):
    """⚙️ The operating levels compressor, fan, coil duty, defrost and heating circuit."""
    return {"compressor": c, "fan-speed": f, "refrigerant-duty": d, "defrost": df, "heating-circuit": h}


def ems(j, p, c):
    """💨️ The emitter strengths cold air jet, defrost plume and condensate drip."""
    return {"cold-air-jet": j, "defrost-plume": p, "condensate-drip": c}


REINSTALLED_BONES = {"pallet": {"dx": -1.20}, "unit": {"dy": -0.144}}
POSES = [
    {"id": "stored", "title": T("Stored on the pallet", "Auf der Palette gelagert"), "description": T("Drained and with the refrigerant recovered, the unit stands on its console on a Euro pallet in the yard of the component exchange: everything at the outdoor air temperature.", "Entleert und mit abgesaugtem Kältemittel steht das Gerät auf seiner Konsole auf einer Europalette im Hof der Bauteilbörse: alles auf Außentemperatur."), "bones": {}, "ground": "authored", "contacts": PALLET_CONTACTS, "thermal": {"operating": ops(0, 0, 0, 0, 0), "regions": bind(0.12, 0), "emitters": ems(0, 0, 0)}, "loads": []},
    {"id": "lifted", "title": T("Lifted on two round slings", "An zwei Rundschlingen gehoben"), "description": T("Two round slings under the console stands carry the 161 kg of unit and console 0.40 m above the pallet, which stays on the floor.", "Zwei Rundschlingen unter den Konsolböcken tragen die 161 kg von Gerät und Konsole 0,40 m über der Palette, die am Boden liegen bleibt."), "bones": {"unit": {"dy": 0.40}}, "ground": "authored", "contacts": PALLET_CONTACTS + SLINGS, "thermal": {"operating": ops(0, 0, 0, 0, 0), "regions": bind(0.12, 0), "emitters": ems(0, 0, 0)}, "loads": []},
    {"id": "reinstalled", "title": T("Reinstalled, heating at A7/W35", "Wieder eingebaut, Heizbetrieb A7/W35"), "description": T("The console stands on the ground on four new anchor bolts, flow and return are connected at 35/30 C, the compressor runs at 70 % and the fan at 80 %: the air leaves the grille about 5 K colder; the empty pallet lies 1.2 m behind.", "Die Konsole steht auf vier neuen Ankerbolzen am Boden, Vor- und Rücklauf sind mit 35/30 C angeschlossen, der Verdichter läuft mit 70 % und der Lüfter mit 80 %: Die Luft verlässt das Gitter etwa 5 K kälter; die leere Palette liegt 1,2 m dahinter."), "bones": REINSTALLED_BONES, "ground": "authored", "contacts": BOLTS + PALLET_CONTACTS, "thermal": {"operating": ops(0.7, 0.8, 1, 0, 1), "regions": bind(0.12, 0.05), "emitters": ems(1, 0, 0)}, "loads": []},
    {"id": "defrosting", "title": T("Reinstalled, defrosting", "Wieder eingebaut, Abtauen"), "description": T("The four-way valve has reversed the cycle: the fan stands still, hot gas warms the coil, the frost melts, vapour rises from the coil and melt water runs out of the condensate spigot.", "Das Vierwegeventil hat den Kreislauf umgekehrt: Der Lüfter steht, Heißgas erwärmt den Verdampfer, der Reif schmilzt, Dampf steigt vom Verdampfer auf und Schmelzwasser läuft aus dem Kondensatstutzen."), "bones": REINSTALLED_BONES, "ground": "authored", "contacts": BOLTS + PALLET_CONTACTS, "thermal": {"operating": ops(0.6, 0, -1, 1, 0.3), "regions": bind(0.12, 0.05), "emitters": ems(0, 1, 1)}, "loads": []},
    {"id": "storm-gust", "title": T("Reinstalled, switched off in a storm gust", "Wieder eingebaut, abgeschaltet im Sturm"), "description": T("Switched off, the reinstalled unit takes a gust on its front face: 0.39 kN/m2 peak velocity pressure times force coefficient 1.3 on 1.46 m2, about 0.74 kN, held by the four anchor bolts.", "Abgeschaltet nimmt das wieder eingebaute Gerät eine Böe auf seine Front: 0,39 kN/m2 Böengeschwindigkeitsdruck mal Kraftbeiwert 1,3 auf 1,46 m2, etwa 0,74 kN, gehalten von den vier Ankerbolzen."), "bones": REINSTALLED_BONES, "ground": "authored", "contacts": BOLTS + PALLET_CONTACTS, "thermal": {"operating": ops(0, 0, 0, 0, 0), "regions": bind(0.12, 0.05), "emitters": ems(0, 0, 0)}, "loads": [{"id": "gust", "kind": "wind", "title": T("Gust on the front face 0.74 kN", "Böe auf die Front 0,74 kN"), "anchor": "wind-face", "fxN": -740, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-4"}]},
]

CLIPS = [
    {"id": "lift", "title": T("Lift from the pallet", "Anheben von der Palette"), "description": T("The slings are taken up for two seconds, then the crane lifts unit and console 0.40 m in six seconds; the pallet stays on the floor.", "Die Schlingen werden zwei Sekunden lang gestrafft, dann hebt der Kran Gerät und Konsole in sechs Sekunden um 0,40 m; die Palette bleibt am Boden."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "stored", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in PALLET_CONTACTS] + [{"anchor": a, "fromS": 2, "toS": 8} for a in SLINGS],
     "phases": [{"id": "sling-up", "fromS": 0, "toS": 2, "title": T("Slings taken up", "Schlingen gestrafft")}, {"id": "hoist", "fromS": 2, "toS": 8, "title": T("Hoist", "Heben")}],
     "events": [{"t": 2, "kind": "contact-on", "title": T("Slings carry the load", "Schlingen tragen die Last")}, {"t": 2, "kind": "motion", "title": T("Unit leaves the pallet", "Gerät verlässt die Palette")}],
     "tracks": []},
    {"id": "heating-defrost", "title": T("Heating with a defrost cycle", "Heizen mit Abtauzyklus"), "description": T("A 60 s loop of the reinstalled unit in a compressed time scale: 20 s heating with the fan turning, 2 s change-over, 20 s defrost with the fan stopped, vapour at the coil and water at the spigot, 3 s change-over and 15 s heating again.", "Eine Schleife von 60 s des wieder eingebauten Geräts in gerafter Zeit: 20 s Heizen mit drehendem Lüfter, 2 s Umschalten, 20 s Abtauen mit stehendem Lüfter, Dampf am Verdampfer und Wasser am Stutzen, 3 s Umschalten und wieder 15 s Heizen."), "durationS": 60, "fps": 30, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled", "ease": "linear", "bones": {"fan": {"twistDeg": 0}}}, {"t": 20, "pose": "reinstalled", "ease": "out", "bones": {"fan": {"twistDeg": 2880}}}, {"t": 22, "pose": "defrosting", "ease": "step", "bones": {"fan": {"twistDeg": 3240}}}, {"t": 42, "pose": "defrosting", "ease": "in", "bones": {"fan": {"twistDeg": 3240}}}, {"t": 45, "pose": "reinstalled", "ease": "linear", "bones": {"fan": {"twistDeg": 3600}}}, {"t": 60, "pose": "reinstalled", "ease": "linear", "bones": {"fan": {"twistDeg": 5760}}}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 60} for a in BOLTS + PALLET_CONTACTS],
     "phases": [{"id": "heating", "fromS": 0, "toS": 20, "title": T("Heating", "Heizen")}, {"id": "defrost", "fromS": 20, "toS": 45, "title": T("Defrost", "Abtauen")}, {"id": "heating-again", "fromS": 45, "toS": 60, "title": T("Heating again", "Wieder heizen")}],
     "events": [{"t": 20, "kind": "operating", "title": T("Four-way valve reverses, fan stops", "Vierwegeventil schaltet um, Lüfter stoppt"), "target": "defrost"}, {"t": 42, "kind": "operating", "title": T("Coil clear, heating resumes", "Verdampfer frei, Heizen beginnt wieder"), "target": "defrost"}],
     "tracks": [
         {"target": "operating:compressor", "ease": "smooth", "keys": [{"t": 0, "v": 0.7}, {"t": 20, "v": 0.7}, {"t": 22, "v": 0.6}, {"t": 42, "v": 0.6}, {"t": 45, "v": 0.7}, {"t": 60, "v": 0.7}]},
         {"target": "operating:fan-speed", "ease": "smooth", "keys": [{"t": 0, "v": 0.8}, {"t": 20, "v": 0.8}, {"t": 22, "v": 0}, {"t": 42, "v": 0}, {"t": 45, "v": 0.8}, {"t": 60, "v": 0.8}]},
         {"target": "operating:refrigerant-duty", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 20, "v": 1}, {"t": 22, "v": -1}, {"t": 42, "v": -1}, {"t": 45, "v": 1}, {"t": 60, "v": 1}]},
         {"target": "operating:defrost", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 20, "v": 1}, {"t": 42, "v": 0}]},
         {"target": "operating:heating-circuit", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 20, "v": 1}, {"t": 22, "v": 0.3}, {"t": 42, "v": 0.3}, {"t": 45, "v": 1}, {"t": 60, "v": 1}]},
         {"target": "emitter:cold-air-jet", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 20, "v": 1}, {"t": 22, "v": 0}, {"t": 42, "v": 0}, {"t": 45, "v": 1}, {"t": 60, "v": 1}]},
         {"target": "emitter:defrost-plume", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 20, "v": 0}, {"t": 24, "v": 1}, {"t": 40, "v": 1}, {"t": 43, "v": 0}, {"t": 60, "v": 0}]},
         {"target": "emitter:condensate-drip", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 24, "v": 0}, {"t": 28, "v": 1}, {"t": 44, "v": 1}, {"t": 48, "v": 0}, {"t": 60, "v": 0}]},
     ]},
]


def region(rid, en, de, kind, model, mat, area, geom, sky, cap, length, src="authored-estimate", refid="ashrae-2021", **extra):
    """🔥️ A thermal region without layers."""
    d = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": mat, "areaM2": area, "characteristicLengthM": length, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": src, "reference": refid}
    d.update(extra)
    return d


AREAS = {"pallet": 3.14, "console": 0.6, "base-pan": 1.4, "compressor-compartment": 2.6, "coil": 1.49, "casing": 2.41, "top-cover": 1.53, "fan": 0.665, "water-connections": 0.033}
THERMAL = {
    "environment": {"airTempC": 7, "meanRadiantTempC": 7, "skyTempC": -3, "groundTempC": 8, "relativeHumidity": 0.87, "windMS": 1, "windDirectionDeg": 180, "solarWM2": 150, "diffuseFraction": 0.3, "pressurePa": 101325},
    "passengers": [],
    "operating": [
        {"id": "compressor", "title": T("Compressor modulation", "Verdichtermodulation"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.35, "title": T("Minimum modulation", "Minimale Modulation")}, {"v": 0.7, "title": T("Part load", "Teillast")}, {"v": 1, "title": T("Rated (A7/W35 8 kW)", "Nennlast (A7/W35 8 kW)")}]},
        {"id": "fan-speed", "title": T("Fan speed", "Lüfterdrehzahl"), "levels": [{"v": 0, "title": T("Stopped", "Stillstand")}, {"v": 0.4, "title": T("Low", "Niedrig")}, {"v": 0.8, "title": T("Medium", "Mittel")}, {"v": 1, "title": T("High", "Hoch")}]},
        {"id": "refrigerant-duty", "title": T("Coil duty (positive evaporating, negative defrosting)", "Verdampfer-Betrieb (positiv verdampfend, negativ abtauend)"), "levels": [{"v": -1, "title": T("Defrost: hot gas in the coil", "Abtauen: Heißgas im Verdampfer")}, {"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Evaporating, rated", "Verdampfend, Nennlast")}]},
        {"id": "defrost", "title": T("Defrost and base pan heater", "Abtauen und Bodenwannenheizung"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Defrosting", "Abtauend")}]},
        {"id": "heating-circuit", "title": T("Heating water flow", "Heizwasserdurchfluss"), "levels": [{"v": 0, "title": T("No flow", "Kein Durchfluss")}, {"v": 0.3, "title": T("Defrost draw-down", "Absenkung beim Abtauen")}, {"v": 1, "title": T("Flow 35 C, return 30 C", "Vorlauf 35 C, Rücklauf 30 C")}]},
    ],
    "regions": [
        region("pallet", "Euro pallet boards", "Bretter der Europalette", "surface", "passive", "pallet-softwood", AREAS["pallet"], "horizontal-plate-up", 0.5, 18300, 0.8, src="authored-estimate", refid="en-iso-10456"),
        region("console", "Galvanised ground console, mounts and bolt stubs", "Verzinkte Bodenkonsole, Dämpfer und Bolzenstummel", "surface", "passive", "galvanised-steel", AREAS["console"], "vertical-plate", 0.3, 10800, 0.3),
        region("base-pan", "Base pan with defrost heater and condensate spigot", "Bodenwanne mit Abtauheizung und Kondensatstutzen", "heater", "operating", "galvanised-steel", AREAS["base-pan"], "horizontal-plate-down", 0.1, 5400, 0.56, operating={"key": "defrost", "heatW": 100}, ratedPowerW=100, ratedPowerBasis="electrical-input", **{"reference": "class-data-note"}),
        region("compressor-compartment", "Compressor and hydraulics compartment with type plate", "Verdichter- und Hydraulikraum mit Typenschild", "engine", "operating", "powder-coat-faded", AREAS["compressor-compartment"], "vertical-plate", 0.5, 4300, 1.2, operating={"key": "compressor", "heatW": 150}, ratedPowerW=3000, ratedPowerBasis="electrical-input", **{"reference": "class-data-note"}),
        region("coil", "Evaporator coil face with the crushed fin patch", "Verdampferfläche mit der zerdrückten Lamellenstelle", "surface", "operating", "aluminium-fins", AREAS["coil"], "vertical-plate", 0.5, 2000, 1.18, operating={"key": "refrigerant-duty", "heatW": -200}, areaBasis="exchange", **{"reference": "en-14511"}),
        region("casing", "Front panel, front post and motor bracket", "Frontblech, Eckpfosten vorne und Motorhalter", "surface", "passive", "powder-coat-faded", AREAS["casing"], "vertical-plate", 0.5, 4300, 1.18),
        region("top-cover", "Top cover", "Abdeckhaube", "surface", "passive", "powder-coat-faded", AREAS["top-cover"], "horizontal-plate-up", 1.0, 4300, 0.58),
        region("fan", "Fan motor, impeller and black wire grille", "Lüftermotor, Lüfterrad und schwarzes Drahtgitter", "engine", "operating", "painted-steel", AREAS["fan"], "vertical-plate", 0.1, 5000, 0.5, operating={"key": "fan-speed", "heatW": 40}, ratedPowerW=120, ratedPowerBasis="electrical-input", **{"reference": "class-data-note"}),
        region("water-connections", "Flow and return stubs with caps", "Vor- und Rücklaufstutzen mit Kappen", "heater", "hydronic", "copper-oxidised", AREAS["water-connections"], "cylinder", 0.3, 5200, 0.042, hydronic={"key": "heating-circuit", "rows": [{"v": 0.3, "supplyTempC": 28, "returnTempC": 26}, {"v": 1, "supplyTempC": 35, "returnTempC": 30}], "flowPosition": 0.5, "filmM2KW": 0.002}, ratedPowerW=0, ratedPowerBasis="passive", **{"reference": "en-14511"}),
    ],
    "emitters": [
        {"id": "cold-air-jet", "kind": "fan", "title": T("Air cooled by the evaporator leaving the fan grille", "Vom Verdampfer abgekühlte Luft aus dem Lüftergitter"), "anchor": "fan-outlet", "directionDeg": 0, "diameterM": 0.56, "operatingKey": "fan-speed", "volumeFlowM3S": 0.83, "temperatureRiseK": -5, "source": "authored-estimate", "reference": "en-14511"},
        {"id": "defrost-plume", "kind": "vent", "title": T("Vapour rising from the rear coil during defrost", "Dampf vom hinteren Verdampfer beim Abtauen"), "anchor": "coil-rear", "directionDeg": 120, "diameterM": 0.5, "operatingKey": "defrost", "volumeFlowM3S": 0.15, "temperatureRiseK": 4, "vapourKgPerKg": 0.008, "source": "authored-estimate", "reference": "en-14825"},
        {"id": "condensate-drip", "kind": "vent", "title": T("Moist air with the melt water at the condensate spigot", "Feuchte Luft mit dem Schmelzwasser am Kondensatstutzen"), "anchor": "condensate-outlet", "directionDeg": 270, "diameterM": 0.024, "operatingKey": "defrost", "volumeFlowM3S": 0.002, "temperatureRiseK": 2, "vapourKgPerKg": 0.01, "source": "authored-estimate", "reference": "vdi-4645"},
    ],
    "assumptions": [
        T("Reference case: the EN 14511 heating point A7/W35 with 87 % humidity, the weather in which the coil frosts and defrosts.", "Referenzfall: der Heizprüfpunkt A7/W35 nach EN 14511 bei 87 % Feuchte, das Wetter, in dem der Verdampfer bereift und abtaut."),
        T("Stored, lifted and storm poses are switched off and drained: every surface follows sun, sky and air; the refrigerant is recovered, so no circuit holds heat or cold.", "Gelagert, gehoben und im Sturm ist das Gerät abgeschaltet und entleert: jede Fläche folgt Sonne, Himmel und Luft; das Kältemittel ist abgesaugt, kein Kreis hält Wärme oder Kälte."),
        T("The coil is an operating surface that takes about 200 W out of its face at rated evaporation (fins about 9 K below the air) and gives it back in defrost; its area is the porous face, an exchange area.", "Der Verdampfer ist eine Betriebsfläche, die bei Nennverdampfung etwa 200 W aus ihrer Fläche nimmt (Lamellen etwa 9 K unter der Luft) und sie beim Abtauen zurückgibt; ihre Fläche ist die poröse Ansichtsfläche, eine Austauschfläche."),
        T("Nameplate: 3.0 kW electrical input of compressor and controls, 0.12 kW of the fan motor and 0.1 kW of the base pan heater; the 8 kW heating capacity is not power of the asset.", "Typenschild: 3,0 kW elektrische Aufnahme von Verdichter und Regelung, 0,12 kW des Lüftermotors und 0,1 kW der Bodenwannenheizung; die Heizleistung von 8 kW ist keine Leistung des Assets."),
        T("Weathered surfaces: galvanised steel at emissivity 0.28 instead of 0.07 when new, the faded powder coat absorbs 0.48 instead of 0.40 of the sun, the pallet timber is grey at 0.65.", "Verwitterte Flächen: verzinkter Stahl mit Emissionsgrad 0,28 statt 0,07 im Neuzustand, die verblasste Pulverbeschichtung absorbiert 0,48 statt 0,40 der Sonne, das Palettenholz ist grau mit 0,65."),
    ],
}

STRUCTURE = {
    "gravityMS2": 9.81,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T("Each pose names its supports: the three bottom boards of the pallet as floor contacts, the four sling points under the console stands when lifted, the four anchor bolts of the console when reinstalled; the solver treats the asset as one rigid body, so the pallet left on the floor shares the contacts of the lifted or reinstalled unit in proportion to stiffness, not as a separate body.", "Jede Pose nennt ihre Auflager: die drei Bodenbretter der Palette als Bodenkontakte, die vier Schlingenpunkte unter den Konsolböcken beim Heben, die vier Ankerbolzen der Konsole nach dem Wiedereinbau; der Löser behandelt das Asset als einen starren Körper, die am Boden gebliebene Palette teilt daher die Auflager des gehobenen oder eingebauten Geräts nach Steifigkeit, nicht als eigener Körper."),
        T("Masses: Euro pallet 23.6 kg from 0.0453 m3 of softwood at 520 kg/m3 (EN 13698-1 geometry), console 16 kg (two folded 3 mm stands), unit 145 kg drained and without refrigerant (class data).", "Massen: Europalette 23,6 kg aus 0,0453 m3 Nadelholz zu 520 kg/m3 (Geometrie nach EN 13698-1), Konsole 16 kg (zwei gekantete 3-mm-Böcke), Gerät 145 kg entleert und ohne Kältemittel (Klassendaten)."),
        T("Storm gust: peak velocity pressure 0.39 kN/m2 at 2 m in open terrain times a block force coefficient of 1.3 on the 1.46 m2 front face (EN 1991-1-4), applied at the face centre.", "Sturmböe: Böengeschwindigkeitsdruck 0,39 kN/m2 in 2 m Höhe im offenen Gelände mal Kraftbeiwert 1,3 eines Blocks auf die Frontfläche von 1,46 m2 (EN 1991-1-4), in der Flächenmitte angesetzt."),
    ],
    "clinical": False,
}

VALID = {50: {"minDenominator": 20, "maxDenominator": 75}, 100: {"minDenominator": 75, "maxDenominator": 150}, 200: {"minDenominator": 150, "maxDenominator": 350}, 500: {"minDenominator": 350, "maxDenominator": 1000}}
PENS = {50: {"body": 0.35, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.18}}
HATCHES = {50: [{"id": "fins", "title": T("Evaporator fins", "Verdampferlamellen"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.8, "penMm": 0.1}], 100: [{"id": "fins", "title": T("Evaporator fins", "Verdampferlamellen"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.1}], 200: [], 500: []}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝️ The eight decisions of a representation, each a pair (en, de)."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


REP_DECISIONS = {
    50: D(
        ("The true views of the 1:50 model: pallet board by board, the two console stands with their lightening windows, mounts and cut anchor bolts, the casing with base pan, cover, grille ring 40 mm proud of the front, capped stubs at the back and the spigot under the pan.", "Die wahren Ansichten des 1:50-Modells: Palette Brett für Brett, die zwei Konsolböcke mit ihren Erleichterungsfenstern, Dämpfern und abgetrennten Ankerbolzen, das Gehäuse mit Bodenwanne, Haube, Gitterring 40 mm vor der Front, verschlossenen Stutzen hinten und dem Stutzen unter der Wanne."),
        ("Side: five top boards, the stringer, three blocks and three bottom boards, the right stand, two mounts, two bolt stubs, the right side coil with its fin hatch and the crushed fin patch, the front post, the return stub with its cap. Front: the fan opening with the coil and its fins behind, motor, three blades and hub, the grille with ring, three hoops, eight spokes and cap. Plan: cover, grille ring, both stubs and caps, the stand ends and the pallet boards around the unit.", "Seite: fünf Deckbretter, das Längsbrett, drei Klötze und drei Bodenbretter, der rechte Bock, zwei Dämpfer, zwei Bolzenstummel, der rechte Seitenverdampfer mit Lamellenschraffur und der zerdrückten Stelle, der vordere Eckpfosten, der Rücklaufstutzen mit Kappe. Front: die Lüfteröffnung mit dem Verdampfer und seinen Lamellen dahinter, Motor, drei Flügel und Nabe, das Gitter mit Ring, drei Ringen, acht Speichen und Kappe. Draufsicht: Haube, Gitterring, beide Stutzen mit Kappen, die Bockenden und die Palettenbretter um das Gerät."),
        ("The flow stub lies behind the return stub in the side view and is the same shape; the three block rows of the pallet coincide in the side view; the casing frame behind the coil carries the casing and the compressor compartment, which faces away on the left.", "Der Vorlaufstutzen liegt in der Seitenansicht hinter dem Rücklaufstutzen und ist dieselbe Form; die drei Klotzreihen der Palette fallen in der Seitenansicht zusammen; der Gehäuserahmen hinter dem Verdampfer trägt Gehäuse und Verdichterraum, der links abgewandt liegt."),
        ("Screws, sheet folds, the refrigerant pipework and compressor inside the closed compartment, the slings (only their points are anchors) and the straps.", "Schrauben, Blechkanten, die Kältemittelleitungen und der Verdichter im geschlossenen Raum, die Schlingen (nur ihre Punkte sind Anker) und die Gurte."),
        ("Grille wires 4 mm drawn as 0.13 mm strokes; bolt stubs 12 mm (0.24 mm) and the spigot 24 mm stay filled.", "Gitterdrähte 4 mm als 0,13-mm-Striche; Bolzenstummel 12 mm (0,24 mm) und Stutzen 24 mm bleiben gefüllt."),
        ("Vertical fin hatch at 0.8 mm on the side coil and in the coil seen through the fan opening (real fin pitch 2 mm, typified).", "Senkrechte Lamellenschraffur mit 0,8 mm auf dem Seitenverdampfer und im Verdampfer hinter der Lüfteröffnung (wahre Lamellenteilung 2 mm, typisiert)."),
        ("Pens 0.35 / 0.25 / 0.13 mm; light faded casing, dark coil and grille, mid-grey galvanised console, grey weathered timber.", "Stifte 0,35 / 0,25 / 0,13 mm; helles verblasstes Gehäuse, dunkler Verdampfer und dunkles Gitter, mittelgrau verzinkte Konsole, graues verwittertes Holz."),
        ("The fan turns in the heating clip; the drawing shows the blades at rest. Thermal and air flow overlays use the region of every shape.", "Der Lüfter dreht sich im Heizclip; die Zeichnung zeigt die Flügel in Ruhe. Thermische und Luftströmungsüberlagerungen nutzen die Region jeder Form."),
    ),
    100: D(
        ("The true views of the 1:100 model: the stringer and the three blocks of each pallet row as one runner, the stands reaching the base pan, the stubs with their caps as one cylinder each, the grille as its ring.", "Die wahren Ansichten des 1:100-Modells: Längsbrett und drei Klötze jeder Palettenreihe als ein Kufenprofil, die Böcke bis an die Bodenwanne, die Stutzen mit Kappen als je ein Zylinder, das Gitter als sein Ring."),
        ("Top and bottom boards, runners, the stand windows, base pan, compartment, cover, ring, the fan with blades and hub behind the opening, stubs and spigot.", "Deck- und Bodenbretter, Kufen, die Bockfenster, Bodenwanne, Verdichterraum, Haube, Ring, der Lüfter mit Flügeln und Nabe hinter der Öffnung, Stutzen und Kondensatstutzen."),
        ("Stringer and blocks are one runner profile with its three notches; mounts are part of the stand top; caps are part of the stubs.", "Längsbrett und Klötze sind ein Kufenprofil mit seinen drei Aussparungen; die Dämpfer gehören zum Bockkopf; die Kappen gehören zu den Stutzen."),
        ("Grille wires, bolt stubs, type plate and the crushed fin patch (below 15 mm or texture only).", "Gitterdrähte, Bolzenstummel, Typenschild und die zerdrückte Lamellenstelle (unter 15 mm oder nur Textur)."),
        ("The spigot 24 mm (0.24 mm) is the smallest filled part.", "Der Kondensatstutzen 24 mm (0,24 mm) ist die kleinste gefüllte Form."),
        ("Fin hatch at 0.6 mm on the side coil and in the coil behind the opening.", "Lamellenschraffur mit 0,6 mm auf dem Seitenverdampfer und im Verdampfer hinter der Öffnung."),
        ("Pens 0.25 / 0.18 / 0.1 mm, tones as at 1:50.", "Stifte 0,25 / 0,18 / 0,1 mm, Tonwerte wie bei 1:50."),
        ("As at 1:50.", "Wie bei 1:50."),
    ),
    200: D(
        ("The true views of the 1:200 model: pallet as deck and three bearers, solid trapezoid stands, the casing as one block under its cover, the grille as a disc, the two stubs as one block.", "Die wahren Ansichten des 1:200-Modells: Palette als Deck und drei Kufen, volle Trapezböcke, das Gehäuse als ein Block unter seiner Haube, das Gitter als Scheibe, die zwei Stutzen als ein Block."),
        ("Pallet height and bearers, stand profile, the cover overhang, the 40 mm grille disc and the stub block, the compartment joint and the fan ring as strokes on the front.", "Palettenhöhe und Kufen, Bockprofil, der Haubenüberstand, die 40-mm-Gitterscheibe und der Stutzenblock, die Fuge des Verdichterraums und der Lüfterring als Striche auf der Front."),
        ("Top boards and stringers are one deck, blocks and bottom boards one bearer per row; base pan, compartment, coil and front panel are one casing block carrying their regions.", "Deckbretter und Längsbretter sind ein Deck, Klötze und Bodenbretter eine Kufe je Reihe; Bodenwanne, Verdichterraum, Verdampfer und Frontblech sind ein Gehäuseblock mit ihren Regionen."),
        ("Fan, motor, stand windows, mounts, spigot and caps (below 30 mm or behind the merged front).", "Lüfter, Motor, Bockfenster, Dämpfer, Kondensatstutzen und Kappen (unter 30 mm oder hinter der zusammengefassten Front)."),
        ("The 60 mm stands (0.3 mm) are the thinnest filled parts.", "Die 60-mm-Böcke (0,3 mm) sind die dünnsten gefüllten Formen."),
        ("No hatch at this size.", "Keine Schraffur in dieser Größe."),
        ("Pens 0.18 / 0.13 mm.", "Stifte 0,18 / 0,13 mm."),
        ("Overlays colour the casing with the area-weighted value of its merged regions (labelled merged).", "Überlagerungen färben das Gehäuse mit dem flächengewichteten Wert seiner zusammengefassten Regionen (als zusammengefasst gekennzeichnet)."),
    ),
    500: D(
        ("Pallet as one slab, two stands of 75 mm, the unit as one block from base pan to cover; the fan opening as one stroke on the front.", "Palette als eine Platte, zwei Böcke von 75 mm, das Gerät als ein Block von der Bodenwanne bis zur Haube; die Lüfteröffnung als ein Strich auf der Front."),
        ("Overall height 1.744 m, pallet 0.80 x 1.20 m, the console height and the stand spacing, the unit footprint and the fan side.", "Gesamthöhe 1,744 m, Palette 0,80 x 1,20 m, die Konsolhöhe und der Bockabstand, die Grundfläche des Geräts und die Lüfterseite."),
        ("All unit regions in one block; the pallet in one slab; the stands thickened from 60 to 75 mm.", "Alle Geräteregionen in einem Block; die Palette in einer Platte; die Böcke von 60 auf 75 mm verdickt."),
        ("Grille ring, cover overhang, stubs, spigot and every detail below 75 mm.", "Gitterring, Haubenüberstand, Stutzen, Kondensatstutzen und jede Einzelheit unter 75 mm."),
        ("The 75 mm stands (0.15 mm plus the 0.13 mm pen) are the thinnest parts.", "Die 75-mm-Böcke (0,15 mm plus 0,13-mm-Stift) sind die dünnsten Teile."),
        ("No hatch.", "Keine Schraffur."),
        ("Pens 0.13 / 0.1 mm.", "Stifte 0,13 / 0,1 mm."),
        ("Overlays colour the unit block with the area-weighted value of its merged regions (labelled merged).", "Überlagerungen färben den Geräteblock mit dem flächengewichteten Wert seiner zusammengefassten Regionen (als zusammengefasst gekennzeichnet)."),
    ),
}

VOLUME_DECISIONS = {
    50: [
        T("Body: the EUR pallet board by board (three bottom boards, nine blocks as three repeated rows, three stringer boards, five top boards), the console as two 60 mm folded stands with lightening windows, four EPDM mounts and four anchor bolts cut 25 mm above the feet, the unit panel by panel: base pan, compressor compartment, L-shaped coil, front post, 10 mm front panel with the 0.60 m opening, cover with 10 mm overhang, grille ring, three hoops and eight spokes of 4 mm wire (slender), cap, motor, bracket, the two 42 mm stubs with red caps and the 24 mm condensate spigot.", "Körper: die Europalette Brett für Brett (drei Bodenbretter, neun Klötze als drei wiederholte Reihen, drei Längsbretter, fünf Deckbretter), die Konsole als zwei gekantete 60-mm-Böcke mit Erleichterungsfenstern, vier EPDM-Dämpfer und vier 25 mm über den Füßen abgetrennte Ankerbolzen, das Gerät Blech für Blech: Bodenwanne, Verdichterraum, L-förmiger Verdampfer, vorderer Eckpfosten, 10-mm-Frontblech mit der 0,60-m-Öffnung, Haube mit 10 mm Überstand, Gitterring, drei Ringe und acht Speichen aus 4-mm-Draht (schlank), Kappe, Motor, Halter, die zwei 42-mm-Stutzen mit roten Kappen und der 24-mm-Kondensatstutzen."),
        T("Fan: hub and three 5 mm blades of glass-fibre polypropylene pitched at 25 degrees, declared sheets. Type plate and the crushed fin patch are texture sheets on the front panel and on the right coil face.", "Lüfter: Nabe und drei 5-mm-Flügel aus glasfaserverstärktem Polypropylen mit 25 Grad Anstellung, als Blätter erklärt. Typenschild und die zerdrückte Lamellenstelle sind Texturblätter auf dem Frontblech und auf der rechten Verdampferfläche."),
        T("Thermal: every body solid by reference in its region, plus the three emitter outlets 8 mm thick: the disc in the grille, a patch on the rear coil face, the mouth of the spigot.", "Thermisch: jeder Körper per Verweis in seiner Region, dazu die drei 8 mm dicken Emitter-Auslässe: die Scheibe im Gitter, eine Fläche auf der hinteren Verdampferfläche, die Mündung des Stutzens."),
        T("Structure: the pallet boards and blocks as members, the stands as members with the mounts as joints, four 40 mm corner posts, base pan, cover, bracket and motor as the unit frame, the hub with its bearing, three mass markers.", "Tragwerk: die Palettenbretter und Klötze als Glieder, die Böcke als Glieder mit den Dämpfern als Gelenken, vier 40-mm-Eckpfosten, Bodenwanne, Haube, Halter und Motor als Rahmen des Geräts, die Nabe mit ihrem Lager, drei Massenmarken."),
    ],
    100: [
        T("Body: stringer and three blocks of each pallet row merged into one notched runner extrude, top and bottom boards kept (22 mm), stands without mounts and bolts reaching the base pan, caps merged into the stubs as 52 mm cylinders, front panel 15 mm, grille as its ring only; wires, type plate and fin patch left out.", "Körper: Längsbrett und drei Klötze jeder Palettenreihe zu einer gekerbten Kufe zusammengefasst, Deck- und Bodenbretter behalten (22 mm), Böcke ohne Dämpfer und Bolzen bis an die Bodenwanne, Kappen in die Stutzen als 52-mm-Zylinder aufgenommen, Frontblech 15 mm, Gitter nur als Ring; Drähte, Typenschild und Lamellenstelle entfallen."),
        T("Thermal: references in the regions and 15 mm emitter outlets.", "Thermisch: Verweise in den Regionen und 15-mm-Emitter-Auslässe."),
        T("Structure: runners and boards as pallet members, stands as members with two 15 mm mount pads per stand as joints, posts, pan, cover, bracket, motor, hub and bearing, three mass markers.", "Tragwerk: Kufen und Bretter als Palettenglieder, Böcke als Glieder mit zwei 15-mm-Dämpferplatten je Bock als Gelenken, Pfosten, Wanne, Haube, Halter, Motor, Nabe und Lager, drei Massenmarken."),
    ],
    200: [
        T("Body: pallet as a 44 mm deck and three 100 mm bearers, stands as full trapezoids, the casing as one block, the cover, a 40 mm grille disc and the stubs as one block; fan, motor, spigot and stand windows left out.", "Körper: Palette als 44-mm-Deck und drei 100-mm-Kufen, Böcke als volle Trapeze, das Gehäuse als ein Block, die Haube, eine 40-mm-Gitterscheibe und die Stutzen als ein Block; Lüfter, Motor, Kondensatstutzen und Bockfenster entfallen."),
        T("Thermal: the merged casing split into its own region solids (pan, compartment, L coil, 30 mm front panel with the opening, fan cylinder, grille ring), the rest by reference; 30 mm emitter outlets, the drip outlet under the pan.", "Thermisch: das zusammengefasste Gehäuse in eigene Regionskörper geteilt (Wanne, Verdichterraum, L-Verdampfer, 30-mm-Frontblech mit Öffnung, Lüfterzylinder, Gitterring), der Rest per Verweis; 30-mm-Emitter-Auslässe, der Tropfauslass unter der Wanne."),
        T("Structure: deck and bearers, stands, four 50 mm posts, base and top frame as 60 mm plates, three mass markers; the fan is not a member at this scale.", "Tragwerk: Deck und Kufen, Böcke, vier 50-mm-Pfosten, Grund- und Kopfrahmen als 60-mm-Platten, drei Massenmarken; der Lüfter ist in diesem Maßstab kein Glied."),
    ],
    500: [
        T("Body: the pallet as one 144 mm slab, the stands as two 75 mm blocks with the mounts, the unit as one block from the base pan to the top of the cover.", "Körper: die Palette als eine 144-mm-Platte, die Böcke als zwei 75-mm-Blöcke mit den Dämpfern, das Gerät als ein Block von der Bodenwanne bis zur Oberkante der Haube."),
        T("Thermal: pallet and stands by reference, the unit as 75 mm region solids (pan, compartment, L coil, front panel with the opening, grille ring, fan, cover, stub block); 75 mm emitter outlets.", "Thermisch: Palette und Böcke per Verweis, das Gerät als 75-mm-Regionskörper (Wanne, Verdichterraum, L-Verdampfer, Frontblech mit Öffnung, Gitterring, Lüfter, Haube, Stutzenblock); 75-mm-Emitter-Auslässe."),
        T("Structure: slab, stands, four 80 mm posts, 75 mm base and top frames, three 80 mm mass markers.", "Tragwerk: Platte, Böcke, vier 80-mm-Pfosten, 75-mm-Grund- und Kopfrahmen, drei 80-mm-Massenmarken."),
    ],
}
