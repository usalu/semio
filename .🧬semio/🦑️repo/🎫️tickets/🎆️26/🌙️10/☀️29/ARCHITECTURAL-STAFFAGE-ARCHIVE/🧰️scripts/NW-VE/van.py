import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nw import *
D = tx

REL = "🚜️vehicles/delivery-van"
RW = 0.356
XF, XR = 1.70, -1.60
LOOP_TURNS = 5
LOOP_M = LOOP_TURNS * 2 * math.pi * RW
V = 11.1
T_LOOP = round(LOOP_M / V, 4)
SL_X0, SL_X1 = -0.80, 0.25
SL_Y0, SL_Y1 = 0.356, 1.80
RD_W = 0.95

bones = [
    bone("chassis", None, "Body, cab and powertrain", "Aufbau, Fahrerhaus und Antriebsstrang", 0, 0, 0, 5.34, FIXED, mass(1366, 0.15, 0.85)),
    bone("slider", "chassis", "Sliding side door", "Seitliche Schiebetür", SL_X0, SL_Y0, 0, SL_X1 - SL_X0, PRI(180, 0, 0.85), mass(60, 0.5, 0.65)),
    bone("rear-door", "chassis", "Rear door leaf (projected swing)", "Hecktürflügel (projizierter Schwenk)", -2.67, 0.50, 0, RD_W, PRI(180, 0, RD_W), mass(42, -0.45, 0.7)),
    bone("axle-f", "chassis", "Front axle with suspension", "Vorderachse mit Federung", XF, RW, 0, RW, PRI(90, -0.1, 0.1), mass(210)),
    bone("axle-r", "chassis", "Rear axle with suspension", "Hinterachse mit Federung", XR, RW, 0, RW, PRI(90, -0.1, 0.1), mass(190)),
    bone("wheel-f", "axle-f", "Front wheels", "Vorderräder", 0, 0, 0, RW, FREE, mass(66, ref="wong-2008")),
    bone("wheel-r", "axle-r", "Rear wheels", "Hinterräder", 0, 0, 0, RW, FREE, mass(66, ref="wong-2008")),
]
L = Loc(bones)
Dw = Draw(L)
TP = (-2.55, 0.42)
anchors = [
    contact("contact-f", "axle-f", 0, -RW, "Front tyre patches", "Vorderreifenaufstandsflächen", 0.11, 0.47),
    contact("contact-r", "axle-r", 0, -RW, "Rear tyre patches", "Hinterreifenaufstandsflächen", 0.11, 0.47),
    anchor("tailpipe", "chassis", TP[0], TP[1], "emitter", "Tailpipe mouth", "Auspuffmündung"),
    anchor("cargo-floor", "chassis", -0.9, 0.85, "load", "Cargo floor load point", "Lastpunkt Laderaumboden"),
    anchor("sill-step", "chassis", -0.25, 0.45, "load", "Sliding door sill", "Schwelle der Schiebetür"),
]
mats = [
    material("paint", "Painted sheet steel", "Lackiertes Stahlblech", 0.9, 0.45, 7850, 50, "literature", "ashrae-2021", 490),
    material("glass", "Laminated and toughened glass", "Verbund- und Einscheibensicherheitsglas", 0.84, 0.15, 2500, 1.0, "literature", "ashrae-2021"),
    material("rubber", "Tyre rubber", "Reifengummi", 0.95, 0.92, 1150, 0.2, "literature", "wong-2008"),
    material("steel-rim", "Painted steel rim", "Lackierte Stahlfelge", 0.8, 0.5, 7850, 50, "literature", "ashrae-2021"),
    material("cast-iron", "Grey cast iron brake disc", "Graugussbremsscheibe", 0.65, 0.7, 7200, 46, "literature", "limpert-1999"),
    material("exhaust-steel", "Oxidised exhaust steel", "Oxidierter Abgasstahl", 0.7, 0.8, 7800, 25, "literature", "heywood-2018"),
    material("plastic-trim", "Bumper and trim plastic", "Stoßfänger- und Zierteilkunststoff", 0.92, 0.8, 1050, 0.2),
    material("engine-metal", "Engine casting and manifold", "Motorguss und Krümmer", 0.8, 0.8, 2700, 120, "authored-estimate", "heywood-2018"),
]
KEYS = [
    opkey("engine-load", "Engine load", "Motorlast", [(0, "Engine off", "Motor aus"), (0.2, "Idle", "Leerlauf"), (0.5, "Part load (cruising)", "Teillast (Reisefahrt)"), (1, "Full load", "Volllast")]),
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.8, "About 40 km/h", "Etwa 40 km/h"), (1, "About 50 km/h", "Etwa 50 km/h")]),
    opkey("braking", "Braking", "Bremsen", [(0, "Not braking", "Nicht gebremst"), (0.3, "Light braking", "Leichte Bremsung"), (1, "Full braking from 50 km/h", "Vollbremsung aus 50 km/h")]),
]
sheet = lambda: steel_layer(0.0008, 3100)
paintl = layer("paint", "Paint film", "Lackschicht", "polymer", "paint", 0.00008, 0.00008 / 0.3, 150)
panel = lambda id, en, de, area, ln, geo, sky, **k: region(id, en, de, "surface", k.get("model", "passive"), "paint", area, ln, geo, sky, k.get("hc", 6000), [sheet(), paintl], k.get("op"), ref="ashrae-2021")
REGIONS = [
    panel("body-panels", "Painted side and end panels", "Lackierte Seiten- und Endbleche", 22, 1.5, "vertical-plate", 0.5),
    panel("roof", "Roof panel", "Dachblech", 8.5, 2.0, "horizontal-plate-up", 0.95),
    panel("bonnet", "Bonnet, warmed from below", "Motorhaube, von unten erwärmt", 1.5, 1.0, "horizontal-plate-up", 0.9, model="operating", hc=6500, op=("engine-load", 300)),
    panel("slider-door", "Sliding side door", "Seitliche Schiebetür", 2.4, 1.3, "vertical-plate", 0.5),
    panel("rear-doors", "Rear doors", "Hecktüren", 3.4, 1.4, "vertical-plate", 0.6),
    region("cab-glazing", "Windscreen and cab glazing", "Windschutzscheibe und Fahrerhausverglasung", "glazing", "passive", "glass", 2.6, 0.9, "vertical-plate", 0.6, 7500, [layer("glass", "Laminated or toughened glass 4 mm", "Verbund- oder Einscheibensicherheitsglas 4 mm", "glass", "glass", 0.004, 0.004 / 1.0, 7500, source="literature", ref="incropera-2011")], ref="ashrae-2021"),
    region("engine-bay", "Diesel engine and turbocharger surfaces", "Dieselmotor- und Turboladeroberflächen", "engine", "operating", "engine-metal", 1.4, 0.6, "cylinder", 0.1, 9000, [], ("engine-load", 3500), ref="heywood-2018"),
    region("exhaust-system", "Exhaust pipe, particulate filter and silencer", "Abgasrohr, Partikelfilter und Schalldämpfer", "exhaust", "operating", "exhaust-steel", 1.2, 0.15, "cylinder", 0.2, 8500, [layer("pipe", "Steel pipe 1.5 mm", "Stahlrohr 1,5 mm", "metal", "exhaust-steel", 0.0015, 0.0015 / 45, 8500)], ("engine-load", 3200), ref="heywood-2018"),
    region("tyres", "Tyres 235/65 R16", "Reifen 235/65 R16", "tyre", "operating", "rubber", 3.6, 0.7, "cylinder", 0.4, 27000, [layer("tread", "Rubber tread and carcass", "Gummilauffläche und Karkasse", "polymer", "rubber", 0.012, 0.012 / 0.2, 27000, ref="wong-2008")], ("rolling", 3100), ref="wong-2008"),
    region("brakes", "Brake discs and calipers", "Bremsscheiben und Bremssättel", "surface", "operating", "cast-iron", 0.6, 0.3, "vertical-plate", 0.2, 21000, [], ("braking", 90000), ref="limpert-1999"),
]
EM = [emitter("tailpipe", "exhaust", "Exhaust jet", "Abgasstrahl", "tailpipe", 185, 0.06, "engine-load", 0.12, 300, 0.058, ref="heywood-2018")]
DEF = {"body-panels": (1, 0.5), "roof": (1, 1), "bonnet": (1, 1), "slider-door": (1, 0.5), "rear-doors": (1, 0.5), "cab-glazing": (1, 0.5), "engine-bay": (0.2, 0), "exhaust-system": (0.6, 0), "tyres": (1, 0.15), "brakes": (0.5, 0)}
CT = {"tyres": (0.04, 30)}


