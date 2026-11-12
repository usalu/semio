import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("excavator")
PS = dict(material="paint-steel", role="body")
PSD = dict(material="paint-steel", role="detail")
TS = dict(material="track-steel", role="body")
TSD = dict(material="track-steel", role="detail")
WS = dict(material="wear-steel", role="body")
WSD = dict(material="wear-steel", role="detail")
CI = dict(material="cast-iron", role="body")
CH = dict(material="chrome", role="detail")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]
loc = lambda bone, pts: [tuple(V.W(bone, [p[0], p[1], 0])[:2]) for p in pts]

def bez(p0, p1, p2, p3, n=10):
    return [tuple((1 - t) ** 3 * p0[k] + 3 * (1 - t) ** 2 * t * p1[k] + 3 * (1 - t) * t ** 2 * p2[k] + t ** 3 * p3[k] for k in (0, 1)) for t in [i / n for i in range(1, n + 1)]]

def stadium(cx, cy, hl, r, n=10):
    pts = []
    for i in range(n + 1):
        a = -math.pi / 2 + math.pi * i / n
        pts.append((cx + hl + r * math.cos(a), cy + r * math.sin(a)))
    for i in range(n + 1):
        a = math.pi / 2 + math.pi * i / n
        pts.append((cx - hl + r * math.cos(a), cy + r * math.sin(a)))
    return pts

