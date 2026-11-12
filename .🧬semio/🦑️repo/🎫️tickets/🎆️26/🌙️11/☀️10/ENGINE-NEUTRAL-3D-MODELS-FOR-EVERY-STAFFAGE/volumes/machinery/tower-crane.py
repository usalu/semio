import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("tower-crane")
ST = dict(material="steel-s355", role="body")
STD = dict(material="steel-s355", role="detail")
MC = dict(material="machinery", role="body")
MCD = dict(material="machinery", role="detail")
CN = dict(material="concrete", role="body")
WR = dict(material="wire-rope", role="detail")
SK = dict(material="skip-steel", role="body")
SKD = dict(material="skip-steel", role="detail")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]
sq = lambda h: [(-h, -h), (h, -h), (h, h), (-h, h)]
P = lambda x, y, z: [x, y, z]

V.box("ballast-slab", "base", [-2.625, 0.5, 0], [1.74, 1.0, 3.5], tone=0.35, region="ballast", rep=dict(count=4, step=[1.75, 0, 0]), **CN)
V.loft("mast-foot", "base", [(P(0, 1.0, 0), 1.5, 1.5, 8), (P(0, 1.5, 0), 1.15, 1.15, 8)], tone=0.5, region="lattice", **STD)
for sx in (1, -1):
    for sz in (1, -1):
        V.box(f"mast-chord-{'p' if sx > 0 else 'n'}x{'p' if sz > 0 else 'n'}z", "mast", [sx * 0.92, 20.5, sz * 0.92], [0.16, 39.0, 0.16], tone=0.5, region="lattice", **ST)
for nm, a, b in [("px", (0.92, 0, -0.92), (0.92, 0, 0.92)), ("nx", (-0.92, 0, -0.92), (-0.92, 0, 0.92)), ("pz", (-0.92, 0, 0.92), (0.92, 0, 0.92)), ("nz", (-0.92, 0, -0.92), (0.92, 0, -0.92))]:
    V.cap(f"mast-tie-{nm}", "mast", [a[0], 1.0, a[2]], [b[0], 1.0, b[2]], 0.05, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=20, step=[0, 2.0, 0]), **STD)
faces = {"pz": ((-0.92, 0.92), 0.92, "x"), "nz": ((0.92, -0.92), -0.92, "x"), "px": ((-0.92, 0.92), 0.92, "z"), "nx": ((0.92, -0.92), -0.92, "z")}
for nm, ((s0, s1), fixed, axis) in faces.items():
    def pt(s, y):
        return [s, y, fixed] if axis == "x" else [fixed, y, s]
    V.cap(f"mast-diag-a-{nm}", "mast", pt(s0, 1.0), pt(s1, 3.0), 0.04, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=10, step=[0, 4.0, 0]), **STD)
    V.cap(f"mast-diag-b-{nm}", "mast", pt(s1, 3.0), pt(s0, 5.0), 0.04, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=10, step=[0, 4.0, 0]), **STD)
V.cyl("slew-ring", "slewing", [0, 40.0, 0], [0, 40.5, 0], 1.4, tone=0.3, region="slew-drive", maxDenominator=100, **MCD)
V.box("slew-platform", "slewing", [0.5, 40.85, 0], [5.8, 0.7, 2.4], tone=0.5, region="house", **MC)
hh = lambda y: 1.2 - (1.2 - 0.12) * (y - 41.2) / 7.2
corners = [(1, 1), (1, -1), (-1, -1), (-1, 1)]
for k, (sx, sz) in enumerate(corners, 1):
    V.cap(f"tower-head-leg-{k}", "slewing", [sx * 1.2, 41.2, sz * 1.2], [sx * 0.12, 48.4, sz * 0.12], 0.07, tone=0.5, region="lattice", **ST)
levels = [41.2, 43.8, 45.2, 46.6, 48.0]
for k, y in enumerate(levels[1:], 1):
    h = hh(y)
    V.ext(f"tower-head-tie-{k}", "slewing", "plan", sq(h), y - 0.04, y + 0.04, holes=[sq(h - 0.06)], tone=0.5, region="lattice", maxDenominator=200, **STD)
