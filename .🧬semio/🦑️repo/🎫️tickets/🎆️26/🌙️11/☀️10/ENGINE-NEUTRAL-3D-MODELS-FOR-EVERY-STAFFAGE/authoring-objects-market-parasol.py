import importlib.util, json, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

AL, CN, CV = "aluminium-anodised", "concrete", "polyester-canvas"
A = lib.Asset("market-parasol", AL, 0.35, "frame-alu")
A.box("base-block", "base", [0, 0.1, 0], [0.4, 0.2, 0.4], corner=0.01, material=CN, tone=0.62, region="base-concrete")
A.box("base-plate", "base", [0, 0.2075, 0], [0.28, 0.015, 0.28], corner=0.002, tone=0.35)
A.rev("base-socket", "base", [0, 0.215, 0], [[0, 0], [0, 0.045], [0.085, 0.035], [0.085, 0]], tone=0.3)
A.ell("mast-knuckle", "mast", [0, 0.2, 0], [0.04, 0.04, 0.04], tone=0.3, role="detail", maxDenominator=100)
A.cap("mast", "mast", [0, 0.2, 0], [0, 2.35, 0], 0.026, 0.021, tone=0.45)
A.rev("mast-tilt-collar", "mast", [0, 1.02, 0], [[0, 0], [0, 0.03], [0.004, 0.034], [0.106, 0.034], [0.11, 0.03], [0.11, 0]], tone=0.25, role="detail", maxDenominator=100)
crank = [[-0.021, 1.38, 0], [-0.075, 1.38, 0], [-0.095, 1.34, 0]]
for k in range(2):
    A.cap(f"mast-crank-{k + 1}", "mast", crank[k], crank[k + 1], 0.004, tone=0.3, role="detail", maxDenominator=50)
A.ell("mast-crank-knob", "mast", [-0.097, 1.335, 0], [0.012, 0.012, 0.012], tone=0.2, role="detail", maxDenominator=50)

A.rev("crown-hub", "crown", [0, 2.29, 0], [[0, 0], [0, 0.03], [0.006, 0.041], [0.016, 0.045], [0.124, 0.045], [0.134, 0.041], [0.14, 0.03], [0.14, 0]])
A.rev("crown-finial", "crown", [0, 2.43, 0], [[0, 0], [0, 0.012], [0.02, 0.012], [0.0245, 0.009], [0.0285, 0.005], [0.0313, 0]], tone=0.3)
A.rev("runner-collar", "runner", [0, 1.917, 0], [[0, 0], [0, 0.03], [0.01, 0.035], [0.07, 0.035], [0.08, 0.03], [0.08, 0]])
for side, nm in ((1, "right"), (-1, "left")):
    p0 = [0, 1.95744, 0]
    ang = math.radians(29.0537 if side > 0 else 150.9463)
    A.cap(f"stretch-{nm}", f"stretch-{nm}", p0, [0.55 * math.cos(ang), 1.95744 + 0.55 * math.sin(ang), 0], 0.006, tone=0.4)

R = 1.237426
POSE = json.load(open(os.path.join(lib.T, "🗑️generated", "authoring-objects", "pose-market-parasol-open-still.json")))
ribs = [("right", 1), ("h", 1), ("b", 1), ("g", 1), ("left", -1), ("f", -1), ("d", -1), ("c", -1)]
def dot(a, b):
    return sum(x * y for x, y in zip(a, b))
def local(n, d):
    f = POSE["rib-" + n]
    return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]
az = {n: math.degrees(math.atan2(POSE["rib-" + n]["u"][2], POSE["rib-" + n]["u"][0])) % 360 for n, s in ribs}
order = sorted(az, key=lambda n: az[n])
gap = {}
for i, n in enumerate(order):
    prev, nxt = order[i - 1], order[(i + 1) % len(order)]
    lo = az[n] - ((az[n] - az[prev]) % 360) / 2
    hi = az[n] + ((az[nxt] - az[n]) % 360) / 2
    gap[n] = (lo, hi)
T = math.radians(15.2803)
def tilt(s, h):
    return [-math.sin(T) * s + math.cos(T) * h, math.cos(T) * s + math.sin(T) * h]
