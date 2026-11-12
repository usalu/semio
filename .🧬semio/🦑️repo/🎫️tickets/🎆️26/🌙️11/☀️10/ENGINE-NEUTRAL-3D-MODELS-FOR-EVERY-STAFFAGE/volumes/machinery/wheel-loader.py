import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("wheel-loader")
PS = dict(material="paint-steel", role="body")
PSD = dict(material="paint-steel", role="detail")
CI = dict(material="cast-iron", role="body")
CID = dict(material="cast-iron", role="detail")
WS = dict(material="wear-steel", role="body")
WSD = dict(material="wear-steel", role="detail")
CH = dict(material="chrome", role="detail")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]
loc = lambda bone, pts: [tuple(V.W(bone, [p[0], p[1], 0])[:2]) for p in pts]
mir = lambda z: dict(rep=dict(count=2, step=[0, 0, -2 * z]))

hood = [(-2.6, 0.62), (-2.6, 1.9), (-3, 2.4), (-1.1, 2.4), (-0.9, 2.2), (-0.9, 0.62)]
V.ext("hood", "rear-frame", "side", hood, -1.0, 1.0, tone=0.45, region="engine-bay", **PS)
V.box("hood-louvres", "rear-frame", [-1.75, 2.1, 1.0075], [1.1, 0.03, 0.015], tone=0.3, region="engine-bay", maxDenominator=100, rep=dict(count=2, step=[0, 0, -2.015]), **PSD)
V.box("hood-louvres-2", "rear-frame", [-1.75, 1.85, 1.0075], [1.1, 0.03, 0.015], tone=0.3, region="engine-bay", maxDenominator=100, rep=dict(count=2, step=[0, 0, -2.015]), **PSD)
cw = [(-3.5, 0.62), (-3.5, 2.05), (-3.35, 2.3), (-3, 2.4), (-2.6, 1.9), (-2.6, 0.62)]
V.ext("counterweight", "rear-frame", "side", cw, -1.1, 1.1, tone=0.3, region="counterweight", **CI)
V.box("grille", "rear-frame", [-3.495, 1.4, 0], [0.05, 0.03, 1.2], tone=0.2, region="counterweight", maxDenominator=100, rep=dict(count=6, step=[0, 0.16, 0]), **CID)
V.cyl("exhaust-stack", "rear-frame", [-1.23, 2.3, -0.9], [-1.23, 3.15, -0.9], 0.09, tone=0.2, region="exhaust-stack", material="exhaust-steel", role="body")
for x in (-2.9, -1.4):
    V.cap(f"handrail-post-{int(abs(x) * 10)}", "rear-frame", [x, 2.4, 0.95], [x, 2.62, 0.95], 0.02, tone=0.3, region="engine-bay", maxDenominator=100, rep=dict(count=2, step=[0, 0, -1.9]), **PSD)
