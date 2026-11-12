import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("bookshelf")
MEL, HB = "melamine-chipboard", "hardboard-back"
def span(a, b): return (a + b) / 2, b - a
def bx(id, bone, mat, tone, x, y, z, region, **k):
    (cx, sx), (cy, sy), (cz, sz) = span(*x), span(*y), span(*z)
    return c.box(id, bone, mat, "body", tone, [cx, cy, cz], [sx, sy, sz], region=region, **k)
bx("back", "frame", HB, 0.9, (0.018, 0.782), (0.07, 1.782), (0.132, 0.15), "back-panel")
bx("side-l", "frame", MEL, 0.7, (0, 0.018), (0.06, 1.8), (-0.15, 0.15), "carcass")
bx("side-r", "frame", MEL, 0.7, (0.782, 0.8), (0.06, 1.8), (-0.15, 0.15), "carcass")
bx("top", "frame", MEL, 0.7, (0.018, 0.782), (1.782, 1.8), (-0.15, 0.15), "carcass")
bx("bottom", "frame", MEL, 0.7, (0.018, 0.782), (0.06, 0.078), (-0.15, 0.15), "carcass")
bx("plinth", "frame", MEL, 0.4, (0.014, 0.786), (0, 0.06), (-0.12, 0.13), "plinth")
for n, y in enumerate((0.36, 0.66, 0.96, 1.26, 1.56), 1):
    bx(f"shelf-{n}-l", f"shelf-{n}-l", MEL, 0.7, (0.014, 0.405), (y, y + 0.018), (-0.145, 0.132), "shelves")
    bx(f"shelf-{n}-r", f"shelf-{n}-r", MEL, 0.7, (0.395, 0.786), (y, y + 0.018), (-0.145, 0.132), "shelves")
    for tag, x in (("l", (0.018, 0.038)), ("r", (0.762, 0.782))):
        bx(f"pin-{n}-{tag}", f"shelf-{n}", MEL, 0.3, x, (y - 0.012, y + 0.001), (-0.12, -0.108), "shelves", maxD=50, repeat={"count": 2, "translation": [0, 0, 0.22]})
c.dec("The carcass is built of boards with their real thicknesses: 18 mm sides, top and bottom, a hardboard back panel flush with the rear edge and a recessed plinth, so the 0.3 m depth reads as an open shelf unit in 3D.",
      "Der Korpus besteht aus Platten mit ihren echten Staerken: 18 mm Seiten, Deckel und Boden, eine Hartfaserrueckwand buendig mit der Rueckkante und ein zurueckgesetzter Sockel, damit die 0,3 m Tiefe als offenes Regal lesbar ist.")
c.dec("Each shelf is two boxes on the two half-shelf bones (sagging by +-0.87 degrees about the pins at the sides, as the loaded pose does) overlapping 10 mm at mid span so the halves never open a gap; the four shelf pins (two per end, front and rear) sit on the shelf's prismatic bone and use repeat for the front/rear pair.",
      "Jeder Boden besteht aus zwei Boxen auf den beiden Halbboden-Knochen (die sich wie in der Lastpose um +-0,87 Grad um die seitlichen Stifte durchbiegen) mit 10 mm Ueberlappung in Feldmitte, damit die Haelften nie eine Fuge oeffnen; die vier Bodentraeger (zwei je Ende, vorn und hinten) sitzen am Schubgelenk des Bodens und nutzen repeat fuer das Paar vorn/hinten.")
c.dec("The unit stands front towards -z (front edge of the shelves at z = -0.145, back at +z) as the front view and the plan draw it; pins are drawn at 1:50 only.",
      "Das Regal steht mit der Front nach -z (Vorderkante der Boeden bei z = -0,145, Rueckwand bei +z), wie Vorderansicht und Grundriss es zeigen; Bodentraeger werden nur bei 1:50 gezeichnet.")
c.write()