outer = stadium(0, 0.43, 1.65, 0.43)
inner = stadium(0, 0.43, 1.65, 0.36)
V.ext("track-belt", "undercarriage", "side", outer, 0.895, 1.495, holes=[inner], tone=0.2, region="tracks", rep=dict(count=2, step=[0, 0, -2.39]), **TS)
V.box("grouser-top", "undercarriage", [-1.4, 0.875, 1.195], [0.05, 0.03, 0.6], tone=0.4, maxDenominator=50, rep=dict(count=16, step=[0.2, 0, 0]), **TSD)
V.box("grouser-top-r", "undercarriage", [-1.4, 0.875, -1.195], [0.05, 0.03, 0.6], tone=0.4, maxDenominator=50, rep=dict(count=16, step=[0.2, 0, 0]), **TSD)
V.box("grouser-bottom", "undercarriage", [-1.5, -0.015, 1.195], [0.05, 0.03, 0.6], tone=0.4, maxDenominator=50, rep=dict(count=16, step=[0.2, 0, 0]), **TSD)
V.box("grouser-bottom-r", "undercarriage", [-1.5, -0.015, -1.195], [0.05, 0.03, 0.6], tone=0.4, maxDenominator=50, rep=dict(count=16, step=[0.2, 0, 0]), **TSD)
V.box("track-frame", "undercarriage", [0, 0.475, 1.195], [3.4, 0.45, 0.4], tone=0.4, maxDenominator=200, rep=dict(count=2, step=[0, 0, -2.39]), **dict(PS, role="detail"))
V.cyl("sprocket", "undercarriage", [-1.65, 0.43, 0.98], [-1.65, 0.43, 1.41], 0.33, tone=0.5, material="track-steel", role="detail", rep=dict(count=2, step=[0, 0, -2.39]))
V.cyl("idler", "undercarriage", [1.65, 0.43, 0.98], [1.65, 0.43, 1.41], 0.33, tone=0.5, material="track-steel", role="detail", rep=dict(count=2, step=[0, 0, -2.39]))
V.cyl("sprocket-hub", "undercarriage", [-1.65, 0.43, 0.95], [-1.65, 0.43, 1.44], 0.1, tone=0.25, maxDenominator=100, rep=dict(count=2, step=[0, 0, -2.39]), **TSD)
V.cyl("idler-hub", "undercarriage", [1.65, 0.43, 0.95], [1.65, 0.43, 1.44], 0.1, tone=0.25, maxDenominator=100, rep=dict(count=2, step=[0, 0, -2.39]), **TSD)
V.cyl("roller", "undercarriage", [-1.0, 0.2, 1.045], [-1.0, 0.2, 1.345], 0.11, tone=0.55, maxDenominator=100, **TSD, rep=dict(count=5, step=[0.5, 0, 0]))
V.cyl("roller-r", "undercarriage", [-1.0, 0.2, -1.345], [-1.0, 0.2, -1.045], 0.11, tone=0.55, maxDenominator=100, **TSD, rep=dict(count=5, step=[0.5, 0, 0]))
V.cyl("carrier-roller", "undercarriage", [0, 0.74, 1.095], [0, 0.74, 1.295], 0.07, tone=0.55, maxDenominator=50, rep=dict(count=2, step=[0, 0, -2.39]), **TSD)
V.box("carbody", "undercarriage", [0, 0.58, 0], [2.0, 0.56, 1.79], tone=0.4, region="tracks", **dict(PS))
V.cyl("slew-ring", "undercarriage", [0, 0.86, 0], [0, 1.0, 0], 0.8, tone=0.3, maxDenominator=100, **dict(PS, role="detail"))
house = [(-1.75, 1.0), (1.6, 1.0), (1.6, 1.3), (0.12, 1.3), (0.12, 2.25), (-1.4, 2.25), (-1.65, 2.2), (-1.75, 1.95)]
V.ext("house", "upper", "side", house, -1.25, 1.25, tone=0.62, region="house", **PS)
cab = [(0.12, 1.3), (1.6, 1.3), (1.58, 1.95), (1.55, 2.4), (1.49, 2.72), (1.42, 2.88), (1.36, 2.96), (1.28, 2.97), (0.4, 2.97), (0.3, 2.95), (0.16, 2.85), (0.12, 2.7)]
V.ext("cab", "upper", "side", cab, 0.1, 1.25, tone=0.62, region="house", **PS)
V.ext("cab-glass-side-window", "upper", "side", [(0.3, 1.6), (0.95, 1.6), (0.95, 2.9), (0.45, 2.9), (0.3, 2.85)], 1.25, 1.26, tone=0.85, region="cab-glazing", material="glass", role="glazing")
V.ext("cab-glass-rear-window", "upper", "side", [(1.02, 1.58), (1.46, 1.58), (1.44, 2.1), (1.4, 2.6), (1.34, 2.82), (1.2, 2.87), (1.02, 2.87)], 1.25, 1.26, tone=0.85, region="cab-glazing", material="glass", role="glazing", maxDenominator=100)
V.box("cab-windscreen", "upper", [1.5, 2.2, 0.675], [0.02, 1.29, 0.95], rz=-7, tone=0.85, region="cab-glazing", material="glass", role="glazing")
V.ext("cab-sill", "upper", "side", [(0.12, 1.55), (1.58, 1.55), (1.58, 1.6), (0.12, 1.6)], 1.25, 1.265, tone=0.3, maxDenominator=100, **PSD)
V.box("hood-grille-top", "upper", [-1.05, 2.26, 0], [1.0, 0.02, 1.8], tone=0.2, region="engine-compartment", maxDenominator=200, material="engine-metal", role="detail")
V.box("hood-grille-side", "upper", [-1.05, 1.725, -1.255], [1.0, 0.65, 0.01], tone=0.15, region="engine-compartment", maxDenominator=200, material="engine-metal", role="detail")
V.cap("hood-rail-bar", "upper", [-1.65, 2.5, -1.15], [-0.3, 2.5, -1.15], 0.025, tone=0.3, maxDenominator=100, **PSD)
V.cap("hood-rail-post", "upper", [-1.65, 2.25, -1.15], [-1.65, 2.5, -1.15], 0.025, tone=0.3, maxDenominator=100, rep=dict(count=2, step=[1.35, 0, 0]), **PSD)
V.cyl("stack", "upper", [-1.2, 2.25, -0.9], [-1.2, 2.8, -0.9], 0.055, tone=0.25, region="exhaust-stack", material="exhaust-steel", role="detail")
V.cyl("stack-cap", "upper", [-1.2, 2.78, -0.9], [-1.2, 2.82, -0.9], 0.08, tone=0.2, maxDenominator=100, material="exhaust-steel", role="detail")
V.ext("boom-foot", "upper", "side", [(0.78, 1.22), (1.3, 1.22), (1.3, 1.55), (1.15, 1.78), (0.9, 1.78), (0.78, 1.55)], -0.7, -0.55, tone=0.4, region="house", maxDenominator=200, **PS)
V.ext("boom-foot-inner", "upper", "side", [(0.78, 1.22), (1.3, 1.22), (1.3, 1.55), (1.15, 1.78), (0.9, 1.78), (0.78, 1.55)], 0.05, 0.1, tone=0.4, region="house", maxDenominator=200, **PS)
cwp = [(0.65, 0), (-0.5, 0), (-0.58, 0.05), (-0.65, 0.25), (-0.63, 0.85), (-0.57, 1.05), (-0.4, 1.17), (-0.15, 1.2), (0.65, 1.2)]
V.ext("counterweight-body", "counterweight", "side", loc("counterweight", cwp), -1.2, 1.2, tone=0.35, region="counterweight", **CI)
boom = [(0, -0.5)] + bez((0, -0.5), (0.6, -0.72), (1.6, -0.6), (2.6, -0.28)) + bez((2.6, -0.28), (3.6, 0.04), (4.8, -0.14), (5.7, -0.3)) + [(5.7, 0.3)] + bez((5.7, 0.3), (4.8, 0.34), (3.9, 0.75), (2.9, 0.98)) + bez((2.9, 0.98), (2.1, 1.1), (1.1, 0.95), (0, 0.5))
V.ext("boom-body", "boom", "side", loc("boom", boom), -0.55, 0.05, tone=0.58, region="attachment", **PS)
V.cyl("boom-pin", "boom", [1.0, 1.65, -0.7], [1.0, 1.65, 0.1], 0.09, tone=0.12, maxDenominator=200, **TSD)
V.cyl("boom-tip-boss", "boom", V.W("boom", [5.7, 0, -0.55]), V.W("boom", [5.7, 0, 0.05]), 0.26, tone=0.35, maxDenominator=200, **PSD)
stick = [(0, -0.38)] + bez((0, -0.38), (0.9, -0.44), (2.0, -0.32), (2.9, -0.2)) + [(2.9, 0.2)] + bez((2.9, 0.2), (2.0, 0.3), (0.9, 0.46), (0, 0.42))
V.ext("stick-body", "stick", "side", loc("stick", stick), -0.49, -0.01, tone=0.58, region="attachment", **PS)
V.cyl("stick-root-boss", "stick", V.W("stick", [0, 0, -0.49]), V.W("stick", [0, 0, -0.01]), 0.28, tone=0.35, maxDenominator=200, **PSD)
V.cyl("stick-tip-boss", "stick", V.W("stick", [2.9, 0, -0.49]), V.W("stick", [2.9, 0, -0.01]), 0.22, tone=0.35, maxDenominator=200, **PSD)
V.cyl("boom-cyl-barrel", "boom-cyl", V.W("boom-cyl", [0, 0, 0.25]), V.W("boom-cyl", [1.45, 0, 0.25]), 0.1, tone=0.35, region="hydraulics", rep=dict(count=2, step=[0, 0, -1.0]), **PSD)
V.cyl("boom-cyl-rod", "boom-rod", V.W("boom-rod", [-0.4, 0, 0.25]), V.W("boom-rod", [1.65, 0, 0.25]), 0.045, tone=0.85, region="hydraulics", maxDenominator=100, rep=dict(count=2, step=[0, 0, -1.0]), **CH)
V.cyl("stick-cyl-barrel", "stick-cyl", V.W("stick-cyl", [0, 0, -0.25]), V.W("stick-cyl", [1.6, 0, -0.25]), 0.095, tone=0.35, region="hydraulics", **PSD)
V.cyl("stick-cyl-rod", "stick-rod", V.W("stick-rod", [-0.5, 0, -0.25]), V.W("stick-rod", [2.35, 0, -0.25]), 0.05, tone=0.85, region="hydraulics", **CH)
V.cyl("bucket-cyl-barrel", "bucket-cyl", V.W("bucket-cyl", [0, 0, -0.25]), V.W("bucket-cyl", [0.85, 0, -0.25]), 0.075, tone=0.35, region="hydraulics", **PSD)
V.cyl("bucket-cyl-rod", "bucket-rod", V.W("bucket-rod", [-0.4, 0, -0.25]), V.W("bucket-rod", [0.95, 0, -0.25]), 0.04, tone=0.85, region="hydraulics", maxDenominator=100, **CH)
for nm, pos, bone, z0, z1, r in [("boom-cyl-foot", (0.45, 1.1), "upper", -0.85, 0.35, 0.08), ("boom-rod-eye", (2.668, 2.462), "boom", -0.85, 0.35, 0.07), ("stick-cyl-foot", (2.687, 4.306), "boom", -0.4, -0.1, 0.07), ("stick-rod-eye", (6.236, 4.868), "stick", -0.4, -0.1, 0.07), ("bucket-cyl-foot", (6.446, 4.588), "stick", -0.4, -0.1, 0.06)]:
    V.cyl(nm, bone, [pos[0], pos[1], z0], [pos[0], pos[1], z1], r, tone=0.12, maxDenominator=50, **TSD)
