"""Assembles and writes heat-pump-outdoor-unit."""
from hpump import *

EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
materials = [material("powder-coat", "Powder-coated galvanised steel (light grey)", "Pulverbeschichteter verzinkter Stahl (hellgrau)", 0.9, 0.4, 7850, 50, 460, "literature", "ashrae-2021"), MAT_ALU, MAT_GALV, MAT_RUBBER]
R = region
regions = [
    R("casing", "Casing panels", "Gehäuseverkleidung", "surface", "passive", "powder-coat", 3.4, 1.4, "vertical-plate", 0.5, [steel_layer("panel", 0.0012, "powder-coat")], 4300, ref="ashrae-2021"),
    R("coil-guard", "Coil face and louvred guard", "Wärmetauscherfläche und Lamellenschutz", "surface", "operating", "aluminium", 1.6, 1.0, "vertical-plate", 0.5, [layer("fins", "Aluminium fins (smeared), 0.4 mm", "Aluminiumlamellen (verschmiert), 0,4 mm", "metal", "aluminium", 0.0004, 2e-6, 970, None, "literature", "incropera-2011")], 970, op=("refrigerant-duty", -1800), ref="en-14511"),
    R("compressor-casing", "Compressor compartment panel", "Verdichterraum-Verkleidung", "engine", "operating", "powder-coat", 1.0, 0.25, "vertical-plate", 0.4, [steel_layer("panel", 0.0012, "powder-coat")], 4300, op=("compressor", 700)),
    R("base-tray", "Base tray with defrost heating", "Bodenwanne mit Abtauheizung", "heater", "operating", "galvanised-steel", 0.5, 0.1, "horizontal-plate-down", 0.2, [steel_layer("tray", 0.0015, "galvanised-steel")], 5400, op=("defrost", 2500)),
    R("fan-guard", "Fan outlet grille and bell mouth", "Lüfter-Ausblasgitter und Düse", "surface", "passive", "galvanised-steel", 0.6, 0.5, "vertical-plate", 0.5, [steel_layer("wire", 0.004, "galvanised-steel")], 1000, ref="ashrae-2021"),
    R("top-lid", "Service lid", "Wartungsdeckel", "surface", "passive", "powder-coat", 0.5, 0.45, "horizontal-plate-up", 1.0, [steel_layer("lid", 0.0012, "powder-coat")], 4300, ref="ashrae-2021"),
    R("discharge-vane", "Discharge guide vane", "Luftleitlamelle", "surface", "passive", "powder-coat", 0.25, 0.22, "vertical-plate", 0.6, [steel_layer("vane", 0.002, "powder-coat")], 7200),
    R("feet", "Rubber feet", "Gummifüße", "tyre", "passive", "rubber", 0.1, 0.1, "vertical-plate", 0.2, [layer("rubber", "Rubber 30 mm", "Gummi 30 mm", "polymer", "rubber", 0.03, 0.2, 56000, None, "literature", "incropera-2011")], 56000),
]
RID = [r["id"] for r in regions]
DEF = {"casing": {"exposedFraction": 1, "sunlitFraction": 0.5}, "coil-guard": {"exposedFraction": 1, "sunlitFraction": 0.2}, "compressor-casing": {"exposedFraction": 1, "sunlitFraction": 0.3},
       "base-tray": {"exposedFraction": 0.3, "sunlitFraction": 0}, "fan-guard": {"exposedFraction": 1, "sunlitFraction": 0.5}, "top-lid": {"exposedFraction": 1, "sunlitFraction": 1.0},
       "discharge-vane": {"exposedFraction": 1, "sunlitFraction": 0.5}, "feet": {"exposedFraction": 0.3, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 25}}}