V.ext("tower-head-frame", "slewing", "plan", sq(1.2), 41.2, 41.28, holes=[sq(1.14)], tone=0.5, region="lattice", maxDenominator=200, **STD)
n = 1
for k in range(4):
    y0, y1 = levels[k], levels[k + 1]
    h0, h1 = hh(y0), hh(y1)
    for f, ((ax, az), (bx, bz)) in enumerate([((1, -1), (1, 1)), ((1, 1), (-1, 1)), ((-1, 1), (-1, -1)), ((-1, -1), (1, -1))]):
        V.cap(f"tower-head-diagonal-{n}", "slewing", [ax * h0, y0, az * h0], [bx * h1, y1, bz * h1], 0.03, tone=0.5, region="lattice", maxDenominator=200, **STD)
        n += 1
V.cyl("head-sheave", "slewing", [0, 48.6, -0.1], [0, 48.6, 0.1], 0.25, tone=0.3, maxDenominator=200, material="steel-s355", role="body")
V.box("cab", "slewing", [2.35, 42.3, -0.5], [1.9, 2.2, 1.0], cr=0.1, tone=0.6, region="cab", **MC)
V.box("cab-glass-side", "slewing", [2.6, 42.45, -1.005], [1.2, 1.5, 0.01], tone=0.85, region="cab-glazing", material="glass", role="glazing")
V.box("cab-glass-front", "slewing", [3.305, 42.45, -0.5], [0.01, 1.5, 0.7], tone=0.85, region="cab-glazing", material="glass", role="glazing")
V.cap("pendant-back", "slewing", [0, 48.6, 0], [-15.6, 45.1, 0], 0.015, tone=0.3, region="pendants", maxDenominator=200, **WR)
V.cap("pendant-fore-1", "slewing", [0, 48.6, 0.15], [22, 45.2, 0.15], 0.015, tone=0.3, region="pendants", maxDenominator=200, **WR)
V.cap("pendant-fore-2", "slewing", [0, 48.6, -0.15], [50, 45.2, -0.15], 0.015, tone=0.3, region="pendants", maxDenominator=200, **WR)
V.cap("jib-chord", "jib", [0, 45.14, 0.74], [55, 45.14, 0.74], 0.07, tone=0.5, region="lattice", rep=dict(count=2, step=[0, 0, -1.48]), **ST)
V.cap("jib-rail", "jib", [0, 43.66, 0.45], [55, 43.66, 0.45], 0.06, tone=0.35, region="lattice", maxDenominator=200, rep=dict(count=2, step=[0, 0, -0.9]), **STD)
for s, zt, zb in [("l", 0.74, 0.45), ("r", -0.74, -0.45)]:
    V.cap(f"jib-diag-down-{s}", "jib", [0, 45.14, zt], [1.8, 43.66, zb], 0.04, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=15, step=[3.6, 0, 0]), **STD)
    V.cap(f"jib-diag-up-{s}", "jib", [1.8, 43.66, zb], [3.6, 45.14, zt], 0.04, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=15, step=[3.6, 0, 0]), **STD)
V.cap("jib-brace-a", "jib", [0, 45.14, -0.74], [1.8, 45.14, 0.74], 0.03, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=15, step=[3.6, 0, 0]), **STD)
V.cap("jib-brace-b", "jib", [1.8, 45.14, 0.74], [3.6, 45.14, -0.74], 0.03, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=15, step=[3.6, 0, 0]), **STD)
V.box("jib-frame-root", "jib", [0.05, 44.4, 0], [0.1, 1.6, 1.6], tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=2, step=[54.9, 0, 0]), **STD)
V.cyl("jib-tip-sheave", "jib", [55.0, 44.4, -0.1], [55.0, 44.4, 0.1], 0.3, tone=0.3, maxDenominator=200, material="steel-s355", role="detail")
V.cap("counter-chord-top", "counter-jib", V.W("counter-jib", [0, -0.64, 0.64]), V.W("counter-jib", [16.5, -0.64, 0.64]), 0.06, tone=0.5, region="lattice", rep=dict(count=2, step=[0, 0, -1.28]), **ST)
V.cap("counter-chord-bottom", "counter-jib", V.W("counter-jib", [0, 0.64, 0.64]), V.W("counter-jib", [16.5, 0.64, 0.64]), 0.06, tone=0.5, region="lattice", rep=dict(count=2, step=[0, 0, -1.28]), **ST)
for s, z in [("l", 0.64), ("r", -0.64)]:
    a = lambda u, v, zz: V.W("counter-jib", [u, v, zz])
    V.cap(f"counter-diag-down-{s}", "counter-jib", a(0, -0.64, z), a(1.65, 0.64, z), 0.035, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=5, step=[-3.3, 0, 0]), **STD)
    V.cap(f"counter-diag-up-{s}", "counter-jib", a(1.65, 0.64, z), a(3.3, -0.64, z), 0.035, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=5, step=[-3.3, 0, 0]), **STD)
