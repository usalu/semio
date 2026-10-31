import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nw import *

REL = "🚜️vehicles/city-bicycle"
RW = 0.345
R = (-0.58, 0.345)
F = (0.56, 0.345)
BB = (-0.10, 0.30)
HTB = (0.347, 0.80)
HTT = (0.299, 0.932)
ST = (-0.285, 0.80)
STAND_HINGE = (-0.42, 0.3295)
STAND_ANG = -2
STAND_LEN = 0.32
CRANK_R = 0.17
LOOP_TURNS = 2
LOOP_M = LOOP_TURNS * 2 * math.pi * RW

bones = [
    bone("frame", None, "Frame, rack, chain case and saddle", "Rahmen, Gepäckträger, Kettenkasten und Sattel", 0, 0, 0, 1.12, FIXED, mass(8.5, -0.05, 0.62, ref="en-14764")),
    bone("steer", "frame", "Fork, handlebar and front mudguard", "Gabel, Lenker und Vorderradschutzblech", HTT[0], HTT[1], 0, 0.5, REV(-12, 12), mass(3.4, 0.1, -0.3)),
    bone("wheel-f", "steer", "Front wheel", "Vorderrad", F[0] - HTT[0], F[1] - HTT[1], 0, RW, FREE, mass(2.1, ref="etrto-2023")),
    bone("wheel-r", "frame", "Rear wheel with hub gear", "Hinterrad mit Nabenschaltung", R[0], R[1], 0, RW, FREE, mass(2.5, ref="etrto-2023")),
    bone("crank", "frame", "Crankset", "Kurbelgarnitur", BB[0], BB[1], 0, CRANK_R, FREE, mass(1.3)),
    bone("pedal-n", "crank", "Near pedal", "Vorderes Pedal", CRANK_R, 0, 0, 0.1, FREE, mass(0.3)),
    bone("pedal-f", "crank", "Far pedal", "Hinteres Pedal", -CRANK_R, 0, 0, 0.1, FREE, mass(0.3)),
    bone("stand", "frame", "Rear stand", "Hinterbauständer", STAND_HINGE[0], STAND_HINGE[1], STAND_ANG, STAND_LEN, REV(-100, 0), mass(0.6, 0.17, 0, "uniform")),
]
L = Loc(bones)
anchors = [
    contact("contact-r", "frame", R[0], 0.0, "Rear tyre patch", "Hinterreifenaufstandsfläche", 0.04, 0.037),
    contact("contact-f", "steer", F[0] - HTT[0], -HTT[1], "Front tyre patch", "Vorderreifenaufstandsfläche", 0.04, 0.037),
    contact("stand-foot", "stand", STAND_LEN, 0, "Stand foot", "Ständerfuß", 0.06, 0.03, mu=0.6),
    anchor("rack-deck", "frame", -0.60, 0.82, "load", "Rack load point", "Lastpunkt Gepäckträger"),
    anchor("saddle-seat", "frame", -0.35, 0.99, "load", "Saddle seat point", "Sitzpunkt Sattel"),
]

mats = [
    material("paint", "Painted steel tube and sheet", "Lackiertes Stahlrohr und -blech", 0.9, 0.6, 7850, 50, "literature", "ashrae-2021", 490),
    material("rubber", "Tyre rubber", "Reifengummi", 0.95, 0.92, 1150, 0.2, "literature", "wong-2008"),
    material("alloy", "Anodised aluminium rim", "Eloxierte Aluminiumfelge", 0.3, 0.4, 2700, 150, "literature", "ashrae-2021"),
    material("saddle-skin", "Synthetic saddle cover", "Kunstleder-Sattelbezug", 0.9, 0.75, 1100, 0.2),
    material("chrome", "Chromed steel", "Verchromter Stahl", 0.1, 0.4, 7850, 50, "literature", "incropera-2011"),
    material("plastic", "Lamp housing and lens", "Lampengehäuse und Streuscheibe", 0.9, 0.7, 1100, 0.2),
]

KEYS = [
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.6, "About 15 km/h", "Etwa 15 km/h"), (1, "About 25 km/h", "Etwa 25 km/h")]),
    opkey("pedalling", "Pedalling power", "Tretleistung", [(0, "Coasting", "Rollen"), (0.5, "About 75 W", "Etwa 75 W"), (1, "About 150 W", "Etwa 150 W")]),
    opkey("braking", "Hub braking", "Nabenbremsen", [(0, "Not braking", "Nicht gebremst"), (0.3, "Light braking", "Leichte Bremsung"), (1, "Full stop from 20 km/h", "Vollbremsung aus 20 km/h")]),
    opkey("lighting", "Dynamo lighting", "Dynamobeleuchtung", [(0, "Off", "Aus"), (1, "On", "An")]),
]

