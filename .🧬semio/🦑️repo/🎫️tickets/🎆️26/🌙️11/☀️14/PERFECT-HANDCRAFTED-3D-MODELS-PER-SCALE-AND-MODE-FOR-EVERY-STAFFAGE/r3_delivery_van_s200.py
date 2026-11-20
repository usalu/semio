"""🚐 The 1:200 lists of the round 3 delivery van (smallest feature 30 mm): the hollow panel van reduced to its masses: side panels with the front doors merged in and arches of 8 chords, upper panels, roof, front end with the bumper merged in, pillars, glass, both door kinds in their apertures, plain wheels."""
import math

from r3_delivery_van_kit import QX, RA, SLIDER, T, W, XF, XR, X_B, X_BULK, X_HINGE, X_QUARTER, YA, Y_BELT, Y_CANT, Y_FLOOR, Y_SILL, ZW, Z_LEAF, arc, arch_points, arch_stations, box, capsule, ellipsoid, extrude, loft, member, nose_patch, nose_stations, pillar_x, plate, post, ref, revolve, roof, sec, side_z, wall, windscreen

SIDES = (("l", 1), ("r", -1))
PAINT = dict(material="paint", role="body", tone=0.65)
DOOR = dict(material="paint", role="body", tone=0.6)
STEEL = dict(material="cast-iron", role="detail", tone=0.4)
GLASS = dict(material="glass", role="glazing", tone=0.88)
ARCH = 8
ROOF = [(-2.64, 1.945, 0.04, 0.89), (-2.55, 1.945, 0.045, 0.905), (-2.49, 1.90, 0.09, 0.905), (0.60, 1.90, 0.09, 0.90), (1.0, 1.875, 0.075, 0.875), (1.12, 1.845, 0.04, 0.845)]
STATIONS = [1.22] + arch_stations(ARCH) + [2.30, 2.45, 2.56, 2.63, 2.67]
TYRE = [[-0.1175, 0.206], [-0.1175, 0.33], [-0.09, 0.356], [0.09, 0.356], [0.1175, 0.33], [0.1175, 0.206], [-0.1175, 0.206]]
DISC = [[-0.09, 0.0], [-0.09, 0.21], [0.05, 0.21], [0.085, 0.1], [0.095, 0.0]]


def quarter_outline(x_front):
    return [[-2.60, 0.36], [-2.32, 0.36], [-2.12, Y_SILL]] + arch_points(XR, ARCH) + [[x_front, Y_SILL], [x_front, Y_BELT], [-2.60, Y_BELT]]


