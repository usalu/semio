import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-03-lib.py"))
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


def geometry():
    sc = V.scale
    K = {50: 1.0, 100: 1.1, 200: 1.25, 500: 1.6}[sc]
    FT = max(FEAT[sc], 0.0075)
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
    V.ext("track-belt", "undercarriage", "side", outer, 0.895, 1.495, holes=[inner] if sc < 200 else None, tone=0.2, region="tracks", rep=dict(count=2, step=[0, 0, -2.39]), **TS)
    if sc < 200:
        V.ext("track-web", "undercarriage", "side", inner, 1.145, 1.245, tone=0.2, region="tracks", rep=dict(count=2, step=[0, 0, -2.39]), **TSD)
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
    HY0 = 1.0 if sc < 100 else 0.84
    house = [(-1.75, HY0), (1.6, HY0), (1.6, 1.3), (0.12, 1.3), (0.12, 2.25), (-1.4, 2.25), (-1.65, 2.2), (-1.75, 1.95)]
    V.ext("house", "upper", "side", house, -1.25, 1.25, tone=0.62, region="house", **PS)
    cab = [(0.12, 1.3), (1.6, 1.3), (1.58, 1.95), (1.55, 2.4), (1.49, 2.72), (1.42, 2.88), (1.36, 2.96), (1.28, 2.97), (0.4, 2.97), (0.3, 2.95), (0.16, 2.85), (0.12, 2.7)]
    V.ext("cab", "upper", "side", cab, 0.1, 1.25, tone=0.62, region="house", **PS)
    V.ext("cab-glass-side-window", "upper", "side", [(0.3, 1.6), (0.95, 1.6), (0.95, 2.9), (0.45, 2.9), (0.3, 2.85)], 1.25, 1.26, tone=0.85, region="cab-glazing", material="glass", role="glazing")
    V.ext("cab-glass-rear-window", "upper", "side", [(1.02, 1.58), (1.46, 1.58), (1.44, 2.1), (1.4, 2.6), (1.34, 2.82), (1.2, 2.87), (1.02, 2.87)], 1.25, 1.26, tone=0.85, region="cab-glazing", material="glass", role="glazing", maxDenominator=100)
    V.box("cab-windscreen", "upper", [1.5, 2.2, 0.675], [0.02, 1.29, 0.95], rz=-7, tone=0.85, region="cab-glazing", material="glass", role="glazing")
    V.ext("cab-sill", "upper", "side", [(0.12, 1.55), (1.58, 1.55), (1.58, 1.6), (0.12, 1.6)], 1.25, 1.25 + max(0.015, FT * 1.03), tone=0.3, maxDenominator=100, **PSD)
    V.box("hood-grille-top", "upper", [-1.05, 2.25 + max(0.02, FT * 1.02) / 2, 0], [1.0, max(0.02, FT * 1.02), 1.8], tone=0.2, region="engine-compartment", maxDenominator=200, material="engine-metal", role="detail")
    V.box("hood-grille-side", "upper", [-1.05, 1.725, -1.25 - max(0.01, FT * 1.02) / 2], [1.0, 0.65, max(0.01, FT * 1.02)], tone=0.15, region="engine-compartment", maxDenominator=200, material="engine-metal", role="detail")
    V.ext("hood-rail-panel", "upper", "side", rect(-1.65, -0.3, 2.25, 2.5), -1.165, -1.135, tone=0.3, maxDenominator=100, **PSD)
    V.cap("hood-rail-bar", "upper", [-1.65, 2.5, -1.15], [-0.3, 2.5, -1.15], 0.025, tone=0.3, maxDenominator=100, **PSD)
    V.cap("hood-rail-post", "upper", [-1.65, 2.25, -1.15], [-1.65, 2.5, -1.15], 0.025, tone=0.3, maxDenominator=100, rep=dict(count=2, step=[1.35, 0, 0]), **PSD)
    V.box("engine-block", "engine", [-1.0, 1.525, 0], [1.1, 0.65, 2.4], tone=0.2, region="engine-compartment", maxDenominator=50, material="engine-metal", role="detail")
    V.cyl("stack", "upper", [-1.2, 2.25, -0.9], [-1.2, 2.8, -0.9], 0.055, tone=0.25, region="exhaust-stack", material="exhaust-steel", role="detail")
    V.cyl("stack-cap", "upper", [-1.2, 2.78, -0.9], [-1.2, 2.82, -0.9], 0.08, tone=0.2, maxDenominator=100, material="exhaust-steel", role="detail")
    V.ext("boom-foot", "upper", "side", [(0.78, 1.22), (1.3, 1.22), (1.3, 1.55), (1.15, 1.78), (0.9, 1.78), (0.78, 1.55)], -0.7, -0.55, tone=0.4, region="house", maxDenominator=200, **PS)
    V.ext("boom-foot-inner", "upper", "side", [(0.78, 1.22), (1.3, 1.22), (1.3, 1.55), (1.15, 1.78), (0.9, 1.78), (0.78, 1.55)], 0.05, 0.1, tone=0.4, region="house", maxDenominator=200, **PS)
    CY0 = 0 if sc < 100 else -0.16
    cwp = [(0.65, CY0), (-0.5, CY0), (-0.58, 0.05), (-0.65, 0.25), (-0.63, 0.85), (-0.57, 1.05), (-0.4, 1.17), (-0.15, 1.2), (0.65, 1.2)]
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
    V.cyl("boom-cyl-rod", "boom-rod", V.W("boom-rod", [-0.4, 0, 0.25]), V.W("boom-rod", [1.65, 0, 0.25]), 0.045 * K, tone=0.85, region="hydraulics", rep=dict(count=2, step=[0, 0, -1.0]), **CH)
    V.cyl("stick-cyl-barrel", "stick-cyl", V.W("stick-cyl", [0, 0, -0.25]), V.W("stick-cyl", [1.6, 0, -0.25]), 0.095, tone=0.35, region="hydraulics", **PSD)
    V.cyl("stick-cyl-rod", "stick-rod", V.W("stick-rod", [-0.5, 0, -0.25]), V.W("stick-rod", [2.35, 0, -0.25]), 0.05 * K, tone=0.85, region="hydraulics", **CH)
    V.cyl("bucket-cyl-barrel", "bucket-cyl", V.W("bucket-cyl", [0, 0, -0.25]), V.W("bucket-cyl", [0.85, 0, -0.25]), 0.075, tone=0.35, region="hydraulics", **PSD)
    V.cyl("bucket-cyl-rod", "bucket-rod", V.W("bucket-rod", [-0.4, 0, -0.25]), V.W("bucket-rod", [0.95, 0, -0.25]), 0.0378 * max(1.0, K / 1.1), tone=0.85, region="hydraulics", **CH)
    for nm, pos, bone, z0, z1, r in [("boom-cyl-foot", (0.45, 1.1), "upper", -0.85, 0.35, 0.08), ("boom-rod-eye", (2.668, 2.462), "boom", -0.85, 0.35, 0.07), ("stick-cyl-foot", (2.687, 4.306), "boom", -0.4, -0.1, 0.07), ("stick-rod-eye", (6.236, 4.868), "stick", -0.4, -0.1, 0.07), ("bucket-cyl-foot", (6.446, 4.588), "stick", -0.4, -0.1, 0.06)]:
        V.cyl(nm, bone, [pos[0], pos[1], z0], [pos[0], pos[1], z1], r, tone=0.12, maxDenominator=50, **TSD)
    bk = [(1.35, 0), (1.3, 0.05)] + bez((1.3, 0.05), (1.05, 0.62), (0.35, 0.78), (-0.08, 0.34), 14) + [(-0.12, 0.06), (0.02, -0.12)] + bez((0.02, -0.12), (0.4, -0.1), (0.95, -0.1), (1.35, 0), 6)[:-1]
    bkw = loc("bucket", bk)
    V.ext("bucket-cheek", "bucket", "side", bkw, -0.75, -0.75 + max(0.05, FT * 1.02), tone=0.4, region="attachment", rep=dict(count=2, step=[0, 0, 0.95]), **WS)
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


