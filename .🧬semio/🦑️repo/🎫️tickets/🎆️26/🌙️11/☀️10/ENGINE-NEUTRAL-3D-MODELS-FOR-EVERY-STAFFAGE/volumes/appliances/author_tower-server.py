import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

S = [
    box("foot-l", "case", "rubber-foot", "body", 0.15, "feet", (0.04, 0.012, 0.03), at=(-0.18, 0.006, 0.075), corner=0.003, repeat={"count": 2, "translation": [0.36, 0, 0]}),
    box("foot-r", "case", "rubber-foot", "body", 0.15, "feet", (0.04, 0.012, 0.03), at=(-0.18, 0.006, -0.075), corner=0.003, repeat={"count": 2, "translation": [0.36, 0, 0]}),
    box("case-side", "case", "chassis-steel", "body", 0.55, "side-panel", (0.413, 0.438, 0.2), at=(-0.0065, 0.231, 0), corner=0.004),
    box("rear-panel", "case", "chassis-steel", "body", 0.5, "rear-panel", (0.012, 0.438, 0.2), at=(-0.219, 0.231, 0), corner=0.004),
    box("front-bezel", "case", "bezel-abs", "body", 0.18, "front-bezel", (0.025, 0.438, 0.2), at=(0.2125, 0.231, 0), corner=0.005),
    box("psu-vent-rear", "case", "chassis-steel", "body", 0.38, "psu-vent", (0.0265, 0.108, 0.17), at=(-0.21275, 0.384, 0), corner=0.002, maxd=100),
    box("psu-vent-top", "case", "chassis-steel", "body", 0.38, "psu-vent", (0.09, 0.0016, 0.17), at=(-0.155, 0.4492, 0), maxd=100),
    box("cpu-grille", "case", "chassis-steel", "body", 0.38, "cpu-grille", (0.17, 0.13, 0.0016), at=(0.015, 0.267, 0.1008), corner=0.0006, maxd=100),
    revolve("fan-grille", "case", "chassis-steel", "detail", 0.3, "rear-panel", [(-0.2258, 0.05), (-0.2245, 0.05)], axis=(1, 0, 0), origin=(0, 0.2, 0), maxd=50),
    box("intake-mesh", "case", "bezel-abs", "detail", 0.3, "front-bezel", (0.002, 0.34, 0.17), at=(0.226, 0.22, 0), maxd=100),
    box("drive-bay", "case", "bezel-abs", "detail", 0.4, "front-bezel", (0.002, 0.03, 0.14), at=(0.226, 0.41, 0), maxd=50),
    box("power-button", "case", "bezel-abs", "detail", 0.7, "front-bezel", (0.003, 0.012, 0.012), at=(0.2265, 0.036, 0.056), corner=0.0015, maxd=50),
]
D = [dec("The chassis is four boxes along x (rear panel 12 mm, steel body, 25 mm ABS bezel); the vent plates, intake mesh and drive bay are thin plates 0.8 to 2 mm proud of their faces.",
         "Das Chassis besteht aus drei Quadern entlang x (Rückwand 12 mm, Stahlkörper, 25-mm-ABS-Blende); Lüftungsplatten, Ansauggitter und Laufwerksschacht sind dünne Platten, die 0,8 bis 2 mm vor ihren Flächen stehen."),
     dec("The CPU grille sits on the left (+z) side panel, as the plan view shows its edge at z = +0.092 to +0.1; the elevation draws it although it is seen from the right, the only inconsistency of the 2D shapes. The PSU vent is a rear plate and a top plate (the elevation and the plan draw different extents of the same region).",
         "Das CPU-Gitter sitzt an der linken (+z) Seitenwand, wie der Grundriss seine Kante bei z = +0,092 bis +0,1 zeigt; die Seitenansicht zeichnet es, obwohl sie von rechts blickt, die einzige Unstimmigkeit der 2D-Formen. Der Netzteilschlitz ist eine Rück- und eine Deckelplatte (Seitenansicht und Grundriss zeichnen verschiedene Ausdehnungen derselben Region)."),
     dec("Four rubber feet are two solids repeated along x; slit lines and the unmaterialled fan ring are reduced to a thin rear fan plate, drawn at 1:50 only.",
         "Vier Gummifüße sind zwei entlang x wiederholte Körper; Schlitzlinien und der materiallose Lüfterring sind auf eine dünne Lüfterplatte an der Rückseite reduziert, nur im Maßstab 1:50.")]
write("tower-server", S, D)
