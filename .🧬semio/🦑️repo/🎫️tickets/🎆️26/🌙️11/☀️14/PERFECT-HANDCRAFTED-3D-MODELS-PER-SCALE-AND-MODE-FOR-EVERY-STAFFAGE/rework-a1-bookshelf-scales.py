"""📚 Hand edit of `furniture/bookshelf`: shelf masses at 1:50, and 1:100 / 1:200 / 1:500 modelled for their own smallest feature (no 5 mm pins, shelves from side to side, typified bearings in the structure list); the 2D elevations of the coarse scales lose their pin strokes.

Usage: python3 rework-a1-bookshelf-scales.py [--catalog]   (--catalog also rewrites the 2D shapes of the catalog asset)
"""
import importlib.util
import json
import sys

SOURCE = "volumes/furniture/bookshelf.json"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/bookshelf/🔣️.json"
SHELVES = [1, 2, 3, 4, 5]
BOARD = "melamine-chipboard"
STEEL = "zinc-plated-steel"
HALF_SPAN, THICKNESS, SHELF_DEPTH, SHELF_CENTRE_Z = 0.382, 0.018, 0.28, -0.005
HOLE_ROW_Z, HOLE_ROW_PITCH, CLEAR_WIDTH = -0.113, 0.224, 0.764
MASS_RADIUS = {"50": 0.012, "100": 0.015, "200": 0.025, "500": 0.04}
BEARING = {"100": 0.015, "200": 0.03}


def half(shelf, side):
    v = THICKNESS / 2 if side == "l" else -THICKNESS / 2
    return {"id": f"shelf-{shelf}-{side}", "kind": "box", "bone": f"shelf-{shelf}-{side}", "material": BOARD, "role": "body", "tone": 0.7,
            "size": [HALF_SPAN, THICKNESS, SHELF_DEPTH], "transform": {"translation": [HALF_SPAN / 2, v, SHELF_CENTRE_Z]}}


def board(shelf):
    return {"id": f"shelf-{shelf}", "kind": "box", "bone": f"shelf-{shelf}", "material": BOARD, "role": "body", "tone": 0.7,
            "size": [CLEAR_WIDTH, THICKNESS, SHELF_DEPTH], "transform": {"translation": [CLEAR_WIDTH / 2, THICKNESS / 2, SHELF_CENTRE_Z]}}


def mass(scale, shelf, side):
    v = THICKNESS / 2 if side == "l" else -THICKNESS / 2
    r = MASS_RADIUS[scale]
    return {"id": f"mass-shelf-{shelf}-{side}", "kind": "ellipsoid", "bone": f"shelf-{shelf}-{side}", "material": BOARD, "role": "detail", "tone": 0.3,
            "radii": [r, r, r], "transform": {"translation": [0.191, v, SHELF_CENTRE_Z]}, "member": f"shelf-{shelf}-{side}", "part": "mass"}


def bearing(scale, shelf, side):
    a = BEARING[scale]
    u = a / 2 if side == "l" else CLEAR_WIDTH - a / 2
    solid = {"id": f"bearing-{shelf}-{side}", "kind": "box", "bone": f"shelf-{shelf}", "material": STEEL, "role": "detail", "tone": 0.25}
    if scale == "100":
        solid |= {"size": [a, a, a], "transform": {"translation": [u, -a / 2, HOLE_ROW_Z]}, "repeat": {"count": 2, "translation": [0, 0, HOLE_ROW_PITCH]}}
    else:
        solid |= {"size": [a, a, HOLE_ROW_PITCH + a], "transform": {"translation": [u, -a / 2, HOLE_ROW_Z + HOLE_ROW_PITCH / 2]}}
    return solid | {"member": f"shelf-{shelf}", "part": "member"}