ops = [opkey("compressor", "Compressor modulation", "Verdichtermodulation", [(0, "Off", "Aus"), (0.35, "Minimum modulation", "Minimale Modulation"), (0.7, "Part load", "Teillast"), (1, "Rated (A7/W35 12 kW)", "Nennlast (A7/W35 12 kW)")]),
       opkey("fan-speed", "Fan speed", "Lüfterdrehzahl", [(0, "Stopped", "Stillstand"), (0.4, "Low", "Niedrig"), (0.7, "Medium", "Mittel"), (1, "High", "Hoch")]),
       opkey("refrigerant-duty", "Coil duty (positive evaporating, negative condensing)", "Wärmetauscher-Betrieb (positiv verdampfend, negativ verflüssigend)", [(-1, "Condensing: coil rejects heat", "Verflüssigend: Tauscher gibt Wärme ab"), (0, "Off", "Aus"), (0.7, "Evaporating, part load", "Verdampfend, Teillast"), (1, "Evaporating, rated", "Verdampfend, Nennlast")]),
       opkey("defrost", "Defrost", "Abtauen", [(0, "Off", "Aus"), (1, "Defrosting", "Abtauend")])]
emitters = [emitter("discharge-heating", "fan", "Cooled fan discharge in heating mode", "Abgekühlte Lüfterluft im Heizbetrieb", "fan-outlet", 0, 0.5, "fan-speed", 1.53, -5.1, -0.0008, "en-14511"),
            emitter("discharge-cooling", "fan", "Warmed fan discharge in cooling mode", "Erwärmte Lüfterluft im Kühlbetrieb", "fan-outlet", 0, 0.5, "fan-speed", 1.53, 7.0, 0.0, "en-14511"),
            emitter("defrost-vapour", "vent", "Vapour rising from the coil during defrost", "Dampf vom Wärmetauscher beim Abtauen", "coil-top", 100, 0.4, "defrost", 0.25, 3.0, 0.008)]
anchors = [anchor("contact-r", "unit", 0.13, 0.0, "contact", "Front foot contact", "Auflager vorderer Fuß", "contact", 0.08, 0.45, 0.6, 90), anchor("contact-l", "unit", -0.13, 0.0, "contact", "Rear foot contact", "Auflager hinterer Fuß", "contact", 0.08, 0.45, 0.6, 90),
           anchor("fan-outlet", "unit", FX, 0.8, "emitter", "Fan outlet centre", "Mitte des Lüfterausblases"), anchor("coil-top", "unit", -0.05, 1.3, "emitter", "Top of the coil", "Oberseite des Wärmetauschers")]
CONTACTS = ["contact-r", "contact-l"]


def th(op, em, over=None):
    return tbind(RID, DEF, over or {}, em, op)


