import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("single-bed")
PI, BS, QC = "pine-timber", "birch-slat", "quilted-cover"
c.bw("head", "frame", PI, "body", 0.5, (0, 0.06), (0, 0.95), (-0.45, 0.45), corner=0.02, region="bed-frame")
c.bw("foot-post", "frame", PI, "body", 0.5, (2.06, 2.12), (0, 0.55), (0.39, 0.45), corner=0.02, region="bed-frame", repeat={"count": 2, "translation": [0, 0, -0.84]})
c.bw("foot-rail", "frame", PI, "body", 0.5, (2.06, 2.12), (0.47, 0.55), (-0.45, 0.45), corner=0.02, region="bed-frame")
c.bw("foot-cross-rail", "frame", PI, "body", 0.55, (2.07, 2.11), (0.14, 0.3), (-0.4, 0.4), region="bed-frame")
c.bw("rail", "frame", PI, "body", 0.55, (0.04, 2.08), (0.14, 0.3), (0.41, 0.45), region="bed-frame", repeat={"count": 2, "translation": [0, 0, -0.86]})
c.bw("center-beam", "frame", PI, "body", 0.5, (0.06, 2.06), (0.16, 0.263), (-0.025, 0.025), region="bed-frame")
c.bw("leg-mid", "frame", PI, "body", 0.5, (1.03, 1.09), (0, 0.16), (-0.03, 0.03), region="bed-frame")
for bone, x0, x1, n in (("slat-head", 0.06, 0.78, 11), ("slat-mid", 0.78, 1.38, 9), ("slat-foot", 1.38, 2.06, 10)):
    p = (x1 - x0 - 0.06) / (n - 1)
    c.bw("slats-" + bone[5:], bone, BS, "body", 0.6, (x0, x0 + 0.06), (0.263, 0.285), (-0.43, 0.43), corner=0.004, region="slats", repeat={"count": n, "translation": [p, 0, 0]})
for bone, x0, x1 in (("mat-head", 0.055, 0.8), ("mat-mid", 0.78, 1.4), ("mat-foot", 1.38, 2.065)):
    c.bw(bone, bone, QC, "body", 0.85, (x0, x1), (0.275, 0.485), (-0.41, 0.41), corner=0.04, regions=["mattress"])
c.dec("The frame is a solid 0.06 m headboard, an open footboard (two posts, a top rail and a lower cross rail the front view draws as rail-end), two 40 mm side rails and a centre beam on the centre leg that the drawings imply but do not show, so the slats have a real support at z = 0.",
      "Der Rahmen ist ein massives 0,06 m starkes Kopfteil, ein offenes Fussteil (zwei Pfosten, eine obere Leiste und die in der Vorderansicht als rail-end gezeichnete untere Querleiste), zwei 40 mm starke Seitenholme und ein Mittelbalken auf dem Mittelbein, den die Zeichnungen voraussetzen aber nicht zeigen, damit die Latten bei z = 0 eine echte Auflage haben.")
c.dec("Each of the three slat sections is one batten (60 by 22 mm, 0.86 m long) repeated 11, 9 and 10 times across its section with the gaps the pitch leaves; the head section sits on the revolute slat-head bone and carries its mattress bone, so raising the head lifts slats and mattress together.",
      "Jeder der drei Lattenrostabschnitte ist eine Latte (60 mal 22 mm, 0,86 m lang), die 11-, 9- und 10-mal ueber den Abschnitt wiederholt wird, mit den Fugen, die der Abstand laesst; der Kopfabschnitt sitzt auf dem Drehknochen slat-head und traegt seinen Matratzenknochen, sodass sich beim Anheben des Kopfteils Latten und Matratze gemeinsam heben.")
c.dec("The mattress is three 0.82 m wide sections with 40 mm rounded edges and 20 mm overlaps (as drawn), each on its own prismatic bone so that it sinks under load; the quilting piping lines are not modelled.",
      "Die Matratze besteht aus drei 0,82 m breiten Abschnitten mit 40 mm gerundeten Kanten und 20 mm Ueberlappung (wie gezeichnet), jeder auf eigenem Schubknochen, damit er unter Last einsinkt; die Steppnahtlinien sind nicht modelliert.")
c.write()
