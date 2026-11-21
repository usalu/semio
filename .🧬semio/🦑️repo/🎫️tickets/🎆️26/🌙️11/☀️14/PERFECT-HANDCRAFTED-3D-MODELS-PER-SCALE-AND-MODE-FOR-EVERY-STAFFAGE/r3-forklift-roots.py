"""🌳 Root values of the round 3 forklift outside `volumes`: reference, dimensions, the EFG 425 rig, anchors, solved poses, clip texts, thermal areas and lengths, representation shapes and texts, applied to the parsed asset."""
import copy
import math

REFERENCE = {"id": "jungheinrich-efg-425", "citation": "Jungheinrich data sheet EFG 425k/425/430k/430/S30, Technical data in line with VDI 2198, issue 06/2018 (EN), read 2026-11-19 at https://secondowner.com/storage/attachments/efg-425k-425-430k-430-s30-en(a8b).pdf: EFG 425 rated load 2.5 t at c = 500 mm, load distance x = 425 mm, wheelbase 1,720 mm, net weight 4,680 kg incl. battery, axle loads laden 6,590 / 590 kg and unladen 2,720 / 1,960 kg, SE tyres 225/75-10 front and 180/70-8 rear, track 990 / 940 mm, mast tilt 6 / 8 degrees, Duplex ZT 3100 mast lowered 2,225 mm with 150 mm free lift and 3,855 mm extended, overhead guard 2,240 mm, seat 1,190 mm, coupling 385 / 540 mm, length 3,591 mm with 1,150 mm forks, 2,441 mm to the fork face, width 1,198 mm, forks 40 x 120 x 1,150 mm, carriage ISO 2328 class 2A 1,120 mm wide, ground clearance 117 / 135 mm, turning radius 2,150 mm, 80 V 620 Ah battery of 1,540 kg (1,028 x 711 x 784 mm), drive motor 15 kW, lift motor 22 kW, travel 19 km/h, lift 0.48 m/s laden."}
EPAL = {"id": "epal-euro-pallet", "citation": "EN 13698-1 and EPAL Euro pallet (EPAL 1): 1,200 x 800 x 144 mm; five top deck boards 22 mm (145, 100, 145, 100, 145 mm), three stringer boards 145 x 22 mm, nine blocks 78 mm high (corner 145 x 100 mm, centre row 145 x 145 mm), three bottom boards 22 mm (100, 145, 100 mm); four-way entry, fork openings 227.5 mm wide on the 800 mm face."}
AREAS = {"battery-pack": 4.2, "chassis-paint": 12, "counterweight": 4.3, "mast": 5.9, "fork-gear": 3.0, "brakes": 0.25}
HIDDEN = {
    "battery-pack": {"en": "The battery stands in its compartment between the axles, enclosed by the frame side plates, the battery door and the hood under the seat: it is drawn in no elevation.", "de": "Die Batterie steht in ihrem Fach zwischen den Achsen, umschlossen von den Rahmenseitenblechen, der Batterietür und der Haube unter dem Sitz: sie wird in keiner Ansicht gezeichnet."},
    "drive-motor": {"en": "The twin drive motors lie in the front axle under the floor plate, between the drive wheels: covered by the wheels and fenders from the side, by the pallet and goods from ahead and by the floor from above.", "de": "Die beiden Fahrmotoren liegen in der Vorderachse unter der Bodenplatte zwischen den Antriebsrädern: seitlich von Rädern und Kotflügeln, von vorn von Palette und Ladegut und von oben vom Boden verdeckt."},
    "brakes": {"en": "The gear and brake housings sit inside the drive wheels behind the rim discs.", "de": "Die Getriebe- und Bremsgehäuse sitzen in den Antriebsrädern hinter den Felgenschüsseln."},
}


def text(en, de):
    return {"en": en, "de": de}


def mass(K, bone, kg, x, y):
    u, v = K.to_local(bone, x, y)
    return {"kg": kg, "comX": K.r5(u), "comY": K.r5(v), "distribution": "point", "source": "authored-estimate"}


MASSES = {"chassis": (905, -0.70, 0.72), "battery": (1540, -0.7155, 0.542), "counterweight": (870, -1.80, 0.72), "axle-f": (320, 0.0, 0.296), "axle-r": (125, -1.72, 0.23), "wheel-f": (96, 0.0, 0.296), "wheel-r": (50, -1.72, 0.2276), "tilt-cyl": (30, -0.09, 0.82), "tilt-rod": (12, 0.06, 0.89), "mast-outer": (262, 0.17, 1.10), "mast-inner": (200, 0.18, 1.20), "carriage": (130, 0.36, 0.55), "forks": (140, 0.80, 0.10)}