V.cap("handrail-side", "rear-frame", [-2.9, 2.62, 0.95], [-1.4, 2.62, 0.95], 0.02, tone=0.3, region="engine-bay", maxDenominator=100, rep=dict(count=2, step=[0, 0, -1.9]), **PSD)
V.cap("handrail-end", "rear-frame", [-2.9, 2.62, -0.95], [-2.9, 2.62, 0.95], 0.02, tone=0.3, region="engine-bay", maxDenominator=100, rep=dict(count=2, step=[1.5, 0, 0]), **PSD)
lower = [(-0.9, 0.62), (2.35, 0.62), (2.35, 1.2), (1.5, 1.45), (-0.9, 1.45)]
V.ext("front-frame", "front-frame", "side", lower, -0.9, 0.9, tone=0.45, region="front-frame", **PS)
V.box("tower", "front-frame", [1.25, 1.825, 0.75], [0.5, 0.75, 0.26], tone=0.45, region="front-frame", rep=dict(count=2, step=[0, 0, -1.5]), **PS)
V.box("access-step", "front-frame", [-0.7, 1.0, -0.9], [0.4, 0.2, 0.2], tone=0.3, region="front-frame", maxDenominator=100, **PSD)
V.box("head-lamp", "front-frame", [2.325, 1.05, 0.6], [0.05, 0.2, 0.2], tone=0.9, region="front-frame", maxDenominator=100, rep=dict(count=2, step=[0, 0, -1.2]), material="glass", role="detail")
cab = [(-0.9, 1.45), (1.0, 1.45), (0.98, 2.2), (0.82, 3.32), (0.7, 3.4), (-0.8, 3.4), (-0.9, 3.28)]
V.ext("cab", "front-frame", "side", cab, -0.8, 0.8, tone=0.55, region="cab", **PS)
V.ext("cab-glass-front-pane", "front-frame", "side", [(0.78, 2.1), (0.66, 3.2), (0.2, 3.2), (0.2, 2.1)], 0.8, 0.81, tone=0.85, region="cab-glazing", material="glass", role="glazing", rep=dict(count=2, step=[0, 0, -1.61]))
for nm, z0, z1 in [("l", 0.8, 0.81), ("r", -0.81, -0.8)]:
    V.ext(f"cab-glass-side-{nm}", "front-frame", "side", [(0, 2.1), (0, 3.2), (-0.6, 3.2), (-0.6, 2.1)], z0, z1, tone=0.85, region="cab-glazing", material="glass", role="glazing")
V.box("cab-windscreen", "front-frame", [0.925, 2.7, 0], [0.02, 1.11, 1.3], rz=8.1, tone=0.85, region="cab-glazing", material="glass", role="glazing")
V.ext("mirror", "front-frame", "side", [(0.85, 3.25), (1.1, 3.25), (1.1, 3.0), (0.95, 3.0)], -0.95, -0.8, tone=0.2, region="cab", maxDenominator=50, **PSD)
V.box("door-handle", "front-frame", [0, 2.025, -0.815], [0.4, 0.05, 0.03], tone=0.2, region="cab", maxDenominator=50, **PSD)
arm = loc("lift-arm", [(-0.25, 0.32), (1, 0.48), (2, 0.38), (2.85, 0.25), (2.85, -0.25), (2, -0.22), (1, -0.28), (-0.25, -0.32)])
V.ext("lift-arm", "lift-arm", "side", arm, 0.9, 1.2, tone=0.35, region="lift-arm", rep=dict(count=2, step=[0, 0, -2.1]), **PS)
V.cyl("torque-tube", "lift-arm", V.W("lift-arm", [2.2, 0, -1.05]), V.W("lift-arm", [2.2, 0, 1.05]), 0.1, tone=0.35, region="lift-arm", **PSD)
V.cyl("lift-pivot", "front-frame", [1.1, 2.0, -1.2], [1.1, 2.0, 1.2], 0.07, tone=0.2, region="lift-arm", maxDenominator=200, **PSD)
V.cyl("lift-cyl-barrel", "lift-cyl", V.W("lift-cyl", [0, 0, 0.6]), V.W("lift-cyl", [1.1, 0, 0.6]), 0.12, tone=0.5, region="hydraulics", rep=dict(count=2, step=[0, 0, -1.2]), **PSD)
V.cyl("lift-rod", "lift-rod", V.W("lift-rod", [-0.9, 0, 0.6]), V.W("lift-rod", [0.4564, 0, 0.6]), 0.05, tone=0.8, region="hydraulics", maxDenominator=200, rep=dict(count=2, step=[0, 0, -1.2]), **CH)
V.cyl("tilt-cyl-barrel", "tilt-cyl", V.W("tilt-cyl", [0, 0, 0]), V.W("tilt-cyl", [0.9, 0, 0]), 0.1, tone=0.5, region="hydraulics", **PSD)
V.cyl("tilt-rod", "tilt-rod", V.W("tilt-rod", [-0.5, 0, 0]), V.W("tilt-rod", [0.2679, 0, 0]), 0.045, tone=0.8, region="hydraulics", **CH)
bk = [(-0.45, -0.6), (1.5, -0.8), (1.62, -0.62), (1.05, 0.05), (0.15, 0.6), (-0.35, 0.55), (-0.55, 0)]
bw = loc("bucket", bk)
V.ext("bucket-cheek", "bucket", "side", bw, 1.4, 1.475, tone=0.25, region="bucket", rep=dict(count=2, step=[0, 0, -2.875]), **WS)
cx = sum(p[0] for p in bw) / len(bw); cy = sum(p[1] for p in bw) / len(bw)
def sheet(i, j, t=0.05):
    a, b = bw[i], bw[j]
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = math.hypot(dx, dy)
    nx, ny = -dy / L, dx / L
    if (cx - a[0]) * nx + (cy - a[1]) * ny < 0:
        nx, ny = -nx, -ny
    return [a, b, (b[0] + t * nx, b[1] + t * ny), (a[0] + t * nx, a[1] + t * ny)]
