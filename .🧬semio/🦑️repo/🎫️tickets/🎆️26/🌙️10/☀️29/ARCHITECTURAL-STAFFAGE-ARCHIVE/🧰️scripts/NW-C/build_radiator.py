import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-C")
from lib import *

WALL_X0, X_BACK, X_FRONT = -0.15, 0.035, 0.135
Y0, Y1 = 0.11, 0.71
P1, C1, C2 = 0.057, 0.085, 0.113
VALVE = (0.09, 0.085, 0.03)
WALL_TOP = 0.85

bones = [bone("wall", None, "Partition wall stub (fixed root)", "Wandscheibe (feste Wurzel)", 0, 0, 90, WALL_TOP, fixed(),
              mass(0.15 * WALL_TOP * 1.0 * 1800, WALL_TOP / 2, -0.075, "uniform", "literature", "en-iso-10456"))]
bones.append(bone_world("radiator", "wall", "Type 22 radiator body on its wall brackets", "Kompaktheizkörper Typ 22 an den Wandkonsolen", X_BACK, Y0, 90, 0.6, fixed(), bones,
                        mass(26.0, 0.3, -0.05, "uniform", "authored-estimate", "radiator-catalogues")))
bones.append(bone_world("valve-head", "wall", "Thermostatic valve head (dial)", "Thermostatkopf (Einstellrad)", VALVE[0], VALVE[1], 0, 0.025, revolute(0, 270), bones,
                        mass(0.18, 0, 0, "point", "authored-estimate", "radiator-catalogues")))
D = Draw(bones)
FR = D.f

# --------------------------------------------------------------- shapes
def zig(x_a, x_b, y_a, y_b, pitch):
    n = int(round((y_b - y_a) / pitch))
    pts = []
    for i in range(n + 1):
        y = y_a + (y_b - y_a) * i / n
        pts.append((x_a if i % 2 == 0 else x_b, y))
    return pts

def common_ground(pen=None):
    return D.line("floor", "wall", "ground", 0, 0.5, [(WALL_X0, 0.0), (X_FRONT, 0.0)])

def s50():
    sh = [common_ground()]
    sh.append(D.rect("wall-stub", "wall", "body", 1, True, 0.5, WALL_X0, 0, 0, WALL_TOP, region="wall-stub", material="masonry", hatch="cut-masonry"))
    sh.append(D.line("wall-break", "wall", "detail", 2, 0.4, [(WALL_X0, WALL_TOP), (-0.1, WALL_TOP - 0.02), (-0.05, WALL_TOP), (0, WALL_TOP - 0.02)], region="wall-stub"))
    sh.append(D.rect("panel-rear", "radiator", "detail", 3, True, 0.7, X_BACK, Y0, P1, Y1 - 0.015, region="panel-rear", material="painted-steel"))
    sh.append(D.rect("panel-front", "radiator", "detail", 3, True, 0.7, C2, Y0, X_FRONT, Y1 - 0.015, region="panel-front", material="painted-steel"))
    sh.append(D.line("convector-rear", "radiator", "texture", 3, 0.5, zig(P1, C1, Y0 + 0.03, Y1 - 0.015, 0.05), region="convector"))
    sh.append(D.line("convector-front", "radiator", "texture", 3, 0.5, zig(C1, C2, Y0 + 0.03, Y1 - 0.015, 0.05), region="convector"))
    sh.append(D.line("bottom-cap", "radiator", "detail", 3, 0.5, [(X_BACK, Y0), (X_FRONT, Y0)], region="panel-rear"))
    sh.append(D.rect("top-grille", "radiator", "detail", 4, True, 0.35, X_BACK, Y1 - 0.015, X_FRONT, Y1, region="top-grille", material="painted-steel"))
    for i, xs in enumerate((0.05, 0.075, 0.095, 0.12)):
        sh.append(D.line(f"grille-slot-{i}", "radiator", "texture", 5, 0.2, [(xs, Y1 - 0.0125), (xs, Y1 - 0.0025)], region="top-grille"))
    for nm, y in (("upper", 0.58), ("lower", 0.16)):
        sh.append(D.rect(f"bracket-{nm}", "wall", "detail", 3, True, 0.4, 0.0, y, X_BACK, y + 0.012, region="bracket", material="painted-steel"))
    cx, cy, r = VALVE
    sh.append(D.circ("valve-head", "valve-head", "detail", 4, True, 0.3, cx, cy, r, region="valve", material="brass"))
    sh.append(D.line("valve-index", "valve-head", "texture", 5, 0.1, [(cx, cy), (cx, cy + r * 0.9)], region="valve"))
    sh.append(D.line("supply-pipe", "wall", "detail", 3, 0.3, [(cx, 0.0), (cx, cy - r)], region="valve", penMm=0.18))
    return sh