def th(eng, roll, brake, em=None, regs=None, grounded=True):
    return binding(DEF, regs, {"engine-load": eng, "rolling": roll, "braking": brake}, {"tailpipe": (1 if eng > 0 else 0) if em is None else em}, CT if grounded else None)


poses = [
    pose("parked", "Parked, doors closed", "Geparkt, Türen geschlossen", "Engine off and cold, all doors closed, wheels on level ground.", "Motor aus und kalt, alle Türen geschlossen, Räder auf ebenem Grund.", {}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0, 0, 0)),
    pose("side-door-open", "Sliding door open, delivering", "Schiebetür offen, Zustellung", "Parked with the sliding door fully open (0.85 m travel) and 120 kg of parcels on the sill.", "Geparkt mit ganz geöffneter Schiebetür (0,85 m Weg) und 120 kg Paketen an der Schwelle.", {"slider": {"dx": -0.85}}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"],
         th(0, 0, 0, regs={"slider-door": {"exposedFraction": 1, "sunlitFraction": 0.9}}), [load("parcels-sill", "carried", "Parcels on the sill", "Pakete an der Schwelle", 0, -1177, anchor="sill-step")]),
    pose("rear-doors-open", "Rear doors open, loading", "Hecktüren offen, Beladen", "Both rear leaves swung 90 degrees (projected 0.95 m behind the rear panel), engine off, 350 kg of cargo.", "Beide Hecktürflügel um 90 Grad geschwenkt (projiziert 0,95 m hinter der Heckwand), Motor aus, 350 kg Ladung.", {"rear-door": {"dx": -RD_W}}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"],
         th(0, 0, 0, regs={"rear-doors": {"exposedFraction": 1, "sunlitFraction": 0.9}}), [load("cargo", "carried", "Cargo 350 kg", "Ladung 350 kg", 0, -3433, anchor="cargo-floor")]),
    pose("driving-loaded", "Driving loaded at 40 km/h", "Fahrt beladen mit 40 km/h", "Steady urban drive with 400 kg of cargo: diesel at part load, warm exhaust, tyres rolling.", "Gleichmäßige Stadtfahrt mit 400 kg Ladung: Diesel im Teillastbetrieb, warmes Abgas, rollende Reifen.", {}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.5, 0.8, 0),
         [load("cargo", "carried", "Cargo 400 kg", "Ladung 400 kg", 0, -3924, anchor="cargo-floor")]),
    pose("braking-nose-down", "Braking, nose down", "Bremsung, Nase unten", "Hard braking attitude: front springs compressed, rear extended, body pitched 1.2 degrees nose down; discs hot.", "Haltung bei kräftiger Bremsung: vordere Federn eingefedert, hintere ausgefedert, Aufbau 1,2 Grad nach vorn geneigt; Scheiben heiß.", {"axle-f": {"dy": 0.04}, "axle-r": {"dy": -0.04}}, {"x": 0, "y": 0, "angleDeg": -1.2}, ["contact-f", "contact-r"], th(0.15, 0.4, 1),
         [load("cargo", "carried", "Cargo 400 kg", "Ladung 400 kg", 0, -3924, anchor="cargo-floor")]),
]
for name, frac in (("loop-start", 0.0), ("loop-mid", 0.5), ("loop-end", 1.0)):
    poses.append(pose(name, "Drive loop " + name.split("-")[1], "Fahrschleife " + name.split("-")[1], "Technical key of the drive loop: wheels have turned theta = -x / r.", "Technische Schlüsselstellung der Fahrschleife: die Räder haben theta = -x / r gedreht.",
                      {"wheel-f": {"angleDeg": wheel_turn(LOOP_M * frac, RW)}, "wheel-r": {"angleDeg": wheel_turn(LOOP_M * frac, RW)}}, {"x": round(LOOP_M * frac, 4), "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.5, 0.8, 0), [load("cargo", "carried", "Cargo 400 kg", "Ladung 400 kg", 0, -3924, anchor="cargo-floor")], key=True))
poses.append(pose("stop-in", "Rolling in at 30 km/h", "Anrollen mit 30 km/h", "Technical key: enters the stop clip at 30 km/h with 400 kg cargo.", "Technische Schlüsselstellung: Einfahrt in den Halt-Clip mit 30 km/h und 400 kg Ladung.", {}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.4, 0.6, 0), [load("cargo", "carried", "Cargo 400 kg", "Ladung 400 kg", 0, -3924, anchor="cargo-floor")], key=True))
poses.append(pose("stop-out", "Stopped at the kerb", "Am Bordstein angehalten", "Technical key: rolled 12.5 m to rest, engine idling, doors closed.", "Technische Schlüsselstellung: 12,5 m bis zum Stillstand gerollt, Motor im Leerlauf, Türen geschlossen.",
                  {"wheel-f": {"angleDeg": wheel_turn(12.5, RW)}, "wheel-r": {"angleDeg": wheel_turn(12.5, RW)}}, {"x": 12.5, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.2, 0, 0.3), [load("cargo", "carried", "Cargo 400 kg", "Ladung 400 kg", 0, -3924, anchor="cargo-floor")], key=True))
SW = {"wheel-f": {"angleDeg": wheel_turn(12.5, RW)}, "wheel-r": {"angleDeg": wheel_turn(12.5, RW)}}
clips = [
    clip("drive-loop", "Delivery drive loop at 40 km/h", "Zustellfahrt-Schleife mit 40 km/h", "Five wheel turns (%.2f m) per cycle at 11.1 m/s with 400 kg cargo: wheels roll with theta = -x / r, the root advances one cycle length every loop, diesel at part load." % LOOP_M, "Fünf Radumdrehungen (%.2f m) pro Zyklus mit 11,1 m/s und 400 kg Ladung: die Räder rollen mit theta = -x / r, die Wurzel rückt je Schleife um eine Zykluslänge vor, Diesel im Teillastbetrieb." % LOOP_M,
         T_LOOP, True, [ck(0, "loop-start"), ck(round(T_LOOP / 2, 4), "loop-mid"), ck(T_LOOP, "loop-end")], [span("contact-f", 0, T_LOOP), span("contact-r", 0, T_LOOP)], [phase("cruise", 0, T_LOOP, "Steady cruise", "Gleichmäßige Fahrt")],
         [event(0, "operating", "Diesel at part load", "Diesel im Teillastbetrieb", "tailpipe")], [track("operating:engine-load", [(0, 0.5), (T_LOOP, 0.5)]), track("operating:rolling", [(0, 0.8), (T_LOOP, 0.8)]), track("emitter:tailpipe", [(0, 1), (T_LOOP, 1)])]),
    clip("stop-and-unload", "Stop at the kerb and unload through the side door", "Am Bordstein halten und über die Seitentür entladen", "The van brakes uniformly from 30 km/h to rest over 12.5 m (3 s), idles, the engine is switched off, the sliding door opens and 120 kg of parcels are taken out.", "Der Lieferwagen bremst gleichmäßig von 30 km/h in 12,5 m (3 s) bis zum Stillstand, läuft im Leerlauf, der Motor wird abgestellt, die Schiebetür öffnet und 120 kg Pakete werden entnommen.",
         12.0, False, [ck(0, "stop-in", "out"), ck(3.0, "stop-out", "linear"), ck(5.0, "stop-out", "smooth", SW), ck(7.0, "stop-out", "smooth", {**SW, "slider": {"dx": -0.85}}), ck(12.0, "stop-out", "linear", {**SW, "slider": {"dx": -0.85}})],
         [span("contact-f", 0, 12.0), span("contact-r", 0, 12.0)], [phase("brake", 0, 3.0, "Braking to rest", "Bremsen bis zum Stillstand"), phase("door", 5.0, 7.0, "Sliding door opens", "Schiebetür öffnet"), phase("unload", 7.0, 12.0, "Unloading", "Entladen")],
         [event(3.0, "motion", "Standstill", "Stillstand"), event(4.0, "shutdown", "Engine switched off", "Motor abgestellt", "tailpipe"), event(7.0, "load-off", "Parcels taken out", "Pakete entnommen", "cargo")],
         [track("operating:engine-load", [(0, 0.4), (3.0, 0.2), (4.0, 0.2), (4.3, 0), (12.0, 0)]), track("operating:rolling", [(0, 0.6), (3.0, 0), (12.0, 0)]), track("operating:braking", [(0, 0.5), (3.0, 1), (4.5, 0), (12.0, 0)]), track("emitter:tailpipe", [(0, 1), (4.2, 0)], "step"), track("load:cargo", [(0, 1), (7.0, 1), (11.0, 0.7)])]),
]
PEN = {500: pens(0.15, None, None, 0.13, 0.25), 200: pens(0.18, 0.13, None, 0.13, 0.25), 100: pens(0.25, 0.18, None, 0.13, 0.35), 50: pens(0.35, 0.25, 0.13, 0.18, 0.5)}


def outline(arches=True):
    p = [(-2.67, 0.38), (XR - 0.48 if arches else -2.0, 0.38)]
    if arches:
        p += arc_poly(XR, RW, 0.48, 180, 0, 20)
        p += [(XF - 0.48, RW)]
        p += arc_poly(XF, RW, 0.48, 180, 0, 20)
        p += [(2.60, RW)]
    else:
        p += [(2.60, RW)]
    p += [(2.67, 0.42), (2.67, 0.62), (2.64, 0.88), (2.30, 1.0), (1.82, 1.10), (1.20, 1.95), (1.12, 1.99), (-2.58, 1.99), (-2.67, 1.93), (-2.67, 0.60)]
    return p


GL_WS = [(1.80, 1.13), (1.22, 1.93), (1.14, 1.93), (1.14, 1.20)]
GL_DOOR = [(1.60, 1.20), (1.24, 1.78), (0.45, 1.78), (0.45, 1.20)]
GL_ALL = [(1.82, 1.11), (1.20, 1.93), (0.45, 1.80), (0.45, 1.20)]


def wheelshapes(k):
    s = []
    for nm, bn in (("f", "wheel-f"), ("r", "wheel-r")):
        c = (XF if nm == "f" else XR, RW)
        if k == 500:
            s.append(Dw.ell("wheel-" + nm, bn, c, RW, regions=["tyres", "brakes"], z=4, tone=0.1, material="rubber"))
        elif k == 200:
            s.append(Dw.ell("tyre-" + nm, bn, c, RW, region="tyres", z=4, tone=0.1, material="rubber"))
            s.append(Dw.ell("disc-" + nm, bn, c, 0.2, region="brakes", z=4.4, tone=0.5, material="cast-iron"))
            s.append(Dw.ell("hub-" + nm, bn, c, 0.06, z=5, tone=0.75, material="steel-rim", role="detail"))
        elif k == 100:
            s.append(Dw.ell("tyre-" + nm, bn, c, RW, region="tyres", z=4, tone=0.1, material="rubber"))
            s.append(Dw.ell("rim-" + nm, bn, c, 0.2, z=4.2, tone=0.6, material="steel-rim", role="detail"))
            s.append(Dw.ell("disc-" + nm, bn, c, 0.15, region="brakes", z=4.4, tone=0.45, material="cast-iron", role="detail"))
            s.append(Dw.ell("hub-" + nm, bn, c, 0.07, z=5, tone=0.75, material="steel-rim", role="detail"))
        else:
            s.append(Dw.ell("tyre-" + nm, bn, c, RW, region="tyres", z=4, tone=0.1, material="rubber"))
            s.append(Dw.ell("rim-" + nm, bn, c, 0.20, z=4.2, tone=0.65, material="steel-rim", role="detail"))
            s.append(Dw.ell("disc-" + nm, bn, c, 0.14, region="brakes", z=4.4, tone=0.45, material="cast-iron", role="detail"))
            s.append(Dw.ell("hub-" + nm, bn, c, 0.055, z=4.8, tone=0.75, material="steel-rim", role="detail"))
    return s


def mk(k):
    s = wheelshapes(k)
    arch = k != 500
    body_regs = ["body-panels", "roof", "bonnet", "engine-bay", "exhaust-system"] if k == 500 else ["body-panels", "roof", "bonnet", "engine-bay", "exhaust-system"]
    s.append(Dw.poly("body-shell", "chassis", outline(arch), regions=body_regs, z=2, tone=0.65, material="paint"))
    sl = [(SL_X0, SL_Y0), (SL_X1, SL_Y0), (SL_X1, SL_Y1), (SL_X0, SL_Y1)]
    s.append(Dw.poly("slider-panel", "slider", sl, region="slider-door", z=2.5, tone=0.6, material="paint"))
    s.append(Dw.poly("rear-leaf", "rear-door", [(-2.67, 0.90), (-2.67 + RD_W, 0.90), (-2.67 + RD_W, 1.90), (-2.67, 1.90)], region="rear-doors", z=1.5, tone=0.6, material="paint"))
    if k == 500:
        s.append(Dw.poly("glazing", "chassis", GL_ALL, region="cab-glazing", z=6, tone=0.88, material="glass", role="glazing"))
        return s
    s.append(Dw.poly("windscreen", "chassis", GL_WS, region="cab-glazing", z=6, tone=0.88, material="glass", role="glazing"))
    s.append(Dw.poly("door-window", "chassis", GL_DOOR, region="cab-glazing", z=6, tone=0.88, material="glass", role="glazing"))
    s.append(Dw.cap("tail-pipe", "chassis", (-2.40, 0.42), (-2.62, 0.42), 0.04, region="exhaust-system", z=2.6, tone=0.4, material="exhaust-steel", role="detail")) if k <= 100 else None
    if k <= 100:
        s.append(Dw.stroke("door-front", "chassis", [(1.64, 1.17), (1.64, 0.98)], 0.18, z=7, role="detail"))
        s.append(Dw.stroke("door-rear", "chassis", [(0.36, 1.80), (0.36, 0.45)], 0.18, z=7, role="detail"))
        s.append(Dw.poly("headlamp", "chassis", [(2.45, 0.76), (2.64, 0.84), (2.64, 0.72), (2.5, 0.68)], z=7, tone=0.9, material="glass", role="glazing"))
        s.append(Dw.rect("tail-lamp", "chassis", -2.67, 0.85, -2.62, 1.35, z=7, tone=0.3, material="plastic-trim", role="detail"))
        s.append(Dw.ell("mirror", "chassis", (1.98, 1.30), 0.07, 0.11, z=7, tone=0.3, material="plastic-trim", role="detail"))
    if k == 50:
        s.append(Dw.stroke("rub-strip", "chassis", [(-2.4, 1.0), (-1.2, 1.0)], 0.13, z=7, role="texture"))
        s.append(Dw.cap("handle-slider", "slider", (0.0, 1.12), (0.18, 1.12), 0.018, z=7, tone=0.2, material="plastic-trim", role="detail"))
    return s


def dd(k):
    t = {
        500: ("One composed body silhouette without arches (the tyres overlap the sill), one glass band, a sliding door panel, a rear leaf and two wheel discs.", "Eine zusammengesetzte Karosseriesilhouette ohne Radläufe (die Reifen überlappen den Schweller), ein Glasband, eine Schiebetürtafel, ein Hecktürflügel und zwei Radscheiben.", 
              "Body, glass band, both doors and wheels.", "Aufbau, Glasband, beide Türen und Räder.", "Engine, exhaust, bonnet, roof and panels are one body fill (area-weighted overlay); tyre and disc are one disc.", "Motor, Abgasanlage, Haube, Dach und Bleche sind eine Aufbaufläche (flächengewichtete Überlagerung); Reifen und Scheibe sind eine Scheibe.", "Arches, handles, mirror, lamps, door gaps, tail pipe, rim.", "Radläufe, Griffe, Spiegel, Leuchten, Türfugen, Endrohr, Felge."),
        200: ("Body with wheel arches 0.124 m (0.62 mm) clear of the tyres, windscreen and door glass, sliding door panel, rear leaf, wheel discs with brake disc and hub.", "Aufbau mit Radläufen im Abstand von 0,124 m (0,62 mm) zu den Reifen, Windschutz- und Türscheibe, Schiebetürtafel, Hecktürflügel, Radscheiben mit Bremsscheibe und Nabe.",
              "All main masses and both door bones.", "Alle Hauptmassen und beide Türbones.", "Roof, panels, bonnet, engine bay and exhaust share the body shell.", "Dach, Bleche, Haube, Motorraum und Abgasanlage teilen die Aufbauhülle.", "Handles, mirror, lamps, door gaps, spokes.", "Griffe, Spiegel, Leuchten, Türfugen, Speichen."),
        100: ("Body with arches, glazing in two parts, sliding door panel, rear leaf, door gaps front and rear, lamps, mirror, tail pipe, wheels with rim, brake disc and hub.", "Aufbau mit Radläufen, Verglasung in zwei Teilen, Schiebetürtafel, Hecktürflügel, Türfugen vorn und hinten, Leuchten, Spiegel, Endrohr, Räder mit Felge, Bremsscheibe und Nabe.",
              "Everything above 0.05 m.", "Alles über 0,05 m.", "Engine bay and exhaust share the body shell; rim holes are not drawn.", "Motorraum und Abgasanlage teilen die Aufbauhülle; Felgenlöcher werden nicht gezeichnet.", "Handles, rub strips, wipers, tread.", "Griffe, Scheuerleisten, Wischer, Profil."),
        50: ("Full side elevation with arches, glazing, sliding door with handle, rear leaf, door gaps, lamps, mirror, rub strip, tail pipe and wheels with ring rim, five holes, brake disc and hub.", "Vollständige Seitenansicht mit Radläufen, Verglasung, Schiebetür mit Griff, Hecktürflügel, Türfugen, Leuchten, Spiegel, Scheuerleiste, Endrohr und Rädern mit Felgenring, fünf Löchern, Bremsscheibe und Nabe.",
             "All printable parts.", "Alle druckbaren Teile.", "Engine bay and exhaust are shown through the body shell fill.", "Motorraum und Abgasanlage zeigen sich über die Aufbaufläche.", "Wipers, seals, tread, badges.", "Wischer, Dichtungen, Profil, Embleme."),
    }[k]
    return dec(two_(t[0], t[1]), two_(t[2], t[3]), two_(t[4], t[5]), two_(t[6], t[7]),
               two_("Panels are drawn at real thickness only where they are doors; no shape is thinner than the pen.", "Bleche sind nur bei Türen in realer Stärke gezeichnet; keine Form ist dünner als der Stift."),
               two_("None; material is tone only.", "Keine; das Material zeigt sich nur über den Ton."),
               two_("Body mid-tone, doors one step lighter, glass lightest, tyres near black.", "Aufbau mittlerer Ton, Türen eine Stufe heller, Glas am hellsten, Reifen fast schwarz."),
               two_("Regions map onto their own shapes; merged shapes list their regions; the rear leaf is hidden behind the rear panel when closed.", "Regionen bilden sich auf eigene Formen ab; zusammengefasste Formen führen ihre Regionen; der Hecktürflügel liegt geschlossen hinter der Heckwand verdeckt."))


def two_(a_, b_): return (a_, b_)


reps = {str(k): rep(k, *r, PEN[k], [], mk(k), dd(k)) for k, r in ((500, (350, 1000)), (200, (150, 350)), (100, (75, 150)), (50, (20, 75)))}
plan = [
    poly("plan-body", "chassis", "body", 3, [(-2.67, -0.97), (2.30, -0.97), (2.67, -0.80), (2.67, 0.80), (2.30, 0.97), (-2.67, 0.97)], True, 0.6, True, material="paint"),
    poly("plan-cab", "chassis", "body", 4, [(0.40, -0.90), (1.55, -0.90), (2.10, -0.80), (2.10, 0.80), (1.55, 0.90), (0.40, 0.90)], False, 0.9, False, material="glass", pen=0.13),
    poly("plan-wheel-f", "chassis", "body", 5, [(1.34, 0.97), (2.06, 0.97), (2.06, 1.13), (1.34, 1.13)], True, 0.1, True, material="rubber"),
    poly("plan-wheel-r", "chassis", "body", 5, [(-1.96, 0.97), (-1.24, 0.97), (-1.24, 1.13), (-1.96, 1.13)], True, 0.1, True, material="rubber"),
]
symbols = [symbol("van-plan", "Delivery van plan symbol", "Lieferwagen-Grundrisszeichen", "True-extent plan: body outline, cab and the right-hand wheels beside the body.", "Grundriss in wahrer Ausdehnung: Aufbaukontur, Fahrerhaus und die rechten Räder neben dem Aufbau.", "true-extent", "model-m", [500, 200], pens(0.15, None, None, None, 0.25), [], [], plan, "Model metres: length 5.34 m, body width 1.99 m.", "Modellmeter: Länge 5,34 m, Aufbaubreite 1,99 m.")]
ASSUME = [
    D("Heat at level 1: engine surfaces 3.5 kW, silencer and particulate filter 3.2 kW, bonnet 0.3 kW, tyres 3.1 kW (Crr 0.011 of 2.8 t at 50 km/h = 4.2 kW, 75 % into the surface), brakes 90 kW (270 kJ from 50 km/h in 3 s of a 2.8 t van). Authored estimates calibrated by film coefficients 12 to 30 W/(m2 K); heat capacities are active shell masses on a minutes timescale.", "Wärme bei Stufe 1: Motoroberflächen 3,5 kW, Schalldämpfer und Partikelfilter 3,2 kW, Haube 0,3 kW, Reifen 3,1 kW (Crr 0,011 bei 2,8 t und 50 km/h = 4,2 kW, 75 % in die Oberfläche), Bremsen 90 kW (270 kJ aus 50 km/h in 3 s bei einem 2,8-t-Lieferwagen). Selbst gesetzte Schätzwerte, kalibriert mit Wärmeübergangszahlen von 12 bis 30 W/(m2 K); Wärmekapazitäten sind wirksame Schalenmassen im Minutenbereich."),
    D("Tailpipe at level 1 (2.0 L diesel, 3500 rpm): air flow rho_a * eta_v * V * N / 2 = 1.19 * 0.9 * 2.0e-3 * 29 = 0.062 kg/s plus fuel, gas at about 320 C (0.55 kg/m3) gives 0.12 m3/s through a 60 mm pipe; combustion water 0.058 kg/kg (diesel CH1.8, lambda 1.3, Heywood Ch. 4). Flow, rise and vapour scale linearly with the load level.", "Endrohr bei Stufe 1 (2,0-L-Diesel, 3500 U/min): Luftstrom rho_a * eta_v * V * N / 2 = 1,19 * 0,9 * 2,0e-3 * 29 = 0,062 kg/s plus Kraftstoff, Gas mit etwa 320 C (0,55 kg/m3) ergibt 0,12 m3/s durch ein 60-mm-Rohr; Verbrennungswasser 0,058 kg/kg (Diesel CH1,8, Lambda 1,3, Heywood Kap. 4). Volumenstrom, Erwärmung und Dampf skalieren linear mit der Laststufe."),
    D("The doors are projected: the sliding door travels 0.85 m along the side; a rear leaf swung 90 degrees appears 0.95 m behind the rear panel (an x-slide of the projected leaf; the end states are exact, intermediate states nominal).", "Die Türen sind projiziert: die Schiebetür fährt 0,85 m an der Seite entlang; ein um 90 Grad geschwenkter Hecktürflügel erscheint 0,95 m hinter der Heckwand (eine x-Verschiebung des projizierten Flügels; die Endzustände sind exakt, Zwischenzustände nominell)."),
]
STR = [("Rigid bodies on two axles with unilateral tyre contacts; payload acts at the cargo floor point.", "Starre Körper auf zwei Achsen mit einseitigen Reifenkontakten; die Nutzlast wirkt am Lastpunkt des Laderaumbodens."),
       ("During clips the d'Alembert forces shift load to the front axle under braking and to the rear under acceleration.", "In Clips verlagern die d'Alembert-Kräfte Last beim Bremsen auf die Vorderachse und beim Beschleunigen auf die Hinterachse.")]
a = {
    "id": "delivery-van", "category": "vehicles", "title": tx("Delivery Van", "Lieferwagen"),
    "summary": tx("A medium diesel panel van (5.34 m, 2.0 t, 235/65 R16) with a sliding side door, rear doors, cab glazing, exhaust jet, hot engine, particulate filter, tyres and brakes.", "Ein mittlerer Diesel-Kastenwagen (5,34 m, 2,0 t, 235/65 R16) mit Schiebetür, Hecktüren, Fahrerhausverglasung, Abgasstrahl, heißem Motor, Partikelfilter, Reifen und Bremsen."),
    "tags": ["vehicle", "van", "delivery", "diesel", "logistics", "wheeled"],
    "provenance": provenance(["ec-2007-715", "heywood-2018", "wong-2008", "gillespie-1992", "limpert-1999", "etrto-2023", "ashrae-2021", "incropera-2011"],
        "Typical medium panel van (wheelbase 3.30 m, kerb mass 2.0 t, 235/65 R16); dimensions, masses and heat data are authored class values (authored-estimate). Wheels roll with theta = -x / r (r 0.356 m).",
        "Typischer mittlerer Kastenwagen (Radstand 3,30 m, Leermasse 2,0 t, 235/65 R16); Abmessungen, Massen und Wärmedaten sind selbst gesetzte Klassenwerte (authored-estimate). Die Räder rollen mit theta = -x / r (r 0,356 m)."),
    "dimensions": {"heightM": 1.99, "widthM": 5.34, "depthM": 1.99, "massKg": 2000},
    "materials": mats, "rig": {"kind": "mechanical", "root": "chassis", "bones": bones}, "anchors": anchors,
    "representations": reps, "symbols": symbols, "poses": poses, "clips": clips,
    "thermal": {"environment": dict(ENV), "operating": KEYS, "regions": REGIONS, "emitters": EM, "passengers": [], "assumptions": ASSUME},
    "structure": structure(STR),
    "presentation": {"defaultPose": "parked", "defaultClip": "drive-loop", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": EXPORTS, "completion": "authored",
}
if __name__ == "__main__":
    a, probs = finish(REL, a)
