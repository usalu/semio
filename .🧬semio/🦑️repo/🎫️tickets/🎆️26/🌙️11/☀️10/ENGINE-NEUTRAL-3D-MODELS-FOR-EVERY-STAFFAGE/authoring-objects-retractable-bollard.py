import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

SG, RF, PA, CI, SS, SH = "steel-grey", "retro-film", "pc-amber", "cast-iron-frame", "steel-sleeve", "steel-hyd"
A = lib.Asset("retractable-bollard", SG, 0.5, "shaft-steel")
cap = [[-0.07 + 0.0, 0.1]] + [[y, r] for r, y in lib.bez([0.1, -0.07], [0.1, -0.02], [0.05, 0], [0, 0], 6)][1:]
A.rev("shaft", "shaft", [0, 0, 0], [[-1.15, 0], [-1.15, 0.1], [-0.07, 0.1]] + cap[1:])
A.rev("reflector", "shaft", [0, -0.24, 0], [[0, 0], [0, 0.1015], [0.11, 0.1015], [0.11, 0]], material=RF, tone=0.08, region="reflector-band")
A.rev("beacon", "shaft", [0, -0.13, 0], [[0, 0], [0, 0.1015], [0.06, 0.1015], [0.06, 0]], material=PA, tone=0.7, region="beacon-led")
A.rev("led", "shaft", [0.1015, -0.1, 0], [[0, 0], [0, 0.01], [0.002, 0.01], [0.002, 0]], axis="x", tone=0.1, role="detail", region="beacon-led", maxDenominator=50, repeat={"count": 12, "rotationAxis": [0, 1, 0], "rotationRad": round(math.pi / 6, 6), "origin": [0, 0, 0]})

A.ring("sleeve-wall", "sleeve", [0, 0], -1.25, -0.07, 0.15, 0.15, 0.005, material=SS, tone=0.3, region="sleeve-steel")
A.rev("sleeve-floor", "sleeve", [0, -1.25, 0], [[0, 0], [0, 0.15], [0.01, 0.15], [0.01, 0]], material=SS, tone=0.3, region="sleeve-steel")

def sq(h, c):
    return [[-h + c, -h], [-h, -h + c], [-h, h - c], [-h + c, h], [h - c, h], [h, h - c], [h, -h + c], [h - c, -h]]

A.plan("cover-frame", "sleeve", [[p[1], p[0]] for p in sq(0.2, 0.02)], -0.07, 0.015, holes=[[[0.145 * math.cos(2 * math.pi * i / 40), 0.145 * math.sin(2 * math.pi * i / 40)] for i in range(40)]], material=CI, tone=0.6, region="cover-frame")
for k, (x, z) in enumerate(((-0.175, -0.175), (0.175, -0.175), (-0.175, 0.175), (0.175, 0.175))):
    A.rev(f"frame-bolt-{k + 1}", "sleeve", [x, 0.015, z], [[0, 0], [0, 0.01], [0.003, 0.01], [0.003, 0]], material=CI, tone=0.2, role="detail", region="cover-frame", maxDenominator=50)
A.ring("brush-seal", "sleeve", [0, 0], -0.02, -0.01, 0.145, 0.145, 0.045, material=CI, tone=0.3, role="detail", region="cover-frame", maxDenominator=100)

A.box("power-unit", "hydraulic", [0.255, -0.75, 0.095], [0.21, 0.4, 0.21], corner=0.01, material=SH, tone=0.5, region="hydraulic-unit")
A.rev("motor", "hydraulic", [0.255, -0.64, 0.025], [[0, 0], [0, 0.035], [0.14, 0.035], [0.14, 0]], axis="z", material=SH, tone=0.8, role="detail", region="hydraulic-unit", maxDenominator=100)
A.rev("ram", "hydraulic", [0, -1.25, 0], [[0, 0], [0, 0.06], [0.55, 0.06], [0.55, 0]], material=SH, tone=0.7, region="hydraulic-unit")
A.rev("ram-rod", "hydraulic", [0, -0.7, 0], [[0, 0], [0, 0.02], [0.25, 0.02], [0.25, 0]], material=SH, tone=0.3, role="detail", region="hydraulic-unit", maxDenominator=100)
A.cap("hose", "hydraulic", [0.15, -0.7, 0.03], [0.055, -0.7, 0.03], 0.008, material=SH, tone=0.3, role="detail", region="hydraulic-unit", maxDenominator=100)
A.save([
    ("The asset is drawn as a cut through the ground: sleeve, hydraulic unit and ram are buried (y -1.25 to -0.07) and invisible in the raised pose except for the shaft; the shaft rises with its prismatic joint by up to 0.65 m. `dimensions` (1.9 m high) describe the raised default pose; at rest (shaft lowered) the body is 1.27 m high.",
     "Das Asset ist als Schnitt durch den Boden gezeichnet: Hülse, Hydraulikeinheit und Zylinder sind vergraben (y -1,25 bis -0,07) und in der ausgefahrenen Pose bis auf den Schaft unsichtbar; der Schaft fährt mit seinem Schiebegelenk bis 0,65 m aus. `dimensions` (1,9 m hoch) beschreiben die ausgefahrene Standardpose; in Ruhe (Schaft abgesenkt) ist der Körper 1,27 m hoch."),
    ("The sleeve is a hollow tube (a 5 mm ring extrusion with a floor disc) so the shaft slides inside it; the cast cover frame is a rounded-square ring around the 0.29 m opening with four bolts; the beacon carries a ring of 12 LED discs (the views show three per side).",
     "Die Hülse ist ein Hohlrohr (Ringextrusion von 5 mm mit Bodenscheibe), in dem der Schaft gleitet; der Gussrahmen ist ein abgerundeter Quadratring um die 0,29 m Öffnung mit vier Schrauben; die Leuchte trägt einen Kranz aus 12 LED-Scheiben (die Ansichten zeigen drei pro Seite)."),
    ("The hydraulic unit sits beside the sleeve at +x (plan and front agree), the ram stands under the shaft with its rod reaching the shaft base in the raised pose and the hose runs through the sleeve wall.",
     "Die Hydraulikeinheit sitzt neben der Hülse bei +x (Draufsicht und Vorderansicht stimmen überein), der Zylinder steht unter dem Schaft, seine Stange erreicht in der ausgefahrenen Pose den Schaftfuss, der Schlauch führt durch die Hülsenwand."),
])