def s100():
    sh = [common_ground()]
    sh.append(D.rect("wall-stub", "wall", "body", 1, True, 0.5, WALL_X0, 0, 0, WALL_TOP, region="wall-stub", material="masonry", hatch="cut-masonry", penMm=0.18))
    sh.append(D.rect("radiator-body", "radiator", "body", 3, True, 0.65, X_BACK, Y0, X_FRONT, Y1 - 0.03, regions=["panel-rear", "panel-front", "convector"], material="painted-steel", penMm=0.18))
    sh.append(D.line("panel-split", "radiator", "detail", 4, 0.4, [(C1, Y0), (C1, Y1 - 0.03)], region="convector"))
    sh.append(D.rect("top-grille", "radiator", "detail", 4, True, 0.35, X_BACK, Y1 - 0.03, X_FRONT, Y1, region="top-grille", material="painted-steel"))
    for nm, y in (("upper", 0.58), ("lower", 0.16)):
        sh.append(D.line(f"bracket-{nm}", "wall", "detail", 3, 0.4, [(0.0, y + 0.006), (X_BACK, y + 0.006)], region="bracket"))
    cx, cy, r = VALVE
    sh.append(D.circ("valve-head", "valve-head", "detail", 4, True, 0.3, cx, cy, r, region="valve", material="brass"))
    sh.append(D.line("supply-pipe", "wall", "detail", 3, 0.3, [(cx, 0.0), (cx, cy - r)], region="valve", penMm=0.13))
    return sh

def s200():
    sh = [common_ground()]
    sh.append(D.rect("wall-stub", "wall", "body", 1, True, 0.5, WALL_X0, 0, 0, WALL_TOP, region="wall-stub", material="masonry"))
    sh.append(D.rect("radiator-body", "radiator", "body", 3, True, 0.65, X_BACK, Y0, X_FRONT, Y1, regions=["panel-rear", "panel-front", "convector", "top-grille", "bracket"], material="painted-steel"))
    cx, cy, r = VALVE
    sh.append(D.circ("valve-head", "valve-head", "body", 4, True, 0.3, cx, cy, r, region="valve", material="brass", penMm=0.1))
    return sh

PEN = {"200": pens(0.18, 0.1, None, None, 0.25), "100": pens(0.25, 0.13, 0.1, None, 0.35), "50": pens(0.35, 0.18, 0.1, None, 0.5)}
reps = {}
reps["500"] = unavailable(500,
    ("The radiator is a slim 0.10 m deep panel hung 0.035 m off its wall: at 1:500 that is 0.2 mm of panel and a 0.07 mm gap, below every print profile, and the heating level cannot be read.",
     "Der Heizkörper ist ein schlankes, 0,10 m tiefes Paneel mit 0,035 m Wandabstand: bei 1:500 sind das 0,2 mm Paneel und 0,07 mm Spalt, unter jedem Druckprofil, und die Heizstufe ist nicht ablesbar."),
    ("Panel depth 0.10 m (0.2 mm at 1:500) and wall clearance 0.035 m (0.07 mm).", "Paneeltiefe 0,10 m (0,2 mm bei 1:500) und Wandabstand 0,035 m (0,07 mm)."),
    0.1, [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}])

