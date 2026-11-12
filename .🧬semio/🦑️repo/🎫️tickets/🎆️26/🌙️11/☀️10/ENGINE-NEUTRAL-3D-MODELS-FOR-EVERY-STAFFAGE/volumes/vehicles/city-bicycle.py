import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("city-bicycle")
F = "frame"
ST = "steer"
S = []


def tube(id, bone, mat, tone, region, a, b, rr, r1=None, **kw):
    return capsule(id, bone, mat, "body", tone, region, a, b, rr, r1, **kw)


S.append(tube("seat-tube-lower", F, "paint", 0.3, "frame-tubes", (-0.1, 0.3, 0), (-0.285, 0.8, 0), 0.016))
S.append(tube("seat-tube-upper", F, "paint", 0.3, "frame-tubes", (-0.285, 0.8, 0), (-0.348, 0.98, 0), 0.016))
S.append(tube("down-tube-front", F, "paint", 0.3, "frame-tubes", (0.347, 0.8, 0), (0.24, 0.52, 0), 0.02))
S.append(tube("down-tube-mid", F, "paint", 0.3, "frame-tubes", (0.24, 0.52, 0), (0.1, 0.38, 0), 0.02))
S.append(tube("down-tube-low", F, "paint", 0.3, "frame-tubes", (0.1, 0.38, 0), (-0.1, 0.3, 0), 0.02))
S.append(tube("head-tube", F, "paint", 0.3, "frame-tubes", (0.347, 0.8, 0), (0.299, 0.932, 0), 0.024))
S.append(tube("bb-shell", F, "paint", 0.3, "frame-tubes", (-0.1, 0.3, -0.04), (-0.1, 0.3, 0.04), 0.015))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(tube("chainstay-" + n, F, "paint", 0.3, "frame-tubes", (-0.58, 0.345, s_ * 0.053), (-0.1, 0.3, s_ * 0.037), 0.012))
    S.append(tube("seatstay-" + n, F, "paint", 0.3, "frame-tubes", (-0.58, 0.345, s_ * 0.051), (-0.285, 0.8, s_ * 0.024), 0.012))
S.append(extrude("chaincase", F, "paint", "body", 0.6, "drivetrain", [(-0.1, 0.375), (-0.55, 0.385), (-0.58, 0.345), (-0.55, 0.305), (-0.3, 0.31), (-0.1, 0.225), (-0.025, 0.3)], "xy", -0.1, -0.04))
S.append(box("rack", F, "paint", "body", 0.3, "frame-tubes", (0.54, 0.02, 0.18), at=(-0.57, 0.805, 0), corner=0.006))
S.append(tube("rack-stay", F, "paint", 0.3, "frame-tubes", (-0.595, 0.8, 0), (-0.595, 0.7, 0), 0.006, maxd=100))
S.append(capsule("seat-post", F, "paint", "body", 0.3, "frame-tubes", (-0.348, 0.97, 0), (-0.35, 1.0, 0), 0.014))
S.append(loft("saddle", F, "saddle-skin", "body", 0.2, "saddle", [((-0.19, 1.003, 0), 0.02, 0.022, 2.4), ((-0.26, 1.012, 0), 0.05, 0.028, 2.4), ((-0.37, 1.012, 0), 0.08, 0.028, 2.4), ((-0.49, 1.02, 0), 0.085, 0.02, 2.4)]))
S.append(tube("saddle-rails", F, "chrome", 0.7, "saddle", (-0.3, 0.995, 0), (-0.42, 1.0, 0), 0.006, maxd=50))
S.append(box("tail-lamp", F, "plastic", "body", 0.3, "lamps", (0.045, 0.06, 0.06), at=(-0.9375, 0.2, 0), corner=0.01))
S.append(extrude("fender-r", F, "paint", "body", 0.55, "mudguards", arc_strip((-0.58, 0.345), 0.34, 0.356, 25, 205), "xy", -0.032, 0.032))
S.append(tube("fender-r-stay", F, "paint", 0.55, "mudguards", (-0.595, 0.8, 0), (-0.595, 0.694, 0), 0.006, maxd=100))

