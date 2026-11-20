"""🚕 The 1:200 lists of the round 3 city-car, authored for 30 mm features: hull with arches cut as four-chord arcs, a glasshouse of one glass mass under the roof with plate pillars on it, bonnet, tailgate, two-part wheels, lamps and mirrors; thermal with the hidden cabin, engine and brake discs as real solids; structure as platform, pillars and roof frame."""
from r3_city_car_kit import QX, ZW, box, capsule, ellipsoid, hull_patch, hull_stations, loft, member, plan_loft, ref, revolve, roof_loft, sec, side_plate

SIDES = (("l", 1), ("r", -1))
WHEELS = (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1))
PAINT = dict(material="paint", role="body", tone=0.62)
GLASS = dict(material="glass", role="glazing", tone=0.85)
STATIONS = [-1.95, -1.905, -1.78, -1.665, -1.558, -1.3, -1.042, -0.935, -0.927, 0.777, 0.785, 0.892, 1.15, 1.408, 1.515, 1.7, 1.864, 1.9, 1.95]
ROOF = [(0.335, 1.405, 1.435, 0.25, 5), (0.26, 1.4, 1.458, 0.56, 5), (0.14, 1.4, 1.478, 0.622, 5), (-0.6, 1.4, 1.5, 0.622, 4.5), (-1.35, 1.4, 1.476, 0.612, 5), (-1.525, 1.405, 1.44, 0.575, 6)]
TYRE = [[-0.07, 0.3], [0.07, 0.3], [0.0925, 0.26], [0.08, 0.195], [-0.08, 0.195], [-0.0925, 0.26], [-0.07, 0.3]]


def body():
    out = [hull_stations("hull", STATIONS, **PAINT)]
    out.append(loft("underbody", [sec([x, (b + t) / 2, 0], h, (t - b) / 2, 6, QX) for x, b, t, h in [(-1.85, 0.34, 0.7, 0.55), (-0.92, 0.3, 0.72, 0.6), (-0.9, 0.245, 0.72, 0.6), (0.78, 0.245, 0.72, 0.6), (1.85, 0.29, 0.68, 0.55)]], material="plastic-trim", role="body", tone=0.1))
    out.append(plan_loft("glasshouse", [(0.96, -1.79, 0.985, 0.785), (1.0, -1.765, 0.925, 0.77), (1.41, -1.515, 0.325, 0.616)], 7, **GLASS))
    out.append(roof_loft("roof", ROOF, **PAINT))
    for side, s in SIDES:
        out.append(side_plate(f"a-pillar-{side}", s, [(0.985, 0.7, 0.85), (1.42, 0.15, 0.275)], 0.02, -0.012, 4, **PAINT))
        out.append(side_plate(f"b-pillar-{side}", s, [(0.99, -0.38, -0.22), (1.42, -0.38, -0.22)], 0.02, -0.012, 4, **PAINT))
        out.append(side_plate(f"c-pillar-{side}", s, [(0.99, -1.77, -1.28), (1.42, -1.51, -1.16)], 0.02, -0.012, 4, **PAINT))
        at, q = hull_patch(1.84, 0.7, s, 0.02)
        out.append(ellipsoid(f"headlamp-{side}", [0.13, 0.06, 0.035], at, rotation=q, material="glass", role="glazing", tone=0.9))
        out.append(box(f"tail-lamp-{side}", [0.05, 0.16, 0.1], [-1.912, 0.7, s * 0.65], radius=0.02, material="glass", role="detail", tone=0.7))
        out.append(ellipsoid(f"mirror-{side}", [0.06, 0.06, 0.11], [0.76, 1.08, s * 0.84], material="plastic-trim", role="detail", tone=0.3))
    out.append(loft("bonnet-skin", [sec([0.0, -0.004, 0], 0.705, 0.022, 2.5), sec([0.915, -0.004, 0], 0.63, 0.018, 2.5)], bone="bonnet", **PAINT))
    out.append(loft("hatch-panel", [sec([0.47, 0.022, 0], 0.625, 0.024, 6), sec([0.745, 0.022, 0], 0.655, 0.024, 6)], bone="hatch", **PAINT))
    out.append(loft("hatch-lip", [sec([0.735, 0.018, 0], 0.024, 0.6, 6), sec([0.911, 0.144, 0], 0.024, 0.6, 6)], bone="hatch", **PAINT))
    out.append(loft("hatch-glass", [sec([0.02, 0.012, 0], 0.54, 0.008, 8), sec([0.49, 0.012, 0], 0.61, 0.008, 8)], bone="hatch", **GLASS))
    out.append(capsule("silencer", [-1.66, 0.315, -0.44], [-1.66, 0.315, 0.0], 0.085, material="exhaust-steel", role="detail", tone=0.4))
    out.append(capsule("tail-pipe", [-1.7, 0.305, -0.3], [-1.92, 0.31, -0.3], 0.027, material="exhaust-steel", role="detail", tone=0.45))
    for tag, bone, s in WHEELS:
        out.append(revolve(f"{tag}-tyre", TYRE, [0, 0, s], [0, 0, s * ZW], bone=bone, material="rubber", role="body", tone=0.1))
        out.append(revolve(f"{tag}-rim", [[-0.08, 0.0], [-0.08, 0.2], [0.08, 0.2], [0.06, 0.16], [0.07, 0.06], [0.078, 0.0]], [0, 0, s], [0, 0, s * ZW], bone=bone, material="alloy", role="detail", tone=0.55))
    steel = dict(material="cast-iron", role="detail", tone=0.3)
    out.append(capsule("axle-beam-front", [0, -0.04, -0.63], [0, -0.04, 0.63], 0.03, bone="axle-f", **steel))
    out.append(capsule("axle-beam-rear", [0.2, 0.0, -0.6], [0.2, 0.0, 0.6], 0.035, bone="axle-r", **steel))
    return out