hat = lambda sp, pen: [H("cut-masonry", "Cut masonry", "Geschnittenes Mauerwerk", "lines", 45, sp, pen)]
reps["200"] = rep(200, s200(), dec(
    ("A wall stub and the radiator as one filled panel mass beside it, with the valve head as a dot below.", "Ein Wandstück und der Heizkörper als eine gefüllte Paneelmasse daneben, darunter der Ventilkopf als Punkt."),
    ("Wall stub 0.15 m, radiator 0.10 m deep and 0.60 m high at 0.11 m above the floor, valve head (0.06 m).", "Wandstück 0,15 m, Heizkörper 0,10 m tief und 0,60 m hoch, 0,11 m über dem Boden, Ventilkopf (0,06 m)."),
    ("Rear panel, both convectors, front panel, top grille and the two brackets are one filled mass listing all their regions; the 0.035 m wall clearance closes up because its ink gap is below the minimum.", "Rückseitiges Paneel, beide Konvektoren, vorderes Paneel, Abdeckgitter und die beiden Konsolen sind eine gefüllte Masse mit allen Regionen; der Wandabstand von 0,035 m schließt sich, weil seine Tintenlücke unter dem Minimum liegt."),
    ("Fin corrugation, grille slots, supply pipe, valve index line, wall break line.", "Lamellenwellung, Gitterschlitze, Zuleitung, Ventilmarke, Bruchlinie."),
    ("None kept; the valve head is drawn at its real 0.06 m diameter.", "Keine beibehalten; der Ventilkopf ist mit seinem echten Durchmesser 0,06 m gezeichnet."),
    ("No hatch: the wall stub is a tone fill.", "Keine Schraffur: das Wandstück ist Tonfüllung."),
    ("Dark filled masses, 0.18 mm outlines.", "Dunkle gefüllte Massen, 0,18-mm-Konturen."),
    ("Thermal cells: wall stub, radiator mass (area-weighted merge of its five regions), valve.", "Thermische Zellen: Wandstück, Heizkörpermasse (flächengewichtete Zusammenfassung seiner fünf Regionen), Ventil.")),
    pens_=PEN["200"], rng=(150, 350))
reps["100"] = rep(100, s100(), dec(
    ("The cut wall stub, the radiator as one panel body with a split line between panel and convector, grille bar, brackets and valve.", "Das geschnittene Wandstück, der Heizkörper als ein Paneelkörper mit Teilungslinie zwischen Platte und Konvektor, Gitterleiste, Konsolen und Ventil."),
    ("Wall stub, panel body, centre split, grille, two bracket lines, valve head and supply pipe.", "Wandstück, Paneelkörper, Mittelteilung, Gitter, zwei Konsollinien, Ventilkopf und Zuleitung."),
    ("Rear panel, convectors and front panel are one body; the two 0.022 m water panels are too thin (0.22 mm) to print as separate fills.", "Rückplatte, Konvektoren und Frontplatte sind ein Körper; die beiden 0,022 m dicken Wasserplatten sind zu dünn (0,22 mm) für eigene Füllungen."),
    ("Fin corrugation and grille slots.", "Lamellenwellung und Gitterschlitze."),
    ("Bracket plates (4 mm) are drawn as 0.13 mm lines, about 0.09 mm wider than real; the supply pipe (18 mm) as 0.13 mm line.", "Konsolplatten (4 mm) sind als 0,13-mm-Linien gezeichnet, etwa 0,09 mm breiter als real; die Zuleitung (18 mm) als 0,13-mm-Linie."),
    ("Cut masonry as 45 degree lines at 0.5 mm.", "Geschnittenes Mauerwerk als 45-Grad-Linien mit 0,5 mm."),
    ("Outline 0.25 mm, detail 0.13 mm.", "Kontur 0,25 mm, Detail 0,13 mm."),
    ("Wall stub, radiator body (merged panels and convector), split line for the convector, grille, bracket lines, valve.", "Wandstück, Heizkörperkörper (zusammengefasste Platten und Konvektor), Teilungslinie für den Konvektor, Gitter, Konsollinien, Ventil.")),
    hatches=hat(0.5, 0.1), pens_=PEN["100"], rng=(75, 150))
