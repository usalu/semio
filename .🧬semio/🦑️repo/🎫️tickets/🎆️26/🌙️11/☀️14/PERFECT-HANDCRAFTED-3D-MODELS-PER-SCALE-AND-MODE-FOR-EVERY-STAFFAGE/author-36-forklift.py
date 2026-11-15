import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-36-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("forklift")
PT = dict(material="paint", role="body")
PTD = dict(material="paint", role="detail")
ST = dict(material="steel", role="body")
STD = dict(material="steel", role="detail")
IN = dict(material="interior", role="detail")
HY = dict(material="hydraulic", role="detail")
CI = dict(material="cast-iron", role="detail")
RB = dict(material="rubber", role="body")
WD = dict(material="wood", role="body")
GD = dict(material="goods", role="body")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]
def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def capsule2d(p0, r0, p1, r1, n=8):
    a = math.atan2(p1[1] - p0[1], p1[0] - p0[0])
    pts = []
    for i in range(n + 1):
        b = a - math.pi / 2 + math.pi * i / n
        pts.append((p1[0] + r1 * math.cos(b), p1[1] + r1 * math.sin(b)))
    for i in range(n + 1):
        b = a + math.pi / 2 + math.pi * i / n
        pts.append((p0[0] + r0 * math.cos(b), p0[1] + r0 * math.sin(b)))
    return pts


FR = 0.291
fa0 = math.degrees(math.asin((0.18 - 0.29) / FR))
fa1 = math.degrees(math.asin((0.5 - 0.29) / FR))
front_right = arc(0.0, 0.29, FR, fa0, fa1, 16)
front_left = [(-x, y) for x, y in reversed(front_right)]
RR = 0.231
ra0 = math.degrees(math.asin((0.18 - 0.23) / RR))
rear_right = arc(-1.7, 0.23, RR, ra0, 90, 16)
rear_left = arc(-1.7, 0.23, RR, 180 - ra0, 90, 16)
CHASSIS = [(-1.7, 0.5), (-1.7, 0.58), (-0.02, 0.58), (0.05, 0.96), (0.35, 0.92), (0.43, 0.86), (0.45, 0.78), (0.45, 0.28), (0.4, 0.19)] + front_right + front_left + rear_right[:-1]
CW = [(-1.7, 1.2), (-2.3, 1.2), (-2.4, 1.15), (-2.45, 1.0), (-2.45, 0.35), (-2.4, 0.22), (-2.3, 0.18), (-1.95, 0.18)] + [p for p in rear_left if p[0] >= -1.95]
if os.environ.get("NOARCH"):
    CHASSIS = [(-1.7, 0.18), (0.3, 0.18), (0.4, 0.2), (0.45, 0.28), (0.45, 0.78), (0.43, 0.86), (0.35, 0.92), (0.05, 0.96), (-0.02, 0.58), (-1.7, 0.58)]
    CW = [(-1.7, 1.2), (-2.3, 1.2), (-2.4, 1.15), (-2.45, 1.0), (-2.45, 0.35), (-2.4, 0.22), (-2.3, 0.18), (-1.7, 0.18)]
EPS = 1.03
WHEELS = [("wheel-f", 0.0, 0.29, 0.1, 0.17, 0.055, 0.5, 0.6, "f"), ("wheel-r", -1.7, 0.23, 0.08, 0.13, 0.04, 0.24, 0.32, "r")]


def chassis_parts(cowl=True, drive=True):
    V.ext("chassis-body", "chassis", "side", CHASSIS, -0.5, 0.5, tone=0.55, region="chassis-paint", **PT)
    if drive:
        V.box("drive-cover", "chassis", [-0.05, 0.63, 0], [0.5, 0.1, 0.8], cr=0.01, tone=0.3, region="drive-motor", material="drive-electronics", role="detail")
    if cowl:
        V.ext("cowl-dash", "chassis", "side", [(0.05, 0.96), (-0.02, 0.58), (-0.18, 0.6), (-0.1, 1.0)], -0.3, 0.3, tone=0.25, region="controller", material="drive-electronics", role="detail")


def seat(wheel_n):
    if wheel_n:
        c = (-0.22, 1.35, 0.0)
        n = (-0.94, 0.34, 0.0)
        t = 2 * math.pi / wheel_n
        m = (0.34, 0.94, 0.0)
        V.cap("steering-wheel", "chassis", [c[0], c[1], c[2] + 0.17], [c[0] + 0.17 * math.sin(t) * m[0], c[1] + 0.17 * math.sin(t) * m[1], 0.17 * math.cos(t)], 0.012 if wheel_n > 12 else 0.0155, tone=0.2, rep=dict(count=wheel_n, angle=t, axis=list(n), origin=list(c)), **IN)
        V.cyl("steering-web", "chassis", list(c), [c[0] + 0.0155 * n[0], c[1] + 0.0155 * n[1], 0.0], 0.168, tone=0.2, **IN)
        V.box("seat-cushion", "chassis", [-0.7, 1.1, 0], [0.5, 0.1, 0.5], cr=0.03, tone=0.22, **IN)
        V.ext("seat-back", "chassis", "side", capsule2d((-0.92, 1.1), 0.07, (-1.0, 1.6), 0.06), -0.22, 0.22, tone=0.22, **IN)