def thermal():
    out = [ref("th-hull", "hull", "body-panels", faces=["+v"]), ref("th-underbody", "underbody", "body-panels")]
    for side, face in (("l", "-w"), ("r", "+w")):
        for p in "abc":
            out.append(ref(f"th-{p}-pillar-{side}", f"{p}-pillar-{side}", "body-panels", faces=[face]))
        out.append(ref(f"th-headlamp-{side}", f"headlamp-{side}", "glazing"))
    out.append(ref("th-roof", "roof", "roof", faces=["-v"]))
    out.append(ref("th-bonnet", "bonnet-skin", "bonnet", faces=["-v"]))
    out.append(ref("th-hatch-panel", "hatch-panel", "tailgate", faces=["+v"]))
    out.append(ref("th-hatch-lip", "hatch-lip", "tailgate", faces=["+v"]))
    out.append(ref("th-glasshouse", "glasshouse", "glazing", faces=["-v", "+v"]))
    cabin = dict(material="interior-trim", role="detail", tone=0.3, region="interior")
    out.append(loft("th-cabin-floor", [sec([x, 0.6, 0], 0.66, 0.1, 5, QX) for x in (-1.6, 0.8)], **cabin))
    out.append(loft("th-front-seats", [sec([-0.17, 0.7, 0], 0.06, 0.6, 4), sec([-0.38, 1.3, 0], 0.05, 0.55, 4)], **cabin))
    out.append(loft("th-rear-seats", [sec([-0.98, 0.7, 0], 0.06, 0.63, 4), sec([-1.14, 1.2, 0], 0.05, 0.6, 4)], **cabin))
    engine = dict(material="engine-metal", role="detail", tone=0.2, region="engine-bay")
    out.append(box("th-engine", [0.32, 0.4, 0.75], [1.35, 0.55, -0.02], radius=0.05, **engine))
    out.append(capsule("th-exhaust-pipe", [0.95, 0.25, -0.1], [-1.5, 0.3, -0.16], 0.027, material="exhaust-steel", role="detail", tone=0.35, region="exhaust-system"))
    out.append(ref("th-silencer", "silencer", "exhaust-system"))
    out.append(ref("th-tail-pipe", "tail-pipe", "exhaust-system"))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.024], [0.03, 0.024], [0.03, 0.0]], [-1, 0, 0], [-1.925, 0.31, -0.3], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", tags=["tailpipe"]))
    for tag, bone, s in WHEELS:
        out.append(ref(f"th-{tag}-tyre", f"{tag}-tyre", "tyres"))
        out.append(revolve(f"th-{tag}-disc", [[-0.13, 0.0], [-0.13, 0.15], [-0.1, 0.15], [-0.1, 0.0]], [0, 0, s], [0, 0, s * ZW], bone=bone, material="cast-iron", role="detail", tone=0.42, region="brakes"))
    return out


