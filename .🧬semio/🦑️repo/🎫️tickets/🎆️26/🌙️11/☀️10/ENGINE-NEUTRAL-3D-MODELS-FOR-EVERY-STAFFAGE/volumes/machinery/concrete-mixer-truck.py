import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("concrete-mixer-truck")
PS = dict(material="paint-steel", role="body")
PSD = dict(material="paint-steel", role="detail")
DP = dict(material="drum-paint", role="body")
DPD = dict(material="drum-paint", role="detail")
GS = dict(material="galvanised-steel", role="body")
GSD = dict(material="galvanised-steel", role="detail")
CI = dict(material="cast-iron", role="detail")
GL = dict(material="glass", role="glazing")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]
loc = lambda bone, pts: [tuple(V.W(bone, [p[0], p[1], 0])[:2]) for p in pts]

V.ext("frame-rail", "chassis", "side", [(-4.5, 0.5), (-4.5, 0.9), (4.3, 0.9), (4.3, 0.42), (4.2, 0.42), (4.2, 0.62), (-4.2, 0.62), (-4.2, 0.5)], 0.3, 0.45, tone=0.4, region="chassis", rep=dict(count=2, step=[0, 0, -0.75]), **PS)
V.box("frame-cross", "chassis", [-3.5, 0.76, 0], [0.12, 0.28, 0.6], tone=0.4, region="chassis", maxDenominator=200, rep=dict(count=5, step=[1.8, 0, 0]), **PSD)
V.box("bumper-frame", "chassis", [4.25, 0.66, 0], [0.1, 0.48, 2.3], tone=0.4, region="chassis", **PS)
V.cyl("fuel-tank", "chassis", [0, 0.77, -0.62], [1.2, 0.77, -0.62], 0.13, tone=0.35, region="chassis", maxDenominator=100, **PSD)
V.cyl("air-tank", "chassis", [1.5, 0.81, 0.62], [2.2, 0.81, 0.62], 0.15, tone=0.35, region="chassis", maxDenominator=50, **PSD)
cab = [(2.4, 0.9), (2.4, 3.14), (2.43, 3.24), (2.5, 3.29), (2.62, 3.3), (4.02, 3.3), (4.14, 3.27), (4.22, 3.2), (4.36, 2.66), (4.46, 2.52), (4.5, 2.36), (4.5, 0.9)]
V.ext("cab", "chassis", "side", cab, -1.2, 1.2, tone=0.5, region="cab", **PS)
V.box("cab-windscreen", "chassis", [4.31, 2.85, 0], [0.02, 0.62, 2.1], rz=14.5, tone=0.85, region="cab-glazing", **GL)
V.ext("cab-door-glass", "chassis", "side", rect(3.0, 3.76, 2.52, 3.14), 1.2, 1.21, tone=0.85, region="cab-glazing", rep=dict(count=2, step=[0, 0, -2.41]), **GL)
V.ext("cab-quarter-glass", "chassis", "side", [(4.3, 2.52), (4.14, 3.12), (3.92, 3.12), (3.92, 2.52)], 1.2, 1.21, tone=0.85, region="cab-glazing", rep=dict(count=2, step=[0, 0, -2.41]), **GL)
V.ext("door-line", "chassis", "side", [(2.7, 1.12), (3.9, 1.12), (3.9, 3.0), (2.7, 3.0)], 1.2, 1.205, holes=[[(2.73, 1.15), (2.73, 2.97), (3.87, 2.97), (3.87, 1.15)]], tone=0.3, region="cab", maxDenominator=200, rep=dict(count=2, step=[0, 0, -2.405]), **PSD)
V.box("door-handle", "chassis", [3.77, 2.03, 1.215], [0.3, 0.06, 0.03], tone=0.2, region="cab", maxDenominator=50, rep=dict(count=2, step=[0, 0, -2.43]), **PSD)
V.box("front-grille", "chassis", [4.505, 1.625, 0], [0.01, 0.65, 1.4], tone=0.2, region="engine-bay", maxDenominator=200, **PSD)
V.box("head-lamp", "chassis", [4.465, 1.35, 0.9], [0.07, 0.3, 0.4], tone=0.9, region="cab", maxDenominator=50, rep=dict(count=2, step=[0, 0, -1.8]), material="glass", role="detail")
V.ext("mirror-arm", "chassis", "side", [(4.3, 2.7), (4.5, 2.7), (4.5, 2.95), (4.4, 2.95)], 1.2, 1.275, tone=0.2, region="cab", maxDenominator=100, rep=dict(count=2, step=[0, 0, -2.475]), **PSD)
V.box("cab-step", "chassis", [4.1, 0.87, 1.1], [0.4, 0.26, 0.3], tone=0.3, region="chassis", maxDenominator=100, rep=dict(count=2, step=[0, 0, -2.2]), **PSD)
V.cyl("exhaust-stack", "chassis", [2.32, 1.3, -1.1], [2.32, 3.35, -1.1], 0.1, tone=0.2, region="exhaust-stack", material="exhaust-steel", role="body")
V.cyl("stack-cap", "chassis", [2.32, 3.35, -1.1], [2.32, 3.45, -1.1], 0.12, tone=0.25, maxDenominator=100, material="exhaust-steel", role="detail")
V.box("pedestal", "chassis", [-2.325, 1.825, 0.55], [0.55, 1.85, 0.2], tone=0.5, region="chassis", rep=dict(count=2, step=[0, 0, -1.1]), **PS)
V.cyl("pedestal-roller", "chassis", [-2.325, 2.7, -0.55], [-2.325, 2.7, 0.55], 0.1, tone=0.3, region="chassis", maxDenominator=200, **PSD)
V.box("hinge-post", "chassis", [-3.925, 1.625, 0], [0.25, 1.45, 0.3], tone=0.5, region="chassis", **PS)
V.box("rear-ladder-rail", "chassis", [-4.2, 1.81, 0.3], [0.04, 1.38, 0.04], tone=0.4, region="chassis", maxDenominator=50, rep=dict(count=2, step=[0, 0, -0.6]), **PSD)
V.cyl("rear-ladder-rung", "chassis", [-4.2, 1.3, -0.3], [-4.2, 1.3, 0.3], 0.02, tone=0.4, region="chassis", maxDenominator=50, rep=dict(count=5, step=[0, 0.27, 0]), **PSD)
dprof = [(0, 0), (0.05, 0.3681), (0.14, 0.5999), (0.28, 0.8119), (0.45, 0.9699), (0.65, 1.0759), (0.9, 1.12)]
V.rev("drum-dome-cap", "drum", V.W("drum", [0, 0, 0]), list(V.frames["drum"]["u"]), [[a, r] for a, r in dprof + [(1.15, 1.12)]], tone=0.3, region="drum", **DP)
V.rev("drum-belly", "drum", V.W("drum", [1.15, 0, 0]), list(V.frames["drum"]["u"]), [[0, 1.12], [1.7, 1.12]], tone=0.3, region="drum", **DP)
cone_out = [(0, 1.12), (2.55, 0.5)]
cone = [[a, r] for a, r in cone_out] + [[2.55, 0.47], [0, 1.09], [0, 1.12]]
V.rev("drum-cone-cap", "drum", V.W("drum", [2.85, 0, 0]), list(V.frames["drum"]["u"]), cone, tone=0.3, region="drum", **DP)
def radius(a):
    pts = dprof + [(3.0, 1.12), (5.4, 0.5)]
    for (a0, r0), (a1, r1) in zip(pts, pts[1:]):
        if a0 <= a <= a1:
            return r0 + (r1 - r0) * (a - a0) / (a1 - a0)
    return 0.5
