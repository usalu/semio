"""🚐 The 1:500 lists of the round 3 delivery van (smallest feature 75 mm): the van as a hollow box of 80 mm panels that still opens: side panels with arches of 6 chords, upper panels, roof, front end, pillars, glass, sliding door, rear leaves, floor and one-piece wheels."""
import math

from r3_delivery_van_kit import RA, SLIDER, W, XF, XR, X_B, X_HINGE, X_QUARTER, YA, Y_BELT, Y_CANT, Y_FLOOR, Y_SILL, ZW, Z_LEAF, arc, arch_points, arch_stations, box, capsule, ellipsoid, extrude, loft, member, nose_stations, pillar_x, plate, ref, revolve, roof, sec, side_z, wall, windscreen

SIDES = (("l", 1), ("r", -1))
PAINT = dict(material="paint", role="body", tone=0.65)
DOOR = dict(material="paint", role="body", tone=0.6)
GLASS = dict(material="glass", role="glazing", tone=0.88)
T5 = 0.08
ARCH = 6
ROOF = [(-2.64, 1.945, 0.045, 0.90), (-2.49, 1.90, 0.09, 0.905), (0.60, 1.90, 0.09, 0.90), (1.12, 1.85, 0.045, 0.85)]
STATIONS = [1.22] + arch_stations(ARCH) + [2.40, 2.60, 2.67]
WHEEL = [[-0.1175, 0.0], [-0.1175, 0.33], [-0.09, 0.356], [0.09, 0.356], [0.1175, 0.33], [0.1175, 0.0]]


def quarter_outline(x_front):
    return [[-2.625, 0.36], [-2.12, Y_SILL]] + arch_points(XR, ARCH) + [[x_front, Y_SILL], [x_front, Y_BELT], [-2.625, Y_BELT]]


def body():
    out = [wall("side-l", 1, quarter_outline(X_HINGE), thickness=T5, **PAINT)]
    out.append(plate("side-upper-l", 1, [(1.16, -2.625, 0.37), (1.945, -2.625, 0.37)], thickness=T5, **PAINT))
    out.append(wall("quarter-r", -1, quarter_outline(X_QUARTER), thickness=T5, **PAINT))
    out.append(plate("quarter-upper-r", -1, [(1.16, -2.625, X_QUARTER), (1.945, -2.625, X_QUARTER)], thickness=T5, **PAINT))
    out.append(wall("cab-side-r", -1, [[X_B, Y_SILL], [X_HINGE, Y_SILL], [X_HINGE, Y_BELT], [X_B, Y_BELT]], thickness=T5, **PAINT))
    out.append(plate("b-post-upper-r", -1, [(1.16, X_B, 0.37), (Y_CANT, X_B, 0.37)], thickness=T5, **PAINT))
    for side, s in SIDES:
        out.append(plate(f"a-pillar-{side}", s, [(y, pillar_x(y) - 0.06, pillar_x(y) + 0.06) for y in (1.15, Y_CANT)], thickness=0.09, n=6, **PAINT))
        out.append(plate(f"door-glass-{side}", s, [(1.165, 0.365, 1.535), (1.89, 0.365, 0.90)], thickness=0.005, sunk=0.02, n=10, **GLASS))
        out.append(ellipsoid(f"mirror-{side}", [0.06, 0.135, 0.095], [1.415, 1.385, s * 1.04], material="plastic-trim", role="body", tone=0.3))
    out.append(roof("roof", ROOF, **{**PAINT, "tone": 0.7}))
    out.append(nose_stations("front-end", STATIONS, exponent=8, **PAINT))
    out.append(box("engine-undertray", [0.95, 0.56, 1.32], [1.72, 0.52, 0], material="plastic-trim", role="body", tone=0.12))
    out.append(box("bumper-rear", [0.16, 0.2, 1.97], [-2.59, 0.44, 0], material="plastic-trim", role="body", tone=0.2))
    floor = [[0.95, 1.25], [0.95, -1.15], [0.67, -1.15], [0.67, -2.05], [0.95, -2.05], [0.95, -2.625], [-0.95, -2.625], [-0.95, -2.05], [-0.67, -2.05], [-0.67, -1.15], [-0.95, -1.15], [-0.95, 1.25]]
    out.append(extrude("floor", floor, "zx", 0.36, Y_FLOOR, material="paint", role="body", tone=0.45))
    low = math.degrees(math.asin((Y_SILL - YA) / RA))
    for side, s in SIDES:
        out.append(extrude(f"wheelhouse-{side}", arc(XR, YA, 0.50, 180 - low, low, ARCH) + arc(XR, YA, RA, low, 180 - low, ARCH), "xy", s * 0.67, s * 0.95, material="paint", role="body", tone=0.4))
        out.append(extrude(f"wheelhouse-wall-{side}", arc(XR, YA, 0.50, 180 - low, low, ARCH), "xy", s * 0.59, s * 0.67, material="paint", role="body", tone=0.4))
    out.append(extrude("bulkhead", [[Y_FLOOR, -0.91], [Y_BELT, -0.91], [1.86, -0.84], [1.86, 0.84], [Y_BELT, 0.91], [Y_FLOOR, 0.91]], "yz", 0.25, 0.33, material="paint", role="body", tone=0.55))
    out.append(windscreen("windscreen", [-0.89, -0.845, 0.0, 0.845, 0.89]))
    b = dict(bone="slider", origin=SLIDER)
    out.append(wall("slider-panel", -1, [[X_QUARTER, Y_SILL], [X_B, Y_SILL], [X_B, Y_BELT], [X_QUARTER, Y_BELT]], thickness=T5, **b, **DOOR))
    out.append(plate("slider-upper", -1, [(1.16, X_QUARTER, X_B), (Y_CANT, X_QUARTER, X_B)], thickness=T5, **b, **DOOR))
    for side, s in SIDES:
        out.append(box(f"rear-leaf-{side}", [1.35, Z_LEAF, T5], [0.675, Z_LEAF / 2, -s * T5 / 2], bone=f"rear-door-{side}", **DOOR))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        out.append(revolve(f"{tag}-wheel", WHEEL, [0, 0, s], [0, 0, s * ZW], bone=bone, material="rubber", role="body", tone=0.1))
    out.append(capsule("rear-axle-beam", [0, 0, -0.74], [0, 0, 0.74], 0.04, bone="axle-r", material="cast-iron", role="detail", tone=0.3))
    out.append(capsule("front-axle-beam", [0, 0, -0.74], [0, 0, 0.74], 0.04, bone="axle-f", material="cast-iron", role="detail", tone=0.3))
    return out