def guard(post_r, slats):
    for nm, x0, x1 in [("front", -0.1, -0.05), ("rear", -1.0, -1.0)]:
        V.cap(f"guard-post-{nm}", "chassis", [x0, 0.96 if nm == "front" else 0.58, 0.5], [x1, 2.1, 0.5], post_r, tone=0.25, region="guard", rep=dict(count=2, step=[0, 0, -1.0]), **PTD)
    V.box("guard-rail-side", "chassis", [-0.525, 2.125, 0.525], [1.35, 0.05, 0.05], tone=0.3, region="guard", rep=dict(count=2, step=[0, 0, -1.05]), **PT)
    V.box("guard-rail-end", "chassis", [0.125, 2.125, 0], [0.05, 0.05, 1.0], tone=0.3, region="guard", rep=dict(count=2, step=[-1.3, 0, 0]), **PT)
    if slats:
        V.box("guard-slat", "chassis", [-1.1, 2.125, 0], [0.03, 0.04, 1.0], tone=0.2, region="guard", rep=dict(count=6, step=[0.2, 0, 0]), **PTD)


def hyd(trunnion, r_barrel, r_rod):
    for z in (0.26, -0.26):
        s = "left" if z > 0 else "right"
        V.cap(f"tilt-barrel-{s}", "tilt-cyl", V.W("tilt-cyl", [0, 0, z]), V.W("tilt-cyl", [0.45, 0, z]), r_barrel, tone=0.35, region="hydraulics", **HY)
        V.cap(f"tilt-rod-{s}", "tilt-rod", V.W("tilt-rod", [0, 0, z]), V.W("tilt-rod", [0.45, 0, z]), r_rod, tone=0.8, region="hydraulics", **HY)


def mast(pivot, rail_w):
    V.box("mast-outer-rail", "mast-outer", [0.05, 1.075, 0.41], [0.14, 2.05, 0.14], tone=0.35, region="mast", rep=dict(count=2, step=[0, 0, -0.82]), **ST)
    if pivot:
        V.cyl("mast-pivot", "mast-outer", [0.05, 0.3, -0.48], [0.05, 0.3, 0.48], 0.05, tone=0.15, **STD)
    V.box("mast-inner-rail", "mast-inner", [0.17, 0.9, 0.25], [0.1, 1.6, 0.14], tone=0.45, region="mast", rep=dict(count=2, step=[0, 0, -0.5]), **ST)
    V.box("carriage-plate", "carriage", [0.29, 0.37, 0], [0.1, 0.6, 0.84], tone=0.5, region="fork-gear", **ST)


def forks(bars, rail):
    V.box("fork-heel", "forks", [0.375, 0.2, 0.4], [0.09, 0.26, 0.12], tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -0.8]), **ST)
    V.box("fork-tine", "forks", [0.91, 0.0925, 0.4], [0.98, 0.045, 0.12], tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -0.8]), **ST)
    for nm, z in bars:
        V.box(f"fork-backrest-bar-{nm}", "forks", [0.375, 0.595, z], [0.09, 1.05, 0.05], tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -2 * z]), **ST)
    V.box("fork-backrest-top", "forks", [0.375, 1.095, 0], [0.09, 0.05, 1.1], tone=0.3, region="fork-gear", **ST)
    if rail:
        V.box("fork-backrest-rail", "forks", [0.375, 0.59, 0], [0.09, 0.04, 1.1], tone=0.3, region="fork-gear", **STD)


