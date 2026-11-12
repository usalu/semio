import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("motor-scooter")
FR, ST, SW, SE, SD = "frame", "steer", "swing", "seat", "stand"
S = []
Lt = lambda p: loc(ST, p)
Lw = lambda p: loc(SW, p)
Ls = lambda p: loc(SE, p)


def sec(x, bot, top, rx, e=3.2):
    return ((x, (bot + top) / 2, 0), rx, (top - bot) / 2, e)


S.append(loft("rear-body-top", FR, "paint", "body", 0.55, "body-panels", [sec(-0.945, 0.56, 0.75, 0.08), sec(-0.9, 0.54, 0.79, 0.15), sec(-0.85, 0.52, 0.805, 0.17), sec(-0.7, 0.55, 0.745, 0.19), sec(-0.36, 0.52, 0.69, 0.2), sec(-0.2, 0.52, 0.67, 0.2)]))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(loft("rear-body-side-" + n, FR, "paint", "body", 0.55, "body-panels", [((-0.9, 0.52, s_ * 0.13), 0.065, 0.22, 3.5), ((-0.8, 0.5, s_ * 0.13), 0.065, 0.25, 3.5), ((-0.36, 0.49, s_ * 0.13), 0.065, 0.2, 3.5), ((-0.2, 0.5, s_ * 0.13), 0.065, 0.17, 3.5)]))
S.append(box("floorboard", FR, "paint", "body", 0.55, "body-panels", (0.4, 0.07, 0.4), at=(-0.01, 0.305, 0), corner=0.02))
S.append(loft("leg-shield", FR, "paint", "body", 0.55, "body-panels", [((0.33, 0.33, 0), 0.2, 0.15, 3), ((0.37, 0.5, 0), 0.19, 0.17, 3), ((0.385, 0.7, 0), 0.17, 0.16, 3), ((0.395, 0.88, 0), 0.15, 0.1, 3), ((0.4, 1.0, 0), 0.13, 0.06, 3)]))
S.append(box("tail-lamp", FR, "glass", "body", 0.3, "headlamp", (0.03, 0.08, 0.14), at=(-0.94, 0.6, 0), corner=0.008))

S.append(loft("seat", SE, "seat-vinyl", "body", 0.1, "seat", [(Ls((-0.23, 0.722, 0)), 0.155, 0.052, 3), (Ls((-0.4, 0.757, 0)), 0.155, 0.053, 3), (Ls((-0.6, 0.795, 0)), 0.155, 0.055, 3), (Ls((-0.76, 0.816, 0)), 0.15, 0.046, 3), (Ls((-0.815, 0.83, 0)), 0.1, 0.022, 3)]))

S.append(ellipsoid("engine-cover", SW, "engine-metal", "body", 0.35, "engine", (0.11, 0.09, 0.03), at=Lw((-0.43, 0.52, -0.185))))
S.append(box("engine", SW, "engine-metal", "detail", 0.35, "engine", (0.3, 0.22, 0.14), at=Lw((-0.42, 0.43, 0)), corner=0.03))
S.append(capsule("muffler", SW, "exhaust-steel", "body", 0.4, "exhaust-system", Lw((-0.84, 0.465, -0.15)), Lw((-0.55, 0.465, -0.15)), 0.055, 0.055))
S.append(capsule("tail-pipe", SW, "exhaust-steel", "body", 0.45, "exhaust-system", Lw((-0.98, 0.47, -0.15)), Lw((-0.84, 0.47, -0.15)), 0.022, 0.022))
S.append(capsule("muffler-link", SW, "exhaust-steel", "detail", 0.4, "exhaust-system", Lw((-0.55, 0.465, -0.15)), Lw((-0.5, 0.45, -0.1)), 0.03, 0.03, maxd=100))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("swing-arm-" + n, SW, "paint", "detail", 0.4, "body-panels", Lw((-0.2, 0.3, s_ * 0.075)), Lw((-0.67, 0.23, s_ * 0.075)), 0.02, 0.02))

for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("stand-leg-" + n, SD, "paint", "body", 0.4, "body-panels", (0, 0, s_ * 0.11), (0.335, 0, s_ * 0.11), 0.015, 0.015))
S.append(capsule("stand-pivot", SD, "paint", "detail", 0.4, "body-panels", (0, 0, -0.11), (0, 0, 0.11), 0.015, 0.015))
S.append(capsule("stand-foot", SD, "paint", "detail", 0.4, "body-panels", (0.335, 0, -0.11), (0.335, 0, 0.11), 0.015, 0.015))

for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("fork-" + n, ST, "paint", "body", 0.4, "steer-assembly", Lt((0.67, 0.23, s_ * 0.08)), Lt((0.42, 0.99, s_ * 0.08)), 0.016, 0.016))
    S.append(capsule("grip-" + n, ST, "rubber", "body", 0.1, "steer-assembly", Lt((0.4, 1.0625, s_ * 0.32)), Lt((0.4, 1.0625, s_ * 0.34)), 0.03, 0.03))