S.append(tube("stand-pivot", "stand", "paint", 0.3, "frame-tubes", (0, 0, -0.075), (0, 0, 0.075), 0.007))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(tube("stand-leg-" + n, "stand", "paint", 0.3, "frame-tubes", (0, 0, s_ * 0.075), (0.32, 0, s_ * 0.075), 0.007))
    S.append(tube("stand-link-" + n, "stand", "paint", 0.3, "frame-tubes", (0, 0, s_ * 0.075), (0, 0, s_ * 0.045), 0.007))

S.append(tube("crank-axle", "crank", "chrome", 0.7, "drivetrain", (0, 0, -0.075), (0, 0, 0.075), 0.01))
S.append(tube("arm-n", "crank", "paint", 0.3, "drivetrain", (0, 0, -0.07), (0.17, 0, -0.07), 0.011))
S.append(tube("arm-f", "crank", "paint", 0.3, "drivetrain", (0, 0, 0.07), (-0.17, 0, 0.07), 0.011))
S.append(revolve("chainring", "crank", "alloy", "detail", 0.6, "drivetrain", [(-0.066, 0.09), (-0.058, 0.09)], axis=(0, 0, 1), maxd=50))
S.append(capsule("pedal-n-spindle", "pedal-n", "chrome", "detail", 0.2, "drivetrain", (0, 0, -0.07), (0, 0, -0.16), 0.006))
S.append(box("pedal-n", "pedal-n", "chrome", "body", 0.2, "drivetrain", (0.1, 0.025, 0.09), at=(0, 0, -0.12), corner=0.006))
S.append(capsule("pedal-f-spindle", "pedal-f", "chrome", "detail", 0.2, "drivetrain", (0, 0, 0.07), (0, 0, 0.16), 0.006))
S.append(box("pedal-f", "pedal-f", "chrome", "body", 0.2, "drivetrain", (0.1, 0.025, 0.09), at=(0, 0, 0.12), corner=0.006))

L = lambda p: loc(ST, p)
for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("fork-blade-" + n, ST, "paint", "body", 0.3, "steer-assembly", L((0.347, 0.8, s_ * 0.045)), L((0.56, 0.345, s_ * 0.045)), 0.012, 0.009))
    S.append(capsule("handlebar-" + n, ST, "paint", "body", 0.3, "steer-assembly", L((0.04, 1.066, 0)), L((-0.022, 1.045, s_ * 0.26)), 0.014))
    S.append(capsule("grip-" + n, ST, "rubber", "body", 0.1, "steer-assembly", L((-0.015, 1.045, s_ * 0.277)), L((-0.015, 1.045, s_ * 0.293)), 0.017))
    S.append(capsule("fender-f-link-" + n, ST, "paint", "detail", 0.55, "mudguards", L((0.36, 0.797, s_ * 0.045)), L((0.465, 0.675, s_ * 0.02)), 0.005, maxd=100))
S.append(capsule("fork-crown", ST, "paint", "body", 0.3, "steer-assembly", L((0.347, 0.797, -0.052)), L((0.347, 0.797, 0.052)), 0.01))
S.append(capsule("steerer", ST, "paint", "body", 0.3, "steer-assembly", L((0.347, 0.8, 0)), L((0.3, 0.935, 0)), 0.013))
S.append(capsule("stem-post", ST, "paint", "body", 0.3, "steer-assembly", L((0.3, 0.932, 0)), L((0.285, 1.045, 0)), 0.013))
S.append(capsule("stem-rise", ST, "paint", "body", 0.3, "steer-assembly", L((0.285, 1.045, 0)), L((0.224, 1.075, 0)), 0.012))
S.append(capsule("stem-head", ST, "paint", "body", 0.3, "steer-assembly", L((0.224, 1.075, 0)), L((0.04, 1.067, 0)), 0.012))
S.append(ellipsoid("head-lamp", ST, "plastic", "body", 0.85, "lamps", (0.045, 0.035, 0.045), at=L((0.445, 0.94, 0))))
S.append(capsule("head-lamp-bracket", ST, "plastic", "detail", 0.5, "lamps", L((0.36, 0.905, 0)), L((0.42, 0.94, 0)), 0.008, maxd=100))
S.append(extrude("fender-f", ST, "paint", "body", 0.55, "mudguards", [loc(ST, (x, y, 0))[:2] for x, y in arc_strip((0.56, 0.345), 0.34, 0.356, -25, 150)], "xy", -0.032, 0.032))