def pallet(runner, straps):
    V.box("pallet-runner", "pallet", [1.45, 0.065, 0], [0.1, 0.1, 0.8], tone=0.5, region="pallet", rep=dict(count=3, step=[0.55, 0, 0]), **WD)
    V.box("pallet-deck", "pallet", [2.0, 0.137, 0], [1.2, 0.044 if runner == 1 else 0.06, 0.8], tone=0.62, region="pallet", **WD)
    V.box("pallet-goods", "pallet", [2.0, 0.609, 0], [1.16, 0.9, 0.76], cr=0.02, tone=0.78, region="cargo", **GD)
    if straps:
        V.ext("goods-strap-around", "pallet", "front", [(0.155, -0.384), (1.063, -0.384), (1.063, 0.384), (0.155, 0.384)], 1.79, 1.81, holes=[[(0.16, -0.376), (1.058, -0.376), (1.058, 0.376), (0.16, 0.376)]], tone=0.3, rep=dict(count=2, step=[0.4, 0, 0]), region="cargo", material="goods", role="detail")
        V.ext("goods-strap-long", "pallet", "side", rect(1.416, 2.584, 0.155, 1.063), 0.12, 0.14, holes=[rect(1.424, 2.576, 0.16, 1.058)], tone=0.3, rep=dict(count=2, step=[0, 0, -0.26]), region="cargo", material="goods", role="detail")


def wheels(detail, thick_scale=1.0):
    for bone, cx, r, hw, rim, hub, ztyre, zaxle, tag in WHEELS:
        cy = r
        for side, sg in [("left", 1), ("right", -1)]:
            if detail >= 1:
                prof = [[sg * a * hw / 0.1, rr] for a, rr in [(-0.1, 0), (-0.1, r * 0.8), (-0.085, r * 0.93), (-0.05, r), (0.05, r), (0.085, r * 0.93), (0.1, r * 0.8), (0.1, 0)]]
            else:
                prof = [[sg * a * hw / 0.1, rr] for a, rr in [(-0.1, 0), (-0.1, r * 0.9), (-0.06, r), (0.06, r), (0.1, r * 0.9), (0.1, 0)]]
            V.rev(f"tyre-{tag}-{side}", bone, [cx, cy, sg * ztyre], [0, 0, 1], prof, tone=0.1, region="tyres", **RB)
            zo = sg * (ztyre + hw + 0.002)
            if detail >= 1:
                V.cyl(f"rim-{tag}-{side}", bone, [cx, cy, sg * ztyre], [cx, cy, zo], rim, tone=0.6, region="brakes", material="paint", role="detail")
                if detail >= 2:
                    V.cyl(f"rim-bolt-{tag}-{side}", bone, [cx + rim * 0.6, cy, zo - sg * 0.004], [cx + rim * 0.6, cy, zo + sg * 0.004], 0.015, tone=0.35, region="brakes", rep=dict(count=6, angle=2 * math.pi / 6, axis=[0, 0, 1], origin=[cx, cy, 0]), **CI)
                V.cyl(f"hub-{tag}-{side}", bone, [cx, cy, zo - sg * (0.012 if detail >= 2 else 0.0155)], [cx, cy, zo + sg * 0.004], hub, tone=0.45, region="brakes", material="paint", role="detail")
            else:
                V.cyl(f"hub-{tag}-{side}", bone, [cx, cy, zo - sg * 0.031], [cx, cy, zo], hub + 0.02, tone=0.45, region="brakes", material="paint", role="detail")


def axles():
    for bone, cx, r, hw, rim, hub, ztyre, zaxle, tag in WHEELS:
        V.cyl(f"axle-{tag}", "axle-" + tag, [cx, r, -zaxle], [cx, r, zaxle], 0.04 if tag == "r" else 0.05, tone=0.3, material="steel", role="detail")


def thermal_emitter(f):
    V.to("thermal")
    t = max(f, 0.0075) * EPS
    V.cyl("th-motor-vent", "counterweight", [-2.455, 1.05, 0], [-2.455 + t, 1.05, 0], 0.125, tone=0.2, region="drive-motor", tags=["motor-vent"], material="cast-iron", role="detail")