E0 = {"discharge-heating": 0, "discharge-cooling": 0, "defrost-vapour": 0}
poses = [
    pose("heating", "Heating operation", "Heizbetrieb", "Part-load heating at an outdoor air of 7 C: the fan blows air cooled by about 5 K, the vane points slightly down.", "Teillast-Heizbetrieb bei 7 C Außenluft: Der Lüfter bläst um etwa 5 K abgekühlte Luft aus, die Lamelle zeigt leicht nach unten.", {"vane": {"angleDeg": -12}}, CONTACTS,
         th({"compressor": 0.7, "fan-speed": 0.7, "refrigerant-duty": 0.7, "defrost": 0}, {**E0, "discharge-heating": 1})),
    pose("cooling", "Cooling operation", "Kühlbetrieb", "Part-load cooling at 35 C: the coil rejects heat and the fan blows air warmed by about 7 K; the vane points slightly up.", "Teillast-Kühlbetrieb bei 35 C: Der Tauscher gibt Wärme ab, der Lüfter bläst um etwa 7 K erwärmte Luft aus; die Lamelle zeigt leicht nach oben.", {"vane": {"angleDeg": 12}}, CONTACTS,
         th({"compressor": 0.7, "fan-speed": 0.7, "refrigerant-duty": -0.7, "defrost": 0}, {**E0, "discharge-cooling": 1}, {"coil-guard": {"exposedFraction": 1, "sunlitFraction": 0.2}})),
    pose("defrosting", "Defrost cycle", "Abtauzyklus", "Fan stopped, hot gas through the coil and the tray heating melting the frost: vapour rises from the coil.", "Lüfter steht, Heißgas im Wärmetauscher und Wannenheizung tauen den Reif ab: Dampf steigt vom Tauscher auf.", {"vane": {"angleDeg": 0}}, CONTACTS,
         th({"compressor": 0.6, "fan-speed": 0, "refrigerant-duty": -0.8, "defrost": 1}, {**E0, "defrost-vapour": 1}, {"base-tray": {"exposedFraction": 0.3, "sunlitFraction": 0, "wettedness": 0.8}})),
    pose("off", "Off, lid closed", "Aus, Deckel geschlossen", "Standby without operation, everything at ambient.", "Bereitschaft ohne Betrieb, alles auf Umgebungstemperatur.", {}, CONTACTS, th({"compressor": 0, "fan-speed": 0, "refrigerant-duty": 0, "defrost": 0}, E0)),
    pose("serviced", "Serviced, lid open", "Gewartet, Deckel offen", "Off with the service lid lifted 80 degrees for access to the electronics.", "Aus mit um 80 Grad gehobenem Wartungsdeckel für den Zugang zur Elektronik.", {"lid": {"angleDeg": 80}}, CONTACTS, th({"compressor": 0, "fan-speed": 0, "refrigerant-duty": 0, "defrost": 0}, E0)),
]
K = lambda t, p, ease="smooth", b=None: key(t, p, ease, b)
sweep = [K(0, "heating", "smooth", {"vane": {"angleDeg": -15}}), K(5, "heating", "smooth", {"vane": {"angleDeg": 15}}), K(10, "heating", "smooth", {"vane": {"angleDeg": -15}}), K(15, "heating", "smooth", {"vane": {"angleDeg": 15}}), K(20, "heating", "smooth", {"vane": {"angleDeg": -15}})]
clips = [
    clip("heating-sweep", "Heating with swinging vane", "Heizbetrieb mit schwingender Lamelle", "A 20 s loop of heating operation: the guide vane swings between 15 degrees down and 15 degrees up while the compressor modulates between 60 and 80 percent; the cooled air keeps leaving the fan.",
         "Eine Schleife von 20 s im Heizbetrieb: Die Leitlamelle schwingt zwischen 15 Grad unten und 15 Grad oben, während der Verdichter zwischen 60 und 80 Prozent moduliert; die abgekühlte Luft strömt weiter aus dem Lüfter.", 20, 30, True,
         [K(0, "heating", "smooth", {"vane": {"angleDeg": -15}}), K(10, "heating", "smooth", {"vane": {"angleDeg": 15}}), K(20, "heating", "smooth", {"vane": {"angleDeg": -15}})],
         [{"anchor": a, "fromS": 0, "toS": 20} for a in CONTACTS], [phase("sweep-up", 0, 10, "Vane swings up", "Lamelle schwingt nach oben"), phase("sweep-down", 10, 20, "Vane swings down", "Lamelle schwingt nach unten")],
         [event(0, "operating", "Heating at part load", "Heizen in Teillast", "compressor")],
         [track("operating:compressor", [(0, 0.6), (10, 0.8), (20, 0.6)]), track("operating:refrigerant-duty", [(0, 0.6), (10, 0.8), (20, 0.6)]), track("operating:fan-speed", [(0, 0.7), (20, 0.7)], "linear"), track("emitter:discharge-heating", [(0, 1)], "step")]),
    clip("defrost-sequence", "Start, defrost and shutdown", "Anlaufen, Abtauen und Abschalten", "A 30 s sequence: compressor starts, heating runs, the fan stops and the defrost cycle melts the frost under rising vapour, heating resumes, the unit shuts down and the lid is lifted for service.",
         "Eine Folge von 30 s: Der Verdichter startet, Heizbetrieb läuft, der Lüfter stoppt und der Abtauzyklus taut den Reif unter aufsteigendem Dampf ab, der Heizbetrieb setzt wieder ein, das Gerät schaltet ab und der Deckel wird zur Wartung gehoben.", 30, 30, False,
         [K(0, "off"), K(1, "off"), K(3, "heating"), K(10, "heating"), K(11, "defrosting"), K(17, "defrosting"), K(18.5, "heating"), K(25, "heating"), K(26, "off"), K(28, "serviced"), K(30, "serviced")],
         [{"anchor": a, "fromS": 0, "toS": 30} for a in CONTACTS],
         [phase("start", 0, 3, "Compressor start", "Verdichterstart"), phase("heat", 3, 11, "Heating", "Heizen"), phase("defrost", 11, 18, "Defrost", "Abtauen"), phase("heat-2", 18, 26, "Heating again", "Wieder heizen"), phase("service", 26, 30, "Shutdown and service", "Abschalten und Wartung")],
         [event(1.0, "ignition", "Compressor starts", "Verdichter startet", "discharge-heating"), event(11.0, "thermal", "Reversing valve switches to defrost", "Umschaltventil schaltet auf Abtauen", "defrost-vapour"), event(26.0, "shutdown", "Unit switches off", "Gerät schaltet ab", "discharge-heating")],
         [track("operating:compressor", [(0, 0), (1, 0), (2, 0.7), (10, 0.7), (11, 0.6), (17, 0.6), (18.5, 0.7), (25, 0.7), (26, 0)]), track("operating:fan-speed", [(0, 0), (1.5, 0), (3, 0.7), (10.5, 0.7), (11, 0), (18, 0), (18.5, 0.7), (25.8, 0.7), (26, 0)]),
          track("operating:refrigerant-duty", [(0, 0), (1, 0), (2, 0.7), (10.5, 0.7), (11, -0.8), (18, -0.8), (18.5, 0.7), (25.8, 0.7), (26, 0)]), track("operating:defrost", [(0, 0), (10.9, 0), (11, 1), (18, 1), (18.1, 0)], "step"),
          track("emitter:discharge-heating", [(0, 0), (1.5, 1), (10.8, 1), (11, 0), (18.4, 1), (26, 0)], "step"), track("emitter:defrost-vapour", [(0, 0), (11.5, 1), (18, 0)], "step"), track("region:base-tray:wettedness", [(0, 0), (11, 0), (12, 0.8), (18, 0.5), (22, 0)])]),
]