bk = [(1.35, 0), (1.3, 0.05)] + bez((1.3, 0.05), (1.05, 0.62), (0.35, 0.78), (-0.08, 0.34), 14) + [(-0.12, 0.06), (0.02, -0.12)] + bez((0.02, -0.12), (0.4, -0.1), (0.95, -0.1), (1.35, 0), 6)[:-1]
bkw = loc("bucket", bk)
V.ext("bucket-cheek", "bucket", "side", bkw, -0.75, -0.7, tone=0.4, region="attachment", rep=dict(count=2, step=[0, 0, 0.95]), **WS)
shell = [(1.3, 0.05)] + bez((1.3, 0.05), (1.05, 0.62), (0.35, 0.78), (-0.08, 0.34), 14) + [(-0.12, 0.06), (0.02, -0.12)]
cx = sum(p[0] for p in bk) / len(bk); cy = sum(p[1] for p in bk) / len(bk)
inner, outerp = [], []
for i, p in enumerate(shell):
    a = shell[max(i - 1, 0)]; b = shell[min(i + 1, len(shell) - 1)]
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = math.hypot(dx, dy) or 1
    nx, ny = -dy / L, dx / L
    if (cx - p[0]) * nx + (cy - p[1]) * ny < 0:
        nx, ny = -nx, -ny
    outerp.append(p); inner.append((p[0] + 0.05 * nx, p[1] + 0.05 * ny))
