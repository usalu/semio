import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nw import *
D = tx

REL = "🚜️vehicles/electric-kick-scooter"
RW = 0.108
R = (-0.43, 0.108)
F = (0.43, 0.108)
H = (0.40, 0.30)
STEM_ANG = 98
STEM_LEN = 0.80
TOP = (H[0] + STEM_LEN * math.cos(math.radians(STEM_ANG)), H[1] + STEM_LEN * math.sin(math.radians(STEM_ANG)))
LOOP_TURNS = 6
LOOP_M = LOOP_TURNS * 2 * math.pi * RW
V = 5.56
T_LOOP = round(LOOP_M / V, 4)

bones = [
    bone("deck", None, "Deck with battery, controller and rear fender", "Trittbrett mit Akku, Steuerung und Hinterradschutzblech", 0, 0, 0, 0.8, FIXED, mass(6.6, -0.02, 0.12)),
    bone("fork", "deck", "Steering head and fork", "Lenkkopf und Gabel", H[0], H[1], 0, 0.2, REV(-12, 12), mass(0.9, 0.0, -0.1)),
    bone("stem", "fork", "Folding stem with handlebar", "Klappbarer Lenker mit Vorbau", 0, 0, STEM_ANG, STEM_LEN, REV(0, 90), mass(1.6, 0.4, 0, "uniform")),
    bone("wheel-f", "fork", "Front wheel with hub motor", "Vorderrad mit Nabenmotor", F[0] - H[0], F[1] - H[1], 0, RW, FREE, mass(2.4, ref="etrto-2023")),
    bone("wheel-r", "deck", "Rear wheel with disc", "Hinterrad mit Scheibe", R[0], R[1], 0, RW, FREE, mass(1.0, ref="etrto-2023")),
]
L = Loc(bones)
Dw = Draw(L)
anchors = [
    contact("contact-r", "deck", R[0], 0.0, "Rear tyre patch", "Hinterreifenaufstandsfläche", 0.03, 0.04),
    contact("contact-f", "fork", F[0] - H[0], -H[1], "Front tyre patch", "Vorderreifenaufstandsfläche", 0.03, 0.04),
    anchor("deck-stance", "deck", -0.05, 0.157, "load", "Standing point on the deck", "Standpunkt auf dem Trittbrett"),
]
mats = [
    material("anodised", "Anodised aluminium deck and stem", "Eloxiertes Aluminium von Trittbrett und Lenker", 0.8, 0.85, 2700, 150, "literature", "ashrae-2021", 900),
    material("rubber", "Tyre and grip rubber", "Reifen- und Griffgummi", 0.95, 0.92, 1150, 0.2, "literature", "wong-2008"),
    material("griptape", "Abrasive grip tape", "Griptape-Antirutschbelag", 0.9, 0.92, 1200, 0.3),
    material("plastic", "Fender and housing plastic", "Schutzblech- und Gehäusekunststoff", 0.9, 0.8, 1100, 0.2),
    material("steel-disc", "Brake disc steel", "Stahl der Bremsscheibe", 0.7, 0.7, 7800, 25, "literature", "limpert-1999"),
    material("motor-alloy", "Hub motor housing aluminium", "Aluminiumgehäuse des Nabenmotors", 0.6, 0.6, 2700, 150, "literature", "incropera-2011"),
]
KEYS = [
    opkey("drive", "Drive power", "Antriebsleistung", [(0, "Off", "Aus"), (0.05, "Standby", "Bereitschaft"), (0.4, "Cruising 20 km/h", "Reisefahrt 20 km/h"), (1, "Rated 250 W", "Nennleistung 250 W")]),
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.8, "About 20 km/h", "Etwa 20 km/h"), (1, "About 25 km/h", "Etwa 25 km/h")]),
    opkey("braking", "Disc braking", "Scheibenbremsen", [(0, "Not braking", "Nicht gebremst"), (0.3, "Light braking", "Leichte Bremsung"), (1, "Full stop from 25 km/h", "Vollbremsung aus 25 km/h")]),
    opkey("charging", "Battery charging", "Akkuladen", [(0, "Not charging", "Nicht ladend"), (1, "Charging at 63 W", "Laden mit 63 W")]),
]
anod = lambda t, c: steel_layer(t, c, "anodised")
REGIONS = [
    region("deck-battery", "Deck and battery enclosure", "Trittbrett und Akkugehäuse", "surface", "operating", "anodised", 0.26, 0.2, "horizontal-plate-up", 0.8, 4500, [anod(0.002, 4500)], ("drive", 12), ref="iec-62133"),
    region("deck-grip", "Grip tape on the deck", "Griptape auf dem Trittbrett", "surface", "passive", "griptape", 0.08, 0.15, "horizontal-plate-up", 0.95, 1500, [layer("tape", "Abrasive tape 1.5 mm", "Antirutschbelag 1,5 mm", "polymer", "griptape", 0.0015, 0.0015 / 0.3, 1500)]),
    region("bms-controller", "Controller and charging circuit inside the deck", "Steuerung und Ladeschaltung im Trittbrett", "surface", "operating", "plastic", 0.02, 0.1, "horizontal-plate-down", 0.1, 3000, [], ("charging", 6), hidden=None) if False else None,
]
bms = region("bms-controller", "Controller and charging circuit inside the deck", "Steuerung und Ladeschaltung im Trittbrett", "surface", "operating", "plastic", 0.02, 0.1, "horizontal-plate-down", 0.1, 3000, [], ("charging", 6))
bms["hidden"] = tx("Enclosed inside the deck, not visible in elevation.", "Im Trittbrett eingeschlossen, in der Ansicht nicht sichtbar.")
REGIONS = [REGIONS[0], REGIONS[1], bms,
    region("hub-motor", "Front hub motor", "Vorderrad-Nabenmotor", "engine", "operating", "motor-alloy", 0.04, 0.1, "cylinder", 0.4, 18000, [], ("drive", 55), ref="iec-62133"),
    region("stem-bars", "Stem, handlebar and fork", "Lenkstange, Lenker und Gabel", "surface", "passive", "anodised", 0.14, 0.03, "cylinder", 0.6, 4500, [anod(0.002, 4500)], ref="ashrae-2021"),
    region("fenders", "Rear fender and front mudguard", "Hinterradschutzblech und Vorderradabdeckung", "surface", "passive", "plastic", 0.05, 0.1, "cylinder", 0.6, 1800, [layer("shell", "Plastic shell 2 mm", "Kunststoffschale 2 mm", "polymer", "plastic", 0.002, 0.002 / 0.2, 1800)]),
    region("tyres", "Tyres 8.5 inch", "Reifen 8,5 Zoll", "tyre", "operating", "rubber", 0.14, 0.05, "cylinder", 0.45, 8000, [layer("tread", "Rubber tread and carcass", "Gummilauffläche und Karkasse", "polymer", "rubber", 0.006, 0.006 / 0.2, 8000)], ("rolling", 40), ref="wong-2008"),
    region("brake-disc", "Rear brake disc", "Hintere Bremsscheibe", "surface", "operating", "steel-disc", 0.012, 0.12, "vertical-plate", 0.3, 5600, [], ("braking", 890), ref="limpert-1999"),
]
EM = []
DEF = {"deck-battery": (1, 0.6), "deck-grip": (1, 1), "bms-controller": (0.1, 0), "hub-motor": (0.5, 0.2), "stem-bars": (1, 0.6), "fenders": (1, 0.6), "tyres": (1, 0.2), "brake-disc": (0.7, 0.1)}


