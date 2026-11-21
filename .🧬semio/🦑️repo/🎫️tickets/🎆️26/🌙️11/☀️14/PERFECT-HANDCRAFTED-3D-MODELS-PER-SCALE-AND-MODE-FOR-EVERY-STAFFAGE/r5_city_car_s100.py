"""🚗 Round 5 1:100 lists of the city-car: the round 3 design for 15 mm features with the lower body opened into nose, tail panel, 60 mm side walls, stepped floor, rear wheel-house plates and belt rails, bumper covers, wrap-around headlamps and lofted mirror housings; seat cushions are body solids; thermal and structure follow."""
import r3_city_car_s100 as r3
from r3_city_car_kit import ZW, XF, XR, box, capsule, hull_half, loft, plan_loft, ref, revolve, sec
from r5_city_car_kit import FLOOR_XS, WHEELHOUSE_Z, X_FIREWALL, X_TAIL, bumper, floor_pan, headlamp, hull_range, mirror_housing, side_wall, wheelhouse_plate

SIDES = r3.SIDES
PAINT = r3.PAINT
SEAT = r3.SEAT
WALL = 0.06
DROPPED = {"hull", "underbody", "waist", "cabin-deck", "grille"} | {f"{p}-{s}" for p in ("headlamp", "mirror") for s in "lr"}


def shields():
    """🛡️ Brake dust shields behind every wheel on the axle bones: the dark disc seen through the spokes."""
    out = []
    for side, s in SIDES:
        for axle, bone in (("f", "axle-f"), ("r", "axle-r")):
            out.append(r3.revolve(f"dust-shield-{axle}{side}", [[0.0, 0.0], [0.0, 0.182], [0.016, 0.182], [0.016, 0.0]], [0, 0, s], [0, 0, s * (ZW - 0.04)], bone=bone, material="cast-iron", role="detail", tone=0.15))
    return out


def body():
    out = shields() + [hull_range("nose", X_FIREWALL, 1.95, 6, **PAINT), hull_range("tail-panel", -1.95, X_TAIL, 6, **PAINT)]
    for side, s in SIDES:
        out.append(side_wall(f"side-wall-{side}", s, 6, WALL, **PAINT))
        out.append(loft(f"belt-rail-{side}", [sec([x, 0.975, s * (hull_half(x) - 0.065)], 0.035, 0.07, 6, r3.QX) for x in (-1.765, -1.0, 0.0, 0.915)], **PAINT))
        out.append(wheelhouse_plate(f"wheelhouse-plate-{side}", s, XR, WHEELHOUSE_Z + 0.01, 0.30, material="plastic-trim", role="body", tone=0.12))
        out.append(wheelhouse_plate(f"wheelhouse-plate-f{side}", s, XF, 0.55, 0.27, material="plastic-trim", role="body", tone=0.12))
        out.append(headlamp(f"headlamp-{side}", s, 0.715, [1.64, 1.75, 1.85, 1.905], 0.045, 0.02, material="glass", role="glazing", tone=0.9))
        out.append(mirror_housing(f"mirror-{side}", s, 0.775, 0.95, 0.77, 1.085, [(0.0, 0.035, 0.04), (0.3, 0.055, 0.06), (0.85, 0.055, 0.06), (1.0, 0.04, 0.045)], material="plastic-trim", role="detail", tone=0.3))
        out.append(box(f"seat-front-cushion-{side}", [0.5, 0.11, 0.48], [0.06, 0.53, s * 0.36], radius=0.04, **SEAT))
        out.append(box(f"seat-front-base-{side}", [0.38, 0.15, 0.36], [0.06, 0.40, s * 0.36], radius=0.02, **{**SEAT, "tone": 0.12}))
    out.append(floor_pan("floor", WALL, FLOOR_XS, material="interior-trim", role="body", tone=0.18))
    out.append(loft("belt-rail-rear", [sec([-1.745, 0.975, z], 0.035, 0.07, 6) for z in (-0.73, 0.0, 0.73)], **PAINT))
    out.append(plan_loft("dashboard", [(0.74, 0.62, 0.79, 0.74), (0.92, 0.48, 0.86, 0.75), (1.03, 0.55, 0.90, 0.74)], 5, **{**SEAT, "tone": 0.18}))
    out.append(box("seat-rear-cushion", [0.48, 0.11, 1.22], [-0.72, 0.565, 0], radius=0.04, **SEAT))
    out.append(bumper("bumper-front", 1.52, 1.95, 0.25, 0.50, 0.015, 1.95, material="paint", role="body", tone=0.58))
    out.append(bumper("bumper-rear", -1.95, -1.66, 0.345, 0.60, 0.015, -1.95, material="paint", role="body", tone=0.58))
    out.append(box("grille", [0.03, 0.12, 0.84], [1.952, 0.37, 0], radius=0.014, material="engine-metal", role="detail", tone=0.12))
    return out + [s for s in r3.body() if s["id"] not in DROPPED]


INWARD = {"nose": ["-u"], "tail-panel": ["+u"], "floor": ["+v"], "belt-rail-rear": ["+u"], "side-wall-l": ["-w"], "side-wall-r": ["+w"], "belt-rail-l": ["-w"], "belt-rail-r": ["+w"], "wheelhouse-plate-l": ["-w"], "wheelhouse-plate-r": ["+w"], "a-pillar-l": ["-w"], "a-pillar-r": ["+w"], "b-pillar-l": ["-w"], "b-pillar-r": ["+w"], "c-pillar-l": ["-w"], "c-pillar-r": ["+w"]}