DECISIONS = {
    "50": None,
    "100": [
        {"en": "Body 1:100 (smallest feature 15 mm): the carcass is its six boards at their true 18 mm, plain boxes without the back groove, the 3 mm hardboard back between them and the kick board set back 30 mm. Each of the five shelves is two plain half boards 382 x 18 x 280 mm from side to side; the 1 mm play, the rounded edge and the 5 mm shelf pins (0.05 mm on paper) do not exist at this scale and are not modelled.",
         "de": "Körper 1:100 (Mindestmaß 15 mm): Der Korpus besteht aus seinen sechs Platten in wahrer Stärke von 18 mm, schlichte Quader ohne Rückwandnut, dazwischen die 3-mm-Hartfaserrückwand und die 30 mm zurückgesetzte Sockelblende. Jeder der fünf Fachböden besteht aus zwei schlichten Halbplatten 382 x 18 x 280 mm von Seite zu Seite; die 1 mm Luft, die gerundete Kante und die 5-mm-Bodenträger (0,05 mm auf dem Papier) gibt es in diesem Maßstab nicht, sie sind nicht modelliert."},
        {"en": "Thermal 1:100 references the boards of each region. Structure 1:100: sides, top, bottom and kick board are the frame and the back its racking brace, a 30 mm block at each of the eight cam fittings is a carcass joint, the ten half boards are the beams, a 15 mm block under each board end at both hole rows is the typified pin bearing, the member of the carrier bone, the floor edges of the sides are the supports, and the masses of the carcass and of every half board sit at their centres.",
         "de": "Die Thermik 1:100 verweist auf die Platten jeder Region. Tragwerk 1:100: Seiten, Deckel, Boden und Sockelblende sind der Rahmen, die Rückwand seine Aussteifung, ein 30-mm-Klotz an jedem der acht Exzenterbeschläge ist eine Korpusverbindung, die zehn Halbplatten sind die Träger, ein 15-mm-Klotz unter jedem Bodenende an beiden Lochreihen ist das typisierte Stiftauflager, das Glied des Trägerknochens, die Bodenkanten der Seiten sind die Auflager, und die Massen von Korpus und jeder Halbplatte sitzen in ihren Schwerpunkten."},
    ],
    "200": [
        {"en": "Body 1:200 (smallest feature 30 mm): the four carcass boards are one frame extruded over the 300 mm depth, with the sides running down to the floor and the 60 mm toe space under the bottom; the 3 mm back and the kick board are true thin sheets. The five shelves are two plain half boards each, from side to side; pins, play and edges are far below the feature and are not modelled.",
         "de": "Körper 1:200 (Mindestmaß 30 mm): Die vier Korpusplatten sind ein Rahmen, über die Tiefe von 300 mm extrudiert, mit den bis zum Boden laufenden Seiten und dem 60 mm hohen Sockelraum unter dem Boden; die 3-mm-Rückwand und die Sockelblende sind echte dünne Platten. Die fünf Fachböden bestehen je aus zwei schlichten Halbplatten von Seite zu Seite; Bodenträger, Luft und Kanten liegen weit unter dem Mindestmaß und sind nicht modelliert."},
        {"en": "Thermal 1:200 references the frame, the back, the kick board and the ten half boards. Structure 1:200: the frame with the back as brace and the kick board, the ten half boards as beams, one 30 mm bearing strip under each board end that stands for the two pins of that side on the carrier bone, the floor edges of the sides as supports, and the masses of the carcass and of every half board at their centres.",
         "de": "Die Thermik 1:200 verweist auf Rahmen, Rückwand, Sockelblende und die zehn Halbplatten. Tragwerk 1:200: der Rahmen mit der Rückwand als Aussteifung und der Sockelblende, die zehn Halbplatten als Träger, eine 30-mm-Auflagerleiste unter jedem Bodenende, die für die zwei Bodenträger dieser Seite am Trägerknochen steht, die Bodenkanten der Seiten als Auflager und die Massen von Korpus und jeder Halbplatte in ihren Schwerpunkten."},
    ],
    "500": [
        {"en": "Body 1:500 (smallest feature 75 mm): the bookcase is 1.6 x 3.6 mm on paper. It stays what it is, an open frame of 18 mm boards with a 3 mm back, a kick board and five shelves as single boards from side to side, all true thin sheets at their true section, because a solid block would hide the compartments that make it a bookshelf; the 5.8 mm sag of a loaded board is 0.01 mm on paper, so the board rides on its carrier bone and does not bend; nothing smaller than a board is modelled.",
         "de": "Körper 1:500 (Mindestmaß 75 mm): Das Regal misst auf dem Papier 1,6 x 3,6 mm. Es bleibt, was es ist: ein offener Rahmen aus 18-mm-Platten mit 3-mm-Rückwand, Sockelblende und fünf Fachböden als ganze Platten von Seite zu Seite, alle echte dünne Platten in wahrem Querschnitt, weil ein Vollkörper die Fächer verdecken würde, die es zum Regal machen; die 5,8 mm Durchbiegung eines beladenen Bodens sind 0,01 mm auf dem Papier, deshalb sitzt die Platte auf ihrem Trägerknochen und biegt sich nicht; nichts Kleineres als eine Platte ist modelliert."},
        {"en": "Thermal 1:500 references the frame, the back, the kick board and the five boards. Structure 1:500: the frame with back and kick board, the five boards as beams bearing directly on the sides (each board is the member of its carrier bone and of its two half-span bones, a bearing would be smaller than the feature), the floor edges of the sides as supports, and the masses of the carcass and of every half board.",
         "de": "Die Thermik 1:500 verweist auf Rahmen, Rückwand, Sockelblende und die fünf Böden. Tragwerk 1:500: der Rahmen mit Rückwand und Sockelblende, die fünf Böden als Träger, die direkt auf den Seiten aufliegen (jeder Boden ist das Glied seines Trägerknochens und seiner zwei Halbfeldknochen, ein Auflager wäre kleiner als das Mindestmaß), die Bodenkanten der Seiten als Auflager und die Massen von Korpus und jeder Halbplatte."},
    ],
}


