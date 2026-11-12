import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("three-seat-sofa")
UF, BE = "upholstery-fabric", "beech-timber"
c.bw("arm-l", "arm-left", UF, "body", 0.5, (0, 0.2), (0.1, 0.62), (-0.45, 0.45), corner=0.09, region="arm-pads")
c.bw("arm-r", "arm-right", UF, "body", 0.5, (1.8, 2.0), (0.1, 0.62), (-0.45, 0.45), corner=0.09, region="arm-pads")
cols = [(0.19, 0.7333), (0.7333, 1.2667), (1.2667, 1.81)]
profile = [(0.2, 0.19), (0.2, 0.44), (0.8, 0.45)] + [(y, z) for z, y in qbez((0.45, 0.8), (0.45, 0.85), (0.4, 0.85), 4)[1:]] + [(0.85, 0.34)] + [(y, z) for z, y in qbez((0.34, 0.85), (0.3, 0.85), (0.3, 0.8), 4)[1:]]
for k, (x0, x1) in enumerate(cols, 1):
    c.bw(f"seat-{k}", f"seat-{k}", UF, "body", 0.72, (x0, x1), (0.2, 0.43), (-0.42, 0.2), corner=0.04, region="seat-cushions")
    c.ext(f"back-{k}", f"back-{k}", UF, "body", 0.62, profile, "yz", x0, x1, region="back-cushions")
c.bw("skirt", "frame", UF, "body", 0.45, (0.15, 1.85), (0.1, 0.24), (-0.43, 0.43), corner=0.01, region="base-rail")
for tag, x in (("l", 0.12), ("r", 1.88)):
    c.loft(f"foot-{tag}", "frame", BE, "body", 0.4, [([x, 0.0, 0.36], 0.025, 0.025, 2), ([x, 0.12, 0.36], 0.035, 0.035, 2)], Z, X, region="feet", repeat={"count": 2, "translation": [0, 0, -0.72]})
c.dec("The sofa is a skirted base block on four turned, tapered beech feet (lofts from 50 to 70 mm, at z = +-0.36 and x = 0.12 and 1.88, as the elevation and front views draw them), two roll arms (boxes with 90 mm rounded edges, 0.9 m deep) and three seat cushions and three back cushions on their own prismatic bones so each sinks separately.",
      "Das Sofa ist ein Zargenblock auf vier gedrechselten, verjuengten Buchenfuessen (Lofts von 50 auf 70 mm bei z = +-0,36 und x = 0,12 und 1,88, wie Ansicht und Vorderansicht sie zeichnen), zwei Rollenarmlehnen (Boxen mit 90 mm gerundeten Kanten, 0,9 m tief) sowie drei Sitz- und drei Rueckenkissen auf eigenen Schubknochen, damit jedes einzeln einsinkt.")
c.dec("The back cushions are exact extrusions of the front-view profile (0.25 m thick at the seat, 0.15 m at the top, rounded top, reclined front face) across each cushion width; seat cushions are 0.62 m deep boxes with 40 mm rounded edges. The 2D plan rounds the vertical corners by 40 and 55 mm, the 3D back cushions keep sharp vertical edges.",
      "Die Rueckenkissen sind exakte Extrusionen des Vorderansicht-Profils (0,25 m dick am Sitz, 0,15 m oben, gerundete Oberkante, geneigte Vorderflaeche) ueber die jeweilige Kissenbreite; Sitzkissen sind 0,62 m tiefe Boxen mit 40 mm gerundeten Kanten. Der 2D-Grundriss rundet die senkrechten Ecken um 40 und 55 mm, die 3D-Rueckenkissen behalten scharfe senkrechte Kanten.")
c.write()
