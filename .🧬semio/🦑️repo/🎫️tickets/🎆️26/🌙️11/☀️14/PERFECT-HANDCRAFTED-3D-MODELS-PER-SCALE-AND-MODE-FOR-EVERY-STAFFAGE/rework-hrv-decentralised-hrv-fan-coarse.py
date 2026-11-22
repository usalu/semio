"""🌀 Hand-chosen 1:100, 1:200 and 1:500 models of appliances/decentralised-hrv-fan (rework pass 1).

Every number below is chosen for the smallest feature of its scale (15 / 30 / 75 mm); the 1:50 list is the
reference for placement (axis w, room at -w, tray -0.12..-0.10, sleeve -0.10..0.12, core -0.037..0.063).
Run from the ticket folder: python3 rework-hrv-decentralised-hrv-fan-coarse.py
"""
import copy
import json
import math
import pathlib

SOURCE = pathlib.Path(__file__).parent / "volumes" / "appliances" / "decentralised-hrv-fan.json"
document = json.loads(SOURCE.read_text())
volumes = document["volumes"]
fine = {solid["id"]: solid for solid in volumes["50"]["body"]}
fine_structure = {solid["id"]: solid for solid in volumes["50"]["structure"]}
fine_thermal = {solid["id"]: solid for solid in volumes["50"]["thermal"]}


def r6(value):
    """🔢 Round to the micrometre."""
    return round(value, 6)


def circle(radius, count, clockwise=False):
    """⭕ Regular polygon of a circle."""
    points = [[r6(radius * math.cos(2 * math.pi * k / count)), r6(radius * math.sin(2 * math.pi * k / count))] for k in range(count)]
    return points[::-1] if clockwise else points


def square(half, corner, segments, clockwise=False):
    """⬜ Square of half-width `half` with rounded corners of `segments` chords (0 = sharp)."""
    if segments == 0:
        points = [[half, -half], [half, half], [-half, half], [-half, -half]]
    else:
        points = []
        for cx, cy, start in ((half - corner, -half + corner, -90), (half - corner, half - corner, 0), (-half + corner, half - corner, 90), (-half + corner, -half + corner, 180)):
            for k in range(segments + 1):
                angle = math.radians(start + 90 * k / segments)
                points.append([r6(cx + corner * math.cos(angle)), r6(cy + corner * math.sin(angle))])
    return points[::-1] if clockwise else points


def hexagon(cx, cy, across_flats):
    """⬡ Clockwise hexagon hole with a vertex up."""
    radius = across_flats / math.sqrt(3)
    return [[r6(cx + radius * math.cos(math.radians(-30 - 60 * k))), r6(cy + radius * math.sin(math.radians(-30 - 60 * k)))] for k in range(6)]


def base(identifier, **changes):
    """🧱 The 1:50 solid of that id with hand-chosen changes."""
    solid = copy.deepcopy(fine[identifier])
    for key, value in changes.items():
        if value is None:
            solid.pop(key, None)
        else:
            solid[key] = value
    return solid


def turn(quaternion, degrees):
    """🔄 Rotate an xyzw quaternion about w by `degrees`."""
    s, c = math.sin(math.radians(degrees) / 2), math.cos(math.radians(degrees) / 2)
    x, y, z, w = quaternion
    return [r6(c * x - s * y), r6(c * y + s * x), r6(c * z + s * w), r6(c * w - s * z)]


def blade(stations, chords, thicknesses, count, sector_start=54.0, radii=None):
    """🍃 One radial blade from stations of the 1:50 blade (index into its sections), repeated around the hub."""
    reference = fine["blade-1"]["sections"]
    sections = []
    for position, (index, chord, thickness) in enumerate(zip(stations, chords, thicknesses)):
        section = copy.deepcopy(reference[index])
        radius = radii[position] if radii else math.hypot(section["at"][0], section["at"][1])
        angle = math.radians(sector_start)
        section["at"] = [r6(radius * math.cos(angle)), r6(radius * math.sin(angle)), section["at"][2]]
        section["rotation"] = turn(section["rotation"], sector_start - 54.0)
        section["rx"], section["ry"] = chord / 2, thickness / 2
        sections.append(section)
    solid = base("blade-1", sections=sections)
    solid["repeat"] = {"count": count, "rotationAxis": [0, 0, 1], "rotationRad": r6(2 * math.pi / count)}
    return solid