V.cap("counter-brace-a", "counter-jib", V.W("counter-jib", [0, -0.64, -0.64]), V.W("counter-jib", [1.65, -0.64, 0.64]), 0.03, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=5, step=[-3.3, 0, 0]), **STD)
V.cap("counter-brace-b", "counter-jib", V.W("counter-jib", [1.65, -0.64, 0.64]), V.W("counter-jib", [3.3, -0.64, -0.64]), 0.03, tone=0.5, region="lattice", maxDenominator=200, rep=dict(count=5, step=[-3.3, 0, 0]), **STD)
V.box("counterweight-slabs", "counterweight", [-14.9, 41.8667, 0], [2.4, 0.7233, 2.4], tone=0.3, region="counterweight", rep=dict(count=3, step=[0, 0.7333, 0]), **CN)
V.box("winch-house", "winch", [-8.0, 46.05, 0], [4.0, 1.8, 1.4], cr=0.05, tone=0.55, region="winch-house", **MC)
V.box("winch-motor-grille", "winch", [-9.0, 45.95, 0.705], [1.2, 0.8, 0.01], tone=0.15, region="hoist-motor", maxDenominator=200, rep=dict(count=2, step=[0, 0, -1.41]), **MCD)
V.box("winch-louvre", "winch", [-9.0, 45.85, 0.71], [1.0, 0.05, 0.01], tone=0.3, region="winch-house", maxDenominator=50, rep=dict(count=3, step=[0, 0.2, 0]), **MCD)
V.box("trolley-frame", "trolley", [3.0, 43.325, 0], [1.4, 0.55, 2.0], tone=0.4, region="trolley", **MC)
V.box("trolley-wheel", "trolley", [2.6, 43.54, 0.45], [0.2, 0.12, 0.12], tone=0.2, region="trolley", maxDenominator=100, rep=dict(count=2, step=[0.8, 0, 0]), **MCD)
V.box("trolley-wheel-r", "trolley", [2.6, 43.54, -0.45], [0.2, 0.12, 0.12], tone=0.2, region="trolley", maxDenominator=100, rep=dict(count=2, step=[0.8, 0, 0]), **MCD)
V.cyl("trolley-sheave", "trolley", [3.0, 43.28, -0.1], [3.0, 43.28, 0.1], 0.18, tone=0.25, region="trolley", maxDenominator=100, material="steel-s355", role="detail")
V.box("block-body", "hoist-block", [3.0, 37.05, 0], [0.56, 0.5, 0.56], cr=0.03, tone=0.3, region="skip", **SK)
V.cyl("block-sheave", "hoist-block", [3.0, 37.05, -0.1], [3.0, 37.05, 0.1], 0.18, tone=0.5, region="skip", maxDenominator=100, **SKD)
def bez(p0, p1, p2, p3, n=8):
    return [tuple((1 - t) ** 3 * p0[k] + 3 * (1 - t) ** 2 * t * p1[k] + 3 * (1 - t) * t ** 2 * p2[k] + t ** 3 * p3[k] for k in (0, 1)) for t in [i / n for i in range(0, n + 1)]]
hook = [(0, -0.5), (0, -0.8)] + bez((0, -0.8), (0.3, -0.8), (0.35, -1.15), (0.05, -1.2))[1:] + bez((0.05, -1.2), (-0.1, -1.22), (-0.16, -1.1), (-0.1, -1.02))[1:]
V.loft("hook", "hoist-block", [([3.0 + p[0], 37.3 + p[1], 0.0], 0.04, 0.04) for p in hook], tone=0.2, region="skip", maxDenominator=100, caps=(True, True), **SKD)
V.box("skip-wall-side", "skip", [3.0, 35.3, 0.685], [1.3, 1.0, 0.03], tone=0.45, region="skip", rep=dict(count=2, step=[0, 0, -1.37]), **SK)
ang = math.degrees(math.atan(0.25 / 1.0))
V.box("skip-wall-end-p", "skip", [3.575, 35.3, 0], [0.03, 1.03, 1.4], rz=-ang, tone=0.45, region="skip", **SK)
V.box("skip-wall-end-n", "skip", [2.425, 35.3, 0], [0.03, 1.03, 1.4], rz=ang, tone=0.45, region="skip", **SK)
V.box("skip-bottom", "skip", [3.0, 34.815, 0], [0.9, 0.03, 0.9], tone=0.45, region="skip", **SK)
V.box("skip-gate", "skip", [3.0, 34.725, 0], [0.4, 0.15, 0.4], tone=0.25, region="skip", maxDenominator=100, **SKD)
for k, (sx, sz) in enumerate([(0.7, 0.7), (0.7, -0.7), (-0.7, 0.7), (-0.7, -0.7)], 1):
    V.cap(f"sling-{k}", "skip", [3.0, 36.8, 0.0], [3.0 + sx, 35.8, sz], 0.015, tone=0.3, region="pendants", maxDenominator=100, **WR)
