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


def ext_local(id, bone, plane, pts, lo, hi, **m):
    d = V._meta(id, bone, "extrude", m)
    d["profile"] = [lib.rv(p) for p in pts]
    d["plane"] = plane
    d["from"], d["to"] = lib.r5(lo), lib.r5(hi)
    return V.put(d, {})


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
CHASSIS_SOLID = [(-1.7, 0.18), (0.3, 0.18), (0.4, 0.2), (0.45, 0.28), (0.45, 0.78), (0.43, 0.86), (0.35, 0.92), (0.05, 0.96), (-0.02, 0.58), (-1.7, 0.58)]
CW_SOLID = [(-1.7, 1.2), (-2.3, 1.2), (-2.4, 1.15), (-2.45, 1.0), (-2.45, 0.35), (-2.4, 0.22), (-2.3, 0.18), (-1.7, 0.18)]
SEAT = [(-0.99, 1.05), (-0.45, 1.05), (-0.45, 1.15), (-0.86, 1.15), (-0.93, 1.64), (-1.0, 1.66), (-1.06, 1.62), (-0.99, 1.15)]
WHEELS = [("wheel-f", 0.0, 0.29, 0.1, 0.17, 0.5, 0.6, "f"), ("wheel-r", -1.7, 0.23, 0.08, 0.13, 0.24, 0.32, "r")]
EPS = 1.03
BODY_BONE = {}


def bones():
    return {s["id"]: s["bone"] for s in V.lists["body"]}


def hyd(r_barrel, r_rod):
    for z in (0.26, -0.26):
        s = "left" if z > 0 else "right"
        V.cap(f"tilt-barrel-{s}", "tilt-cyl", V.W("tilt-cyl", [0, 0, z]), V.W("tilt-cyl", [0.45, 0, z]), r_barrel, tone=0.35, region="hydraulics", **HY)
        V.cap(f"tilt-rod-{s}", "tilt-rod", V.W("tilt-rod", [0, 0, z]), V.W("tilt-rod", [0.45, 0, z]), r_rod, tone=0.8, region="hydraulics", **HY)


def mast_rails():
    V.box("mast-outer-rail", "mast-outer", [0.05, 1.075, 0.41], [0.14, 2.05, 0.14], tone=0.35, region="mast", rep=dict(count=2, step=[0, 0, -0.82]), **ST)
    V.box("mast-inner-rail", "mast-inner", [0.17, 0.9, 0.25], [0.1, 1.6, 0.14], tone=0.45, region="mast", rep=dict(count=2, step=[0, 0, -0.5]), **ST)


def fork_pair(bone_dims=None, thick=0.045):
    V.ext("fork", "forks", "side", [(0.33, 0.07), (1.4, 0.07), (1.4, 0.07 + thick), (0.42, 0.07 + thick), (0.42, 0.33), (0.33, 0.33)], 0.34, 0.46, tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -0.8]), **ST)


def backrest_frame(top=1.12, bar=0.05, bottom=0.07):
    h = top - bottom
    pts = [(-0.55, 0), (-0.55, h), (0.55, h), (0.55, 0), (0.55 - bar, 0), (0.55 - bar, h - bar), (-0.55 + bar, h - bar), (-0.55 + bar, 0)]
    ext_local("fork-backrest", "forks", "zx", pts, -0.09, 0.0, tone=0.3, region="fork-gear", **ST)


def pallet_comb(deck_top=0.175):
    pts = [(1.4, 0.015), (1.5, 0.015), (1.5, 0.115), (1.95, 0.115), (1.95, 0.015), (2.05, 0.015), (2.05, 0.115), (2.5, 0.115), (2.5, 0.015), (2.6, 0.015), (2.6, deck_top), (1.4, deck_top)]
    V.ext("pallet-body", "pallet", "side", pts, -0.4, 0.4, tone=0.55, region="pallet", **WD)


def thermal_emitter(f):
    V.to("thermal")
    t = max(f, 0.0075) * EPS
    V.cyl("th-motor-vent", "counterweight", [-2.455, 1.05, 0], [-2.455 + t, 1.05, 0], 0.125, tone=0.2, region="drive-motor", tags=["motor-vent"], material="cast-iron", role="detail")


