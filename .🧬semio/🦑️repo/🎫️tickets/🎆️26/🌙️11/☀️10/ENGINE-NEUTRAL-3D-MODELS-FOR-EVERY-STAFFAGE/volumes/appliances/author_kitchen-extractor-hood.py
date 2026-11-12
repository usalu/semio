import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

centres = [(-0.2, 0.1), (0.2, 0.1), (-0.2, 0.37), (0.2, 0.37)]
hole = lambda cz, cx: [(cz + 0.03 * math.cos(math.radians(a)), cx + 0.03 * math.sin(math.radians(a))) for a in range(0, 360, 30)]
S = [
    extrude("canopy", "canopy", "stainless-steel", "body", 0.55, "canopy", [(0, 0.02), (0.5, 0.02), (0.5, 0.06), (0.4, 0.2), (0, 0.2)], "xy", -0.45, 0.45),
    extrude("filter", "canopy", "filter-mesh", "body", 0.7, "filter", rrect(0, 0.25, 0.86, 0.46), "zx", 0, 0.02, holes=[hole(cz, cx) for cz, cx in centres], maxd=100),
    revolve("lamp-l", "canopy", "lamp-diffuser", "detail", 0.95, "lamps", [(0, 0.028), (0.0015, 0.029), (0.012, 0.029)], axis=(0, 1, 0), translate=[0.1, 0, -0.2], repeat={"count": 2, "translation": [0.27, 0, 0]}, maxd=50),
    revolve("lamp-r", "canopy", "lamp-diffuser", "detail", 0.95, "lamps", [(0, 0.028), (0.0015, 0.029), (0.012, 0.029)], axis=(0, 1, 0), translate=[0.1, 0, 0.2], repeat={"count": 2, "translation": [0.27, 0, 0]}, maxd=50),
    box("chimney-lower", "chimney", "stainless-steel", "body", 0.6, "chimney", (0.3, 0.35, 0.3), at=(0.15, 0.175, 0), corner=0.002),
    box("chimney-upper", "chimney-top", "stainless-steel", "body", 0.65, "chimney", (0.28, 0.35, 0.28), at=(0.14, 0.175, 0), corner=0.002),
    box("collar", "chimney-top", "stainless-steel", "body", 0.45, "chimney", (0.2, 0.015, 0.2), at=(0.14, 0.3575, 0), corner=0.002, maxd=100),
]
D = [dec("The canopy is the extrusion of its elevation profile (flat back at the wall, 140 mm sloped front panel) across the 0.9 m width drawn in front and plan; the chimney sections and the collar are square boxes of their plan footprints.",
         "Die Haube ist die Extrusion ihres Seitenprofils (flache Rückwand an der Wand, 140 mm geneigte Frontblende) über die in Vorderansicht und Grundriss gezeichnete Breite von 0,9 m; die Kaminteile und der Kragen sind quadratische Quader ihrer Grundrissflächen."),
     dec("The grease filter is an extrusion of the filter outline with four round openings in which the four lamp discs sit: the two views draw lamps at two x positions and at two z positions, read as a 2 x 2 grid (two repeated pairs).",
         "Der Fettfilter ist eine Extrusion der Filterkontur mit vier runden Öffnungen, in denen die vier Lampenscheiben sitzen: die beiden Ansichten zeigen Lampen an zwei x-Lagen und an zwei z-Lagen, gelesen als Raster 2 x 2 (zwei wiederholte Paare)."),
     dec("The canopy is modelled as a solid body (not a hollow shell); hairline details (panel edge, lip, seam) are not modelled.",
         "Die Haube ist als Vollkörper modelliert (keine Hohlschale); Haarliniendetails (Blendenkante, Lippe, Naht) werden nicht modelliert.")]
write("kitchen-extractor-hood", S, D)