EPS = 1.03
FEAT = {50: 0.0077, 100: 0.0155, 200: 0.031, 500: 0.0755}


def structure(sc):
    f = FEAT[sc]
    V.to("structure")
    ids = {s["id"]: s["bone"] for s in V.lists["body"]}
    mem = ["carbody", "track-frame", "house", "counterweight-body", "boom-body", "stick-body", "bucket-shell", "bucket-cheek", "boom-cyl-barrel", "boom-cyl-rod", "stick-cyl-barrel", "stick-cyl-rod", "bucket-cyl-barrel", "bucket-cyl-rod", "cab"]
    for sid in mem:
        if sid in ids:
            V.member(sid, ids[sid])
    if "carbody" not in ids:
        pass
    if "engine-block" in ids:
        V.member("engine-block", "engine")
    else:
        V.box("member-engine", "engine", [-1.0, 1.525, 0], [1.1, 0.65, 2.4], tone=0.3, member="engine", part="member", material="engine-metal", role="detail")
    V.cyl("joint-slew", "slew-ring", [0, 1.0, 0], [0, 1.0 + max(0.03, f * 1.3), 0], 0.6, tone=0.3, member="upper", part="joint", **PSD)
    for nm, bone, pos in [("joint-boom-foot", "boom", (1.0, 1.65)), ("joint-stick-root", "stick", (5.366, 5.314)), ("joint-bucket-root", "bucket", (7.231, 3.092))]:
        V.cyl(nm, bone, [pos[0], pos[1], -0.45], [pos[0], pos[1], 0.0], max(0.07, f * 0.6), tone=0.12, member=bone, part="joint", **TSD)
    for tag, x, z in [("r2-l", -1.35, 1.2), ("r2-r", -1.35, -1.2), ("r1-l", -0.45, 1.2), ("r1-r", -0.45, -1.2), ("f1-l", 0.45, 1.2), ("f1-r", 0.45, -1.2), ("f2-l", 1.35, 1.2), ("f2-r", 1.35, -1.2)]:
        V.box(f"support-track-{tag}", "undercarriage", [x, f * 0.6, z], [0.3, f * 1.2, 0.5], tone=0.2, member="undercarriage", part="support", anchor="track-" + tag, **TSD)
    V.ell("mass-undercarriage", "undercarriage", [0, 0.58, 0], [0.7, 0.2, 0.5], tone=0.2, member="undercarriage", part="mass", **PSD)
    V.ell("mass-upper", "upper", [-0.5, 1.7, 0], [0.5, 0.4, 0.6], tone=0.2, member="upper", part="mass", **PSD)
    V.ell("mass-counterweight", "counterweight", [-2.35, 1.6, 0], [0.3, 0.4, 0.6], tone=0.2, member="counterweight", part="mass", **CI)
    V.ell("mass-bucket-load", "bucket", V.W("bucket", [0.65, 0.22, -0.25]), [0.3, 0.2, 0.3], tone=0.2, member="bucket", part="mass", **WSD)


