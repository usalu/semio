import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nw import *
D = tx

REL = "🚜️vehicles/tram"
RW = 0.30
WY = 0.30
BF, BR = 7.2, 1.9
PB = (2.6, 3.55)
LA = 1.9
AR = (22, 136, -158)
AD = (3, 180, -183)
LOOP_TURNS = 6
LOOP_M = LOOP_TURNS * 2 * math.pi * RW
V = 8.33
T_LOOP = round(LOOP_M / V, 4)
DOORS = {"door-a1": (3.50, 4.15), "door-a2": (4.15, 4.80), "door-b1": (-4.00, -3.35), "door-b2": (-3.35, -2.70)}

bones = [
    bone("body-end", None, "End module with cab", "Endmodul mit Fahrerstand", 0, 0, 0, 9.0, FIXED, mass(8300, 5.0, 1.5)),
    bone("gangway", "body-end", "Articulation bellows", "Gelenkbalg", 0, 0.3, 0, 0.9, FIXED, mass(300, 0, 1.4)),
    bone("body-mid", "body-end", "Suspended middle module", "Eingehängtes Mittelmodul", 0, 0.3, 180, 5.55, REV(-3, 3), mass(3900, 2.8, 1.2)),
    bone("bogie-f", "body-end", "Powered front bogie", "Angetriebenes vorderes Drehgestell", BF, WY, 0, 1.8, PRI(90, -0.06, 0.06), mass(2900)),
    bone("bogie-r", "body-end", "Trailing bogie under the articulation", "Laufdrehgestell unter dem Gelenk", BR, WY, 0, 1.8, PRI(90, -0.06, 0.06), mass(1900)),
    bone("ws-f1", "bogie-f", "Front bogie wheelset 1", "Radsatz 1 des Vordergestells", -0.9, 0, 0, RW, FREE, mass(550, ref="en-13803")),
    bone("ws-f2", "bogie-f", "Front bogie wheelset 2", "Radsatz 2 des Vordergestells", 0.9, 0, 0, RW, FREE, mass(550, ref="en-13803")),
    bone("ws-r1", "bogie-r", "Rear bogie wheelset 1", "Radsatz 1 des Hintergestells", -0.9, 0, 0, RW, FREE, mass(550, ref="en-13803")),
    bone("ws-r2", "bogie-r", "Rear bogie wheelset 2", "Radsatz 2 des Hintergestells", 0.9, 0, 0, RW, FREE, mass(550, ref="en-13803")),
    bone("pan-lower", "body-end", "Pantograph lower arm", "Stromabnehmer-Unterarm", PB[0], PB[1], AR[0], LA, REV(-30, 5), mass(70, 0.95, 0, "uniform")),
    bone("pan-upper", "pan-lower", "Pantograph upper arm", "Stromabnehmer-Oberarm", LA, 0, AR[1], LA, REV(-10, 50), mass(60, 0.95, 0, "uniform")),
    bone("pan-head", "pan-upper", "Collector head", "Schleifleiste", LA, 0, AR[2], 0.5, REV(-30, 5), mass(30, 0, 0)),
]
for nm, (x0, x1) in DOORS.items():
    parent = "body-end" if nm.startswith("door-a") else "body-mid"
    mid = nm.endswith("1")
    bones.append(bone(nm, "body-end", "Door leaf " + nm[-2:].upper(), "Türflügel " + nm[-2:].upper(), x0, 0.30, 0, x1 - x0, PRI(180 if mid else 0, 0, 0.6), mass(85, (x1 - x0) / 2, 1.2)))