def layout():
    spec = importlib.util.spec_from_file_location("insert", "volumes-insert.py")
    insert = importlib.util.module_from_spec(spec)
    argv, sys.argv = sys.argv, ["volumes-insert.py"]
    try:
        spec.loader.exec_module(insert)
    except SystemExit:
        pass
    sys.argv = argv
    return insert


def edit_source():
    with open(SOURCE, encoding="utf-8") as handle:
        source = json.load(handle)
    volumes = source["volumes"]
    fine = volumes["50"]
    fine["structure"] = [s for s in fine["structure"] if not s["id"].startswith("mass-shelf-")] + [mass("50", n, side) for n in SHELVES for side in "lr"]
    fine["decisions"][1]["en"] = fine["decisions"][1]["en"].replace("and the carcass mass sits at its centre 0.89 m up.", "the carcass mass sits at its centre 0.89 m up and the mass of every half board at its own centre.").replace("  ", " ")
    fine["decisions"][1]["de"] = fine["decisions"][1]["de"].replace("und die Korpusmasse sitzt in ihrem Schwerpunkt auf 0,89 m.", "die Korpusmasse sitzt in ihrem Schwerpunkt auf 0,89 m und die Masse jeder Halbplatte in ihrem eigenen.")
    for scale in ("100", "200", "500"):
        volume = volumes[scale]
        frame = [s for s in volume["body"] if s["bone"] == "frame"]
        keep = [s for s in volume["structure"] if not any(key in s["id"] for key in ("shelf", "pin", "bearing"))]
        thermal = [s for s in volume["thermal"] if "shelf" not in s["id"]]
        masses = [mass(scale, n, side) for n in SHELVES for side in "lr"]
        if scale == "500":
            volume["body"] = frame + [board(n) for n in SHELVES]
            volume["thermal"] = thermal + [{"id": f"th-shelf-{n}", "of": f"shelf-{n}", "region": "shelves"} for n in SHELVES]
            beams = [{"id": f"m-shelf-{n}", "of": f"shelf-{n}", "member": f"shelf-{n}", "part": "member", "members": [f"shelf-{n}-l", f"shelf-{n}-r"]} for n in SHELVES]
            volume["structure"] = keep + beams + masses
        else:
            volume["body"] = frame + [half(n, side) for n in SHELVES for side in "lr"]
            volume["thermal"] = thermal + [{"id": f"th-shelf-{n}-{side}", "of": f"shelf-{n}-{side}", "region": "shelves"} for n in SHELVES for side in "lr"]
            beams = [{"id": f"m-shelf-{n}-{side}", "of": f"shelf-{n}-{side}", "member": f"shelf-{n}-{side}", "part": "member"} for n in SHELVES for side in "lr"]
            volume["structure"] = keep + beams + [bearing(scale, n, side) for n in SHELVES for side in "lr"] + masses
        volume["decisions"] = DECISIONS[scale]
    with open(SOURCE, "w", encoding="utf-8") as handle:
        handle.write(layout().dump(source, 0) + "\n")


def edit_catalog():
    insert = layout()
    with open(CATALOG, encoding="utf-8") as handle:
        text = handle.read()
    document = json.loads(text)
    start, end = insert.member_range(text, "    ", "representations")

    def member(value):
        return '    "representations": ' + insert.dump(value, 2)

    if member(document["asset"]["representations"]) != text[start:end]:
        raise SystemExit("representations are not in the layout of volumes-insert.py; edit the shapes by hand")
    for scale in ("100", "200", "500"):
        geometry = document["asset"]["representations"][scale]["geometry"]
        shapes = [s for s in geometry["elevation"] if not s["id"].startswith("pin-")]
        for shape in shapes:
            if shape["id"].startswith("shelf-") and shape["kind"] == "rect":
                shape["x"], shape["w"] = 0, HALF_SPAN
        if scale == "500":
            shapes = [s for s in shapes if not s["id"].endswith("-r")]
            for shape in shapes:
                if shape["id"].startswith("shelf-") and shape["id"].endswith("-l"):
                    shape["id"] = shape["id"][:-2]
                    shape["bone"], shape["w"] = shape["id"], CLEAR_WIDTH
            for shape in geometry["front"]:
                if shape["id"].startswith("shelf-"):
                    shape["bone"] = shape["id"]
        geometry["elevation"] = shapes
    result = text[:start] + member(document["asset"]["representations"]) + text[end:]
    if json.loads(result)["asset"]["representations"] != document["asset"]["representations"]:
        raise SystemExit("splice did not round-trip")
    with open(CATALOG, encoding="utf-8") as handle:
        if handle.read() != text:
            raise SystemExit("catalog changed while editing; retry")
    with open(CATALOG, "w", encoding="utf-8") as handle:
        handle.write(result)


edit_source()
if "--catalog" in sys.argv:
    edit_catalog()
