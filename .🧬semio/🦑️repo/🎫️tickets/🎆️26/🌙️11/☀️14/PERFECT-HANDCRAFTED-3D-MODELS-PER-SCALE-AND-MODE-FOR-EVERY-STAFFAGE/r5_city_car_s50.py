"""🚙 Round 5 1:50 lists of the city-car: the round 3 design with the lower body opened into a real body in white (nose, tail panel, side walls, stepped floor, rear wheel houses, belt rails) so cabin, boot and seats are real spaces, plus bumper covers, wrap-around headlamps, lofted mirror housings and arch lips; thermal shells and structure follow."""
import r3_city_car_s50 as r3
from r3_city_car_kit import ZW, RA, XF, XR, box, capsule, hull_half, loft, plan_loft, sec, shrunk
from r5_city_car_kit import FLOOR_XS, WHEELHOUSE_Z, X_FIREWALL, X_TAIL, bumper, floor_pan, headlamp, hull_range, lip, mirror_housing, side_wall, wheelhouse, wheelhouse_plate

SIDES = r3.SIDES
PAINT = r3.PAINT
TRIM = dict(material="plastic-trim", role="detail", tone=0.2)
SEAT = r3.SEAT
WALL = 0.05
DROPPED = {"hull", "underbody", "waist", "cabin-deck", "intake", "plate-front", "grille"} | {f"{p}-{s}" for p in ("headlamp", "mirror", "mirror-foot", "mirror-glass") for s in "lr"}


def lower_body():
    out = [hull_range("nose", X_FIREWALL, 1.95, 12, **PAINT), hull_range("tail-panel", -1.95, X_TAIL, 12, **PAINT)]
    for side, s in SIDES:
        out.append(side_wall(f"side-wall-{side}", s, 12, WALL, **PAINT))
        out.append(loft(f"belt-rail-{side}", [sec([x, 0.975, s * (hull_half(x) - 0.06)], 0.035, 0.07, 6, r3.QX) for x in (-1.765, -1.3, 0.0, 0.5, 0.915)], **PAINT))
        out.append(wheelhouse(f"wheelhouse-{side}", s, XR, WHEELHOUSE_Z, hull_half(XR) - WALL / 2, material="plastic-trim", role="body", tone=0.12))
        out.append(wheelhouse_plate(f"wheelhouse-plate-{side}", s, XR, WHEELHOUSE_Z + 0.0075, 0.30, material="plastic-trim", role="body", tone=0.12))
        out.append(wheelhouse_plate(f"wheelhouse-plate-f{side}", s, XF, 0.55, 0.27, material="plastic-trim", role="body", tone=0.12))
        for axle, xc in (("f", XF), ("r", XR)):
            out.append(lip(f"arch-lip-{axle}{side}", s, xc, **PAINT))
    out.append(floor_pan("floor", WALL, FLOOR_XS, material="interior-trim", role="body", tone=0.18))
    out.append(loft("belt-rail-rear", [sec([-1.745, 0.975, z], 0.035, 0.07, 6) for z in (-0.74, 0.0, 0.74)], **PAINT))
    out.append(plan_loft("dashboard", [(0.74, 0.62, 0.79, 0.74), (0.92, 0.48, 0.86, 0.75), (1.03, 0.55, 0.90, 0.74)], 5, **{**SEAT, "tone": 0.18}))
    return out


def front_and_rear():
    out = [bumper("bumper-front", 1.52, 1.95, 0.25, 0.50, 0.008, 1.95, material="paint", role="body", tone=0.58)]
    out.append(bumper("bumper-rear", -1.95, -1.66, 0.345, 0.60, 0.008, -1.95, material="paint", role="body", tone=0.58))
    out.append(box("grille", [0.03, 0.07, 0.6], [1.94, 0.655, 0], radius=0.015, material="engine-metal", role="detail", tone=0.12))
    out.append(box("intake", [0.02, 0.1, 0.84], [1.958, 0.37, 0], radius=0.009, material="plastic-trim", role="detail", tone=0.1))
    out.append(box("plate-front", [0.012, 0.11, 0.52], [1.967, 0.47, 0], radius=0.004, material="plastic-trim", role="detail", tone=0.9))
    out.append(box("diffuser-rear", [0.02, 0.05, 0.4], [-1.958, 0.375, 0.1], radius=0.009, material="plastic-trim", role="detail", tone=0.1))
    for side, s in SIDES:
        out.append(headlamp(f"headlamp-{side}", s, 0.715, [1.64, 1.72, 1.79, 1.85, 1.885, 1.905], 0.045, 0.02, material="glass", role="glazing", tone=0.9))
        out.append(mirror_housing(f"mirror-{side}", s, 0.775, 0.95, 0.775, 1.085, [(0.0, 0.03, 0.035), (0.2, 0.05, 0.055), (0.55, 0.06, 0.063), (0.85, 0.055, 0.06), (1.0, 0.035, 0.045)], material="paint", role="detail", tone=0.55))
        out.append(box(f"mirror-glass-{side}", [0.006, 0.1, 0.14], [0.718, 1.085, s * 0.875], radius=0.003, material="glass", role="glazing", tone=0.9))
        out.append(box(f"fog-lamp-{side}", [0.02, 0.045, 0.1], [1.945, 0.37, s * 0.56], radius=0.009, material="glass", role="glazing", tone=0.85))
        out.append(box(f"reflector-rear-{side}", [0.012, 0.03, 0.12], [-1.962, 0.40, s * 0.62], radius=0.005, material="glass", role="detail", tone=0.7))
    return out