# door leaves hang on the module they belong to; middle-module doors use body-end parent for simplicity of the chain only through offsets
L = Loc(bones)
Dw = Draw(L)
anchors = [
    contact("contact-f1", "bogie-f", -0.9, -RW, "Front wheelset 1 contact", "Aufstandspunkt Radsatz vorn 1", 0.012, 0.014, 0.3),
    contact("contact-f2", "bogie-f", 0.9, -RW, "Front wheelset 2 contact", "Aufstandspunkt Radsatz vorn 2", 0.012, 0.014, 0.3),
    contact("contact-r1", "bogie-r", -0.9, -RW, "Rear wheelset 1 contact", "Aufstandspunkt Radsatz hinten 1", 0.012, 0.014, 0.3),
    contact("contact-r2", "bogie-r", 0.9, -RW, "Rear wheelset 2 contact", "Aufstandspunkt Radsatz hinten 2", 0.012, 0.014, 0.3),
    anchor("resistor-vent", "body-end", 5.5, 3.62, "emitter", "Brake resistor fan outlet", "Ventilatorauslass Bremswiderstand"),
    anchor("hvac-vent", "body-mid", -3.5 - 0, 3.62 - 0.3, "emitter", "Roof air-conditioning outlet", "Auslass der Dachklimaanlage"),
    anchor("floor-aisle", "body-end", 3.5, 0.45, "load", "Aisle load point of the end module", "Lastpunkt Gang des Endmoduls"),
    anchor("floor-aisle-mid", "body-mid", -3.5, 0.45 - 0.3, "load", "Aisle load point of the middle module", "Lastpunkt Gang des Mittelmoduls"),
]
anchors[5] = anchor("hvac-vent", "body-mid", -3.5, 3.62, "emitter", "Roof air-conditioning outlet", "Auslass der Dachklimaanlage")
anchors[7] = anchor("floor-aisle-mid", "body-mid", -3.5, 0.45, "load", "Aisle load point of the middle module", "Lastpunkt Gang des Mittelmoduls")
mats = [
    material("paint", "Painted aluminium and GRP body", "Lackierter Aluminium- und GFK-Aufbau", 0.9, 0.4, 2000, 100, "literature", "ashrae-2021"),
    material("glass", "Laminated and toughened glass", "Verbund- und Einscheibensicherheitsglas", 0.84, 0.15, 2500, 1.0, "literature", "ashrae-2021"),
    material("interior-trim", "Interior lining", "Innenverkleidung", 0.9, 0.8, 300, 0.1),
    material("steel-wheel", "Wheel tyre steel", "Radreifenstahl", 0.7, 0.7, 7850, 45, "literature", "incropera-2011"),
    material("cast-iron", "Brake disc cast iron", "Graugussbremsscheibe", 0.65, 0.7, 7200, 46, "literature", "limpert-1999"),
    material("rubber", "Bellows fabric and rubber", "Balgstoff und Gummi", 0.92, 0.9, 1200, 0.2),
    material("motor-steel", "Traction motor housing", "Fahrmotorgehäuse", 0.8, 0.85, 7200, 46, "authored-estimate", "en-13452"),
    material("resistor-steel", "Resistor box stainless steel", "Widerstandsgehäuse Edelstahl", 0.6, 0.5, 7900, 16, "authored-estimate", "en-13452"),
    material("carbon", "Collector strip carbon", "Schleifleiste Kohle", 0.85, 0.9, 1800, 25, "authored-estimate"),
    material("pan-alloy", "Pantograph aluminium arms", "Stromabnehmer-Aluminiumarme", 0.3, 0.45, 2700, 150, "literature", "ashrae-2021"),
]
KEYS = [
    opkey("traction", "Traction power", "Fahrleistung", [(0, "Off", "Aus"), (0.05, "Ready, standing", "Bereit, stehend"), (0.5, "Cruising", "Reisefahrt"), (1, "Full power 480 kW", "Volle Leistung 480 kW")]),
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.6, "About 30 km/h", "Etwa 30 km/h"), (1, "About 50 km/h", "Etwa 50 km/h")]),
    opkey("braking", "Braking (resistor and friction)", "Bremsen (Widerstand und Reibung)", [(0, "Not braking", "Nicht gebremst"), (0.3, "Service braking", "Betriebsbremsung"), (1, "Emergency braking from 50 km/h", "Schnellbremsung aus 50 km/h")]),
    opkey("hvac", "Roof air conditioning", "Dachklimaanlage", [(0, "Off", "Aus"), (0.3, "Ventilation", "Lüftung"), (0.7, "Climate control", "Klimatisierung"), (1, "Maximum", "Maximum")]),
]
cond = {"interiorTempC": 21, "interiorFilmM2KW": 0.13}


def cregion(id, en, de, kind, area, ln, geo, sky, layers, hc):
    r = region(id, en, de, kind, "conditioned", layers[0]["material"], area, ln, geo, sky, hc, layers, ref="ashrae-2021")
    r["conditioned"] = cond
    return r


skin = layer("skin", "Painted aluminium skin 1.5 mm", "Lackierte Aluminiumhaut 1,5 mm", "metal", "paint", 0.0015, 3e-5, 4000, source="literature", ref="incropera-2011")
wool = layer("insulation", "Mineral wool lining 50 mm", "Mineralwolle-Verkleidung 50 mm", "insulation", "interior-trim", 0.05, 1.25, 800)
glassl = [layer("pane-out", "Toughened glass 6 mm", "Einscheibensicherheitsglas 6 mm", "glass", "glass", 0.006, 0.006, 11000, source="literature", ref="incropera-2011"), layer("gap", "Air gap 12 mm", "Luftspalt 12 mm", "air-gap", "glass", 0.012, 0.15, source="literature", ref="incropera-2011"), layer("pane-in", "Toughened glass 6 mm", "Einscheibensicherheitsglas 6 mm", "glass", "glass", 0.006, 0.006, 11000, source="literature", ref="incropera-2011")]
REGIONS = [
    cregion("body-panels", "Painted side and end panels", "Lackierte Seiten- und Stirnwände", "wall", 55, 2.4, "vertical-plate", 0.5, [skin, wool], 8000),
    cregion("roof", "Roof panel", "Dachblech", "wall", 36, 2.4, "horizontal-plate-up", 0.95, [skin, wool], 8000),
    cregion("glazing", "Window band and cab glazing", "Fensterband und Fahrerstandverglasung", "glazing", 26, 1.2, "vertical-plate", 0.6, glassl, 22000),
    cregion("doors", "Plug-sliding door leaves", "Schwenkschiebetürflügel", "wall", 11, 1.3, "vertical-plate", 0.5, [skin, wool], 8000),
    region("gangway", "Articulation bellows", "Gelenkbalg", "surface", "passive", "rubber", 9, 1.5, "vertical-plate", 0.4, 3000, [layer("fabric", "Rubber-coated fabric 8 mm", "Gummiertes Gewebe 8 mm", "polymer", "rubber", 0.008, 0.008 / 0.2, 3000)]),
    region("traction-motors", "Traction motors and bogie frames", "Fahrmotoren und Drehgestellrahmen", "engine", "operating", "motor-steel", 4.5, 0.5, "cylinder", 0.15, 20000, [], ("traction", 12000), ref="en-13452"),
    region("wheelsets", "Steel wheels and axles", "Stahlräder und Achsen", "surface", "operating", "steel-wheel", 3.5, 0.6, "cylinder", 0.2, 22000, [], ("rolling", 600), ref="en-13803"),
    region("brake-discs", "Friction brake discs", "Reibbremsscheiben", "surface", "operating", "cast-iron", 1.6, 0.6, "vertical-plate", 0.2, 22000, [], ("braking", 30000), ref="en-13452"),
    region("brake-resistor", "Roof brake resistor box", "Dach-Bremswiderstandskasten", "surface", "operating", "resistor-steel", 3.2, 1.8, "horizontal-plate-up", 0.9, 9000, [], ("braking", 30000), ref="en-13452"),
    region("hvac-unit", "Roof air-conditioning unit", "Dachklimagerät", "surface", "operating", "paint", 7, 2.8, "horizontal-plate-up", 0.9, 6000, [], ("hvac", 3000), ref="ashrae-2021"),
    region("pantograph", "Pantograph arms and collector", "Stromabnehmerarme und Schleifleiste", "surface", "operating", "pan-alloy", 1.1, 0.06, "cylinder", 0.9, 3000, [], ("traction", 300), ref="en-50163"),
]
EM = [
    emitter("resistor-fan", "fan", "Brake resistor cooling air", "Kühlluft des Bremswiderstands", "resistor-vent", 90, 0.7, "braking", 2.0, 55, 0.0, ref="en-13452"),
    emitter("hvac-vent", "vent", "Roof air-conditioning exhaust air", "Abluft der Dachklimaanlage", "hvac-vent", 90, 0.9, "hvac", 2.5, 20, 0.0, ref="ashrae-2021"),
]
DEF = {"body-panels": (1, 0.5), "roof": (1, 1), "glazing": (1, 0.5), "doors": (1, 0.5), "gangway": (1, 0.5), "traction-motors": (0.3, 0), "wheelsets": (0.5, 0.05), "brake-discs": (0.5, 0), "brake-resistor": (1, 1), "hvac-unit": (1, 1), "pantograph": (1, 1)}
CONT = {"wheelsets": (0.003, 28)}