def structure(scale, f, drop=()):
    V.to("structure")
    ids = {s["id"]: s for s in V.lists["body"]}
    skip = {"steering-column", "steering-wheel", "seat-cushion", "seat-back", "beacon", "guard-slat", "drive-cover", "cowl-dash", "fork-backrest-rail", "goods-strap-around", "goods-strap-long", "pallet-goods", "rim-bolt", "hub", "mast-pivot"}
    for sid, s in ids.items():
        if sid in skip or any(sid.startswith(k) for k in ("hub-", "rim-bolt", "rim-", "tilt-trunnion", "tilt-bracket")):
            continue
        if sid in drop:
            continue
        V.member(sid, s["bone"])
    for bone in ["axle-f", "axle-r"]:
        pass
    V.cyl("member-axle-f", "axle-f", [0, 0.29, -0.5], [0, 0.29, 0.5], 0.05 if scale < 200 else 0.0155 + f * 0.6, tone=0.3, member="axle-f", part="member", material="steel", role="detail")
    V.cyl("member-axle-r", "axle-r", [-1.7, 0.23, -0.24], [-1.7, 0.23, 0.24], 0.04 if scale < 200 else 0.0155 + f * 0.6, tone=0.3, member="axle-r", part="member", material="steel", role="detail")
    # joints
    V.cyl("joint-mast-pivot", "mast-outer", [0.05, 0.3, -0.3], [0.05, 0.3, 0.3], max(0.05, f * 0.7), tone=0.15, member="mast-outer", part="joint", **STD)
    V.cyl("joint-tilt-cyl", "tilt-cyl", [-0.56, 0.74, -0.26], [-0.56, 0.74, 0.26], max(0.03, f * 0.55), tone=0.3, member="tilt-cyl", part="joint", **HY)
    V.cyl("joint-tilt-rod", "tilt-rod", [-0.135, 1.2, -0.26], [-0.135, 1.2, 0.26], max(0.02, f * 0.55), tone=0.3, member="tilt-rod", part="joint", **HY)
    V.box("joint-carriage", "carriage", [0.29, 0.37, 0], [0.12, max(0.2, f * 3), 0.7], tone=0.4, member="carriage", part="joint", **ST)
    V.box("joint-forks", "forks", [0.375, 0.2, 0], [0.11, 0.2, 0.7], tone=0.3, member="forks", part="joint", **ST)
    V.box("joint-mast-inner", "mast-inner", [0.17, 0.4, 0], [0.12, 0.2, 0.4], tone=0.4, member="mast-inner", part="joint", **ST)
    V.box("joint-battery", "battery", [-1.2, 0.6, 0], [0.9, max(f * 1.3, 0.02), 0.8], tone=0.4, member="battery", part="joint", material="battery-case", role="detail")
    V.box("joint-counterweight", "counterweight", [-1.9, 0.5, 0], [0.15, 0.2, 0.9], tone=0.3, member="counterweight", part="joint", material="cast-iron", role="detail")
    V.box("joint-pallet", "pallet", [1.45, 0.12, 0], [max(0.2, f * 3), 0.1, 0.4], tone=0.5, member="pallet", part="joint", **WD)
    for tag, bone, x, z in [("f-l", "axle-f", 0.0, 0.5), ("f-r", "axle-f", 0.0, -0.5), ("r-l", "axle-r", -1.7, 0.24), ("r-r", "axle-r", -1.7, -0.24)]:
        V.box(f"support-contact-{tag}", bone, [x, f / 2, z], [0.16, f, 0.1 if scale < 200 else 0.1], tone=0.1, member=bone, part="support", anchor=f"contact-{tag}", **RB)
    V.box("member-pallet-x", "pallet-x", [2.0, 0.065, 0.3], [1.1, max(0.03, f * 1.3), max(0.1, f * 1.3)], tone=0.5, member="pallet-x", part="member", **WD)
    V.box("member-pallet-y", "pallet-y", [2.0, 0.065, -0.3], [max(0.1, f * 1.3), max(0.1, f * 1.3), max(0.1, f * 1.3)], tone=0.5, member="pallet-y", part="member", **WD)
    V.ell("mass-chassis", "chassis", [-0.9, 0.38, 0], [0.5, 0.1, 0.25], tone=0.2, member="chassis", part="mass", **PT)
    V.ell("mass-battery", "battery", [-1.2, 0.815, 0], [0.4, 0.15, 0.3], tone=0.2, member="battery", part="mass", material="battery-case", role="body")
    V.ell("mass-counterweight", "counterweight", [-2.05, 0.7, 0], [0.18, 0.3, 0.35], tone=0.2, member="counterweight", part="mass", material="cast-iron", role="body")