PANELS = {"side-l": ["-w"], "side-upper-l": ["-w"], "quarter-r": ["+w"], "quarter-upper-r": ["+w"], "cab-side-r": ["+w"], "b-post-upper-r": ["+w"], "a-pillar-l": ["-w"], "a-pillar-r": ["+w"], "front-end": ["-u"], "floor": ["+v"], "wheelhouse-l": ["+v"], "wheelhouse-r": ["+v"]}


def thermal():
    out = [ref(f"th-{i}", i, "body-panels", None, faces) for i, faces in PANELS.items()]
    out.append(ref("th-roof", "roof", "roof", None, ["-v"]))
    out.append(box("th-bonnet", [0.75, 0.08, 1.6], [2.12, 1.09, 0], rotation=[0.0, 0.0, -0.1219, 0.9925], material="paint", role="body", tone=0.62, region="bonnet", sectionFaces=["-v"]))
    out += [ref("th-slider-panel", "slider-panel", "slider-door", None, ["+w"]), ref("th-slider-upper", "slider-upper", "slider-door", None, ["+w"])]
    out += [ref("th-rear-leaf-l", "rear-leaf-l", "rear-doors", None, ["-w"]), ref("th-rear-leaf-r", "rear-leaf-r", "rear-doors", None, ["+w"])]
    out += [ref("th-windscreen", "windscreen", "cab-glazing", None, ["-u", "-v"]), ref("th-door-glass-l", "door-glass-l", "cab-glazing", None, ["-w"]), ref("th-door-glass-r", "door-glass-r", "cab-glazing", None, ["+w"])]
    out.append(box("th-engine", [0.5, 0.42, 0.75], [2.12, 0.70, 0.0], material="engine-metal", role="detail", tone=0.2, region="engine-bay"))
    out.append(capsule("th-exhaust", [1.80, 0.42, -0.31], [-2.60, 0.34, -0.31], 0.04, material="exhaust-steel", role="detail", tone=0.35, region="exhaust-system"))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.04], [0.075, 0.04], [0.075, 0.0]], [-1, 0, 0], [-2.57, 0.34, -0.31], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", tags=["tailpipe"]))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        out.append(ref(f"th-{tag}-tyre", f"{tag}-wheel", "tyres"))
        out.append(revolve(f"th-{tag}-brake", [[-0.2, 0.0], [-0.2, 0.15], [-0.12, 0.15], [-0.12, 0.0]], [0, 0, s], [0, 0, s * ZW], bone=bone, material="cast-iron", role="detail", tone=0.42, region="brakes"))
    return out