def th(tr, roll, br, hv, regs=None, em=None):
    return binding(DEF, regs, {"traction": tr, "rolling": roll, "braking": br, "hvac": hv}, em or {"resistor-fan": 1 if br > 0.2 else 0, "hvac-fan": 0, "hvac-vent": 1 if hv > 0 else 0} if False else {"resistor-fan": 1 if br > 0.2 else 0, "hvac-vent": 1 if hv > 0 else 0}, CONT)


PANDOWN = {"pan-lower": {"angleDeg": AD[0] - AR[0]}, "pan-upper": {"angleDeg": AD[1] - AR[1]}, "pan-head": {"angleDeg": AD[2] - AR[2]}}
OPEN = {}
for nm, (x0, x1) in DOORS.items():
    OPEN[nm] = {"dx": -0.55 if nm.endswith("1") else 0.55}
R0 = {"x": 0, "y": 0, "angleDeg": 0}
CT = ["contact-f1", "contact-f2", "contact-r1", "contact-r2"]
LOADN = lambda kg: [load("passengers-end", "occupant", "Passengers in the end module", "Fahrgäste im Endmodul", 0, 0, bone="body-end", dist={"bone": "body-end", "fromM": 1.0, "toM": 8.0, "wxNPerM": 0, "wyNPerM": -round(kg * 9.80665 / 7.0, 1)}, ref="vdv-154")]
poses = [
    pose("running", "Running at 30 km/h, pantograph raised", "Fahrt mit 30 km/h, Stromabnehmer angelegt", "Doors closed, pantograph in contact with the wire at 5.0 m, traction at half power, air conditioning in climate control.", "Türen geschlossen, Stromabnehmer am Fahrdraht in 5,0 m Höhe, Traktion mit halber Leistung, Klimaanlage im Klimabetrieb.", {}, R0, CT, th(0.5, 0.6, 0, 0.7), LOADN(2800)),
    pose("at-stop-doors-open", "At the stop, doors open", "An der Haltestelle, Türen offen", "Standing with all four door pairs open (0.55 m per leaf), traction ready, pantograph raised.", "Stehend mit allen vier Türpaaren offen (0,55 m je Flügel), Traktion bereit, Stromabnehmer angelegt.", dict(OPEN), R0, CT, th(0.05, 0, 0.3, 0.9), LOADN(1800)),
    pose("braking-stop", "Emergency braking", "Schnellbremsung", "Brake resistor and friction discs at full effort after 50 km/h; resistor fan running; pantograph raised.", "Bremswiderstand und Reibscheiben mit voller Wirkung nach 50 km/h; Widerstandslüfter läuft; Stromabnehmer angelegt.", {}, R0, CT, th(0, 0.3, 1, 0.7), LOADN(2800)),
    pose("depot-pantograph-down", "Depot, pantograph lowered", "Depot, Stromabnehmer abgesenkt", "Everything off and cold, pantograph folded to 3.75 m, doors closed.", "Alles aus und kalt, Stromabnehmer auf 3,75 m eingefahren, Türen geschlossen.", dict(PANDOWN), R0, CT, th(0, 0, 0, 0, {"roof": {"exposedFraction": 1, "sunlitFraction": 1}})),
]
for name, frac in (("loop-start", 0.0), ("loop-mid", 0.5), ("loop-end", 1.0)):
    ang = wheel_turn(LOOP_M * frac, RW)
    poses.append(pose(name, "Run loop " + name.split("-")[1], "Fahrschleife " + name.split("-")[1], "Technical key of the run loop: wheelsets have turned theta = -x / r.", "Technische Schlüsselstellung der Fahrschleife: die Radsätze haben theta = -x / r gedreht.",
                      {w: {"angleDeg": ang} for w in ("ws-f1", "ws-f2", "ws-r1", "ws-r2")}, {"x": round(LOOP_M * frac, 4), "y": 0, "angleDeg": 0}, CT, th(0.5, 0.6, 0, 0.7), LOADN(2800), key=True))
