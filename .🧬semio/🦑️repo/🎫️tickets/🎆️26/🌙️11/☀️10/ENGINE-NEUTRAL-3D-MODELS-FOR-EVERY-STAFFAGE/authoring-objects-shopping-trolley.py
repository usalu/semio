import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

ZW, ZT, PG, PU = "zinc-wire", "zinc-tube", "pp-grip", "pu-tyre"
A = lib.Asset("shopping-trolley", ZW, 0.08, "basket-wire")

def rrect(hz, hx, c, cx):
    return [[-hz + c, cx - hx], [-hz, cx - hx + c], [-hz, cx + hx - c], [-hz + c, cx + hx], [hz - c, cx + hx], [hz, cx + hx - c], [hz, cx - hx + c], [hz - c, cx - hx]]

S = 0.46 / 0.32
s = A.meta("basket-shell", "basket", {})
t = 0.004
body = {"profile": rrect(0.2, 0.32, 0.02, 0.028), "holes": [rrect(0.2 - t, 0.32 - t, 0.02, 0.028)], "plane": "zx", "from": 0, "to": 0.5, "taperTo": [1, round(S, 5)]}
A.finish(s, "extrude", body)
A.box("basket-floor", "basket", [0.02, 0.383, 0], [0.64, 0.006, 0.4], tone=0.08)
def at(y):
    k = (y - 0.38) / 0.5
    return 0.028 * (1 + 0.43 * k) + 0.0, 0.32 * (1 + 0.4375 * k), 0.2 * (1 + 0.4263 * k)
for k, y in enumerate((0.5, 0.62, 0.74, 0.86)):
    cx, hx, hz = at(y)
    cx = 0.0286 * (1 + 0) + (0.04 - 0.028) * (y - 0.38) / 0.5
    pts = [[cx - hx, y, -hz], [cx - hx, y, hz], [cx + hx, y, hz], [cx + hx, y, -hz]]
    for i in range(4):
        A.cap(f"wire-{k + 1}-{i + 1}", "basket", pts[i], pts[(i + 1) % 4], 0.0035, tone=0.3, role="detail", maxDenominator=100)
cx, hx, hz = 0.04, 0.46, 0.285
rim = [[cx - hx, 0.88, -hz], [cx - hx, 0.88, hz], [cx + hx, 0.88, hz], [cx + hx, 0.88, -hz]]
for i in range(4):
    A.cap(f"rim-{i + 1}", "basket", rim[i], rim[(i + 1) % 4], 0.006, tone=0.3, role="detail")

fr = [[-0.32, 0.17, -0.215], [-0.32, 0.17, 0.215], [0.36, 0.17, 0.215], [0.36, 0.17, -0.215]]
for i in range(4):
    A.cap(f"chassis-{i + 1}", "chassis", fr[i], fr[(i + 1) % 4], 0.007, material=ZT, tone=0.3, region="frame-tube")
for sz in (1, -1):
    nm = "l" if sz > 0 else "r"
    for fb, (xt, xm, xb) in (("rear", (-0.3, -0.32, -0.3)), ("front", (0.34, 0.36, 0.33))):
        A.cap(f"fork-{fb}-{nm}-upper", "chassis", [xt, 0.38, sz * 0.2], [xm, 0.17, sz * 0.215], 0.007, material=ZT, tone=0.3, region="frame-tube")
        A.cap(f"fork-{fb}-{nm}-lower", "chassis", [xm, 0.17, sz * 0.215], [xb, 0.0625, sz * 0.2], 0.007, material=ZT, tone=0.3, region="frame-tube")
        xw = xb
        zc = sz * 0.22
        z0 = zc - 0.02
        A.rev(f"wheel-{fb}-{nm}", "chassis", [xw, 0.0625, z0], [[0, 0], [0, 0.0575], [0.005, 0.0625], [0.035, 0.0625], [0.04, 0.0575], [0.04, 0]], axis="z", material=PU, tone=0.15, region="wheels")
        A.rev(f"hub-{fb}-{nm}", "chassis", [xw, 0.0625, z0 - 0.002], [[0, 0], [0, 0.022], [0.044, 0.022], [0.044, 0]], axis="z", material=PU, tone=0.7, role="detail", region="wheels", maxDenominator=100)
    A.box(f"castor-plate-{nm}", "chassis", [0.36, 0.1725, sz * 0.21], [0.05, 0.015, 0.05], material=ZT, tone=0.6, region="frame-tube")