def structure():
    frame = dict(material="cast-iron", role="detail", tone=0.5)
    out = []
    m = lambda s: out.append(member(s, "chassis"))
    for side, s in SIDES:
        m(box(f"sill-{side}", [1.72, 0.12, 0.1], [-0.075, 0.27, s * 0.74], radius=0.02, **frame))
        m(loft(f"front-rail-{side}", [sec([1.86, 0.47, s * 0.42], 0.05, 0.04, 4), sec([0.9, 0.47, s * 0.42], 0.05, 0.04, 4), sec([0.6, 0.29, s * 0.52], 0.04, 0.04, 4)], **frame))
        m(loft(f"rear-rail-{side}", [sec([-0.84, 0.28, s * 0.5], 0.04, 0.04, 4), sec([-1.05, 0.52, s * 0.5], 0.04, 0.04, 4), sec([-1.86, 0.52, s * 0.5], 0.04, 0.04, 4)], **frame))
        m(capsule(f"a-pillar-{side}", [0.78, 0.3, s * 0.72], [0.8, 0.97, s * 0.735], 0.04, **frame))
        m(capsule(f"screen-pillar-{side}", [0.8, 0.97, s * 0.735], [0.235, 1.42, s * 0.585], 0.035, **frame))
        m(capsule(f"b-pillar-{side}", [-0.3, 0.3, s * 0.745], [-0.3, 1.42, s * 0.6], 0.04, 0.035, **frame))
        m(capsule(f"c-pillar-{side}", [-1.0, 0.72, s * 0.74], [-1.3, 1.42, s * 0.6], 0.045, 0.035, **frame))
        m(capsule(f"roof-rail-{side}", [0.235, 1.425, s * 0.585], [-1.46, 1.425, s * 0.575], 0.035, **frame))
    for name, x, y, w, h in (("bumper-beam-front", 1.87, 0.47, 1.0, 0.1), ("heel-kick", -0.86, 0.3, 1.4, 0.1), ("bumper-beam-rear", -1.87, 0.52, 1.1, 0.1)):
        m(box(name, [0.08, h, w], [x, y, 0], radius=0.015, **frame))
    for name, x, y, half in (("windscreen-header", 0.235, 1.425, 0.585), ("rear-header", -1.46, 1.425, 0.575)):
        m(capsule(name, [x, y, -half], [x, y, half], 0.035, **frame))
    out.append(member(ellipsoid("com-chassis", [0.14, 0.14, 0.14], [0.28, 0.55, 0], **frame), "chassis", "mass"))
    for side, s in SIDES:
        out.append(member(box(f"bonnet-frame-{side}", [0.86, 0.032, 0.06], [0.46, -0.02, s * 0.6], radius=0.01, bone="bonnet", **frame), "bonnet"))
        out.append(member(capsule(f"hatch-frame-{side}", [0.03, 0.03, s * 0.52], [0.72, 0.03, s * 0.6], 0.022, bone="hatch", **frame), "hatch"))
    out.append(member(box("bonnet-frame-front", [0.06, 0.032, 1.2], [0.88, -0.02, 0], radius=0.01, bone="bonnet", **frame), "bonnet"))
    out.append(member(capsule("hatch-frame-bottom", [0.72, 0.03, -0.6], [0.72, 0.03, 0.6], 0.022, bone="hatch", **frame), "hatch"))
    out.append({"id": "st-axle-beam-front", "of": "axle-beam-front", "member": "axle-f", "part": "member"})
    out.append({"id": "st-axle-beam-rear", "of": "axle-beam-rear", "member": "axle-r", "part": "member"})
    for tag, bone, s in WHEELS:
        out.append({"id": f"st-{tag}-tyre", "of": f"{tag}-tyre", "member": bone, "part": "member"})
        out.append({"id": f"st-{tag}-rim", "of": f"{tag}-rim", "member": bone, "part": "member"})
    return out


def decisions():
    return [
        {"en": "At 1:200 the smallest feature is 30 mm: the hull keeps its two wheel arches as arcs of four chords, the sills are part of the hull section, and the whole glasshouse is one glass mass from the belt to the roof on which the A, B and C pillars lie as plates; seats, steering wheel, cowl, door joints and handles are not modelled.", "de": "Im Maßstab 1:200 ist das kleinste Merkmal 30 mm: Der Rumpf behält seine beiden Radhäuser als Bögen aus vier Sehnen, die Schweller gehören zum Rumpfquerschnitt, und das ganze Glashaus ist eine Glasmasse von der Brüstung bis zum Dach, auf der A-, B- und C-Säule als Platten liegen; Sitze, Lenkrad, Windlauf, Türfugen und Griffe sind nicht modelliert."},
        {"en": "Each wheel is a tyre and one dished wheel disc, the exhaust is the silencer with its tail pipe; in the thermal model the cabin floor and seat rows, the engine, the exhaust pipe under the floor and the four brake discs are real hidden solids and the tailpipe mouth is the outlet patch.", "de": "Jedes Rad besteht aus Reifen und einer geschüsselten Radscheibe, die Abgasanlage aus Schalldämpfer und Endrohr; im thermischen Modell sind Kabinenboden und Sitzreihen, Motor, Abgasrohr unter dem Boden und die vier Bremsscheiben echte verdeckte Körper, die Endrohrmündung ist die Austrittsfläche."},
        {"en": "The structure is the platform (sills, front and rear rails, three crossmembers), the pillar pairs with the roof rails and two headers, the edge frames of bonnet and tailgate, the two axle beams and the wheels.", "de": "Das Tragwerk ist die Plattform (Schweller, vordere und hintere Längsträger, drei Querträger), die Säulenpaare mit Dachholmen und zwei Querholmen, die Randrahmen von Haube und Heckklappe, die beiden Achsträger und die Räder."},
    ]