def joints_supports(f, wide):
    for tag, bone, x, z in [("f-l", "axle-f", 0.0, 0.5), ("f-r", "axle-f", 0.0, -0.5), ("r-l", "axle-r", -1.7, 0.24), ("r-r", "axle-r", -1.7, -0.24)]:
        V.box(f"support-contact-{tag}", bone, [x, f / 2, z], [0.16, f, 0.1], tone=0.1, member=bone, part="support", anchor=f"contact-{tag}", **RB)


def axle_bars(r):
    V.cyl("member-axle-f", "axle-f", [0, 0.29, -0.5], [0, 0.29, 0.5], r, tone=0.3, member="axle-f", part="member", material="steel", role="detail")
    V.cyl("member-axle-r", "axle-r", [-1.7, 0.23, -0.24], [-1.7, 0.23, 0.24], r, tone=0.3, member="axle-r", part="member", material="steel", role="detail")


def s100():
    f = 0.0155
    V.ext("chassis-body", "chassis", "side", CHASSIS, -0.5, 0.5, tone=0.55, region="chassis-paint", **PT)
    V.box("drive-cover", "chassis", [-0.05, 0.63, 0], [0.5, 0.1, 0.8], cr=0.01, tone=0.3, region="drive-motor", material="drive-electronics", role="detail")
    V.ext("cowl-dash", "chassis", "side", [(0.05, 0.96), (-0.02, 0.58), (-0.18, 0.6), (-0.1, 1.0)], -0.3, 0.3, tone=0.25, region="controller", material="drive-electronics", role="detail")
    V.ext("seat", "chassis", "side", SEAT, -0.22, 0.22, tone=0.22, **IN)
    c, n = (-0.22, 1.35, 0.0), (-0.94, 0.34, 0.0)
    t = 2 * math.pi / 8
    m = (0.34, 0.94, 0.0)
    V.cap("steering-wheel", "chassis", [c[0], c[1], c[2] + 0.17], [c[0] + 0.17 * math.sin(t) * m[0], c[1] + 0.17 * math.sin(t) * m[1], 0.17 * math.cos(t)], 0.0155, tone=0.2, rep=dict(count=8, angle=t, axis=list(n), origin=list(c)), **IN)
    V.cap("steering-column", "chassis", [-0.02, 0.98, 0], [-0.2, 1.3, 0], 0.0075, tone=0.2, slender=True, **IN)
    for nm, x0, x1 in [("front", -0.1, -0.05), ("rear", -1.0, -1.0)]:
        V.cap(f"guard-post-{nm}", "chassis", [x0, 0.96 if nm == "front" else 0.58, 0.5], [x1, 2.1, 0.5], 0.03, tone=0.25, region="guard", rep=dict(count=2, step=[0, 0, -1.0]), **PTD)
    xs = [-1.15, -1.025, -0.975, -0.775, -0.725, -0.525, -0.475, -0.275, -0.225, -0.025, 0.025, 0.10]
    holes = [rect(xs[i], xs[i + 1], -0.5, 0.5) for i in range(0, len(xs), 2)]
    V.ext("guard-roof", "chassis", "plan", rect(-1.2, 0.15, -0.55, 0.55), 2.1, 2.16, holes=holes, tone=0.3, region="guard", **PT)
    V.ell("beacon", "chassis", [-0.5, 2.2, 0], [0.06, 0.06, 0.06], tone=0.85, **IN)
    V.box("battery-case", "battery", [-1.2, 0.815, 0], [1.0, 0.47, 0.9], cr=0.02, tone=0.4, region="battery-pack", material="battery-case", role="body")
    V.ext("counterweight-body", "counterweight", "side", CW, -0.55, 0.55, tone=0.3, region="counterweight", material="cast-iron", role="body")
    hyd(0.04, 0.025)
    mast_rails()
    V.box("carriage-plate", "carriage", [0.29, 0.37, 0], [0.1, 0.6, 0.84], tone=0.5, region="fork-gear", **ST)
    fork_pair()
    backrest_frame()
    pallet_comb(0.175)
    V.box("pallet-goods", "pallet", [2.0, 0.617, 0], [1.16, 0.884, 0.76], cr=0.02, tone=0.78, region="cargo", **GD)
    for bone, cx, r, hw, rim, ztyre, zaxle, tag in WHEELS:
        for side, sg in [("left", 1), ("right", -1)]:
            prof = [[sg * a * hw / 0.1, rr] for a, rr in [(-0.1, 0), (-0.1, r * 0.8), (-0.085, r * 0.93), (-0.05, r), (0.05, r), (0.085, r * 0.93), (0.1, r * 0.8), (0.1, 0)]]
            V.rev(f"tyre-{tag}-{side}", bone, [cx, r, sg * ztyre], [0, 0, 1], prof, tone=0.1, region="tyres", **RB)
            V.cyl(f"brake-drum-{tag}-{side}", bone, [cx, r, sg * ztyre], [cx, r, sg * (ztyre + hw + 0.002)], rim, tone=0.6, region="brakes", material="paint", role="detail")
    for bone, cx, r, hw, rim, ztyre, zaxle, tag in WHEELS:
        V.cyl(f"axle-{tag}", "axle-" + tag, [cx, r, -zaxle], [cx, r, zaxle], 0.04 if tag == "r" else 0.05, tone=0.3, material="steel", role="detail")
    V.auto_thermal()
    thermal_emitter(f)
    V.to("structure")
    for sid in ["chassis-body", "battery-case", "counterweight-body", "guard-post-front", "guard-post-rear", "guard-roof", "mast-outer-rail", "mast-inner-rail", "carriage-plate", "fork", "fork-backrest", "pallet-body", "tilt-barrel-left", "tilt-barrel-right", "tilt-rod-left", "tilt-rod-right"]:
        V.member(sid, bones()[sid])
    for k in ("f", "r"):
        for side in ("left", "right"):
            V.member(f"tyre-{k}-{side}", f"wheel-{k}")
    axle_bars(0.05)
    V.cyl("joint-mast-pivot", "mast-outer", [0.05, 0.3, -0.3], [0.05, 0.3, 0.3], 0.05, tone=0.15, member="mast-outer", part="joint", **STD)
    V.cyl("joint-tilt-cyl", "tilt-cyl", [-0.56, 0.74, -0.26], [-0.56, 0.74, 0.26], 0.03, tone=0.3, member="tilt-cyl", part="joint", **HY)
    V.cyl("joint-tilt-rod", "tilt-rod", [-0.135, 1.2, -0.26], [-0.135, 1.2, 0.26], 0.02, tone=0.3, member="tilt-rod", part="joint", **HY)
    V.box("joint-carriage", "carriage", [0.29, 0.37, 0], [0.12, 0.2, 0.7], tone=0.4, member="carriage", part="joint", **ST)
    V.box("joint-forks", "forks", [0.375, 0.2, 0], [0.11, 0.2, 0.7], tone=0.3, member="forks", part="joint", **ST)
    V.box("joint-mast-inner", "mast-inner", [0.17, 0.4, 0], [0.12, 0.2, 0.4], tone=0.4, member="mast-inner", part="joint", **ST)
    V.box("joint-pallet", "pallet", [1.45, 0.12, 0], [0.2, 0.1, 0.4], tone=0.5, member="pallet", part="joint", **WD)
    joints_supports(f, False)
    V.box("member-pallet-x", "pallet-x", [2.0, 0.065, 0.3], [1.1, 0.0455, 0.1], tone=0.5, member="pallet-x", part="member", **WD)
    V.box("member-pallet-y", "pallet-y", [2.0, 0.065, -0.3], [0.1, 0.1, 0.1], tone=0.5, member="pallet-y", part="member", **WD)
    V.ell("mass-chassis", "chassis", [-0.9, 0.38, 0], [0.5, 0.1, 0.25], tone=0.2, member="chassis", part="mass", **PT)
    V.ell("mass-battery", "battery", [-1.2, 0.815, 0], [0.4, 0.15, 0.3], tone=0.2, member="battery", part="mass", material="battery-case", role="body")
    V.ell("mass-counterweight", "counterweight", [-2.05, 0.7, 0], [0.18, 0.3, 0.35], tone=0.2, member="counterweight", part="mass", material="cast-iron", role="body")
    V.decide("1:100 is modelled from its own drawings. The chassis and counterweight keep their side-profile extrusions with the arches cut along the tyre circles (the drawing shows both big masses). What the 1:50 list built from many small bodies is merged: the seat cushion and back are one extruded L, each fork is one extruded L (heel and tine, 15 mm walls), the fork backrest is one extruded U frame instead of four bars, the overhead guard roof is one plan extrusion with six openings between 50 mm slats instead of rails and slats, the pallet is one comb extrusion (deck and three runners). Steering wheel stays an eight-segment ring, the column a slender 15 mm tube (the drawing strokes it), the rim and hub of each wheel are one brake drum cylinder, bolts, straps, mast pivot, hydraulic brackets and trunnions are not modelled.",
             "1:100 ist aus den eigenen Zeichnungen modelliert. Fahrgestell und Gegengewicht behalten ihre Seitenprofil-Extrusionen mit den entlang der Reifenkreise geschnittenen Radkästen (die Zeichnung zeigt beide großen Massen). Was die 1:50-Liste aus vielen kleinen Körpern baut, ist zusammengefasst: Sitzkissen und Lehne sind ein extrudiertes L, jede Gabel ein extrudiertes L (Fersen und Zinke, 15-mm-Wände), die Gabelrückwand ein extrudierter U-Rahmen statt vier Stäben, das Schutzdach eine Draufsicht-Extrusion mit sechs Öffnungen zwischen 50-mm-Latten statt Holmen und Latten, die Palette eine Kammextrusion (Deck und drei Kufen). Das Lenkrad bleibt ein Ring aus acht Segmenten, die Säule ein schlankes 15-mm-Rohr (die Zeichnung zieht sie als Strich), Felge und Nabe jedes Rades sind ein Bremstrommelzylinder, Schrauben, Gurte, Mastdrehpunkt, Hydraulikkonsolen und -zapfen sind nicht modelliert.")
    V.decide("Thermal: thermal solids reference the body solids that carry each region (chassis, roof grate and posts for the guard, battery case, counterweight, drive cover for the drive motor, cowl for the controller, tilt barrels and rods, mast rails, forks, backrest and carriage, tyres, brake drums, pallet comb, goods); the motor vent is a 15.5 mm disc on the back face of the counterweight. The chassis-paint exposure (5.13 m2) stays below the committed 8 m2 estimate, which the area review keeps.",
             "Thermik: Die Thermikkörper verweisen auf die Körper, die jede Region tragen (Fahrgestell, Dachrost und Pfosten für das Schutzdach, Batteriekasten, Gegengewicht, Antriebsabdeckung für den Fahrmotor, Haube für die Steuerung, Kippzylinder und -stangen, Mastschienen, Gabeln, Rückwand und Hubschlitten, Reifen, Bremstrommeln, Palettenkamm, Ladung); der Motorauslass ist eine 15,5 mm dicke Scheibe an der Rückseite des Gegengewichts. Die Exposition der Region Rahmenlack (5,13 m2) bleibt unter der festgeschriebenen Schätzung von 8 m2, die die Flächenprüfung beibehält.")
    V.decide("Structure: chassis plate, counterweight, battery, the guard posts and roof grate, mast rails, carriage, the fork and backrest members, tyres, pallet comb and tilt parts are members, two axle bars run through the wheel centres, joints sit at the mast pivot, tilt pins, carriage, fork, inner mast and pallet seat, four 160 mm pads under the tyre contact anchors, three masses (chassis, battery, counterweight).",
             "Tragwerk: Fahrgestellplatte, Gegengewicht, Batterie, Schutzdachpfosten und -rost, Mastschienen, Hubschlitten, Gabel- und Rückwandstäbe, Reifen, Palettenkamm und Kippteile sind Stäbe, zwei Achsstäbe laufen durch die Radmitten, Gelenke sitzen am Mastdrehpunkt, an den Kippzapfen, am Hubschlitten, an der Gabel, am inneren Mast und am Palettensitz, vier 160-mm-Auflager unter den Aufstandsankern der Reifen, drei Massen (Fahrgestell, Batterie, Gegengewicht).")
    V.finish(100)