reps["50"] = rep(50, s50(), dec(
    ("The vertical section of a type 22 radiator (panel, convector, convector, panel) hung on two brackets 0.035 m off a cut wall, valve head on the bottom connection.", "Der Vertikalschnitt eines Heizkörpers Typ 22 (Platte, Konvektor, Konvektor, Platte) an zwei Konsolen 0,035 m vor einer geschnittenen Wand, Ventilkopf am unteren Anschluss."),
    ("Both water panels (22 mm), both convector fin packs, top grille with four slots, brackets, valve head with index mark, supply pipe, wall break line.", "Beide Wasserplatten (22 mm), beide Konvektorlamellenpakete, Abdeckgitter mit vier Schlitzen, Konsolen, Ventilkopf mit Marke, Zuleitung, Wandbruchlinie."),
    ("Both convectors share the zigzag vertices at the centre plane.", "Beide Konvektoren teilen die Zickzack-Knoten in der Mittelebene."),
    ("Side covers and end caps (not in the section plane).", "Seitenabdeckungen und Endkappen (nicht in der Schnittebene)."),
    ("The fin corrugation is symbolic: pitch 0.05 m (real fins about 0.01 m, 0.2 mm on paper cannot print).", "Die Lamellenwellung ist symbolisch: Teilung 0,05 m (echte Lamellen etwa 0,01 m, 0,2 mm auf Papier sind nicht druckbar)."),
    ("Cut masonry as 45 degree lines at 0.6 mm.", "Geschnittenes Mauerwerk als 45-Grad-Linien mit 0,6 mm."),
    ("Outline 0.35 mm for the wall, 0.18 mm for the radiator, 0.1 mm for fins.", "Kontur 0,35 mm für die Wand, 0,18 mm für den Heizkörper, 0,1 mm für Lamellen."),
    ("One overlay cell per region: both panels, the convector (two zigzags), grille, brackets, valve, wall stub.", "Eine Überlagerungszelle je Region: beide Platten, Konvektor (zwei Zickzacklinien), Gitter, Konsolen, Ventil, Wandstück.")),
    hatches=hat(0.6, 0.1), pens_=PEN["50"], rng=(20, 75))

# --------------------------------------------------------------- thermal
REFS = [
    ref("en-442-2", "EN 442-2:2014 Radiators and convectors, test methods and rating: heat output Phi = K_m * dT^n, standard rating at 75/65/20 degC (dT = 50 K) with exponent n typically 1.2 to 1.4 for panel radiators."),
    ref("en-442-1", "EN 442-1:2014 Radiators and convectors, technical specifications and requirements (panel and convector designation: type 22 = two panels and two convectors)."),
    ref("radiator-catalogues", "Manufacturer catalogues for compact panel radiators type 22, height 600 mm, length 1000 mm, depth 100 mm (rated output about 1.1 to 1.3 kW at 75/65/20 degC, mass about 25 kg with water). Typical values consulted, not a table look-up."),
    ref("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties (clay masonry rho 1800, lambda 0.8 W/mK; steel 50 W/mK, 450 J/kgK; emissivity of paint 0.9)."),
    ref("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts (hatching)."),
    ref("din-1356-1", "DIN 1356-1:1995 Building drawings, types of drawings and basic principles of presentation."),
    ref("en-iso-6946", "EN ISO 6946:2017 Building components, thermal resistance and transmittance (surface resistances)."),
    ref("din-4701", "Kuchling, Taschenbuch der Physik, and common heating practice: a panel radiator gives about 25 to 35 % of its output by radiation, the rest by convection (convector channels deliver most of the convective share)."),
]
mats = [
    mat("masonry", "Plastered clay masonry", "Verputztes Ziegelmauerwerk", 0.9, 0.5, 1800, 0.8, 1000, src="literature", reference="en-iso-10456"),
    mat("painted-steel", "White powder-coated sheet steel 1.2 mm", "Weiß pulverbeschichtetes Stahlblech 1,2 mm", 0.9, 0.3, 7800, 50, 450, 210e9, 235e6, "literature", "en-iso-10456"),
    mat("brass", "Nickel-plated brass valve", "Vernickeltes Messingventil", 0.25, 0.6, 8400, 120, 380, src="authored-estimate"),
]
RATED = 1200.0
regions = [
    region("wall-stub", "Plastered wall stub (visible face)", "Verputztes Wandstück (Sichtfläche)", "wall", "passive", "masonry", 0.85, 0.85, "vertical-plate", 0.5,
           [layer("plaster", "Gypsum plaster 15 mm", "Gipsputz 15 mm", "masonry", "masonry", 0.015, 0.026, "literature", "en-iso-10456", c=0.015 * 1300 * 1000)], 60000, "literature", "en-iso-10456"),
    region("panel-rear", "Rear water panel", "Rückseitige Wasserplatte", "heater", "operating", "painted-steel", 0.6, 0.6, "vertical-plate", 0.5, [], 22000, "authored-estimate", "radiator-catalogues",
           operating=("heating", 0.28 * RATED)),
    region("panel-front", "Front water panel", "Vordere Wasserplatte", "heater", "operating", "painted-steel", 0.6, 0.6, "vertical-plate", 0.5, [], 22000, "authored-estimate", "radiator-catalogues",
           operating=("heating", 0.37 * RATED)),
    region("convector", "Convector fins between the panels", "Konvektorlamellen zwischen den Platten", "heater", "operating", "painted-steel", 1.4, 0.6, "vertical-plate", 0.3, [], 3500, "authored-estimate", "radiator-catalogues",
           operating=("heating", 0.35 * RATED)),
    region("top-grille", "Top grille", "Abdeckgitter oben", "surface", "passive", "painted-steel", 0.1, 0.1, "horizontal-plate-up", 1.0, [], 2500, "authored-estimate", "radiator-catalogues"),
    region("bracket", "Wall brackets", "Wandkonsolen", "surface", "passive", "painted-steel", 0.004, 0.05, "vertical-plate", 0.5, [], 4000, "authored-estimate", "radiator-catalogues"),
    region("valve", "Thermostatic valve and supply pipe", "Thermostatventil und Zuleitung", "surface", "passive", "brass", 0.012, 0.05, "cylinder", 0.5, [], 15000, "authored-estimate", "radiator-catalogues"),
]
LEVELS = [(0.0, "Off, valve closed (*)", "Aus, Ventil geschlossen (*)", 0),
          (0.515, "Night, dT 30 K (55/45/20 degC, dial 2)", "Nacht, dT 30 K (55/45/20 °C, Stufe 2)", 108),
          (0.748, "Comfort, dT 40 K (65/55/20 degC, dial 3)", "Komfort, dT 40 K (65/55/20 °C, Stufe 3)", 162),
          (1.0, "Rated, dT 50 K (75/65/20 degC, dial 5)", "Nennleistung, dT 50 K (75/65/20 °C, Stufe 5)", 270)]