def body():
    out = [wall("side-l", 1, quarter_outline(X_HINGE), **PAINT)]
    out.append(plate("side-upper-l", 1, [(1.16, -2.60, 0.37), (Y_CANT, -2.60, 0.37)], **PAINT))
    out.append(wall("quarter-r", -1, quarter_outline(X_QUARTER), **PAINT))
    out.append(plate("quarter-upper-r", -1, [(1.16, -2.60, X_QUARTER), (Y_CANT, -2.60, X_QUARTER)], **PAINT))
    out.append(wall("cab-side-r", -1, [[X_B, Y_SILL], [X_HINGE, Y_SILL], [X_HINGE, Y_BELT], [X_B, Y_BELT]], **PAINT))
    out.append(plate("b-post-upper-r", -1, [(1.16, X_B, 0.37), (Y_CANT, X_B, 0.37)], **PAINT))
    for side, s in SIDES:
        out.append(plate(f"a-pillar-{side}", s, [(y, pillar_x(y) - 0.06, pillar_x(y) + 0.06) for y in (1.15, Y_CANT)], thickness=0.09, n=6, **PAINT))
        out.append(plate(f"door-glass-{side}", s, [(1.165, 0.365, 1.535), (1.89, 0.365, 0.90)], thickness=0.005, sunk=0.012, n=10, **GLASS))
        out.append(post(f"corner-post-{side}", s, (0.52, Y_BELT, 1.945), 0.0525, 0.0625, n=8, **PAINT))
        out.append(ellipsoid(f"mirror-{side}", [0.06, 0.135, 0.095], [1.415, 1.385, s * 1.04], material="plastic-trim", role="detail", tone=0.25))
        at, q = nose_patch(2.50, 0.86, s, 0.015)
        out.append(ellipsoid(f"headlamp-{side}", [0.15, 0.085, 0.03], at, rotation=q, material="glass", role="glazing", tone=0.9))
    out.append(roof("roof", ROOF, **{**PAINT, "tone": 0.7}))
    out.append(nose_stations("front-end", STATIONS, **PAINT))
    out.append(box("engine-undertray", [0.95, 0.56, 1.32], [1.72, 0.52, 0], radius=0.05, material="plastic-trim", role="body", tone=0.12))
    out.append(box("bumper-rear", [0.31, 0.2, 1.99], [-2.515, 0.44, 0], radius=0.05, material="plastic-trim", role="body", tone=0.2))
    floor = [[0.95, 1.25], [0.95, -1.15], [0.67, -1.15], [0.67, -2.05], [0.95, -2.05], [0.95, -2.61], [-0.95, -2.61], [-0.95, -2.05], [-0.67, -2.05], [-0.67, -1.15], [-0.95, -1.15], [-0.95, 1.25]]
    out.append(extrude("floor", floor, "zx", 0.40, Y_FLOOR, material="paint", role="body", tone=0.45))
    low = math.degrees(math.asin((Y_SILL - YA) / RA))
    for side, s in SIDES:
        out.append(extrude(f"wheelhouse-{side}", arc(XR, YA, 0.465, 180 - low, low, ARCH) + arc(XR, YA, RA, low, 180 - low, ARCH), "xy", s * 0.69, s * 0.95, material="paint", role="body", tone=0.4))
        out.append(extrude(f"wheelhouse-wall-{side}", arc(XR, YA, 0.465, 180 - low, low, ARCH), "xy", s * 0.65, s * 0.69, material="paint", role="body", tone=0.4))
        out.append(box(f"chassis-rail-{side}", [3.85, 0.1, 0.08], [-0.63, 0.35, s * 0.5], **STEEL))
    out.append(extrude("bulkhead", [[Y_FLOOR, -0.945], [Y_BELT, -0.945], [1.86, -0.87], [1.86, 0.87], [Y_BELT, 0.945], [Y_FLOOR, 0.945]], "yz", 0.285, X_BULK, material="paint", role="body", tone=0.55))
    out.append(windscreen("windscreen", [-0.895, -0.845, 0.0, 0.845, 0.895]))
    b = dict(bone="slider", origin=SLIDER)
    out.append(wall("slider-panel", -1, [[X_QUARTER, Y_SILL], [X_B, Y_SILL], [X_B, Y_BELT], [X_QUARTER, Y_BELT]], proud=0.015, **b, **DOOR))
    out.append(plate("slider-upper", -1, [(1.16, X_QUARTER, X_B), (Y_CANT, X_QUARTER, X_B)], proud=0.015, **b, **DOOR))
    for side, s in SIDES:
        out.append(box(f"rear-leaf-{side}", [1.35, Z_LEAF, T], [0.675, Z_LEAF / 2, -s * T / 2], bone=f"rear-door-{side}", **DOOR))
    out.append(capsule("tail-pipe", [-2.30, 0.30, -0.31], [-2.60, 0.30, -0.31], 0.03, material="exhaust-steel", role="detail", tone=0.4))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        out.append(revolve(f"{tag}-tyre", TYRE, [0, 0, s], [0, 0, s * ZW], bone=bone, material="rubber", role="body", tone=0.1))
        out.append(revolve(f"{tag}-wheel", DISC, [0, 0, s], [0, 0, s * ZW], bone=bone, material="steel-rim", role="detail", tone=0.62))
    out.append(capsule("rear-axle-beam", [0, -0.01, -0.74], [0, -0.01, 0.74], 0.042, bone="axle-r", **{**STEEL, "tone": 0.3}))
    out.append(box("subframe", [0.5, 0.06, 1.44], [0.02, -0.1, 0], bone="axle-f", **{**STEEL, "tone": 0.3}))
    return out


PANELS = {
    "side-l": ["-w"], "side-upper-l": ["-w"], "quarter-r": ["+w"], "quarter-upper-r": ["+w"], "cab-side-r": ["+w"], "b-post-upper-r": ["+w"], "a-pillar-l": ["-w"], "a-pillar-r": ["+w"],
    "corner-post-l": ["-w", "+u"], "corner-post-r": ["+w", "+u"], "front-end": ["-u"], "floor": ["+v"], "wheelhouse-l": ["+v"], "wheelhouse-r": ["+v"], "wheelhouse-wall-l": ["-w"], "wheelhouse-wall-r": ["+w"],
}


