"""🚘 The 1:500 lists of the round 3 city-car, authored for 75 mm features: hull with both arches, glass mass, roof, rear quarter pillars, bonnet, tailgate, one-piece wheels and the two door mirrors; thermal with cabin, engine, exhaust and brake discs as hidden solids; structure as two longitudinals, three crossmembers and the B-pillar hoop."""
from r3_city_car_kit import QX, ZW, box, capsule, ellipsoid, hull_stations, loft, member, plan_loft, ref, revolve, roof_loft, sec, side_plate

SIDES = (("l", 1), ("r", -1))
WHEELS = (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1))
PAINT = dict(material="paint", role="body", tone=0.62)
STATIONS = [-1.95, -1.905, -1.78, -1.665, -1.558, -1.3, -1.042, -0.935, -0.927, 0.777, 0.785, 0.892, 1.15, 1.408, 1.515, 1.7, 1.864, 1.95]


def body():
    out = [hull_stations("hull", STATIONS, **PAINT)]
    out.append(loft("underbody", [sec([x, 0.5, 0], 0.6, 0.2, 6, QX) for x in (-1.85, 1.85)], material="plastic-trim", role="body", tone=0.1))
    out.append(plan_loft("glasshouse", [(0.96, -1.79, 0.985, 0.785), (1.41, -1.515, 0.325, 0.616)], 7, material="glass", role="glazing", tone=0.85))
    out.append(roof_loft("roof", [(0.335, 1.4, 1.44, 0.4, 5), (0.15, 1.4, 1.48, 0.622, 5), (-0.6, 1.4, 1.5, 0.622, 4.5), (-1.525, 1.4, 1.445, 0.59, 6)], **PAINT))
    for side, s in SIDES:
        out.append(side_plate(f"c-pillar-{side}", s, [(0.99, -1.77, -1.28), (1.42, -1.51, -1.16)], 0.04, -0.034, 4, **PAINT))
        out.append(ellipsoid(f"mirror-{side}", [0.07, 0.07, 0.115], [0.76, 1.08, s * 0.835], material="plastic-trim", role="detail", tone=0.3))
    out.append(loft("bonnet-skin", [sec([0.0, -0.02, 0], 0.705, 0.038, 2.5), sec([0.915, -0.02, 0], 0.63, 0.038, 2.5)], bone="bonnet", **PAINT))
    out.append(loft("hatch-panel", [sec([0.0, 0.03, 0], 0.55, 0.038, 6), sec([0.47, 0.03, 0], 0.625, 0.038, 6), sec([0.745, 0.03, 0], 0.655, 0.038, 6)], bone="hatch", **PAINT))
    for tag, bone, s in WHEELS:
        out.append(revolve(f"{tag}-wheel", [[-0.0925, 0.0], [-0.0925, 0.27], [-0.07, 0.3], [0.07, 0.3], [0.0925, 0.27], [0.0925, 0.0]], [0, 0, s], [0, 0, s * ZW], bone=bone, material="rubber", role="body", tone=0.1))
    return out


def thermal():
    out = [ref("th-hull", "hull", "body-panels", faces=["+v"]), ref("th-underbody", "underbody", "body-panels"), ref("th-c-pillar-l", "c-pillar-l", "body-panels", faces=["-w"]), ref("th-c-pillar-r", "c-pillar-r", "body-panels", faces=["+w"])]
    out.append(ref("th-roof", "roof", "roof", faces=["-v"]))
    out.append(ref("th-bonnet", "bonnet-skin", "bonnet", faces=["-v"]))
    out.append(ref("th-hatch-panel", "hatch-panel", "tailgate", faces=["+v"]))
    out.append(ref("th-glasshouse", "glasshouse", "glazing", faces=["-v", "+v"]))
    out.append(box("th-cabin", [2.3, 0.6, 1.2], [-0.4, 0.75, 0], radius=0.1, material="interior-trim", role="detail", tone=0.3, region="interior"))
    out.append(box("th-engine", [0.35, 0.4, 0.75], [1.35, 0.55, 0], radius=0.075, material="engine-metal", role="detail", tone=0.2, region="engine-bay"))
    out.append(capsule("th-exhaust", [0.95, 0.3, -0.12], [-1.86, 0.32, -0.3], 0.04, material="exhaust-steel", role="detail", tone=0.35, region="exhaust-system"))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.04], [0.075, 0.04], [0.075, 0.0]], [-1, 0, 0], [-1.87, 0.32, -0.3], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", tags=["tailpipe"]))
    for tag, bone, s in WHEELS:
        out.append(ref(f"th-{tag}-wheel", f"{tag}-wheel", "tyres"))
        out.append(revolve(f"th-{tag}-disc", [[-0.19, 0.0], [-0.19, 0.15], [-0.115, 0.15], [-0.115, 0.0]], [0, 0, s], [0, 0, s * ZW], bone=bone, material="cast-iron", role="detail", tone=0.42, region="brakes"))
    return out


