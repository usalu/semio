import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("oil-pumpjack")
PS = dict(material="painted-steel", role="body")
PSD = dict(material="painted-steel", role="detail")
CI = dict(material="cast-iron", role="body")
CID = dict(material="cast-iron", role="detail")
MS = dict(material="motor-steel", role="body")
P = lambda o, pts: [(o[0] + p[0], o[1] + p[1]) for p in pts]

V.box("slab", "base", [2.0, 0.15, 0], [7.8, 0.3, 2.4], tone=0.82, region="base", material="concrete", role="body")
V.box("skid-rail", "skid", [2.0, 0.425, 1.025], [7.2, 0.25, 0.15], tone=0.5, region="base", rep=dict(count=2, step=[0, 0, -2.05]), **PS)
V.box("skid-cross", "skid", [-1.525, 0.425, 0], [0.15, 0.25, 2.2], tone=0.5, region="base", rep=dict(count=5, step=[1.78, 0, 0]), **PS)
V.cyl("wellhead-base", "base", [-3.0, 0.0, 0], [-3.0, 0.5, 0], 0.17, tone=0.55, region="wellhead", maxDenominator=100, **PS)
V.cyl("wellhead-valve", "base", [-3.0, 0.5, 0], [-3.0, 1.0, 0], 0.1, tone=0.7, region="wellhead", maxDenominator=50, **PS)
V.cyl("stuffing-box", "base", [-3.0, 1.0, 0], [-3.0, 1.28, 0], 0.08, tone=0.45, region="wellhead", maxDenominator=50, **PS)
V.cyl("flowline", "base", [-2.83, 0.68, 0], [-2.0, 0.68, 0], 0.06, tone=0.55, region="wellhead", maxDenominator=50, **PS)
V.cyl("polished-rod", "rod", [-3.0, 1.28, 0], [-3.0, 3.1, 0], 0.03, tone=0.9, region="steel", **PS)
V.cap("bridle", "rod", [-3.0, 3.26, 0.08], [-3.0, 4.4, 0.08], 0.012, tone=0.9, region="steel", rep=dict(count=2, step=[0, 0, -0.16]), **PS)
V.box("carrier", "rod", [-3.0, 3.18, 0], [0.6, 0.16, 0.3], tone=0.3, region="steel", **PS)

legs = {"leg-fl": ((-1.48, 0.55, 0.87), (-0.04, 4.15, 0.16)), "leg-fr": ((-1.48, 0.55, -0.87), (-0.04, 4.15, -0.16)),
        "leg-rl": ((1.48, 0.55, 0.87), (0.04, 4.15, 0.16)), "leg-rr": ((1.48, 0.55, -0.87), (0.04, 4.15, -0.16))}
for name, (a, b) in legs.items():
    V.loft(name, "post", [(a, 0.09, 0.1, 3.0), (b, 0.09, 0.1, 3.0)], hint=(0, 0, 1), tone=0.62, region="steel", **PS)