STOPX = 28.9
poses.append(pose("stop-in", "Arriving at 30 km/h", "Anfahrt mit 30 km/h", "Technical key: enters the stop clip at 30 km/h.", "Technische Schlüsselstellung: Einfahrt in den Halt-Clip mit 30 km/h.", {}, R0, CT, th(0.2, 0.6, 0.3, 0.7), LOADN(2800), key=True))
SWS = {w: {"angleDeg": wheel_turn(STOPX, RW)} for w in ("ws-f1", "ws-f2", "ws-r1", "ws-r2")}
poses.append(pose("stop-out", "Stopped at the platform", "Am Bahnsteig zum Stillstand gekommen", "Technical key: stopped after 28.9 m of braking at 1.2 m/s2, doors still closed.", "Technische Schlüsselstellung: nach 28,9 m Bremsweg bei 1,2 m/s2 zum Stillstand gekommen, Türen noch geschlossen.", dict(SWS), {"x": STOPX, "y": 0, "angleDeg": 0}, CT, th(0.05, 0, 0.3, 0.9), LOADN(2800), key=True))
OPENS = {**SWS, **OPEN}
clips = [
    clip("run-loop", "Run loop at 30 km/h", "Fahrschleife mit 30 km/h", "Six wheel turns (%.2f m) per cycle at 8.33 m/s: all four wheelsets roll with theta = -x / r, the root advances one cycle length every loop, traction at half power." % LOOP_M, "Sechs Radumdrehungen (%.2f m) pro Zyklus mit 8,33 m/s: alle vier Radsätze rollen mit theta = -x / r, die Wurzel rückt je Schleife um eine Zykluslänge vor, Traktion mit halber Leistung." % LOOP_M,
         T_LOOP, True, [ck(0, "loop-start"), ck(round(T_LOOP / 2, 4), "loop-mid"), ck(T_LOOP, "loop-end")], [span(c, 0, T_LOOP) for c in CT], [phase("cruise", 0, T_LOOP, "Steady run", "Gleichmäßige Fahrt")],
         [event(0, "operating", "Traction at half power", "Traktion mit halber Leistung")], [track("operating:traction", [(0, 0.5), (T_LOOP, 0.5)]), track("operating:rolling", [(0, 0.6), (T_LOOP, 0.6)]), track("emitter:hvac-vent", [(0, 1), (T_LOOP, 1)])]),
    clip("stop-and-open-doors", "Brake at the stop and open the doors", "An der Haltestelle bremsen und Türen öffnen", "The tram brakes at 1.2 m/s2 from 30 km/h to rest over 28.9 m (6.9 s) with the brake resistor, then all door pairs slide open.", "Die Straßenbahn bremst mit 1,2 m/s2 von 30 km/h in 28,9 m (6,9 s) bis zum Stillstand mit dem Bremswiderstand, dann gleiten alle Türpaare auf.",
         14.0, False, [ck(0, "stop-in", "out"), ck(6.94, "stop-out", "linear"), ck(8.0, "stop-out", "smooth", dict(SWS)), ck(9.5, "stop-out", "smooth", dict(OPENS)), ck(14.0, "stop-out", "linear", dict(OPENS))],
         [span(c, 0, 14.0) for c in CT], [phase("brake", 0, 6.94, "Braking to rest", "Bremsen bis zum Stillstand"), phase("doors", 8.0, 9.5, "Doors open", "Türen öffnen"), phase("dwell", 9.5, 14.0, "Passenger exchange", "Fahrgastwechsel")],
         [event(6.94, "motion", "Standstill", "Stillstand"), event(8.0, "motion", "Doors start to open", "Türen beginnen zu öffnen")],
         [track("operating:braking", [(0, 0.5), (3.0, 0.7), (6.94, 0.3), (9, 0), (14, 0)]), track("operating:traction", [(0, 0.2), (1.0, 0), (6.94, 0), (14, 0.05)]), track("operating:rolling", [(0, 0.6), (6.94, 0), (14, 0)]), track("emitter:resistor-fan", [(0, 1), (8.0, 1), (9.0, 0)], "step"), track("emitter:hvac-vent", [(0, 1), (14, 1)])]),
    clip("pantograph-cycle", "Lower and raise the pantograph", "Stromabnehmer senken und heben", "At standstill in the depot the pantograph is lowered to 3.75 m in 3 s, rests, and is raised again to the wire at 5.0 m; traction current collection stops in between.", "Im Stand im Depot wird der Stromabnehmer in 3 s auf 3,75 m gesenkt, ruht und wird wieder an den Fahrdraht in 5,0 m gehoben; die Stromabnahme der Traktion setzt dazwischen aus.",
         12.0, False, [ck(0, "running", "smooth", {}), ck(1.0, "running", "smooth", {}), ck(4.0, "depot-pantograph-down", "linear"), ck(7.0, "depot-pantograph-down", "smooth"), ck(10.0, "running", "linear"), ck(12.0, "running", "linear")],
         [span(c, 0, 12.0) for c in CT], [phase("lower", 1.0, 4.0, "Pantograph lowers", "Stromabnehmer senkt"), phase("raise", 7.0, 10.0, "Pantograph raises", "Stromabnehmer hebt")],
         [event(4.0, "shutdown", "Traction off", "Traktion aus"), event(10.0, "ignition", "Wire contact restored", "Fahrdrahtkontakt hergestellt")],
         [track("operating:traction", [(0, 0.05), (1.0, 0.05), (2.0, 0), (9.0, 0), (10, 0.05), (12, 0.05)]), track("operating:hvac", [(0, 0.7), (4.0, 0.0), (10.0, 0.7), (12, 0.7)])]),
]
PEN = {500: pens(0.15, None, None, 0.13, 0.25), 200: pens(0.18, 0.13, None, 0.13, 0.25), 100: pens(0.25, 0.18, None, 0.13, 0.35), 50: pens(0.35, 0.25, 0.13, 0.18, 0.5)}
E_RANGE = (0.45, 9.45)
M_RANGE = (-6.0, -0.45)