for nm, (i, j) in {"floor": (0, 1), "lip": (1, 2), "back-top": (4, 5), "back-mid": (5, 6), "back-low": (6, 0)}.items():
    V.ext(f"bucket-{nm}", "bucket", "side", sheet(i, j), -1.4, 1.4, tone=0.25, region="bucket", **WS)
V.ext("bucket-edge", "bucket", "side", loc("bucket", [(1.3, -0.77), (1.62, -0.62), (1.5, -0.8)]), -1.475, 1.475, tone=0.1, region="bucket", maxDenominator=100, **WSD)
V.ext("bucket-rib-outer", "bucket", "side", bw, 0.875, 0.895, tone=0.25, region="bucket", maxDenominator=100, rep=dict(count=2, step=[0, 0, -1.77]), **WSD)
V.ext("bucket-rib-inner", "bucket", "side", bw, 0.285, 0.305, tone=0.25, region="bucket", maxDenominator=200, rep=dict(count=2, step=[0, 0, -0.59]), **WSD)
V.box("bucket-ear", "bucket", V.W("bucket", [-0.2, 0, 1.05]), [0.7, 0.3, 0.3], rz=V.angle("bucket"), tone=0.35, region="bucket", rep=dict(count=2, step=[0, 0, -2.1]), **WS)
V.cyl("bucket-pin", "bucket", V.W("bucket", [0, 0, -1.2]), V.W("bucket", [0, 0, 1.2]), 0.07, tone=0.2, region="bucket", maxDenominator=200, **WSD)
V.box("tilt-lug", "bucket", [3.12, 1.545, 0], [0.14, 0.14, 0.2], tone=0.3, region="bucket", maxDenominator=200, **WSD)
for bone, cx_, frame in [("wheel-r", -1.7, "rear-frame"), ("wheel-f", 1.7, "front-frame")]:
    tag = bone[-1]
    cyw = 0.875
    prof = [[a, r] for a, r in [(-0.3, 0), (-0.3, 0.74), (-0.26, 0.82), (0.26, 0.82), (0.3, 0.74), (0.3, 0)]]
    for side, sg in [("l", 1), ("r", -1)]:
        V.rev(f"tyre-{tag}-{side}", bone, [cx_, cyw, sg * 1.1], [0, 0, 1], prof, tone=0.12, region="tyres", material="rubber", role="body")
        V.box(f"lugs-{tag}-{side}", bone, [cx_ + 0.8475, cyw, sg * 1.1], [0.055, 0.17, 0.5], tone=0.05, region="tyres", maxDenominator=50, material="rubber", role="detail",
              rep=dict(count=24, angle=2 * math.pi / 24, axis=[0, 0, 1], origin=[cx_, cyw, 0]))
        zr = sg * 1.4
        V.cyl(f"rim-{tag}-{side}", bone, [cx_, cyw, zr], [cx_, cyw, zr + sg * 0.005], 0.5, tone=0.6, region="tyres", maxDenominator=200, **CID)
        V.cyl(f"hub-{tag}-{side}", bone, [cx_, cyw, zr + sg * 0.005], [cx_, cyw, zr + sg * 0.02], 0.14, tone=0.3, region="tyres", maxDenominator=100, **CID)
        V.cyl(f"bolt-{tag}-{side}", bone, [cx_ + 0.3, cyw, zr + sg * 0.02], [cx_ + 0.3, cyw, zr + sg * 0.035], 0.045, tone=0.0, region="tyres", maxDenominator=100, material="cast-iron", role="detail",
              rep=dict(count=6, angle=2 * math.pi / 6, axis=[0, 0, 1], origin=[cx_, cyw, 0]))
    V.cyl(f"axle-{tag}", frame, [cx_, cyw, -1.1], [cx_, cyw, 1.1], 0.12, tone=0.3, material="cast-iron", role="detail", region="tyres")