steel = lambda t, cap: layer("wall", "Steel wall %.1f mm" % (t * 1000), "Stahlwand %.1f mm" % (t * 1000), "metal", "paint", t, t / 50, cap, source="literature", ref="incropera-2011")
paintl = layer("paint", "Paint film", "Lackschicht", "polymer", "paint", 0.00006, 0.00006 / 0.3, 100, source="authored-estimate")
REGIONS = [
    region("frame-tubes", "Frame tubes, rack and stand", "Rahmenrohre, Gepäckträger und Ständer", "surface", "passive", "paint", 0.32, 0.03, "cylinder", 0.5, 5000, [steel(0.0013, 4900), paintl], ref="ashrae-2021"),
    region("steer-assembly", "Fork, stem and handlebar", "Gabel, Vorbau und Lenker", "surface", "passive", "paint", 0.2, 0.025, "cylinder", 0.6, 4600, [steel(0.0012, 4500), paintl], ref="ashrae-2021"),
    region("mudguards", "Front and rear mudguards", "Vorder- und Hinterradschutzblech", "surface", "passive", "paint", 0.5, 0.06, "cylinder", 0.6, 2400, [steel(0.0006, 2300), paintl], ref="ashrae-2021"),
    region("tyres", "Tyres 37-622", "Reifen 37-622", "tyre", "operating", "rubber", 0.46, 0.04, "cylinder", 0.45, 7000, [layer("tread", "Rubber tread and carcass", "Gummilauffläche und Karkasse", "polymer", "rubber", 0.006, 0.006 / 0.2, 7000, source="authored-estimate", ref="wong-2008")], ("rolling", 40), ref="wilson-2020"),
    region("rims-spokes", "Rims and spokes", "Felgen und Speichen", "surface", "passive", "alloy", 0.14, 0.02, "cylinder", 0.5, 6000, [layer("rim", "Aluminium rim wall", "Aluminiumfelgenwand", "metal", "alloy", 0.002, 0.002 / 150, 5400, source="literature", ref="incropera-2011")], ref="ashrae-2021"),
    region("saddle", "Saddle", "Sattel", "surface", "passive", "saddle-skin", 0.12, 0.2, "horizontal-plate-up", 0.9, 3000, [layer("cover", "Cover over foam", "Bezug über Schaum", "polymer", "saddle-skin", 0.02, 0.02 / 0.05, 3000)]),
    region("drivetrain", "Chain case, chainring, cranks and hub gear", "Kettenkasten, Kettenblatt, Kurbeln und Nabenschaltung", "surface", "operating", "paint", 0.2, 0.1, "cylinder", 0.5, 4000, [steel(0.001, 3900), paintl], ("pedalling", 9), ref="spicer-2001"),
    region("hub-brakes", "Hubs with roller brakes", "Naben mit Rollenbremsen", "surface", "operating", "chrome", 0.04, 0.09, "cylinder", 0.4, 15000, [], ("braking", 600), ref="limpert-1999"),
    region("lamps", "Dynamo lamps front and rear", "Dynamoleuchten vorn und hinten", "lamp", "operating", "plastic", 0.012, 0.05, "vertical-plate", 0.5, 2000, [], ("lighting", 2.0)),
]
EM = []
DEFAULTS = {"frame-tubes": (1, 0.5), "steer-assembly": (1, 0.6), "mudguards": (1, 0.6), "tyres": (1, 0.2), "rims-spokes": (1, 0.4), "saddle": (1, 1), "drivetrain": (0.7, 0.2), "hub-brakes": (0.6, 0.1), "lamps": (1, 0.5)}


def th(roll, ped, brake, light, contacts=None, regs=None):
    return binding(DEFAULTS, regs, {"rolling": roll, "pedalling": ped, "braking": brake, "lighting": light}, {}, contacts)


C_RIDE = {"tyres": (0.004, 30)}
C_STAND = {"tyres": (0.002, 30), "frame-tubes": (0.003, 28)}

STAND_DOWN = -90


def fk_pitch():
    a = {"rig": {"bones": bones}}
    best = None
    for k in range(-400, 400):
        pitch = k / 100
        W = world(a, {"bones": {"stand": {"angleDeg": STAND_DOWN}, "steer": {"angleDeg": 6}}, "root": {"angleDeg": pitch}})
        f = ap(W["steer"], (F[0] - HTT[0], -HTT[1]))[1]
        s = ap(W["stand"], (STAND_LEN, 0))[1]
        if best is None or abs(f - s) < best[0]:
            best = (abs(f - s), pitch)
    return best[1]


PITCH = fk_pitch()
print("stand pitch", PITCH)