helix = []
a0, a1, pitch = 0.4, 4.55, 0.85
n = int((a1 - a0) / pitch * 12)
for i in range(n + 1):
    a = a0 + (a1 - a0) * i / n
    th = 2 * math.pi * (a - a0) / pitch
    r = radius(a) + 0.02
    helix.append(V.W("drum-spiral", [a, r * math.cos(th), r * math.sin(th)]))
V.loft("drum-flight", "drum-spiral", [(p, 0.035, 0.035) for p in helix], tone=0.75, region="drum", maxDenominator=200, caps=(True, True), **DPD)
t = 2 * math.pi / 28
V.cap("drum-seam", "drum", V.W("drum", [3.0, 1.12, 0]), V.W("drum", [3.0, 1.12 * math.cos(t), 1.12 * math.sin(t)]), 0.025, tone=0.3, region="drum", maxDenominator=100,
      rep=dict(count=28, angle=t, axis=list(V.frames["drum"]["u"]), origin=V.W("drum", [3.0, 0, 0])), **DPD)
V.ext("drive-housing", "drum", "side", loc("drum", [(0.3798, 1.3701), (-0.3964, 1.1765), (-0.2634, 0.6429), (0.5129, 0.8364)]), -0.4, 0.4, tone=0.35, region="drum-drive", maxDenominator=200, **PS)
hp = [(5.1936, 0.4575), (5.4677, 0.598), (6.0058, 0.0932), (6.1026, -0.2949), (5.4429, -0.3357)]
hw = loc("drum", hp)
V.ext("hopper-cheek", "drum", "side", hw, 0.5, 0.55, tone=0.55, region="hopper", rep=dict(count=2, step=[0, 0, -1.05]), **GS)
cx = sum(p[0] for p in hw) / 5; cy = sum(p[1] for p in hw) / 5
def strip(pts, t=0.04):
    out = []
    for i, p in enumerate(pts):
        a = pts[max(i - 1, 0)]; b = pts[min(i + 1, len(pts) - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy) or 1
        nx, ny = -dy / L, dx / L
        if (cx - p[0]) * nx + (cy - p[1]) * ny < 0:
            nx, ny = -nx, -ny
        out.append((p[0] + t * nx, p[1] + t * ny))
    return list(pts) + out[::-1]
V.ext("hopper-shell", "drum", "side", strip(hw[1:]), -0.55, 0.55, tone=0.55, region="hopper", **GS)
V.cyl("chute-swivel-collar", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.4, 0], 0.2, tone=0.4, region="chute", **GSD)
V.cyl("chute-pin", "chute", V.W("chute", [0, 0, -0.35]), V.W("chute", [0, 0, 0.35]), 0.07, tone=0.2, region="chute", maxDenominator=200, **GSD)
cm = [(0, -0.14), (2.4, -0.1), (2.4, 0.14), (0, 0.14)]
V.ext("chute-wall", "chute", "side", loc("chute", cm), 0.245, 0.275, tone=0.5, region="chute", rep=dict(count=2, step=[0, 0, -0.52]), **GS)
V.box("chute-floor", "chute", V.W("chute", [1.2, -0.125, 0]), [2.4, 0.03, 0.52], rz=V.angle("chute"), tone=0.5, region="chute", **GS)
V.ext("chute-ext-wall", "chute-ext", "side", loc("chute-ext", rect(0, 1.3, 0, 0.22)), 0.27, 0.3, tone=0.6, region="chute", rep=dict(count=2, step=[0, 0, -0.57]), **GS)
V.box("chute-ext-floor", "chute-ext", V.W("chute-ext", [0.65, 0.015, 0]), [1.3, 0.03, 0.54], rz=V.angle("chute-ext"), tone=0.6, region="chute", **GS)
V.cyl("chute-ext-pin", "chute-ext", V.W("chute-ext", [0, 0, -0.32]), V.W("chute-ext", [0, 0, 0.32]), 0.06, tone=0.2, region="chute", maxDenominator=200, **GSD)
for k, x in enumerate([3.55, 1.95, -1.7, -3.05], 1):
    bone = f"wheel-{k}"
    for side, sg in [("l", 1), ("r", -1)]:
        prof = [[sg * a, r] for a, r in [(-0.16, 0), (-0.16, 0.42), (-0.14, 0.49), (-0.09, 0.52), (0.09, 0.52), (0.14, 0.49), (0.16, 0.42), (0.16, 0.3), (0.13, 0.3), (0.13, 0)]]
        zc = sg * 1.115
        V.rev(f"tyre-{k}-{side}", bone, [x, 0.52, zc], [0, 0, 1], prof, tone=0.12, region="tyres", material="rubber", role="body")
        V.cyl(f"brake-drum-{k}-{side}", bone, [x, 0.52, zc + sg * 0.13], [x, 0.52, zc + sg * 0.155], 0.3, tone=0.6, region="brakes", maxDenominator=200, **CI)
        V.cyl(f"hub-{k}-{side}", bone, [x, 0.52, zc + sg * 0.155], [x, 0.52, zc + sg * 0.16], 0.1, tone=0.3, region="brakes", maxDenominator=100, **CI)
        V.cyl(f"handhole-{k}-{side}", bone, [x, 0.72, zc + sg * 0.155], [x, 0.72, zc + sg * 0.162], 0.05, tone=0.0, region="brakes", maxDenominator=100,
              rep=dict(count=5, angle=2 * math.pi / 5, axis=[0, 0, 1], origin=[x, 0.52, 0]), **CI)
    V.cyl(f"axle-{k}", "chassis", [x, 0.52, -1.0], [x, 0.52, 1.0], 0.08, tone=0.3, region="chassis", material="cast-iron", role="detail")
