import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

AL, SZ, RB = "alu-mill", "steel-zinc", "rubber-black"
A = lib.Asset("stepladder", AL, 0.5, "front-rails")
for sz, nm in ((1, "l"), (-1, "r")):
    A.loft(f"front-rail-{nm}", "front-frame", [dict(c=[-0.36, 0.03, sz * 0.2075], rx=0.0225, ry=0.03, e=8), dict(c=[-0.1205, 1.485, sz * 0.1325], rx=0.0225, ry=0.03, e=8)])
    A.loft(f"rear-rail-{nm}", "rear-frame", [dict(c=[0.225, 0.03, sz * 0.1775], rx=0.0225, ry=0.03, e=8), dict(c=[-0.1, 1.215, sz * 0.1225], rx=0.0225, ry=0.03, e=8)], region="rear-frame")
    A.box(f"foot-front-{nm}", "front-frame", [-0.36, 0.0225, sz * 0.205], [0.09, 0.045, 0.06], corner=0.006, material=RB, tone=0.9, region="feet")
    A.box(f"foot-rear-{nm}", "rear-frame", [0.225, 0.0225, sz * 0.1775], [0.09, 0.045, 0.055], corner=0.006, material=RB, tone=0.9, region="feet")
    A.cap(f"spreader-front-{nm}", "front-frame", [-0.2608, 0.62, sz * 0.18], [-0.0621, 0.55, sz * 0.18], 0.013, material=SZ, tone=0.6, region="spreaders")
    A.cap(f"spreader-rear-{nm}", "rear-frame", [-0.119, 0.57, sz * 0.15], [0.08, 0.50, sz * 0.15], 0.013, material=SZ, tone=0.6, region="spreaders")
    A.ell(f"spreader-knuckle-{nm}", "front-frame", [-0.0905, 0.56, sz * 0.165], [0.016, 0.016, 0.016], material=SZ, tone=0.7, role="detail", region="spreaders", maxDenominator=100)
for k, (x, y, w) in enumerate(((-0.35, 0.253, 0.3547), (-0.31, 0.503, 0.3297), (-0.27, 0.753, 0.3047), (-0.23, 1.003, 0.2797))):
    A.box(f"tread-{k}", "front-frame", [x, y, 0], [0.12, 0.03, w], corner=0.008, tone=0.55, region="treads")
A.box("platform", "front-frame", [-0.095, 1.245, 0], [0.27, 0.04, 0.33], corner=0.006, tone=0.4, region="platform")
A.box("top-cap", "front-frame", [-0.12, 1.47, 0], [0.06, 0.06, 0.23], corner=0.02, tone=0.2)
A.cap("hinge", "rear-frame", [-0.13, 1.2, -0.15], [-0.13, 1.2, 0.15], 0.025, tone=0.7, role="detail", region="platform", maxDenominator=100)
A.cap("rear-brace-low", "rear-frame", [0.164, 0.22, -0.17], [0.164, 0.22, 0.17], 0.012, tone=0.6, region="rear-frame")
A.cap("rear-brace-high", "rear-frame", [0.0, 0.85, -0.14], [0.0, 0.85, 0.14], 0.012, tone=0.6, region="rear-frame", maxDenominator=100)
A.save([
    ("Front and rear frame are four lofts of rounded-rectangle section (45 x 60 mm) leaning towards each other, with the rail centre lines taken from the plan capsules (front z +-0.2075 at the foot to +-0.1325 at the top, rear +-0.1775 to +-0.1225); the four treads and the platform are boxes whose widths follow the front view.",
     "Vorder- und Hinterrahmen sind vier Lofts mit abgerundetem Rechteckquerschnitt (45 x 60 mm), die gegeneinander geneigt sind; die Holmachsen stammen aus den Draufsicht-Kapseln (vorn z +-0,2075 am Fuss bis +-0,1325 oben, hinten +-0,1775 bis +-0,1225); die vier Stufen und die Plattform sind Quader, deren Breiten der Vorderansicht folgen."),
    ("The plan draws the upper rear brace at x 0.105 although the elevation puts it on the rear rail near x 0.0; the brace is placed on the rail as the elevation says. The hinge is one pin tube across the rear frame; the two spreader stays meet in knuckles.",
     "Die Draufsicht zeichnet die obere hintere Querstrebe bei x 0,105, die Ansicht setzt sie auf den Hinterholm bei x 0,0; die Strebe sitzt wie in der Ansicht auf dem Holm. Das Gelenk ist ein Bolzenrohr quer über dem Hinterrahmen; die zwei Spreizen treffen sich in Knotenpunkten."),
])