def end_outline(arches, r=0.37):
    p = [(0.45, 0.28)]
    for xc in (BR - 0.9, BR + 0.9, BF - 0.9, BF + 0.9):
        if arches:
            p += [(xc - r, 0.28)] + arc_poly(xc, WY, r, 180, 0, 14)
    p += [(9.20, 0.28), (9.40, 0.40), (9.45, 0.55), (9.45, 1.50), (9.40, 1.65), (8.55, 3.0), (8.35, 3.40), (0.45, 3.40)]
    return p


def mid_outline():
    return [(-6.0, 0.28), (-0.45, 0.28), (-0.45, 3.40), (-6.0, 3.40)]


def wheelset(k):
    s = []
    names = [("f1", "ws-f1", BF - 0.9), ("f2", "ws-f2", BF + 0.9), ("r1", "ws-r1", BR - 0.9), ("r2", "ws-r2", BR + 0.9)]
    for nm, bn, xc in names:
        c = (xc, WY)
        if k == 500:
            s.append(Dw.ell("wheel-" + nm, bn, c, RW, regions=["wheelsets", "brake-discs", "traction-motors"], z=4, tone=0.2, material="steel-wheel"))
        elif k == 200:
            s.append(Dw.ell("wheel-" + nm, bn, c, RW, regions=["wheelsets", "traction-motors"], z=4, tone=0.2, material="steel-wheel"))
            s.append(Dw.ell("disc-" + nm, bn, c, 0.17, region="brake-discs", z=4.4, tone=0.55, material="cast-iron", role="detail"))
        else:
            s.append(Dw.ell("wheel-" + nm, bn, c, RW, region="wheelsets", z=4, tone=0.2, material="steel-wheel"))
            s.append(Dw.ell("disc-" + nm, bn, c, 0.20, region="brake-discs", z=4.4, tone=0.55, material="cast-iron", role="detail"))
            s.append(Dw.ell("hub-" + nm, bn, c, 0.07, z=4.8, tone=0.75, material="steel-wheel", role="detail")) if k == 50 else None
    if k <= 100:
        for nm, bn, xc in (("f", "bogie-f", BF), ("r", "bogie-r", BR)):
            s.append(Dw.rect("motor-" + nm, bn, xc - 0.45, WY - 0.02, xc + 0.45, WY + 0.30, region="traction-motors", z=3.0, tone=0.25, material="motor-steel", role="detail"))
    return s


def panes(k):
    if k == 500:
        return [Dw.poly("window-band-end", "body-end", [(0.9, 1.25), (8.3, 1.25), (8.45, 2.75), (0.9, 2.75)], region="glazing", z=6, tone=0.88, material="glass", role="glazing"),
                Dw.poly("window-band-mid", "body-mid", [(-5.85, 1.25), (-0.7, 1.25), (-0.7, 2.75), (-5.85, 2.75)], region="glazing", z=6, tone=0.88, material="glass", role="glazing")]
    out = []
    for i, (x0, x1) in enumerate(((0.90, 3.30), (5.00, 8.00)) if k == 100 else ((0.90, 3.30), (5.00, 6.55), (6.70, 8.00))):
        out.append(Dw.rect("pane-e%d" % (i + 1), "body-end", x0, 1.25, x1, 2.75, region="glazing", z=6, tone=0.88, material="glass", role="glazing"))
    out.append(Dw.poly("pane-cab", "body-end", [(8.15, 1.25), (8.40, 1.25), (8.50, 2.75), (8.15, 2.75)], region="glazing", z=6, tone=0.88, material="glass", role="glazing"))
    out.append(Dw.poly("windscreen", "body-end", [(9.40, 1.75), (8.70, 2.85), (8.62, 2.85), (8.62, 1.75)], region="glazing", z=6, tone=0.88, material="glass", role="glazing")) if False else None
    for i, (x0, x1) in enumerate(((-5.85, -4.25), (-2.55, -0.85))):
        out.append(Dw.rect("pane-m%d" % (i + 1), "body-mid", x0, 1.25, x1, 2.75, region="glazing", z=6, tone=0.88, material="glass", role="glazing"))
    return [x for x in out if x]