V.box("brace-side", "post", [0, 1.68, 0.647], [1.9, 0.16, 0.1], tone=0.55, region="steel", rep=dict(count=2, step=[0, 0, -1.294]), **PS)
V.box("brace-end", "post", [-1.028, 1.68, 0], [0.1, 0.16, 1.24], tone=0.55, region="steel", maxDenominator=200, rep=dict(count=2, step=[2.056, 0, 0]), **PS)
V.cyl("saddle", "post", [0, 4.28, -0.28], [0, 4.28, 0.28], 0.16, tone=0.3, region="steel", maxDenominator=50, **PSD)
beam = [(-2.15, -0.2), (-2.15, 0.22), (-1, 0.32), (1, 0.32), (2.8, 0.18), (2.8, -0.18), (1, -0.32), (-1, -0.32)]
V.ext("beam", "beam", "side", P((0, 4.4), beam), -0.18, 0.18, tone=0.7, region="steel", **PS)
horse = [(-2.81908, 1.02606), (-2.89778, 0.77646), (-2.95442, 0.52094), (-2.98858, 0.26147), (-3, 0), (-2.98858, -0.26147), (-2.95442, -0.52094), (-2.89778, -0.77646), (-2.81908, -1.02606), (-2.1, -0.45), (-2.1, 0.45)]
web = [(-2.54734, 0.73044), (-2.60421, 0.4905), (-2.63853, 0.24632), (-2.65, 0), (-2.63853, -0.24632), (-2.60421, -0.4905), (-2.54734, -0.73044), (-2.3, -0.3), (-2.3, 0.3)]
V.ext("horsehead-cheek", "beam", "side", P((0, 4.4), horse), 0.2, 0.35, tone=0.78, region="steel", rep=dict(count=2, step=[0, 0, -0.55]), **PS)
V.ext("horsehead-web", "beam", "side", P((0, 4.4), web), -0.2, 0.2, tone=0.15, region="steel", maxDenominator=50, **PSD)
V.cyl("centre-bearing", "beam", [0, 4.4, -0.3], [0, 4.4, 0.3], 0.2, tone=0.3, region="steel", maxDenominator=50, **PSD)
V.cyl("tail-pin", "beam", [2.7, 4.4, -0.3], [2.7, 4.4, 0.3], 0.14, tone=0.3, region="steel", maxDenominator=50, **PSD)
V.box("pitman", "pitman", V.W("pitman", [0, -1.325, 0.575]), [0.14, 2.65, 0.15], rz=V.angle("pitman"), tone=0.6, region="steel", rep=dict(count=2, step=[0, 0, -1.15]), **PS)
V.cyl("pitman-eye", "pitman", V.W("pitman", [0, -2.65, 0.5]), V.W("pitman", [0, -2.65, 0.65]), 0.12, tone=0.3, region="steel", maxDenominator=50, rep=dict(count=2, step=[0, 0, -1.15]), **PSD)
V.box("gearbox", "gearbox", [2.7, 1.35, 0], [1.4, 1.3, 1.2], cr=0.05, tone=0.62, region="gearbox", **CI)
V.box("pedestal", "gearbox", [2.7, 0.625, 0], [1.1, 0.15, 1.1], tone=0.5, region="gearbox", **CI)
V.ext("belt-guard", "gearbox", "side", P((2.7, 1.75), [(0.7, -0.15), (1.75, -0.53), (1.75, -0.97), (0.7, -1.05)]), -0.95, -0.6, tone=0.45, region="steel", **PS)
sector = [(-0.88257, 0.68954), (-0.98428, 0.53442), (-1.05898, 0.36464), (-1.10464, 0.18485), (-1.12, 0), (-1.10464, -0.18485), (-1.05898, -0.36464), (-0.98428, -0.53442), (-0.88257, -0.68954), (-0.2758, -0.21548), (-0.33093, -0.11395), (-0.35, 0), (-0.33093, 0.11395), (-0.2758, 0.21548)]
V.ext("counterweight", "crank", "side", P((2.7, 1.75), sector), 0.45, 0.7, tone=0.4, region="crank", rep=dict(count=2, step=[0, 0, -1.15]), **CI)
V.box("crank-arm", "crank", [3.175, 1.75, 0.575], [0.95, 0.28, 0.25], tone=0.58, region="crank", rep=dict(count=2, step=[0, 0, -1.15]), **CI)
V.cyl("crank-pin", "crank", V.W("crank", [0.85, 0, 0.45]), V.W("crank", [0.85, 0, 0.7]), 0.1, tone=0.25, region="crank", maxDenominator=100, rep=dict(count=2, step=[0, 0, -1.15]), **CID)
V.cyl("crank-hub", "crank", [2.7, 1.75, -0.8], [2.7, 1.75, 0.8], 0.2, tone=0.3, region="crank", maxDenominator=50, **CID)
V.box("motor", "motor", [4.9, 1.0, 0], [1.1, 0.7, 0.8], cr=0.06, tone=0.68, region="motor", **MS)
V.box("motor-feet", "motor", [4.9, 0.6, 0], [1.0, 0.1, 0.9], tone=0.5, region="motor", **MS)
V.box("motor-cowl", "motor", [5.55, 1.0, 0], [0.2, 0.5, 0.6], cr=0.04, tone=0.5, region="motor", **MS)
V.box("terminal-box", "motor", [4.9, 1.45, 0], [0.3, 0.2, 0.3], cr=0.01, tone=0.5, region="motor", maxDenominator=50, **MS)
V.box("sheave-bracket", "motor", [4.35, 1.0, -0.55], [0.3, 0.2, 0.3], tone=0.4, region="motor", **dict(MS, role="detail"))
V.cyl("motor-sheave", "motor", [4.25, 1.0, -0.85], [4.25, 1.0, -0.7], 0.2, tone=0.3, region="steel", maxDenominator=50, **PSD)
V.cyl("sheave-hub", "motor", [4.25, 1.0, -0.7], [4.25, 1.0, -0.45], 0.05, tone=0.3, region="steel", maxDenominator=50, **PSD)
V.decide("The Samson post is four slanted tube-like legs (rounded-square lofts) from the skid to the saddle, joined by a rectangular brace frame at 1.68 m; leg positions are derived from the elevation polygon (x), the plan capsule (axis in x and z) and the front polygon (z) together.",
         "Der Samsonbock besteht aus vier schraegen rohrartigen Beinen (Lofts mit abgerundetem Quadratprofil) von der Schlitten bis zum Sattel, verbunden durch einen rechteckigen Verstrebungsrahmen bei 1,68 m; die Beinlage folgt zugleich dem Polygon der Ansicht (x), der Kapsel des Grundrisses (Achse in x und z) und dem Polygon der Vorderansicht (z).")