def build_reps():
    r = {}
    nb = tx
    r["500"] = rep(500, 350, 1000, pens(0.15, None, None, 0.13, 0.25), [], shapes(500), [
        nb("One composed casing silhouette with the fan bulge on the front, the service lid on top and the air-guide vane on the discharge side.", "Eine zusammengesetzte Gehäusesilhouette mit der Lüfterwölbung vorn, dem Wartungsdeckel oben und der Luftleitlamelle an der Ausblasseite."),
        nb("The 0.45 x 1.45 m outline, the fan bulge, lid and vane (they move).", "Der Umriss von 0,45 x 1,45 m, die Lüfterwölbung, Deckel und Lamelle (sie bewegen sich)."),
        nb("Casing, coil guard, compressor panel, tray, fan grille and feet are one form.", "Gehäuse, Tauscherschutz, Verdichterpaneel, Wanne, Lüftergitter und Füße sind eine Form."),
        nb("Louvres, panel lines, feet notches, handle.", "Lamellen, Paneelfugen, Fußaussparungen, Griff."),
        nb("Lid (real 0.04 m) and vane (real 0.05 m) are drawn 0.1 and 0.12 m thick (0.2 and 0.24 mm), the least that prints; the lid thickening lowers the casing top by 0.06 m to keep the 1.45 m height.", "Deckel (real 0,04 m) und Lamelle (real 0,05 m) sind 0,1 und 0,12 m dick (0,2 und 0,24 mm) gezeichnet, das Mindeste, das druckt; die Deckelverdickung senkt die Gehäuseoberkante um 0,06 m, um die Höhe von 1,45 m zu halten."),
        nb("None.", "Keine."),
        nb("Casing mid-tone, lid and vane dark; pen 0.15 mm.", "Gehäuse mittlerer Ton, Deckel und Lamelle dunkel; Stift 0,15 mm."),
        nb("The casing form carries six regions (merged); lid and vane have their own regions.", "Die Gehäuseform trägt sechs Regionen (zusammengefasst); Deckel und Lamelle haben eigene Regionen."),
    ])
    r["200"] = rep(200, 150, 350, pens(0.18, 0.13, None, 0.13, 0.25), [], shapes(200), [
        nb("Casing down to the ground, compressor panel, coil guard field, fan bulge, base tray, lid and vane.", "Gehäuse bis zum Boden, Verdichterpaneel, Tauscherschutzfeld, Lüfterwölbung, Wanne, Deckel und Lamelle."),
        nb("The coil guard field and the compressor panel.", "Das Tauscherschutzfeld und das Verdichterpaneel."),
        nb("Feet merge into the casing at 1:200 (notches would be 0.28 mm).", "Die Füße gehen bei 1:200 im Gehäuse auf (Aussparungen wären 0,28 mm)."),
        nb("Louvres, panel line, handle.", "Lamellen, Paneelfuge, Griff."),
        nb("Lid 0.05 m (0.25 mm) and vane 0.06 m (0.3 mm) at about real thickness.", "Deckel 0,05 m (0,25 mm) und Lamelle 0,06 m (0,3 mm) in etwa Realdicke."),
        nb("None.", "Keine."),
        nb("Coil guard darkest, compressor panel mid-dark, casing mid-tone.", "Tauscherschutz am dunkelsten, Verdichterpaneel mitteldunkel, Gehäuse mittlerer Ton."),
        nb("Every region has its own form except the feet (merged into the casing).", "Jede Region hat eine eigene Form außer den Füßen (im Gehäuse zusammengefasst)."),
    ])
    hat = [hatch("fins", "Coil fins and guard louvres", "Tauscherlamellen und Schutzlamellen", "lines", 0, 0.4, 0.1)]
    r["100"] = rep(100, 75, 150, pens(0.25, 0.18, None, 0.13, 0.35), hat, shapes(100), [
        nb("As 1:200 with separate rubber feet, a service panel outline and louvre hatching on the coil guard.", "Wie 1:200 mit getrennten Gummifüßen, einem Wartungspaneelumriss und Lamellenschraffur auf dem Tauscherschutz."),
        nb("Feet with notches, service panel line, louvre hatch.", "Füße mit Aussparungen, Wartungspaneelfuge, Lamellenschraffur."),
        nb("Handle merged.", "Griff zusammengefasst."),
        nb("Handle, screws, nameplate.", "Griff, Schrauben, Typenschild."),
        nb("Lid 0.04 m (0.4 mm).", "Deckel 0,04 m (0,4 mm)."),
        nb("Louvre hatch at 0.4 mm spacing, pen 0.1 mm.", "Lamellenschraffur mit 0,4 mm Abstand, Stift 0,1 mm."),
        nb("As 1:200, detail pen 0.18 mm.", "Wie 1:200, Detailstift 0,18 mm."),
        nb("As 1:200; feet carry the feet region.", "Wie 1:200; Füße tragen die Fußregion."),
    ])
    r["50"] = rep(50, 20, 75, pens(0.35, 0.25, 0.13, 0.18, 0.5), hat, shapes(50), [
        nb("All 1:100 parts plus the panel handle.", "Alle 1:100-Teile plus der Paneelgriff."),
        nb("Handle crossing the service panel line.", "Griff über der Wartungspaneelfuge."),
        nb("Nothing merged.", "Nichts zusammengefasst."),
        nb("Screws, nameplate, fan blade edges (not visible from the side).", "Schrauben, Typenschild, Lüfterblattkanten (von der Seite nicht sichtbar)."),
        nb("Lid 0.04 m (0.8 mm).", "Deckel 0,04 m (0,8 mm)."),
        nb("Louvre hatch at 0.4 mm spacing.", "Lamellenschraffur mit 0,4 mm Abstand."),
        nb("Heavier pens 0.35 / 0.25 mm.", "Stärkere Stifte 0,35 / 0,25 mm."),
        nb("As 1:100.", "Wie 1:100."),
    ])
    return r