def of(identifier, target, **fields):
    """🔗 Reference entry of a thermal or structure list."""
    return {"id": identifier, "of": target, **fields}


def casing(identifier, target):
    """🟥 Casing shell reference."""
    return of(identifier, target, region="casing", layer="shell")


def member(identifier, target, bone):
    """🟦 Structural member reference."""
    return of(identifier, target, member=bone, part="member")


def own(source, identifier, **changes):
    """✍️ An own thermal or structure solid of the 1:50 list with hand-chosen changes."""
    solid = copy.deepcopy(source[identifier])
    solid.update(changes)
    return solid


def mass(identifier, bone, radius, at_w, like="mass-core"):
    """⚫ Mass sphere at the centre of mass on the axis."""
    return own(fine_structure, like, id=identifier, bone=bone, member=bone, radii=[radius, radius, radius], transform={"translation": [0, 0, at_w]})


def supports(radius, top_from, top_to, roller_length, roller_x, roller_w):
    """🟩 The pin screw into the wall (upper left) and the roller on the wall face (lower right)."""
    top = own(fine_structure, "support-wall-screw-top", profile=[[top_from, 0.0], [top_from, radius], [top_to, radius], [top_to, 0.0]])
    bottom = own(fine_structure, "support-wall-screw-bottom", profile=[[0.0, 0.0], [0.0, radius], [roller_length, radius], [roller_length, 0.0]], origin=[roller_x, -0.108, roller_w])
    return [top, bottom]


def patches(thickness, radius_room, radius_outside):
    """🌬️ The two outlet patches on the faces of the ceramic."""
    room = own(fine_thermal, "th-supply-in", profile=[[r6(-0.037 - thickness / 2), 0.0], [r6(-0.037 - thickness / 2), radius_room], [r6(-0.037 + thickness / 2), radius_room], [r6(-0.037 + thickness / 2), 0.0]])
    outside = own(fine_thermal, "th-exhaust-out", profile=[[r6(0.063 - thickness / 2), 0.0], [r6(0.063 - thickness / 2), radius_outside], [r6(0.063 + thickness / 2), radius_outside], [r6(0.063 + thickness / 2), 0.0]])
    return [room, outside]


def cylinder(w0, w1, radius):
    """🥫 Open revolve profile of a filled cylinder."""
    return [[w0, 0.0], [w0, radius], [w1, radius], [w1, 0.0]]


JACKET_PLAIN = own(fine_structure, "member-core")

# ───────────────────────── 1:100, feature 15 mm ─────────────────────────
PITCH_100 = 0.031
CELLS_100 = [(0.0, 0.0)]
CELLS_100 += [(PITCH_100 * math.cos(math.radians(60 * k)), PITCH_100 * math.sin(math.radians(60 * k))) for k in range(6)]
CELLS_100 += [(2 * PITCH_100 * math.cos(math.radians(60 * k)), 2 * PITCH_100 * math.sin(math.radians(60 * k))) for k in range(6)]
CELLS_100 += [(math.sqrt(3) * PITCH_100 * math.cos(math.radians(30 + 60 * k)), math.sqrt(3) * PITCH_100 * math.sin(math.radians(30 + 60 * k))) for k in range(6)]