def seats():
    out = []
    for side, s in SIDES:
        out.append(box(f"seat-front-cushion-{side}", [0.5, 0.11, 0.48], [0.06, 0.53, s * 0.36], radius=0.045, **SEAT))
        out.append(box(f"seat-front-base-{side}", [0.38, 0.15, 0.36], [0.06, 0.40, s * 0.36], radius=0.02, **{**SEAT, "tone": 0.12}))
    out.append(box("seat-rear-cushion", [0.48, 0.11, 1.22], [-0.72, 0.565, 0], radius=0.045, **SEAT))
    return out


def shields():
    """🛡️ Brake dust shields behind every wheel on the axle bones: the dark disc seen through the spokes."""
    out = []
    for side, s in SIDES:
        for axle, bone in (("f", "axle-f"), ("r", "axle-r")):
            out.append(r3.revolve(f"dust-shield-{axle}{side}", [[0.0, 0.0], [0.0, 0.182], [0.008, 0.182], [0.008, 0.0]], [0, 0, s], [0, 0, s * (ZW - 0.04)], bone=bone, material="cast-iron", role="detail", tone=0.15))
    return out


def body():
    kept = [s for s in r3.body() if s["id"] not in DROPPED]
    return lower_body() + front_and_rear() + seats() + shields() + kept


INWARD = {"nose": ["-u"], "tail-panel": ["+u"], "side-wall-l": ["-w"], "side-wall-r": ["+w"], "belt-rail-l": ["-w"], "belt-rail-r": ["+w"], "belt-rail-rear": ["+u"], "a-pillar-l": ["-w"], "a-pillar-r": ["+w"], "b-pillar-l": ["-w"], "b-pillar-r": ["+w"], "c-pillar-l": ["-w"], "c-pillar-r": ["+w"]}
SHEETED = ["nose", "tail-panel", "side-wall-l", "side-wall-r", "belt-rail-l", "belt-rail-r", "belt-rail-rear", "sill-l", "sill-r", "bumper-front", "bumper-rear", "d-pillar-l", "d-pillar-r"] + [f"arch-lip-{a}{s}" for a in "fr" for s in "lr"] + [f"{p}-pillar-{s}" for p in "abc" for s in "lr"]
LINERS = {"floor": ["+v"], "wheelhouse-l": ["+v"], "wheelhouse-r": ["+v"], "wheelhouse-plate-l": ["-w"], "wheelhouse-plate-r": ["+w"], "wheelhouse-plate-fl": None, "wheelhouse-plate-fr": None}
HIDDEN_INTERIOR = ("seat-front-back-l", "seat-front-back-r", "seat-front-headrest-l", "seat-front-headrest-r", "seat-rear-back", "seat-rear-headrest-l", "seat-rear-headrest-r", "binnacle", "steering-wheel", "dashboard", "seat-front-cushion-l", "seat-front-cushion-r", "seat-rear-cushion")


