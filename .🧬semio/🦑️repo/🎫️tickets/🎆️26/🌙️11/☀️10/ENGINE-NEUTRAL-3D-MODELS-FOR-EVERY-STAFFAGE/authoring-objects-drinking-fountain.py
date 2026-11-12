import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

CI, SB, CB, WA = "cast-iron", "stainless-basin", "chrome-brass", "water"
A = lib.Asset("drinking-fountain", CI, 0.6, "pedestal-cast")
ped = [[0, 0], [0, 0.2], [0.04, 0.2], [0.055, 0.17], [0.075, 0.16], [0.08, 0.15], [0.7, 0.1], [0.72, 0.11], [0.745, 0.09], [0.753, 0.09]]
A.rev("pedestal", "pedestal", [0, 0, 0], [[a, r] for a, r in ped])
for k, y in enumerate((0.2, 0.42, 0.62)):
    r = 0.15 - (y - 0.08) / 0.62 * 0.05
    A.rev(f"collar-{k + 1}", "pedestal", [0, y - 0.005, 0], [[0, 0], [0, r + 0.004], [0.01, r + 0.004], [0.01, 0]], role="detail", tone=0.3, maxDenominator=200)
A.rev("bolt", "pedestal", [0.19, 0.04, 0], [[0, 0], [0, 0.008], [0.008, 0.008], [0.008, 0]], role="detail", tone=0.2, maxDenominator=50, repeat={"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": 1.570796, "origin": [0, 0, 0]})

BY = 0.74
outer = lib.bez([0, 0], [0.12, 0], [0.2, 0.05], [0.2, 0.125], 12)
outer = [[r, y + BY] for r, y in outer]
K = 3
A.rev("basin-dish", "basin", [0, 0, 0], [[y, r] for r, y in outer[: K + 1]], material=SB, tone=0.5, region="basin-steel")
for i in range(K, len(outer) - 1):
    (r0, y0), (r1, y1) = outer[i], outer[i + 1]
    A.ring(f"basin-wall-{i - K + 1}", "basin", [0, 0], y0, y1, r0, r1, 0.004, material=SB, tone=0.5, region="basin-steel")
A.ring("basin-rim-bead", "basin", [0, 0], BY + 0.119, BY + 0.131, 0.205, 0.205, 0.016, material=SB, tone=0.5, region="basin-steel")
yk = outer[K][1]
A.rev("drain", "basin", [0.04, yk, 0], [[0, 0], [0, 0.022], [0.0008, 0.022], [0.0008, 0]], material=SB, tone=0.2, role="detail", region="basin-steel", maxDenominator=50)

BX, BYb = -0.15, 0.86
col = -0.155
path = [[col, 0.855, 0], [col, 0.93, 0]]
path += lib.arc([col + 0.056, 0.93, 0], 0.056, 180, 90, 6)[1:]
path += [[-0.075, 0.986, 0], [-0.057, 0.981, 0], [-0.05, 0.968, 0]]
radii = [0.026, 0.026] + [0.026 - (0.026 - 0.014) * i / 6 for i in range(1, 7)] + [0.014, 0.014, 0.014]
A.loft("bubbler", "bubbler", [dict(c=c, rx=r, ry=r) for c, r in zip(path, radii)], smooth=True, material=CB, tone=0.7, region="bubbler-brass")
A.rev("bubbler-collar", "bubbler", [col, 0.895, 0], [[0, 0], [0, 0.031], [0.01, 0.031], [0.01, 0]], material=CB, tone=0.3, role="detail", region="bubbler-brass", maxDenominator=200)
A.rev("nozzle-ring", "bubbler", [-0.05, 0.958, 0], [[0, 0], [0, 0.016], [0.011, 0.016], [0.011, 0]], material=CB, tone=0.3, region="bubbler-brass")
A.rev("nozzle-bore", "bubbler", [-0.05, 0.968, 0], [[0, 0], [0, 0.007], [0.0012, 0.007], [0.0012, 0]], material=CB, tone=0.2, role="detail", region="bubbler-brass", maxDenominator=50)

