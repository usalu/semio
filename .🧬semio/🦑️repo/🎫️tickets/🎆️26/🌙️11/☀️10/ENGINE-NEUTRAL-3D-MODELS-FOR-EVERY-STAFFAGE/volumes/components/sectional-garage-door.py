import sys, os, math
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
T = "track"; STEEL = "steel"; PS = "panel-sandwich"; HW = "hardware-steel"; TS = "track-steel"; SA = "spring-assembly"
Q = math.sqrt(0.5)
def c_profile(side, a0, a1):
    z0, z1 = (1.21 * side, 1.25 * side)
    w0, w1 = (1.21 * side, 1.247 * side)
    return [(z0, a0), (z1, a0), (z1, a1), (z0, a1), (z0, a1 - 0.003), (w1, a1 - 0.003), (w1, a0 + 0.003), (z0, a0 + 0.003)]
s = []
SEG = 10
half = math.pi / (4 * SEG)
chord = 2 * 0.5 * math.sin(half) + 0.006
mid = (0.5 + 0.5 * math.cos(math.pi - half), 2.0 + 0.5 * math.sin(math.pi - half))
for side, nm in ((1, "l"), (-1, "r")):
    cp = c_profile(side, -0.02, 0.02)
    s.append(L.ext("rail-up-" + nm, T, STEEL, "detail", 0.4, cp, "zx", 0, 2.0, TS))
    s.append(L.ext("rail-arc-" + nm, T, STEEL, "detail", 0.4, cp, "zx", -chord / 2, chord / 2, TS,
                   transform={"rotation": [0, 0, -math.sin(half / 2), math.cos(half / 2)], "translation": [mid[0], mid[1], 0]},
                   repeat={"count": SEG, "rotationAxis": [0, 0, 1], "rotationRad": round(-2 * half, 7), "origin": [0.5, 2.0, 0]}))
    s.append(L.ext("rail-back-" + nm, T, STEEL, "detail", 0.4, [(2.5 + u, w) for w, u in cp], "yz", 0.5, 2.6, TS))
s.append(L.box("track-end-bar", T, STEEL, "detail", 0.4, [0.02, 0.04, 2.42], [2.59, 2.5, 0], TS))
for side, nm in ((1, "l"), (-1, "r")):
    s.append(L.box("hanger-" + nm, T, STEEL, "detail", 0.4, [0.04, 0.35, 0.03], [1.3, 2.675, 1.23 * side], TS, repeat={"count": 2, "translation": [1.2, 0, 0]}))
for side, nm in ((1, "l"), (-1, "r")):
    s.append(L.cyl("hanger-pin-" + nm, T, STEEL, "detail", 0.2, [1.3, 2.85, 1.215 * side], [0, 0, side], 0.03, 0.011, TS, maxD=50, repeat={"count": 2, "translation": [1.2, 0, 0]}))
s.append(L.box("bearing-plate", T, STEEL, "detail", 0.4, [0.2, 0.65, 0.01], [0.12, 2.525, 1.205], TS, repeat={"count": 2, "translation": [0, 0, -2.41]}))
for n in range(1, 5):
    P = "panel-%d" % n
    s.append(L.box("skin-outer-%d" % n, P, "sandwich-skin", "body", 0.55, [0.5, 0.0005, 2.42], [0.25, 0.01975, 0], PS))
    s.append(L.box("core-%d" % n, P, "pu-foam", "body", 0.85, [0.5, 0.039, 2.42], [0.25, 0, 0], PS))
    s.append(L.box("skin-inner-%d" % n, P, "sandwich-skin", "body", 0.55, [0.5, 0.0005, 2.42], [0.25, -0.01975, 0], PS))
    rep = {"count": 2, "translation": [-0.5, 0, 0]} if n == 1 else None
    for side, nm in ((1, "l"), (-1, "r")):
        s.append(L.rev("roller-%d-%s" % (n, nm), P, STEEL, "detail", 0.2, [(0, 0.004), (0.01, 0.004), (0.01, 0.011), (0.03, 0.011)], [0, 0, side], [0.5, 0, 1.205 * side], region=HW, maxD=50, repeat=rep))
    if n > 1:
        s.append(L.box("hinge-%d" % n, P, STEEL, "detail", 0.3, [0.1, 0.025, 0.12], [0, -0.0325, -0.8], HW, maxD=50, repeat={"count": 5, "translation": [0, 0, 0.4]}))