S.append(capsule("fork-crown", ST, "paint", "body", 0.4, "steer-assembly", Lt((0.42, 0.99, -0.08)), Lt((0.42, 0.99, 0.08)), 0.02, 0.02))
S.append(capsule("handlebar", ST, "paint", "body", 0.4, "steer-assembly", Lt((0.4, 1.0625, -0.32)), Lt((0.4, 1.0625, 0.32)), 0.0175, 0.0175))
S.append(ellipsoid("cowl", ST, "paint", "body", 0.5, "steer-assembly", (0.125, 0.11, 0.17), at=Lt((0.44, 1.025, 0))))
S.append(ellipsoid("headlamp", ST, "glass", "body", 0.9, "headlamp", (0.045, 0.03, 0.065), at=Lt((0.46, 1.04, 0))))
S.append(extrude("fender-f", ST, "paint", "body", 0.55, "body-panels", [loc(ST, (x, y, 0))[:2] for x, y in arc_strip((0.67, 0.23), 0.243, 0.273, 35, 160)], "xy", -0.065, 0.065))
S.append(capsule("fender-f-stay", ST, "paint", "detail", 0.55, "body-panels", Lt((0.55, 0.5, 0)), Lt((0.45, 0.9, 0)), 0.01, 0.01, maxd=100))

for bone, hx, pre, wt in (("wheel-f", 0.67, "f", 0.05), ("wheel-r", -0.67, "r", 0.055)):
    S.append(ring(pre + "-tyre", bone, "rubber", "body", 0.1, "tyres", 0.23 - wt, wt, n=40))
    S.append(revolve(pre + "-rim", bone, "alloy", "body", 0.78, "tyres", [(-0.04, 0.153), (0.04, 0.153)], axis=(0, 0, 1)))
    S.append(revolve(pre + "-disc", bone, "cast-iron", "body", 0.45, "brake-disc", [(0.04, 0.095), (0.046, 0.095)], axis=(0, 0, 1)))
    S.append(revolve(pre + "-hub", bone, "alloy", "body", 0.7, "brake-disc", [(-0.05, 0.045), (0.05, 0.045)], axis=(0, 0, 1)))
    S.append(revolve(pre + "-axle", bone, "alloy", "detail", 0.7, "brake-disc", [(-0.075, 0.015), (0.075, 0.015)], axis=(0, 0, 1)))
    S.append(capsule(pre + "-spoke", bone, "alloy", "detail", 0.78, "tyres", (0.05, 0, 0.042), (0.15, 0, 0.042), 0.014, 0.014, repeat={"count": 5, "rotationAxis": [0, 0, 1], "rotationRad": r(2 * math.pi / 5)}, maxd=100))

D = [
    dec("The shell is split into the parts a real scooter has: a leg shield (loft leaning from the floor to the steering head), a floorboard, and a rear body made of a top tray and two side panels 0.13 m off the centre so that the rear wheel runs in a slot between them; the engine, swing arms and muffler hang on the swing bone.",
        "Die Verkleidung ist in die Teile eines echten Rollers zerlegt: ein Beinschild (Loft, vom Boden zum Steuerkopf geneigt), ein Trittbrett und ein Heck aus einer oberen Schale und zwei Seitenteilen 0,13 m neben der Mitte, damit das Hinterrad in einem Schlitz dazwischen läuft; Motor, Schwingenarme und Auspuff hängen am Schwingenknochen."),
    dec("Tyres are rings of 40 capsule segments (one solid with repeat about the axle), rims are solid revolves 0.08 m wide with a raised brake disc, hub and axle; five spokes per wheel are one solid with repeat; the seat is a loft on the seat bone and opens with it.",
        "Reifen sind Ringe aus 40 Kapselsegmenten (ein Körper mit repeat um die Achse), Felgen sind volle Rotationskörper von 0,08 m Breite mit aufgesetzter Bremsscheibe, Nabe und Achse; fünf Speichen je Rad sind ein Körper mit repeat; die Sitzbank ist ein Loft am Sitzknochen und klappt mit ihm auf."),
    dec("The two-legged centre stand and the single exhaust on the right (w = -0.15 m, from the plan) are explicit capsule solids; the fork legs, handlebar, grips, cowl, headlamp and front fender turn with the steer bone.",
        "Der zweibeinige Hauptständer und die einzelne Auspuffanlage rechts (w = -0,15 m, aus dem Grundriss) sind eigene Kapselkörper; Gabelholme, Lenker, Griffe, Verkleidung, Scheinwerfer und Vorderkotflügel drehen mit dem Lenkknochen."),
]
write("motor-scooter", S, D)