V.ext("bucket-shell", "bucket", "side", loc("bucket", outerp + inner[::-1]), -0.7, 0.2, tone=0.4, region="attachment", **WS)
V.ext("bucket-floor-lip", "bucket", "side", loc("bucket", [(0.02, -0.12), (1.35, 0), (1.35, -0.04), (0.02, -0.17)]), -0.75, 0.25, tone=0.3, region="attachment", maxDenominator=200, **WSD)
V.ext("bucket-tooth", "bucket", "side", loc("bucket", [(1.3, 0.07), (1.5, -0.03), (1.28, -0.09)]), -0.72, -0.6, tone=0.2, region="attachment", maxDenominator=100, rep=dict(count=5, step=[0, 0, 0.2]), **WSD)
V.ext("bucket-rib", "bucket", "side", loc("bucket", [(0.28, 0.1), (0.42, 0.1), (0.45, 0.57), (0.3, 0.58)]), -0.28, -0.22, tone=0.3, maxDenominator=100, **WSD)
V.cyl("bucket-pin", "bucket", V.W("bucket", [0, 0, -0.78]), V.W("bucket", [0, 0, 0.28]), 0.1, tone=0.12, maxDenominator=100, **TSD)
V.cyl("bucket-ear", "bucket", V.W("bucket", [0.35, 0.18, -0.4]), V.W("bucket", [0.35, 0.18, -0.1]), 0.06, tone=0.12, maxDenominator=100, **TSD)
V.decide("Every moving part is its own solid on the bone that drives it: tracks, sprocket, idler and rollers on the undercarriage; house, cab and counterweight on the slewing bones; boom, stick and bucket as extruded side outlines (Bezier outlines flattened) with pivot bosses and pins on their own bones; three cylinder pairs as barrel and rod cylinders, the barrel at the lateral position of the front view (boom cylinders at z +0.25 and -0.75 around the boom, stick and bucket cylinders on the boom centre line z -0.25).",
         "Jedes bewegliche Teil ist ein eigener Koerper auf dem Knochen, der es antreibt: Ketten, Kettenrad, Leitrad und Rollen auf dem Unterwagen; Oberwagen, Kabine und Gegengewicht auf den Drehknochen; Ausleger, Stiel und Loeffel als extrudierte Seitenumrisse (Bezier-Umrisse abgeflacht) mit Gelenkbossen und Bolzen auf ihren eigenen Knochen; drei Zylinderpaare als Rohr- und Stangenzylinder, das Rohr an der seitlichen Lage der Vorderansicht (Auslegerzylinder bei z +0,25 und -0,75 um den Ausleger, Stiel- und Loeffelzylinder auf der Auslegermitte z -0,25).")