def thermal_extra(sc):
    f = FEAT[sc]
    V.to("thermal")
    t = max(f, 0.0075) * EPS
    V.cyl("th-exhaust-stack-jet", "upper", [-1.2, 2.8, -0.9], [-1.2, 2.8 + t, -0.9], 0.06, tone=0.2, region="exhaust-stack", tags=["exhaust-stack"], material="exhaust-steel", role="detail")
    if sc >= 500:
        V.box("th-engine-hood", "upper", [-1.05, 1.9, 0], [1.0, 0.5, 1.4], tone=0.2, region="engine-compartment", material="engine-metal", role="detail")


def build(sc):
    V.scale = sc
    geometry()
    V.auto_thermal()
    thermal_extra(sc)
    structure(sc)


build(50)
V.decide("Every moving part is its own solid on the bone that drives it: tracks (two stadium belts with 16 grousers a side top and bottom), sprocket, idler, five rollers and carrier rollers on the undercarriage; house, cab with sloped glass, hood grille, rail and stack on the slewing bones; counterweight; boom, stick and bucket as extruded Bezier side outlines with bosses, pins and eyes; three cylinder pairs (barrel plus chrome rod) that reach back into their barrels; the bucket is a scoop of cheeks, back shell, floor lip, rib, five teeth, pin and ear. The rest pose is not a closed linkage, dimensions describe the stowed default pose.",
         "Jedes bewegliche Teil ist ein eigener Körper auf dem Knochen, der es antreibt: Ketten (zwei Langloch-Bänder mit je 16 Greifern oben und unten), Kettenrad, Leitrad, fünf Laufrollen und Tragrollen am Unterwagen; Oberwagen, Kabine mit geneigter Verglasung, Haubengitter, Geländer und Auspuff an den Drehknochen; Gegengewicht; Ausleger, Stiel und Löffel als extrudierte Bézier-Seitenumrisse mit Bossen, Bolzen und Augen; drei Zylinderpaare (Rohr und Chromstange), die ins Rohr zurückreichen; der Löffel ist eine Schaufel aus Wangen, Rückenschale, Bodenlippe, Rippe, fünf Zähnen, Bolzen und Ohr. Die Ruhelage ist kein geschlossenes Gestänge, dimensions beschreibt die verstaute Standardpose.")