body_100 = [
    base("plate", profile=square(0.13, 0.02, 3), holes=[circle(0.0945, 36, True)]),
    base("plate-rim", profile=square(0.13, 0.02, 3), holes=[square(0.1275, 0.0175, 3, True)]),
    base("terminal", cornerRadius=None),
    base("ring", profile=[[-0.045, 0.088], [-0.045, 0.0945], [-0.1075, 0.0945], [-0.1075, 0.098], [-0.12, 0.098], [-0.12, 0.094], [-0.118243, 0.089757], [-0.114, 0.088], [-0.045, 0.088]]),
    base("sleeve", profile=circle(0.098, 36), holes=[circle(0.095, 36, True)]),
    base("rim", profile=circle(0.104, 36), holes=[circle(0.098, 36, True)]),
    base("jacket"),
    base("core", profile=circle(0.08, 36), holes=[hexagon(x, y, 0.025) for x, y in CELLS_100]),
    base("strut", size=[0.015, 0.0715, 0.015], transform={"translation": [0, 0.05375, -0.0575]}, cornerRadius=None),
    base("stator", profile=[[-0.082, 0.0075], [-0.082, 0.02], [-0.05, 0.02], [-0.05, 0.0075], [-0.082, 0.0075]]),
    blade([0, 1, 4, 8, 10], [0.034, 0.0342, 0.0435, 0.056, 0.022], [0.0076, 0.003, 0.0026, 0.002, 0.002], 5),
    base("hub", profile=[[-0.103, 0.0], [-0.103, 0.019], [-0.1005, 0.0265], [-0.098, 0.028], [-0.073, 0.028], [-0.073, 0.0215], [-0.0845, 0.0215], [-0.0845, 0.0]]),
    base("shaft", profile=cylinder(-0.0845, -0.054, 0.0075)),
]
thermal_100 = [
    casing("th-plate", "plate"), casing("th-plate-rim", "plate-rim"), casing("th-terminal", "terminal"), casing("th-ring", "ring"),
    casing("th-sleeve", "sleeve"), casing("th-rim", "rim"), casing("th-strut", "strut"),
    own(fine_thermal, "th-stator"),
    of("th-core", "core", region="core", layer="wall"),
    of("th-blade-1", "blade-1", region="blades", layer="blade"),
    own(fine_thermal, "th-hub", profile=[[-0.103, 0.0], [-0.103, 0.019], [-0.1005, 0.0265], [-0.098, 0.028], [-0.073, 0.028], [-0.073, 0.0]]),
    *patches(0.003, 0.08, 0.076),
]
structure_100 = [
    member("member-plate", "plate", "plate"), member("member-sleeve", "sleeve", "ring"), member("member-ring", "ring", "ring"),
    member("member-strut", "strut", "ring"), member("member-stator", "stator", "ring"), member("member-jacket", "jacket", "core"),
    JACKET_PLAIN, member("member-hub", "hub", "rotor"), member("member-shaft", "shaft", "rotor"),
    own(fine_structure, "joint-plate-ring", profile=[[-0.1075, 0.091], [-0.1075, 0.1015], [-0.0925, 0.1015], [-0.0925, 0.091], [-0.1075, 0.091]]),
    own(fine_structure, "joint-core-seat", profile=[[0.058, 0.07], [0.058, 0.083], [0.073, 0.083], [0.073, 0.07], [0.058, 0.07]]),
    {**own(fine_structure, "joint-plate-ring", id="joint-bearing", bone="ring", member="rotor"), "profile": [[-0.0735, 0.0075], [-0.0735, 0.0145], [-0.0585, 0.0145], [-0.0585, 0.0075], [-0.0735, 0.0075]]},
    mass("mass-plate", "plate", 0.0076, -0.11, "mass-plate"), mass("mass-ring", "ring", 0.0107, -0.012, "mass-ring"),
    mass("mass-core", "core", 0.014, 0.013), mass("mass-rotor", "rotor", 0.0075, -0.088, "mass-rotor"),
    *supports(0.0075, -0.124, -0.09, 0.015, 0.1005, -0.1175),
]