def thermal():
    parts = {s["id"]: s for s in body()}
    panels = ["nose", "tail-panel", "floor", "belt-rail-rear", "bumper-front", "bumper-rear"] + [f"{i}-{side}" for i in ("side-wall", "belt-rail", "sill", "wheelhouse-plate", "mirror", "d-pillar", "a-pillar", "b-pillar", "c-pillar") for side in "lr"] + ["wheelhouse-plate-fl", "wheelhouse-plate-fr"]
    out = [ref(f"th-{i}", i, "body-panels", faces=INWARD.get(i)) for i in panels]
    for side, face in (("l", "-w"), ("r", "+w")):
        for i in ("side-glass-front", "side-glass-rear"):
            out.append(ref(f"th-{i}-{side}", f"{i}-{side}", "glazing", faces=[face]))
        out.append(ref(f"th-headlamp-{side}", f"headlamp-{side}", "glazing"))
    out.append(ref("th-roof", "roof", "roof", faces=["-v"]))
    out.append(ref("th-bonnet", "bonnet-skin", "bonnet", faces=["-v"]))
    out.append(ref("th-hatch-panel", "hatch-panel", "tailgate", faces=["+v"]))
    out.append(ref("th-hatch-lip", "hatch-lip", "tailgate", faces=["+v"]))
    out.append(ref("th-windscreen", "windscreen", "glazing", faces=["-v"]))
    out.append(ref("th-hatch-glass", "hatch-glass", "glazing", faces=["+v"]))
    for i in ("seat-front-l", "seat-front-r", "seat-rear", "seat-front-cushion-l", "seat-front-cushion-r", "seat-rear-cushion", "dashboard", "steering-wheel"):
        out.append({**parts[i], "id": f"th-{i}", "region": "interior"})
    engine = dict(material="engine-metal", role="detail", tone=0.2, region="engine-bay")
    out.append(box("th-engine-block", [0.3, 0.38, 0.38], [1.36, 0.56, 0.13], radius=0.03, **engine))
    out.append(capsule("th-gearbox", [1.33, 0.48, -0.14], [1.33, 0.48, -0.36], 0.13, **engine))
    out.append(capsule("th-exhaust-manifold", [1.17, 0.62, 0.02], [1.12, 0.34, -0.08], 0.045, **engine))
    for i in ("exhaust-pipe", "silencer", "tail-pipe"):
        out.append(ref(f"th-{i}", i, "exhaust-system"))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.0225], [0.015, 0.0225], [0.015, 0.0]], [-1, 0, 0], [-1.94, 0.31, -0.3], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", tags=["tailpipe"]))
    for w in ("fl", "fr", "rl", "rr"):
        out.append(ref(f"th-{w}-tyre", f"{w}-tyre", "tyres"))
        out.append(ref(f"th-{w}-disc", f"{w}-disc", "brakes"))
    return out


def structure():
    return r3.structure()


def decisions():
    return [
        {"en": "At 1:100 the body is the same design drawn for 15 mm features: nose, tail panel and 60 mm side walls on the master hull lines with both wheel arches, a stepped floor, wheel-house plates and belt rails, so the cabin and the boot are real spaces; a roof with crown and swept header, plate pillars in the tumblehome plane, thin glass, bumper covers, wrap-around headlamps and lofted mirror housings; seams, handles, plates, wipers, arch lips and calipers are left to the drawing.", "de": "Im Maßstab 1:100 ist die Karosserie derselbe Entwurf für 15-mm-Merkmale: Bug, Heckblech und 60 mm starke Seitenwände auf den Rumpflinien mit beiden Radläufen, ein gestufter Boden, Radhausbleche und Brüstungsholme, sodass Kabine und Kofferraum echte Räume sind; ein Dach mit Wölbung und gepfeiltem Windlauf, Plattenpfosten in der Ebene der Seitenscheiben, dünnes Glas, Stoßfängerverkleidungen, umlaufende Scheinwerfer und geloftete Spiegelgehäuse; Fugen, Griffe, Kennzeichen, Wischer, Radlauflippen und Bremssättel bleiben der Zeichnung überlassen."},
        {"en": "Each front seat is a cushion on a base and one loft from back to headrest, the bench is a cushion and one back loft; the exhaust is one pipe with silencer and tail pipe, each wheel is tyre, dished rim, hub, five spokes and one plain brake disc; thermal surfaces reference these solids with the layers of 1:50 merged into single surfaces, and the engine, gearbox and manifold are real hidden solids.", "de": "Jeder Vordersitz ist ein Polster auf einem Untergestell und ein Loft von der Lehne bis zur Kopfstütze, die Rückbank ein Polster und ein Lehnen-Loft; die Abgasanlage ist ein Rohr mit Schalldämpfer und Endrohr, jedes Rad besteht aus Reifen, geschüsselter Felge, Nabe, fünf Speichen und einer glatten Bremsscheibe; die thermischen Flächen verweisen auf diese Körper mit den Schichten von 1:50 zu einfachen Flächen zusammengefasst, und Motor, Getriebe und Krümmer sind echte verdeckte Körper."},
        {"en": "The structure is the unibody frame without its secondary members: sills, front and rear rails with their kick-ups, hinge, A, B and C pillars, roof rails, five floor crossmembers and four roof bows, the bonnet and tailgate frames, struts, subframe, twist beam and the wheels on their bearings.", "de": "Das Tragwerk ist der selbsttragende Rahmen ohne Nebenträger: Schweller, vordere und hintere Längsträger mit ihren Kröpfungen, Scharnier-, A-, B- und C-Säulen, Dachholme, fünf Bodenquerträger und vier Dachspriegel, die Rahmen von Haube und Heckklappe, Federbeine, Hilfsrahmen, Verbundlenkerachse und die Räder auf ihren Lagern."},
    ]
