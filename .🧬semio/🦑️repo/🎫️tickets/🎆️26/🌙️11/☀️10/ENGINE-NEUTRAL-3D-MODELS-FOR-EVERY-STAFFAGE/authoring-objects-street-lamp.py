import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

GS, CO, AA, PC = "galv-steel", "concrete", "aluminium-anodised", "polycarbonate"
A = lib.Asset("street-lamp", GS, 0.4, "pole-galv")
A.box("plinth", "foundation", [0, 0.05, 0], [0.3, 0.1, 0.3], material=CO, tone=0.65, region="plinth-concrete")
A.box("base-plate", "foundation", [0, 0.11, 0], [0.3, 0.02, 0.3], corner=0.002, tone=0.35)
A.rev("base-nut", "foundation", [0.12, 0.12, 0.12], [[0, 0], [0, 0.0125], [0.014, 0.0125], [0.014, 0]], tone=0.2, role="detail", maxDenominator=100, repeat={"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": round(math.pi / 2, 6), "origin": [0, 0, 0]})
cov = [[y, r] for r, y in lib.bez([0.12, 0.13], [0.11, 0.20], [0.085, 0.24], [0.082, 0.30], 8)]
A.rev("base-cover", "foundation", [0, 0, 0], [[0.13, 0]] + cov, tone=0.33, maxDenominator=100)

prof = [[0, 0]] + [[p[0], p[1]] for p in lib.bez([0, 0.085], [2, 0.08], [4, 0.06], [5.88, 0.04], 12)] + [[5.8705, 0.034], [5.877, 0.022], [5.88, 0]]
A.rev("pole", "pole", [0, 0.12, 0], prof)
A.rev("pole-collar", "pole", [0, 5.72, 0], [[0, 0], [0, 0.055], [0.08, 0.055], [0.08, 0]], tone=0.3, role="detail", maxDenominator=100)
A.box("pole-hatch", "pole", [0, 0.92, -0.0815], [0.09, 0.5, 0.004], corner=0.002, tone=0.3, role="detail", maxDenominator=50)

AL = dict(material=AA, region="arm-alu")
A.cap("arm", "arm", [0, 5.8, 0], [0.95, 5.8, 0], 0.034, 0.01, tone=0.5, **AL)
A.cap("arm-brace", "arm", [0.02, 5.34, 0], [0.62, 5.788, 0], 0.012, tone=0.45, **AL)
A.ell("arm-brace-joint", "arm", [0.62, 5.788, 0], [0.02, 0.02, 0.02], tone=0.3, role="detail", maxDenominator=100, **AL)
A.box("arm-clamp", "arm", [0, 5.8, 0], [0.1, 0.1, 0.1], corner=0.008, tone=0.3, role="detail", maxDenominator=100, **AL)

HX, HY = 0.72, 5.8
LH = dict(material=AA, region="lamp-housing")
secs = [(0.0, 0.07, 0.035, -0.005), (0.05, 0.125, 0.0575, -0.0025), (0.15, 0.13, 0.071, 0.009), (0.25, 0.125, 0.068, 0.01), (0.30, 0.08, 0.045, -0.005), (0.33, 0.04, 0.015, -0.01)]
A.loft("head-housing", "head", [dict(c=[HX + x, HY + cy, 0], rx=rz, ry=ry, e=3) for x, rz, ry, cy in secs], tone=0.5, **LH)
A.rev("head-spigot", "head", [0.68, HY + 0.005, 0], [[0, 0], [0, 0.028], [0.07, 0.028], [0.07, 0]], axis="x", tone=0.35, role="detail", maxDenominator=100, **LH)
A.box("head-fin", "head", [HX + 0.15, HY + 0.0775, 0], [0.004, 0.023, 0.15], tone=0.3, role="detail", maxDenominator=50, repeat={"count": 6, "translation": [0.028, 0, 0]}, **LH)
A.ell("lens", "head", [HX + 0.165, HY - 0.058, 0], [0.135, 0.03, 0.1], material=PC, tone=0.85, role="glazing", region="lamp-lens")
A.save([
    ("The pole is one revolve of the drawn tapering outline (0.17 m at the foot to 0.08 m at the top, the 1:50 front view agrees) with a rounded cap; arm, brace, clamp and lamp head are bound to their own bones so the 1.8 degree wind lean of the pole carries them along. The dimension 1.2 m wide is the plinth edge at x -0.15 to the head tip at x 1.05.",
     "Der Mast ist ein Revolve der gezeichneten konischen Kontur (0,17 m am Fuss bis 0,08 m oben, die Vorderansicht 1:50 stimmt überein) mit gerundeter Kappe; Ausleger, Strebe, Klemme und Leuchtenkopf sind an eigene Knochen gebunden, damit die Windneigung des Mastes von 1,8 Grad sie mitnimmt. Die Breite 1,2 m reicht von der Sockelkante bei x -0,15 bis zur Kopfspitze bei x 1,05."),
    ("The lamp head is a loft of six superelliptic sections whose lateral half widths follow the plan outline and heights the elevation outline, with six cooling fins as one repeated plate and a flattened-ellipsoid lens under it; the base cover, plinth nuts (four, as the plan shows, one repeated solid) and the hand-hole plate on the pole are details shown to 1:100 or 1:50.",
     "Der Leuchtenkopf ist ein Loft aus sechs superelliptischen Schnitten, deren seitliche Halbbreiten der Umriss der Draufsicht und deren Höhen dem Umriss der Ansicht folgen, mit sechs Kühlrippen als eine wiederholte Platte und einer flachen Ellipsoidlinse darunter; Sockelabdeckung, Fundamentmuttern (vier wie in der Draufsicht, ein wiederholter Körper) und die Handlochplatte am Mast sind Details bis 1:100 beziehungsweise 1:50."),
])
