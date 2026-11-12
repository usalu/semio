import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

EN = "enamel-steel"
A = lib.Asset("charcoal-grill", EN, 0.3, "bowl-shell")
T = 0.004
outer = [[r, y] for r, y in lib.bez([0, 0.5], [0.18, 0.5], [0.2925, 0.6], [0.2925, 0.74], 14)]
K = 4
dish = [[y, r] for r, y in outer[: K + 1]]
A.rev("bowl-dish", "bowl", [0, 0, 0], dish)
for i in range(K, len(outer) - 1):
    (r0, y0), (r1, y1) = outer[i], outer[i + 1]
    A.ring(f"bowl-wall-{i - K + 1}", "bowl", [0, 0], y0, y1, r0, r1, T)

def rin(y):
    return lib.interp(outer, 1, y, 0) - T - 0.002

yk = outer[K][1]
heap = [[yk, 0], [yk + 0.0001, rin(yk + 0.0001)]] + [[y, rin(y)] for y in (0.54, 0.56, 0.58, 0.60)] + [[0.612, 0.2], [0.625, 0.14], [0.634, 0.07], [0.638, 0]]
A.rev("charcoal-bed", "charcoal", [0, 0, 0], heap, material="charcoal", tone=0.25, region="charcoal-bed")

ax = 0.7
A.ring("grate-rim", "grate", [0, 0], ax - 0.003, ax + 0.003, 0.27, 0.27, 0.008, material="chrome-steel", tone=0.5, region="cooking-grate")
for k, z in enumerate((-0.18, -0.09, 0, 0.09, 0.18)):
    h = math.sqrt(0.27 ** 2 - z * z) - 0.004
    A.cap(f"grate-bar-{k + 1}", "grate", [-h, ax, z], [h, ax, z], 0.0035, material="chrome-steel", tone=0.5, role="detail", region="cooking-grate", maxDenominator=100)
for k, x in enumerate((-0.1, 0.1)):
    A.cap(f"grate-brace-{k + 1}", "grate", [x, ax - 0.0075, -0.2], [x, ax - 0.0075, 0.2], 0.0035, material="chrome-steel", tone=0.5, role="detail", region="cooking-grate", maxDenominator=100)

yr = lib.interp(outer, 0, 0.12, 1)
A.rev("ash-catcher", "bowl", [0, 0, 0], [[0.42, 0], [0.42, 0.08], [0.4235, 0.0975], [0.44, 0.12], [yr + 0.002, 0.12]], material="alu-cast", tone=0.6, region="ash-catcher")
A.rev("ash-damper", "ash-damper", [0, 0.47, 0], [[0, 0], [0, 0.05], [0.025, 0.05], [0.025, 0]], material="alu-cast", tone=0.8, role="detail", region="vents-alu", maxDenominator=50)

for sz, nm in ((1, "left"), (-1, "right")):
    A.box(f"side-handle-{nm}", "bowl", [0, 0.73, sz * 0.3045], [0.07, 0.03, 0.035], corner=0.004, material="wood-handle", tone=0.5, region="handle-wood")
    for sx, fb in ((1, "front"), (-1, "back")):
        x0, x1 = sx * 0.27, sx * 0.17
        A.cap(f"leg-{fb}-{nm}", "bowl", [x0, 0.012, sz * 0.14], [x1, 0.58, sz * 0.10], 0.012, material="steel-black", tone=0.3, region="legs-steel")
    A.rev(f"wheel-{nm}", "bowl", [0.27, 0.05, sz * 0.13 if sz > 0 else -0.155], [[0, 0], [0, 0.05], [0.025, 0.05], [0.025, 0]], axis="z", material="steel-black", tone=0.9, region="legs-steel", maxDenominator=100)
    zc = sz * 0.1425
    A.rev(f"wheel-hub-{nm}", "bowl", [0.27, 0.05, zc - 0.0145], [[0, 0], [0, 0.016], [0.029, 0.016], [0.029, 0]], axis="z", material="steel-black", tone=0.2, role="detail", region="legs-steel", maxDenominator=50)
ring_pts = [[-0.2334, 0.22, -0.1254], [-0.2334, 0.22, 0.1254], [0.2334, 0.22, 0.1254], [0.2334, 0.22, -0.1254]]
for i in range(4):
    A.cap(f"leg-ring-{i + 1}", "bowl", ring_pts[i], ring_pts[(i + 1) % 4], 0.005, material="steel-black", tone=0.3, role="detail", region="legs-steel", maxDenominator=100)

