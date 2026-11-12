import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("electric-kick-scooter")
DK, FK, SM = "deck", "fork", "stem"
S = []
Lf = lambda p: loc(FK, p)
Ls = lambda p: loc(SM, p)

S.append(box("deck", DK, "anodised", "body", 0.3, "deck-battery", (0.71, 0.05, 0.17), at=(0.025, 0.12, 0), corner=0.02))
S.append(box("deck-rail", DK, "anodised", "body", 0.3, "deck-battery", (0.13, 0.05, 0.043), at=(-0.375, 0.12, 0.0635), corner=0.015, repeat={"count": 2, "translation": [0, 0, -0.127]}))
S.append(box("grip-tape", DK, "griptape", "body", 0.12, "deck-grip", (0.66, 0.023, 0.14), at=(-0.03, 0.1465, 0), corner=0.004))
S.append(extrude("fender-r", DK, "plastic", "body", 0.55, "fenders", arc_strip((-0.43, 0.108), 0.114, 0.126, 15, 170), "xy", -0.03, 0.03))
S.append(capsule("fender-r-stay", DK, "plastic", "detail", 0.55, "fenders", (-0.43, 0.23, 0), (-0.43, 0.15, 0), 0.006, 0.006, maxd=100))
S.append(box("fender-r-bridge", DK, "plastic", "detail", 0.55, "fenders", (0.04, 0.012, 0.09), at=(-0.43, 0.236, 0), corner=0.004))

for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("rear-axle-" + n, "wheel-r", "steel-disc", "detail", 0.45, "brake-disc", (0, 0, s_ * 0.022), (0, 0, s_ * 0.0638), 0.01, 0.01))
S.append(ring("r-tyre", "wheel-r", "rubber", "body", 0.1, "tyres", 0.0855, 0.0225, n=32))
S.append(revolve("r-rim", "wheel-r", "rubber", "body", 0.75, "tyres", [(-0.02, 0.075), (0.02, 0.075)], axis=(0, 0, 1)))
S.append(revolve("r-hub", "wheel-r", "steel-disc", "body", 0.45, "brake-disc", [(0.02, 0.03), (0.028, 0.03)], axis=(0, 0, 1)))
S.append(ring("f-tyre", "wheel-f", "rubber", "body", 0.1, "tyres", 0.0855, 0.0225, n=32))
S.append(revolve("f-motor", "wheel-f", "motor-alloy", "body", 0.75, "hub-motor", [(-0.021, 0.075), (0.021, 0.075)], axis=(0, 0, 1)))
S.append(revolve("f-hub", "wheel-f", "motor-alloy", "body", 0.45, "hub-motor", [(0.02, 0.035), (0.045, 0.035)], axis=(0, 0, 1)))
S.append(capsule("f-axle", "wheel-f", "motor-alloy", "detail", 0.45, "hub-motor", (0, 0, -0.02), (0, 0, 0.0535), 0.01, 0.01))

S.append(flat_tube("fork-blade", FK, "anodised", "body", 0.4, "stem-bars", Lf((0.43, 0.108, 0.0535))[:2] + [0.0535], Lf((0.405, 0.305, 0.0535))[:2] + [0.0535], 0.0135, 0.0085))
S.append(capsule("fork-crown", FK, "anodised", "body", 0.4, "stem-bars", Lf((0.4, 0.29, 0)), Lf((0.4, 0.29, 0.0535)), 0.0155, 0.0155))
S.append(box("head-block", FK, "anodised", "body", 0.3, "stem-bars", (0.08, 0.07, 0.07), at=Lf((0.4, 0.29, 0)), corner=0.012))

S.append(capsule("stem", SM, "anodised", "body", 0.4, "stem-bars", (0.0, 0, 0), (0.8, 0, 0), 0.019, 0.019))
S.append(capsule("bar", SM, "anodised", "body", 0.4, "stem-bars", (0.8, 0, -0.19), (0.8, 0, 0.19), 0.0145, 0.0145))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("grip-" + n, SM, "rubber", "body", 0.1, "stem-bars", (0.8, 0, s_ * 0.148), (0.8, 0, s_ * 0.187), 0.028, 0.028))
S.append(capsule("brake-lever", SM, "anodised", "detail", 0.2, "stem-bars", Ls((0.39, 1.083, 0.17)), Ls((0.3, 1.1, 0.17)), 0.0095, 0.0095, maxd=100))
S.append(capsule("stem-clamp", SM, "anodised", "detail", 0.3, "stem-bars", (0.03, 0, 0), (0.1, 0, 0), 0.024, 0.024, maxd=100))

D = [
    dec("The deck is a rounded box (0.79 m x 0.05 m x 0.17 m, battery pack) with a thin grip-tape slab on top; the rear 0.13 m is split into two side rails so that the rear wheel, which reaches into the deck's footprint in the drawings, runs in a slot between them.",
        "Das Deck ist ein abgerundeter Quader (0,79 m x 0,05 m x 0,17 m, Akkupaket) mit einer dünnen Griptape-Platte obenauf; die hinteren 0,13 m sind in zwei Seitenschienen geteilt, damit das Hinterrad, das in den Zeichnungen in den Grundriss des Decks hineinreicht, in einem Schlitz dazwischen läuft."),
    dec("Wheels are rings of 32 capsule segments for the pneumatic tyres (one solid with repeat about the axle) with a solid revolve for the hub motor (front) and the rim and brake rotor (rear); the single-sided fork blade and axle sit on the left (w = +0.0535 m) as in front and plan view.",
        "Die Räder sind Ringe aus 32 Kapselsegmenten für die Luftreifen (ein Körper mit repeat um die Achse) mit einem vollen Rotationskörper für den Nabenmotor (vorn) bzw. Felge und Bremsscheibe (hinten); der einseitige Gabelarm und die Achse sitzen links (w = +0,0535 m), wie in Vorderansicht und Grundriss."),
    dec("Stem, handlebar, grips and brake lever hang on the stem bone and the fork blade, head block and crown on the fork bone, so the stem folds about the head block and the fork turns about the steering axis; the stem tube leans 8 degrees back along the bone axis.",
        "Lenkerstange, Lenker, Griffe und Bremshebel hängen am Stangenknochen, Gabelarm, Steuerkopf und Gabelkopf am Gabelknochen, sodass die Stange um den Steuerkopf klappt und die Gabel um die Lenkachse dreht; die Lenkerstange ist 8 Grad nach hinten geneigt entlang der Knochenachse."),
]
write("electric-kick-scooter", S, D)