def thermal():
    parts = {s["id"]: s for s in body()}
    out = []
    for i in SHEETED:
        out += r3.shells([parts[i]], "body-panels", INWARD.get(i))
    out += r3.shells([parts["roof"]], "roof", ["-v"])
    out += r3.shells([parts["bonnet-skin"]], "bonnet", ["-v"])
    out += r3.shells([parts[i] for i in ("hatch-panel", "hatch-header", "hatch-lip")], "tailgate", ["+v"])
    for i, faces in LINERS.items():
        out.append(r3.ref(f"th-{i}", i, "body-panels", "paint", faces))
    out.append(r3.ref("th-windscreen", "windscreen", "glazing", "glass", ["-v"]))
    for side, face in (("l", "-w"), ("r", "+w")):
        for i in ("side-glass-front", "side-glass-rear"):
            out.append(r3.ref(f"th-{i}-{side}", f"{i}-{side}", "glazing", "glass", [face]))
        for i in ("headlamp", "mirror-glass"):
            out.append(r3.ref(f"th-{i}-{side}", f"{i}-{side}", "glazing", "glass"))
        out.append(r3.ref(f"th-mirror-{side}", f"mirror-{side}", "body-panels", "paint"))
    out.append(r3.ref("th-hatch-glass", "hatch-glass", "glazing", "glass", ["+v"]))
    for i in HIDDEN_INTERIOR:
        out.append({**parts[i], "id": f"th-{i}", "region": "interior", "layer": "foam"})
    engine = dict(material="engine-metal", role="detail", tone=0.2, region="engine-bay")
    out.append(box("th-engine-block", [0.3, 0.38, 0.38], [1.36, 0.56, 0.13], radius=0.03, **engine))
    out.append(capsule("th-gearbox", [1.33, 0.48, -0.14], [1.33, 0.48, -0.36], 0.13, **engine))
    out.append(capsule("th-exhaust-manifold", [1.17, 0.62, 0.02], [1.12, 0.34, -0.08], 0.045, **engine))
    for i in ("exhaust-front-pipe", "exhaust-resonator", "exhaust-mid-pipe", "exhaust-axle-pipe", "silencer", "tail-pipe"):
        out.append(r3.ref(f"th-{i}", i, "exhaust-system", "pipe"))
    out.append(r3.revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.0225], [0.004, 0.0225], [0.004, 0.0]], [-1, 0, 0], [-1.948, 0.31, -0.30], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", layer="pipe", tags=["tailpipe"]))
    for w in ("fl", "fr", "rl", "rr"):
        out.append(r3.ref(f"th-{w}-tyre", f"{w}-tyre", "tyres", "tread"))
        out.append(r3.ref(f"th-{w}-disc", f"{w}-disc", "brakes"))
        out.append(r3.ref(f"th-{w}-disc-hat", f"{w}-disc-hat", "brakes"))
        out.append(r3.ref(f"th-caliper-{w}", f"caliper-{w}", "brakes"))
    return out


def structure():
    return r3.structure()


def decisions():
    return [
        {"en": "At 1:50 the lower body is a real body in white, not a filled hull: the nose ahead of the firewall, the tail panel, two 50 mm side walls whose outer faces are the master hull side, a floor stepped from the footwells (0.33 m) over the rear seat riser (0.45 m) to the boot floor (0.58 m), rear wheel houses with their inner plates and belt rails, so cabin and boot are open spaces in the hatch-open and section views.", "de": "Im Maßstab 1:50 ist der Unterbau eine echte Rohkarosserie statt eines gefüllten Rumpfs: Bug vor der Stirnwand, Heckblech, zwei 50 mm starke Seitenwände, deren Außenfläche die Rumpfseite der Leitlinien ist, ein Boden in Stufen vom Fußraum (0,33 m) über den Fondsitzsockel (0,45 m) zum Kofferraumboden (0,58 m), hintere Radhäuser mit Innenblechen und Brüstungsholme, sodass Kabine und Kofferraum bei offener Heckklappe und im Schnitt offene Räume sind."},
        {"en": "Bumper covers stand 8 mm proud of the hull with their shut lines at 0.50 m and 0.60 m, the headlamps are lens lofts wrapped round the front corners under the bonnet shut line, the door mirrors are lofted housings with the glass on the rear face, every wheel arch has a flared lip, and the seats have cushions, bases and backs on the floor.", "de": "Die Stoßfängerverkleidungen stehen 8 mm vor dem Rumpf mit ihren Fugen auf 0,50 m und 0,60 m, die Scheinwerfer sind um die vorderen Ecken unter der Haubenfuge gelegte Linsen-Lofts, die Außenspiegel geloftete Gehäuse mit dem Glas auf der Rückseite, jeder Radlauf hat eine ausgestellte Lippe, und die Sitze haben Polster, Untergestelle und Lehnen auf dem Boden."},
        {"en": "Thermal follows the same solids: paint film and steel sheet as nested shells on every steel panel, single shells on floor and wheel-house liners, glass, seat foam, hidden engine, gearbox and manifold, the exhaust run with its outlet patch, tyres and brakes; the faces of panels, glass and floor that look into the cabin and the boot are marked as section faces, as in round 3, because the plate pillars and the hinged tailgate leave seams wider than the flood cell (C31). The structure is the unibody frame of round 3.", "de": "Das thermische Modell folgt denselben Körpern: Lackfilm und Stahlblech als verschachtelte Schalen auf jedem Stahlblechteil, einfache Schalen auf Boden und Radhausschalen, Glas, Sitzschaum, verdeckter Motor, Getriebe und Krümmer, die Abgasanlage mit ihrer Austrittsfläche, Reifen und Bremsen; die Flächen von Blechen, Glas und Boden, die in Kabine und Kofferraum blicken, sind wie in Runde 3 als Schnittflächen markiert, weil die Plattenpfosten und die angeschlagene Heckklappe Fugen lassen, die breiter als die Flutzelle sind (C31). Das Tragwerk ist der selbsttragende Rahmen aus Runde 3."},
    ]