V.decide("Cylinder rods extend back inside their barrels (0.4 to 0.5 m) so they stay in the barrel over the stroke; the rig's rest pose is not a closed linkage (rod tips do not meet their eyes at rest), the authored poses close it. dimensions describe the stowed default pose; at rest (boom angle 40) the box is larger.",
         "Zylinderstangen reichen ins Rohr zurueck (0,4 bis 0,5 m), damit sie ueber den Hub im Rohr bleiben; die Ruhelage des Rigs ist kein geschlossenes Gestaenge (Stangenspitzen treffen ihre Augen in Ruhe nicht), die autorisierten Posen schliessen es. dimensions beschreibt die verstaute Standardpose; in Ruhe (Auslegerwinkel 40) ist die Box groesser.")
V.decide("The cab (left half, z 0.1 to 1.25) is taller than the hood (right half, to 2.25 m plus rail); the front view draws a 1.97 m high rectangle over the whole width, a union envelope that the 3D does not fill on the right. The bucket is a scoop (two cheeks, a 50 mm back shell, floor lip, rib, five teeth) with the mouth edge of the 2D outline open; the engine block and the seat inside the closed house and cab are not modelled.",
         "Die Kabine (linke Haelfte, z 0,1 bis 1,25) ist hoeher als die Haube (rechte Haelfte, bis 2,25 m plus Gelaender); die Vorderansicht zeichnet ein 1,97 m hohes Rechteck ueber die ganze Breite, eine Huellkurve, die das 3D rechts nicht fuellt. Der Loeffel ist eine Schaufel (zwei Wangen, 50 mm Rueckenschale, Bodenlippe, Rippe, fuenf Zaehne) mit offener Muendungskante des 2D-Umrisses; Motorblock und Sitz im geschlossenen Oberwagen und in der Kabine sind nicht modelliert.")
V.write(os.path.join(here, "excavator.json"))