V.decide("The horsehead is two cheek plates (z 0.2 to 0.35, repeated) around a narrower web, so the arc reads as a grooved face; the polished rod and the two bridle cables are bound to the rod bone with the carrier bar, not to the base as in the 2D drawing, so they travel with the stroke (the 2D rod stays put in every pose). The bridle top stays within 0.2 m of the horsehead arc in every pose.",
         "Der Pferdekopf besteht aus zwei Wangenplatten (z 0,2 bis 0,35, gespiegelt) um einen schmaleren Steg, so liest sich der Bogen als gerillte Flaeche; Polierstange und die zwei Seile sind mit der Traverse auf den Knochen rod gebunden und nicht wie in der 2D-Zeichnung an die Basis, damit sie den Hub mitmachen (die 2D-Stange bleibt in jeder Pose stehen). Das Seilende bleibt in jeder Pose innerhalb von 0,2 m am Pferdekopfbogen.")
V.decide("Pitman rods, crank arms, counterweight sectors and crank pins exist as left and right solids with a mirror repeat; the rest pose has all bone angles 0 so the crank pin (x 3.55) and the pitman eye (x 2.7) do not meet at rest, they meet in the authored poses where the rig closes the linkage (checked). The skid is 2.2 m wide as in the front view (plan: 2.0 m).",
         "Schubstangen, Kurbelarme, Gegengewichtssektoren und Kurbelzapfen sind links und rechts je ein Koerper mit Spiegelwiederholung; die Ruhelage hat alle Knochenwinkel 0, daher treffen sich Kurbelzapfen (x 3,55) und Schubstangenauge (x 2,7) in Ruhe nicht, sie treffen sich in den autorisierten Posen, in denen das Rig das Gestaenge schliesst (geprueft). Der Schlitten ist 2,2 m breit wie in der Vorderansicht (Grundriss: 2,0 m).")
V.write(os.path.join(here, "oil-pumpjack.json"))