def thermal():
    out = [ref(f"th-{i}", i, "body-panels", None, faces) for i, faces in PANELS.items()]
    out.append(ref("th-roof", "roof", "roof", None, ["-v"]))
    out.append(loft("th-bonnet", [sec([x, y, 0], 0.016, h, 3) for x, y, h in ((1.80, 1.19, 0.86), (2.2, 1.105, 0.855), (2.54, 1.0, 0.77))], material="paint", role="body", tone=0.62, region="bonnet", sectionFaces=["-v"]))
    out += [ref("th-slider-panel", "slider-panel", "slider-door", None, ["+w"]), ref("th-slider-upper", "slider-upper", "slider-door", None, ["+w"])]
    out += [ref("th-rear-leaf-l", "rear-leaf-l", "rear-doors", None, ["-w"]), ref("th-rear-leaf-r", "rear-leaf-r", "rear-doors", None, ["+w"])]
    out += [ref("th-windscreen", "windscreen", "cab-glazing", None, ["-u", "-v"]), ref("th-door-glass-l", "door-glass-l", "cab-glazing", None, ["-w"]), ref("th-door-glass-r", "door-glass-r", "cab-glazing", None, ["+w"])]
    out.append(box("th-engine", [0.5, 0.42, 0.75], [2.12, 0.70, 0.0], radius=0.04, material="engine-metal", role="detail", tone=0.2, region="engine-bay"))
    pipe = dict(material="exhaust-steel", role="detail", tone=0.35, region="exhaust-system")
    z = -0.31
    out.append(capsule("th-pipe", [1.80, 0.50, z], [-0.75, 0.33, z], 0.035, **pipe))
    out.append(capsule("th-silencer", [-0.80, 0.31, z], [-1.22, 0.31, z], 0.08, **pipe))
    out.append(capsule("th-rear-pipe", [-1.25, 0.36, z], [-2.60, 0.30, z], 0.03, **pipe))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.03], [0.03, 0.03], [0.03, 0.0]], [-1, 0, 0], [-2.61, 0.30, z], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", tags=["tailpipe"]))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        out.append(ref(f"th-{tag}-tyre", f"{tag}-tyre", "tyres"))
        out.append(revolve(f"th-{tag}-brake", [[-0.14, 0.0], [-0.14, 0.15], [-0.105, 0.15], [-0.105, 0.0]], [0, 0, s], [0, 0, s * ZW], bone=bone, material="cast-iron", role="detail", tone=0.42, region="brakes"))
    return out


def structure():
    frame = dict(material="cast-iron", role="detail", tone=0.5)
    out = [{"id": f"st-chassis-rail-{side}", "of": f"chassis-rail-{side}", "member": "chassis", "part": "member"} for side in "lr"]
    m = lambda s: out.append(member(s, "chassis"))
    for name, x in (("front", 1.2), ("mid", -0.3), ("rear", -2.45)):
        m(box(f"crossmember-{name}", [0.08, 0.1, 0.92], [x, 0.35, 0], **frame))
    for side, s in SIDES:
        m(box(f"sill-{side}", [2.47, 0.12, 0.09], [0.045, 0.37, s * 0.93], **frame))
        m(capsule(f"a-pillar-{side}", [1.25, 0.36, s * 0.94], [pillar_x(1.17), 1.17, s * 0.945], 0.04, **frame))
        m(capsule(f"a-pillar-upper-{side}", [pillar_x(1.17), 1.17, s * 0.945], [pillar_x(1.88), 1.88, s * (side_z(1.88) - 0.045)], 0.035, **frame))
        m(capsule(f"b-pillar-{side}", [0.30, 0.36, s * 0.945], [0.30, 1.88, s * (side_z(1.88) - 0.04)], 0.04, 0.035, **frame))
        m(capsule(f"c-pillar-{side}", [-0.84, 0.36, s * 0.945], [-0.84, 1.88, s * (side_z(1.88) - 0.04)], 0.04, 0.035, **frame))
        m(capsule(f"d-pillar-{side}", [-2.57, 0.52, s * 0.925], [-2.57, 1.90, s * (side_z(1.90) - 0.06)], 0.045, 0.035, **frame))
        m(capsule(f"cant-rail-{side}", [-2.57, 1.88, s * (side_z(1.88) - 0.04)], [pillar_x(1.88), 1.88, s * (side_z(1.88) - 0.045)], 0.035, **frame))
        m(box(f"front-rail-{side}", [1.45, 0.12, 0.08], [1.82, 0.50, s * 0.46], **frame))
    for name, x, y, half in (("windscreen-header", 1.0, 1.86, 0.84), ("roof-bow-b", 0.30, 1.90, 0.86), ("roof-bow-c", -0.84, 1.90, 0.86), ("rear-header", -2.57, 1.92, 0.85)):
        m(capsule(name, [x, y, -half], [x, y, half], 0.032, **frame))
    out.append(member(ellipsoid("com-chassis", [0.15, 0.15, 0.15], [0.48, 0.85, 0], **frame), "chassis", "mass"))
    b = dict(bone="slider", **frame)
    wz, lean = -(W - 0.02), -(side_z(1.80) - 0.02)
    for name, u in (("rear", 0.05), ("front", 1.0)):
        out.append(member(capsule(f"slider-stile-{name}", [u, 0.08, wz], [u, 0.80, wz], 0.025, **b), "slider"))
        out.append(member(capsule(f"slider-stile-upper-{name}", [u, 0.80, wz], [u, 1.44, lean], 0.025, **b), "slider"))
    for name, v, w in (("bottom", 0.08, wz), ("top", 1.44, lean)):
        out.append(member(capsule(f"slider-rail-{name}", [0.05, v, w], [1.0, v, w], 0.025, **b), "slider"))
    for side, s in SIDES:
        bone = f"rear-door-{side}"
        d = dict(bone=bone, **frame)
        w = -s * 0.0225
        for name, p0, p1 in (("hinge-stile", (0.04, 0.04), (1.31, 0.04)), ("lock-stile", (0.04, 0.82), (1.31, 0.82)), ("bottom-rail", (0.04, 0.04), (0.04, 0.82)), ("top-rail", (1.31, 0.04), (1.31, 0.82))):
            out.append(member(capsule(f"{bone}-{name}", [p0[0], p0[1], w], [p1[0], p1[1], w], 0.02, **d), bone))
    out.append({"id": "st-subframe", "of": "subframe", "member": "axle-f", "part": "member"})
    out.append({"id": "st-rear-axle-beam", "of": "rear-axle-beam", "member": "axle-r", "part": "member"})
    for side, s in SIDES:
        out.append(member(capsule(f"axle-f-strut-{side}", [-0.01, 0.08, s * 0.72], [-0.06, 0.60, s * 0.64], 0.035, bone="axle-f", **frame), "axle-f"))
        out.append(member(box(f"axle-r-spring-{side}", [1.3, 0.035, 0.07], [0, 0.0, s * 0.56], bone="axle-r", **frame), "axle-r"))
    for tag, bone in (("fl", "wheel-f"), ("fr", "wheel-f"), ("rl", "wheel-r"), ("rr", "wheel-r")):
        for part in ("tyre", "wheel"):
            out.append({"id": f"st-{tag}-{part}", "of": f"{tag}-{part}", "member": bone, "part": "member"})
    return out