operating = [{"id": "heating", "title": T("Heating water through the valve", "Heizwasser durch das Ventil"),
              "levels": [{"v": v, "title": T(en, de)} for v, en, de, _ in LEVELS]}]
emitters = [
    {"id": "convection-outlet", "kind": "vent", "title": T("Warm air leaving the convector channels at the top grille", "Warme Luft aus den Konvektorkanälen am Abdeckgitter"),
     "anchor": "grille-centre", "directionDeg": 90, "diameterM": 0.19, "operatingKey": "heating", "volumeFlowM3S": 0.018, "temperatureRiseK": 28, "vapourKgPerKg": 0.0,
     "source": "authored-estimate", "reference": "radiator-catalogues"},
    {"id": "front-plume", "kind": "plume", "title": T("Free-convection plume rising along the front panel", "Freie Konvektionsfahne an der Frontplatte"),
     "anchor": "grille-centre", "directionDeg": 90, "diameterM": 0.1, "region": "panel-front", "source": "authored-estimate", "reference": "radiator-catalogues"},
]
anchors = [
    anchor("wall-foot", "wall", 0, 0, "support", "Wall foot, fixed in the floor slab", "Wandfuß, im Boden eingespannt", support="fixed"),
    anchor("bracket-top", "wall", 0.58, 0, "attachment", "Upper bracket", "Obere Konsole"),
    anchor("front-top-edge", "radiator", 0.6, 0.1, "load", "Front top edge of the radiator", "Vorderkante oben am Heizkörper"),
    anchor("grille-centre", "radiator", 0.6, 0.05, "emitter", "Centre of the top grille", "Mitte des Abdeckgitters"),
]

def regs(level, sun=0.0):
    return {r["id"]: RB(1.0, sun) for r in regions}