def s200():
    f = 0.031
    V.ext("chassis-body", "chassis", "side", CHASSIS_SOLID, -0.5, 0.5, tone=0.55, region="chassis-paint", **PT)
    V.ext("dash-block", "chassis", "side", [(-0.3, 0.58), (-0.02, 0.58), (0.05, 0.96), (-0.1, 1.0), (-0.18, 0.68), (-0.3, 0.68)], -0.35, 0.35, tone=0.28, region="controller", material="drive-electronics", role="detail")
    V.box("drive-cover", "chassis", [-0.1, 0.63, 0], [0.4, 0.1, 0.8], tone=0.3, region="drive-motor", material="drive-electronics", role="detail")
    for nm, x0, x1 in [("front", -0.1, -0.05), ("rear", -1.0, -1.0)]:
        V.cap(f"guard-post-{nm}", "chassis", [x0, 0.96 if nm == "front" else 0.58, 0.5], [x1, 2.1, 0.5], 0.03, tone=0.25, region="guard", rep=dict(count=2, step=[0, 0, -1.0]), **PTD)
    V.box("guard-roof", "chassis", [-0.525, 2.13, 0], [1.35, 0.06, 1.1], tone=0.3, region="guard", **PT)
    V.ell("beacon", "chassis", [-0.5, 2.2, 0], [0.06, 0.06, 0.06], tone=0.85, **IN)
    V.box("battery-case", "battery", [-1.2, 0.815, 0], [1.0, 0.47, 0.9], tone=0.4, region="battery-pack", material="battery-case", role="body")
    V.ext("counterweight-body", "counterweight", "side", CW_SOLID, -0.55, 0.55, tone=0.3, region="counterweight", material="cast-iron", role="body")
    hyd(0.04, 0.025)
    mast_rails()
    V.box("carriage-plate", "carriage", [0.29, 0.37, 0], [0.1, 0.6, 0.84], tone=0.5, region="fork-gear", **ST)
    fork_pair(thick=0.045)
    backrest_frame(bar=0.06)
    pallet_comb(0.175)
    V.box("pallet-goods", "pallet", [2.0, 0.617, 0], [1.16, 0.884, 0.76], tone=0.78, region="cargo", **GD)
    for bone, cx, r, hw, rim, ztyre, zaxle, tag in WHEELS:
        for side, sg in [("left", 1), ("right", -1)]:
            prof = [[sg * a * hw / 0.1, rr] for a, rr in [(-0.1, 0), (-0.1, r * 0.9), (-0.06, r), (0.06, r), (0.1, r * 0.9), (0.1, 0)]]
            V.rev(f"tyre-{tag}-{side}", bone, [cx, r, sg * ztyre], [0, 0, 1], prof, tone=0.1, region="tyres", **RB)
            V.cyl(f"brake-drum-{tag}-{side}", bone, [cx, r, sg * (ztyre + hw - 0.04)], [cx, r, sg * (ztyre + hw + 0.002)], rim, tone=0.6, region="brakes", material="paint", role="detail")
    V.auto_thermal()
    thermal_emitter(f)
    V.to("structure")
    for sid in ["chassis-body", "battery-case", "counterweight-body", "guard-post-front", "guard-post-rear", "guard-roof", "mast-outer-rail", "mast-inner-rail", "carriage-plate", "fork", "fork-backrest", "pallet-body", "tilt-barrel-left", "tilt-barrel-right", "tilt-rod-left", "tilt-rod-right"]:
        V.member(sid, bones()[sid])
    for k in ("f", "r"):
        for side in ("left", "right"):
            V.member(f"tyre-{k}-{side}", f"wheel-{k}")
    axle_bars(0.0455)
    V.cyl("joint-mast-pivot", "mast-outer", [0.05, 0.3, -0.3], [0.05, 0.3, 0.3], 0.05, tone=0.15, member="mast-outer", part="joint", **STD)
    V.box("joint-carriage", "carriage", [0.29, 0.37, 0], [0.12, 0.2, 0.7], tone=0.4, member="carriage", part="joint", **ST)
    V.box("joint-forks", "forks", [0.375, 0.2, 0], [0.11, 0.2, 0.7], tone=0.3, member="forks", part="joint", **ST)
    V.box("joint-mast-inner", "mast-inner", [0.17, 0.4, 0], [0.12, 0.2, 0.4], tone=0.4, member="mast-inner", part="joint", **ST)
    joints_supports(f, False)
    V.box("member-pallet-x", "pallet-x", [2.0, 0.065, 0.3], [1.1, 0.062, 0.1], tone=0.5, member="pallet-x", part="member", **WD)
    V.box("member-pallet-y", "pallet-y", [2.0, 0.065, -0.3], [0.1, 0.1, 0.1], tone=0.5, member="pallet-y", part="member", **WD)
    V.ell("mass-chassis", "chassis", [-0.9, 0.38, 0], [0.5, 0.1, 0.25], tone=0.2, member="chassis", part="mass", **PT)
    V.ell("mass-battery", "battery", [-1.2, 0.815, 0], [0.4, 0.15, 0.3], tone=0.2, member="battery", part="mass", material="battery-case", role="body")
    V.ell("mass-counterweight", "counterweight", [-2.05, 0.7, 0], [0.18, 0.3, 0.35], tone=0.2, member="counterweight", part="mass", material="cast-iron", role="body")
    V.decide("1:200 is modelled from its own drawings: the chassis and the counterweight are plain side-profile blocks without wheel arches (the 1:200 drawing shows the chassis solid behind the tyre), the cowl and drive cover are one 60 mm dash extrusion plus a cover plate, the overhead guard is a 60 mm roof slab on four 60 mm posts as the drawing fills it, the pallet is one comb extrusion (deck 60 mm, three 100 mm runners as the 1:200 drawing gaps them), each fork is an extruded L and the backrest a 60 mm U frame. Seat, steering wheel, column, straps, bolts, hubs and the mast pivot do not exist at this scale; each tyre is a plain 62 mm revolve with a 40 mm brake drum on its outer face.",
             "1:200 ist aus den eigenen Zeichnungen modelliert: Fahrgestell und Gegengewicht sind schlichte Seitenprofilblöcke ohne Radkästen (die 1:200-Zeichnung zeigt das Fahrgestell voll hinter dem Reifen), Haube und Antriebsabdeckung sind eine 60-mm-Armaturen-Extrusion plus Abdeckplatte, das Schutzdach eine 60-mm-Dachplatte auf vier 60-mm-Pfosten, so wie die Zeichnung sie füllt, die Palette eine Kammextrusion (Deck 60 mm, drei 100-mm-Kufen, wie die 1:200-Zeichnung sie mit Lücken zeigt), jede Gabel ein extrudiertes L und die Rückwand ein 60-mm-U-Rahmen. Sitz, Lenkrad, Säule, Gurte, Schrauben, Naben und Mastdrehpunkt gibt es bei diesem Maßstab nicht; jeder Reifen ist ein schlichter Drehkörper mit einer 40-mm-Bremstrommel an der Außenseite.")
    V.decide("Thermal: references to the body solids that carry each region; the dash block carries the controller, the cover plate the drive motor, the brake drums the brakes, the roof slab and posts the guard; the motor vent is a 31 mm disc on the counterweight back. Only the total area is compared at this scale.",
             "Thermik: Verweise auf die Körper, die jede Region tragen; der Armaturenblock trägt die Steuerung, die Abdeckplatte den Fahrmotor, die Bremstrommeln die Bremsen, Dachplatte und Pfosten das Schutzdach; der Motorauslass ist eine 31-mm-Scheibe an der Gegengewichtsrückseite. Bei diesem Maßstab wird nur die Gesamtfläche verglichen.")
    V.decide("Structure: the same load path as at 1:100 (chassis, counterweight, battery, guard posts and roof, mast rails, carriage, fork, backrest, pallet comb, tyres, tilt parts, two axles), with joints at the mast pivot, carriage, forks and inner mast, four 31 mm pads and three masses.",
             "Tragwerk: derselbe Lastpfad wie bei 1:100 (Fahrgestell, Gegengewicht, Batterie, Schutzdachpfosten und -dach, Mastschienen, Hubschlitten, Gabel, Rückwand, Palettenkamm, Reifen, Kippteile, zwei Achsen) mit Gelenken am Mastdrehpunkt, Hubschlitten, an den Gabeln und am inneren Mast, vier 31-mm-Auflagern und drei Massen.")
    V.finish(200)


