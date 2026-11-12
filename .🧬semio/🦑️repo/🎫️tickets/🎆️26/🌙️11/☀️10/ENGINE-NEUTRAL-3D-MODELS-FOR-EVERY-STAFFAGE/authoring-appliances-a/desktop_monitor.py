from lib import *

ST = Ctx("base", "stand-plastic", "stand", "body", 0.5)
NK = ST(bone="neck", tone=0.45)
HS = Ctx("panel", "housing-plastic", "housing", "body", 0.55)
BZ = Ctx("panel", "bezel-plastic", "panel-front", "body", 0.6)
S = []

foot = Ctx("base", "rubber-foot", "stand", "body", 0.15)
for name, x in (("foot-front", 0.096), ("foot-rear", -0.077)):
    S.append(box(name, foot, (x - 0.011, 0, -0.101 - 0.011), (x + 0.011, 0.002, -0.101 + 0.011), repeat={"count": 2, "translation": [0, 0, 0.202]}, maxd=50))

S.append(ext("base-plate", ST, rrect(-0.125, -0.096, 0.125, 0.114, 0.012), "zx", 0.002, 0.0122))

S.append(lf("neck", NK, "v", [(0, 0.035, 0.018, 4), (0.2, 0.03, 0.013, 4), (0.243, 0.032, 0.016, 4)]))
S.append(cyl("hinge", NK(tone=0.35), "w", (0, 0.243, 0), 0.07, 0.012, maxd=50))

def hump(v):
    t = (v + 0.0305) / 0.0265
    def bez(p0, p1, p2, p3, s):
        return (1 - s) ** 3 * p0 + 3 * (1 - s) ** 2 * s * p1 + 3 * (1 - s) * s ** 2 * p2 + s ** 3 * p3
    lo = hi = None
    for k in range(2001):
        s = k / 2000
        y = bez(-0.0305, -0.015, -0.004, -0.004, s)
        if y >= v - 1e-9 and lo is None:
            lo = bez(-0.125, -0.125, -0.105, -0.085, s)
            hi = bez(0.11, 0.11, 0.09, 0.07, s)
    return lo, hi

rows = []
for v, hw in ((-0.0305, 0.265), (-0.0245, 0.261), (-0.0185, 0.253), (-0.0125, 0.242), (-0.0075, 0.23), (-0.004, 0.222)):
    lo, hi = hump(v)
    rows.append((v, hw, (hi - lo) / 2, 4, 0, (hi + lo) / 2))
S.append(lf("housing", HS, "v", rows))
S.append(box("vents", HS(role="detail", tone=0.3, region=None), (0.012, -0.0045, -0.003), (0.068, -0.0025, 0.003), repeat={"count": 5, "translation": [0, 0, 0.075]}, tr=(0, 0, -0.15), maxd=50))

S.append(ext("panel-slab", BZ, rrect(-0.27, -0.17, 0.27, 0.153, 0.006), "zx", -0.04, -0.0305))
S.append(box("screen-glass", BZ(material="display-glass", role="glazing", tone=0.1), (-0.158, -0.0402, -0.255), (0.141, -0.0384, 0.255)))
S.append(cyl("power-led", BZ(role="detail", tone=0.9), "v", (-0.164, -0.0405, -0.24), 0.0006, 0.003, maxd=50))

D = [
    text("Bone frames: the panel bone is the neck tip turned 95 degrees, so its u axis runs up the screen (5 degrees backwards) and v points backwards; the screen faces +x. All panel solids are written in that frame, with lateral extents from plan and front: 0.54 m wide bezel, 0.51 m glass, 0.25 m wide base.",
         "Knochenrahmen: der Knochen Panel ist die um 95 Grad gedrehte Halsspitze, seine u-Achse läuft den Bildschirm hinauf (5 Grad nach hinten) und v zeigt nach hinten; der Bildschirm blickt nach +x. Alle Panelkörper sind in diesem Rahmen geschrieben, mit seitlichen Massen aus Grundriss und Ansicht: 0,54 m breite Blende, 0,51 m Glas, 0,25 m breiter Fuss."),
    text("The rear housing hump is a loft of rounded-rectangle sections along v whose u extents are read from the drawn elevation curve and whose width tapers from 0.53 m to 0.44 m; the 2D plan draws it as wide as the bezel, a monitor hump is physically narrower. The neck is a tapered loft with the front-view widths and elevation depths.",
         "Der hintere Gehäusebuckel ist ein Loft aus Rechteckabschnitten mit Rundung entlang v, dessen u-Ausdehnung der gezeichneten Ansichtskurve entnommen ist und dessen Breite von 0,53 m auf 0,44 m abnimmt; der Grundriss zeichnet ihn so breit wie die Blende, ein Monitorbuckel ist aber physikalisch schmaler. Der Hals ist ein sich verjüngender Loft mit den Breiten der Ansicht und den Tiefen des Aufrisses."),
    text("The panel bone carries bezel, glass, LED, housing and vents so the tilt joint (-5 to 20 degrees) turns them together about the hinge cylinder on the neck bone. Feet, hinge barrel, vent slats and LED carry maxDenominator 50 like the 1:100 drawing that drops them.",
         "Der Knochen Panel trägt Blende, Glas, LED, Gehäuse und Lüftung, sodass das Neigegelenk (-5 bis 20 Grad) sie gemeinsam um den Scharnierzylinder am Halsknochen dreht. Füsse, Scharnier, Lüftungsrippen und LED tragen maxDenominator 50 wie die 1:100-Zeichnung, die sie weglässt."),
]
write("desktop-monitor", S, D)