A.ell("button", "button", [0.108, 0.66, 0], [0.02, 0.025, 0.02], material=CB, tone=0.8, region="bubbler-brass")
A.rev("button-ring", "button", [0.099, 0.66, 0], [[0, 0], [0, 0.03], [0.006, 0.03], [0.006, 0]], axis="x", material=CB, tone=0.3, role="detail", region="bubbler-brass", maxDenominator=100)

jx, jy = -0.045, 0.955
curve = lib.bez([0, 0], [0.025, 0.045], [0.095, 0.045], [0.13, -0.085], 10)
for nm, part in (("jet-rise", curve[:6]), ("jet-fall", curve[5:])):
    A.loft(nm, "jet", [dict(c=[jx + x, jy + y, 0], rx=0.003, ry=0.003) for x, y in part], smooth=True, material=WA, tone=0.5, role="detail", region="water-jet")
for k, (x, y) in enumerate(((0.115, -0.04), (0.14, -0.07))):
    A.ell(f"drop-{k + 1}", "jet", [jx + x, jy + y, 0], [0.007, 0.007, 0.007], material=WA, tone=0.3, role="detail", region="water-jet", maxDenominator=50)
A.save([
    ("The pedestal is one revolve of the drawn outline (flange, tapering fluted column, neck), raised 8 mm at the top so that it meets the underside of the basin; the flutes, hatched in the drawing, are not modelled, the three collars are thin discs embedded in the column, the four flange bolts are one repeated solid turned by 90 degrees.",
     "Der Sockel ist ein Revolve der gezeichneten Kontur (Flansch, konisch zulaufende kannelierte Säule, Hals), oben 8 mm erhöht, damit er die Beckenunterseite trifft; die in der Zeichnung schraffierten Kanneluren werden nicht modelliert, die drei Kragen sind dünne, in die Säule eingebettete Scheiben, die vier Flanschschrauben sind ein um je 90 Grad gedrehter, wiederholter Körper."),
    ("The basin is hollow: a solid dish at the bottom, a stack of tapered rings of 4 mm wall for the curved wall (the kernel closes every revolve profile to the axis, so an open vessel cannot be a revolve) and a rim bead ring that reaches 5 mm beyond the 0.4 m flange, 2.5 % on width and depth.",
     "Das Becken ist hohl: eine massive Schale unten, ein Stapel konischer Ringe von 4 mm Wandstärke für die gekrümmte Wand (der Kernel schliesst jedes Revolve-Profil zur Achse, ein offenes Gefäss kann deshalb kein Revolve sein) und ein Randwulst, der 5 mm über den 0,4 m breiten Flansch hinausreicht, 2,5 % in Breite und Tiefe."),
    ("The bubbler is one loft along the gooseneck (round column of 52 mm tapering to the 28 mm arm) ending in a nozzle ring with an upward bore, which resolves the contradiction of the nozzle drawn as a circle in both the elevation and the front view; the water jet is a thin two-part loft along the drawn parabola plus two drops on the jet bone, which in the idle poses is lowered by 0.25 m into the basin and pedestal and thereby disappears.",
     "Der Sprudler ist ein Loft entlang des Schwanenhalses (runde Säule von 52 mm, auslaufend in den 28 mm Arm), das in einem Düsenring mit nach oben gerichteter Bohrung endet; das löst den Widerspruch, dass die Düse in Ansicht und Vorderansicht als Kreis gezeichnet ist; der Wasserstrahl ist ein dünnes zweiteiliges Loft entlang der gezeichneten Parabel plus zwei Tropfen am Strahlknochen, der in den Ruheposen um 0,25 m ins Becken und in den Sockel abgesenkt wird und so verschwindet."),
    ("The push button is a flattened ellipsoid with a bezel disc on the pedestal front (+x), moving 10 mm into the pedestal in the drinking pose.",
     "Der Druckknopf ist ein flacher Ellipsoid mit Blendscheibe an der Sockelfront (+x), der sich in der Trinkpose 10 mm in den Sockel bewegt."),
])