s.append(L.box("bottom-seal", "panel-1", STEEL, "detail", 0.15, [0.07, 0.044, 2.42], [0.035, 0, 0], HW))
s.append(L.box("bottom-bracket", "panel-1", STEEL, "detail", 0.3, [0.1, 0.02, 0.12], [0.05, -0.03, 1.15], HW, maxD=50, repeat={"count": 2, "translation": [0, 0, -2.3]}))
S = "spring-shaft"
s += [
 L.rev("shaft", S, STEEL, "detail", 0.2, [(0, 0.0125), (2.42, 0.0125)], [0, 0, 1], [0, 0, -1.21], region=SA, maxD=50),
 L.rev("drum", S, STEEL, "body", 0.5, [(0, 0.09), (0.1, 0.09)], [0, 0, 1], [0, 0, 1.1], region=SA, repeat={"count": 2, "translation": [0, 0, -2.3]}),
 L.rev("coil", S, STEEL, "detail", 0.4, [(k * 0.0125 + d, r) for k in range(80) for d, r in ((0, 0.031), (0.004, 0.0375), (0.0085, 0.0375))], [0, 0, 1], [0, 0, -0.5], region=SA, maxD=50),
 L.box("drum-index", S, STEEL, "detail", 0.9, [0.0525, 0.004, 0.002], [0.06375, 0, -1.201], SA, maxD=50, repeat={"count": 2, "translation": [0, 0, 2.402]}),
]
L.write("sectional-garage-door", s, [
 ("Each of the four 500 x 2420 mm panels is a sandwich of three boxes through its 40 mm thickness (0.5 mm outer skin on the street side +v, 39 mm PU foam core, 0.5 mm inner skin) bound to its own bone, so the thermal view colours skin and core and the door folds with the rig; the 2D layers sum to 41 mm, 40 mm is used.", "Jedes der vier Paneele 500 x 2420 mm ist ein Sandwich aus drei Quadern über seine 40 mm Dicke (0,5 mm Aussenhaut an der Strassenseite +v, 39 mm PU-Schaumkern, 0,5 mm Innenhaut) an einem eigenen Knochen, sodass die thermische Ansicht Haut und Kern einfärbt und die Tür sich mit dem Rig faltet; die 2D-Schichten summieren sich auf 41 mm, es gelten 40 mm."),
 ("The guide rails are 40 x 40 mm C-sections open towards the door at z = +-1.23: a vertical extrude, a quarter-circle of 0.5 m radius as ten straight segments (one solid with a rotating repeat) and a horizontal extrude, plus the end bar; the elevation draws the rails only as open polylines, so the elevation silhouette gate counts them as extra area.", "Die Führungsschienen sind 40 x 40 mm C-Profile, zur Tür hin offen, bei z = +-1,23: eine senkrechte Extrusion, ein Viertelkreis mit 0,5 m Radius aus zehn geraden Segmenten (ein Körper mit drehender Wiederholung) und eine waagrechte Extrusion, dazu der Endbügel; die Ansicht zeichnet die Schienen nur als offene Polylinien, daher zählt die Silhouettenprüfung der Ansicht sie als Mehrfläche."),
 ("Roller wheels with axle stubs run inside the C-sections at every panel end and the bottom; hinge blocks (five per joint) and corner brackets sit on the inner face (only at 1:50); the torsion spring is a shaft, two cable drums, a ridged coil of 80 turns and index marks, all on the rotating spring-shaft bone; the 10 mm bearing plates between drum and rail carry the spring shaft (an addition the 2D views do not draw, needed to keep the spring attached); hanger top pins reach the 2.861 m overall height; lift cables, panel ribs and the two carriage bones (load points) have no solids.", "Laufrollen mit Achsstummeln laufen in den C-Profilen an jedem Paneelende und unten; Scharnierblöcke (fünf je Gelenk) und Eckwinkel sitzen auf der Innenseite (nur bei 1:50); die Torsionsfeder ist eine Welle, zwei Seiltrommeln, eine gerippte Wendel aus 80 Windungen und Indexmarken, alles am drehenden Knochen spring-shaft; die 10 mm Lagerplatten zwischen Trommel und Schiene tragen die Federwelle (eine Ergänzung, die die 2D-Ansichten nicht zeichnen und die die Feder angebunden hält); die Aufhängerbolzen oben erreichen die Gesamthöhe von 2,861 m; Hubseile, Paneelrippen und die beiden Schlittenknochen (Lastpunkte) haben keine Körper."),
])