lid_pts = [[0.2925, 0], [0.28264, 0.06343], [0.25444, 0.11407], [0.21, 0.1525], [0.14983, 0.17926], [0.08069, 0.19491], [0, 0.2]]
LX, LY = -0.2925, 0.74
KL = 5
ypts = [[0.2925, -0.005]] + lid_pts[1:]
for i in range(KL):
    (r0, y0), (r1, y1) = ypts[i], ypts[i + 1]
    A.ring(f"lid-wall-{i + 1}", "lid", [0.0, 0], LY + y0, LY + y1, r0, r1, 0.004, material=EN, tone=0.3, region="lid-shell")
crown = [[LY + y, r] for r, y in ypts[KL:]]
A.rev("lid-crown", "lid", [0, 0, 0], crown, material=EN, tone=0.3, region="lid-shell")
hp = [(0.5475, 0.095), (0.5925, 0.1), (0.5925, 0.08), (0.6775, 0.08), (0.6775, 0.14), (0.5925, 0.14), (0.5925, 0.125), (0.5475, 0.13)]
A.ext("lid-handle", "lid", [[LX + x, LY + y] for x, y in hp], -0.03, 0.03, material="wood-handle", tone=0.5, region="handle-wood")

A.rev("vent-dial", "lid-vent", [0, 0.935, 0], [[0, 0], [0, 0.04], [0.035, 0.04], [0.035, 0]], material="alu-cast", tone=0.7, region="vents-alu")
for k, x in enumerate((-0.022, 0, 0.022)):
    A.rev(f"vent-hole-{k + 1}", "lid-vent", [x, 0.969, 0], [[0, 0], [0, 0.006], [0.0015, 0.006], [0.0015, 0]], material="alu-cast", tone=0.1, role="detail", region="vents-alu", maxDenominator=50)
A.save([
    ("Bowl and lid are hollow: the curved wall is a stack of tapered rings (extrusions with a hole) of 4 mm wall, a solid dish closes the bottom and a solid crown the lid, so charcoal bed and grate are visible when the lid is open. The kernel always closes a revolve profile to the axis, so an open-top vessel cannot be a revolve.",
     "Kessel und Deckel sind hohl: die gekrümmte Wand ist ein Stapel konischer Ringe (Extrusionen mit Loch) von 4 mm Wandstärke, eine massive Schale schliesst den Boden und eine massive Kuppe den Deckel, sodass Kohlebett und Rost bei offenem Deckel sichtbar sind. Der Kernel schliesst ein Revolve-Profil immer zur Achse, ein oben offenes Gefäss kann deshalb kein Revolve sein."),
    ("The grill has four tubular legs (two with wheels at the front, two feet at the back) tied by a rectangular ring; the elevation draws only two legs and the front view two, the plan shows the four footprints.",
     "Der Grill hat vier Rohrbeine (vorn zwei mit Rädern, hinten zwei Füsse), verbunden durch einen rechteckigen Ring; die Ansicht zeichnet nur zwei Beine, die Vorderansicht zwei, die Draufsicht zeigt die vier Standpunkte."),
    ("The two wooden side handles sit at the rim at z +-0.3 (front view and plan agree); the 1:50 elevation draws a single handle at the front (x 0.29 to 0.345) that no other view supports and that is not modelled. The plan draws the lid handle with the elevation coordinates (z +0.08 to +0.14); the handle is modelled centred (z +-0.03) as the elevation extrusion hint says.",
     "Die zwei Holz-Seitengriffe sitzen am Rand bei z +-0,3 (Vorderansicht und Draufsicht stimmen überein); die Ansicht 1:50 zeichnet einen einzelnen Griff vorn (x 0,29 bis 0,345), den keine andere Ansicht stützt und der nicht modelliert wird. Die Draufsicht zeichnet den Deckelgriff mit den Koordinaten der Ansicht (z +0,08 bis +0,14); der Griff ist mittig modelliert (z +-0,03), wie der Extrusionshinweis der Ansicht sagt."),
    ("The ash damper is a disc inside the ash catcher, hidden at rest; it follows its bone when the vent is opened. The charcoal bed is a heap that follows the inner wall of the bowl; the grate is a rim with five bars and two braces.",
     "Der Aschedämpfer ist eine Scheibe im Aschefänger, in Ruhe verdeckt; er folgt seinem Knochen beim Öffnen. Das Kohlebett ist ein Haufen entlang der Innenwand des Kessels; der Rost ist ein Rand mit fünf Stäben und zwei Streben."),
])
