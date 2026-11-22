"""🔩 Hand edits of components/steel-column (second round, pass 1): carrier travel limited to the head plate, the 1:100 plates, gussets and studs at their true 20 / 16 mm, the 1:200 plates at the 30 mm feature instead of the drawn 50 mm, and the stray seam across the web of the 1:50 elevation removed. `--candidate PATH` writes a whole asset document for `r3-sheet.ts --file` / `r3-validate.ts --file`; `--apply` writes the source only; the same rig and drawing edits are made by hand in the catalog file, whose formatting stays untouched."""
import json
import sys
from pathlib import Path

T = Path(__file__).parent
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/steel-column/🔣️.json")
SOURCE = T / "volumes/components/steel-column.json"
PLATE = 0.02
GUSSET_HALF = 0.008
PLATE_200 = 0.03
BODY_200 = {
    "en": ("between the drawn 50 mm base and head plates of 300 x 260 and 240 x 220 mm.", "between base and head plates of 300 x 260 and 240 x 220 mm that are 30 mm thick, the feature (20 mm real; only the drawing thickens them to 50 mm so that they print)."),
    "de": ("zwischen den gezeichneten 50-mm-Fuß- und Kopfplatten von 300 x 260 und 240 x 220 mm.", "zwischen Fuß- und Kopfplatte von 300 x 260 und 240 x 220 mm, die 30 mm dick sind, das Merkmal (wirklich 20 mm; nur die Zeichnung verdickt sie auf 50 mm, damit sie drucken)."),
}
TRUE_PLATES_AT_100 = "--true-100" in sys.argv
BODY_100 = {
    "en": "At 1:100 the shaft is one H extrusion with the real 15 mm flanges and 9 mm web; each 18 mm root fillet is one 45 degree chamfer. The plates keep their true 20 mm (above the 15 mm feature; only the drawing thickens them to 35 mm so that they still print at 1:150), without holes: an 18 mm hole is 0.18 mm on paper. The gussets are the true 16 mm plates, trapezoids of the true 50 mm width with the true 25 mm nose up to the true height of 0.11 m, the top widened from 10 to 15 mm, without snipe. Each anchor is one 24 mm stud from the plate to the true bolt end 75 mm above the footing (washer, nut and rod end merged). The welds (8.5 mm) are below the 15 mm feature and left out.",
    "de": "Bei 1:100 ist der Schaft ein H-Strang mit den wirklichen 15-mm-Flanschen und dem 9-mm-Steg; jede 18-mm-Ausrundung ist eine 45-Grad-Fase. Die Platten behalten ihre wahren 20 mm (über dem 15-mm-Merkmal; nur die Zeichnung verdickt sie auf 35 mm, damit sie auch bei 1:150 drucken), ohne Löcher: ein 18-mm-Loch misst auf dem Papier 0,18 mm. Die Rippen sind die wahren 16-mm-Bleche, Trapeze der wahren Breite von 50 mm mit der wahren 25-mm-Nase bis zur wahren Höhe von 0,11 m, die Oberkante von 10 auf 15 mm verbreitert, ohne Eckausschnitt. Jeder Anker ist ein 24-mm-Bolzen von der Platte bis zum wahren Bolzenende 75 mm über dem Fundament (Scheibe, Mutter und Stangenende zusammengefasst). Die Nähte (8,5 mm) liegen unter dem 15-mm-Merkmal und entfallen.",
}
DRAWING_100 = {
    "en": "The model keeps the true 20 mm plates and 16 mm gussets; the drawn plates are 15 mm thicker, one model feature of this scale.",
    "de": "Das Modell behält die wahren 20-mm-Platten und 16-mm-Rippen; die gezeichneten Platten sind 15 mm dicker, ein Modellmerkmal dieses Maßstabs.",
}


def solid(solids, identifier):
    return next(item for item in solids if item["id"] == identifier)


def edit_rig(asset):
    joint = solid(asset["rig"]["bones"], "eccentric-carrier")["joint"]
    joint["minM"], joint["maxM"] = -0.1, 0.1


