import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

FR = dict(region="frame", material="aluminium-frame")
FB = dict(regions=["frame", "board-face"], material="enamel-board")
BF = dict(region="board-face", material="enamel-board")
TR = dict(region="tray", material="aluminium-frame")
LF = dict(region="leaf", material="aluminium-frame")
LB = dict(region="leaf", material="enamel-board")

views = {}

views[200] = (
    [rect("end-tray", "tray", -0.04, 0, 0.08, 0.07, z=3.2, tone=0.65, **TR),
     rect("end-frame", "frame", -0.04, 0.07, 0.08, 1.19, z=3, tone=0.5, **FB),
     rect("end-leaf", "leaf", -0.04, 0.07, 0.035, 1.19, z=4, tone=0.35, **LB)],
    [rect("top-frame", "frame", -2, -0.04, 4, 0.08, z=3, tone=0.5, **FB),
     rect("top-leaf", "leaf", -2, -0.04, 2, 0.035, z=4, tone=0.35, **LB)],
    {"silhouette": ("Front: the end of the board as a 0.08 m strip (tray, frame with panels, leaf in front); plan: the 4 m top edge with the leaf as a front strip that slides with it.",
                    "Vorderansicht: die Stirnseite der Tafel als Streifen von 0,08 m (Ablage, Rahmen mit Platten, Flügel davor); Draufsicht: die 4 m lange Oberkante mit dem Flügel als vorderem Streifen, der mit ihm mitgleitet."),
     "omitted": ("Tray, board faces and chalk are not visible in plan, the panel edges not in front.",
                 "Ablage, Tafelflächen und Kreide sind in der Draufsicht nicht sichtbar, die Plattenkanten nicht in der Vorderansicht.")})

views[100] = (
    [rect("end-tray", "tray", -0.04, 0, 0.08, 0.06, z=3.2, tone=0.65, **TR),
     rect("end-frame", "frame", -0.04, 0.06, 0.08, 1.2, z=3, tone=0.75, **FR),
     rect("end-panel", "frame", 0.0, 0.1, 0.03, 1.12, z=3.1, tone=0.3, **BF),
     rect("end-leaf-frame", "leaf", -0.04, 0.06, 0.035, 1.2, z=4, tone=0.85, **LF),
     rect("end-leaf-face", "leaf", -0.032, 0.1, 0.02, 1.12, z=4.1, tone=0.24, **LB)],
    [rect("top-frame", "frame", -2, -0.04, 4, 0.08, z=3, tone=0.75, **FR),
     rect("top-leaf-frame", "leaf", -2, -0.04, 2, 0.035, z=4, tone=0.85, **LF),
     rect("top-leaf-face", "leaf", -1.96, -0.032, 1.92, 0.02, z=4.1, tone=0.24, **LB)],
    {"silhouette": ("Front: the end cap of the aluminium frame with the board panel edge behind and the leaf rail and leaf panel edge in front; plan: the frame top rail with the leaf's top rail and panel edge sliding over it.",
                    "Vorderansicht: die Stirnkappe des Aluminiumrahmens mit der Plattenkante dahinter sowie Flügelrahmen und Flügelplattenkante davor; Draufsicht: die obere Rahmenschiene mit der oberen Flügelschiene und der Plattenkante, die darüber gleitet."),
     "retained": ("Leaf and frame stay separate solids bound to their own bones, so plan follows every leaf pose and clip; in front the leaf stays in its channel.",
                  "Flügel und Rahmen bleiben getrennte Körper an ihren eigenen Knochen, die Draufsicht folgt daher jeder Flügelpose und jedem Clip; in der Vorderansicht bleibt der Flügel in seiner Laufschiene.")})

views[50] = (
    [rect("end-tray", "tray", -0.04, 0, 0.08, 0.06, z=3.2, tone=0.65, **TR),
     rect("end-tray-lip", "tray", -0.032, 0.006, 0.064, 0.016, z=3.25, tone=0.35, role="detail", **TR),
     rect("end-frame", "frame", -0.04, 0.06, 0.08, 1.2, z=3, tone=0.75, **FR),
     rect("end-frame-profile", "frame", -0.034, 0.072, 0.068, 1.176, z=3.05, tone=0.35, role="detail", **FR),
     rect("end-panel", "frame", 0.0, 0.095, 0.03, 1.13, z=3.1, tone=0.3, **BF),
     rect("end-leaf-frame", "leaf", -0.04, 0.06, 0.035, 1.2, z=4, tone=0.85, **LF),
     rect("end-leaf-face", "leaf", -0.032, 0.095, 0.02, 1.13, z=4.1, tone=0.24, **LB),
     rect("end-leaf-profile", "leaf", -0.037, 0.072, 0.029, 1.176, z=4.05, tone=0.35, role="detail", **LF)],
    [rect("top-frame", "frame", -2, -0.04, 4, 0.08, z=3, tone=0.75, **FR),
     rect("top-frame-profile", "frame", -1.988, -0.034, 3.976, 0.068, z=3.05, tone=0.35, role="detail", **FR),
     rect("top-leaf-frame", "leaf", -2, -0.04, 2, 0.035, z=4, tone=0.85, **LF),
     rect("top-leaf-profile", "leaf", -1.988, -0.037, 1.976, 0.029, z=4.05, tone=0.35, role="detail", **LF),
     rect("top-leaf-face", "leaf", -1.965, -0.032, 1.93, 0.02, z=4.1, tone=0.24, **LB)],
    {"silhouette": ("Front: frame end cap with its inner profile line, panel edge, tray end with lip, and the leaf rail with its own profile line and panel edge; plan: frame rail and leaf rail each with their profile line and the leaf panel edge.",
                    "Vorderansicht: Rahmenstirnkappe mit innerer Profillinie, Plattenkante, Ablagenende mit Lippe sowie Flügelschiene mit eigener Profillinie und Plattenkante; Draufsicht: Rahmenschiene und Flügelschiene mit je ihrer Profillinie und der Flügelplattenkante."),
     "omitted": ("Chalk, sponge and grips lie below the top edge and behind the end cap and do not appear in front or plan.",
                 "Kreide, Schwamm und Griffe liegen unter der Oberkante und hinter der Stirnkappe und erscheinen weder in der Vorder- noch in der Draufsicht.")})

apply("chalkboard", views)