def s50():
    chassis_parts()
    V.cap("steering-column", "chassis", [-0.02, 0.98, 0], [-0.2, 1.3, 0], 0.0075, tone=0.2, slender=True, **IN)
    seat(16)
    guard(0.03, True)
    V.ell("beacon", "chassis", [-0.5, 2.2, 0], [0.06, 0.06, 0.06], tone=0.85, **IN)
    V.box("battery-case", "battery", [-1.2, 0.815, 0], [1.0, 0.47, 0.9], cr=0.02, tone=0.4, region="battery-pack", material="battery-case", role="body")
    V.ext("counterweight-body", "counterweight", "side", CW, -0.55, 0.55, tone=0.3, region="counterweight", material="cast-iron", role="body")
    V.box("counterweight-vent", "counterweight", [-2.445, 1.0, 0], [0.01, 0.2, 0.4], tone=0.2, material="cast-iron", role="detail")
    hyd(1, 0.04, 0.025)
    mast(True, 0.14)
    forks([("outer", 0.525), ("inner", 0.225)], True)
    pallet(1, True)
    wheels(2)
    V.auto_thermal()
    thermal_emitter(0.0075)
    structure(50, 0.0077)
    V.decide("The chassis is one side-profile extrusion (z +-0.5) with the wheel arches cut through it along the tyre circles (1 mm clearance, because the drawing shows the chassis solid behind the tyre) and the battery well behind the cowl, the counterweight a second extrusion with its own rear arch; seat, 16-segment steering wheel on its column, guard with four posts, rails and six roof slats, a beacon, battery case, hydraulic tilt barrels and rods with trunnions and brackets, two outer and two inner mast rails with pivot, carriage plate, forks with heels, tines, two backrest bars on each side, top bar and rail; the pallet with runners, deck, goods block and four straps, and eight bolted tyres with rims and hubs.",
             "Das Fahrgestell ist eine Seitenprofil-Extrusion (z +-0,5) mit entlang der Reifenkreise durchgeschnittenen Radkästen (1 mm Spiel, weil die Zeichnung das Fahrgestell hinter dem Reifen voll zeigt) und der Batteriewanne hinter der Haube, das Gegengewicht eine zweite Extrusion mit eigenem Hinterradkasten; Sitz, Lenkrad mit 16 Segmenten auf der Säule, Schutzdach mit vier Pfosten, Holmen und sechs Dachlatten, ein Rundumlicht, Batteriekasten, hydraulische Kippzylinder mit Stangen, Zapfen und Konsolen, zwei äußere und zwei innere Mastschienen mit Drehpunkt, Hubschlitten, Gabeln mit Fersen, Zinken, je zwei Rückenstäben, Oberholm und Schiene; die Palette mit Kufen, Deck, Ladeblock und vier Gurten sowie vier Reifenpaare mit Schrauben, Felgen und Naben.")
    V.decide("Thermal: every region is a thin skin or has no layer, so thermal solids reference body solids (chassis, guard, battery, counterweight, drive cover for the drive motor, cowl for the controller, hydraulics, mast, fork gear, tyres, rims and bolts for the brakes, pallet and cargo); the motor vent is a 250 mm disc on the back face of the counterweight tagged on the drive-motor region. areaM2 stays the committed value wherever an earlier correction exceeded a factor of 1.5 (chassis-paint, battery-pack, cargo, drive-motor, mast); the areas this model exposes are listed in area-review.md.",
             "Thermik: Jede Region ist eine dünne Haut oder hat keine Schicht, daher verweisen die Thermikkörper auf Körper (Fahrgestell, Schutzdach, Batterie, Gegengewicht, Antriebsabdeckung für den Fahrmotor, Haube für die Steuerung, Hydraulik, Mast, Gabelgeräte, Reifen, Felgen und Schrauben für die Bremsen, Palette und Ladung); der Motorauslass ist eine 250-mm-Scheibe an der Rückseite des Gegengewichts an der Region drive-motor. areaM2 bleibt der festgeschriebene Wert, wo eine frühere Korrektur den Faktor 1,5 überschritt (chassis-paint, battery-pack, cargo, drive-motor, mast); die von diesem Modell freigelegten Flächen stehen in area-review.md.")
    V.decide("Structure: chassis plate, counterweight, battery, guard, mast rails, carriage, fork parts, tyres, pallet runners and deck are members, axles are bars through the wheel centres inside the track, joints are the mast pivot, tilt cylinder pins, carriage, fork and pallet seats, four 160 mm pads under the tyres at the declared contact anchors and three masses (chassis, battery, counterweight).",
             "Tragwerk: Fahrgestellplatte, Gegengewicht, Batterie, Schutzdach, Mastschienen, Hubschlitten, Gabelteile, Reifen, Palettenkufen und -deck sind Stäbe, Achsen sind Stäbe durch die Radmitten innerhalb der Spur, Gelenke sind Mastdrehpunkt, Kippzylinderbolzen, Hubschlitten-, Gabel- und Palettensitze, vier 160-mm-Auflager unter den Reifen an den angegebenen Aufstandsankern und drei Massen (Fahrgestell, Batterie, Gegengewicht).")
    V.finish(50)