def doors(k):
    s = []
    for nm, (x0, x1) in DOORS.items():
        s.append(Dw.rect("leaf-" + nm[-2:], nm, x0, 0.28, x1, 2.70, region="doors", z=3, tone=0.6, material="paint"))
        if k <= 100:
            s.append(Dw.rect("leaf-glass-" + nm[-2:], nm, x0 + 0.12, 1.25, x1 - 0.12, 2.55, region="glazing", z=6, tone=0.88, material="glass", role="glazing"))
    return s


def pantograph(k):
    pen = {500: 0.13, 200: 0.18, 100: 0.3, 50: 0.7}[k]
    kn = (PB[0] + LA * math.cos(math.radians(AR[0])), PB[1] + LA * math.sin(math.radians(AR[0])))
    hd = (kn[0] + LA * math.cos(math.radians(AR[0] + AR[1])), kn[1] + LA * math.sin(math.radians(AR[0] + AR[1])))
    s = [
        Dw.stroke("pan-lower", "pan-lower", [PB, kn], pen, "pantograph", z=3, material="pan-alloy"),
        Dw.stroke("pan-upper", "pan-upper", [kn, hd], pen, "pantograph", z=3.1, material="pan-alloy"),
        Dw.cap("pan-head", "pan-head", (hd[0] - 0.25, hd[1]), (hd[0] + 0.25, hd[1]), 0.03 if k <= 200 else 0.06, region="pantograph", z=3.2, tone=0.2, material="carbon", role="body" if k == 500 else "detail"),
    ]
    s.append(Dw.rect("pan-base", "body-end", PB[0] - 0.3, 3.40, PB[0] + 0.3, PB[1], region="pantograph", z=3.3, tone=0.4, material="pan-alloy", role="body" if k >= 200 else "detail"))
    return s


def roofgear(k):
    s = [Dw.rect("resistor-box", "body-end", 4.5, 3.40, 6.5, 3.64, region="brake-resistor", z=2.5, tone=0.35, material="resistor-steel"),
         Dw.rect("hvac-box", "body-mid", -5.2, 3.40, -1.4, 3.66, region="hvac-unit", z=2.5, tone=0.7, material="paint")]
    if k == 50:
        for i in range(4):
            x = 4.8 + 0.4 * i
            s.append(Dw.stroke("louvre-r%d" % i, "body-end", [(x, 3.46), (x, 3.58)], 0.13, z=2.6, role="texture"))
        for i in range(5):
            x = -4.8 + 0.7 * i
            s.append(Dw.stroke("louvre-h%d" % i, "body-mid", [(x, 3.46), (x, 3.60)], 0.13, z=2.6, role="texture"))
    return s


def mk(k):
    s = wheelset(k)
    arch = k != 500
    s.append(Dw.poly("shell-end", "body-end", end_outline(arch, {200: 0.42, 100: 0.40, 50: 0.37}.get(k, 0.37)), regions=["body-panels", "roof"], z=2, tone=0.62, material="paint"))
    s.append(Dw.poly("shell-mid", "body-mid", mid_outline(), regions=["body-panels", "roof"], z=2, tone=0.62, material="paint"))
    s.append(Dw.poly("gangway", "gangway", [(-0.45, 0.45), (0.45, 0.45), (0.45, 3.25), (-0.45, 3.25)], region="gangway", z=2.2, tone=0.2, material="rubber"))
    s += panes(k) + doors(k) + pantograph(k) + roofgear(k)
    if k <= 100:
        s.append(Dw.poly("headlamp", "body-end", [(9.30, 0.85), (9.45, 0.90), (9.45, 1.10), (9.30, 1.05)], z=7, tone=0.9, material="glass", role="glazing"))
    return s


