"""📜 Texts, materials, rig, anchors, poses, clips, thermal and structural model of the used washing machine; loaded by `author-used-washing-machine.py` into its namespace (needs `T`, `r6`, `PALLET_KG`, `PALLET_COMY`, `CABINET_KG`, `CABINET_COM`, `DRUM_KG`, `DRUM_COM`).

@see ./author-used-washing-machine.py
"""

SUMMARY = T(
    "A used 7 kg front-loading washing machine of the common 2005 to 2015 German mid-range type, as a second-hand department store takes it in from a flat clearance: a white enamelled steel cabinet 595 mm wide, 540 mm deep and 805 mm high on four screw feet, a 25 mm laminated worktop, the control fascia with the detergent drawer at the top left and the programme knob at the right, and the porthole door of 340 mm with its glass bowl. It was emptied, drained and wiped, the three transit bolts were screwed back into the back panel for the transport, the inlet and drain hoses and the mains cable are coiled and taped to the back and the door is taped shut. Its use left limescale crusted in the door seal, yellowed and scratched enamel, a dent and rust at the lower front corner, detergent residue at the drawer, a chipped worktop corner, cup rings and the residue of the energy label. It stands on an EPAL half pallet (800 x 600 mm), is carried tilted back by two people with the pallet left behind, and back in a utility room it runs the 60 C cotton programme: the heater warms the water, the drum tumbles and finally spins.",
    "Eine gebrauchte 7-kg-Frontlader-Waschmaschine der verbreiteten deutschen Mittelklasse der Jahre 2005 bis 2015, wie ein Gebrauchtwarenkaufhaus sie aus einer Haushaltsauflösung übernimmt: ein weiß emailliertes Stahlgehäuse von 595 mm Breite, 540 mm Tiefe und 805 mm Höhe auf vier Schraubfüßen, eine 25 mm starke beschichtete Arbeitsplatte, die Bedienblende mit der Waschmittelschublade oben links und dem Programmwahlknopf rechts und die Bullaugentür von 340 mm mit ihrer Glasschüssel. Sie wurde geleert, entwässert und abgewischt, die drei Transportsicherungen sind für den Transport wieder in die Rückwand geschraubt, Zulauf- und Ablaufschlauch und das Netzkabel sind aufgerollt und auf die Rückseite geklebt, und die Tür ist zugeklebt. Ihr Gebrauch hinterließ Kalkkrusten in der Türmanschette, vergilbten und zerkratzten Lack, eine Delle und Rost an der unteren vorderen Ecke, Waschmittelreste an der Schublade, eine abgeplatzte Ecke der Arbeitsplatte, Tassenränder und die Reste des Energielabels. Sie steht auf einer EPAL-Halbpalette (800 x 600 mm), wird von zwei Personen nach hinten gekippt getragen, während die Palette stehen bleibt, und läuft wieder in einem Hauswirtschaftsraum das Baumwollprogramm 60 C: Die Heizung erwärmt das Wasser, die Trommel wendet und schleudert zum Schluss.",
)

