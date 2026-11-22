"""🔩 Second-round chisel of `appliances/kitchen-extractor-hood`: closes the hem of the rim, makes the stressed skin the structure, joins the 1:100 motor to its wheel and binds the 2D blower outlines to the canopy.

Usage: python3 rework-kitchen-extractor-hood-pass2.py source|root
`source` edits `volumes/appliances/kitchen-extractor-hood.json` in place (idempotent); `root` re-reads the catalog file and rewrites its `representations`.
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/appliances/kitchen-extractor-hood.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/kitchen-extractor-hood/🔣️.json"
WHEEL_U, WHEEL_V = 0.105, 0.135
UPSTAND = {"id": "rim-upstand", "bone": "canopy", "material": "stainless-steel", "role": "body", "tone": 0.72, "kind": "extrude", "profile": [[-0.438, 0.012], [0.438, 0.012], [0.438, 0.488], [-0.438, 0.488]], "plane": "zx", "from": 0.0006, "to": 0.006, "holes": [[[-0.4374, 0.0126], [0.4374, 0.0126], [0.4374, 0.4874], [-0.4374, 0.4874]]]}
SHROUD = {"id": "wheel-inlet-ring", "bone": "blower-wheel", "material": "galvanised-steel", "role": "body", "tone": 0.43, "kind": "extrude", "profile": [[round(0.067 * math.cos(math.pi * k / 24), 5), round(0.067 * math.sin(math.pi * k / 24), 5)] for k in range(48)], "plane": "yz", "from": -0.034, "to": -0.0325, "holes": [[[round(0.051 * math.cos(math.pi * k / 24), 5), round(0.051 * math.sin(math.pi * k / 24), 5)] for k in range(48)]], "sheet": True}
MOTOR_100 = [[0.034, 0.0], [0.034, 0.02], [0.04, 0.02], [0.04, 0.04], [0.094, 0.04], [0.094, 0.0]]
SKIN = {"50": ["slope-front", "slope-left", "slope-right"], "100": ["slope-front", "slope-left", "slope-right"], "200": ["pyramid-front", "pyramid-left", "pyramid-right"], "500": ["pyramid-front", "pyramid-left", "pyramid-right"]}
STRUCTURE_TEXT = {
    "50": ("Structure: the load path of a hung sheet box, a stressed skin. The back sheet hangs on two keyhole screws and bears on two spacer pads; the three sloping sheets, folded to each other along the two hips, carry rim band and top plate as a pyramid shell; the chimney channels stand on the top plate (seat joint), slide in each other (telescope joint) and are held by the wall bracket. The masses are the scroll housing, the motor and the transition themselves, the wheel on its bone and the two chimney channels.", "Tragwerk: der Lastpfad eines aufgehängten Blechkastens, eine tragende Haut. Das Rückblech hängt an zwei Schlüssellochschrauben und stützt sich auf zwei Distanzpuffer; die drei geneigten Bleche, an den zwei Graten miteinander gekantet, tragen Randband und Deckplatte als Pyramidenschale; die Kaminschächte stehen auf der Deckplatte (Sitzfuge), stecken ineinander (Teleskopfuge) und werden vom Wandwinkel gehalten. Die Massen sind Spiralgehäuse, Motor und Übergangsstück selbst, das Laufrad auf seinem Knochen und die zwei Kaminschächte."),
    "100": ("Structure: back plate on two screws and two pads, rim frame, top plate and the three sloping plates of 15 mm as the stressed skin (the hips are their folds, not separate bars), the chimney channels with their seat on the top plate and the wall bracket; the masses are the blower housing and the motor themselves, the wheel drum and the two chimney channels. The two backdraft leaves are posed bones and stay as half-octagon plates of 15 mm inside the duct.", "Tragwerk: Rückplatte an zwei Schrauben und zwei Puffern, Randrahmen, Deckplatte und die drei geneigten Platten von 15 mm als tragende Haut (die Grate sind ihre Kantungen, keine eigenen Stäbe), die Kaminschächte mit ihrem Sitz auf der Deckplatte und der Wandwinkel; die Massen sind Gebläsegehäuse und Motor selbst, die Laufradtrommel und die zwei Kaminschächte. Die zwei Rückstauklappen sind gestellte Knochen und bleiben als Halbachteckplatten von 15 mm im Rohr."),
    "200": ("Structure: back plate and top plate of 30 mm, the rim frame and the three pyramid wedges as the stiff canopy body that carries rim and top plate to the back plate, both chimney blocks, the two screw and pad supports as bars on the wall, and two masses where they are: the fan unit as an ellipsoid at the blower, 70 mm left of the centre line, and the wheel on its bone.", "Tragwerk: Rück- und Deckplatte von 30 mm, der Randrahmen und die drei Pyramidenkeile als steifer Haubenkörper, der Rand und Deckplatte an die Rückplatte bringt, beide Kaminblöcke, die zwei Schrauben- und Pufferauflager als Stäbe an der Wand und zwei Massen an ihrem Ort: die Lüftereinheit als Ellipsoid am Gebläse, 70 mm links der Mittellinie, und das Laufrad auf seinem Knochen."),
    "500": ("Structure: a hanger angle (back sheet and top plate), the rim frame, the three pyramid wedges as the stiff canopy body, both chimney channels, two wall supports and the fan unit and wheel masses at the blower, 70 mm left of the centre line.", "Tragwerk: ein Hängewinkel (Rückblech und Deckplatte), der Randrahmen, die drei Pyramidenkeile als steifer Haubenkörper, beide Kaminschächte, zwei Wandauflager und die Massen von Lüftereinheit und Laufrad am Gebläse, 70 mm links der Mittellinie."),
}
HEM_TEXT = ("Underside, 6 mm above the rim edge: the 12 mm return flange of the rim is folded up 5.4 mm to a hem that carries the base panel with three grease filter cassettes of 0.28 m by 0.32 m (frame, mesh pack, latch) and two lamp modules of 0.12 m by 0.05 m in the front strip. The switch panel with four buttons sits on the front rim.", "Unterseite, 6 mm über der Randkante: der 12 mm breite Umbug des Rands ist 5,4 mm hochgekantet und trägt als Falz das Bodenblech mit drei Fettfilterkassetten von 0,28 m mal 0,32 m (Rahmen, Gitterpaket, Riegel) und zwei Lampenmodulen von 0,12 m mal 0,05 m im vorderen Streifen. Die Schalterblende mit vier Tasten sitzt am vorderen Rand.")


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth):
    """🖨️ The catalog file format (same writer as `volumes-insert.py` and `rework-kitchen-extractor-hood-model.py`)."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def insert_after(items, after, item):
    """📌 Puts `item` behind the entry `after`, replacing an earlier copy."""
    kept = [entry for entry in items if entry["id"] != item["id"]]
    index = next(i for i, entry in enumerate(kept) if entry["id"] == after)
    return kept[: index + 1] + [item] + kept[index + 1:]