def dd(k):
    t = {
        500: ("Two body silhouettes (end module without arches and middle module) joined by one bellows block, wheel discs overlapping the sill, one window band per module, door leaves and a pantograph of two strokes and a head.", "Zwei Aufbausilhouetten (Endmodul ohne Radläufe und Mittelmodul), verbunden durch einen Balgblock, Radscheiben über der Schwelle, ein Fensterband je Modul, Türflügel und ein Stromabnehmer aus zwei Strichen und Kopf.", "Bodies, glass bands, door leaves, bellows, wheels, resistor, air-conditioning unit, pantograph.", "Aufbauten, Glasbänder, Türflügel, Balg, Räder, Widerstand, Klimagerät, Stromabnehmer.", "Wheelset, brake disc and motor share one wheel disc; panels and roof share the body shell.", "Radsatz, Bremsscheibe und Motor teilen sich eine Radscheibe; Bleche und Dach teilen die Aufbauhülle.", "Pillars, bogie frames, lamps, handrails, wipers.", "Pfosten, Drehgestellrahmen, Leuchten, Haltestangen, Wischer."),
        200: ("Body shells with open wheel arches 0.12 m (0.6 mm) clear of the tyres, individual panes separated by 0.25 m pillars, door leaves, bellows, roof boxes, pantograph and wheel discs with brake discs.", "Aufbauhüllen mit offenen Radläufen im Abstand von 0,12 m (0,6 mm) zu den Rädern, einzelne Scheiben mit 0,25 m breiten Pfosten, Türflügel, Balg, Dachkästen, Stromabnehmer und Radscheiben mit Bremsscheiben.", "Everything above 0.2 m.", "Alles über 0,2 m.", "Wheel and motor share one disc.", "Rad und Motor teilen sich eine Scheibe.", "Handles, lamps, wipers, bogie frames.", "Griffe, Leuchten, Wischer, Drehgestellrahmen."),
        100: ("Body shells with arches 0.10 m clear of the wheels, the side windows of the front half merged into one pane, door leaves with glass, bellows, roof boxes, pantograph, wheels with brake discs, motor blocks in the bogies and the head lamp.", "Aufbauhüllen mit Radläufen im Abstand von 0,10 m zu den Rädern, die Seitenfenster der vorderen Hälfte zu einer Scheibe zusammengefasst, Türflügel mit Glas, Balg, Dachkästen, Stromabnehmer, Räder mit Bremsscheiben, Motorblöcke in den Drehgestellen und der Scheinwerfer.", "Everything above 0.1 m.", "Alles über 0,1 m.", "Bogie frame and motor are one block.", "Drehgestellrahmen und Motor sind ein Block.", "Handrails, wipers, seals.", "Haltestangen, Wischer, Dichtungen."),
        50: ("Full side elevation with louvred roof boxes, pantograph with insulated base, door leaves with glass, bellows, motor blocks, wheels with brake discs and hubs, head lamp.", "Vollständige Seitenansicht mit Lamellen der Dachkästen, Stromabnehmer mit Isolatorfuß, Türflügel mit Glas, Balg, Motorblöcke, Räder mit Bremsscheiben und Naben, Scheinwerfer.", "All printable parts.", "Alle druckbaren Teile.", "Bogie frame and motor are one block.", "Drehgestellrahmen und Motor sind ein Block.", "Wipers, seals, handrails, bolts.", "Wischer, Dichtungen, Haltestangen, Schrauben."),
    }[k]
    return dec((t[0], t[1]), (t[2], t[3]), (t[4], t[5]), (t[6], t[7]),
               ("Pantograph arms 0.06 m wide are drawn as strokes of %s mm; lamellae 0.13 mm." % {500: "0.13 (real 0.12)", 200: "0.18 (real 0.3)", 100: "0.3 (real 0.6)", 50: "0.7 (real 1.2)"}[k], "Stromabnehmerarme von 0,06 m Breite werden als Striche von %s mm gezeichnet; Lamellen 0,13 mm." % {500: "0,13 (real 0,12)", 200: "0,18 (real 0,3)", 100: "0,3 (real 0,6)", 50: "0,7 (real 1,2)"}[k]),
               ("None; material is tone only.", "Keine; das Material zeigt sich nur über den Ton."),
               ("Body mid-tone, glass lightest, bellows and wheels dark, roof boxes distinct.", "Aufbau mittlerer Ton, Glas am hellsten, Balg und Räder dunkel, Dachkästen abgesetzt."),
               ("Merged shapes list their regions; the middle module is cut at its rear end (continuation not modelled).", "Zusammengefasste Formen führen ihre Regionen; das Mittelmodul ist am hinteren Ende abgeschnitten (Fortsetzung nicht modelliert)."))