PROVENANCE = {
    "author": "Semio staffage author 22 of the used household objects wave",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": [
        {"id": "class-data-note", "citation": "Catalogue-typical data of 7 kg front-loading household washing machines of German mid-range brands sold 2005 to 2015 (types, not brands; quoted as orders of magnitude from memory of data sheets and installation manuals, not retrieved for this asset): cabinet 595 to 600 mm wide, 845 to 850 mm high including feet, 540 to 590 mm deep plus 40 to 60 mm door; porthole door 300 to 340 mm with a glass bowl; 65 to 80 kg empty with two concrete counterweights of about 25 kg in all; drum of 50 to 55 l, 1400 rpm; heater 2.0 kW, connected load 2.2 to 2.3 kW at 230 V; three or four transit bolts in the back panel that must be refitted for every transport; inlet hose 1.5 m (3/4 inch), drain hose 1.5 m with a U-bend hook. Every number derived from it is tagged authored-estimate."},
        {"id": "en-60456", "citation": "EN 60456:2016 Clothes washing machines for household use, Methods for measuring the performance (rated capacity, cotton 60 C programme, water and energy consumption)."},
        {"id": "en-60335-2-7", "citation": "EN 60335-2-7:2010+A2:2019 Household and similar electrical appliances, Safety, Part 2-7: Particular requirements for washing machines (rated power input and stability)."},
        {"id": "din-18022", "citation": "DIN 18022:2007-11 Küchen, Bad und WC im Wohnungsbau, Planungsgrundlagen (Stellfläche einer Waschmaschine 60 x 60 cm, Bewegungsfläche davor)."},
        {"id": "epal-half-pallet", "citation": "EPAL technical regulations for the EPAL 6 half pallet, 800 x 600 x 144 mm, about 9.5 kg (board and block layout simplified to three bottom boards, nine 100 x 100 x 78 mm blocks, three stringer boards and five top boards of 100 x 22 mm)."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, Strength classes (C24 softwood mean density 420 kg/m3; pallet timber taken at 450 kg/m3 air-dry)."},
        {"id": "en-312", "citation": "EN 312:2010 Particleboards, Specifications (type P2 for interior fitments, density 600 to 680 kg/m3), the core of the melamine-faced worktop."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, Hygrothermal properties (softwood 500 kg/m3, 0.13 W/mK, 1600 J/kgK; particleboard 600 kg/m3, 0.14 W/mK, 1700 J/kgK)."},
        {"id": "iso-11228-1", "citation": "ISO 11228-1:2021 Ergonomics, Manual handling, Part 1: Lifting, lowering and carrying (reference mass 25 kg per person; heavier household appliances are carried by two people with carrying straps)."},
        {"id": "incropera-2011", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley. Appendix A (properties of steel, stainless steel, glass, polymers) and Table A.11 (total emissivities)."},
        {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), Chapter 4 Heat Transfer and Chapter 33 (emissivity and solar absorptance of white enamel and paint, new and aged; galvanised steel)."},
    ],
    "notes": T(
        "Class data of a 7 kg front loader as it comes back from a flat clearance: emptied, the residual water drained through the pump filter, the three transit bolts refitted, the hoses and the cable taped to the back, the door taped shut. Nothing is measured; the cabinet follows the 60 x 60 cm module of DIN 18022, the masses the class data (72 kg machine with 25 kg of counterweights inside), the pallet the EPAL half pallet and the operating point the EN 60456 cotton 60 C programme with 14 l of water heated by 2.0 kW.",
        "Klassendaten eines 7-kg-Frontladers, wie er aus einer Haushaltsauflösung zurückkommt: geleert, das Restwasser über das Flusensieb abgelassen, die drei Transportsicherungen wieder eingesetzt, Schläuche und Kabel auf die Rückseite geklebt, die Tür zugeklebt. Nichts ist gemessen; das Gehäuse folgt dem 60-x-60-cm-Raster der DIN 18022, die Massen den Klassendaten (72 kg Maschine mit 25 kg Gegengewichten im Inneren), die Palette der EPAL-Halbpalette und der Betriebspunkt dem Baumwollprogramm 60 C nach EN 60456 mit 14 l Wasser, die 2,0 kW erwärmen.",
    ),
}

MATERIALS = [
    {"id": "pallet-softwood", "title": T("Grey pallet softwood (spruce, pine)", "Graues Palettenholz (Fichte, Kiefer)"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 10000000000, "strengthPa": 21000000, "source": "literature", "reference": "en-338"},
    {"id": "enamelled-steel-aged", "title": T("Yellowed white enamelled sheet steel 0.7 mm", "Vergilbtes weiß emailliertes Stahlblech 0,7 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.32, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 280000000, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "galvanised-steel", "title": T("Galvanised back panel and transit bolts", "Verzinkte Rückwand und Transportsicherungen"), "emissivity": 0.25, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 280000000, "source": "literature", "reference": "ashrae-2021"},
    {"id": "steel-foot", "title": T("Zinc-plated screw foot with rubber pad", "Verzinkter Schraubfuß mit Gummiauflage"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "incropera-2011"},
    {"id": "melamine-chipboard-aged", "title": T("Melamine-faced P2 chipboard worktop, scratched and chipped", "Melaminbeschichtete Spanplatte P2, zerkratzt und abgeplatzt"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": 650, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 2500000000, "strengthPa": 11000000, "source": "literature", "reference": "en-312"},
    {"id": "abs-aged", "title": T("Yellowed ABS of fascia, drawer and knob", "Vergilbtes ABS von Blende, Schublade und Knopf"), "emissivity": 0.92, "solarAbsorptivity": 0.4, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "incropera-2011"},
    {"id": "abs-chrome-worn", "title": T("Worn chrome-look ABS of the door ring", "Abgegriffenes ABS in Chromoptik des Türrings"), "emissivity": 0.5, "solarAbsorptivity": 0.45, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "incropera-2011"},
    {"id": "door-glass", "title": T("Pressed soda-lime glass bowl of the door", "Gepresste Kalknatronglasschüssel der Tür"), "emissivity": 0.92, "solarAbsorptivity": 0.15, "densityKgM3": 2500, "conductivityWmK": 1.0, "specificHeatJKgK": 840, "source": "literature", "reference": "incropera-2011"},
    {"id": "pvc-hose", "title": T("Grey PVC drain and inlet hoses with cable", "Graue PVC-Ablauf- und Zulaufschläuche mit Kabel"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 1300, "conductivityWmK": 0.19, "specificHeatJKgK": 1000, "source": "literature", "reference": "incropera-2011"},
    {"id": "pp-tub", "title": T("Glass-filled polypropylene tub with the concrete counterweights", "Laugenbehälter aus glasgefülltem Polypropylen mit den Betongegengewichten"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 1200, "conductivityWmK": 0.3, "specificHeatJKgK": 1500, "source": "literature", "reference": "incropera-2011"},
    {"id": "stainless-steel", "title": T("Stainless steel drum", "Edelstahltrommel"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 7900, "conductivityWmK": 15, "specificHeatJKgK": 500, "elasticModulusPa": 193000000000, "source": "literature", "reference": "incropera-2011"},
]

RIG = {
    "kind": "mechanical",
    "root": "floor",
    "bones": [
        {"id": "floor", "parent": None, "title": T("Depot floor (massless reference)", "Lagerboden (masseloser Bezug)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "pallet", "parent": "floor", "title": T("EPAL half pallet 800 x 600 x 144 mm (storage aid, stays behind)", "EPAL-Halbpalette 800 x 600 x 144 mm (Lagerhilfe, bleibt stehen)"), "x": -0.40, "y": 0, "angleDeg": 0, "lengthM": 0.80, "joint": {"type": "fixed"}, "mass": {"kg": r6(PALLET_KG), "comX": 0.40, "comY": r6(PALLET_COMY), "distribution": "point", "source": "authored-estimate", "reference": "epal-half-pallet"}},
        {"id": "lift", "parent": "floor", "title": T("Carriers' hands: lifting and setting down", "Hände der Träger: Heben und Absetzen"), "x": -0.29, "y": 0.164, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.144, "maxM": 0.2}},
        {"id": "travel", "parent": "lift", "title": T("Carriers' hands: walking away from the pallet", "Hände der Träger: Weggehen von der Palette"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.2}},
        {"id": "cabinet", "parent": "travel", "title": T("Machine cabinet with worktop, fascia, tub, counterweights, motor, back panel and hoses, tilting back about its rear bottom edge", "Maschinengehäuse mit Arbeitsplatte, Blende, Laugenbehälter, Gegengewichten, Motor, Rückwand und Schläuchen, nach hinten kippend um seine hintere Unterkante"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.54, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 30}, "mass": {"kg": CABINET_KG, "comX": CABINET_COM[0], "comY": CABINET_COM[1], "distribution": "point", "source": "authored-estimate", "reference": "class-data-note"}},
        {"id": "drum", "parent": "cabinet", "title": T("Stainless drum on its rear bearing, turning about the door axis", "Edelstahltrommel auf ihrem hinteren Lager, drehend um die Türachse"), "x": 0.17, "y": 0.46, "angleDeg": 0, "lengthM": 0.33, "joint": {"type": "revolute", "continuousTwist": True}, "mass": {"kg": DRUM_KG, "comX": DRUM_COM[0], "comY": DRUM_COM[1], "distribution": "point", "source": "authored-estimate", "reference": "class-data-note"}},
        {"id": "door", "parent": "cabinet", "title": T("Porthole door on its vertical hinge at the right (its 3 kg counted in the cabinet)", "Bullaugentür an ihrem senkrechten Scharnier rechts (ihre 3 kg im Gehäuse enthalten)"), "x": 0.552, "y": 0.46, "z": -0.18, "angleDeg": 0, "twistDeg": 90, "lengthM": 0.04, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 175}},
    ],
}

ANCHORS = [
    {"id": "pallet-contact-rear", "bone": "pallet", "x": 0.05, "y": 0, "kind": "contact", "title": T("Rear bottom board of the pallet on the floor", "Hinteres Bodenbrett der Palette auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 0.60, "frictionCoefficient": 0.5},
    {"id": "pallet-contact-mid", "bone": "pallet", "x": 0.40, "y": 0, "kind": "contact", "title": T("Middle bottom board of the pallet on the floor", "Mittleres Bodenbrett der Palette auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 0.60, "frictionCoefficient": 0.5},
    {"id": "pallet-contact-front", "bone": "pallet", "x": 0.75, "y": 0, "kind": "contact", "title": T("Front bottom board of the pallet on the floor", "Vorderes Bodenbrett der Palette auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 0.60, "frictionCoefficient": 0.5},
]
FOOT_NAMES = {(-0.19, -0.25): ("rear-r", "rear right", "hinten rechts"), (-0.19, 0.25): ("rear-l", "rear left", "hinten links"), (0.19, -0.25): ("front-r", "front right", "vorne rechts"), (0.19, 0.25): ("front-l", "front left", "vorne links")}
for (fx, fz), (nm, en, de) in FOOT_NAMES.items():
    ANCHORS.append({"id": f"foot-{nm}", "bone": "cabinet", "x": r6(fx + 0.29), "y": -0.02, "z": fz, "kind": "contact", "title": T(f"Screw foot {en} on the floor", f"Schraubfuß {de} auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.036, "footprintWidthM": 0.036, "frictionCoefficient": 0.6})
for side, z, en, de in (("r", -0.2975, "right", "rechts"), ("l", 0.2975, "left", "links")):
    ANCHORS.append({"id": f"grip-low-{side}", "bone": "cabinet", "x": 0.54, "y": 0.0, "z": z, "kind": "support", "title": T(f"Lower carrier's hand under the front bottom edge, {en}", f"Hand des unteren Trägers unter der vorderen Unterkante, {de}"), "support": "pin"})
    ANCHORS.append({"id": f"grip-high-{side}", "bone": "cabinet", "x": 0.0, "y": 0.80, "z": z, "kind": "support", "title": T(f"Upper carrier's hand at the rear top edge under the worktop, {en}", f"Hand des oberen Trägers an der hinteren Oberkante unter der Arbeitsplatte, {de}"), "support": "pin"})
ANCHORS += [
    {"id": "drawer-vent", "bone": "cabinet", "x": 0.565, "y": 0.751, "z": -0.185, "kind": "emitter", "title": T("Vent slot of the tub at the detergent drawer", "Entlüftungsschlitz des Laugenbehälters an der Waschmittelschublade")},
    {"id": "drum-centre", "bone": "drum", "x": 0.165, "y": 0, "kind": "load", "title": T("Centre of the drum (laundry and water)", "Mitte der Trommel (Wäsche und Wasser)")},
]

PALLET_CONTACTS = ["pallet-contact-rear", "pallet-contact-mid", "pallet-contact-front"]
GRIPS = ["grip-low-r", "grip-low-l", "grip-high-r", "grip-high-l"]
FEET_CONTACTS = ["foot-rear-r", "foot-rear-l", "foot-front-r", "foot-front-l"]
REGION_IDS = ["pallet", "worktop", "cabinet", "fascia", "door", "back-panel", "hoses"]


def bind(on_floor):
    """🌬️ The region bindings of a pose: everything in room air without sun, the pallet on the floor, the feet on the floor when installed."""
    out = {}
    for r in REGION_IDS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "pallet":
            b = {"exposedFraction": 0.88, "sunlitFraction": 0, "contact": {"fraction": 0.12, "tempC": 18}}
        if r == "cabinet" and on_floor:
            b = {"exposedFraction": 0.99, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 20}}
        out[r] = b
    return out


def ops(h, m):
    """⚙️ The operating levels of heater and drum motor."""
    return {"heater": h, "motor": m}


def laundry(kg, en, de):
    """🧺️ The weight of laundry and water in the drum."""
    return {"id": "laundry", "kind": "equipment", "title": T(en, de), "anchor": "drum-centre", "fxN": 0, "fyN": r6(-kg * 9.81), "source": "authored-estimate", "reference": "en-60456"}


INSTALLED = {"lift": {"dy": -0.144}, "travel": {"dx": 0.95}}
POSES = [
    {"id": "stored", "title": T("Stored on the half pallet", "Auf der Halbpalette gelagert"), "description": T("Emptied, drained, transit bolts refitted, hoses and cable taped to the back and the door taped shut, the machine stands on its four feet on an EPAL half pallet in the second-hand store: everything at room temperature.", "Geleert, entwässert, Transportsicherungen eingesetzt, Schläuche und Kabel auf die Rückseite geklebt und die Tür zugeklebt, steht die Maschine auf ihren vier Füßen auf einer EPAL-Halbpalette im Gebrauchtwarenkaufhaus: alles auf Raumtemperatur."), "bones": {}, "ground": "authored", "contacts": PALLET_CONTACTS, "thermal": {"operating": ops(0, 0), "regions": bind(False), "emitters": {"tub-vent": 0}}, "loads": []},
    {"id": "carried", "title": T("Carried by two people, tilted back", "Von zwei Personen getragen, nach hinten gekippt"), "description": T("Two carriers hold the 72 kg machine tilted back by 20 degrees, one hand pair under the front bottom edge and one at the rear top edge, about 36 kg each with carrying straps; the empty pallet stays where it was.", "Zwei Träger halten die 72-kg-Maschine um 20 Grad nach hinten gekippt, ein Händepaar unter der vorderen Unterkante und eines an der hinteren Oberkante, je etwa 36 kg mit Tragegurten; die leere Palette bleibt stehen."), "bones": {"lift": {"dy": 0.10}, "travel": {"dx": 0.95}, "cabinet": {"angleDeg": 20}}, "ground": "authored", "contacts": PALLET_CONTACTS + GRIPS, "thermal": {"operating": ops(0, 0), "regions": bind(False), "emitters": {"tub-vent": 0}}, "loads": []},
    {"id": "in-use", "title": T("In use: heating for the 60 C wash", "In Betrieb: Aufheizen für die 60-C-Wäsche"), "description": T("Back in a utility room on its four feet, connected to tap and siphon, the machine heats 14 l of water with 7 kg of cotton to 60 C with its 2.0 kW heater while the drum tumbles; warm moist air escapes at the detergent drawer; the empty pallet stands behind.", "Wieder in einem Hauswirtschaftsraum auf ihren vier Füßen, an Wasserhahn und Siphon angeschlossen, heizt die Maschine 14 l Wasser mit 7 kg Baumwolle mit ihrer 2,0-kW-Heizung auf 60 C, während die Trommel wendet; warme feuchte Luft entweicht an der Waschmittelschublade; die leere Palette steht dahinter."), "bones": INSTALLED, "ground": "authored", "contacts": PALLET_CONTACTS + FEET_CONTACTS, "thermal": {"operating": ops(1, 0.3), "regions": bind(True), "emitters": {"tub-vent": 1}}, "loads": [laundry(21, "Wet laundry 7 kg and 14 l of water in drum and tub (0.21 kN)", "Nasse Wäsche 7 kg und 14 l Wasser in Trommel und Laugenbehälter (0,21 kN)")]},
    {"id": "spinning", "title": T("In use: final spin at 1400 rpm", "In Betrieb: Endschleudern mit 1400 U/min"), "description": T("The water is pumped out, the heater is off and the drum spins the laundry at 1400 rpm; the laundry still holds about 4 l of water.", "Das Wasser ist abgepumpt, die Heizung aus und die Trommel schleudert die Wäsche mit 1400 U/min; die Wäsche hält noch etwa 4 l Wasser."), "bones": INSTALLED, "ground": "authored", "contacts": PALLET_CONTACTS + FEET_CONTACTS, "thermal": {"operating": ops(0, 1), "regions": bind(True), "emitters": {"tub-vent": 0.2}}, "loads": [laundry(11, "Spun laundry 7 kg with 4 l of water (0.11 kN)", "Geschleuderte Wäsche 7 kg mit 4 l Wasser (0,11 kN)")]},
    {"id": "lifted-key", "key": True, "title": T("Lifted off the pallet", "Von der Palette gehoben"), "description": T("Key of the carry clip: the machine is lifted 100 mm off the pallet, still upright.", "Schlüssel des Trageclips: Die Maschine ist 100 mm von der Palette gehoben, noch aufrecht."), "bones": {"lift": {"dy": 0.10}}, "ground": "authored", "contacts": PALLET_CONTACTS + GRIPS, "thermal": {"operating": ops(0, 0), "regions": bind(False), "emitters": {"tub-vent": 0}}, "loads": []},
    {"id": "tilted-key", "key": True, "title": T("Tilted back above the pallet", "Über der Palette nach hinten gekippt"), "description": T("Key of the carry clip: lifted and tilted back by 20 degrees above the pallet.", "Schlüssel des Trageclips: gehoben und über der Palette um 20 Grad nach hinten gekippt."), "bones": {"lift": {"dy": 0.10}, "cabinet": {"angleDeg": 20}}, "ground": "authored", "contacts": PALLET_CONTACTS + GRIPS, "thermal": {"operating": ops(0, 0), "regions": bind(False), "emitters": {"tub-vent": 0}}, "loads": []},
]


def drum_key(t, pose, deg, ease="smooth"):
    """🥁️ A wash clip key with the drum turned by `deg`."""
    return {"t": t, "pose": pose, "ease": ease, "bones": {"drum": {"twistDeg": deg}}}


CLIPS = [
    {"id": "carry", "title": T("Carry away from the pallet", "Von der Palette wegtragen"), "description": T("The two carriers lift the machine 100 mm off the pallet, tilt it back by 20 degrees and walk 0.95 m away; the pallet stays on the floor.", "Die zwei Träger heben die Maschine 100 mm von der Palette, kippen sie um 20 Grad nach hinten und gehen 0,95 m weg; die Palette bleibt am Boden."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.5, "pose": "lifted-key", "ease": "smooth"}, {"t": 3.5, "pose": "tilted-key", "ease": "smooth"}, {"t": 8, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in PALLET_CONTACTS] + [{"anchor": a, "fromS": 0.3, "toS": 8} for a in GRIPS],
     "phases": [{"id": "lift", "fromS": 0, "toS": 1.5, "title": T("Lift off the pallet", "Von der Palette heben")}, {"id": "tilt", "fromS": 1.5, "toS": 3.5, "title": T("Tilt back", "Nach hinten kippen")}, {"id": "walk", "fromS": 3.5, "toS": 8, "title": T("Walk away", "Weggehen")}],
     "events": [{"t": 0.3, "kind": "contact-on", "title": T("Hands take the load", "Hände übernehmen die Last")}, {"t": 0.4, "kind": "motion", "title": T("Feet leave the pallet", "Füße verlassen die Palette")}],
     "tracks": []},
    {"id": "wash-60", "title": T("60 C cotton wash with final spin", "Baumwollwäsche 60 C mit Endschleudern"), "description": T("A 60 s loop of the installed machine in compressed time: 6 s filling, heating to 60 C while the drum tumbles back and forth, washing, 4 s pumping out, 14 s spin (drawn far slower than 1400 rpm) and the stop.", "Eine Schleife von 60 s der aufgestellten Maschine in gerafter Zeit: 6 s Einlaufen, Aufheizen auf 60 C, während die Trommel hin und her wendet, Waschen, 4 s Abpumpen, 14 s Schleudern (weit langsamer als 1400 U/min gezeichnet) und der Stillstand."), "durationS": 60, "fps": 30, "loop": True, "rootMotion": "none",
     "keys": [drum_key(0, "in-use", 0), drum_key(6, "in-use", 720), drum_key(12, "in-use", 0), drum_key(18, "in-use", 720), drum_key(24, "in-use", 0), drum_key(30, "in-use", 720), drum_key(36, "in-use", 0), drum_key(42, "spinning", 0, "in"), drum_key(56, "spinning", 7200, "out"), drum_key(60, "in-use", 7200, "linear")],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 60} for a in PALLET_CONTACTS + FEET_CONTACTS],
     "phases": [{"id": "fill", "fromS": 0, "toS": 6, "title": T("Filling", "Einlaufen")}, {"id": "heat", "fromS": 6, "toS": 30, "title": T("Heating to 60 C", "Aufheizen auf 60 C")}, {"id": "wash", "fromS": 30, "toS": 38, "title": T("Washing", "Waschen")}, {"id": "drain", "fromS": 38, "toS": 42, "title": T("Pumping out", "Abpumpen")}, {"id": "spin", "fromS": 42, "toS": 56, "title": T("Spin", "Schleudern")}, {"id": "stop", "fromS": 56, "toS": 60, "title": T("Stop", "Stillstand")}],
     "events": [{"t": 7, "kind": "operating", "title": T("Heater on", "Heizung an"), "target": "heater"}, {"t": 28, "kind": "operating", "title": T("60 C reached, heater off", "60 C erreicht, Heizung aus"), "target": "heater"}, {"t": 42, "kind": "operating", "title": T("Spin starts", "Schleudern beginnt"), "target": "motor"}],
     "tracks": [
         {"target": "operating:heater", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 6, "v": 0}, {"t": 7, "v": 1}, {"t": 28, "v": 1}, {"t": 29, "v": 0}, {"t": 60, "v": 0}]},
         {"target": "operating:motor", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": 37, "v": 0.3}, {"t": 39, "v": 0}, {"t": 42, "v": 0}, {"t": 46, "v": 1}, {"t": 55, "v": 1}, {"t": 57, "v": 0}, {"t": 59, "v": 0}, {"t": 60, "v": 0.3}]},
         {"target": "emitter:tub-vent", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 10, "v": 0}, {"t": 22, "v": 1}, {"t": 36, "v": 1}, {"t": 44, "v": 0.2}, {"t": 56, "v": 0.2}, {"t": 60, "v": 0}]},
     ]},
]


def region(rid, en, de, kind, model, mat, area, geom, sky, cap, length, src="authored-estimate", refid="ashrae-2021", **extra):
    """🔥️ A thermal region without layers."""
    d = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": mat, "areaM2": area, "characteristicLengthM": length, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": src, "reference": refid}
    d.update(extra)
    return d


AREAS = {"pallet": 1.60, "worktop": 0.41, "cabinet": 1.56, "fascia": 0.10, "door": 0.135, "back-panel": 0.51, "hoses": 0.136}
THERMAL = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "groundTempC": 18, "relativeHumidity": 0.6, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [
        {"id": "heater", "title": T("Tub heater", "Laugenheizung"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Heating 2.0 kW", "Heizen 2,0 kW")}]},
        {"id": "motor", "title": T("Drum motor", "Trommelmotor"), "levels": [{"v": 0, "title": T("Stopped", "Stillstand")}, {"v": 0.3, "title": T("Tumbling at 50 rpm", "Wenden mit 50 U/min")}, {"v": 1, "title": T("Spinning at 1400 rpm", "Schleudern mit 1400 U/min")}]},
    ],
    "regions": [
        region("pallet", "Half pallet boards and blocks", "Bretter und Klötze der Halbpalette", "surface", "passive", "pallet-softwood", AREAS["pallet"], "horizontal-plate-up", 0.0, 15840, 0.6, refid="en-iso-10456"),
        region("worktop", "Laminated worktop, scratched, ring-marked, chipped corner", "Beschichtete Arbeitsplatte, zerkratzt, mit Rändern und abgeplatzter Ecke", "surface", "passive", "melamine-chipboard-aged", AREAS["worktop"], "horizontal-plate-up", 0.0, 27600, 0.6, refid="en-312"),
        region("cabinet", "Yellowed enamelled cabinet with dent, rust and label residue, and the feet", "Vergilbtes emailliertes Gehäuse mit Delle, Rost und Labelresten, und die Füße", "heater", "operating", "enamelled-steel-aged", AREAS["cabinet"], "vertical-plate", 0.0, 2530, 0.8, operating={"key": "heater", "heatW": 45}, ratedPowerW=2200, ratedPowerBasis="electrical-input", refid="en-60335-2-7"),
        region("fascia", "Control fascia, detergent drawer and programme knob", "Bedienblende, Waschmittelschublade und Programmwahlknopf", "surface", "passive", "abs-aged", AREAS["fascia"], "vertical-plate", 0.0, 4400, 0.105, refid="incropera-2011"),
        region("door", "Porthole door: worn ring and glass bowl over the limescaled seal", "Bullaugentür: abgegriffener Ring und Glasschüssel über der verkalkten Manschette", "glazing", "operating", "door-glass", AREAS["door"], "vertical-plate", 0.0, 12600, 0.34, operating={"key": "heater", "heatW": 15}, refid="incropera-2011"),
        region("back-panel", "Galvanised back panel with the refitted transit bolts, motor behind", "Verzinkte Rückwand mit den wieder eingesetzten Transportsicherungen, Motor dahinter", "engine", "operating", "galvanised-steel", AREAS["back-panel"], "vertical-plate", 0.0, 2530, 0.71, operating={"key": "motor", "heatW": 25}, refid="class-data-note"),
        region("hoses", "Coiled and taped drain and inlet hoses with the mains cable", "Aufgerollte und angeklebte Ablauf- und Zulaufschläuche mit dem Netzkabel", "surface", "passive", "pvc-hose", AREAS["hoses"], "cylinder", 0.0, 2600, 0.036, refid="incropera-2011"),
    ],
    "emitters": [
        {"id": "tub-vent", "kind": "vent", "title": T("Warm moist air from the tub vent at the detergent drawer", "Warme feuchte Luft aus der Behälterentlüftung an der Waschmittelschublade"), "anchor": "drawer-vent", "directionDeg": 45, "diameterM": 0.02, "operatingKey": "heater", "volumeFlowM3S": 0.0004, "temperatureRiseK": 20, "vapourKgPerKg": 0.012, "source": "authored-estimate", "reference": "class-data-note"},
    ],
    "assumptions": [
        T("Reference case: a room at 20 C without sun, the second-hand store as well as the utility room.", "Referenzfall: ein Raum mit 20 C ohne Sonne, das Gebrauchtwarenkaufhaus ebenso wie der Hauswirtschaftsraum."),
        T("Stored and carried the machine is switched off and drained: every surface follows the room air.", "Gelagert und getragen ist die Maschine ausgeschaltet und entleert: jede Fläche folgt der Raumluft."),
        T("Heating at 60 C: the 2.0 kW heater warms 14 l of water by 45 K in about 22 minutes (0.73 kWh); only some 60 W reach the room through cabinet and door glass, the rest is stored in water and laundry and leaves with the drain water.", "Aufheizen auf 60 C: Die 2,0-kW-Heizung erwärmt 14 l Wasser in etwa 22 Minuten um 45 K (0,73 kWh); nur etwa 60 W erreichen über Gehäuse und Türglas den Raum, der Rest bleibt in Wasser und Wäsche und geht mit dem Abwasser."),
        T("Nameplate: 2.2 kW electrical input (2.0 kW heater, motor, pump and controls), stated once on the cabinet region; the motor releases about 25 W behind the back panel while spinning.", "Typenschild: 2,2 kW elektrische Aufnahme (2,0 kW Heizung, Motor, Pumpe und Steuerung), einmal an der Gehäuseregion angegeben; der Motor gibt beim Schleudern etwa 25 W hinter der Rückwand ab."),
        T("Aged surfaces: the white enamel has yellowed and absorbs 0.32 of the light instead of about 0.25 when new, the chrome-look door ring is worn to emissivity 0.5 instead of about 0.1 of fresh metallisation.", "Gealterte Flächen: Der weiße Lack ist vergilbt und absorbiert 0,32 des Lichts statt etwa 0,25 im Neuzustand, der Türring in Chromoptik ist auf einen Emissionsgrad von 0,5 statt etwa 0,1 einer frischen Metallisierung abgegriffen."),
    ],
}

STRUCTURE = {
    "gravityMS2": 9.81,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T("Each pose names its supports: the three bottom boards of the pallet as floor contacts, the four hands of the two carriers when carried, the four screw feet when installed; the solver treats the asset as one rigid body, so the pallet left on the floor shares the supports of the carried or installed machine in proportion to stiffness, not as a separate body.", "Jede Pose nennt ihre Auflager: die drei Bodenbretter der Palette als Bodenkontakte, die vier Hände der zwei Träger beim Tragen, die vier Schraubfüße nach dem Aufstellen; der Löser behandelt das Asset als einen starren Körper, die am Boden gebliebene Palette teilt daher die Auflager der getragenen oder aufgestellten Maschine nach Steifigkeit, nicht als eigener Körper."),
        T(f"Masses: half pallet {PALLET_KG:.1f} kg from 0.0229 m3 of softwood at 450 kg/m3; machine 72 kg (class data), of which the drum with spider and shaft 6.5 kg and the cabinet 65.5 kg: shell 0.7 mm steel about 11 kg, worktop 5.5 kg, two concrete counterweights 25 kg, polypropylene tub, motor, pump, heater, door and fascia 24 kg.", f"Massen: Halbpalette {PALLET_KG:.1f} kg aus 0,0229 m3 Nadelholz zu 450 kg/m3; Maschine 72 kg (Klassendaten), davon die Trommel mit Kreuz und Welle 6,5 kg und das Gehäuse 65,5 kg: Blechmantel 0,7 mm Stahl etwa 11 kg, Arbeitsplatte 5,5 kg, zwei Betongegengewichte 25 kg, Laugenbehälter aus Polypropylen, Motor, Pumpe, Heizung, Tür und Blende 24 kg."),
        T("Carried: about 36 kg per carrier, above the 25 kg reference mass of ISO 11228-1 for one person, hence two carriers with carrying straps; the drum is held by the refitted transit bolts and does not swing.", "Getragen: etwa 36 kg je Träger, über der Bezugsmasse von 25 kg nach ISO 11228-1 für eine Person, daher zwei Träger mit Tragegurten; die Trommel ist durch die wieder eingesetzten Transportsicherungen gehalten und pendelt nicht."),
        T("In use: laundry and water as a static load at the drum centre (EN 60456 rated capacity 7 kg); the dynamic out-of-balance forces of the spin are taken by springs, dampers and counterweights inside the cabinet and are not modelled.", "In Betrieb: Wäsche und Wasser als ruhende Last in der Trommelmitte (Nennfüllmenge 7 kg nach EN 60456); die dynamischen Unwuchtkräfte beim Schleudern nehmen Federn, Dämpfer und Gegengewichte im Gehäuse auf, sie sind nicht modelliert."),
    ],
    "clinical": False,
}

VALID = {50: {"minDenominator": 20, "maxDenominator": 75}, 100: {"minDenominator": 75, "maxDenominator": 150}, 200: {"minDenominator": 150, "maxDenominator": 350}, 500: {"minDenominator": 350, "maxDenominator": 1000}}
PENS = {50: {"body": 0.35, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.18}}
HATCHES = {
    50: [
        {"id": "grain", "title": T("Timber grain of the pallet boards", "Holzmaserung der Palettenbretter"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1},
        {"id": "corrugation", "title": T("Corrugation of the drain hose", "Wellung des Ablaufschlauchs"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.1},
        {"id": "limescale", "title": T("Limescale crust in the door seal", "Kalkkruste in der Türmanschette"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1},
    ],
    100: [],
    200: [],
    500: [],
}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝️ The eight decisions of a representation, each a pair (en, de)."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


REP_DECISIONS = {
    50: D(
        ("The true views of the 1:50 model: the half pallet board by board, four screw feet, the cabinet with worktop, fascia, drawer and knob, the door ring with its glass bowl, handle and hinge, the raised back panel with three transit bolts and the taped hose coil with the drain hose hook.", "Die wahren Ansichten des 1:50-Modells: die Halbpalette Brett für Brett, vier Schraubfüße, das Gehäuse mit Arbeitsplatte, Blende, Schublade und Knopf, der Türring mit Glasschüssel, Griff und Scharnier, die erhabene Rückwand mit drei Transportsicherungen und die angeklebte Schlauchrolle mit dem Haken des Ablaufschlauchs."),
        ("Side: pallet boards, blocks and stringer, feet, cabinet, back panel, bolts, coil with two tape bands, hook, worktop with chipped corner, fascia, drawer, knob, door ring, hinge, handle and glass; scratches, a dent and rust at the lower front edge. Front: the drum face with its three lifters behind the glass, the limescale ring of the seal, the tape across the door, display, buttons, pump flap, label residue, scuffs, rust at the lower right corner. Plan: worktop with cup and bottle rings, tape strip, chipped corner and scratches.", "Seite: Palettenbretter, Klötze und Längsbrett, Füße, Gehäuse, Rückwand, Sicherungen, Schlauchrolle mit zwei Klebebändern, Haken, Arbeitsplatte mit abgeplatzter Ecke, Blende, Schublade, Knopf, Türring, Scharnier, Griff und Glas; Kratzer, eine Delle und Rost an der unteren Vorderkante. Front: die Trommel mit ihren drei Mitnehmern hinter dem Glas, der Kalkring der Manschette, das Klebeband über der Tür, Anzeige, Tasten, Pumpenklappe, Labelreste, Schrammen, Rost an der unteren rechten Ecke. Draufsicht: Arbeitsplatte mit Tassen- und Flaschenrändern, Klebestreifen, abgeplatzter Ecke und Kratzern."),
        ("The far feet and the far row of blocks coincide with the near ones in the side view; the door ring is drawn on the cabinet in the side view and on the door bone in front and plan.", "Die fernen Füße und die ferne Klotzreihe fallen in der Seitenansicht mit den nahen zusammen; der Türring ist in der Seitenansicht am Gehäuse und in Front und Draufsicht am Türknochen gezeichnet."),
        ("Screws, sheet folds, the tub, springs, dampers, counterweights, motor and pump inside the closed cabinet, the cable plug and the hose ends inside the coil.", "Schrauben, Blechkanten, der Laugenbehälter, Federn, Dämpfer, Gegengewichte, Motor und Pumpe im geschlossenen Gehäuse, der Netzstecker und die Schlauchenden in der Rolle."),
        ("The 10 mm back panel and drawer front and the 24 mm hose hook stay filled; buttons, knob pointer and scratches are 0.13 mm strokes.", "Die 10-mm-Rückwand und Schubladenfront und der 24-mm-Schlauchhaken bleiben gefüllt; Tasten, Knopfmarke und Kratzer sind 0,13-mm-Striche."),
        ("Grain lines at 0.8 mm on the pallet boards, corrugation at 0.6 mm on the hose coil, stipple at 0.6 mm on the limescale ring.", "Maserung mit 0,8 mm auf den Palettenbrettern, Wellung mit 0,6 mm auf der Schlauchrolle, Punktierung mit 0,6 mm auf dem Kalkring."),
        ("Pens 0.35 / 0.25 / 0.13 mm, glazing 0.18 mm; yellowed white cabinet, light worktop and fascia, grey chrome ring, dark glass, grey timber.", "Stifte 0,35 / 0,25 / 0,13 mm, Verglasung 0,18 mm; vergilbt weißes Gehäuse, helle Arbeitsplatte und Blende, grauer Chromring, dunkles Glas, graues Holz."),
        ("The drum lifters turn in the wash clip; the glass is drawn as glazing so the drum shows through. Thermal and air flow overlays use the region of every shape.", "Die Mitnehmer drehen sich im Waschclip; das Glas ist als Verglasung gezeichnet, damit die Trommel durchscheint. Thermische und Luftströmungsüberlagerungen nutzen die Region jeder Form."),
    ),
    100: D(
        ("The true views of the 1:100 model: pallet runners, feet, the cabinet to the back panel, the fascia with the drawer as an outline, knob, door ring, glass, handle, hinge, transit bolts and the hose coil.", "Die wahren Ansichten des 1:100-Modells: Palettenkufen, Füße, das Gehäuse bis zur Rückwand, die Blende mit der Schublade als Umriss, Knopf, Türring, Glas, Griff, Scharnier, Transportsicherungen und die Schlauchrolle."),
        ("Top and bottom boards, runners, feet, worktop, fascia, knob, door with glass and the drum behind it, bolts, coil with one tape band; the tape across the door, the limescale ring, the label residue, cup ring and worktop tape.", "Deck- und Bodenbretter, Kufen, Füße, Arbeitsplatte, Blende, Knopf, Tür mit Glas und der Trommel dahinter, Sicherungen, Schlauchrolle mit einem Klebeband; das Klebeband über der Tür, der Kalkring, die Labelreste, ein Tassenrand und das Klebeband auf der Platte."),
        ("Stringer and blocks of each pallet row as one notched runner; the 10 mm back panel in the cabinet; the drawer front in the fascia.", "Längsbrett und Klötze jeder Palettenreihe als eine gekerbte Kufe; die 10-mm-Rückwand im Gehäuse; die Schubladenfront in der Blende."),
        ("Hose hook, scratches, dent, rust, buttons, display texture strokes and the hatches (below 15 mm or too fine).", "Schlauchhaken, Kratzer, Delle, Rost, Tasten, Anzeigenstriche und die Schraffuren (unter 15 mm oder zu fein)."),
        ("The 20 mm feet and bolt heads (0.2 mm) are the smallest filled parts.", "Die 20-mm-Füße und Bolzenköpfe (0,2 mm) sind die kleinsten gefüllten Formen."),
        ("No hatch at this size.", "Keine Schraffur in dieser Größe."),
        ("Pens 0.25 / 0.18 / 0.1 mm, glazing 0.13 mm, tones as at 1:50.", "Stifte 0,25 / 0,18 / 0,1 mm, Verglasung 0,13 mm, Tonwerte wie bei 1:50."),
        ("As at 1:50.", "Wie bei 1:50."),
    ),
    200: D(
        ("The true views of the 1:200 model: pallet as deck and three bearers, the machine as one block from the pallet to the worktop, a 30 mm fascia band, the door as one 40 mm disc and the hose coil.", "Die wahren Ansichten des 1:200-Modells: Palette als Deck und drei Kufen, die Maschine als ein Block von der Palette bis zur Arbeitsplatte, ein 30-mm-Blendenband, die Tür als eine 40-mm-Scheibe und die Schlauchrolle."),
        ("Pallet height and bearers, the cabinet block, fascia band, door disc with the glass as a stroke, coil at the back, the worktop joint as a stroke.", "Palettenhöhe und Kufen, der Gehäuseblock, Blendenband, Türscheibe mit dem Glas als Strich, Rolle hinten, die Fuge der Arbeitsplatte als Strich."),
        ("Top boards and stringers as one deck, blocks and bottom boards as one bearer per row; feet, shell, back panel and worktop as one block carrying their regions; drawer and knob in the fascia band; ring, glass, handle and hinge in the door disc.", "Deckbretter und Längsbretter als ein Deck, Klötze und Bodenbretter als eine Kufe je Reihe; Füße, Mantel, Rückwand und Arbeitsplatte als ein Block mit ihren Regionen; Schublade und Knopf im Blendenband; Ring, Glas, Griff und Scharnier in der Türscheibe."),
        ("Transit bolts, hose hook, feet, every trace of use (below 30 mm).", "Transportsicherungen, Schlauchhaken, Füße, jede Gebrauchsspur (unter 30 mm)."),
        ("The 30 mm fascia band (0.15 mm plus pen) is the thinnest filled part.", "Das 30-mm-Blendenband (0,15 mm plus Stift) ist die dünnste gefüllte Form."),
        ("No hatch.", "Keine Schraffur."),
        ("Pens 0.18 / 0.13 mm.", "Stifte 0,18 / 0,13 mm."),
        ("Overlays colour the cabinet block with the area-weighted value of its merged regions (labelled merged).", "Überlagerungen färben den Gehäuseblock mit dem flächengewichteten Wert seiner zusammengefassten Regionen (als zusammengefasst gekennzeichnet)."),
    ),
    500: D(
        ("Pallet as one slab and the machine as one block of 0.55 x 0.85 m; the door as one circle stroke on the front.", "Palette als eine Platte und die Maschine als ein Block von 0,55 x 0,85 m; die Tür als ein Kreisstrich auf der Front."),
        ("Overall height 0.994 m, pallet 0.80 x 0.60 m, the 60 x 60 cm machine footprint and the door side.", "Gesamthöhe 0,994 m, Palette 0,80 x 0,60 m, die Stellfläche der Maschine von 60 x 60 cm und die Türseite."),
        ("All machine regions in one block, the pallet in one slab.", "Alle Maschinenregionen in einem Block, die Palette in einer Platte."),
        ("Door disc, fascia band, hose coil and every detail below 75 mm.", "Türscheibe, Blendenband, Schlauchrolle und jede Einzelheit unter 75 mm."),
        ("The 144 mm pallet (0.29 mm) is the thinnest filled part.", "Die 144-mm-Palette (0,29 mm) ist die dünnste gefüllte Form."),
        ("No hatch.", "Keine Schraffur."),
        ("Pens 0.13 / 0.1 mm.", "Stifte 0,13 / 0,1 mm."),
        ("Overlays colour the machine block with the area-weighted value of its merged regions (labelled merged).", "Überlagerungen färben den Maschinenblock mit dem flächengewichteten Wert seiner zusammengefassten Regionen (als zusammengefasst gekennzeichnet)."),
    ),
}

VOLUME_DECISIONS = {
    50: [
        T("Body: the EPAL half pallet board by board (three bottom boards, nine blocks, three stringer boards, five top boards), four 36 mm screw feet, the cabinet shell with 6 mm rounded edges, the 10 mm raised back panel, three transit bolt heads, the hose coil as a 36 x 40 mm ring of 0.38 m with the drain hose hook as a 24 mm capsule, the 25 mm worktop, fascia, drawer front, knob, the door ring with its 10 mm chamfer, the 36 mm glass bowl, handle and hinge, and the drum inside.", "Körper: die EPAL-Halbpalette Brett für Brett (drei Bodenbretter, neun Klötze, drei Längsbretter, fünf Deckbretter), vier 36-mm-Schraubfüße, der Gehäusemantel mit 6 mm gerundeten Kanten, die 10 mm erhabene Rückwand, drei Köpfe der Transportsicherungen, die Schlauchrolle als 36 x 40 mm starker Ring von 0,38 m mit dem Haken des Ablaufschlauchs als 24-mm-Kapsel, die 25-mm-Arbeitsplatte, Blende, Schubladenfront, Knopf, der Türring mit 10 mm Fase, die 36 mm gewölbte Glasschüssel, Griff und Scharnier und die Trommel im Inneren."),
        T("Thermal: every outer body solid by reference in its region (the hidden drum is no surface), plus the 8 mm outlet of the tub vent on the drawer.", "Thermisch: jeder äußere Körper per Verweis in seiner Region (die verborgene Trommel ist keine Fläche), dazu der 8-mm-Auslass der Behälterentlüftung an der Schublade."),
        T("Structure: pallet boards and blocks as pallet members, cabinet shell and worktop as the cabinet member, the four feet as its joints, the drum as member with its rear bearing as joint, three mass markers.", "Tragwerk: Palettenbretter und Klötze als Palettenglieder, Gehäusemantel und Arbeitsplatte als Gehäuseglied, die vier Füße als seine Gelenke, die Trommel als Glied mit ihrem hinteren Lager als Gelenk, drei Massenmarken."),
    ],
    100: [
        T("Body: stringer and three blocks of each pallet row as one notched runner, boards kept (22 mm); the back panel merged into the cabinet shell, the drawer front into the fascia, the hose hook left out; feet, bolts, knob, door ring, glass, handle and hinge kept at 15 mm and more.", "Körper: Längsbrett und drei Klötze jeder Palettenreihe als eine gekerbte Kufe, Bretter behalten (22 mm); die Rückwand im Gehäusemantel, die Schubladenfront in der Blende aufgegangen, der Schlauchhaken entfällt; Füße, Sicherungen, Knopf, Türring, Glas, Griff und Scharnier ab 15 mm behalten."),
        T("Thermal: references in the regions and a 15 mm vent outlet.", "Thermisch: Verweise in den Regionen und ein 15-mm-Entlüftungsauslass."),
        T("Structure: runners and boards as pallet members, shell and worktop, feet as joints, drum and bearing, three mass markers.", "Tragwerk: Kufen und Bretter als Palettenglieder, Mantel und Arbeitsplatte, Füße als Gelenke, Trommel und Lager, drei Massenmarken."),
    ],
    200: [
        T("Body: pallet as a 44 mm deck and three 100 mm bearers, the machine as one block from the pallet top to the worktop (feet, shell, back panel and worktop merged), a 30 mm fascia band, the door as a 40 mm disc of 0.34 m, the hose coil and the drum; bolts, hook, knob, handle and hinge left out.", "Körper: Palette als 44-mm-Deck und drei 100-mm-Kufen, die Maschine als ein Block von der Palettenoberseite bis zur Arbeitsplatte (Füße, Mantel, Rückwand und Arbeitsplatte zusammengefasst), ein 30-mm-Blendenband, die Tür als 40-mm-Scheibe von 0,34 m, die Schlauchrolle und die Trommel; Sicherungen, Haken, Knopf, Griff und Scharnier entfallen."),
        T("Thermal: the merged block split into its region solids (back panel 30 mm, cabinet, worktop 30 mm), fascia, door and coil by reference, a 30 mm vent outlet.", "Thermisch: der zusammengefasste Block in seine Regionskörper geteilt (Rückwand 30 mm, Gehäuse, Arbeitsplatte 30 mm), Blende, Tür und Rolle per Verweis, ein 30-mm-Entlüftungsauslass."),
        T("Structure: deck and bearers, the cabinet block as member resting on the deck, drum and bearing, three mass markers.", "Tragwerk: Deck und Kufen, der Gehäuseblock als Glied auf dem Deck, Trommel und Lager, drei Massenmarken."),
    ],
    500: [
        T("Body: the pallet as one 144 mm slab and the machine as one block from the pallet to the top of the worktop, with the drum inside.", "Körper: die Palette als eine 144-mm-Platte und die Maschine als ein Block von der Palette bis zur Oberkante der Arbeitsplatte, mit der Trommel im Inneren."),
        T("Thermal: the pallet by reference, the machine as 75 mm region solids (back panel, cabinet, cabinet front, fascia, worktop), the door as a 75 mm disc and the hoses as a 75 mm block on the back, a 75 mm vent outlet.", "Thermisch: die Palette per Verweis, die Maschine als 75-mm-Regionskörper (Rückwand, Gehäuse, Gehäusefront, Blende, Arbeitsplatte), die Tür als 75-mm-Scheibe und die Schläuche als 75-mm-Block auf der Rückseite, ein 75-mm-Entlüftungsauslass."),
        T("Structure: slab, the machine block as member, drum and bearing, three mass markers.", "Tragwerk: Platte, der Maschinenblock als Glied, Trommel und Lager, drei Massenmarken."),
    ],
}