def th(drive, roll, brake, charge, contacts=None, regs=None):
    return binding(DEF, regs, {"drive": drive, "rolling": roll, "braking": brake, "charging": charge}, {}, contacts)


CT = {"tyres": (0.003, 30)}
FOLD = 72
poses = [
    pose("riding-cruise", "Riding at 20 km/h", "Fahrt mit 20 km/h", "Stem upright, both wheels on the ground, motor at cruising power; a rider would stand on the deck (not included).", "Lenker aufrecht, beide Räder am Boden, Motor mit Reiseleistung; ein Fahrer stünde auf dem Trittbrett (nicht enthalten).",
         {}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.4, 0.8, 0, 0, CT)),
    pose("braking-stop", "Braking to a stop", "Bremsen bis zum Stillstand", "Disc brake at full effort after 25 km/h, motor off, steering turned 4 degrees.", "Scheibenbremse mit voller Wirkung nach 25 km/h, Motor aus, Lenkung um 4 Grad eingeschlagen.",
         {"fork": {"angleDeg": 4}}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0, 0.2, 1, 0, CT)),
    pose("standby-unfolded", "Unfolded, switched on", "Aufgeklappt, eingeschaltet", "Standing upright, switched on in standby.", "Aufrecht stehend, eingeschaltet im Standby.",
         {}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.05, 0, 0, 0, CT)),
    pose("folded-charging", "Folded and charging", "Zusammengeklappt und ladend", "Stem folded back along the deck over the rear fender, charging at 63 W indoors.", "Lenker über das Trittbrett nach hinten auf das Schutzblech geklappt, Laden mit 63 W in Innenräumen.",
         {"stem": {"angleDeg": FOLD}}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0, 0, 0, 1, CT, {"hub-motor": {"exposedFraction": 0.5, "sunlitFraction": 0}, "deck-grip": {"exposedFraction": 1, "sunlitFraction": 0}, "deck-battery": {"exposedFraction": 1, "sunlitFraction": 0}, "stem-bars": {"exposedFraction": 1, "sunlitFraction": 0}, "fenders": {"exposedFraction": 1, "sunlitFraction": 0}, "tyres": {"exposedFraction": 1, "sunlitFraction": 0}, "brake-disc": {"exposedFraction": 0.7, "sunlitFraction": 0}})),
]
for name, frac in (("loop-start", 0.0), ("loop-mid", 0.5), ("loop-end", 1.0)):
    poses.append(pose(name, "Ride loop " + name.split("-")[1], "Fahrschleife " + name.split("-")[1], "Technical key of the ride loop: wheels have turned theta = -x / r.", "Technische Schlüsselstellung der Fahrschleife: die Räder haben theta = -x / r gedreht.",
                      {"wheel-f": {"angleDeg": wheel_turn(LOOP_M * frac, RW)}, "wheel-r": {"angleDeg": wheel_turn(LOOP_M * frac, RW)}}, {"x": round(LOOP_M * frac, 4), "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.4, 0.8, 0, 0, CT), key=True))
clips = [
    clip("ride-loop", "Ride loop at 20 km/h", "Fahrschleife mit 20 km/h", "Six wheel turns (%.2f m) per cycle at 5.56 m/s: both wheels roll with theta = -x / r and the root advances one cycle length every loop." % LOOP_M, "Sechs Radumdrehungen (%.2f m) pro Zyklus mit 5,56 m/s: beide Räder rollen mit theta = -x / r und die Wurzel rückt je Schleife um eine Zykluslänge vor." % LOOP_M,
         T_LOOP, True, [ck(0, "loop-start"), ck(round(T_LOOP / 2, 4), "loop-mid"), ck(T_LOOP, "loop-end")], [span("contact-f", 0, T_LOOP), span("contact-r", 0, T_LOOP)], [phase("cruise", 0, T_LOOP, "Steady cruise", "Gleichmäßige Fahrt")],
         [event(0, "motion", "Cycle start", "Zyklusbeginn")], [track("operating:drive", [(0, 0.4), (T_LOOP, 0.4)]), track("operating:rolling", [(0, 0.8), (T_LOOP, 0.8)])]),
    clip("fold-and-charge", "Unfold state change: fold and plug in", "Zustandswechsel: zusammenklappen und anstecken", "From switched-on standby the stem folds back over the deck in 1.5 s, the motor switches off and the charger starts.", "Aus dem eingeschalteten Standby klappt der Lenker in 1,5 s über das Trittbrett, der Motor schaltet ab und das Ladegerät startet.",
         5.0, False, [ck(0, "standby-unfolded", "smooth"), ck(1.0, "standby-unfolded", "smooth"), ck(2.5, "folded-charging", "linear"), ck(5.0, "folded-charging", "linear")],
         [span("contact-f", 0, 5.0), span("contact-r", 0, 5.0)], [phase("fold", 1.0, 2.5, "Stem folds", "Lenker klappt")], [event(2.5, "shutdown", "Drive off, folded", "Antrieb aus, zusammengeklappt"), event(3.0, "thermal", "Charging starts", "Laden beginnt")],
         [track("operating:drive", [(0, 0.05), (2.5, 0), (5.0, 0)]), track("operating:charging", [(0, 0), (3.0, 0), (3.6, 1), (5.0, 1)])]),
]
PEN = {200: pens(0.18, 0.13, None, 0.13, 0.25), 100: pens(0.18, 0.13, None, 0.13, 0.35), 50: pens(0.25, 0.18, 0.13, 0.13, 0.5)}


def wheels(kind):
    s = []
    for nm, bn, c in (("f", "wheel-f", F), ("r", "wheel-r", R)):
        hubreg = "hub-motor" if nm == "f" else "brake-disc"
        if kind == 50:
            s.append(Dw.ell("tyre-" + nm, bn, c, 0.108, region="tyres", z=4, tone=0.1, material="rubber"))
            s.append(Dw.ell("rim-" + nm, bn, c, 0.075, region=hubreg if nm == "f" else "tyres", z=4.2, tone=0.75, material="motor-alloy" if nm == "f" else "rubber"))
            s.append(Dw.ell("hub-" + nm, bn, c, 0.035 if nm == "f" else 0.03, region=hubreg, z=4.8, tone=0.45, material="motor-alloy" if nm == "f" else "steel-disc"))
        elif kind == 100:
            s.append(Dw.ell("tyre-" + nm, bn, c, 0.108, region="tyres", z=4, tone=0.1, material="rubber"))
            s.append(Dw.ell("rim-" + nm, bn, c, 0.066, region=hubreg, z=4.2, tone=0.75, material="motor-alloy"))
        else:
            s.append(Dw.ell("wheel-" + nm, bn, c, 0.108, regions=["tyres", hubreg], z=4, tone=0.1, material="rubber"))
    return s


DECK = [(-0.40, 0.095), (0.37, 0.095), (0.38, 0.12), (0.37, 0.145), (-0.40, 0.145), (-0.41, 0.12)]


def rep50():
    s = wheels(50)
    s += [
        Dw.poly("deck", "deck", DECK, region="deck-battery", z=3, tone=0.3, material="anodised"),
        Dw.poly("grip-tape", "deck", [(-0.36, 0.135), (0.30, 0.135), (0.30, 0.158), (-0.36, 0.158)], region="deck-grip", z=3.2, tone=0.12, material="griptape"),
        Dw.band("fender-r", "deck", R, 0.114, 0.126, 15, 170, region="fenders", material="plastic"),
        Dw.cap("fork-blade", "fork", H, F, 0.014, region="stem-bars", z=5, tone=0.4, material="anodised"),
        Dw.rect("head-block", "fork", H[0] - 0.035, H[1] - 0.05, H[0] + 0.035, H[1] + 0.03, region="stem-bars", z=3.6, tone=0.3, material="anodised"),
        Dw.cap("stem", "stem", H, TOP, 0.019, 0.015, region="stem-bars", z=3.5, tone=0.4, material="anodised"),
        Dw.ell("grip-end", "stem", TOP, 0.03, region="stem-bars", z=3.7, tone=0.1, material="rubber"),
        Dw.cap("brake-lever", "stem", (TOP[0] + 0.01, TOP[1] + 0.005), (TOP[0] + 0.1, TOP[1] - 0.01), 0.008, region="stem-bars", z=3.6, tone=0.2, material="anodised"),
    ]
    return s


def rep100():
    s = wheels(100)
    s += [
        Dw.poly("deck", "deck", DECK, regions=["deck-battery", "deck-grip", "bms-controller"], z=3, tone=0.3, material="anodised"),
        Dw.stroke("fork-blade", "fork", [H, F], 0.3, "stem-bars", z=5),
        Dw.stroke("stem", "stem", [H, TOP], 0.35, "stem-bars", z=3.5),
        Dw.ell("grip-end", "stem", TOP, 0.03, region="stem-bars", z=3.7, tone=0.1, material="rubber"),
        Dw.stroke("fender-r", "deck", arc_poly(R[0], R[1], 0.112, 15, 170), 0.13, "fenders", z=3.6),
    ]
    return s


def rep200():
    s = wheels(200)
    s += [
        Dw.poly("deck", "deck", DECK, regions=["deck-battery", "deck-grip", "bms-controller", "fenders"], z=3, tone=0.3, material="anodised"),
        Dw.stroke("fork-stem", "stem", [H, TOP], 0.18, "stem-bars", z=3.5),
        Dw.stroke("fork-blade", "fork", [H, F], 0.16, "stem-bars", z=5),
        Dw.ell("grip-end", "stem", TOP, 0.034, region="stem-bars", z=3.7, tone=0.1, material="rubber"),
    ]
    return s


def sub(x, de): return (x, de)


reps = {
    "500": unavailable(500, "At 1:500 the scooter is 2.2 mm long: the 0.216 m wheels are 0.43 mm discs, the 0.035 m stem 0.07 mm and the 0.05 m deck 0.1 mm, all below the printable feature size, and a composed silhouette would be an unreadable blob.", "Bei 1:500 ist der Roller 2,2 mm lang: die 0,216 m großen Räder sind 0,43-mm-Scheiben, der 0,035 m dicke Lenker 0,07 mm und das 0,05 m dicke Trittbrett 0,1 mm, alles unter der druckbaren Einzelheitsgröße; eine zusammengesetzte Silhouette wäre ein unlesbarer Fleck.",
                       "Stem tube diameter and deck thickness", "Lenkerrohrdurchmesser und Trittbretthöhe", 0.035, [{"kind": "scale", "denominator": 200}, {"kind": "symbol", "symbol": "kick-scooter-plan"}]),
    "200": rep(200, 150, 350, PEN[200], [], rep200(), dec(
        ("Two solid wheel discs, a thin deck bar, a stem stroke leaning back and a grip dot; the folded stem lies along the deck.", "Zwei volle Radscheiben, ein schmaler Trittbrettbalken, ein zurückgeneigter Lenkerstrich und ein Griffpunkt; der geklappte Lenker liegt am Trittbrett."),
         ("Wheels, deck, fork, stem and grip.", "Räder, Trittbrett, Gabel, Lenker und Griff."),
         ("Grip tape, battery enclosure, controller and fender are one deck bar; tyre and hub motor or brake disc are one disc per wheel.", "Griptape, Akkugehäuse, Steuerung und Schutzblech sind ein Trittbrettbalken; Reifen und Nabenmotor beziehungsweise Bremsscheibe sind eine Scheibe je Rad."),
         ("Brake lever, latch, mudguard, rim, disc detail.", "Bremshebel, Verschluss, Abdeckung, Felge, Scheibendetail."),
         ("The 0.035 m stem is drawn 0.18 mm wide (real 0.175 mm).", "Der 0,035 m dicke Lenker ist 0,18 mm breit gezeichnet (real 0,175 mm)."),
         ("None.", "Keine."), ("Dark wheel discs, mid-tone deck, one stroke weight for stem and fork.", "Dunkle Radscheiben, mitteltoniges Trittbrett, ein Strichstärke für Lenker und Gabel."),
         ("Wheel discs paint tyres with hub motor (front) or brake disc (rear) as area-weighted mean; the deck paints battery, grip and fender merged.", "Radscheiben zeigen Reifen mit Nabenmotor (vorn) beziehungsweise Bremsscheibe (hinten) als flächengewichtetes Mittel; das Trittbrett zeigt Akku, Griptape und Schutzblech vereint."))),
    "100": rep(100, 75, 150, PEN[100], [], rep100(), dec(
        ("Tyre rings with hub discs, deck block, rear fender arc, fork and stem strokes at real width and the grip end.", "Reifenringe mit Nabenscheiben, Trittbrettblock, Hinterradschutzblech-Bogen, Gabel- und Lenkerstriche in Realbreite und das Griffende."),
         ("Wheels, hub motor, brake disc, deck, fender, fork, stem, grip.", "Räder, Nabenmotor, Bremsscheibe, Trittbrett, Schutzblech, Gabel, Lenker, Griff."),
         ("Deck, grip tape and controller are one block.", "Trittbrett, Griptape und Steuerung sind ein Block."),
         ("Spokes or rim holes, lever, latch, tread.", "Speichen oder Felgenlöcher, Hebel, Verschluss, Profil."),
         ("Fender stroke 0.13 mm (real 0.1 mm); stem 0.35 mm (real 0.35 mm).", "Schutzblechstrich 0,13 mm (real 0,1 mm); Lenker 0,35 mm (real 0,35 mm)."),
         ("None.", "Keine."), ("Dark tyres, light hubs, mid-tone deck.", "Dunkle Reifen, helle Naben, mitteltoniges Trittbrett."),
         ("Controller, grip tape and battery share the deck block; fender is its own arc.", "Steuerung, Griptape und Akku teilen den Trittbrettblock; das Schutzblech ist ein eigener Bogen."))),
    "50": rep(50, 20, 75, PEN[50], [], rep50(), dec(
        ("Full side elevation: tyre and rim rings, hub motor, brake rotor, deck with grip tape, fender, fork, head block, folding stem, grip end, brake lever and front mudguard.", "Vollständige Seitenansicht: Reifen- und Felgenringe, Nabenmotor, Bremsscheibe, Trittbrett mit Griptape, Schutzblech, Gabel, Lenkkopf, Klappvorbau, Griffende, Bremshebel und Vorderradabdeckung."),
         ("All printable construction parts.", "Alle druckbaren Konstruktionsteile."),
         ("Controller sits inside the deck (hidden region).", "Die Steuerung sitzt im Trittbrett (verdeckte Region)."),
         ("Cables, bolts, lights, tyre profile.", "Kabel, Schrauben, Leuchten, Reifenprofil."),
         ("Grip tape drawn 0.013 m (real 0.0015 m) so it stays 0.26 mm.", "Griptape 0,013 m gezeichnet (real 0,0015 m), damit es 0,26 mm bleibt."),
         ("None.", "Keine."), ("Dark tyres, light rim and motor, mid-tone deck, near black grip tape.", "Dunkle Reifen, helle Felge und Motor, mitteltoniges Trittbrett, fast schwarzes Griptape."),
         ("Each visible region has its own shape; the controller is hidden.", "Jede sichtbare Region hat eine eigene Form; die Steuerung ist verdeckt."))),
}
plan = [
    poly("plan-deck", "deck", "body", 3, [(-0.40, -0.085), (0.37, -0.085), (0.37, 0.085), (-0.40, 0.085)], True, 0.4, True, material="anodised"),
    poly("plan-wheel-f", "deck", "body", 3, [(0.322, -0.025), (0.538, -0.025), (0.538, 0.025), (0.322, 0.025)], True, 0.1, True, material="rubber"),
    poly("plan-wheel-r", "deck", "body", 3, [(-0.538, -0.025), (-0.322, -0.025), (-0.322, 0.025), (-0.538, 0.025)], True, 0.1, True, material="rubber"),
    poly("plan-stem", "deck", "body", 4, [(0.40, 0), (0.29, 0)], False, 0.4, False, material="anodised", pen=0.18),
    poly("plan-bars", "deck", "body", 4, [(0.29, -0.215), (0.29, 0.215)], False, 0.4, False, material="anodised", pen=0.18),
]
symbols = [symbol("kick-scooter-plan", "Kick scooter plan symbol", "Tretroller-Grundrisszeichen", "True-extent plan of the e-scooter: deck, both wheels edge-on, stem and handlebar width.", "Grundriss des E-Rollers in wahrer Ausdehnung: Trittbrett, beide Räder von der Kante, Lenker und Lenkerbreite.", "true-extent", "model-m", [500, 200],
                  pens(0.15, None, None, None, 0.25), [], [], plan, "Model metres: length 1.08 m, handlebar width 0.43 m, deck width 0.17 m.", "Modellmeter: Länge 1,08 m, Lenkerbreite 0,43 m, Trittbrettbreite 0,17 m.")]
ASSUME = [
    D("Heat at level 1: hub motor 55 W (250 W output at 82 % efficiency), battery 12 W (5 % of 250 W discharge, IEC 62133 class lithium pack), brake disc 890 W (2.2 kJ of a 92 kg scooter with rider from 25 km/h in 2.5 s), tyres 40 W (Crr 0.01), charging circuit 6 W (10 % of 63 W).", "Wärme bei Stufe 1: Nabenmotor 55 W (250 W Abgabe bei 82 % Wirkungsgrad), Akku 12 W (5 % von 250 W Entladung, Lithiumpack nach IEC 62133), Bremsscheibe 890 W (2,2 kJ eines 92-kg-Rollers mit Fahrer aus 25 km/h in 2,5 s), Reifen 40 W (Crr 0,01), Ladeschaltung 6 W (10 % von 63 W)."),
    D("The rider is not part of the asset; the levels describe an 80 kg rider as scenario.", "Der Fahrer gehört nicht zum Asset; die Stufen beschreiben als Szenario einen 80-kg-Fahrer."),
]
STR = [("Rigid bodies on two unilateral ground contacts; the rider's weight acts at the standing point and is not included in the asset mass.", "Starre Körper auf zwei einseitigen Bodenkontakten; das Gewicht des Fahrers wirkt am Standpunkt und ist nicht in der Asset-Masse enthalten."),
       ("The stem folds about the head hinge by 82 degrees; steering is a small in-plane rotation of the fork about the head.", "Der Lenker klappt um das Kopfgelenk um 82 Grad; Lenken ist eine kleine ebene Drehung der Gabel um den Kopf.")]
a = {
    "id": "electric-kick-scooter", "category": "vehicles", "title": tx("Electric Kick Scooter", "Elektro-Tretroller"),
    "summary": tx("A riderless folding electric kick scooter (8.5 inch wheels, 250 W front hub motor, deck battery, rear disc brake): riding and folded states, rolling wheels, hub motor, battery, brake and charging heat.", "Ein klappbarer Elektro-Tretroller ohne Fahrer (8,5-Zoll-Räder, 250-W-Nabenmotor vorn, Akku im Trittbrett, hintere Scheibenbremse): Fahr- und Klappzustand, rollende Räder, Wärme von Nabenmotor, Akku, Bremse und Laden."),
    "tags": ["vehicle", "scooter", "electric", "micromobility", "folding", "wheeled"],
    "provenance": provenance(["etrto-2023", "iec-62133", "limpert-1999", "wong-2008", "ashrae-2021", "incropera-2011"],
        "Typical adult folding e-scooter (wheelbase 0.86 m, 8.5 inch tyres, 12.5 kg); dimensions and power data are authored class values (authored-estimate). Wheels roll with theta = -x / r (r 0.108 m). The rider is excluded.",
        "Typischer klappbarer Erwachsenen-E-Roller (Radstand 0,86 m, 8,5-Zoll-Reifen, 12,5 kg); Abmessungen und Leistungsdaten sind selbst gesetzte Klassenwerte (authored-estimate). Die Räder rollen mit theta = -x / r (r 0,108 m). Der Fahrer ist ausgeschlossen."),
    "dimensions": {"heightM": 1.11, "widthM": 1.09, "depthM": 0.43, "massKg": 12.5},
    "materials": mats, "rig": {"kind": "mechanical", "root": "deck", "bones": bones}, "anchors": anchors,
    "representations": reps, "symbols": symbols, "poses": poses, "clips": clips,
    "thermal": {"environment": dict(ENV), "operating": KEYS, "regions": REGIONS, "emitters": EM, "passengers": [], "assumptions": ASSUME},
    "structure": structure(STR),
    "presentation": {"defaultPose": "riding-cruise", "defaultClip": "ride-loop", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": EXPORTS, "completion": "authored",
}

if __name__ == "__main__":
    a, probs = finish(REL, a)