# ───────────────────────── 1:200, feature 30 mm ─────────────────────────
TUBE_200 = [[-0.12, 0.088], [-0.12, 0.098], [0.12, 0.098], [0.12, 0.0945], [-0.045, 0.0945], [-0.045, 0.088], [-0.12, 0.088]]
LINER_200 = [[-0.045, 0.088], [-0.045, 0.0945], [0.12, 0.0945], [0.12, 0.08], [-0.037, 0.08], [-0.045, 0.088]]
body_200 = [
    base("plate", profile=square(0.13, 0.02, 3), holes=[circle(0.093, 24, True)], **{"from": -0.12, "to": -0.1}),
    base("sleeve", kind="revolve", profile=TUBE_200, axis=[0, 0, 1], origin=[0.0, 0.0, 0.0], holes=None, plane=None, **{"from": None, "to": None}),
    base("jacket", profile=LINER_200),
    base("core", kind="revolve", profile=cylinder(-0.037, 0.063, 0.08), axis=[0, 0, 1], origin=[0.0, 0.0, 0.0], holes=None, plane=None, **{"from": None, "to": None}),
    base("strut", size=[0.03, 0.18, 0.03], transform={"translation": [0, 0, -0.058]}, repeat=None, cornerRadius=None),
    base("stator", profile=cylinder(-0.073, -0.043, 0.02)),
    blade([1, 5, 8, 10], [0.036, 0.046, 0.05, 0.024], [0.004, 0.003, 0.002, 0.002], 5),
    base("hub", profile=[[-0.103, 0.0], [-0.103, 0.019], [-0.098, 0.028], [-0.073, 0.028], [-0.073, 0.0]]),
]
thermal_200 = [
    casing("th-plate", "plate"), casing("th-sleeve", "sleeve"), casing("th-strut", "strut"), casing("th-stator", "stator"),
    of("th-core", "core", region="core", layer="wall"),
    of("th-blade-1", "blade-1", region="blades", layer="blade"),
    of("th-hub", "hub", region="hub", layer="can"),
    *patches(0.004, 0.08, 0.08),
]
structure_200 = [
    member("member-plate", "plate", "plate"), member("member-sleeve", "sleeve", "ring"), member("member-strut", "strut", "ring"),
    member("member-stator", "stator", "ring"), member("member-jacket", "jacket", "core"), member("member-core", "core", "core"),
    member("member-hub", "hub", "rotor"),
    {**own(fine_structure, "joint-plate-ring", id="joint-bearing", bone="ring", member="rotor"), "profile": cylinder(-0.088, -0.058, 0.015)},
    mass("mass-unit", "core", 0.0166, -0.009),
    *supports(0.015, -0.125, -0.095, 0.03, 0.093, -0.105),
]

# ───────────────────────── 1:500, feature 75 mm ─────────────────────────
body_500 = [
    base("plate", profile=square(0.13, 0, 0), holes=[circle(0.09, 16, True)], **{"from": -0.12, "to": -0.1}),
    base("sleeve", kind="revolve", profile=[[-0.12, 0.08], [-0.12, 0.098], [0.12, 0.098], [0.12, 0.08], [-0.12, 0.08]], axis=[0, 0, 1], origin=[0.0, 0.0, 0.0], holes=None, plane=None, **{"from": None, "to": None}),
    base("core", kind="revolve", profile=cylinder(-0.037, 0.063, 0.08), axis=[0, 0, 1], origin=[0.0, 0.0, 0.0], holes=None, plane=None, **{"from": None, "to": None}),
    base("strut", size=[0.04, 0.164, 0.02], transform={"translation": [0, 0, -0.055]}, repeat=None, cornerRadius=None),
    base("stator", profile=cylinder(-0.073, -0.045, 0.03)),
    blade([1, 5, 8], [0.034, 0.05, 0.06], [0.004, 0.003, 0.003], 3, sector_start=30.0, radii=[0.027, 0.05, 0.068]),
    base("hub", profile=[[-0.103, 0.0], [-0.103, 0.02], [-0.095, 0.03], [-0.073, 0.03], [-0.073, 0.0]]),
]
thermal_500 = [
    casing("th-plate", "plate"), casing("th-sleeve", "sleeve"), casing("th-strut", "strut"), casing("th-stator", "stator"),
    of("th-core", "core", region="core", layer="wall"),
    of("th-blade-1", "blade-1", region="blades", layer="blade"),
    of("th-hub", "hub", region="hub", layer="can"),
    *patches(0.005, 0.08, 0.08),
]
structure_500 = [
    member("member-plate", "plate", "plate"), member("member-sleeve", "sleeve", "ring"), member("member-strut", "strut", "ring"),
    member("member-stator", "stator", "ring"), member("member-core", "core", "core"), member("member-hub", "hub", "rotor"),
    {**own(fine_structure, "joint-plate-ring", id="joint-bearing", bone="ring", member="rotor"), "profile": cylinder(-0.078, -0.068, 0.02)},
    mass("mass-unit", "core", 0.02, -0.009),
    *supports(0.02, -0.13, -0.09, 0.04, 0.09, -0.11),
]