def edit_elevation_50(asset):
    shapes = asset["representations"]["50"]["geometry"]["elevation"]
    solid(shapes, "web")["points"] = [[PLATE, 0.085], [PLATE, -0.085], [2.98, -0.085], [2.98, 0.085]]
    shapes[:] = [shape for shape in shapes if shape["id"] != "web-foot"]


def edit_volume_100(volume):
    body, thermal, structure = volume["body"], volume["thermal"], volume["structure"]
    shaft = solid(body, "shaft")
    shaft["from"], shaft["to"] = PLATE, 3 - PLATE
    base = solid(body, "base-plate")
    base["size"], base["transform"] = [PLATE, 0.3, 0.26], {"translation": [PLATE / 2, 0, 0]}
    cap = solid(body, "cap-plate")
    cap["size"], cap["transform"] = [PLATE, 0.24, 0.22], {"translation": [3 - PLATE / 2, 0, 0]}
    for name, side in (("stiffener-a", 1), ("stiffener-b", -1)):
        gusset = solid(body, name)
        gusset["profile"] = [[PLATE, side * 0.1], [PLATE, side * 0.15], [0.045, side * 0.15], [0.11, side * 0.115], [0.11, side * 0.1]]
        gusset["from"], gusset["to"] = -GUSSET_HALF, GUSSET_HALF
    for index in range(1, 5):
        solid(body, f"anchor-{index}")["profile"] = [[PLATE, 0], [PLATE, 0.012], [0.075, 0.012], [0.075, 0]]
    for item in thermal:
        if "from" in item and "to" in item:
            item["from"], item["to"] = PLATE, 3 - PLATE
    solid(structure, "joint-load-pad")["profile"] = [[-PLATE, 0], [-PLATE, 0.018], [0.005, 0.018], [0.005, 0]]
    volume["decisions"][0] = BODY_100


def edit_volume_200(volume):
    body, thermal, structure = volume["body"], volume["thermal"], volume["structure"]
    shaft = solid(body, "shaft")
    shaft["from"], shaft["to"] = PLATE_200, 3 - PLATE_200
    solid(body, "base-plate").update(size=[PLATE_200, 0.3, 0.26], transform={"translation": [PLATE_200 / 2, 0, 0]})
    solid(body, "cap-plate").update(size=[PLATE_200, 0.24, 0.22], transform={"translation": [3 - PLATE_200 / 2, 0, 0]})
    for item in thermal:
        if "from" in item and "to" in item:
            item["from"], item["to"] = PLATE_200, 3 - PLATE_200
    solid(structure, "joint-load-pad")["profile"] = [[-PLATE_200, 0], [-PLATE_200, 0.018], [0.008, 0.018], [0.008, 0]]
    for language, (old, new) in BODY_200.items():
        text = volume["decisions"][0][language]
        assert old in text or new in text, language
        volume["decisions"][0][language] = text.replace(old, new)


def edit_drawing_100(asset):
    thin = asset["representations"]["100"]["decisions"]["thinElements"]
    for language, sentence in DRAWING_100.items():
        if sentence not in thin[language]:
            thin[language] = thin[language] + " " + sentence


def main():
    document = json.loads(CATALOG.read_text())
    source = json.loads(SOURCE.read_text())
    asset = document["asset"]
    edit_rig(asset)
    edit_elevation_50(asset)
    if TRUE_PLATES_AT_100:
        edit_volume_100(source["volumes"]["100"])
        edit_drawing_100(asset)
        edit_volume_200(source["volumes"]["200"])
    if "--candidate" in sys.argv:
        asset["volumes"] = source["volumes"]
        Path(sys.argv[sys.argv.index("--candidate") + 1]).write_text(json.dumps(document, ensure_ascii=False))
    if "--apply" in sys.argv:
        SOURCE.write_text(json.dumps(source, ensure_ascii=False, indent=1) + "\n")


if __name__ == "__main__":
    main()