reps = {str(k): rep(k, *r, PEN[k], [], mk(k), dd(k)) for k, r in ((500, (350, 1000)), (200, (150, 350)), (100, (75, 150)), (50, (20, 75)))}
plan = [
    poly("plan-end", "body-end", "body", 3, [(0.45, -1.2), (8.8, -1.2), (9.45, -0.7), (9.45, 0.7), (8.8, 1.2), (0.45, 1.2)], True, 0.6, True, material="paint"),
    poly("plan-mid", "body-end", "body", 3, [(-6.0, -1.2), (-0.45, -1.2), (-0.45, 1.2), (-6.0, 1.2)], True, 0.6, True, material="paint"),
    poly("plan-bellows", "body-end", "body", 4, [(-0.45, -1.0), (0.45, -1.0), (0.45, 1.0), (-0.45, 1.0)], True, 0.2, True, material="rubber"),
    poly("plan-pantograph", "body-end", "body", 5, [(2.0, -0.6), (3.2, -0.6), (3.2, 0.6), (2.0, 0.6)], False, 0.3, False, material="pan-alloy", pen=0.13),
]
symbols = [symbol("tram-plan", "Tram section plan symbol", "Straßenbahnabschnitt-Grundrisszeichen", "True-extent plan: end module, bellows, middle module and the collector frame.", "Grundriss in wahrer Ausdehnung: Endmodul, Balg, Mittelmodul und Stromabnehmerrahmen.", "true-extent", "model-m", [500, 200], pens(0.15, None, None, None, 0.25), [], [], plan, "Model metres: length 15.45 m, width 2.40 m, cut at the rear of the middle module.", "Modellmeter: Länge 15,45 m, Breite 2,40 m, am Ende des Mittelmoduls abgeschnitten.")]
ASSUME = [
    D("Heat at level 1: traction motors 12 kW (losses of 480 kW at 93 %, about a third leaves through the housing), friction brake discs 30 kW and brake resistor box 30 kW surface release (about 150 kW resistor duty of which 120 kW leaves with the cooling air), air conditioning 3 kW, wheels 0.6 kW, collector 0.3 kW. Emergency braking from 50 km/h: 0.5 * 20 t * 13.9 m/s squared = 1.9 MJ in 5 s (EN 13452-1). Authored estimates.", "Wärme bei Stufe 1: Fahrmotoren 12 kW (Verluste von 480 kW bei 93 %, etwa ein Drittel über das Gehäuse), Reibbremsscheiben 30 kW und Bremswiderstandskasten 30 kW Oberflächenabgabe (rund 150 kW Widerstandsleistung, davon 120 kW mit der Kühlluft), Klimaanlage 3 kW, Räder 0,6 kW, Schleifleiste 0,3 kW. Schnellbremsung aus 50 km/h: 0,5 * 20 t * (13,9 m/s)^2 = 1,9 MJ in 5 s (EN 13452-1). Selbst gesetzte Schätzwerte."),
    D("Resistor fan: 150 kW * 0.8 = 120 kW into 2.0 m3/s of air (rho 1.1 kg/m3, cp 1006 J/(kg K)) gives 55 K. Air-conditioning outlet: about 55 kW rejected heat of a 40 kW cooling load into 2.5 m3/s gives 20 K. Both scale linearly with their operating level.", "Widerstandslüfter: 150 kW * 0,8 = 120 kW in 2,0 m3/s Luft (rho 1,1 kg/m3, cp 1006 J/(kg K)) ergibt 55 K. Klimaauslass: etwa 55 kW abgeführte Wärme bei 40 kW Kühlleistung in 2,5 m3/s ergibt 20 K. Beide skalieren linear mit ihrer Betriebsstufe."),
    D("The asset is a section: end module with cab (9.0 m), articulation (0.9 m) and a suspended middle module cut after 5.55 m; the continuation is not modelled. The middle module hangs from the end module, which stands on its two bogies; the centre of mass lies between the bogies.", "Das Asset ist ein Ausschnitt: Endmodul mit Fahrerstand (9,0 m), Gelenk (0,9 m) und ein eingehängtes Mittelmodul, nach 5,55 m abgeschnitten; die Fortsetzung wird nicht modelliert. Das Mittelmodul hängt am Endmodul, das auf seinen beiden Drehgestellen steht; der Schwerpunkt liegt zwischen den Drehgestellen."),
]
STR = [("Rigid bodies on four unilateral wheel-rail contacts; the middle module hangs from the end module through the articulation, whose pitch is limited to 3 degrees.", "Starre Körper auf vier einseitigen Rad-Schiene-Kontakten; das Mittelmodul hängt über das Gelenk am Endmodul, dessen Nicken auf 3 Grad begrenzt ist."),
       ("Door leaves are projected plug-sliding doors; the pantograph is a two-arm linkage whose raised geometry (22 and 158 degrees, 1.9 m arms) puts the collector 5.0 m above the rail.", "Türflügel sind projizierte Schwenkschiebetüren; der Stromabnehmer ist ein Zweiarmgestänge, dessen angelegte Geometrie (22 und 158 Grad, 1,9-m-Arme) die Schleifleiste 5,0 m über der Schiene hält.")]
a = {
    "id": "tram", "category": "vehicles", "title": tx("Low-Floor Tram Section", "Niederflur-Straßenbahnabschnitt"),
    "summary": tx("A section of a 100 % low-floor articulated tram: a cab end module (9.0 m) with powered bogie and trailing bogie, the articulation bellows and a suspended middle module, with plug-sliding doors, a pantograph that raises and lowers, traction motors, brake resistor and discs, and roof air conditioning.", "Ein Abschnitt einer 100-%-Niederflur-Gelenkstraßenbahn: ein Endmodul mit Fahrerstand (9,0 m) mit angetriebenem Drehgestell und Laufdrehgestell, Gelenkbalg und ein eingehängtes Mittelmodul, mit Schwenkschiebetüren, hebe- und senkbarem Stromabnehmer, Fahrmotoren, Bremswiderstand und -scheiben sowie Dachklimaanlage."),
    "tags": ["vehicle", "tram", "rail", "low-floor", "articulated", "electric", "pantograph"],
    "provenance": provenance(["en-13803", "en-50163", "en-13452", "vdv-154", "ashrae-2021", "limpert-1999", "incropera-2011"],
        "Typical 100 % low-floor tram section (width 2.40 m, wheel diameter 0.60 m, floor 0.35 m, 750 V DC, 480 kW); dimensions, masses and heat data are authored class values (authored-estimate). Wheelsets roll with theta = -x / r (r 0.30 m). Pantograph raised: collector 5.0 m above the rail.",
        "Typischer Abschnitt einer 100-%-Niederflur-Straßenbahn (Breite 2,40 m, Raddurchmesser 0,60 m, Fußboden 0,35 m, 750 V Gleichstrom, 480 kW); Abmessungen, Massen und Wärmedaten sind selbst gesetzte Klassenwerte (authored-estimate). Die Radsätze rollen mit theta = -x / r (r 0,30 m). Stromabnehmer angelegt: Schleifleiste 5,0 m über der Schiene."),
    "dimensions": {"heightM": 5.0, "widthM": 15.45, "depthM": 2.4, "massKg": 20000},
    "materials": mats, "rig": {"kind": "mechanical", "root": "body-end", "bones": bones}, "anchors": anchors,
    "representations": reps, "symbols": symbols, "poses": poses, "clips": clips,
    "thermal": {"environment": dict(ENV), "operating": KEYS, "regions": REGIONS, "emitters": EM, "passengers": [], "assumptions": ASSUME},
    "structure": structure(STR),
    "presentation": {"defaultPose": "running", "defaultClip": "run-loop", "defaultScale": 200, "viewpoints": ["elevation"]},
    "exports": EXPORTS, "completion": "authored",
}
if __name__ == "__main__":
    a, probs = finish(REL, a)
