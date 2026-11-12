import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

def rrect(hz, hx, c):
    return [[-hz + c, -hx], [-hz, -hx + c], [-hz, hx - c], [-hz + c, hx], [hz - c, hx], [hz, hx - c], [hz, -hx + c], [hz - c, -hx]]

A = lib.Asset("storage-crate-stack", "hdpe-crate", 0.45)
t = 0.004
for k in range(1, 6):
    bone, y0, d = f"crate-{k}", 0.3 * (k - 1), 0.1 if k % 2 == 0 else 0.0
    kw = dict(region=bone)
    A.box(f"c{k}-foot", bone, [0, y0 + 0.01, 0], [0.54, 0.02, 0.3], tone=0.35 + d, **kw)
    A.box(f"c{k}-floor", bone, [0, y0 + 0.026, 0], [0.56, 0.012, 0.34], corner=0.004, tone=0.45 + d, **kw)
    s = A.meta(f"c{k}-body", bone, dict(tone=0.45 + d, **kw))
    A.finish(s, "extrude", {"profile": rrect(0.17, 0.28, 0.015), "holes": [rrect(0.17 - t, 0.28 - t, 0.012)], "plane": "zx", "from": 0.02, "to": 0.28, "taperTo": [1, 1.09]})
    s = A.meta(f"c{k}-rim", bone, dict(tone=0.55 + d, **kw))
    A.finish(s, "extrude", {"profile": rrect(0.2, 0.31, 0.02), "holes": [rrect(0.186, 0.296, 0.015)], "plane": "zx", "from": 0.28, "to": 0.3})
    for sz, nm in ((1, "a"), (-1, "b")):
        A.box(f"c{k}-slot-{nm}", bone, [-0.17, y0 + 0.14, sz * 0.1805], [0.1, 0.04, 0.002], tone=0.1, role="detail", maxDenominator=50, repeat={"count": 3, "translation": [0.17, 0, 0]}, **kw)
    for sx, nm in ((1, "c"), (-1, "d")):
        A.box(f"c{k}-slot-{nm}", bone, [sx * 0.2955, y0 + 0.14, 0], [0.002, 0.04, 0.14], tone=0.1, role="detail", maxDenominator=50, **kw)
A.save([
    ("Each crate is an open-top hollow body: a foot slab, a floor, a 4 mm tapered wall (extrusion of a rounded-rectangle ring with taper, 0.56 x 0.34 m at the floor to 0.61 x 0.37 m at the rim, the single uniform taper factor cannot reach the drawn 0.60 x 0.38 m exactly) and a 20 mm rim ring of 0.62 x 0.40 m, so a crate sinking into the one below (poses half and low, 0.22 m) nests as drawn.",
     "Jede Kiste ist ein oben offener Hohlkörper: eine Fussplatte, ein Boden, eine 4 mm starke konische Wand (Extrusion eines abgerundeten Rechteckrings mit Verjüngung, 0,56 x 0,34 m am Boden bis 0,61 x 0,37 m am Rand; der eine einheitliche Verjüngungsfaktor trifft die gezeichneten 0,60 x 0,38 m nicht genau) und ein 20 mm Randring von 0,62 x 0,40 m, sodass eine in die untere einsinkende Kiste (Posen half und low, 0,22 m) wie gezeichnet verschachtelt."),
    ("The hand slots are dark plates 2 mm proud of the walls (three on each long side as one repeated solid, one on each end) shown to 1:50 only; the ribs drawn on the front are not modelled. The stack is five crates, each bound to its own bone, with the odd/even tone alternation of the drawing.",
     "Die Grifflöcher sind dunkle Platten, 2 mm vor den Wänden (drei je Längsseite als ein wiederholter Körper, eines je Stirnseite) und nur bis 1:50 gezeigt; die an der Vorderseite gezeichneten Rippen sind nicht modelliert. Der Stapel besteht aus fünf Kisten, je an ihren eigenen Knochen gebunden, mit dem Tonwechsel ungerade/gerade der Zeichnung."),
])