poses = [
    pose("parked-on-stand", "Parked on the rear stand", "Auf dem Hinterbauständer geparkt", "The rear stand is down and lifts the rear wheel about 4 cm off the ground; the front wheel is turned 6 degrees; a 4 kg shopping bag hangs on the rack.", "Der Hinterbauständer ist ausgeklappt und hebt das Hinterrad etwa 4 cm vom Boden; das Vorderrad ist um 6 Grad eingeschlagen; eine 4-kg-Einkaufstasche hängt am Gepäckträger.",
         {"stand": {"angleDeg": STAND_DOWN}, "steer": {"angleDeg": 6}}, {"x": 0, "y": 0, "angleDeg": PITCH}, ["contact-f", "stand-foot"], th(0, 0, 0, 0, C_STAND),
         [load("shopping", "carried", "Shopping bag on the rack", "Einkaufstasche am Gepäckträger", 0, -39.2, anchor="rack-deck")]),
    pose("riding", "Rolling upright", "Aufrecht rollend", "Stand folded up, both wheels on the ground, cranks horizontal; rolling at about 15 km/h with moderate pedalling.", "Ständer hochgeklappt, beide Räder am Boden, Kurbeln waagerecht; Fahrt mit etwa 15 km/h bei mäßigem Treten.",
         {"pedal-n": {"angleDeg": 0}, "pedal-f": {"angleDeg": 0}}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.6, 0.5, 0, 0, C_RIDE)),
    pose("riding-night", "Rolling with dynamo lights", "Rollend mit Dynamobeleuchtung", "Rolling at about 15 km/h at dusk: the hub dynamo feeds the LED head and tail lamps.", "Fahrt mit etwa 15 km/h in der Dämmerung: der Nabendynamo speist die LED-Front- und Rückleuchte.",
         {"crank": {"angleDeg": -35}, "pedal-n": {"angleDeg": 35}, "pedal-f": {"angleDeg": 35}}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.6, 0.5, 0, 1, C_RIDE, {"lamps": {"exposedFraction": 1, "sunlitFraction": 0}})),
    pose("braking-stop", "Braking to a stop", "Bremsen bis zum Stillstand", "Roller brakes at full effort after 20 km/h: front hub shells hot; the steering is straight.", "Rollenbremsen mit voller Wirkung nach 20 km/h: die Nabenmäntel vorn sind heiß; die Lenkung steht gerade.",
         {"pedal-n": {"angleDeg": 0}, "pedal-f": {"angleDeg": 0}}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.2, 0, 1, 0, C_RIDE)),
]
T_LOOP = LOOP_M / 4.17
CRANK_TURN = -360.0 * LOOP_TURNS * 18 / 36
for i, (name, frac) in enumerate((("loop-start", 0.0), ("loop-mid", 0.5), ("loop-end", 1.0))):
    ang = CRANK_TURN * frac
    poses.append(pose(name, "Ride loop " + name.split("-")[1], "Fahrschleife " + name.split("-")[1], "Technical key of the ride loop: wheels have turned theta = -x / r, the crank and the levelled pedals follow the 36:18 gear.", "Technische Schlüsselstellung der Fahrschleife: die Räder haben theta = -x / r gedreht, Kurbel und waagerecht gehaltene Pedale folgen der Übersetzung 36:18.",
                      {"wheel-f": {"angleDeg": wheel_turn(LOOP_M * frac, RW)}, "wheel-r": {"angleDeg": wheel_turn(LOOP_M * frac, RW)}, "crank": {"angleDeg": ang}, "pedal-n": {"angleDeg": -ang}, "pedal-f": {"angleDeg": -ang}},
                      {"x": round(LOOP_M * frac, 4), "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.6, 0.5, 0, 0, C_RIDE), key=True))
poses.append(pose("rolled-to-stop", "Rolled to the stop point", "Am Haltepunkt ausgerollt", "Technical key: the bike has rolled to rest 3 m on, stand still up, brake just released.", "Technische Schlüsselstellung: das Rad ist 3 m weiter zum Stillstand gerollt, Ständer noch oben, Bremse gerade gelöst.",
                  {"wheel-f": {"angleDeg": wheel_turn(3, RW)}, "wheel-r": {"angleDeg": wheel_turn(3, RW)}}, {"x": 3, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.0, 0, 0.3, 0, C_RIDE), key=True))
poses.append(pose("parked-stopped", "Parked where it stopped", "Dort abgestellt, wo es hielt", "Technical key: rear stand down 3 m on, front wheel turned 6 degrees, rear wheel lifted.", "Technische Schlüsselstellung: Hinterbauständer 3 m weiter ausgeklappt, Vorderrad um 6 Grad eingeschlagen, Hinterrad angehoben.",
                  {"stand": {"angleDeg": STAND_DOWN}, "steer": {"angleDeg": 6}, "wheel-f": {"angleDeg": wheel_turn(3, RW)}, "wheel-r": {"angleDeg": wheel_turn(3, RW)}}, {"x": 3, "y": 0, "angleDeg": PITCH}, ["contact-f", "stand-foot"], th(0, 0, 0, 0, C_STAND), key=True))
poses.append(pose("rolling-in", "Rolling in before stopping", "Anrollen vor dem Halt", "Technical key: the bike enters the clip at about 15 km/h, stand up.", "Technische Schlüsselstellung: das Rad rollt mit etwa 15 km/h in den Clip, Ständer oben.",
                  {}, {"x": 0, "y": 0, "angleDeg": 0}, ["contact-f", "contact-r"], th(0.6, 0.5, 0, 0, C_RIDE), key=True))

loop_keys = [ck(0, "loop-start", "linear"), ck(round(T_LOOP / 2, 4), "loop-mid", "linear"), ck(round(T_LOOP, 4), "loop-end", "linear")]
clips = [
    clip("ride-loop", "Ride loop at 15 km/h", "Fahrschleife mit 15 km/h", "Two wheel turns (4.34 m) per cycle at 4.17 m/s: wheels roll with theta = -x / r, the crank turns once with the 36:18 gear while the pedals stay level; the root advances by one cycle length every loop.", "Zwei Radumdrehungen (4,34 m) pro Zyklus mit 4,17 m/s: die Räder rollen mit theta = -x / r, die Kurbel dreht sich bei der Übersetzung 36:18 einmal, die Pedale bleiben waagerecht; die Wurzel rückt je Schleife um eine Zykluslänge vor.",
         round(T_LOOP, 4), True, loop_keys, [span("contact-f", 0, round(T_LOOP, 4)), span("contact-r", 0, round(T_LOOP, 4))], [phase("pedalling", 0, round(T_LOOP, 4), "Pedalling at steady speed", "Treten mit gleichmäßiger Geschwindigkeit")],
         [event(0, "motion", "Cycle start", "Zyklusbeginn")], [track("operating:rolling", [(0, 0.6), (round(T_LOOP, 4), 0.6)]), track("operating:pedalling", [(0, 0.5), (round(T_LOOP, 4), 0.5)])]),
    clip("brake-and-park", "Brake, stop and put on the stand", "Bremsen, anhalten und aufbocken", "The bike brakes uniformly from 15 km/h to rest over 3 m (1.44 s), the stand is folded down and the rear wheel is lifted; the lamp stays off.", "Das Rad bremst gleichmäßig von 15 km/h in 3 m (1,44 s) bis zum Stillstand, der Ständer wird ausgeklappt und das Hinterrad angehoben; die Leuchte bleibt aus.",
         5.0, False, [ck(0, "rolling-in", "out"), ck(1.44, "rolled-to-stop", "linear"), ck(2.2, "rolled-to-stop", "smooth"), ck(4.2, "parked-stopped", "linear"), ck(5.0, "parked-stopped", "linear")],
         [span("contact-f", 0, 5.0), span("contact-r", 0, 2.6), span("stand-foot", 2.6, 5.0)],
         [phase("brake", 0, 1.44, "Braking to rest", "Bremsen bis zum Stillstand"), phase("stand", 2.2, 4.2, "Stand folds down", "Ständer klappt aus")],
         [event(1.44, "motion", "Standstill", "Stillstand"), event(2.6, "contact-on", "Stand foot touches down", "Ständerfuß setzt auf", "stand-foot"), event(2.6, "contact-off", "Rear wheel lifts off", "Hinterrad hebt ab", "contact-r")],
         [track("operating:braking", [(0, 0.5), (1.44, 1.0), (2.0, 0.0), (5.0, 0.0)]), track("operating:rolling", [(0, 0.6), (1.44, 0), (5.0, 0)]), track("operating:pedalling", [(0, 0), (5.0, 0)])]),
]

PEN = {
    500: pens(0.15, None, None, 0.13, 0.25),
    200: pens(0.18, 0.13, None, 0.13, 0.25),
    100: pens(0.18, 0.13, None, 0.13, 0.35),
    50: pens(0.25, 0.18, 0.13, 0.13, 0.5),
}
RNG = {500: (350, 1000), 200: (150, 350), 100: (75, 150), 50: (20, 75)}


def stroke(id, bn, pts, pen, region=None, regions=None, z=3, material="paint", role="body"):
    return poly(id, bn, role, z, L.pts(bn, pts), False, 0.3, False, region=region, regions=regions, material=material, pen=pen)


def capf(id, bn, p0, p1, r0, r1=None, region=None, z=3, tone=0.5, material="paint", role="body", pen=None):
    a, b = L.pt(bn, *p0), L.pt(bn, *p1)
    return cap(id, bn, role, z, a[0], a[1], b[0], b[1], r0, r1, tone=tone, region=region, material=material, pen=pen)


def ringp(id, bn, cx, cy, ro, ri, region=None, regions=None, z=4, tone=0.1, material="rubber", role="body", pen=None):
    c = L.pt(bn, cx, cy)
    return path(id, bn, role, z, L.d(bn, ring_cmds(cx, cy, ro, ri)), tone, True, region=region, regions=regions, material=material, pen=pen)


def spokes(prefix, bn, c, n, r0, r1, pen, region="rims-spokes"):
    out = []
    for i in range(n):
        a = 2 * math.pi * i / n + 0.1
        p0 = L.pt(bn, c[0] + r0 * math.cos(a), c[1] + r0 * math.sin(a))
        p1 = L.pt(bn, c[0] + r1 * math.cos(a), c[1] + r1 * math.sin(a))
        out.append(poly("%s-%d" % (prefix, i + 1), bn, "detail", 4.5, [p0, p1], False, 0.7, False, region=region, material="alloy", pen=pen))
    return out


def band(id, bn, c, ri, ro, a0, a1, region, z=3.5, tone=0.55, pen=None):
    pts = band_poly(c[0], c[1], ri, ro, a0, a1)
    return poly(id, bn, "body", z, L.pts(bn, pts), True, tone, True, region=region, material="paint", pen=pen)


def rect_at(id, bn, x0, y0, x1, y1, region, z=5, tone=0.3, material="plastic", role="body", pen=None):
    return poly(id, bn, role, z, L.pts(bn, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]), True, tone, True, region=region, material=material, pen=pen)


DOWN = [(0.347, 0.80), (0.24, 0.52), (0.10, 0.38), (-0.10, 0.30)]
BAR = [(0.275, 1.04), (0.22, 1.075), (0.12, 1.088), (0.04, 1.065)]
SADDLE = [(-0.19, 0.975), (-0.25, 0.99), (-0.40, 0.985), (-0.48, 1.0), (-0.49, 1.035), (-0.42, 1.04), (-0.25, 1.03), (-0.19, 1.0)]


def D(en, de):
    return tx(en, de)


def tubes_as_strokes(pen, prefix=""):
    out = [
        stroke("seat-tube", "frame", [BB, ST, (-0.348, 0.98)], pen, "frame-tubes"),
        stroke("down-tube", "frame", DOWN, pen, "frame-tubes"),
        stroke("head-tube", "frame", [HTB, HTT], pen * 1.4, "frame-tubes"),
        stroke("chainstay", "frame", [BB, R], pen, "frame-tubes"),
        stroke("seatstay", "frame", [(-0.285, 0.80), R], pen, "frame-tubes"),
        stroke("rack", "frame", [(-0.84, 0.80), (-0.30, 0.80)], pen, "frame-tubes"),
        stroke("rack-strut", "frame", [(-0.78, 0.80), R], pen, "frame-tubes"),
        stroke("fork", "steer", [HTB, F], pen, "steer-assembly"),
        stroke("stem", "steer", [HTT, (0.275, 1.04)] + BAR[1:], pen, "steer-assembly"),
        stroke("stand-leg", "stand", [(STAND_HINGE[0] + math.cos(math.radians(STAND_ANG)) * 0, STAND_HINGE[1]), (STAND_HINGE[0] + STAND_LEN * math.cos(math.radians(STAND_ANG)), STAND_HINGE[1] + STAND_LEN * math.sin(math.radians(STAND_ANG)))], pen, "frame-tubes"),
    ]
    return out


def rep50():
    s = []
    for nm, bn, c, hub in (("f", "wheel-f", F, 0.055), ("r", "wheel-r", R, 0.05)):
        s.append(ringp("tyre-" + nm, bn, c[0], c[1], 0.345, 0.309, region="tyres", z=4))
        s.append(ringp("rim-" + nm, bn, c[0], c[1], 0.309, 0.283, region="rims-spokes", z=4.2, tone=0.78, material="alloy"))
        s += spokes("spoke-" + nm, bn, c, 12, hub, 0.283, 0.1)
        s.append(ell("hub-" + nm, bn, "body", 4.8, L.pt(bn, *c)[0], L.pt(bn, *c)[1], hub, tone=0.7, region="hub-brakes", material="chrome"))
    s += [
        stroke("seat-tube", "frame", [BB, ST, (-0.348, 0.98)], 0.5, "frame-tubes"),
        stroke("down-tube", "frame", DOWN, 0.6, "frame-tubes"),
        stroke("head-tube", "frame", [HTB, HTT], 0.9, "frame-tubes", z=3.2),
        stroke("chainstay", "frame", [BB, R], 0.4, "frame-tubes"),
        stroke("seatstay", "frame", [(-0.285, 0.80), R], 0.35, "frame-tubes"),
        stroke("rack", "frame", [(-0.84, 0.80), (-0.30, 0.80)], 0.4, "frame-tubes"),
        stroke("fork", "steer", [HTB, F], 0.55, "steer-assembly"),
        stroke("stem-bar", "steer", [HTT, (0.275, 1.04)] + BAR[1:], 0.45, "steer-assembly"),
        stroke("grip", "steer", [BAR[3], (-0.03, 1.04)], 0.75, "steer-assembly", material="rubber", z=3.4),
        poly("saddle", "frame", "body", 5, L.pts("frame", SADDLE), True, 0.2, True, region="saddle", material="saddle-skin"),
        poly("chaincase", "frame", "body", 3.6, L.pts("frame", [(-0.10, 0.375), (-0.30, 0.385), (-0.55, 0.385), (-0.58, 0.345), (-0.55, 0.305), (-0.30, 0.31), (-0.10, 0.225), (-0.025, 0.30)]), True, 0.6, True, region="drivetrain", material="paint"),
        band("fender-r", "frame", R, 0.340, 0.356, 25, 205, "mudguards", pen=0.18),
        band("fender-f", "steer", F, 0.340, 0.356, -25, 150, "mudguards", pen=0.18),
        rect_at("tail-lamp", "frame", -0.96, 0.17, -0.915, 0.23, "lamps", tone=0.3),
        ell("head-lamp", "steer", "body", 5, L.pt("steer", 0.445, 0.94)[0], L.pt("steer", 0.445, 0.94)[1], 0.045, 0.035, tone=0.85, region="lamps", material="plastic"),
        stroke("crank-n", "crank", [BB, (BB[0] + CRANK_R, BB[1])], 0.4, "drivetrain", z=6),
        stroke("crank-f", "crank", [BB, (BB[0] - CRANK_R, BB[1])], 0.4, "drivetrain", z=2),
        poly("pedal-n", "pedal-n", "body", 6.5, [(-0.05, -0.0125), (0.05, -0.0125), (0.05, 0.0125), (-0.05, 0.0125)], True, 0.2, True, region="drivetrain", material="chrome"),
        poly("pedal-f", "pedal-f", "body", 1.5, [(-0.05, -0.0125), (0.05, -0.0125), (0.05, 0.0125), (-0.05, 0.0125)], True, 0.2, True, region="drivetrain", material="chrome"),
        stroke("stand-leg", "stand", [STAND_HINGE, (STAND_HINGE[0] + STAND_LEN * math.cos(math.radians(STAND_ANG)), STAND_HINGE[1] + STAND_LEN * math.sin(math.radians(STAND_ANG)))], 0.4, "frame-tubes"),
    ]
    return s


def rep100():
    s = []
    for nm, bn, c, hub in (("f", "wheel-f", F, 0.055), ("r", "wheel-r", R, 0.05)):
        s.append(ringp("wheel-" + nm, bn, c[0], c[1], 0.345, 0.285, regions=["tyres", "rims-spokes"], z=4, tone=0.25))
        s.append(ell("hub-" + nm, bn, "body", 4.8, L.pt(bn, *c)[0], L.pt(bn, *c)[1], hub, tone=0.7, region="hub-brakes", material="chrome"))
    s += tubes_as_strokes(0.3)
    s += [
        poly("saddle", "frame", "body", 5, L.pts("frame", SADDLE), True, 0.2, True, region="saddle", material="saddle-skin"),
        poly("chaincase", "frame", "body", 3.6, L.pts("frame", [(-0.10, 0.375), (-0.30, 0.385), (-0.55, 0.385), (-0.58, 0.345), (-0.55, 0.305), (-0.30, 0.31), (-0.10, 0.225), (-0.025, 0.30)]), True, 0.6, True, region="drivetrain", material="paint"),
        stroke("grip", "steer", [BAR[3], (-0.03, 1.04)], 0.35, "steer-assembly", material="rubber"),
        stroke("crank-n", "crank", [BB, (BB[0] + CRANK_R, BB[1])], 0.25, "drivetrain", z=6),
        stroke("crank-f", "crank", [BB, (BB[0] - CRANK_R, BB[1])], 0.25, "drivetrain", z=2),
        rect_at("tail-lamp", "frame", -0.96, 0.16, -0.91, 0.24, "lamps", tone=0.3),
        ell("head-lamp", "steer", "body", 5, L.pt("steer", 0.445, 0.94)[0], L.pt("steer", 0.445, 0.94)[1], 0.045, 0.035, tone=0.85, region="lamps", material="plastic"),
        stroke("fender-r", "frame", arc_poly(R[0], R[1], 0.348, 25, 205), 0.3, "mudguards", z=3.5),
        stroke("fender-f", "steer", arc_poly(F[0], F[1], 0.348, -25, 150), 0.3, "mudguards", z=3.5),
    ]
    return [x for x in s if x["id"] not in ("rack-strut",)]


def rep200():
    s = []
    for nm, bn, c, hub in (("f", "wheel-f", F, 0.055), ("r", "wheel-r", R, 0.05)):
        s.append(ringp("wheel-" + nm, bn, c[0], c[1], 0.345, 0.275, regions=["tyres", "rims-spokes", "mudguards"], z=4, tone=0.2))
        s.append(ell("hub-" + nm, bn, "body", 4.8, L.pt(bn, *c)[0], L.pt(bn, *c)[1], hub, tone=0.6, region="hub-brakes", material="chrome"))
    s += [x for x in tubes_as_strokes(0.13) if x["id"] not in ("rack-strut", "rack", "stand-leg", "head-tube")]
    s += [
        poly("saddle", "frame", "body", 5, L.pts("frame", [(-0.19, 0.975), (-0.30, 0.985), (-0.49, 1.0), (-0.49, 1.055), (-0.25, 1.05), (-0.19, 1.015)]), True, 0.2, True, region="saddle", material="saddle-skin"),
        poly("chaincase", "frame", "body", 3.6, L.pts("frame", [(-0.10, 0.375), (-0.55, 0.385), (-0.58, 0.345), (-0.55, 0.305), (-0.10, 0.225), (-0.025, 0.30)]), True, 0.6, True, regions=["drivetrain", "frame-tubes"], material="paint"),
        rect_at("lamps", "frame", -0.97, 0.15, -0.89, 0.25, "lamps", tone=0.3),
        stroke("rack", "frame", [(-0.84, 0.80), (-0.30, 0.80)], 0.13, "frame-tubes"),
    ]
    return s


def rep500():
    s = []
    for nm, bn, c in (("f", "wheel-f", F), ("r", "wheel-r", R)):
        e = L.pt(bn, *c)
        s.append(ell("wheel-" + nm, bn, "body", 4, e[0], e[1], 0.37, tone=0.2, filled=False, regions=["tyres", "rims-spokes", "mudguards", "hub-brakes"], material="rubber"))
    s += [
        stroke("frame-lines", "frame", [R, BB, ST, (-0.348, 0.98)], 0.13, regions=["frame-tubes", "drivetrain", "lamps"]),
        stroke("down-tube", "frame", [HTB, BB], 0.13, "frame-tubes"),
        stroke("fork-bars", "steer", [F, HTB, HTT, (0.275, 1.04), (0.12, 1.088), (0.04, 1.065)], 0.13, "steer-assembly"),
        stroke("saddle", "frame", [(-0.20, 1.0), (-0.48, 1.02)], 0.2, "saddle", material="saddle-skin"),
    ]
    return s


def dec(*a):
    return decisions(*[tx(*x) for x in a])


reps = {
    "500": unavailable(500, "At 1:500 the bicycle is 3.7 mm long. Its tubes (0.03 m) are 0.06 mm, and they pass the tyres at 0.05 to 0.09 m, i.e. 0.1 to 0.18 mm, which is below the smallest white gap that prints (0.15 mm plus the stroke): a composed line drawing would clot into the wheel rings and a filled silhouette would show the open spoked wheels as solid discs.", "Bei 1:500 ist das Fahrrad 3,7 mm lang. Seine Rohre (0,03 m) sind 0,06 mm breit und passieren die Reifen im Abstand von 0,05 bis 0,09 m, also 0,1 bis 0,18 mm; das ist weniger als die kleinste druckbare weiße Lücke (0,15 mm plus Strich): eine zusammengesetzte Strichzeichnung verklumpt mit den Radringen, und eine gefüllte Silhouette ließe die offenen Speichenräder als volle Scheiben erscheinen.",
                       "Clearance between frame tubes and tyres", "Abstand zwischen Rahmenrohren und Reifen", 0.05, [{"kind": "scale", "denominator": 200}, {"kind": "symbol", "symbol": "bicycle-plan"}]),
    "200": rep(200, *RNG[200], PEN[200], [], rep200(), dec(
        ("Two tyre rings with hub dots, a stroked frame and fork, handlebar, saddle block, chain-case slot and tail lamp.", "Zwei Reifenringe mit Nabenpunkten, gestrichelt gezeichneter Rahmen und Gabel, Lenker, Sattelblock, Kettenkasten und Rücklicht."),
         ("Wheel rings, hubs, all frame tubes, fork, handlebar, saddle, chain case, crank arms, rack and lamp.", "Radringe, Naben, alle Rahmenrohre, Gabel, Lenker, Sattel, Kettenkasten, Kurbelarme, Gepäckträger und Lampe."),
         ("Tyre, rim and mudguard form one ring 0.06 m wide; the head and tail lamp are one dot.", "Reifen, Felge und Schutzblech bilden einen 0,06 m breiten Ring; Front- und Rücklicht sind ein Punkt."),
         ("Spokes, pedals, stand, bell, grips.", "Speichen, Pedale, Ständer, Klingel, Griffe."),
         ("Tubes 0.03 m = 0.15 mm real are drawn 0.16 mm wide.", "Rohre mit 0,03 m = 0,15 mm real sind 0,16 mm breit gezeichnet."),
         ("None.", "Keine."), ("Dark rings, mid-tone chain case and saddle, light hubs.", "Dunkle Ringe, mitteltoniger Kettenkasten und Sattel, helle Naben."),
         ("Merged wheel ring carries tyres, rims and mudguards; tubes are strokes with their regions.", "Der Radring trägt Reifen, Felgen und Schutzbleche; Rohre sind Striche mit ihren Regionen.")))
    ,
    "100": rep(100, *RNG[100], PEN[100], [], rep100(), dec(
        ("Wheel rings with ten spokes and hub, tube strokes at real width, chain case, saddle, pedals, lamps and both mudguards as arcs.", "Radringe mit zehn Speichen und Nabe, Rohrstriche in Realbreite, Kettenkasten, Sattel, Pedale, Lampen und beide Schutzbleche als Bögen."),
         ("Frame, fork, handlebar, rack, rack struts, cranks, pedals, chain case, lamps, mudguards.", "Rahmen, Gabel, Lenker, Gepäckträger, Streben, Kurbeln, Pedale, Kettenkasten, Lampen, Schutzbleche."),
         ("Tyre and rim are one ring; the 36 real spokes are ten.", "Reifen und Felge sind ein Ring; aus den 36 realen Speichen werden zehn."),
         ("Bell, grips detail, chain, tread, stand foot, reflectors.", "Klingel, Griffdetail, Kette, Profil, Ständerfuß, Reflektoren."),
         ("Tubes 0.3 mm as strokes (real 0.3 mm); spokes 0.1 mm instead of 0.03 mm.", "Rohre 0,3 mm als Striche (real 0,3 mm); Speichen 0,1 mm statt 0,03 mm."),
         ("None.", "Keine."), ("Dark wheel ring, mid-tone saddle, light hubs, bar grips near black.", "Dunkler Radring, mitteltoniger Sattel, helle Naben, Lenkergriffe fast schwarz."),
         ("Each region has its own shape except tyre and rim (merged ring).", "Jede Region hat eine eigene Form außer Reifen und Felge (gemeinsamer Ring).")))
    ,
    "50": rep(50, *RNG[50], PEN[50], [], rep50(), dec(
        ("Full side elevation: tyre and rim rings, 12 spokes, hubs, step-through wave frame, fork, upswept handlebar with bell, sprung saddle, rack, chain case, mudguards, lamps, cranks with levelled pedals and the rear stand.", "Vollständige Seitenansicht: Reifen- und Felgenringe, 12 Speichen, Naben, Wellenrahmen mit tiefem Einstieg, Gabel, hochgezogener Lenker mit Klingel, gefederter Sattel, Gepäckträger, Kettenkasten, Schutzbleche, Lampen, Kurbeln mit waagerechten Pedalen und der Hinterbauständer."),
         ("All construction parts that are printable.", "Alle druckbaren Konstruktionsteile."),
         ("Frame tubes of one region on one bone merge outlines; stay pairs are single tubes.", "Rahmenrohre einer Region auf einer Bone verschmelzen in den Konturen; Strebenpaare sind einzelne Rohre."),
         ("Chain, saddle springs, cables, tread, reflectors.", "Kette, Sattelfedern, Züge, Profil, Reflektoren."),
         ("Tubes drawn as strokes of real width (0.35 to 0.9 mm); mudguards 0.32 mm bands; 12 instead of 36 spokes at 0.1 mm; bell and rack struts omitted.", "Rohre als Striche in Realbreite (0,35 bis 0,9 mm); Schutzbleche als 0,32-mm-Bänder; 12 statt 36 Speichen mit 0,1 mm; Klingel und Gepäckträgerstreben entfallen."),
         ("None.", "Keine."), ("Dark tyres, light rims, mid-tone tubes, near black grips.", "Dunkle Reifen, helle Felgen, mitteltonige Rohre, fast schwarze Griffe."),
         ("Every region has its own shape.", "Jede Region hat eine eigene Form."))),
}

plan = [
    poly("plan-wheel-r", "frame", "body", 3, [(-0.905, -0.019), (-0.215, -0.019), (-0.215, 0.019), (-0.905, 0.019)], True, 0.15, True, material="rubber"),
    poly("plan-wheel-f", "frame", "body", 3, [(0.215, -0.019), (0.905, -0.019), (0.905, 0.019), (0.215, 0.019)], True, 0.15, True, material="rubber"),
    poly("plan-frame", "frame", "body", 4, [(-0.56, 0), (-0.10, 0), (0.347, 0), (0.56, 0)], False, 0.4, False, material="paint", pen=0.13),
    poly("plan-bar", "frame", "body", 4, [(0.27, -0.31), (0.27, 0.31)], False, 0.4, False, material="paint", pen=0.13),
    ell("plan-saddle", "frame", "body", 4, -0.34, 0, 0.14, 0.07, tone=0.3, material="saddle-skin"),
    poly("plan-pedals", "frame", "body", 4, [(-0.10, -0.2), (-0.10, 0.2)], False, 0.4, False, material="paint", pen=0.13),
]
symbols = [symbol("bicycle-plan", "Bicycle plan symbol", "Fahrrad-Grundrisszeichen", "True-extent plan of the bicycle: both tyres edge-on, frame line, handlebar width, saddle and pedal axle.", "Grundriss des Fahrrads in wahrer Ausdehnung: beide Reifen von der Kante, Rahmenlinie, Lenkerbreite, Sattel und Pedalachse.",
                  "true-extent", "model-m", [500, 200], pens(0.15, None, None, None, 0.25), [], [], plan,
                  "Model metres: length 1.87 m, handlebar width 0.62 m, tyre width 0.037 m; plan geometry only.", "Modellmeter: Länge 1,87 m, Lenkerbreite 0,62 m, Reifenbreite 0,037 m; nur Grundrissgeometrie.")]

th_assume = [
    D("Operating heat at level 1: tyres 40 W (rolling resistance Crr 0.007 of a 104 kg bicycle with rider at 25 km/h = 50 W, 80 % into the tyre surfaces), drivetrain 9 W (chain about 2 W at 98.6 % efficiency, hub gear and bearings about 7 W), hub brakes 600 W (kinetic energy 1.7 kJ of a 110 kg bicycle with rider from 20 km/h in 3 s), lamps 2 W (3 W LED lamps at 33 % light).", "Betriebswärme bei Stufe 1: Reifen 40 W (Rollwiderstand Crr 0,007 eines 104-kg-Fahrrads mit Fahrer bei 25 km/h = 50 W, 80 % in die Reifenoberfläche), Antrieb 9 W (Kette etwa 2 W bei 98,6 % Wirkungsgrad, Nabenschaltung und Lager etwa 7 W), Nabenbremsen 600 W (Bewegungsenergie 1,7 kJ eines 110-kg-Fahrrads mit Fahrer aus 20 km/h in 3 s), Lampen 2 W (3-W-LED-Lampen mit 33 % Licht)."),
    D("The rider is not part of the asset: rolling, pedalling and braking levels describe a typical 85 kg rider as the scenario; no body regions or breath are modelled.", "Der Fahrer gehört nicht zum Asset: Roll-, Tret- und Bremsstufen beschreiben als Szenario einen typischen 85-kg-Fahrer; Körperregionen und Atem werden nicht modelliert."),
    D("Sun: the elevation shows one flank, so tubes and guards are half sunlit; the saddle is fully sunlit; tyre ground contact is 0.4 % of the tyre area on asphalt at 30 C.", "Sonne: die Ansicht zeigt eine Flanke, daher sind Rohre und Schutzbleche zur Hälfte besonnt, der Sattel ganz; der Bodenkontakt der Reifen beträgt 0,4 % der Reifenfläche auf Asphalt mit 30 C."),
]
STRUCT = [("Rigid bodies on two unilateral ground contacts; on the stand the rear wheel is lifted and the stand foot carries the rear load. Bicycle stands upright (lateral balance is not modelled in the elevation).", "Starre Körper auf zwei einseitigen Bodenkontakten; auf dem Ständer ist das Hinterrad angehoben und der Ständerfuß trägt die hintere Last. Das Fahrrad steht aufrecht (seitliches Gleichgewicht wird in der Ansicht nicht modelliert)."),
          ("Steering is a small in-plane rotation of fork, handlebar and front wheel about the head tube; the true lateral swing of a turned handlebar is not visible in side elevation.", "Lenken ist eine kleine ebene Drehung von Gabel, Lenker und Vorderrad um das Steuerrohr; der echte seitliche Ausschlag eines eingeschlagenen Lenkers ist in der Seitenansicht nicht sichtbar.")]

a = {
    "id": "city-bicycle", "category": "vehicles",
    "title": tx("Dutch City Bicycle", "Holland-Stadtrad"),
    "summary": tx("A riderless Dutch city bicycle with step-through frame, 28 inch wheels, chain case, dynamo lamps and a rear stand: rolling wheels, turning cranks with levelled pedals, steering, folding stand, hub-brake and drivetrain heat.", "Ein Holland-Stadtrad ohne Fahrer mit tiefem Einstieg, 28-Zoll-Rädern, Kettenkasten, Dynamobeleuchtung und Hinterbauständer: rollende Räder, drehende Kurbeln mit waagerechten Pedalen, Lenkung, klappbarer Ständer, Wärme von Nabenbremse und Antrieb."),
    "tags": ["vehicle", "bicycle", "city", "dutch", "step-through", "wheeled", "human-powered"],
    "provenance": provenance(["en-14764", "etrto-2023", "wilson-2020", "spicer-2001", "wong-2008", "limpert-1999", "ashrae-2021", "incropera-2011"],
        "Typical Dutch city bicycle (wheelbase 1.12 m, 37-622 tyres, rear stand, hub gear 36:18); dimensions and masses are authored typical class values (authored-estimate) consistent with EN 14764. Wheels roll with theta = -x / r (r 0.345 m); the loop uses a closing key with whole wheel turns. The rider is excluded.",
        "Typisches Holland-Stadtrad (Radstand 1,12 m, Reifen 37-622, Hinterbauständer, Nabenschaltung 36:18); Abmessungen und Massen sind selbst gesetzte typische Klassenwerte (authored-estimate) im Einklang mit EN 14764. Die Räder rollen mit theta = -x / r (r 0,345 m); die Schleife nutzt einen Schlussschlüssel mit ganzen Radumdrehungen. Der Fahrer ist ausgeschlossen."),
    "dimensions": {"heightM": 1.09, "widthM": 1.87, "depthM": 0.62, "massKg": 19.0},
    "materials": mats,
    "rig": {"kind": "mechanical", "root": "frame", "bones": bones}, "anchors": anchors,
    "representations": reps, "symbols": symbols, "poses": poses, "clips": clips,
    "thermal": {"environment": dict(ENV), "operating": KEYS, "regions": REGIONS, "emitters": EM, "passengers": [], "assumptions": th_assume},
    "structure": structure(STRUCT),
    "presentation": {"defaultPose": "riding", "defaultClip": "ride-loop", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored",
}
if __name__ == "__main__":
    a, probs = finish(REL, a)