for sz, nm in ((1, "l"), (-1, "r")):
    A.cap(f"handle-post-{nm}", "handle", [-0.4, 0.88, sz * 0.245], [-0.38, 0.97, sz * 0.245], 0.0125, material=ZT, tone=0.3, region="frame-tube")
    A.cap(f"handle-arm-{nm}", "handle", [-0.37, 0.98, sz * 0.245], [-0.51, 0.98, sz * 0.245], 0.015, material=ZT, tone=0.3, region="frame-tube")
A.cap("grip", "handle", [-0.51, 0.98, -0.245], [-0.53, 0.98, 0.245], 0.02, material=PG, tone=0.7, region="handle-grip")

gate = [(0.014552, 0.003638), (0.134552, -0.476362), (0.105448, -0.483638), (-0.014552, -0.003638)]
A.ext("gate", "gate", [[-0.42 + x, 0.88 + y] for x, y in gate], -0.24, 0.24, material=ZW, tone=0.5, region="gate-wire")
A.cap("gate-hinge", "basket", [-0.42, 0.88, -0.24], [-0.42, 0.88, 0.24], 0.008, tone=0.3, role="detail", region="gate-wire")
for sz, nm in ((1, "l"), (-1, "r")):
    A.ell(f"gate-eye-{nm}", "gate", [-0.42, 0.88, sz * 0.24], [0.014, 0.014, 0.014], material=ZW, tone=0.9, role="detail", region="gate-wire", maxDenominator=100)
A.save([
    ("The wire basket is one tapered shell (an extrusion of a rounded-rectangle ring with taper, 4 mm sheet, 0.92 x 0.57 m at the rim to 0.64 x 0.40 m at the floor) with a floor plate, a 12 mm rim tube and four wire loops of 7 mm as the visible wire grid; the individual wires of the drawing are not modelled one by one, the shell stands for the mesh.",
     "Der Drahtkorb ist eine einzige konische Schale (Extrusion eines abgerundeten Rechteckrings mit Verjüngung, 4 mm Blech, 0,92 x 0,57 m am Rand bis 0,64 x 0,40 m am Boden) mit Bodenplatte, 12 mm Randrohr und vier Drahtschlaufen von 7 mm als sichtbares Drahtgitter; die einzelnen Drähte der Zeichnung werden nicht einzeln modelliert, die Schale steht für das Gitter."),
    ("The chassis is a rectangular tube frame at y 0.17 with four fork legs (two bent tubes per side), four tyres (revolves about z, 0.125 m diameter, 0.04 m wide, at z +-0.22 as the plan shows) with hub discs and two castor plates at the front. The handle is a U of two posts, two arms and a rubber grip across z at x -0.52 as the plan shows.",
     "Das Fahrgestell ist ein Rechteckrohrrahmen auf y 0,17 mit vier Gabelbeinen (je zwei gebogene Rohre pro Seite), vier Reifen (Revolve um z, 0,125 m Durchmesser, 0,04 m breit, bei z +-0,22 wie die Draufsicht zeigt) mit Nabenscheiben und zwei Rollenplatten vorn. Der Griff ist ein U aus zwei Pfosten, zwei Armen und einem Gummigriff quer in z bei x -0,52 wie die Draufsicht zeigt."),
    ("The gate is a flap along the sloping rear wall hinged at the top rear rim, as wide as the basket interior (z +-0.24); it swings with the gate bone.",
     "Die Klappe liegt an der geneigten Rückwand und ist am oberen hinteren Rand angeschlagen, so breit wie der Korbinnenraum (z +-0,24); sie schwingt mit dem Klappenknochen."),
])
