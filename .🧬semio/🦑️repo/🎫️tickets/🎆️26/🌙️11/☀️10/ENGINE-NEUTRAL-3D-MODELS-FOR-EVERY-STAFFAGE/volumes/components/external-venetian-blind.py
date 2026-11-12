import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
F, B, BR = "facade", "box", "bottom-rail"
AL = "aluminium"; BX = "box-and-rails"; FW = "facade-wall"
layers = [("plaster", "masonry", 0.9, -0.27, -0.255), ("block", "masonry", 0.5, -0.255, -0.08), ("eps", "eps", 0.7, -0.08, -0.01), ("render", "masonry", 0.4, -0.01, 0)]
s = []
for id, mat, tone, a, b in layers:
    for band, (y0, y1) in (("sill", (0, 1.0)), ("head", (2.25, 2.75))):
        s.append(L.box("%s-%s" % (id, band), F, mat, "body", tone, [b - a, y1 - y0, 1.2], [(a + b) / 2, (y0 + y1) / 2, 0], FW))
s += [
 L.box("glass-out", F, "glass", "glazing", 0.9, [0.004, 1.25, 1.1], [-0.178, 1.625, 0], "glazing-behind"),
 L.box("glass-in", F, "glass", "glazing", 0.9, [0.004, 1.25, 1.1], [-0.198, 1.625, 0], "glazing-behind"),
]
t = 0.0005
def arch(n=10):
    top = [(-0.045 + 0.09 * k / n, 0.006 - 0.012 * (( -0.045 + 0.09 * k / n) / 0.045) ** 2) for k in range(n + 1)]
    return top + [(u, v - t) for u, v in reversed(top)]
for k in range(17):
    s.append(L.ext("slat-%d" % k, "slat-%d" % k, AL, "detail", 0.35, arch(), "xy", -0.585, 0.585, "lamellae"))
tube = [(0.12, 0), (0.15, 0), (0.15, 0.21), (0, 0.21), (0, 0), (0.03, 0), (0.03, 0.0015), (0.0015, 0.0015), (0.0015, 0.2085), (0.1485, 0.2085), (0.1485, 0.0015), (0.12, 0.0015)]
s.append(L.ext("box", B, AL, "body", 0.45, tube, "xy", -0.585, 0.585, BX))
s.append(L.box("box-end-cap", B, AL, "body", 0.45, [0.15, 0.21, 0.015], [0.075, 0.105, 0.5925], BX, repeat={"count": 2, "translation": [0, 0, -1.185]}))
for side, nm in ((1, "l"), (-1, "r")):
    c = [(0.57 * side, 0.095), (0.6 * side, 0.095), (0.6 * side, 0.115), (0.57 * side, 0.115), (0.57 * side, 0.1135), (0.5985 * side, 0.1135), (0.5985 * side, 0.0965), (0.57 * side, 0.0965)]
    s.append(L.ext("guide-" + nm, B, AL, "detail", 0.3, c, "zx", -1.29, 0, BX))
s.append(L.ext("bottom-rail", BR, AL, "body", 0.4, [(-0.045, -0.02), (-0.045, 0.02), (0.045, 0.02), (0.045, -0.02)], "xy", -0.585, 0.585, BX, holes=[[(-0.0435, -0.0185), (-0.0435, 0.0185), (0.0435, 0.0185), (0.0435, -0.0185)]]))
L.write("external-venetian-blind", s, [
 ("The facade strip is built in its four thermal layers (plaster 15, block 175, EPS 70, render 10 mm) as separate boxes for the sill band below and the head band above the window; the double glazing is two 4 mm panes (the air cavity between them has no material and is left open).", "Der Fassadenstreifen ist in seinen vier thermischen Schichten (Putz 15, Block 175, EPS 70, Putz 10 mm) als getrennte Quader für das Brüstungsband unten und das Sturzband oben aufgebaut; die Doppelverglasung besteht aus zwei 4 mm Scheiben (der Luftraum dazwischen hat kein Material und bleibt offen)."),
 ("The 17 lamellae are 0.5 mm curved sheets (parabolic arch of 12 mm rise over the 90 mm width) each on its own tilting bone, instead of the 12 mm solid bars the 2D views draw to be legible; they run in the C-shaped guide rails and the bottom rail is a hollow 90 x 40 mm tube. The elevation silhouette therefore counts the lamellae by their arch envelope only.", "Die 17 Lamellen sind 0,5 mm dicke gewölbte Bleche (parabolischer Bogen mit 12 mm Pfeilhöhe über die 90 mm Breite), jede an ihrem eigenen Kippknochen, statt der 12 mm dicken Vollstäbe, die die 2D-Ansichten der Lesbarkeit halber zeichnen; sie laufen in den C-förmigen Führungsschienen, und die Endschiene ist ein hohles Rohr 90 x 40 mm. Die Silhouettenprüfung der Ansicht zählt die Lamellen daher nur mit ihrer Bogenhülle."),
 ("The blind box is a 1.5 mm aluminium shell with a bottom slot (one open-section extrude) and two end caps, so the lamellae and the bottom rail disappear into it in the retracted pose; the lamellae bones are slide-n/slat-n pairs and are connected to the rails by their ends only.", "Der Kasten ist eine 1,5 mm Aluminiumschale mit Schlitz unten (eine offene Profilextrusion) und zwei Endkappen, sodass Lamellen und Endschiene in der eingefahrenen Pose darin verschwinden; die Lamellenknochen sind Paare slide-n/slat-n und sind nur mit ihren Enden an den Schienen angebunden."),
])