def mkpose(id, en, de, den, dde, lev_i, loads=()):
    v, _, _, ang = LEVELS[lev_i]
    on = 1 if v > 0 else 0
    return pose(id, en, de, den, dde, {"valve-head": {"angleDeg": ang}} if ang else {}, list(loads), regs(v),
                {"convection-outlet": on, "front-plume": on}, {"heating": v})
leaning = load("leaned-on", "imposed", "Person leaning on the radiator, 0.4 kN down at the front edge", "Person lehnt am Heizkörper, 0,4 kN nach unten an der Vorderkante", 0, -400, "authored-estimate", anchor_="front-top-edge")
poses = [
    mkpose("off", "Valve closed", "Ventil geschlossen", "Dial at the frost mark, no hot water, radiator at room temperature.", "Einstellrad auf Frostschutzmarke, kein Heizwasser, Heizkörper auf Raumtemperatur.", 0),
    mkpose("night", "Night setting", "Nachtabsenkung", "Dial 2, flow 55/45 degC, dT 30 K: 51.5 % of the rated output by the EN 442 exponent n = 1.3.", "Stufe 2, Vorlauf 55/45 °C, dT 30 K: 51,5 % der Nennleistung nach EN-442-Exponent n = 1,3.", 1),
    mkpose("comfort", "Comfort setting", "Komforteinstellung", "Dial 3, flow 65/55 degC, dT 40 K: 74.8 % of the rated output.", "Stufe 3, Vorlauf 65/55 °C, dT 40 K: 74,8 % der Nennleistung.", 2),
    mkpose("rated", "Rated output 75/65/20", "Nennleistung 75/65/20", "Dial 5, flow 75/65 degC, dT 50 K: the catalogue rating of 1.2 kW.", "Stufe 5, Vorlauf 75/65 °C, dT 50 K: die Katalognennleistung von 1,2 kW.", 3),
    mkpose("leaned-on", "Leaned on at comfort", "Angelehnt bei Komfort", "Comfort setting with a person leaning on the front edge (0.4 kN).", "Komforteinstellung, eine Person lehnt an der Vorderkante (0,4 kN).", 2, [leaning]),
]
for p in poses:
    if p["id"] == "leaned-on":
        pass