def decisions():
    return [
        {"en": "Body: the hollow van reduced to masses for a 30 mm feature: one side panel per side up to the front hinge line with the front doors merged in, arches of 8 chords, leaning upper panels, roof, the front end with the bumper merged in, A-pillars, windscreen and door glass up to the roof, rear corner posts, sliding door and rear leaves in their apertures on their bones, floor, wheel houses, bulkhead, two chassis rails, tyres with plain wheel discs, mirrors and lens-shaped headlamps.", "de": "Körper: der hohle Kastenwagen, auf Massen reduziert für 30 mm Mindestmaß: je Seite ein Seitenblech bis zur vorderen Scharnierlinie mit den Vordertüren darin, Radausschnitte aus 8 Sehnen, geneigte obere Bleche, Dach, Vorderwagen mit eingeschlossenem Stoßfänger, A-Säulen, Windschutzscheibe und Türscheiben bis zum Dach, hintere Eckpfosten, Schiebetür und Heckflügel in ihren Öffnungen an ihren Knochen, Boden, Radkästen, Trennwand, zwei Längsträger, Reifen mit glatten Radscheiben, Spiegel und linsenförmige Scheinwerfer."},
        {"en": "Omitted: seats, dashboard and steering wheel (behind the glass tone), sills, cant rails, lamps at the rear, grille, bonnet skin, crossmembers, tank, spare wheel, suspension parts and every part below 30 mm; the bonnet region is a thermal skin of its own on the front end.", "de": "Weggelassen: Sitze, Armaturenbrett und Lenkrad (hinter dem Glaston), Schweller, Dachrahmen, Rückleuchten, Grill, Haubenblech, Querträger, Tank, Reserverad, Fahrwerksteile und alles unter 30 mm; die Haubenregion ist eine eigene thermische Haut auf dem Vorderwagen."},
        {"en": "Thermal and structure: one engine block, the exhaust as pipe, silencer and rear pipe with the outlet patch, brake discs as hidden discs inside the wheels; the frame keeps rails with three crossmembers, sills, front rails, four pillar pairs, cant rails, two roof bows, two headers, the door frames, struts, leaf springs, axles and wheels.", "de": "Thermik und Tragwerk: ein Motorblock, die Abgasanlage als Rohr, Schalldämpfer und Endrohr mit Auslassfläche, Bremsscheiben als verdeckte Scheiben in den Rädern; der Rahmen behält Längsträger mit drei Querträgern, Schweller, vordere Längsträger, vier Säulenpaare, Dachlängsträger, zwei Dachspriegel, zwei Querträger am Dach, die Türrahmen, Federbeine, Blattfedern, Achsen und Räder."},
    ]
