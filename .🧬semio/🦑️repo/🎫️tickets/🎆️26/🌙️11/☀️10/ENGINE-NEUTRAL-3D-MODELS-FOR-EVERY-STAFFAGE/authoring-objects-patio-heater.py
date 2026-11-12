import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

PS, SB, SH, AR, PB = "powder-steel", "stainless-brushed", "stainless-hot", "alu-reflector", "plastic-black"
A = lib.Asset("patio-heater", PS, 0.6, "base-housing")
A.box("base-plate", "base", [0, 0.015, 0], [0.46, 0.03, 0.46], corner=0.006)
top = lib.bez([0.17, 0.6], [0.17, 0.64], [0.05, 0.66], [0.03, 0.66], 6)
secs = [dict(c=[0, 0.03, 0], rx=0.17, ry=0.17, e=5), dict(c=[0, 0.3, 0], rx=0.17, ry=0.17, e=5)] + [dict(c=[0, y, 0], rx=r, ry=r, e=5 if r > 0.1 else 3) for r, y in top]
A.loft("housing", "base", secs, first="z")
A.box("housing-door", "base", [0, 0.305, -0.1715], [0.24, 0.43, 0.003], tone=0.55, role="detail", maxDenominator=200)
for sz, nm in ((1, "left"), (-1, "right")):
    A.rev(f"wheel-{nm}", "base", [-0.14, 0.05, sz * 0.185 - 0.01], [[0, 0], [0, 0.04], [0.02, 0.04], [0.02, 0]], axis="z", material=PB, tone=0.9)

A.rev("pole", "pole", [0, 0.62, 0], [[0, 0], [0, 0.03], [1.36, 0.03], [1.36, 0]], material=SB, tone=0.45, region="pole-stainless")
A.rev("pole-joint", "pole", [0, 1.295, 0], [[0, 0], [0, 0.033], [0.01, 0.033], [0.01, 0]], material=SB, tone=0.3, role="detail", region="pole-stainless", maxDenominator=100)

A.rev("head", "burner-head", [0, 1.98, 0], [[0, 0], [0, 0.11], [0.003, 0.125], [0.012, 0.13], [0.108, 0.13], [0.117, 0.125], [0.12, 0.11], [0.12, 0]], material=SH, tone=0.8, region="emitter-head")
A.cap("guard-rod", "burner-head", [0.1325, 1.99, 0], [0.1325, 2.055, 0], 0.003, material=SH, tone=0.3, role="detail", region="emitter-head", maxDenominator=100, repeat={"count": 12, "rotationAxis": [0, 1, 0], "rotationRad": round(2 * math.pi / 12, 6), "origin": [0, 0, 0]})

outer = [[r, y] for r, y in lib.bez([0.4, 0], [0.4, 0.06], [0.22, 0.19], [0, 0.19], 12)]
K = 8
for i in range(K):
    (r0, y0), (r1, y1) = outer[i], outer[i + 1]
    A.ring(f"reflector-wall-{i + 1}", "reflector", [0, 0], 2.06 + y0, 2.06 + y1, r0, r1, 0.003, material=AR, tone=0.5, region="reflector")
A.rev("reflector-crown", "reflector", [0, 0, 0], [[2.06 + y, r] for r, y in outer[K:]], material=AR, tone=0.5, region="reflector")
A.ell("top-cap", "reflector", [0, 2.06 + 0.178, 0], [0.05, 0.012, 0.05], material=AR, tone=0.2, role="detail", region="reflector", maxDenominator=100)

A.rev("knob", "knob", [0.17, 0.46, -0.02], [[0, 0], [0, 0.03], [0.04, 0.03], [0.04, 0]], axis="z", material=PB, tone=0.2, region="control")
A.box("knob-pointer", "knob", [0.185, 0.46, -0.0215], [0.03, 0.004, 0.003], material=PB, tone=0.3, role="detail", region="control", maxDenominator=100)
A.save([
    ("The base housing is a loft of rounded-square sections (exponent 5) on a 0.46 m plate, 0.34 m wide as the elevation and front view agree, with the domed top of the drawn outline; the door seam is a 3 mm proud panel shown to 1:200.",
     "Das Sockelgehäuse ist ein Loft aus abgerundeten Quadratschnitten (Exponent 5) auf einer 0,46 m Platte, 0,34 m breit wie Ansicht und Vorderansicht übereinstimmen, mit der gezeichneten gewölbten Oberseite; die Türfuge ist ein 3 mm vorstehendes Paneel bis 1:200."),
    ("The two transport wheels are drawn inside the housing footprint in the front view; they are placed just outside the housing sides (z +-0.185) at the drawn rear position so that they are visible and carry the housing.",
     "Die zwei Transporträder sind in der Vorderansicht innerhalb des Gehäusegrundrisses gezeichnet; sie sind knapp ausserhalb der Gehäuseseiten (z +-0,185) an der gezeichneten hinteren Position platziert, damit sie sichtbar sind und das Gehäuse tragen."),
    ("The reflector dome is hollow (a stack of tapered 3 mm rings plus a solid crown) and covers the emitter head, which carries a ring of 12 guard rods modelled as one repeated solid; the kernel closes every revolve to the axis so an open dome cannot be a revolve.",
     "Die Reflektorhaube ist hohl (ein Stapel konischer Ringe von 3 mm plus eine massive Kuppe) und deckt den Brennerkopf ab, der einen Kranz aus 12 Schutzstäben trägt, als ein wiederholter Körper modelliert; der Kernel schliesst jedes Revolve zur Achse, eine offene Haube kann deshalb kein Revolve sein."),
    ("The control knob turns about the lateral axis (rig joint about w), so it is a short cylinder with its axis along z lying on the front face of the housing at the rig position, with a pointer on its -z face that makes the poses 0, 60 and 120 degrees readable; the elevation and the front view both draw it as a circle, which cannot both hold.",
     "Der Reglerknopf dreht um die Querachse (Rig-Gelenk um w); er ist deshalb ein kurzer Zylinder mit Achse in z, der an der Rig-Position auf der Vorderseite des Gehäuses liegt, mit einem Zeiger auf seiner -z-Fläche, der die Posen 0, 60 und 120 Grad lesbar macht; Ansicht und Vorderansicht zeichnen ihn beide als Kreis, was nicht beides stimmen kann."),
])