def symbols():
    shapes_p = [G.rect("footprint", "unit", "body", 1, -0.55, -0.225, 1.1, 0.45, tone=0.6), G.ell("fan-circle", "unit", "detail", 2, 0.22, 0.0, 0.2, 0.2, filled=False), G.poly("flow-arrow", "unit", "detail", 3, [(0.45, 0.0), (0.85, 0.0)], closed=False, filled=False)]
    return [{"id": "site-plan-footprint", "title": tx("Site-plan footprint with fan and air flow", "Baustellenplan-Grundriss mit Lüfter und Luftstrom"), "description": tx("Plan symbol of the 1.10 x 0.45 m unit: the footprint, the fan circle and the discharge air arrow.", "Planzeichen des Geräts von 1,10 x 0,45 m: Grundriss, Lüfterkreis und Pfeil des Ausblasluftstroms."),
             "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": pens(0.25, 0.18, None, None, 0.25), "hatches": [], "geometry": {"elevation": [], "plan": shapes_p},
             "dimensionalNote": tx("True extent in model metres: 1.10 x 0.45 m footprint, fan circle 0.4 m, arrow 0.4 m long showing the discharge direction; real size at both listed scales.", "Wahre Ausdehnung in Modellmetern: Grundriss 1,10 x 0,45 m, Lüfterkreis 0,4 m, Pfeil 0,4 m lang für die Ausblasrichtung; in beiden gelisteten Maßstäben in Realgröße.")}]