def structure():
    frame = dict(material="cast-iron", role="detail", tone=0.5)
    out = []
    m = lambda s: out.append(member(s, "chassis"))
    for side, s in SIDES:
        m(loft(f"longitudinal-{side}", [sec([1.86, 0.47, s * 0.45], 0.06, 0.05, 4), sec([0.9, 0.47, s * 0.45], 0.06, 0.05, 4), sec([0.6, 0.3, s * 0.6], 0.06, 0.05, 4), sec([-0.85, 0.3, s * 0.6], 0.06, 0.05, 4), sec([-1.05, 0.52, s * 0.5], 0.06, 0.05, 4), sec([-1.86, 0.52, s * 0.5], 0.06, 0.05, 4)], **frame))
        m(capsule(f"b-pillar-{side}", [-0.3, 0.3, s * 0.7], [-0.3, 1.41, s * 0.58], 0.045, **frame))
    for name, x, y, w in (("bumper-beam-front", 1.86, 0.47, 1.0), ("heel-kick", -0.86, 0.32, 1.3), ("bumper-beam-rear", -1.86, 0.52, 1.1)):
        m(box(name, [0.1, 0.1, w], [x, y, 0], radius=0.02, **frame))
    m(capsule("roof-bow", [-0.3, 1.42, -0.58], [-0.3, 1.42, 0.58], 0.045, **frame))
    out.append(member(ellipsoid("com-chassis", [0.15, 0.15, 0.15], [0.28, 0.55, 0], **frame), "chassis", "mass"))
    for side, s in SIDES:
        out.append(member(box(f"bonnet-frame-{side}", [0.86, 0.076, 0.08], [0.46, -0.02, s * 0.58], radius=0.02, bone="bonnet", **frame), "bonnet"))
        out.append(member(capsule(f"hatch-frame-{side}", [0.04, 0.03, s * 0.5], [0.7, 0.03, s * 0.57], 0.038, bone="hatch", **frame), "hatch"))
    out.append(member(capsule("axle-beam-front", [0, 0, -0.62], [0, 0, 0.62], 0.04, bone="axle-f", **frame), "axle-f"))
    out.append(member(capsule("axle-beam-rear", [0, 0, -0.62], [0, 0, 0.62], 0.04, bone="axle-r", **frame), "axle-r"))
    for tag, bone, s in WHEELS:
        out.append({"id": f"st-{tag}-wheel", "of": f"{tag}-wheel", "member": bone, "part": "member"})
    return out


def decisions():
    return [
        {"en": "At 1:500 the smallest feature is 75 mm (0.15 mm on paper): the car is its hull with the two wheel arches, the glass mass, the roof, the two wide rear quarter pillars, bonnet and tailgate, four one-piece wheels and the two door mirrors that give the overall width of 1.90 m; lamps, the narrow A and B pillars, the exhaust and all interior parts are below that size and left out.", "de": "Im Maßstab 1:500 ist das kleinste Merkmal 75 mm (0,15 mm auf dem Papier): Das Auto besteht aus dem Rumpf mit beiden Radhäusern, der Glasmasse, dem Dach, den zwei breiten hinteren Seitenwandsäulen, Haube und Heckklappe, vier einteiligen Rädern und den beiden Außenspiegeln, die die Gesamtbreite von 1,90 m ergeben; Leuchten, die schmalen A- und B-Säulen, die Abgasanlage und alle Innenteile liegen unter dieser Größe und entfallen."},
        {"en": "The thermal model keeps every region: the cabin, the engine, the exhaust run with its outlet patch at the tailpipe and the four brake discs are hidden solids inside the body, the wheels stand for the tyres.", "de": "Das thermische Modell behält jede Region: Kabine, Motor, Abgasstrang mit seiner Austrittsfläche am Endrohr und die vier Bremsscheiben sind verdeckte Körper im Aufbau, die Räder stehen für die Reifen."},
        {"en": "The structure is the load path reduced to two longitudinals with their kick-ups over the axles, three crossmembers, the B-pillar hoop with its roof bow, the two side rails each of bonnet and tailgate, the two axle beams and the wheels; the model is minimal at this scale.", "de": "Das Tragwerk ist der Lastpfad, reduziert auf zwei Längsträger mit ihren Kröpfungen über den Achsen, drei Querträger, den B-Säulen-Ring mit Dachspriegel, je zwei Seitenholme von Haube und Heckklappe, die beiden Achsträger und die Räder; das Modell ist in diesem Maßstab minimal."},
    ]