for k in range(1, 9):
    y = 43.05 - 5.0 * (k - 1)
    V.cyl(f"rope-{k}", f"rope-{k}", [3.0, y - 5.75, 0], [3.0, y, 0], 0.012, tone=0.2, region="rope", maxDenominator=200, **WR)
V.decide("Lattice booms are built as real space trusses from members (not filled envelopes): mast with four corner chords, 20 rings of horizontals and zigzag diagonals on every face (2 m pitch, written as repeats); jib with two top chords, two bottom rails (the trolley track at z +-0.45) and side and top zigzag bracing at 1.8 m; counter jib with four chords and bracing at 1.65 m; tower head as a four-leg pyramid with four tie rings and 16 diagonals. The 2D elevation and front fill these envelopes as closed rectangles, so silhouette IoU of this asset is low by construction (see report).",
         "Die Gitterausleger sind als echte Raumfachwerke aus Staeben gebaut (nicht als gefuellte Huellen): Mast mit vier Eckgurten, 20 Ringen aus Horizontalen und Zickzack-Diagonalen auf jeder Seite (2 m Teilung, als Wiederholungen geschrieben); Ausleger mit zwei Obergurten, zwei Untergurten (Katzfahrbahn bei z +-0,45) und seitlicher und oberer Zickzack-Aussteifung bei 1,8 m; Gegenausleger mit vier Gurten und Aussteifung bei 1,65 m; Turmspitze als vierbeinige Pyramide mit vier Ringen und 16 Diagonalen. Die 2D-Ansicht und -Vorderansicht fuellen diese Huellen als geschlossene Rechtecke, daher ist die Silhouetten-IoU dieses Assets konstruktionsbedingt niedrig (siehe Bericht).")
V.decide("Every moving part is on its own bone: trolley (frame, wheels, sheave) on the trolley bone, hoist block with sheave and hook on hoist-block, the skip (four walls, bottom, gate, four slings) on the skip bone, and each of the eight rope segments (5.75 m round bars, 5 m pitch with 0.75 m overlap) on its own rope bone so the rope shortens and lengthens with the hoist; the turntable twist rotates slewing, jib, counter jib, winch and counterweight about the mast axis.",
         "Jedes bewegliche Teil sitzt auf seinem Knochen: Katze (Rahmen, Raeder, Seilrolle) auf dem Knochen trolley, Hakenflasche mit Seilrolle und Haken auf hoist-block, die Mulde (vier Waende, Boden, Klappe, vier Anschlagseile) auf dem Knochen skip, und jedes der acht Seilsegmente (Rundstaebe 5,75 m, Teilung 5 m mit 0,75 m Ueberlappung) auf seinem Seilknochen, so verkuerzt und verlaengert sich das Seil mit dem Hub; die Drehung des Drehkranzes dreht Oberwagen, Ausleger, Gegenausleger, Winde und Gegengewicht um die Mastachse.")
V.decide("dimensions describe the default pose lifting-load: width 71.8 m is the counter jib end (-16.5) to the jib tip sheave (55.3), height 48.85 m the head sheave top, depth 3.5 m the ballast slab; in the rest pose the same box applies because the turntable is not twisted. The block, hook and skip hang at their rest height (y 34.8 to 37.3) and move with the hoist poses.",
         "dimensions beschreibt die Standardpose lifting-load: Breite 71,8 m ist das Gegenauslegerende (-16,5) bis zur Auslegerspitzenrolle (55,3), Hoehe 48,85 m die Oberkante der Spitzenrolle, Tiefe 3,5 m die Ballastplatte; in der Ruhelage gilt dieselbe Box, weil der Drehkranz nicht verdreht ist. Flasche, Haken und Mulde haengen in Ruhe auf ihrer Ruhehoehe (y 34,8 bis 37,3) und folgen den Hubposen.")
V.write(os.path.join(here, "tower-crane.json"))