for bone, hx, pre in (("wheel-f", 0.56, "f"), ("wheel-r", -0.58, "r")):
    S.append(ring(pre + "-tyre", bone, "rubber", "body", 0.1, "tyres", 0.326, 0.019, n=40))
    S.append(ring(pre + "-rim", bone, "alloy", "body", 0.78, "rims-spokes", 0.296, 0.013, n=36))
    S.append(revolve(pre + "-hub", bone, "chrome", "body", 0.7, "hub-brakes", [(-0.03, 0.03), (0.03, 0.03)], axis=(0, 0, 1)))
    S.append(revolve(pre + "-flange-l", bone, "chrome", "detail", 0.7, "hub-brakes", [(0.026, 0.05), (0.03, 0.05)], axis=(0, 0, 1)))
    S.append(revolve(pre + "-flange-r", bone, "chrome", "detail", 0.7, "hub-brakes", [(-0.03, 0.05), (-0.026, 0.05)], axis=(0, 0, 1)))
    S.append(revolve(pre + "-axle", bone, "chrome", "detail", 0.7, "hub-brakes", [(-0.052, 0.01), (0.052, 0.01)], axis=(0, 0, 1)))
    S += spokes(pre + "-spoke", bone, "alloy", 0.7, "rims-spokes", 0.05, 0.283, 0.028, 6, 0.002, maxd=100, lace_deg=6.7)

D = [
    dec("Frame, stays, fork blades, stem and handlebar are tube capsules between the end points read off the elevation tube lines (seat, down, head tube, chain and seat stays) with the lateral offsets of the plan (chain stays 0.037 to 0.053 m off the centre, seat stays converging to 0.024 m, fork blades at 0.045 m, swept-back handlebar to 0.26 m, grips 0.26 to 0.31 m).",
        "Rahmen, Streben, Gabelscheiden, Vorbau und Lenker sind Röhrenkapseln zwischen den Endpunkten der Seitenansicht (Sitz-, Unter-, Steuerrohr, Kettenstreben, Sitzstreben) mit den seitlichen Abständen des Grundrisses (Kettenstreben 0,037 bis 0,053 m neben der Mitte, Sitzstreben zusammenlaufend auf 0,024 m, Gabelscheiden bei 0,045 m, geschwungener Lenker bis 0,26 m, Griffe 0,26 bis 0,31 m)."),
    dec("Tyres and rims are rings of 36 to 40 capsule segments (one solid each with repeat about the axle, torus section 0.019 m and 0.013 m); the revolve kind has no ring profile (profiles are open and are closed to the axis). Hub and axle are revolves, the 12 spokes are two dished sets of six (one solid each with repeat) from the hub flanges at 0.028 m to the rim.",
        "Reifen und Felgen sind Ringe aus 36 bis 40 Kapselsegmenten (je ein Körper mit repeat um die Achse, Torusschnitt 0,019 m und 0,013 m); die Art revolve kennt kein Ringprofil (Profile sind offen und werden zur Achse geschlossen). Nabe und Achse sind Rotationskörper, die 12 Speichen sind zwei gedishte Sätze zu je sechs (je ein Körper mit repeat) von den Nabenflanschen bei 0,028 m zur Felge."),
    dec("Mudguards are flat arc strips (extrude of an annulus sector, 0.064 m wide) on the frame and on the steering bone, held by thin stays; the front mudguard, headlamp, handlebar and fork turn with the steer bone, the pedals and cranks with the crank bone, the kickstand is a two-leg capsule frame on the stand bone.",
        "Schutzbleche sind flache Bogenstreifen (Extrusion eines Kreisringsektors, 0,064 m breit) am Rahmen und am Lenkknochen, von dünnen Streben gehalten; Vorderschutzblech, Scheinwerfer, Lenker und Gabel drehen mit dem Lenkknochen, Pedale und Kurbeln mit dem Kurbelknochen, der Ständer ist ein zweibeiniges Kapselgestell am Ständerknochen."),
]
write("city-bicycle", S, D)
