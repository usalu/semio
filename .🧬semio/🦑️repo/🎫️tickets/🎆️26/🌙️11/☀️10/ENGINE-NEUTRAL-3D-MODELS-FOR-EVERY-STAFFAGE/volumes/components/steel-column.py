import sys, os, math
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
C = "column"; M = "steel"
s = []
for sign, name in ((1, "a"), (-1, "b")):
    s.append(L.box("flange-" + name, C, M, "body", 0.45, [2.96, 0.015, 0.2], [1.5, sign * 0.0925, 0], "column-flange"))
s.append(L.box("web", C, M, "body", 0.6, [2.96, 0.17, 0.008], [1.5, 0, 0], "column-web"))
n = 0
for sv in (1, -1):
    for sw in (1, -1):
        n += 1
        tri = [(sv * 0.085, sw * 0.004), (sv * 0.0655, sw * 0.004), (sv * 0.085, sw * 0.0235)]
        s.append(L.ext("fillet-%d" % n, C, M, "detail", 0.3, tri, "yz", 0.14, 2.98, "column-web", maxD=50))
s.append(L.box("base-plate", C, M, "body", 0.3, [0.02, 0.3, 0.26], [0.01, 0, 0], "base-plate"))
for sv, name in ((1, "a"), (-1, "b")):
    s.append(L.ext("stiffener-" + name, C, M, "body", 0.3, [(0.02, sv * 0.1), (0.02, sv * 0.15), (0.11, sv * 0.13), (0.11, sv * 0.1)], "xy", -0.008, 0.008, "base-plate"))
hexa = lambda cv: [(cv + 0.0137 * math.cos(math.radians(60 * k)), 0.0137 * math.sin(math.radians(60 * k))) for k in range(6)]
for sv, name in ((1, "a"), (-1, "b")):
    s.append(L.ext("anchor-nut-" + name, C, M, "detail", 0.1, [(a, b - 0.1) for a, b in hexa(sv * 0.125)], "yz", 0.02, 0.038, "base-plate", maxD=50, repeat={"count": 2, "translation": [0, 0, 0.2]}))
    s.append(L.cap("anchor-bolt-" + name, C, M, "detail", 0.1, [0.038, sv * 0.125, -0.1], [0.075, sv * 0.125, -0.1], 0.005, region="base-plate", maxD=50, repeat={"count": 2, "translation": [0, 0, 0.2]}))
s.append(L.box("cap-plate", C, M, "body", 0.3, [0.02, 0.24, 0.22], [2.99, 0, 0], "cap-plate"))
for sv, name in ((1, "a"), (-1, "b")):
    s.append(L.box("splice-plate-" + name, C, M, "detail", 0.3, [0.3, 0.006, 0.2], [1.5, sv * 0.103, 0], "column-flange", maxD=50))
L.write("steel-column", s, [
 ("The column is an I-section built from its own solids so that the thermal and structural views can colour them: two flanges 15 mm thick and 200 mm wide in the planes v = +-0.0925, a web of 8 mm (the 2D views draw no web thickness, taken as a typical value) and four fillet triangles of 19.5 mm legs at the web to flange corners.", "Die Stütze ist ein I-Profil aus eigenen Körpern, damit die thermische und die statische Ansicht sie einfärben können: zwei Flansche, 15 mm dick und 200 mm breit, in den Ebenen v = +-0,0925, ein Steg von 8 mm (die 2D-Ansichten zeichnen keine Stegdicke, typischer Wert angenommen) und vier Ausrundungsdreiecke mit 19,5 mm Schenkeln an den Steg-Flansch-Ecken."),
 ("Base plate 300 x 260 x 20 mm, two trapezoid stiffeners of 16 mm, four hexagon anchor nuts with bolt stubs (two solids each with repeat) and the 240 x 220 x 20 mm cap plate follow the elevation, front and plan outlines.", "Fussplatte 300 x 260 x 20 mm, zwei trapezförmige Steifen von 16 mm, vier Sechskant-Ankermuttern mit Gewindestummeln (je zwei Körper mit Wiederholung) und die Kopfplatte 240 x 220 x 20 mm folgen den Umrissen in Ansicht, Vorder- und Draufsicht."),
 ("The 2D splice line at mid height is modelled as two 6 mm flange cover plates (300 mm long, only at 1:50); the eccentric load carrier bone is a load point without volume and the bone axis points up, so v = -x ahead.", "Die 2D-Stosslinie auf halber Höhe ist als zwei 6 mm dicke Flanschlaschen (300 mm lang, nur bei 1:50) modelliert; der Knochen des exzentrischen Lastträgers ist ein Lastpunkt ohne Volumen und die Knochenachse zeigt nach oben, also v = -x nach vorn."),
])