def s100():
    chassis_parts()
    seat(8)
    guard(0.03, False)
    V.box("guard-slat", "chassis", [-1.1, 2.125, 0], [0.04, 0.05, 1.0], tone=0.2, region="guard", rep=dict(count=4, step=[0.3, 0, 0]), **PTD)
    V.ell("beacon", "chassis", [-0.5, 2.2, 0], [0.06, 0.06, 0.06], tone=0.85, **IN)
    V.box("battery-case", "battery", [-1.2, 0.815, 0], [1.0, 0.47, 0.9], cr=0.02, tone=0.4, region="battery-pack", material="battery-case", role="body")
    V.ext("counterweight-body", "counterweight", "side", CW, -0.55, 0.55, tone=0.3, region="counterweight", material="cast-iron", role="body")
    hyd(2, 0.04, 0.025)
    mast(False, 0.14)
    forks([("outer", 0.525)], False)
    pallet(2, False)
    wheels(1)
    V.auto_thermal()
    thermal_emitter(0.0155)
    structure(100, 0.0155, drop=("pallet-runner",))
    V.decide("At 1:100 the chassis and counterweight extrusions stay, the steering wheel is an eight-segment ring, steering column, guard slats, counterweight vent, mast pivot, inner backrest bars and rail, bolts and goods straps are dropped, the hydraulic trunnions and brackets thicken to 15.5 mm, tyres are plain revolves with a rim disc and 31 mm hubs, and the pallet deck is 60 mm.",
             "Bei 1:100 bleiben die Extrusionen von Fahrgestell und Gegengewicht, das Lenkrad ist ein Ring aus acht Segmenten, Lenksäule, Dachlatten, Gegengewichtsöffnung, Mastdrehpunkt, innere Rückenstäbe und Schiene, Schrauben und Ladegurte entfallen, die hydraulischen Zapfen und Konsolen werden 15,5 mm dick, die Reifen sind schlichte Drehkörper mit Felgenscheibe und 31-mm-Naben, und das Palettendeck ist 60 mm dick.")
    V.decide("Thermal: references as at 1:50 for the remaining parts, the motor vent is a 15.5 mm disc.",
             "Thermik: Verweise wie bei 1:50 für die verbleibenden Teile, der Motorauslass ist eine 15,5 mm dicke Scheibe.")
    V.decide("Structure: the same load path as at 1:50 without the pallet runners as separate members (the deck carries them), joints and pads sized to 15.5 mm.",
             "Tragwerk: derselbe Lastpfad wie bei 1:50 ohne die Palettenkufen als eigene Stäbe (das Deck trägt sie), Gelenke und Auflager auf 15,5 mm bemessen.")
    V.finish(100)


def s200():
    chassis_parts()
    guard(0.03, False)
    V.ell("beacon", "chassis", [-0.5, 2.2, 0], [0.06, 0.06, 0.06], tone=0.85, **IN)
    V.box("battery-case", "battery", [-1.2, 0.815, 0], [1.0, 0.47, 0.9], tone=0.4, region="battery-pack", material="battery-case", role="body")
    V.ext("counterweight-body", "counterweight", "side", CW, -0.55, 0.55, tone=0.3, region="counterweight", material="cast-iron", role="body")
    hyd(0, 0.04, 0.0155)
    mast(False, 0.14)
    forks([("outer", 0.525)], False)
    V.box("pallet-deck", "pallet", [2.0, 0.145, 0], [1.2, 0.06, 0.8], tone=0.62, region="pallet", **WD)
    V.box("pallet-feet", "pallet", [2.0, 0.065, 0], [1.2, 0.1, 0.8], tone=0.5, region="pallet", **WD)
    V.box("pallet-goods", "pallet", [2.0, 0.609, 0], [1.16, 0.9, 0.76], cr=0.02, tone=0.78, region="cargo", **GD)
    wheels(0)
    V.auto_thermal()
    thermal_emitter(0.031)
    structure(200, 0.031, drop=("pallet-runner", "pallet-feet"))
    V.decide("At 1:200 steering wheel, steering column, rails' small parts, trunnions, rim discs of the brakes (they remain only as 31 mm hubs), pivot and straps are gone and the seat vanishes (the 1:200 drawing has none); the pallet is a 100 mm foot block, a 60 mm deck and the goods; tyres are plain revolves; the guard keeps four 31 mm posts and two rails.",
             "Bei 1:200 entfallen Sitz (die 1:200-Zeichnung hat keinen), Lenkrad, Lenksäule, kleine Holmteile, Zapfen, die Felgenscheiben der Bremsen (sie bleiben nur als 31-mm-Naben), Drehpunkt und Gurte; die Palette ist ein 100 mm hoher Fußblock mit 60 mm dickem Deck und der Ladung; die Reifen sind schlichte Drehkörper; das Schutzdach behält vier 31-mm-Pfosten und zwei Holme.")
    V.decide("Thermal: references as before; the rim region is carried by the hubs.",
             "Thermik: Verweise wie zuvor; die Bremsregion tragen die Naben.")
    V.decide("Structure: as at 1:100 with 31 mm pads and joints, no separate pallet block member.",
             "Tragwerk: wie bei 1:100 mit 31-mm-Auflagern und -Gelenken, kein eigener Palettenfußstab.")
    V.finish(200)


