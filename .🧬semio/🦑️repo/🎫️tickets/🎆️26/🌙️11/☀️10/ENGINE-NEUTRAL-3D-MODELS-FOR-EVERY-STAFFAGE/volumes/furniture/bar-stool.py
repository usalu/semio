import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("bar-stool")
CH, PU, PLY = "chrome-steel", "pu-leatherette", "birch-plywood"
for tag, sx, sz in (("xp", 1, 0), ("xn", -1, 0), ("zp", 0, 1), ("zn", 0, -1)):
    c.cap("leg-" + tag, "frame", CH, "body", 0.7, [0.14*sx, 0.712, 0.14*sz], [0.1975*sx, 0.0125, 0.1975*sz], 0.0125, region="legs")
    c.rev("ferrule-" + tag, "frame", CH, "body", 0.2, [[0, 0.015], [0.022, 0.015]], Y, [0.1975*sx, 0, 0.1975*sz], region="legs", maxD=50)
c.ring("ring", "frame", CH, "body", 0.7, [0, 0.27, 0], 0.183, 0.0125, region="footrest-ring")
c.rev("plate", "seat", PLY, "body", 0.6, [[-0.018, 0.174], [-0.012, 0.18], [0.022, 0.18]], Y, [0, 0.698, 0], region="seat-plate")
pad = [[-0.01, 0.18]] + [[a, b] for b, a in qbez([0.18, 0.012], [0.18, 0.03], [0.162, 0.03], 5)]
c.rev("pad", "pad", PU, "body", 0.3, pad, Y, [0, 0.72, 0], region="seat-pad")
c.box("back-post", "backrest", CH, "body", 0.7, [-0.14, 0.765, 0], [0.03, 0.09, 0.03], region="backrest", maxD=200)
c.box("back-pad", "backrest", PU, "body", 0.3, [-0.14, 0.84, 0], [0.04, 0.06, 0.1], corner=0.018, region="backrest")
c.dec("The four chromed legs stand on a plus-shaped footprint (+x, -x, +z, -z) and splay from 0.14 m under the seat to 0.1975 m at the floor, as the plan view draws them; the elevation's single centre leg is the pair of lateral legs seen end-on.",
      "Die vier verchromten Beine stehen auf einem Kreuz-Grundriss (+x, -x, +z, -z) und spreizen sich von 0,14 m unter dem Sitz auf 0,1975 m am Boden, wie die Draufsicht sie zeigt; das einzelne Mittelbein der Ansicht sind die beiden seitlichen Beine in Sicht von vorn.")
c.dec("The footrest ring is 24 repeated capsule chords (the profile validator refuses closed loop profiles, so no torus revolve) of tube radius 12.5 mm at 0.183 m centre radius, so it fuses with the splayed legs at 0.27 m height instead of floating inside them.",
      "Der Fussring besteht aus 24 wiederholten Kapselsehnen (der Profilvalidator lehnt geschlossene Profile ab, also kein Torus) mit Rohrradius 12,5 mm und 0,183 m Mittenradius, damit er am Kreuzungspunkt in 0,27 m Hoehe mit den gespreizten Beinen verschmilzt statt frei darin zu schweben.")
c.dec("Seat plate and foam pad are revolves about the vertical seat axis (the pad on its own prismatic bone so it sinks under load); the backrest pad is 0.10 m wide (z) and 0.04 m deep (x), following the authored extrusion hint z0/z1 = +-0.05 swept laterally (the live catalog still lacks the mandatory sweep and is evaluated as sweep v).",
      "Sitzplatte und Schaumpolster sind Rotationskoerper um die senkrechte Sitzachse (das Polster auf eigenem Schubgelenk, damit es unter Last einsinkt); das Lehnenpolster ist 0,10 m breit (z) und 0,04 m tief (x), gemaess dem Extrusionshinweis z0/z1 = +-0,05 mit seitlicher Ausdehnung (im Katalog fehlt noch das verpflichtende sweep, es wird als sweep v ausgewertet).")
c.dec("Ferrules are shown only at 1:50 and the back post only down to 1:200, like the 2D representations.", "Fusskappen erscheinen nur bei 1:50 und der Lehnenpfosten nur bis 1:200, wie in den 2D-Darstellungen.")
c.write()