DECISIONS = {
    "100": [
        {"en": "Own 1:100 model for a 15 mm feature, same axes and stations as 1:50: the tray keeps its floor, its rim and the 60 mm terminal box, the 14 mm screw bosses and the screw heads are left out; fan housing with one chord for the inlet radius, the 196 mm sleeve with its 208 mm outer collar and the black EPP jacket with its 45 degree funnel stay separate parts. The ceramic block keeps its size of 160 mm by 100 mm, but its channels are 19 hexagons of 25 mm at a pitch of 31 mm (one centre cell, a ring of six and a ring of twelve), so every opening and every wall of 6 mm between two of them is still a part at this scale.", "de": "Eigenes 1:100-Modell für ein Merkmal von 15 mm, Achsen und Stationen wie bei 1:50: die Wanne behält Boden, Rand und den Klemmkasten von 60 mm, die Schraubdome von 14 mm und die Schraubenköpfe entfallen; das Ventilatorgehäuse mit einer Sehne für den Einlaufradius, die Hülse von 196 mm mit ihrem Außenkragen von 208 mm und der schwarze EPP-Mantel mit seinem Trichter von 45 Grad bleiben eigene Teile. Der Keramikblock behält seine 160 mm mal 100 mm, seine Kanäle sind aber 19 Sechsecke von 25 mm im Raster von 31 mm (eine Mittelzelle, ein Ring aus sechs und einer aus zwölf), sodass jede Öffnung und jeder Steg von 6 mm dazwischen in diesem Maßstab noch ein Teil ist."},
        {"en": "The impeller keeps its five radial blades with the pitch of 1:50, each lofted through five stations instead of eleven (root boss, 34 mm, 44 mm and 56 mm chord, rounded tip); the can loses its cap and its second nose chord. Three struts of 15 by 15 mm carry the 40 mm stator boss, the shaft is 15 mm thick and runs from the can floor through the boss; the bearing is only a joint of the structural model. Thermal: casing, core, blades and hub refer to these very solids, the stator boss and the can are filled for the thermal model and the outlets are 3 mm patches on the two faces of the ceramic. Structure: tray, housing, sleeve, struts and boss, jacket and plain ceramic cylinder, can and shaft, joints at the tray bore, the core seat and the bearing, four masses and the two wall screws.", "de": "Das Laufrad behält seine fünf radialen Flügel mit der Anstellung von 1:50, jeder durch fünf statt elf Stationen gezogen (Wurzelverdickung, Sehne 34 mm, 44 mm und 56 mm, gerundete Spitze); der Becher verliert Kappe und zweite Sehne der Wölbung. Drei Streben von 15 mal 15 mm tragen den Statorzapfen von 40 mm, die Welle ist 15 mm stark und läuft vom Becherboden durch den Zapfen; das Lager ist nur ein Gelenk des Tragwerksmodells. Thermik: Gehäuse, Kern, Flügel und Nabe verweisen auf genau diese Körper, Statorzapfen und Becher sind für das Wärmemodell gefüllt und die Auslässe sind Flächen von 3 mm auf den beiden Stirnseiten der Keramik. Tragwerk: Wanne, Gehäuse, Hülse, Streben und Zapfen, Mantel und glatter Keramikzylinder, Becher und Welle, Gelenke an Wannenbohrung, Kernsitz und Lager, vier Massen und die zwei Wandschrauben."},
    ],
    "200": [
        {"en": "Own 1:200 model for a 30 mm feature: the tray is one slab of 20 mm with three chords per corner, bored for the tube; fan housing and sleeve are one turned white tube from the room face to the outer end, the black EPP liner inside it runs flush to the outer end, so the unit ends in the dark ring that the drawing shows. The ceramic is a plain disc of 160 mm by 100 mm whose cell pattern has become its tone.", "de": "Eigenes 1:200-Modell für ein Merkmal von 30 mm: die Wanne ist eine Platte von 20 mm mit drei Sehnen je Ecke, für das Rohr gebohrt; Ventilatorgehäuse und Hülse sind ein gedrehtes weißes Rohr von der Raumseite bis zum Außenende, der schwarze EPP-Mantel darin läuft bündig bis zum Außenende, sodass das Gerät im dunklen Ring endet, den die Zeichnung zeigt. Die Keramik ist eine glatte Scheibe von 160 mm mal 100 mm, deren Zellmuster zu ihrem Ton geworden ist."},
        {"en": "The three struts are one bridge bar of 30 by 30 mm across the tube with the 40 mm stator boss in it; the can is a plain turned body with one nose chord and sits on the boss, shaft and bearing are inside it and only the bearing joint of the structural model remains. The five blades stay, since their chord of 36 to 56 mm is still above the feature, each lofted through four stations with its rounded tip. Thermal: tray, tube, bar and boss are the casing, disc, blades and can their own regions, the outlets are 4 mm patches on the faces of the disc. Structure: the same solids as members with the liner as the seat of the core, one mass at the common centre and the two wall screws drawn 30 mm thick.", "de": "Die drei Streben sind ein Brückensteg von 30 mal 30 mm quer durch das Rohr, in dem der Statorzapfen von 40 mm steckt; der Becher ist ein glatter Drehkörper mit einer Sehne an der Wölbung und sitzt auf dem Zapfen, Welle und Lager liegen in ihm, nur das Lagergelenk des Tragwerksmodells bleibt. Die fünf Flügel bleiben, weil ihre Sehne von 36 bis 56 mm noch über dem Merkmal liegt, jeder durch vier Stationen mit gerundeter Spitze gezogen. Thermik: Wanne, Rohr, Steg und Zapfen sind das Gehäuse, Scheibe, Flügel und Becher ihre eigenen Regionen, die Auslässe sind Flächen von 4 mm auf den Stirnseiten der Scheibe. Tragwerk: dieselben Körper als Stäbe mit dem Mantel als Sitz des Kerns, eine Masse im gemeinsamen Schwerpunkt und die zwei Wandschrauben 30 mm stark gezeichnet."},
    ],
    "500": [
        {"en": "Own 1:500 model for a 75 mm feature, where the whole unit is three and a half features wide: a sharp square slab of 0.26 m, one thick tube of 196 mm outside and 160 mm inside for housing, sleeve and liner, and the ceramic as a plug of 160 mm by 100 mm in it. Nothing smaller than the tube wall is kept on the outside.", "de": "Eigenes 1:500-Modell für ein Merkmal von 75 mm, bei dem das ganze Gerät dreieinhalb Merkmale breit ist: eine scharfkantige quadratische Platte von 0,26 m, ein dickes Rohr von 196 mm außen und 160 mm innen für Gehäuse, Hülse und Mantel und die Keramik als Pfropfen von 160 mm mal 100 mm darin. Außen bleibt nichts, was kleiner ist als die Rohrwand."},
        {"en": "The impeller is a 60 mm can with a conical nose and three broad blades of 34 to 60 mm chord that end 12 mm inside the bore, so the turning still reads; one bar of 40 mm and a 60 mm boss keep the load path from the can to the tube. Thermal: slab, tube, bar and boss are the casing, plug, blades and can their regions, the outlets are 5 mm patches on the faces of the plug. Structure: the same solids as members, the bearing joint between boss and can, one mass and the two wall screws drawn 40 mm thick.", "de": "Das Laufrad ist ein Becher von 60 mm mit kegeliger Nase und drei breiten Flügeln mit 34 bis 60 mm Sehne, die 12 mm innerhalb der Bohrung enden, sodass die Drehung lesbar bleibt; ein Steg von 40 mm und ein Zapfen von 60 mm halten den Lastpfad vom Becher zum Rohr. Thermik: Platte, Rohr, Steg und Zapfen sind das Gehäuse, Pfropfen, Flügel und Becher ihre Regionen, die Auslässe sind Flächen von 5 mm auf den Stirnseiten des Pfropfens. Tragwerk: dieselben Körper als Stäbe, das Lagergelenk zwischen Zapfen und Becher, eine Masse und die zwei Wandschrauben 40 mm stark gezeichnet."},
    ],
}

for scale, body, thermal, structure in (("100", body_100, thermal_100, structure_100), ("200", body_200, thermal_200, structure_200), ("500", body_500, thermal_500, structure_500)):
    volumes[scale]["body"], volumes[scale]["thermal"], volumes[scale]["structure"] = body, thermal, structure
    volumes[scale]["decisions"] = DECISIONS[scale]

SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
print("written", SOURCE.name, {scale: [len(volumes[scale][key]) for key in ("body", "thermal", "structure")] for scale in volumes})