def source():
    """🪚 The volume edits."""
    document = json.load(open(SOURCE))
    volumes = document["volumes"]
    fifty = volumes["50"]
    fifty["body"] = insert_after(fifty["body"], "rim-return", UPSTAND)
    fifty["thermal"] = insert_after(fifty["thermal"], "th-rim-flange", {"id": "th-rim-upstand", "of": "rim-upstand", "region": "canopy", "layer": "shell"})
    fifty["decisions"][1] = {"en": HEM_TEXT[0], "de": HEM_TEXT[1]}
    fifty["body"] = [SHROUD if entry["id"] == "wheel-inlet-ring" else entry for entry in fifty["body"]]
    next(entry for entry in volumes["100"]["body"] if entry["id"] == "blower-motor")["profile"] = MOTOR_100
    for scale, skins in SKIN.items():
        structure = [entry for entry in volumes[scale]["structure"] if not entry["id"].startswith("member-hip-") and not entry["id"].startswith("member-skin-")]
        index = next(i for i, entry in enumerate(structure) if entry["id"] == "member-rim")
        members = [{"id": "member-skin-" + skin.split("-")[1], "of": skin, "member": "canopy", "part": "member"} for skin in skins]
        volumes[scale]["structure"] = structure[: index + 1] + members + structure[index + 1:]
        decisions = volumes[scale]["decisions"]
        at = next(i for i, entry in enumerate(decisions) if entry["en"].startswith("Structure:"))
        decisions[at] = {"en": STRUCTURE_TEXT[scale][0], "de": STRUCTURE_TEXT[scale][1]}
    open(SOURCE, "w").write(dump(document, 0) + "\n")
    print("source written", SOURCE)


def root():
    """📐 The 2D blower outlines leave the spinning wheel bone: a turned part looks the same at every angle, so they are drawn on the canopy."""
    raw = open(CATALOG).read()
    document = json.loads(raw)
    representations = document["asset"]["representations"]
    for representation in representations.values():
        for shape in representation.get("geometry", {}).get("elevation", []):
            if shape.get("bone") == "blower-wheel":
                assert shape["kind"] == "rect"
                shape["bone"] = "canopy"
                shape["x"] = round(shape["x"] + WHEEL_U, 4)
                shape["y"] = round(shape["y"] + WHEEL_V, 4)
    key = '\n    "representations": '
    start = raw.index(key) + len(key)
    depth, index, quoted = 0, start, False
    while True:
        char = raw[index]
        if quoted:
            index += 1 if char == "\\" else 0
            quoted = char != '"'
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                break
        index += 1
    raw = raw[:start] + dump(representations, 2) + raw[index + 1:]
    assert json.loads(raw)["asset"]["representations"] == representations
    open(CATALOG, "w").write(raw)
    print("root written", CATALOG)


if __name__ == "__main__":
    {"source": source, "root": root}[sys.argv[1]]()