def s500():
    V.ext("chassis-body", "chassis", "side", CHASSIS, -0.5, 0.5, tone=0.55, region="chassis-paint", **PT)
    for nm, x0, x1 in [("front", -0.1, -0.05), ("rear", -1.0, -1.0)]:
        V.cap(f"guard-post-{nm}", "chassis", [x0, 0.96 if nm == "front" else 0.58, 0.5], [x1, 2.1, 0.5], 0.0378, tone=0.25, region="guard", rep=dict(count=2, step=[0, 0, -1.0]), **PTD)
    V.box("guard-roof", "chassis", [-0.525, 2.1, 0], [1.35, 0.0755, 1.1], tone=0.3, region="guard", **PT)
    V.ell("beacon", "chassis", [-0.5, 2.2, 0], [0.06, 0.06, 0.06], tone=0.85, **IN)
    V.box("battery-case", "battery", [-1.2, 0.815, 0], [1.0, 0.47, 0.9], tone=0.4, region="battery-pack", material="battery-case", role="body")
    V.ext("counterweight-body", "counterweight", "side", CW, -0.55, 0.55, tone=0.3, region="counterweight", material="cast-iron", role="body")
    V.box("mast-outer-rail", "mast-outer", [0.05, 1.075, 0.41], [0.14, 2.05, 0.14], tone=0.35, region="mast", rep=dict(count=2, step=[0, 0, -0.82]), **ST)
    V.box("mast-inner-rail", "mast-inner", [0.17, 0.9, 0.25], [0.1, 1.6, 0.14], tone=0.45, region="mast", rep=dict(count=2, step=[0, 0, -0.5]), **ST)
    for z in (0.26, -0.26):
        sd = "left" if z > 0 else "right"
        V.cap(f"tilt-barrel-{sd}", "tilt-cyl", V.W("tilt-cyl", [0, 0, z]), V.W("tilt-cyl", [0.45, 0, z]), 0.0378, tone=0.35, region="hydraulics", **HY)
        V.cap(f"tilt-rod-{sd}", "tilt-rod", V.W("tilt-rod", [0, 0, z]), V.W("tilt-rod", [0.45, 0, z]), 0.0378, tone=0.8, region="hydraulics", **HY)
    V.box("carriage-plate", "carriage", [0.29, 0.37, 0], [0.1, 0.6, 0.84], tone=0.5, region="fork-gear", **ST)
    V.box("fork-heel", "forks", [0.375, 0.2, 0.4], [0.09, 0.26, 0.12], tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -0.8]), **ST)
    V.box("fork-tine", "forks", [0.91, 0.0925, 0.4], [0.98, 0.0755, 0.12], tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -0.8]), **ST)
    V.box("fork-backrest-top", "forks", [0.375, 1.075, 0], [0.09, 0.0755, 1.1], tone=0.3, region="fork-gear", **ST)
    V.box("pallet-block", "pallet", [2.0, 0.087, 0], [1.2, 0.144, 0.8], tone=0.55, region="pallet", **WD)
    V.box("pallet-goods", "pallet", [2.0, 0.609, 0], [1.16, 0.9, 0.76], tone=0.78, region="cargo", **GD)
    for bone, cx, r, hw, rim, hub, ztyre, zaxle, tag in WHEELS:
        for side, sg in [("left", 1), ("right", -1)]:
            V.cyl(f"tyre-{tag}-{side}", bone, [cx, r, sg * (ztyre - 0.1)], [cx, r, sg * (ztyre + 0.1)], r, tone=0.1, region="tyres", **RB)
    V.auto_thermal()
    V.to("thermal")
    V.box("th-drive-cover", "chassis", [-0.05, 0.63, 0], [0.5, 0.0755, 0.8], tone=0.3, region="drive-motor", material="drive-electronics", role="detail")
    V.box("th-controller", "chassis", [-0.1, 0.8, 0], [0.16, 0.38, 0.6], tone=0.25, region="controller", material="drive-electronics", role="detail")
    V.cyl("th-brake", "wheel-f", [0, 0.29, 0.5245], [0, 0.29, 0.6], 0.17, tone=0.6, region="brakes", rep=dict(count=2, step=[0, 0, -1.1245]), material="paint", role="detail")
    t = 0.0755 * EPS
    V.cyl("th-motor-vent", "counterweight", [-2.455, 1.05, 0], [-2.455 + t, 1.05, 0], 0.125, tone=0.2, region="drive-motor", tags=["motor-vent"], material="cast-iron", role="detail")
    V.to("structure")
    for sid in ["chassis-body", "battery-case", "counterweight-body", "mast-outer-rail", "mast-inner-rail", "carriage-plate", "fork-heel", "fork-tine", "fork-backrest-top", "pallet-block", "guard-post-front", "guard-post-rear", "guard-roof"]:
        V.member(sid, {s["id"]: s["bone"] for s in V.lists["body"]}[sid])
    for k in ("f", "r"):
        for side in ("left", "right"):
            V.member(f"tyre-{k}-{side}", f"wheel-{k}")
    f = 0.0755
    V.cyl("member-axle-f", "axle-f", [0, 0.29, -0.5], [0, 0.29, 0.5], 0.0378, tone=0.3, member="axle-f", part="member", material="steel", role="detail")
    V.cyl("member-axle-r", "axle-r", [-1.7, 0.23, -0.24], [-1.7, 0.23, 0.24], 0.0378, tone=0.3, member="axle-r", part="member", material="steel", role="detail")
    V.member("tilt-barrel-left", "tilt-cyl")
    V.member("tilt-rod-left", "tilt-rod")
    V.member("tilt-barrel-right", "tilt-cyl")
    V.member("tilt-rod-right", "tilt-rod")
    V.cyl("joint-mast-pivot", "mast-outer", [0.05, 0.3, -0.3], [0.05, 0.3, 0.3], 0.0755, tone=0.15, member="mast-outer", part="joint", **STD)
    V.box("joint-carriage", "carriage", [0.29, 0.37, 0], [0.12, 0.3, 0.7], tone=0.4, member="carriage", part="joint", **ST)
    V.box("joint-forks", "forks", [0.375, 0.2, 0], [0.11, 0.2, 0.7], tone=0.3, member="forks", part="joint", **ST)
    for tag, bone, x, z in [("f-l", "axle-f", 0.0, 0.5), ("f-r", "axle-f", 0.0, -0.5), ("r-l", "axle-r", -1.7, 0.24), ("r-r", "axle-r", -1.7, -0.24)]:
        V.box(f"support-contact-{tag}", bone, [x, f / 2, z], [0.16, f, 0.1], tone=0.1, member=bone, part="support", anchor=f"contact-{tag}", **RB)
    V.box("member-pallet-x", "pallet-x", [2.0, 0.065, 0.3], [1.1, 0.0755, 0.1], tone=0.5, member="pallet-x", part="member", **WD)
    V.box("member-pallet-y", "pallet-y", [2.0, 0.065, -0.3], [0.1, 0.1, 0.1], tone=0.5, member="pallet-y", part="member", **WD)
    V.ell("mass-chassis", "chassis", [-0.9, 0.38, 0], [0.5, 0.1, 0.25], tone=0.2, member="chassis", part="mass", **PT)
    V.decide("At 1:500 the truck is the chassis extrusion, the counterweight extrusion, a battery block, the two tilt cylinders as 75 mm bars, a 75 mm guard roof slab on four posts of 75 mm, four mast rails, the carriage plate, two fork heels and tines, a backrest bar, one pallet block with the goods and plain tyre cylinders; seat, steering, hydraulics, brakes, hubs, rims, rails' details, slats and straps are gone.",
             "Bei 1:500 besteht der Stapler aus der Fahrgestell-Extrusion, der Gegengewichts-Extrusion, einem Batterieblock, den zwei Kippzylindern als 75-mm-Stäbe, einer 75 mm dicken Dachplatte auf vier 75-mm-Pfosten, vier Mastschienen, der Hubschlittenplatte, zwei Gabelfersen und -zinken, einem Rückenholm, einem Palettenblock mit der Ladung und schlichten Reifenzylindern; Sitz, Lenkung, Hydraulik, Bremsen, Naben, Felgen, Holmdetails, Latten und Gurte entfallen.")
    V.decide("Thermal: drive motor, controller, hydraulic cylinder and brakes have no surface in the merged masses and are real solids at their places (cover plate, controller block, two brake discs); the other regions reference the masses; the vent is a 75 mm disc.",
             "Thermik: Fahrmotor, Steuerung, Hydraulikzylinder und Bremsen haben in den verschmolzenen Massen keine Oberfläche und sind echte Körper an ihren Orten (Abdeckplatte, Steuerungsblock, zwei Bremsscheiben); die übrigen Regionen verweisen auf die Massen; der Auslass ist eine 75 mm dicke Scheibe.")
    V.decide("Structure: the masses, rails, carriage, forks, pallet block, tyres, posts and the guard slab are members, axles and tilt parts are 75 mm bars, joints and supports sized to 75 mm, one mass in the chassis.",
             "Tragwerk: die Massen, Schienen, Hubschlitten, Gabeln, Palettenblock, Reifen, Pfosten und die Dachplatte sind Stäbe, Achsen und Kippteile sind 75-mm-Stäbe, Gelenke und Auflager auf 75 mm bemessen, eine Masse im Fahrgestell.")
    V.finish(500)


s50(); s100(); s200(); s500()
V.write([50, 100, 200, 500])