def bones(K):
    titles = {
        "chassis": text("Frame with body, cowl, operator compartment and overhead guard", "Rahmen mit Aufbau, Armaturenhaube, Fahrerplatz und Fahrerschutzdach"),
        "battery": text("Traction battery 80 V 620 Ah", "Traktionsbatterie 80 V 620 Ah"),
        "counterweight": text("Cast counterweight", "Gegossenes Gegengewicht"),
        "axle-f": text("Drive axle with twin motors", "Antriebsachse mit zwei Fahrmotoren"),
        "axle-r": text("Steer axle", "Lenkachse"),
        "wheel-f": text("Drive wheels", "Antriebsräder"),
        "wheel-r": text("Steer wheels", "Lenkräder"),
        "tilt-cyl": text("Tilt cylinders, barrels", "Neigezylinder, Zylinderrohre"),
        "tilt-rod": text("Tilt cylinders, rods", "Neigezylinder, Kolbenstangen"),
        "mast-outer": text("Outer mast (tilts)", "Außenmast (neigbar)"),
        "mast-inner": text("Inner mast stage", "Innenmast-Stufe"),
        "carriage": text("Fork carriage with load backrest", "Gabelträger mit Lastschutzgitter"),
        "forks": text("Fork arms", "Gabelzinken"),
        "pallet-x": text("Pallet position, horizontal (no mass)", "Palettenlage horizontal (ohne Masse)"),
        "pallet-y": text("Pallet position, vertical (no mass)", "Palettenlage vertikal (ohne Masse)"),
        "pallet": text("Euro pallet with goods (no mass)", "Europalette mit Ladegut (ohne Masse)"),
    }
    spec = [
        ("chassis", None, 2.441, {"type": "fixed"}),
        ("battery", "chassis", 0.711, {"type": "fixed"}),
        ("counterweight", "chassis", 0.616, {"type": "fixed"}),
        ("axle-f", "chassis", K.RF, {"type": "fixed"}),
        ("axle-r", "chassis", K.RR, {"type": "fixed"}),
        ("wheel-f", "axle-f", K.RF, {"type": "revolute"}),
        ("wheel-r", "axle-r", K.RR, {"type": "revolute"}),
        ("tilt-cyl", "chassis", K.TILT_BARREL, {"type": "revolute", "minDeg": -5, "maxDeg": 7}),
        ("tilt-rod", "tilt-cyl", K.TILT_ROD, {"type": "prismatic", "axisDeg": 0, "minM": -0.09, "maxM": 0.07}),
        ("mast-outer", "chassis", K.MAST_TOP_OUTER - K.PIVOT[1], {"type": "revolute", "minDeg": -6, "maxDeg": 8}),
        ("mast-inner", "mast-outer", K.MAST_TOP_INNER - 0.16, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": K.INNER_MAX}),
        ("carriage", "mast-inner", 0.43, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": K.CARRIAGE_MAX}),
        ("forks", "carriage", K.FORK_L, {"type": "fixed"}),
        ("pallet-x", "chassis", 1.2, {"type": "prismatic", "axisDeg": 0, "minM": -1.2, "maxM": 1.2}),
        ("pallet-y", "pallet-x", 1.2, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 3.2}),
        ("pallet", "pallet-y", 1.2, {"type": "revolute", "minDeg": -8, "maxDeg": 10}),
    ]
    result = []
    for id, parent, length, joint in spec:
        ox, oy, angle = K.REST[id]
        if parent is None:
            x, y, a = ox, oy, angle
        else:
            px, py, pa = K.REST[parent]
            x, y = K.rot(ox - px, oy - py, -pa)
            a = angle - pa
        bone = {"id": id, "parent": parent, "title": titles[id], "x": K.r5(x), "y": K.r5(y), "angleDeg": K.r5(a), "lengthM": K.r5(length), "joint": joint}
        if id in MASSES:
            kg, mx, my = MASSES[id]
            bone["mass"] = mass(K, id, kg, mx, my)
        result.append(bone)
    return result


def axle_loads(K, load=0.0, load_x=0.0):
    total = sum(m[0] for m in MASSES.values()) + load
    moment = sum(m[0] * m[1] for m in MASSES.values()) + load * load_x
    rear = -moment / K.WB
    return total - rear, rear


def anchors(K, old):
    place = {
        "contact-f-l": ("axle-f", 0.0, 0.0, K.ZF), "contact-f-r": ("axle-f", 0.0, 0.0, -K.ZF), "contact-r-l": ("axle-r", -K.WB, 0.0, K.ZR), "contact-r-r": ("axle-r", -K.WB, 0.0, -K.ZR),
        "motor-vent": ("counterweight", K.REAR - 0.006, 0.66, None), "fork-load": ("forks", K.FACE + 0.6, K.FORK_TOP + 0.084 + 0.45, None),
    }
    result = []
    for anchor in old:
        entry = copy.deepcopy(anchor)
        bone, x, y, z = place[anchor["id"]]
        u, v = K.to_local(bone, x, y)
        entry["bone"], entry["x"], entry["y"] = bone, K.r5(u), K.r5(v)
        if z is not None:
            entry["z"] = z
        if anchor["id"].startswith("contact-f"):
            entry["footprintLengthM"], entry["footprintWidthM"] = 0.15, 0.18
        elif anchor["id"].startswith("contact-r"):
            entry["footprintLengthM"], entry["footprintWidthM"] = 0.12, 0.14
        if anchor["id"] == "fork-load":
            entry["title"] = text("Centre of the pallet load, 600 mm from the fork face and 450 mm above the pallet deck", "Schwerpunkt der Palettenlast, 600 mm vor der Gabelfläche und 450 mm über dem Palettendeck")
        if anchor["id"] == "motor-vent":
            entry["title"] = text("Cooling air outlet grille in the rear of the counterweight", "Kühlluft-Austrittsgitter im Heck des Gegengewichts")
        result.append(entry)
    return result


POSES = {
    "lowered-forks": (0.0, 0.0, False, 0.0, 0.0),
    "lifted-load": (3.0, 2.4, True, 0.0, 0.0),
    "tilted-mast": (8.0, 0.3, True, 0.0, 0.0),
    "approach": (0.0, 0.0, False, -0.97, 0.97),
    "travel-accelerated": (8.0, 0.3, True, 1.2, 0.0),
    "travel-cruised": (8.0, 0.3, True, 4.8, 0.0),
    "travel-stopped": (8.0, 0.3, True, 6.0, 0.0),
}


def pallet_floor(K, tilt, rise):
    inner, carriage = K.lift_split(rise)
    fx, fy, fa = K.fork_frame(tilt, inner, carriage)
    ox, oy = K.rot(0.005, -K.STRINGER_LOW, fa)
    return fy + oy


def poses(K, old):
    words = {
        "lowered-forks": text("Standing with the mast upright and the forks fully lowered, 20 mm above the floor, inside the fork openings of a Euro pallet with 1 t of goods that still rests on the ground; the pallet carries its own weight.",
                              "Stillstand mit senkrechtem Mast und ganz abgesenkten Gabeln 20 mm über dem Boden in den Einfahröffnungen einer Europalette mit 1 t Ladegut, die noch auf dem Boden ruht und ihr Gewicht selbst trägt."),
        "lifted-load": text("Duplex ZT mast: carriage 1.275 m and inner stage 1.125 m up, so the forks stand 2.4 m higher; mast tilted 3 degrees back, the 1 t pallet carried: the load moment lowers the rear axle reaction and raises the front one.",
                            "Duplex-ZT-Mast: Gabelträger 1,275 m und Innenmast 1,125 m ausgefahren, die Gabeln stehen 2,4 m höher; Mast 3 Grad zurückgeneigt, die 1-t-Palette getragen: Das Lastmoment senkt die Hinterachsreaktion und erhöht die vordere."),
        "tilted-mast": text("Travel position: the 1 t pallet carried with the forks 0.3 m up and the mast tilted back to its 8 degree stop.",
                            "Fahrstellung: Die 1-t-Palette wird mit 0,3 m angehobenen Gabeln getragen, der Mast ist bis zum Anschlag von 8 Grad zurückgeneigt."),
    }
    result = []
    for pose in old:
        entry = copy.deepcopy(pose)
        tilt, rise, carried, root_x, pallet_dx = POSES[pose["id"]]
        entry["bones"] = K.offsets(tilt, rise, carried=carried, root_x=root_x, pallet_world_dx=pallet_dx)
        if pose["id"] in words:
            entry["description"] = words[pose["id"]]
        result.append(entry)
    return result


def clip_texts(old):
    result = []
    for clip in old:
        entry = copy.deepcopy(clip)
        if clip["id"] == "lift-pallet":
            entry["description"] = text("The truck brakes uniformly over 0.97 m into the pallet, stops, and lifts it 2.4 m (free lift of the carriage, then carriage and inner stage together) while tilting the mast back 3 degrees; the 1 t acts at the load point once the pallet leaves the ground.",
                                        "Der Stapler bremst gleichmäßig auf 0,97 m in die Palette hinein, hält an und hebt sie um 2,4 m (Freihub des Gabelträgers, dann Gabelträger und Innenmast gemeinsam), während der Mast 3 Grad zurückgeneigt wird; die Tonne wirkt am Lastpunkt, sobald die Palette den Boden verlässt.")
        if clip["id"] == "drive-with-load":
            entry["description"] = text("With the 1 t pallet carried 0.3 m up and the mast at its 8 degree back stop, the truck accelerates uniformly to 1.2 m/s over 1.2 m, cruises 3.6 m and brakes uniformly to rest over 1.2 m; the wheels roll with theta = -x / r and the axle reactions follow the d'Alembert forces.",
                                        "Mit der 0,3 m angehobenen 1-t-Palette und dem Mast am 8-Grad-Anschlag beschleunigt der Stapler gleichmäßig auf 1,2 m Strecke auf 1,2 m/s, fährt 3,6 m konstant und bremst gleichmäßig auf 1,2 m bis zum Stillstand; die Räder rollen mit theta = -x / r und die Achsreaktionen folgen den d'Alembert-Kräften.")
        result.append(entry)
    return result


def coarse_pens(scale, geometry):
    ids = {"200": {"body-l", "body-r", "floor-plate"}, "500": {"body-l", "body-r", "fender-l", "fender-r", "pallet", "frame-front"}}
    for shapes in geometry.values():
        for shape in shapes:
            if shape["id"] in ids.get(str(scale), set()):
                shape["penMm"] = 0.21


def apply(asset, K, data):
    asset["summary"] = text("A battery-electric four-wheel counterbalanced forklift of 2.5 t (Jungheinrich EFG 425 class): a Duplex ZT mast with nested channels, two lift cylinders and a free-lifting ISO class 2 carriage with load backrest and 1.15 m forks, two tilt cylinders, an overhead guard, twin-motor drive axle and steer axle with superelastic tyres, the battery under the seat and a cast counterweight, and a Euro pallet with 1 t of goods; the front and rear axle reactions shift with the load moment.",
                            "Ein batterieelektrischer Vierrad-Gegengewichtsstapler mit 2,5 t Tragkraft (Klasse Jungheinrich EFG 425): ein Duplex-ZT-Mast mit geschachtelten Profilen, zwei Hubzylindern und einem freihebenden Gabelträger nach ISO Klasse 2 mit Lastschutzgitter und 1,15-m-Gabeln, zwei Neigezylinder, Fahrerschutzdach, Antriebsachse mit zwei Fahrmotoren und Lenkachse mit Superelastikreifen, die Batterie unter dem Sitz und ein gegossenes Gegengewicht, dazu eine Europalette mit 1 t Ladegut; die Reaktionen von Vorder- und Hinterachse verschieben sich mit dem Lastmoment.")
    refs = [r for r in asset["provenance"]["references"] if r["id"] not in (REFERENCE["id"], EPAL["id"])]
    asset["provenance"]["references"] = refs + [REFERENCE, EPAL]
    asset["provenance"]["notes"] = text("Electric counterbalance forklift after the Jungheinrich EFG 425 data sheet (VDI 2198): no exhaust; waste heat comes from the twin drive motors, their gear and brake housings, the cowl electronics, the battery case and the hydraulic cylinders. Steering is a vertical-axis motion and not a joint of the side view. The chassis bone runs from the rear end of the counterweight to the fork face (length to fork face l2). The pallet chain bones are massless guide bones; pallet and goods are drawn at every scale.",
                                        "Elektro-Gegengewichtsstapler nach dem Datenblatt Jungheinrich EFG 425 (VDI 2198): kein Abgas; Abwärme stammt von den beiden Fahrmotoren, ihren Getriebe- und Bremsgehäusen, der Elektronik in der Armaturenhaube, dem Batteriekasten und den Hydraulikzylindern. Die Lenkung ist eine Bewegung um die Hochachse und kein Gelenk der Seitenansicht. Der Fahrgestellknochen reicht vom Heck des Gegengewichts bis zur Gabelfläche (Länge bis Gabelrücken l2). Die Palettenkette sind masselose Führungsknochen; Palette und Ware sind in jedem Maßstab gezeichnet.")
    asset["dimensions"] = {"heightM": 2.325, "widthM": K.r5(K.FACE + 0.005 + 1.2 - K.REAR), "depthM": 1.2, "massKg": sum(m[0] for m in MASSES.values())}
    asset["rig"]["bones"] = bones(K)
    asset["anchors"] = anchors(K, asset["anchors"])
    asset["poses"] = poses(K, asset["poses"])
    asset["clips"] = clip_texts(asset["clips"])
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] in HIDDEN:
            region["hidden"] = HIDDEN[region["id"]]
    for scale, shapes in data["shapes"].items():
        rep = asset["representations"][scale]
        coarse_pens(scale, shapes)
        rep["geometry"] = shapes
        rep["decisions"] = data["decisions"][scale]