clips = [
    clip("heat-up", "Heat-up from cold", "Aufheizen aus dem kalten Zustand", "The valve is opened from closed to rated; heat output, convection outlet and plume follow the dial.", "Das Ventil wird von geschlossen auf Nennleistung geöffnet; Heizleistung, Konvektionsauslass und Fahne folgen dem Einstellrad.",
         60, False, [key(0, "off"), key(6, "rated"), key(60, "rated")],
         [phase("open", 0, 6, "Turning the dial", "Rad drehen"), phase("warm", 6, 60, "Warming up", "Aufwärmen")],
         [event(6, "operating", "Rated flow reached", "Nennvolumenstrom erreicht", "heating")],
         [track("operating:heating", [(0, 0), (6, 1), (60, 1)]), track("emitter:convection-outlet", [(0, 0), (6, 1), (60, 1)])]),
    clip("thermostat-cycle", "Thermostat cycling", "Thermostatregelung", "The thermostat head modulates between comfort and closed; the radiator cools and heats in a loop.", "Der Thermostatkopf regelt zwischen Komfort und geschlossen; der Heizkörper kühlt ab und heizt in einer Schleife.",
         40, True, [key(0, "comfort"), key(10, "off"), key(25, "off"), key(40, "comfort")],
         [phase("hot", 0, 10, "Closing", "Schließen"), phase("cool", 10, 25, "Cooling", "Abkühlen"), phase("reopen", 25, 40, "Reopening", "Wiederöffnen")],
         [event(10, "operating", "Valve closed", "Ventil geschlossen", "heating"), event(25, "operating", "Valve reopens", "Ventil öffnet", "heating")],
         [track("operating:heating", [(0, 0.748), (10, 0), (25, 0), (40, 0.748)]), track("emitter:convection-outlet", [(0, 1), (10, 0), (25, 0), (40, 1)])]),
]
a = {
    "id": "panel-radiator", "category": "components",
    "title": T("Panel Radiator Type 22", "Plattenheizkörper Typ 22"),
    "summary": T("A compact type 22 steel panel radiator (two water panels, two convectors; 600 mm high, 1000 mm long, 100 mm deep, rated 1.2 kW at 75/65/20 degC) shown in vertical section on two wall brackets 35 mm off a plastered wall, with its thermostatic valve; output scales with the EN 442 exponent n = 1.3 and rises as a convection plume.",
                 "Ein kompakter Stahl-Plattenheizkörper Typ 22 (zwei Wasserplatten, zwei Konvektoren; 600 mm hoch, 1000 mm lang, 100 mm tief, 1,2 kW bei 75/65/20 °C) im Vertikalschnitt an zwei Wandkonsolen 35 mm vor einer verputzten Wand, mit Thermostatventil; die Leistung folgt dem EN-442-Exponenten n = 1,3 und steigt als Konvektionsfahne auf."),
    "tags": ["component", "heating", "radiator", "operating", "convection", "thermal"],
    "provenance": {"author": "Semio staffage component authors (NW-C)", "method": "handcrafted", "created": "2026-10-29", "references": REFS,
                   "notes": T("Output per level uses Phi = Phi_50 (dT/50)^n with n = 1.3; the 1.2 kW rating and the convective share are catalogue-typical authored estimates.", "Die Leistung je Stufe folgt Phi = Phi_50 (dT/50)^n mit n = 1,3; die Nennleistung 1,2 kW und der Konvektionsanteil sind katalogtypische Schätzwerte.")},
    "materials": mats,
    "rig": {"kind": "mechanical", "root": "wall", "bones": bones},
    "anchors": anchors,
    "representations": reps, "symbols": [],
    "poses": poses, "clips": clips,
    "thermal": {"environment": env(20, 19, None, None, 0.4, 0.1, 0, 1.0), "operating": operating, "regions": regions, "emitters": emitters, "passengers": [],
                "assumptions": [
                    T("Reference case: heated room at 20 degC (the EN 442 reference air temperature), no sun; dT is the mean water temperature minus 20 degC.", "Referenzfall: beheizter Raum mit 20 °C (EN-442-Bezugslufttemperatur), keine Sonne; dT ist die mittlere Wassertemperatur minus 20 °C."),
                    T("Heat per level is Phi_50 (dT/50)^1.3 with Phi_50 = 1200 W, split over the front panel (37 %), rear panel (28 %) and convector fins (35 %) as an authored estimate; the passive grille warms by convection only.", "Die Wärme je Stufe ist Phi_50 (dT/50)^1,3 mit Phi_50 = 1200 W, aufgeteilt auf Frontplatte (37 %), Rückplatte (28 %) und Konvektorlamellen (35 %) als Schätzung; das passive Gitter erwärmt sich nur konvektiv."),
                    T("The vent emitter is the air stream of the convector channels (0.018 m3/s, 28 K warmer than the room, about 0.6 kW); the plume is the boundary-layer convection along the front panel.", "Der Auslass ist der Luftstrom der Konvektorkanäle (0,018 m3/s, 28 K wärmer als der Raum, etwa 0,6 kW); die Fahne ist die Grenzschichtkonvektion an der Frontplatte."),
                    T("The wall stub is the plastered partition behind; it receives radiation from the rear panel through the 35 mm gap and is not conditioned.", "Das Wandstück ist die verputzte Trennwand dahinter; sie erhält Strahlung der Rückplatte durch den 35-mm-Spalt und ist nicht konditioniert.")]},
    "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": ["wall-foot"],
                  "assumptions": [T("The wall stub is a rigid cantilever fixed at the slab; the radiator hangs on two brackets and is carried as a rigid part of it.", "Das Wandstück ist ein starrer Kragarm, am Boden eingespannt; der Heizkörper hängt an zwei Konsolen und wird als starrer Teil getragen."),
                                T("Wall and radiator are a 1 m strip; the leaning load is an authored estimate, not a normative load.", "Wand und Heizkörper sind ein 1-m-Streifen; die Anlehnlast ist ein Schätzwert, keine Normlast.")], "clinical": False},
    "presentation": {"defaultPose": "rated", "defaultClip": "heat-up", "defaultScale": 50, "viewpoints": ["elevation"]},
    "exports": ALL_EXPORTS, "completion": "authored", "_dimsFrom": "50", "_depth": 1.0,
}
assemble(a, "panel-radiator")