V.decide("Thermal: house, counterweight, glazing, hood grille, stack, cylinders, boom/stick/bucket and tracks reference their body solids (the engine compartment is the hood grille panels, the tracks the belts and car body); the stack jet is a 120 mm nozzle disc on the stack mouth. Region areas were corrected at the root to the exposed surfaces.",
         "Thermik: Oberwagen, Gegengewicht, Verglasung, Haubengitter, Auspuff, Zylinder, Ausleger/Stiel/Löffel und Ketten verweisen auf ihre Körper (der Motorraum sind die Haubengitterplatten, die Ketten die Bänder und der Wagenkasten); der Auspuffstrahl ist eine 120-mm-Düsenscheibe an der Auspuffmündung. Die Regionsflächen wurden an der Wurzel auf die freien Flächen korrigiert.")
V.decide("Structure: car body, track frames, house, counterweight, boom, stick, bucket shell and cheeks and all six cylinder parts are members, an engine block (structure only) stands in the hood, joints are the slew ring and the three attachment pins, eight 300 mm by 500 mm pads sit under the tracks at the contact anchors and four masses stand for undercarriage, house, counterweight and the bucket payload.",
         "Tragwerk: Wagenkasten, Laufwerksrahmen, Oberwagen, Gegengewicht, Ausleger, Stiel, Löffelschale und -wangen und alle sechs Zylinderteile sind Stäbe, ein Motorblock (nur Tragwerk) steht in der Haube, Gelenke sind der Drehkranz und die drei Anbaubolzen, acht Auflager von 300 mm mal 500 mm sitzen unter den Ketten an den Aufstandsankern und vier Massen stehen für Unterwagen, Oberwagen, Gegengewicht und die Löffellast.")
V.finish(50)
for sc, txt_en, txt_de in [(100, "At 1:100 the grousers, carrier rollers, handrail, brace pins and small eyes drop out (they are below 15 mm or not drawn), tracks keep their belts, sprocket, idler and rollers, rods stay as 76 mm bars; teeth, bucket rib, pin and ear stay.", "Bei 1:100 entfallen Greifer, Tragrollen, Geländer, Gelenkbolzen und kleine Augen (unter 15 mm oder nicht gezeichnet), die Ketten behalten Bänder, Kettenrad, Leitrad und Rollen, Stangen bleiben als 76-mm-Stäbe; Zähne, Löffelrippe, Bolzen und Ohr bleiben."),
                          (200, "At 1:200 sprocket and idler hubs, rollers, rod sleeves, bucket teeth, rib, pin, ear, cab side detail and cab sill are gone; the cab is one block with the glazing, the hood grille panels and boom foot plates disappear into the house.", "Bei 1:200 entfallen Kettenrad- und Leitradnaben, Rollen, Stangenhülsen, Löffelzähne, Rippe, Bolzen, Ohr, Kabinenseitenteile und Fensterbank; die Kabine ist ein Block mit der Verglasung, Haubengitterplatten und Auslegerfußplatten gehen im Oberwagen auf."),
                          (500, "At 1:500 only the masses remain: car body with belts, sprocket and idler as two discs, the house with cab and counterweight, boom, stick and bucket as plain outlines with the cylinder barrels and rods at 75 mm, the stack, windows as glazing sheets; the engine compartment region is carried by an inner hood block.", "Bei 1:500 bleiben nur die Massen: Wagenkasten mit Bändern, Kettenrad und Leitrad als zwei Scheiben, Oberwagen mit Kabine und Gegengewicht, Ausleger, Stiel und Löffel als schlichte Umrisse mit den Zylinderrohren und -stangen zu 75 mm, der Auspuff, Fenster als Glasflächen; die Region Motorraum trägt ein innerer Haubenblock.")]:
    build(sc)
    V.decide(txt_en, txt_de)
    V.decide("Thermal: references to the remaining body solids, the stack jet as a nozzle disc" + (", the engine-compartment surface is a real hood block inside the house." if sc == 500 else "."), "Thermik: Verweise auf die verbleibenden Körper, der Auspuffstrahl als Düsenscheibe" + (", die Oberfläche des Motorraums ist ein echter Haubenblock im Oberwagen." if sc == 500 else "."))
    V.decide("Minimal: the structure and thermal lists are already the smallest complete sets (one member per load-bearing bone, four joints, eight pads, four masses; one solid per region), only sized to the feature of the scale.", "Minimal: Die Tragwerks- und Thermikliste sind bereits die kleinsten vollständigen Sätze (ein Stab je tragendem Knochen, vier Gelenke, acht Auflager, vier Massen; ein Körper je Region), nur auf die Mindestgröße des Maßstabs bemessen.")
    V.decide("Structure: the same members, joints, eight pads and four masses as at 1:50 sized to the feature of the scale.", "Tragwerk: dieselben Stäbe, Gelenke, acht Auflager und vier Massen wie bei 1:50, auf die Mindestgröße des Maßstabs bemessen.")
    V.finish(sc)
V.write()
