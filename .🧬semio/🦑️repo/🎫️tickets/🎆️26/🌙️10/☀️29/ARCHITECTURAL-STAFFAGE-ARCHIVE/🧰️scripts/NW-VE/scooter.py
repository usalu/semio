import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nw import *
D = tx

REL = "🚜️vehicles/motor-scooter"
RW = 0.23
R = (-0.67, 0.23)
F = (0.67, 0.23)
HEAD = (0.42, 0.98)
PIV = (-0.20, 0.30)
SEAT_H = (-0.22, 0.745)
STAND_H = (-0.10, 0.30)
STAND_ANG = 178
STAND_LEN = 0.335
STAND_DOWN = 84
LOOP_TURNS = 4
LOOP_M = LOOP_TURNS * 2 * math.pi * RW
V = 8.33
T_LOOP = round(LOOP_M / V, 4)

bones = [
    bone("frame", None, "Monocoque body, floor, leg shield and tank", "Selbsttragende Karosserie, Boden, Beinschild und Tank", 0, 0, 0, 1.86, FIXED, mass(50, 0.0, 0.55)),
    bone("steer", "frame", "Handlebar, headset and fork", "Lenker, Steuerkopf und Gabel", HEAD[0], HEAD[1], 0, 0.8, REV(-12, 12), mass(13, 0.1, -0.2)),
    bone("wheel-f", "steer", "Front wheel with disc", "Vorderrad mit Scheibe", F[0] - HEAD[0], F[1] - HEAD[1], 0, RW, FREE, mass(8.5, ref="etrto-2023")),
    bone("swing", "frame", "Engine unit with swing arm and muffler", "Motoreinheit mit Schwinge und Schalldämpfer", PIV[0], PIV[1], 0, 0.55, REV(-8, 8), mass(41, -0.3, 0.1)),
    bone("wheel-r", "swing", "Rear wheel with drum", "Hinterrad mit Trommel", R[0] - PIV[0], R[1] - PIV[1], 0, RW, FREE, mass(9.5, ref="etrto-2023")),
    bone("seat", "frame", "Hinged seat", "Klappbare Sitzbank", SEAT_H[0], SEAT_H[1], 180, 0.58, REV(-70, 0), mass(4.5, 0.29, 0.03, "uniform")),
    bone("stand", "frame", "Centre stand", "Hauptständer", STAND_H[0], STAND_H[1], STAND_ANG, STAND_LEN, REV(0, 90), mass(3.5, 0.17, 0, "uniform")),
]
L = Loc(bones)
Dw = Draw(L)
TAIL = (-0.93, 0.46)
anchors = [
    contact("contact-f", "steer", F[0] - HEAD[0], -HEAD[1], "Front tyre patch", "Vorderreifenaufstandsfläche", 0.09, 0.11),
    contact("contact-r", "swing", R[0] - PIV[0], R[1] - PIV[1] - RW, "Rear tyre patch", "Hinterreifenaufstandsfläche", 0.10, 0.12),
    contact("stand-foot", "stand", STAND_LEN, 0, "Stand foot", "Ständerfuß", 0.06, 0.2, mu=0.6),
    anchor("tailpipe", "swing", TAIL[0] - PIV[0], TAIL[1] - PIV[1], "emitter", "Tailpipe mouth", "Auspuffmündung"),
    anchor("seat-storage", "frame", -0.45, 0.62, "load", "Under-seat storage load point", "Lastpunkt Staufach unter der Sitzbank"),
    anchor("rider-seat", "seat", 0.30, 0.06, "load", "Rider seat point", "Fahrersitzpunkt"),
]
mats = [
    material("paint", "Painted sheet steel body", "Lackierte Stahlblechkarosserie", 0.9, 0.5, 7850, 50, "literature", "ashrae-2021", 490),
    material("rubber", "Tyre rubber", "Reifengummi", 0.95, 0.92, 1150, 0.2, "literature", "wong-2008"),
    material("alloy", "Cast aluminium rim", "Aluminium-Gussfelge", 0.3, 0.4, 2700, 150, "literature", "ashrae-2021"),
    material("seat-vinyl", "Vinyl seat cover", "Kunstledersitzbezug", 0.9, 0.85, 1100, 0.2),
    material("cast-iron", "Cast iron brake disc", "Graugussbremsscheibe", 0.65, 0.7, 7200, 46, "literature", "limpert-1999"),
    material("exhaust-steel", "Oxidised exhaust steel", "Oxidierter Abgasstahl", 0.7, 0.8, 7800, 25, "literature", "heywood-2018"),
    material("engine-metal", "Engine and CVT cover casting", "Motor- und Variatorgehäuse aus Guss", 0.8, 0.8, 2700, 120, "authored-estimate", "heywood-2018"),
    material("glass", "Lamp lens polycarbonate", "Streuscheibe Polycarbonat", 0.9, 0.2, 1200, 0.2),
]
KEYS = [
    opkey("engine-load", "Engine load", "Motorlast", [(0, "Engine off", "Motor aus"), (0.2, "Idle", "Leerlauf"), (0.5, "Cruising 30 km/h", "Reisefahrt 30 km/h"), (1, "Full load", "Volllast")]),
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.6, "About 30 km/h", "Etwa 30 km/h"), (1, "About 50 km/h", "Etwa 50 km/h")]),
    opkey("braking", "Braking", "Bremsen", [(0, "Not braking", "Nicht gebremst"), (0.3, "Light braking", "Leichte Bremsung"), (1, "Full stop from 50 km/h", "Vollbremsung aus 50 km/h")]),
]
sheet = lambda: steel_layer(0.0008, 3100)
paintl = layer("paint", "Paint film", "Lackschicht", "polymer", "paint", 0.00008, 0.00008 / 0.3, 150)
REGIONS = [
    region("body-panels", "Body panels, floor and leg shield", "Karosserieblech, Boden und Beinschild", "surface", "passive", "paint", 2.1, 0.8, "vertical-plate", 0.5, 6000, [sheet(), paintl], ref="ashrae-2021"),
    region("steer-assembly", "Handlebar cowl, headset and fork", "Lenkerverkleidung, Steuerkopf und Gabel", "surface", "passive", "paint", 0.6, 0.3, "cylinder", 0.6, 5000, [sheet(), paintl], ref="ashrae-2021"),
    region("headlamp", "Headlamp lens", "Scheinwerferglas", "glazing", "passive", "glass", 0.03, 0.15, "vertical-plate", 0.6, 2500, [layer("lens", "Polycarbonate 3 mm", "Polycarbonat 3 mm", "polymer", "glass", 0.003, 0.003 / 0.2, 2500)]),
    region("seat", "Seat", "Sitzbank", "surface", "passive", "seat-vinyl", 0.35, 0.5, "horizontal-plate-up", 0.95, 3500, [layer("cover", "Vinyl over foam 30 mm", "Kunstleder über Schaum 30 mm", "polymer", "seat-vinyl", 0.03, 0.03 / 0.05, 3500)]),
    region("engine", "Engine and variator cover", "Motor und Variatordeckel", "engine", "operating", "engine-metal", 0.35, 0.3, "cylinder", 0.2, 9000, [], ("engine-load", 1500), ref="heywood-2018"),
    region("exhaust-system", "Exhaust pipe and silencer", "Abgasrohr und Schalldämpfer", "exhaust", "operating", "exhaust-steel", 0.35, 0.14, "cylinder", 0.3, 8500, [layer("pipe", "Steel pipe 1.5 mm", "Stahlrohr 1,5 mm", "metal", "exhaust-steel", 0.0015, 0.0015 / 45, 8500)], ("engine-load", 1000), ref="heywood-2018"),
    region("tyres", "Tyres 110/70-12", "Reifen 110/70-12", "tyre", "operating", "rubber", 0.9, 0.2, "cylinder", 0.4, 15000, [layer("tread", "Rubber tread and carcass", "Gummilauffläche und Karkasse", "polymer", "rubber", 0.01, 0.01 / 0.2, 15000)], ("rolling", 380), ref="wong-2008"),
    region("brake-disc", "Front brake disc and rear drum", "Vordere Bremsscheibe und hintere Trommel", "surface", "operating", "cast-iron", 0.1, 0.2, "vertical-plate", 0.2, 11000, [], ("braking", 4600), ref="limpert-1999"),
]
EM = [emitter("tailpipe", "exhaust", "Exhaust jet", "Abgasstrahl", "tailpipe", 185, 0.035, "engine-load", 0.0132, 280, 0.073, ref="heywood-2018")]
DEF = {"body-panels": (1, 0.5), "steer-assembly": (1, 0.7), "headlamp": (1, 0.6), "seat": (1, 1), "engine": (0.5, 0.1), "exhaust-system": (0.9, 0.1), "tyres": (1, 0.15), "brake-disc": (0.6, 0.05)}