lens = lib.bez([0, 0], [0.3465, 0.115], [0.8909, 0.1035], [R, 0], 12)
profile = [tilt(s, h) for s, h in lens] + [tilt(1.3339, -0.008), tilt(0.05, -0.008)]
profile[0] = [0, 0]
for name, sg in ribs:
    A.cap(f"rib-{name}-rod", f"rib-{name}", [0, 2.35, 0], [sg * R, 2.35, 0], 0.011, 0.007, tone=0.35)
    A.ell(f"rib-{name}-tip", f"rib-{name}", [sg * R, 2.35, 0], [0.013, 0.013, 0.013], tone=0.2, role="detail", maxDenominator=100)
    lo, hi = gap[name]
    lo, hi = lo - 0.5, hi + 0.5
    a_loc = local(name, [0, 1, 0])
    e1 = [1 - a_loc[0] * a_loc[0], -a_loc[0] * a_loc[1], -a_loc[0] * a_loc[2]]
    n1 = math.sqrt(dot(e1, e1))
    e1 = [c / n1 for c in e1]
    t = local(name, [math.cos(math.radians(hi)), 0, math.sin(math.radians(hi))])
    cr = [e1[1] * t[2] - e1[2] * t[1], e1[2] * t[0] - e1[0] * t[2], e1[0] * t[1] - e1[1] * t[0]]
    gam = math.atan2(dot(cr, a_loc), dot(e1, t))
    q = [round(c * math.sin(gam / 2), 6) for c in a_loc] + [round(math.cos(gam / 2), 6)]
    A.rev(f"panel-{name}", f"rib-{name}", [0, 2.35, 0], profile, local_axis=[round(c, 6) for c in a_loc], local_q=q, sweep=math.radians(hi - lo), material=CV, tone=0.82, regions=["canopy-top", "canopy-underside"], region=None)
A.save([
    ("The rig rests with all eight ribs collinear along the x axis (the open umbrella exists only through the pose offsets), so a canopy bound to the ribs cannot look open at rest. The eight canopy gores are therefore authored in the open pose (open-still): each is a sector of a surface of revolution about the vertical mast axis, with the lens profile of the 1:50 elevation tilted by the 15.28 degree rib droop, written into the frame of its own rib through the inverse of the open-pose rib frame. In open-still and tilted-to-sun they form the closed cone (bounding box 2.568 x 2.461 x 2.564 m against 2.6 x 2.46 x 2.6 m); in the furled pose they fold with their ribs. The rest bounding box and the rest silhouettes do not describe this asset and miss the gates (see the report).",
     "Das Rig ruht mit allen acht Rippen kollinear auf der x-Achse (der geöffnete Schirm entsteht nur durch die Posen-Offsets), ein an die Rippen gebundenes Dach kann in Ruhe daher nicht geöffnet aussehen. Die acht Dachbahnen sind deshalb in der geöffneten Pose (open-still) modelliert: jede ist ein Sektor einer Rotationsfläche um die senkrechte Mastachse mit dem Linsenprofil der Ansicht 1:50, um das Rippengefälle von 15,28 Grad gekippt und über die Umkehrung des Rippenrahmens der offenen Pose in den Rahmen der eigenen Rippe geschrieben. In open-still und tilted-to-sun bilden sie den geschlossenen Kegel (Hüllquader 2,568 x 2,461 x 2,564 m gegen 2,6 x 2,46 x 2,6 m); in der gefalteten Pose falten sie mit ihren Rippen. Der Hüllquader und die Silhouetten in Ruhe beschreiben dieses Asset nicht und verfehlen die Schranken (siehe Bericht)."),
    ("The gores take the angular half-gap to both neighbouring ribs (the ribs are not equally spaced: gaps of 32, 46, 46 and 56 degrees), overlap by 1 degree, and are filled lens volumes of 8 mm underside thickness so that the ribs stay visible beneath the fabric; the scalloped hem is replaced by a 0.1 m flare to the drawn hem radius of 1.334 m. Only the two stretchers that have bones are modelled; the other six are not.",
     "Die Bahnen übernehmen den halben Winkelabstand zu beiden Nachbarrippen (die Rippen stehen nicht gleichmässig: Abstände von 32, 46, 46 und 56 Grad), überlappen um 1 Grad und sind gefüllte Linsenkörper mit 8 mm Unterseitendicke, damit die Rippen unter dem Stoff sichtbar bleiben; der Bogensaum ist durch einen 0,1 m breiten Ausstellsaum auf den gezeichneten Saumradius 1,334 m ersetzt. Nur die zwei Spreizen mit Knochen sind modelliert, die übrigen sechs nicht."),
    ("Base block, plate and socket, the mast with ball joint, tilt collar and crank, the crown hub with finial and the runner collar are revolves, boxes and capsules as drawn.",
     "Betonfuss, Platte und Hülse, der Mast mit Kugelgelenk, Kippmuffe und Kurbel, Kronennabe mit Spitze und Läuferhülse sind Revolve, Quader und Kapseln wie gezeichnet."),
])