def s500():
    f = 0.0755
    V.ext("chassis-body", "chassis", "side", CHASSIS_SOLID, -0.5, 0.5, tone=0.55, region="chassis-paint", **PT)
    for nm, x0, x1 in [("front", -0.05, -0.05), ("rear", -1.0, -1.0)]:
        V.cap(f"guard-post-{nm}", "chassis", [x0, 0.96 if nm == "front" else 0.58, 0.5], [x1, 2.1, 0.5], 0.0378, tone=0.25, region="guard", rep=dict(count=2, step=[0, 0, -1.0]), **PTD)
    V.box("guard-roof", "chassis", [-0.525, 2.125, 0], [1.35, 0.0755, 1.04], tone=0.3, region="guard", **PT)
    V.ell("beacon", "chassis", [-0.5, 2.2, 0], [0.06, 0.06, 0.06], tone=0.85, **PT)
    V.box("battery-case", "battery", [-1.2, 0.815, 0], [1.0, 0.47, 0.9], tone=0.4, region="battery-pack", material="battery-case", role="body")
    V.ext("counterweight-body", "counterweight", "side", CW_SOLID, -0.55, 0.55, tone=0.3, region="counterweight", material="cast-iron", role="body")
    hyd(0.0378, 0.0378)
    mast_rails()
    V.box("carriage-plate", "carriage", [0.29, 0.37, 0], [0.1, 0.6, 0.84], tone=0.5, region="fork-gear", **ST)
    V.ext("fork", "forks", "side", [(0.33, 0.07), (1.4, 0.07), (1.4, 0.1455), (0.42, 0.1455), (0.42, 0.33), (0.33, 0.33)], 0.34, 0.46, tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -0.8]), **ST)
    ext_local("fork-backrest", "forks", "zx", [(-0.55, 0.98), (0.55, 0.98), (0.55, 1.05), (-0.55, 1.05)], -0.09, 0.0, tone=0.3, region="fork-gear", **ST)
    V.box("pallet-block", "pallet", [2.0, 0.087, 0], [1.2, 0.144, 0.8], tone=0.55, region="pallet", **WD)
    V.box("pallet-goods", "pallet", [2.0, 0.609, 0], [1.16, 0.9, 0.76], tone=0.78, region="cargo", **GD)
    for bone, cx, r, hw, rim, ztyre, zaxle, tag in WHEELS:
        for side, sg in [("left", 1), ("right", -1)]:
            V.cyl(f"tyre-{tag}-{side}", bone, [cx, r, sg * (ztyre - hw)], [cx, r, sg * (ztyre + hw)], r, tone=0.1, region="tyres", **RB)
    V.auto_thermal()
    V.to("thermal")
    V.box("th-drive-cover", "chassis", [-0.1, 0.63, 0], [0.4, 0.0755, 0.8], tone=0.3, region="drive-motor", material="drive-electronics", role="detail")
    V.box("th-controller", "chassis", [-0.1, 0.8, 0], [0.16, 0.38, 0.6], tone=0.25, region="controller", material="drive-electronics", role="detail")
    V.cyl("th-brake", "wheel-f", [0, 0.29, 0.5245], [0, 0.29, 0.6], 0.17, tone=0.6, region="brakes", rep=dict(count=2, step=[0, 0, -1.1245]), material="paint", role="detail")
    thermal_emitter(f)
    V.to("structure")
    for sid in ["chassis-body", "battery-case", "counterweight-body", "guard-post-front", "guard-post-rear", "guard-roof", "mast-outer-rail", "mast-inner-rail", "carriage-plate", "fork", "fork-backrest", "pallet-block"]:
        V.member(sid, bones()[sid])
    for k in ("f", "r"):
        for side in ("left", "right"):
            V.member(f"tyre-{k}-{side}", f"wheel-{k}")
    axle_bars(0.0378)
    for sid in ["tilt-barrel-left", "tilt-barrel-right", "tilt-rod-left", "tilt-rod-right"]:
        V.member(sid, bones()[sid])
    V.box("member-pallet-x", "pallet-x", [2.0, 0.065, 0.3], [1.1, 0.0755, 0.1], tone=0.5, member="pallet-x", part="member", **WD)
    V.box("member-pallet-y", "pallet-y", [2.0, 0.065, -0.3], [0.1, 0.1, 0.1], tone=0.5, member="pallet-y", part="member", **WD)
    V.cyl("joint-mast-pivot", "mast-outer", [0.05, 0.3, -0.3], [0.05, 0.3, 0.3], 0.0755, tone=0.15, member="mast-outer", part="joint", **STD)
    V.box("joint-carriage", "carriage", [0.29, 0.37, 0], [0.12, 0.3, 0.7], tone=0.4, member="carriage", part="joint", **ST)
    V.box("joint-forks", "forks", [0.375, 0.2, 0], [0.11, 0.2, 0.7], tone=0.3, member="forks", part="joint", **ST)
    joints_supports(f, True)
    V.ell("mass-chassis", "chassis", [-0.9, 0.38, 0], [0.5, 0.1, 0.25], tone=0.2, member="chassis", part="mass", **PT)
    V.decide("1:500 is modelled from its own drawings. The chassis and the counterweight are two solid side-profile blocks, the overhead guard is a 75 mm roof slab on four 75 mm posts (the drawing strokes the frame with a 150 mm pen), the mast is four rails, the carriage a plate, each fork an extruded L under a 75 mm backrest bar, the pallet one block and the goods one box, the tyres plain cylinders. The brakes, drive motor and controller are below what the 1:500 drawing draws, so they exist as real thermal solids at their places and not in the body; the two tilt cylinders stay 75 mm bars in the body because the structure needs them as members inside it (the elevation does not draw them: they show as a thin blue diagonal across the cab). The roof edge: the 1:500 elevation stroked the frame only from x = -1.0 m while the plan and every other scale draw the roof from -1.2 m; the elevation stroke was repaired at the root.",
             "1:500 ist aus den eigenen Zeichnungen modelliert. Fahrgestell und Gegengewicht sind zwei volle Seitenprofilblöcke, das Schutzdach eine 75-mm-Dachplatte auf vier 75-mm-Pfosten (die Zeichnung zieht den Rahmen mit einem 150-mm-Stift), der Mast besteht aus vier Schienen, der Hubschlitten aus einer Platte, jede Gabel aus einem extrudierten L unter einem 75-mm-Rückenholm, die Palette aus einem Block und die Ladung aus einer Box, die Reifen sind schlichte Zylinder. Bremsen, Fahrmotor und Steuerung liegen unter dem, was die 1:500-Zeichnung zeichnet, daher bestehen sie als echte Thermikkörper an ihren Orten und nicht im Körper; die zwei Kippzylinder bleiben 75-mm-Stäbe im Körper, weil das Tragwerk sie als Stäbe darin braucht (die Ansicht zeichnet sie nicht: sie erscheinen als dünne blaue Diagonale durch die Kabine). Die Dachkante: Die 1:500-Ansicht zog den Rahmen nur ab x = -1,0 m, während Grundriss und alle anderen Maßstäbe das Dach ab -1,2 m zeichnen; der Ansichtsstrich wurde an der Wurzel repariert.")
    V.decide("Thermal: regions of the merged masses reference them (chassis, guard, battery, counterweight, mast, fork gear, tyres, pallet, goods); drive motor, controller and brakes are real solids at their places (cover plate, controller block, two brake discs), the hydraulics region references the two tilt bars; the vent is a 75 mm disc.",
             "Thermik: Regionen der verschmolzenen Massen verweisen auf diese (Fahrgestell, Schutzdach, Batterie, Gegengewicht, Mast, Gabelgeräte, Reifen, Palette, Ladung); Fahrmotor, Steuerung und Bremsen sind echte Körper an ihren Orten (Abdeckplatte, Steuerungsblock, zwei Bremsscheiben), die Region Hydraulik verweist auf die zwei Kippstäbe; der Auslass ist eine 75-mm-Scheibe.")
    V.decide("Structure: chassis, counterweight, battery, posts, roof, mast rails, carriage, fork, backrest, pallet block and tyres are members, the axles are 75 mm bars, joints at the mast pivot, carriage and forks, four 75 mm pads and one chassis mass.",
             "Tragwerk: Fahrgestell, Gegengewicht, Batterie, Pfosten, Dach, Mastschienen, Hubschlitten, Gabel, Rückwand, Palettenblock und Reifen sind Stäbe, die Achsen 75-mm-Stäbe, Gelenke am Mastdrehpunkt, Hubschlitten und an den Gabeln, vier 75-mm-Auflager und eine Fahrgestellmasse.")
    V.finish(500)


sel = os.environ.get("SC", "100,200,500").split(",")
if "100" in sel: s100()
if "200" in sel: s200()
if "500" in sel: s500()
V.write([int(k) for k in sel])