def th(eng, roll, brake, contacts=None, regs=None):
    return binding(DEF, regs, {"engine-load": eng, "rolling": roll, "braking": brake}, {"tailpipe": 1 if eng > 0 else 0}, contacts)


CT = {"tyres": (0.008, 30)}
CS = {"tyres": (0.004, 30)}


def solve_pitch(extra):
    a = {"rig": {"bones": bones}}
    best = None
    for k in range(-400, 400):
        p = k / 100
        W = world(a, {"bones": {**extra, "stand": {"angleDeg": STAND_DOWN}}, "root": {"angleDeg": p}})
        f = ap(W["steer"], (F[0] - HEAD[0], -HEAD[1]))[1]
        s = ap(W["stand"], (STAND_LEN, 0))[1]
        if best is None or abs(f - s) < best[0]:
            best = (abs(f - s), p)
    return best[1]


PITCH = solve_pitch({})
print("pitch", PITCH)
poses = [
    pose("parked-on-stand", "Parked on the centre stand", "Auf dem Hauptständer geparkt", "Engine off and cold, centre stand down; the rear wheel is lifted off the ground; seat closed.", "Motor aus und kalt, Hauptständer ausgeklappt; das Hinterrad ist vom Boden abgehoben; Sitzbank geschlossen.",
         {"stand": {"angleDeg": STAND_DOWN}}, {"x": 0, "y": 0, "angleDeg": PITCH}, ["contact-f", "stand-foot"], th(0, 0, 0, CS)),
    pose("cruising", "Cruising at 30 km/h", "Fahrt mit 30 km/h", "Stand up, both wheels on the ground, engine at part load, exhaust warm.", "Ständer oben, beide Räder am Boden, Motor im Teillastbetrieb, Abgas warm.",
         {}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.5, 0.6, 0, CT)),
    pose("braking-nose-down", "Braking, nose down", "Bremsung, Nase unten", "Hard braking attitude: body pitched 1.5 degrees nose down, front disc and rear drum hot.", "Haltung bei kräftiger Bremsung: Aufbau 1,5 Grad nach vorn geneigt, vordere Scheibe und hintere Trommel heiß.",
         {"swing": {"angleDeg": 1.5}}, {"x": 0, "y": 0, "angleDeg": -1.5}, ["contact-f", "contact-r"], th(0.15, 0.4, 1, CT)),
    pose("idling-seat-open", "Idling on the stand, seat open", "Im Leerlauf auf dem Ständer, Sitzbank offen", "Centre stand down, engine idling, seat raised 65 degrees: the storage compartment is open, the warm engine shows no flow of air.", "Hauptständer unten, Motor im Leerlauf, Sitzbank um 65 Grad angehoben: das Staufach ist offen, der warme Motor wird kaum angeströmt.",
         {"stand": {"angleDeg": STAND_DOWN}, "seat": {"angleDeg": -65}}, {"x": 0, "y": 0, "angleDeg": PITCH}, ["contact-f", "stand-foot"], th(0.2, 0, 0, CS, {"seat": {"exposedFraction": 1, "sunlitFraction": 0.9}, "engine": {"exposedFraction": 0.6, "sunlitFraction": 0.1}}),
         [load("helmet", "carried", "Helmet in the storage compartment", "Helm im Staufach", 0, -12.3, anchor="seat-storage")]),
]
for name, frac in (("loop-start", 0.0), ("loop-mid", 0.5), ("loop-end", 1.0)):
    poses.append(pose(name, "Ride loop " + name.split("-")[1], "Fahrschleife " + name.split("-")[1], "Technical key of the ride loop: wheels have turned theta = -x / r.", "Technische Schlüsselstellung der Fahrschleife: die Räder haben theta = -x / r gedreht.",
                      {"wheel-f": {"angleDeg": wheel_turn(LOOP_M * frac, RW)}, "wheel-r": {"angleDeg": wheel_turn(LOOP_M * frac, RW)}}, {"x": round(LOOP_M * frac, 4), "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.5, 0.6, 0, CT), key=True))
poses.append(pose("rolling-in", "Rolling in at 30 km/h", "Anrollen mit 30 km/h", "Technical key: enters the stop clip at 30 km/h.", "Technische Schlüsselstellung: Einfahrt in den Halt-Clip mit 30 km/h.", {}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.5, 0.6, 0, CT), key=True))
poses.append(pose("rolled-to-stop", "Stopped, engine idling", "Angehalten, Motor im Leerlauf", "Technical key: rolled 8 m to standstill, engine idling, stand still up.", "Technische Schlüsselstellung: 8 m bis zum Stillstand gerollt, Motor im Leerlauf, Ständer noch oben.",
                  {"wheel-f": {"angleDeg": wheel_turn(8, RW)}, "wheel-r": {"angleDeg": wheel_turn(8, RW)}}, {"x": 8, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.2, 0, 0.3, CT), key=True))
poses.append(pose("parked-stopped", "Parked where it stopped", "Dort abgestellt, wo es hielt", "Technical key: centre stand down 8 m on, engine off, rear wheel lifted.", "Technische Schlüsselstellung: Hauptständer 8 m weiter ausgeklappt, Motor aus, Hinterrad angehoben.",
                  {"stand": {"angleDeg": STAND_DOWN}, "wheel-f": {"angleDeg": wheel_turn(8, RW)}, "wheel-r": {"angleDeg": wheel_turn(8, RW)}}, {"x": 8, "y": 0, "angleDeg": PITCH}, ["contact-f", "stand-foot"], th(0, 0, 0, CS), key=True))
clips = [
    clip("ride-loop", "Ride loop at 30 km/h", "Fahrschleife mit 30 km/h", "Four wheel turns (%.2f m) per cycle at 8.33 m/s: both wheels roll with theta = -x / r, the root advances one cycle length every loop, the engine runs at part load." % LOOP_M, "Vier Radumdrehungen (%.2f m) pro Zyklus mit 8,33 m/s: beide Räder rollen mit theta = -x / r, die Wurzel rückt je Schleife um eine Zykluslänge vor, der Motor läuft im Teillastbetrieb." % LOOP_M,
         T_LOOP, True, [ck(0, "loop-start"), ck(round(T_LOOP / 2, 4), "loop-mid"), ck(T_LOOP, "loop-end")], [span("contact-f", 0, T_LOOP), span("contact-r", 0, T_LOOP)], [phase("cruise", 0, T_LOOP, "Steady cruise", "Gleichmäßige Fahrt")],
         [event(0, "operating", "Engine at part load", "Motor im Teillastbetrieb", "tailpipe")], [track("operating:engine-load", [(0, 0.5), (T_LOOP, 0.5)]), track("operating:rolling", [(0, 0.6), (T_LOOP, 0.6)]), track("emitter:tailpipe", [(0, 1), (T_LOOP, 1)])]),
    clip("stop-and-shut-down", "Stop, put on the stand and shut down", "Anhalten, aufbocken und abstellen", "The scooter brakes uniformly from 30 km/h to rest over 8 m (1.92 s), idles, the engine is switched off and the centre stand is put down.", "Der Roller bremst gleichmäßig von 30 km/h in 8 m (1,92 s) bis zum Stillstand, läuft im Leerlauf, der Motor wird abgestellt und der Hauptständer ausgeklappt.",
         7.0, False, [ck(0, "rolling-in", "out"), ck(1.92, "rolled-to-stop", "linear"), ck(3.5, "rolled-to-stop", "smooth"), ck(5.2, "parked-stopped", "linear"), ck(7.0, "parked-stopped", "linear")],
         [span("contact-f", 0, 7.0), span("contact-r", 0, 4.2), span("stand-foot", 4.2, 7.0)],
         [phase("brake", 0, 1.92, "Braking to rest", "Bremsen bis zum Stillstand"), phase("stand", 3.5, 5.2, "Stand goes down", "Ständer klappt aus")],
         [event(1.92, "motion", "Standstill", "Stillstand"), event(3.5, "shutdown", "Engine switched off", "Motor abgestellt", "tailpipe"), event(4.2, "contact-on", "Stand foot touches down", "Ständerfuß setzt auf", "stand-foot"), event(4.2, "contact-off", "Rear wheel lifts", "Hinterrad hebt ab", "contact-r")],
         [track("operating:engine-load", [(0, 0.2), (1.92, 0.2), (3.5, 0.2), (3.7, 0), (7.0, 0)]), track("operating:rolling", [(0, 0.6), (1.92, 0), (7.0, 0)]), track("operating:braking", [(0, 0.6), (1.92, 1), (2.8, 0), (7.0, 0)]), track("emitter:tailpipe", [(0, 1), (3.6, 0)], "step")]),
]
PEN = {500: pens(0.15, None, None, 0.13, 0.25), 200: pens(0.18, 0.13, None, 0.13, 0.25), 100: pens(0.25, 0.18, None, 0.13, 0.35), 50: pens(0.35, 0.25, 0.13, 0.18, 0.5)}
BODY = [(0.20, 0.27), (-0.20, 0.27), (-0.26, 0.28), (-0.31, 0.30), (-0.36, 0.30), (-0.62, 0.27), (-0.80, 0.26), (-0.90, 0.31), (-0.945, 0.42), (-0.945, 0.50), (-0.94, 0.66), (-0.93, 0.76), (-0.85, 0.81), (-0.76, 0.80), (-0.22, 0.70), (-0.20, 0.60), (-0.19, 0.36), (-0.14, 0.34), (0.12, 0.34), (0.17, 0.40), (0.21, 0.56), (0.26, 0.78), (0.31, 0.93), (0.34, 1.00), (0.46, 0.90), (0.46, 0.86), (0.51, 0.72), (0.56, 0.54), (0.53, 0.43), (0.47, 0.37), (0.40, 0.34), (0.28, 0.31)]
BODY500 = [(0.20, 0.27), (-0.20, 0.27), (-0.31, 0.34), (-0.62, 0.30), (-0.80, 0.30), (-0.92, 0.36), (-0.945, 0.50), (-0.94, 0.66), (-0.93, 0.78), (-0.85, 0.81), (-0.76, 0.80), (-0.22, 0.70), (-0.20, 0.62), (-0.19, 0.42), (-0.14, 0.40), (0.12, 0.40), (0.17, 0.48), (0.21, 0.62), (0.26, 0.80), (0.31, 0.93), (0.33, 1.00), (0.33, 1.10), (0.38, 1.15), (0.46, 1.15), (0.55, 1.10), (0.56, 0.95), (0.55, 0.70), (0.50, 0.47), (0.36, 0.33)]
SEAT = [(-0.22, 0.745), (-0.22, 0.67), (-0.76, 0.77), (-0.82, 0.83), (-0.80, 0.865), (-0.60, 0.85), (-0.40, 0.81), (-0.24, 0.775)]
SEAT500 = [(-0.22, 0.66), (-0.80, 0.74), (-0.82, 0.92), (-0.22, 0.86)]
COWL = [(0.33, 0.93), (0.34, 1.08), (0.40, 1.15), (0.48, 1.15), (0.55, 1.08), (0.55, 0.96), (0.48, 0.90)]
MUFF = ((-0.34, 0.37), (-0.80, 0.41))


def wheel_set(kind):
    s = []
    for nm, bn, c in (("f", "wheel-f", F), ("r", "wheel-r", R)):
        reg_hub = "brake-disc"
        if kind == 50:
            s.append(Dw.ring("tyre-" + nm, bn, c, 0.23, 0.153, region="tyres", z=1.5, tone=0.1, material="rubber"))
            s.append(Dw.ring("rim-" + nm, bn, c, 0.153, 0.095, region="tyres", z=1.6, tone=0.78, material="alloy"))
            s.append(Dw.ring("disc-" + nm, bn, c, 0.095, 0.06, region=reg_hub, z=1.7, tone=0.45, material="cast-iron"))
            s.append(Dw.ell("hub-" + nm, bn, c, 0.045, region=reg_hub, z=1.8, tone=0.7, material="alloy"))
        elif kind == 100:
            s.append(Dw.ring("tyre-" + nm, bn, c, 0.23, 0.15, regions=["tyres"], z=1.5, tone=0.1, material="rubber"))
            s.append(Dw.ring("rim-" + nm, bn, c, 0.15, 0.07, regions=["tyres", "brake-disc"], z=1.6, tone=0.7, material="alloy"))
            s.append(Dw.ell("hub-" + nm, bn, c, 0.055, region="brake-disc", z=1.8, tone=0.45, material="cast-iron"))
        elif kind == 200:
            s.append(Dw.ell("wheel-" + nm, bn, c, 0.23, regions=["tyres", "brake-disc"], z=1.5, tone=0.15, material="rubber"))
            s.append(Dw.ell("hub-" + nm, bn, c, 0.085, regions=["brake-disc"], z=1.8, tone=0.7, material="alloy", filled=True))
        else:
            s.append(Dw.ell("wheel-" + nm, bn, c, 0.23, regions=["tyres", "brake-disc"], z=1.5, tone=0.15, material="rubber"))
    return s


def mk(kind):
    s = wheel_set(kind)
    ex = lambda x: x
    if kind == 500:
        s.append(Dw.poly("body", "frame", BODY500, regions=["body-panels", "engine", "steer-assembly", "headlamp", "exhaust-system"], z=2, tone=0.55, material="paint"))
        s.append(Dw.poly("seat", "seat", SEAT500, region="seat", z=2.4, tone=0.1, material="seat-vinyl"))
        s.append(Dw.stroke("fork", "steer", [HEAD, F], 0.15, "steer-assembly", z=1.4))
        return s
    s.append(Dw.poly("body", "frame", BODY, regions=["body-panels"] + (["engine", "exhaust-system"] if kind == 200 else []), z=2, tone=0.55, material="paint"))
    s.append(Dw.poly("seat", "seat", SEAT, region="seat", z=2.4, tone=0.1, material="seat-vinyl"))
    s.append(Dw.poly("cowl", "steer", COWL, regions=["steer-assembly", "headlamp"] if kind == 200 else ["steer-assembly"], z=2.6, tone=0.5, material="paint"))
    if kind == 200:
        s.append(Dw.stroke("fork", "steer", [HEAD, F], 0.2, "steer-assembly", z=1.4))
        return s
    s.append(Dw.cap("fork", "steer", HEAD, F, 0.026, 0.02, region="steer-assembly", z=1.4, tone=0.4, material="paint"))
    s.append(Dw.ell("headlamp", "steer", (0.46, 1.04), 0.03, 0.045, region="headlamp", z=2.8, tone=0.9, material="glass"))
    s.append(Dw.cap("muffler", "swing", (-0.55, 0.46), (-0.84, 0.47), 0.05, 0.05, region="exhaust-system", z=2.5, tone=0.4, material="exhaust-steel"))
    s.append(Dw.ell("engine-cover", "swing", (-0.43, 0.52), 0.11, 0.09, region="engine", z=2.4, tone=0.35, material="engine-metal"))
    s.append(Dw.band("fender-f", "steer", F, 0.245, 0.275, 35, 160, region="body-panels", z=2.3, tone=0.55, material="paint"))
    s.append(Dw.cap("stand-leg", "stand", STAND_H, (STAND_H[0] + STAND_LEN * math.cos(math.radians(STAND_ANG)), STAND_H[1] + STAND_LEN * math.sin(math.radians(STAND_ANG))), 0.015, z=1.2, tone=0.4, material="paint", region="body-panels"))
    if kind == 50:
        s.append(Dw.cap("tail-pipe", "swing", (-0.84, 0.47), (-0.96, 0.47), 0.022, 0.02, region="exhaust-system", z=2.5, tone=0.45, material="exhaust-steel"))
        s.append(Dw.rect("tail-lamp", "frame", -0.955, 0.56, -0.925, 0.64, region="headlamp", z=2.7, tone=0.3, material="glass"))
    return s


def T(a_, b_, c_, d_, e_, f_, g_, h_):
    return dec(a_, b_, c_, d_, e_, f_, g_, h_)


def two(en, de): return (en, de)


reps = {}
for k, rng in ((500, (350, 1000)), (200, (150, 350)), (100, (75, 150)), (50, (20, 75))):
    reps[str(k)] = rep(k, rng[0], rng[1], PEN[k], [], mk(k), dec(
        two(["One composed body silhouette with the seat, two solid wheel discs and no internal edge.", "Two wheel rings with hubs, body, seat, cowl, fork, lamp, muffler with heat shield and tail pipe, engine cover, front mudguard, stand and tail lamp.", "Hub-motor-free full side elevation as for 1:100 plus the wheel construction: tyre, rim, brake disc and hub; muffler with heat shield; mudguard band.", "Full side elevation with step-through floor, leg shield, hinged seat, fork, rim, disc and hub rings, engine cover, muffler and tail pipe."][0] if k == 500 else {200: "Body, seat, cowl, fork, lamp, muffler and engine cover with two solid wheel discs.", 100: "Wheel rings with rim and hub, body, seat, cowl, fork, lamp, muffler with tail pipe, engine cover, front mudguard, stand and tail lamp.", 50: "Full side elevation with step-through floor, leg shield, hinged seat, fork, tyre, rim, disc and hub rings, engine cover, muffler with heat shield and tail pipe, mudguard band, stand and tail lamp."}[k],
            {500: "Eine zusammengesetzte Karosseriesilhouette mit Sitzbank, zwei vollen Radscheiben und ohne innere Kante.", 200: "Karosserie, Sitzbank, Verkleidung, Gabel, Lampe, Schalldämpfer und Motordeckel mit zwei vollen Radscheiben.", 100: "Radringe mit Felge und Nabe, Karosserie, Sitzbank, Verkleidung, Gabel, Lampe, Schalldämpfer mit Endrohr, Motordeckel, Vorderradschutzblech, Ständer und Rücklicht.", 50: "Vollständige Seitenansicht mit Durchstieg, Beinschild, klappbarer Sitzbank, Gabel, Reifen-, Felgen-, Scheiben- und Nabenringen, Motordeckel, Schalldämpfer mit Hitzeschild und Endrohr, Schutzblechband, Ständer und Rücklicht."}[k]),
        two({500: "Body, seat, two wheels.", 200: "All main masses.", 100: "All visible parts down to 0.05 m.", 50: "All printable parts."}[k], {500: "Karosserie, Sitzbank, zwei Räder.", 200: "Alle Hauptmassen.", 100: "Alle sichtbaren Teile bis 0,05 m.", 50: "Alle druckbaren Teile."}[k]),
        two({500: "Panels, engine, exhaust, cowl and lamp are one body fill (area-weighted overlay); tyre, rim, disc and drum are one wheel disc.", 200: "Rim and tyre are one disc per wheel; stand and tail lamp merge into the body.", 100: "Disc, caliper and drum merge into the hub; stand is a stroke.", 50: "Caliper and drum cover are drawn as the hub ring."}[k], {500: "Bleche, Motor, Auspuff, Verkleidung und Lampe sind eine Aufbaufläche (flächengewichtete Überlagerung); Reifen, Felge, Scheibe und Trommel sind eine Radscheibe.", 200: "Felge und Reifen sind eine Scheibe je Rad; Ständer und Rücklicht gehen im Aufbau auf.", 100: "Scheibe, Sattel und Trommel gehen in der Nabe auf; der Ständer ist ein Strich.", 50: "Sattel und Trommelabdeckung sind als Nabenring gezeichnet."}[k]),
        two({500: "Handlebar, mirror, fork, stand, lamps, tyre and rim detail.", 200: "Stand, tail lamp, mudguard, handlebar details, rim.", 100: "Mirror, grips, footrests, cables, tyre tread.", 50: "Mirror, cables, bolts, tread, badges."}[k], {500: "Lenker, Spiegel, Gabel, Ständer, Leuchten, Reifen- und Felgendetail.", 200: "Ständer, Rücklicht, Schutzblech, Lenkerdetails, Felge.", 100: "Spiegel, Griffe, Fußrasten, Züge, Reifenprofil.", 50: "Spiegel, Züge, Schrauben, Profil, Embleme."}[k]),
        two("The fork (0.05 m) is thickened only where drawn; no shape is thinner than the real element by more than the pen.", "Die Gabel (0,05 m) wird nur dort verdickt gezeichnet, wo sie erscheint; keine Form ist um mehr als den Stift dünner oder dicker als real."),
        two("None; material is tone only.", "Keine; das Material zeigt sich nur über den Ton."),
        two("Body mid-tone, seat and tyres near black, rims and metal light; line weights per role of the profile.", "Aufbau mittlerer Ton, Sitzbank und Reifen fast schwarz, Felgen und Metall hell; Strichstärken je Rolle des Profils."),
        two("Regions paint their own shapes where drawn; merged shapes list their regions and paint the area-weighted mean.", "Regionen färben ihre eigenen Formen; zusammengefasste Formen führen ihre Regionen und zeigen das flächengewichtete Mittel.")))

plan = [
    poly("plan-body", "frame", "body", 3, [(-0.94, -0.17), (0.0, -0.20), (0.34, -0.13), (0.34, 0.13), (0.0, 0.20), (-0.94, 0.17)], True, 0.5, True, material="paint"),
    poly("plan-wheel-f", "frame", "body", 3, [(0.44, -0.055), (0.90, -0.055), (0.90, 0.055), (0.44, 0.055)], True, 0.1, True, material="rubber"),
    poly("plan-wheel-r", "frame", "body", 3, [(-0.90, -0.06), (-0.44, -0.06), (-0.44, 0.06), (-0.90, 0.06)], True, 0.1, True, material="rubber"),
    poly("plan-bars", "frame", "body", 4, [(0.40, -0.37), (0.40, 0.37)], False, 0.4, False, material="paint", pen=0.18),
]
symbols = [symbol("scooter-plan", "Motor scooter plan symbol", "Motorroller-Grundrisszeichen", "True-extent plan: body, both wheels edge-on and handlebar width.", "Grundriss in wahrer Ausdehnung: Aufbau, beide Räder von der Kante und Lenkerbreite.", "true-extent", "model-m", [500, 200], pens(0.15, None, None, None, 0.25), [], [], plan, "Model metres: length 1.85 m, width 0.74 m.", "Modellmeter: Länge 1,85 m, Breite 0,74 m.")]
ASSUME = [
    D("Heat at level 1 released into the visible surface: engine and variator cover 1.5 kW (air-cooled, about +130 K), silencer 1.0 kW, tyres 380 W (Crr 0.015 of 205 kg at 50 km/h), front disc and rear drum 4.6 kW (20 kJ from 50 km/h in 3 s, 70 % at the front). Authored estimates calibrated by film coefficients 15 to 30 W/(m2 K).", "Wärme bei Stufe 1 in die sichtbare Oberfläche: Motor- und Variatordeckel 1,5 kW (luftgekühlt, etwa +130 K), Schalldämpfer 1,0 kW, Reifen 380 W (Crr 0,015 bei 205 kg und 50 km/h), vordere Scheibe und hintere Trommel 4,6 kW (20 kJ aus 50 km/h in 3 s, 70 % vorn). Selbst gesetzte Schätzwerte, kalibriert mit Wärmeübergangszahlen von 15 bis 30 W/(m2 K)."),
    D("Tailpipe at level 1 (full load, 7500 rpm of 125 cc): rho_a * eta_v * V * N / 2 = 1.19 * 0.8 * 0.125e-3 * 62.5 = 0.0074 kg/s plus fuel, gas at about 300 C (0.6 kg/m3) gives 0.013 m3/s; combustion water 0.073 kg/kg (Heywood Ch. 4). Flow, rise and vapour scale linearly with the load level and overestimate the idle gas temperature.", "Endrohr bei Stufe 1 (Volllast, 7500 U/min bei 125 cm3): rho_a * eta_v * V * N / 2 = 1,19 * 0,8 * 0,125e-3 * 62,5 = 0,0074 kg/s plus Kraftstoff, Gas mit etwa 300 C (0,6 kg/m3) ergibt 0,013 m3/s; Verbrennungswasser 0,073 kg/kg (Heywood Kap. 4). Volumenstrom, Erwärmung und Dampf skalieren linear mit der Laststufe und überschätzen die Leerlaufgastemperatur."),
    D("The rider is not part of the asset.", "Der Fahrer gehört nicht zum Asset."),
]
STR = [("Rigid bodies on two unilateral ground contacts or on the front tyre and the centre stand; the swing arm is a small rotation about its pivot.", "Starre Körper auf zwei einseitigen Bodenkontakten oder auf Vorderreifen und Hauptständer; die Schwinge ist eine kleine Drehung um ihren Drehpunkt."),
       ("Steering is a small in-plane rotation of fork and handlebar about the head; lateral balance is not modelled in the elevation.", "Lenken ist eine kleine ebene Drehung von Gabel und Lenker um den Kopf; das seitliche Gleichgewicht wird in der Ansicht nicht modelliert.")]
a = {
    "id": "motor-scooter", "category": "vehicles", "title": tx("Motor Scooter 125", "Motorroller 125"),
    "summary": tx("A riderless 125 cc four-stroke step-through scooter (130 kg, 12 inch wheels) with hinged seat, centre stand, exhaust jet, hot engine, silencer, tyres and brakes.", "Ein fahrerloser Motorroller mit 125-cm3-Viertaktmotor und Durchstieg (130 kg, 12-Zoll-Räder) mit klappbarer Sitzbank, Hauptständer, Abgasstrahl, heißem Motor, Schalldämpfer, Reifen und Bremsen."),
    "tags": ["vehicle", "scooter", "motorcycle", "125cc", "four-stroke", "petrol", "wheeled"],
    "provenance": provenance(["heywood-2018", "wong-2008", "limpert-1999", "etrto-2023", "ashrae-2021", "incropera-2011"],
        "Typical 125 cc four-stroke step-through scooter (wheelbase 1.34 m, 110/70-12 tyres, kerb mass 130 kg); dimensions and heat data are authored class values (authored-estimate). Wheels roll with theta = -x / r (r 0.23 m). The rider is excluded.",
        "Typischer 125-cm3-Viertakt-Roller mit Durchstieg (Radstand 1,34 m, Reifen 110/70-12, Leermasse 130 kg); Abmessungen und Wärmedaten sind selbst gesetzte Klassenwerte (authored-estimate). Die Räder rollen mit theta = -x / r (r 0,23 m). Der Fahrer ist ausgeschlossen."),
    "dimensions": {"heightM": 1.15, "widthM": 1.85, "depthM": 0.74, "massKg": 130},
    "materials": mats, "rig": {"kind": "mechanical", "root": "frame", "bones": bones}, "anchors": anchors,
    "representations": reps, "symbols": symbols, "poses": poses, "clips": clips,
    "thermal": {"environment": dict(ENV), "operating": KEYS, "regions": REGIONS, "emitters": EM, "passengers": [], "assumptions": ASSUME},
    "structure": structure(STR),
    "presentation": {"defaultPose": "cruising", "defaultClip": "ride-loop", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": EXPORTS, "completion": "authored",
}
if __name__ == "__main__":
    a, probs = finish(REL, a)
