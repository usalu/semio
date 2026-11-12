import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

casing = [(0, 0), (0.065, 0), (0.065, 0.03), (0.185, 0.03)] + bezier((0.185, 0.03), (0.2, 0.045), (0.21, 0.09), (0.21, 0.15)) + [(0.21, 0.245)] + bezier((0.21, 0.245), (0.21, 0.275), (0.195, 0.29), (0.165, 0.29)) + [(0, 0.29)]
vane = [(0, -0.015), (0.11, -0.015)] + bezier((0.11, -0.015), (0.12, -0.015), (0.125, -0.005), (0.125, 0.015)) + [(0.12, 0.015), (0, 0.015)]

S = [
    extrude("casing", "unit", "abs-white", "body", 0.88, "casing", casing, "xy", -0.425, 0.425),
    box("end-cap", "unit", "abs-white", "body", 0.88, "casing", (0.12, 0.03, 0.035), at=(0.125, 0.015, -0.4075), repeat={"count": 2, "translation": [0, 0, 0.815]}),
    box("intake-grille", "unit", "abs-grey", "detail", 0.3, "intake-grille", (0.11, 0.0032, 0.74), at=(0.085, 0.2916, 0), corner=0.0015, maxd=200),
    capsule("grille-slat-z", "unit", "abs-grey", "detail", 0.7, "intake-grille", (0.047, 0.285, -0.36), (0.047, 0.285, 0.36), 0.0035, repeat={"count": 4, "translation": [0.025, 0, 0]}, maxd=50),
    capsule("grille-slat-x", "unit", "abs-grey", "detail", 0.7, "intake-grille", (0.045, 0.281, -0.32), (0.125, 0.281, -0.32), 0.0035, repeat={"count": 9, "translation": [0, 0, 0.08]}, maxd=50),
    box("display-window", "unit", "display", "detail", 0.15, "casing", (0.004, 0.04, 0.06), at=(0.2085, 0.142, 0.35), corner=0.0019, maxd=100),
    extrude("vane", "vane", "abs-grey", "body", 0.6, "discharge-vane", vane, "xy", -0.39, 0.39, maxd=200),
]
D = [dec("The casing is the extrusion of the elevation profile (wall plate, discharge recess of 30 mm, front curvature) over the full 0.85 m length; the recess is closed at both ends by a small end cap (one solid repeated) and spanned by the vane, so the unit reads as a closed body when the vane is shut.",
         "Das Gehäuse ist die Extrusion des Seitenprofils (Wandplatte, 30-mm-Ausblasnische, Frontkrümmung) über die volle Länge von 0,85 m; die Nische ist an beiden Enden durch eine kleine Endkappe (ein wiederholter Körper) geschlossen und von der Lamelle überspannt, sodass das Gerät bei geschlossener Lamelle als geschlossener Körper erscheint."),
     dec("The vane is the extrusion of its profile over 0.78 m on its own bone, hinged at the front edge of the recess; it swings from 0 to -55 degrees without leaving the recess. The grille is a dark panel with four slats along z (elevation) and nine ribs along x (plan), 7 mm thick bars at 1:50 only.",
         "Die Lamelle ist die Extrusion ihres Profils über 0,78 m an ihrem eigenen Knochen, am vorderen Nischenrand angelenkt; sie schwenkt von 0 bis -55 Grad, ohne die Nische zu verlassen. Das Gitter ist eine dunkle Platte mit vier Stäben entlang z (Seitenansicht) und neun Rippen entlang x (Grundriss), 7 mm dick, nur im Maßstab 1:50."),
     dec("The display window is drawn at three different places (elevation y 0.13 to 0.18, front y 0.12 to 0.155 and z 0.32 to 0.38, plan z 0.30 to 0.38); the volume takes the front view for z and y and 4 mm depth. Rounded plan corners (20 mm) and front corners (45 mm) of the casing are not modelled; they are drawn but cost under 1 % of the silhouette.",
         "Das Anzeigefenster ist an drei verschiedenen Orten gezeichnet (Seitenansicht y 0,13 bis 0,18, Vorderansicht y 0,12 bis 0,155 und z 0,32 bis 0,38, Grundriss z 0,30 bis 0,38); das Volumen übernimmt z und y aus der Vorderansicht und 4 mm Tiefe. Die gerundeten Grundrissecken (20 mm) und Frontecken (45 mm) des Gehäuses werden nicht modelliert; sie sind gezeichnet, kosten aber unter 1 % der Silhouette.")]
write("split-ac-indoor-unit", S, D)