def structure():
    frame = dict(material="cast-iron", role="detail", tone=0.5)
    out = []
    m = lambda s: out.append(member(s, "chassis"))
    for side, s in SIDES:
        m(box(f"chassis-rail-{side}", [5.0, 0.1, 0.08], [0.0, 0.42, s * 0.5], **frame))
        m(capsule(f"a-pillar-{side}", [1.25, 0.42, s * 0.93], [pillar_x(1.86), 1.86, s * (side_z(1.86) - 0.05)], 0.04, **frame))
        m(capsule(f"b-pillar-{side}", [0.30, 0.42, s * 0.93], [0.30, 1.86, s * (side_z(1.86) - 0.05)], 0.04, **frame))
        m(capsule(f"d-pillar-{side}", [-2.57, 0.42, s * 0.93], [-2.57, 1.86, s * (side_z(1.86) - 0.05)], 0.04, **frame))
        m(capsule(f"cant-rail-{side}", [-2.57, 1.86, s * (side_z(1.86) - 0.05)], [pillar_x(1.86), 1.86, s * (side_z(1.86) - 0.05)], 0.04, **frame))
    for name, x, y, half in (("crossmember-front", 1.25, 0.42, 0.93), ("crossmember-mid", 0.30, 0.42, 0.93), ("crossmember-rear", -2.57, 0.42, 0.93), ("roof-bow-front", 1.0, 1.86, 0.82), ("roof-bow-b", 0.30, 1.86, 0.86), ("roof-bow-rear", -2.57, 1.86, 0.86)):
        m(capsule(name, [x, y, -half], [x, y, half], 0.04, **frame))
    b = dict(bone="slider", **frame)
    wz, lean = -(W - 0.04), -(side_z(1.80) - 0.04)
    for name, u in (("rear", 0.06), ("front", 0.99)):
        out.append(member(capsule(f"slider-stile-{name}", [u, 0.1, wz], [u, 0.80, wz], 0.038, **b), "slider"))
        out.append(member(capsule(f"slider-stile-upper-{name}", [u, 0.80, wz], [u, 1.44, lean], 0.038, **b), "slider"))
    for side, s in SIDES:
        bone = f"rear-door-{side}"
        w = -s * 0.04
        for name, v in (("hinge-stile", 0.05), ("lock-stile", 0.81)):
            out.append(member(capsule(f"{bone}-{name}", [0.05, v, w], [1.30, v, w], 0.038, bone=bone, **frame), bone))
    out.append({"id": "st-front-axle-beam", "of": "front-axle-beam", "member": "axle-f", "part": "member"})
    out.append({"id": "st-rear-axle-beam", "of": "rear-axle-beam", "member": "axle-r", "part": "member"})
    for tag, bone in (("fl", "wheel-f"), ("fr", "wheel-f"), ("rl", "wheel-r"), ("rr", "wheel-r")):
        out.append({"id": f"st-{tag}-wheel", "of": f"{tag}-wheel", "member": bone, "part": "member"})
    return out


def decisions():
    return [
        {"en": "Body: the van as a hollow box of 80 mm panels (the smallest feature is 75 mm) that still opens: one side panel per side with arches of 6 chords, leaning upper panels that include the rear corners, roof, front end of eight sections, A-pillars, windscreen and door glass, sliding door and rear leaves in their apertures, floor, wheel houses, bulkhead, one-piece wheels on two axle beams and the mirrors that give the overall width.", "de": "Körper: der Kastenwagen als hohler Kasten aus 80-mm-Blechen (das Mindestmaß ist 75 mm), der sich weiter öffnet: je Seite ein Seitenblech mit Radausschnitten aus 6 Sehnen, geneigte obere Bleche einschließlich der Heckecken, Dach, Vorderwagen aus acht Schnitten, A-Säulen, Windschutzscheibe und Türscheiben, Schiebetür und Heckflügel in ihren Öffnungen, Boden, Radkästen, Trennwand, einteilige Räder auf zwei Achsträgern und die Spiegel, die die Gesamtbreite geben."},
        {"en": "Omitted: the front bumper, lamps, grille, corner posts, chassis rails, tail pipe, interior and every wheel detail; the model is minimal for a van whose three doors move.", "de": "Weggelassen: der vordere Stoßfänger, Leuchten, Grill, Eckpfosten, Längsträger, Endrohr, Innenraum und jedes Raddetail; das Modell ist minimal für einen Kastenwagen, dessen drei Türen sich bewegen."},
        {"en": "Thermal and structure: the bonnet is a skin on the front end, engine, exhaust pipe and brake discs are hidden solids, the outlet is a patch at the rear; the frame is two rails with three crossmembers, three pillar pairs, cant rails, three roof bows, the stiles of the doors, two axle beams and the wheels.", "de": "Thermik und Tragwerk: die Haube ist eine Haut auf dem Vorderwagen, Motor, Abgasrohr und Bremsscheiben sind verborgene Körper, der Auslass eine Fläche am Heck; der Rahmen besteht aus zwei Längsträgern mit drei Querträgern, drei Säulenpaaren, Dachlängsträgern, drei Dachspriegeln, den Holmen der Türen, zwei Achsträgern und den Rädern."},
    ]
