import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

deck = [(-0.109, 0.0066), (0.109, 0.0066), (0.109, 0.0092), (0.078, 0.0125), (-0.1055, 0.0125), (-0.1085, 0.0121), (-0.109, 0.0105)]

S = [
    revolve("foot-l", "base", "rubber-foot", "body", 0.15, "underside", [(0, 0.0095), (0.002, 0.009)], axis=(0, 1, 0), translate=[-0.09, 0, 0.11], repeat={"count": 2, "translation": [0.18, 0, 0]}, maxd=50),
    revolve("foot-r", "base", "rubber-foot", "body", 0.15, "underside", [(0, 0.0095), (0.002, 0.009)], axis=(0, 1, 0), translate=[-0.09, 0, -0.11], repeat={"count": 2, "translation": [0.18, 0, 0]}, maxd=50),
    box("base-lower", "base", "alu-anodised", "body", 0.5, "underside", (0.218, 0.0046, 0.313), at=(0, 0.0043, 0), corner=0.0012, maxd=100),
    extrude("base-upper", "base", "alu-anodised", "body", 0.62, "deck", deck, "xy", -0.1565, 0.1565, maxd=100),
    box("hinge-shoulder", "base", "alu-anodised", "body", 0.58, "deck", (0.034, 0.0105, 0.28), at=(-0.092, 0.01185, 0), corner=0.003, maxd=50),
    box("keys", "base", "alu-anodised", "detail", 0.3, "deck", (0.017, 0.0012, 0.23), at=(-0.0595, 0.0131, 0), corner=0.0004, repeat={"count": 5, "translation": [0.021, 0, 0]}, maxd=100),
    box("trackpad", "base", "alu-anodised", "detail", 0.55, "deck", (0.036, 0.0004, 0.08), at=(0.074, 0.0127, 0), corner=0.0002, maxd=50),
    box("lid-shell", "lid", "alu-anodised", "body", 0.55, "lid", (0.218, 0.0065, 0.313), at=(0.104, 0.00325, 0), corner=0.0022),
    box("display", "lid", "display-glass", "glazing", 0.1, "lid", (0.203, 0.0028, 0.29), at=(0.1065, 0.0012, 0), maxd=50),
    revolve("hinge", "lid", "alu-anodised", "body", 0.4, "lid", [(-0.12, 0.0048), (0.12, 0.0048)], axis=(0, 0, 1), origin=(0.002, 0.00325, 0), maxd=50),
]
D = [dec("The dimensions (0.275 x 0.218 x 0.313 m) describe the open laptop (poses open, 105 degrees), whereas the rig rest, every 1:50 shape and this volume are the closed laptop (lid angle 0); at rest the volume is therefore 0.218 x 0.019 x 0.313 m and misses the x and y bounding-box gates by construction, while the poses open and open-idle reach 0.275 m wide.",
         "Die Abmessungen (0,275 x 0,218 x 0,313 m) beschreiben das aufgeklappte Notebook (Posen open, 105 Grad), während Ruhelage des Gerüsts, alle 1:50-Formen und dieses Volumen das geschlossene Notebook sind (Deckelwinkel 0); in Ruhe misst das Volumen daher 0,218 x 0,019 x 0,313 m und verfehlt die Hüllquader-Grenzen in x und y systembedingt, während die Posen open und open-idle 0,275 m Breite erreichen."),
     dec("Base and lid are rounded boxes; the deck is the extrusion of the elevation profile with its 3 mm front chamfer; the display is a glass plate standing 0.2 mm proud of the lid underside so it shows when the lid opens; the hinge is one barrel across the back on the lid bone.",
         "Boden und Deckel sind abgerundete Quader; das Deck ist die Extrusion des Seitenprofils mit der 3-mm-Fase vorn; das Display ist eine Glasplatte, die 0,2 mm aus der Deckelunterseite ragt, damit sie beim Öffnen sichtbar wird; das Scharnier ist eine Walze über die Rückseite am Deckelknochen."),
     dec("Five key rows are one box repeated along x (pitch 21 mm), four rubber feet are two solids repeated along x; the hinge shoulder width (0.28 m), the barrel length and the key area are not drawn in the front view and are estimates.",
         "Fünf Tastenreihen sind ein entlang x wiederholter Quader (Teilung 21 mm), vier Gummifüße zwei entlang x wiederholte Körper; Breite der Scharnierschulter (0,28 m), Walzenlänge und Tastenbereich sind in der Vorderansicht nicht gezeichnet und geschätzt.")]
write("laptop", S, D)