def asset():
    a = {
        "id": "heat-pump-outdoor-unit", "category": "machinery", "title": tx("Air-source heat pump outdoor unit, 12 kW", "Luft-Wasser-Wärmepumpe Außengerät, 12 kW"),
        "summary": tx("A 12 kW air-source heat pump outdoor unit in side elevation: the louvred coil guard, front fan outlet with an adjustable guide vane and a hinged service lid. In heating mode the fan blows air cooled by 5 K, in cooling mode air warmed by 7 K; a defrost cycle releases vapour from the coil.",
                      "Ein Luft-Wasser-Wärmepumpen-Außengerät von 12 kW in der Seitenansicht: Lamellenschutz des Wärmetauschers, vorderer Lüfterausblas mit verstellbarer Leitlamelle und angeschlagener Wartungsdeckel. Im Heizbetrieb bläst der Lüfter um 5 K abgekühlte Luft, im Kühlbetrieb um 7 K erwärmte; ein Abtauzyklus setzt Dampf vom Wärmetauscher frei."),
        "tags": ["machine", "heat-pump", "outdoor-unit", "air-source", "hvac", "fan", "defrost"],
        "provenance": {"author": "Semio staffage content worker NW-MA", "method": "handcrafted", "created": "2026-10-29",
                       "references": references(["en-14511", "incropera-2011", "ashrae-2021", "class-data-note"]),
                       "notes": tx("Class data of 10 to 12 kW monobloc air-source heat pump outdoor units (about 1.10 m wide, 0.45 m deep, 1.45 m high, 150 kg, 5500 m3/h fan flow). Air flow and temperature change follow the EN 14511 test points A7/W35 heating (12 kW, COP 4.5, electric input 2.7 kW) and A35/W7 cooling (10 kW, EER 3.3, 3.0 kW input); all numbers are authored estimates of that class. Fan blade rotation is not visible in the side elevation.",
                                   "Klassendaten von 10- bis 12-kW-Monoblock-Luft-Wasser-Wärmepumpen-Außengeräten (etwa 1,10 m breit, 0,45 m tief, 1,45 m hoch, 150 kg, Lüfterluftstrom 5500 m3/h). Luftstrom und Temperaturänderung folgen den Prüfpunkten nach EN 14511 A7/W35 Heizen (12 kW, COP 4,5, elektrische Aufnahme 2,7 kW) und A35/W7 Kühlen (10 kW, EER 3,3, Aufnahme 3,0 kW); alle Zahlen sind Schätzungen dieser Klasse. Die Lüfterdrehung ist in der Seitenansicht nicht sichtbar.")},
        "dimensions": {"heightM": 1.45, "widthM": 0, "depthM": 1.10, "massKg": 150},
        "materials": materials, "rig": rig.doc(), "anchors": anchors, "representations": build_reps(), "symbols": symbols(),
        "poses": poses, "clips": clips,
        "thermal": {"environment": {**DEFAULT_ENV, "airTempC": 7, "meanRadiantTempC": 7, "skyTempC": -3, "groundTempC": 8, "relativeHumidity": 0.87, "solarWM2": 200, "windMS": 1.0},
                    "passengers": [], "operating": ops, "regions": regions, "emitters": emitters,
                    "assumptions": [tx("Heating, EN 14511 A7/W35: 12 kW heat output with 2.7 kW electric input extracts 9.3 kW from the air; at 5500 m3/h (1.53 m3/s, rho cp V = 1845 W/K) the air cools by 5.1 K; at 87 percent relative humidity and 7 C (w = 0.0049 kg/kg) air at 1.9 C holds 0.0041 kg/kg, hence -0.0008 kg/kg (condensate on the coil). Cooling, A35/W7: 10 kW + 3.0 kW input rejects 13 kW, +7.0 K.", "Heizen nach EN 14511 A7/W35: 12 kW Heizleistung bei 2,7 kW elektrischer Aufnahme entziehen der Luft 9,3 kW; bei 5500 m3/h (1,53 m3/s, rho cp V = 1845 W/K) kühlt die Luft um 5,1 K ab; bei 87 Prozent relativer Feuchte und 7 C (w = 0,0049 kg/kg) enthält Luft von 1,9 C 0,0041 kg/kg, also -0,0008 kg/kg (Kondensat am Tauscher). Kühlen A35/W7: 10 kW + 3,0 kW Aufnahme führen 13 kW ab, +7,0 K."),
                                    tx("Coil duty is a signed operating key: level 1 = evaporating (heat absorbed at the coil surface, -1.8 kW through the visible guard), level -1 = condensing (+1.8 kW). The compressor compartment panel receives 0.7 kW, the tray heating 2.5 kW during defrost. The defrost vapour jet is 0.25 m3/s at +3 K with 0.008 kg/kg. All authored estimates.", "Der Tauscherbetrieb ist ein vorzeichenbehafteter Betriebsschlüssel: Stufe 1 = verdampfend (Wärmeaufnahme an der Tauscheroberfläche, -1,8 kW durch den sichtbaren Schutz), Stufe -1 = verflüssigend (+1,8 kW). Das Verdichterpaneel erhält 0,7 kW, die Wannenheizung beim Abtauen 2,5 kW. Der Abtaudampfstrahl hat 0,25 m3/s bei +3 K mit 0,008 kg/kg. Alles Schätzungen."),
                                    tx("Reference environment is a cold humid heating day (7 C, 87 percent, 200 W/m2).", "Die Referenzumgebung ist ein kalter feuchter Heiztag (7 C, 87 Prozent, 200 W/m2).")]},
        "structure": structure("Two foot contacts carry the unit as unilateral supports; the 150 kg is distributed over the bones; gravity only.", "Zwei Fußauflager tragen das Gerät als einseitige Auflager; die 150 kg verteilen sich auf die Knochen; nur Schwerkraft."),
        "presentation": {"defaultPose": "heating", "defaultClip": "heating-sweep", "defaultScale": 50, "viewpoints": ["elevation"]},
        "exports": EXPORTS, "completion": "authored",
    }
    x0, x1, y0, y1 = bbox(a, rig, 50, poses[0]["bones"])
    a["dimensions"]["widthM"] = round(x1 - x0, 2)
    a["dimensions"]["heightM"] = round(y1, 2)
    return a


if __name__ == "__main__":
    a = asset()
    for p in selfcheck(a):
        print("PROBLEM", p)
    for d in (500, 200, 100, 50):
        print(d, [round(v, 3) for v in bbox(a, rig, d, poses[0]["bones"])])
    print(a["dimensions"], write(a, "heat-pump-outdoor-unit"))