V.decide("The bucket is a real scoop: two 75 mm cheek plates with the drawn side outline, thin floor, lip and back sheets (the outline's mouth edge is left open), cutting edge, four ribs and two lugs with the pivot pin; the 2D draws it as a filled silhouette, which the cheeks reproduce from the side while the hollow shows from the front and above.",
         "Die Schaufel ist ein echter Loeffel: zwei 75 mm dicke Wangenplatten mit dem gezeichneten Seitenumriss, duenne Boden-, Lippen- und Rueckbleche (die Muendungskante des Umrisses bleibt offen), Schneide, vier Rippen und zwei Augen mit dem Drehbolzen; die 2D-Zeichnung zeigt eine gefuellte Silhouette, die die Wangen von der Seite nachbilden, waehrend die Hoehlung von vorn und oben sichtbar ist.")
V.decide("Lift arms are two extruded plates (outline from the elevation, z 0.9 to 1.2) joined by a torque tube; lift cylinders at z +-0.6 and the centre tilt cylinder are barrel and rod cylinders on their own bones, the tilt rod ends in a lug on the bucket; both rods extend back inside their barrels (0.9 m and 0.5 m) so the sliding rods stay in the barrels over the whole stroke; articulation is the rig's (the frames are fixed bones). Arms and cylinders at z 0.9 to 1.2 pass through the front tyres (z 0.8 to 1.4) as the 2D views have them.",
         "Die Hubarme sind zwei extrudierte Platten (Umriss aus der Ansicht, z 0,9 bis 1,2), verbunden durch ein Torsionsrohr; Hubzylinder bei z +-0,6 und der mittlere Kippzylinder sind Rohr- und Stangenzylinder auf eigenen Knochen, die Kippstange endet in einem Auge an der Schaufel; beide Stangen reichen ins Rohr zurueck (0,9 m und 0,5 m), damit die gleitenden Stangen ueber den ganzen Hub im Rohr bleiben; die Knickung ist die des Rigs (die Rahmen sind feste Knochen). Arme und Zylinder bei z 0,9 bis 1,2 laufen durch die Vorderreifen (z 0,8 bis 1,4), wie es die 2D-Ansichten zeigen.")
V.decide("Tyres are revolved cores (r 0.82) with 24 tread lugs (rotational repeat) that make rolling readable, rim, hub and six bolts on the wheel bones; the cab is the drawn polygon extruded 1.6 m wide with glass plates on both sides and a sloped windscreen; hood and counterweight are extruded outlines.",
         "Reifen sind gedrehte Kerne (r 0,82) mit 24 Stollen (Rotationswiederholung), die das Rollen lesbar machen, Felge, Nabe und sechs Bolzen auf den Radknochen; die Kabine ist das gezeichnete Polygon 1,6 m breit extrudiert mit Glasplatten auf beiden Seiten und einer geneigten Frontscheibe; Haube und Gegengewicht sind extrudierte Umrisse.")
V.write(os.path.join(here, "wheel-loader.json"))