V.decide("The drum is a hollow revolved shell about its tilted axis in three pieces (dome cap, belly cylinder, cone cap whose open mouth shows the inside) with a thin helical flight rib as one swept tube on the drum-spiral bone, so the rib slides along the drum as the rig advances the flights (0.85 m = one pitch); the helix covers 0.4 to 4.55 m of the axis at rest so at the largest shift it ends at the mouth.",
         "Die Trommel ist eine hohle gedrehte Schale um ihre geneigte Achse in drei Teilen (Kalotte, Mantelzylinder, Konus mit offener Muendung, die den Innenraum zeigt) mit einer duennen schraubenfoermigen Wendelrippe als ein gefuehrtes Rohr auf dem Knochen drum-spiral, so gleitet die Rippe mit dem Vorschub der Wendel (0,85 m = eine Steigung) an der Trommel entlang; die Schraube deckt in Ruhe 0,4 bis 4,55 m der Achse ab, sodass sie beim groessten Versatz an der Muendung endet.")
V.decide("The chute is a trough (two side walls from the drawn outline plus a floor) on the chute bone and the folded extension a second trough on the chute-ext bone with its pin, the swivel collar on the chute-swivel bone; the hopper is two cheek plates and a back sheet on the drum bone. The 2D chute (z 0.55 wide at x -3.9 to -2.0, y 2.2 to 3.7) lies inside the drum cone silhouette at rest, kept as drawn.",
         "Die Schurre ist eine Rinne (zwei Seitenwaende nach dem gezeichneten Umriss plus Boden) auf dem Knochen chute, die eingeklappte Verlaengerung eine zweite Rinne auf dem Knochen chute-ext mit Bolzen, der Drehkranz auf dem Knochen chute-swivel; der Fuelltrichter besteht aus zwei Wangenplatten und einem Rueckenblech auf dem Knochen drum. Die 2D-Schurre (0,55 m breit bei x -3,9 bis -2,0, y 2,2 bis 3,7) liegt in Ruhe innerhalb der Konus-Silhouette der Trommel und bleibt wie gezeichnet.")
V.decide("Four axles carry eight wheels (revolved tyres with a dished outer face, brake drum, hub and five hand-hole bolts that make the rotation readable), the frame is two rails with cross members and a bumper bar, the cab one extruded outline with glass plates, a sloped windscreen and a door frame; fuel and air tanks hang on opposite frame sides (z -0.62, +0.62: the 2D gives no lateral position).",
         "Vier Achsen tragen acht Raeder (gedrehte Reifen mit eingezogener Aussenseite, Bremstrommel, Nabe und fuenf Handlochbolzen, die die Drehung lesbar machen), der Rahmen besteht aus zwei Laengstraegern mit Querstreben und einem Stossfaenger, die Kabine aus einem extrudierten Umriss mit Glasplatten, geneigter Frontscheibe und Tuerrahmen; Kraftstoff- und Drucklufttank haengen an gegenueberliegenden Rahmenseiten (z -0,62, +0,62: die 2D gibt keine Querlage an).")
V.write(os.path.join(here, "concrete-mixer-truck.json"))
