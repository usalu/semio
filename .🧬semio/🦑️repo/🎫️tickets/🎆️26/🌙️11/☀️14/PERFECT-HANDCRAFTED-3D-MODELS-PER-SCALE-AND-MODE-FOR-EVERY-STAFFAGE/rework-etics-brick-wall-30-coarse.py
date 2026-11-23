"""🧥 Brick wall with 300 mm ETICS, coarse scales modelled for their own smallest feature (15 / 30 / 75 mm): courses (1:50) become course bands behind insulation boards and one render coat (1:100), bond bands behind one ETICS layer in two board courses (1:200) and one homogeneous wall (1:500).

Usage: python3 rework-etics-brick-wall-30-coarse.py   (rewrites the three coarse scales of `volumes/components/etics-brick-wall-30.json`; 1:50 stays as it is)
Frame of bone `root`: u through the wall from the room face (0) outwards, v up (0 to 1 m), w along the wall (-0.5 to 0.5 m).
Layers: 15 mm lime plaster, 365 mm solid brick (twelve courses per metre), 10 mm adhesive, 300 mm mineral wool, 8 mm base coat, 12 mm finishing render.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "components", "etics-brick-wall-30.json")

PLASTER = 0.015
LEAF = 0.365
ADHESIVE = 0.01
WOOL = 0.3
BASE_COAT = 0.008
FINISH = 0.012
MASONRY = round(PLASTER + LEAF, 9)
WOOL_START = round(MASONRY + ADHESIVE, 9)
RENDER_START = round(WOOL_START + WOOL, 9)
WALL = round(RENDER_START + BASE_COAT + FINISH, 9)
COURSE = round(1 / 12, 9)
BOND = round(1 / 6, 9)
CUT = ["-v", "+v", "-w", "+w"]
MM = {"wool": round(WOOL * 1000), "board": round((ADHESIVE + WOOL) * 1000), "etics": round((WALL - MASONRY) * 1000), "wall": round(WALL * 1000)}

DECISIONS = {
    "100": [
        {"en": f"Smallest feature 15 mm: the 10 to 12 mm masonry joints are left out, so every course is one 83 mm band with its bed joint in it; the 15 mm plaster lies in the 380 mm leaf as in the drawing; the 10 mm adhesive lies in the {MM['board']} mm insulation boards; base coat and finishing render are one 20 mm coat; the anchors are left out of the body.", "de": f"Kleinstes Merkmal 15 mm: Die 10 bis 12 mm starken Mauerfugen entfallen, jede Schicht ist ein 83-mm-Band samt Lagerfuge; der 15-mm-Innenputz liegt wie in der Zeichnung im 380-mm-Mauerwerk; der 10-mm-Kleber liegt in den {MM['board']} mm starken Dämmplatten; Armierungs- und Oberputz sind eine 20-mm-Schicht; die Dübel entfallen im Körper."},
        {"en": "Two boards of the lower board course and the upper course stay apart; the structure list keeps the four anchors as joints of the bearing leaf, the thermal list every layer as a shell.", "de": "Zwei Platten der unteren Plattenreihe und die obere Reihe bleiben getrennt; die Tragwerksliste behält die vier Dübel als Verbindungen des tragenden Mauerwerks, die Thermikliste jede Schicht als Schale."},
        {"en": "Minimal: the structure list is the bearing leaf, its support and the four anchors, as at 1:50; it holds nothing a coarser reading could merge before the anchors themselves go at 1:200.", "de": "Minimal: Die Tragwerksliste besteht wie bei 1:50 aus dem tragenden Mauerwerk, seinem Auflager und den vier Dübeln; sie enthält nichts, was sich zusammenfassen ließe, bevor bei 1:200 die Dübel selbst entfallen."},
    ],
    "200": [
        {"en": f"Smallest feature 30 mm: the masonry is six bond bands of 167 mm (one header and one stretcher course each) with the plaster in them; adhesive, wool and render are one {MM['etics']} mm ETICS layer in its two board courses, as the drawing shows the two fields.", "de": f"Kleinstes Merkmal 30 mm: Das Mauerwerk besteht aus sechs Verbandsbändern von 167 mm (je eine Binder- und eine Läuferschicht) samt Innenputz; Kleber, Dämmung und Putz sind eine {MM['etics']} mm starke WDVS-Schicht in ihren zwei Plattenreihen, wie die Zeichnung die zwei Felder zeigt."},
        {"en": "The structure list keeps the bearing 365 mm leaf on its support, without the anchors (8 mm shafts, 60 mm plates); the thermal list keeps every layer as a shell.", "de": "Die Tragwerksliste behält das tragende 365-mm-Mauerwerk auf seinem Auflager, ohne die Dübel (8-mm-Schäfte, 60-mm-Teller); die Thermikliste behält jede Schicht als Schale."},
    ],
    "500": [
        {"en": f"Smallest feature 75 mm: the wall is one homogeneous {MM['wall']} mm body drawn as one poché; courses, joints, boards, anchors and layer boundaries are gone.", "de": f"Kleinstes Merkmal 75 mm: Die Wand ist ein homogener {MM['wall']}-mm-Körper, als eine Poché gezeichnet; Schichten, Fugen, Platten, Dübel und Schichtgrenzen sind verschwunden."},
        {"en": "Minimal: the structure list is one bearing 365 mm leaf, its support and its mass, as at 1:200; the thermal list keeps every layer as a shell.", "de": "Minimal: Die Tragwerksliste besteht wie bei 1:200 aus einem tragenden 365-mm-Mauerwerk, seinem Auflager und seiner Masse; die Thermikliste behält jede Schicht als Schale."},
    ],
}


def box(identifier, material, tone, size, centre, **more):
    return {"id": identifier, "kind": "box", "bone": "root", "material": material, "role": "body", "tone": tone, "size": size, "transform": {"translation": centre}, **more}


def shell(identifier, material, layer, thickness, start, faces):
    return box(identifier, material, 0.5, [thickness, 1.0, 1.0], [round(start + thickness / 2, 9), 0.5, 0.0], sectionFaces=faces, region="wall-field", layer=layer)


def by_id(solids):
    return {solid["id"]: solid for solid in solids}


def deepened(solid, start, depth):
    size, centre = solid["size"], solid["transform"]["translation"]
    return {**solid, "size": [depth, size[1], size[2]], "transform": {"translation": [round(start + depth / 2, 9), centre[1], centre[2]]}}


def thermal(volume):
    wool = [solid for solid in volume["thermal"] if solid.get("layer") == "insulation"]
    brick = by_id(volume["thermal"])["th-brick"]
    return [
        shell("th-plaster", "lime-plaster", "plaster", PLASTER, 0.0, ["-u"] + CUT),
        brick,
        shell("th-adhesive", "etics-adhesive", "adhesive", ADHESIVE, MASONRY, CUT),
        *wool,
        shell("th-render-base-coat", "mineral-render", "base-coat", BASE_COAT, RENDER_START, CUT),
        shell("th-face-render", "mineral-render", "render", FINISH, round(RENDER_START + BASE_COAT, 9), ["-u"] + CUT),
    ]


def scale_100(volume):
    body = by_id(volume["body"])
    brick, render = body["header-a-02"]["tone"], body["render-finish"]["tone"]
    a = box("course-a", "solid-brick", brick, [MASONRY, COURSE, 1.0], [MASONRY / 2, round(COURSE / 2, 9), 0.0], sectionFaces=["-v", "-w", "+w"], repeat={"count": 6, "translation": [0.0, BOND, 0.0]})
    b = box("course-b", "solid-brick", brick, [MASONRY, COURSE, 1.0], [MASONRY / 2, round(COURSE * 1.5, 9), 0.0], sectionFaces=["+v", "-w", "+w"], repeat={"count": 6, "translation": [0.0, BOND, 0.0]})
    boards = [deepened(body[name], MASONRY, round(ADHESIVE + WOOL, 9)) for name in ("wool-board-low-left", "wool-board-low-right", "insulation-upper-course")]
    coat = box("render", "mineral-render", render, [round(BASE_COAT + FINISH, 9), 1.0, 1.0], [round(RENDER_START + (BASE_COAT + FINISH) / 2, 9), 0.5, 0.0], sectionFaces=CUT)
    volume["thermal"] = thermal(volume)
    volume["body"] = [a, b, *boards, coat]


def scale_200(volume):
    body = by_id(volume["body"])
    brick = body["header-a-02"]["tone"]
    bond = box("bond-band", "solid-brick", brick, [MASONRY, BOND, 1.0], [MASONRY / 2, round(BOND / 2, 9), 0.0], sectionFaces=CUT, repeat={"count": 6, "translation": [0.0, BOND, 0.0]})
    etics = [deepened(body[name], MASONRY, round(WALL - MASONRY, 9)) for name in ("insulation-lower-course", "insulation-upper-course")]
    volume["thermal"] = thermal(volume)
    volume["body"] = [bond, *etics]
    volume["structure"] = [solid for solid in volume["structure"] if solid["part"] != "joint"]


def scale_500(volume):
    body = by_id(volume["body"])
    brick = body["header-a-02"]["tone"]
    volume["thermal"] = thermal(volume)
    volume["body"] = [box("wall", "solid-brick", brick, [WALL, 1.0, 1.0], [WALL / 2, 0.5, 0.0], sectionFaces=CUT)]
    volume["structure"] = [solid for solid in volume["structure"] if solid["part"] != "joint"]


def main():
    document = json.load(open(SOURCE, encoding="utf-8"))
    volumes = document["volumes"]
    if "header-a-02" not in by_id(volumes["100"]["body"]):
        raise SystemExit("already applied: the coarse scales no longer carry the 1:50 parts list")
    for scale, build in (("100", scale_100), ("200", scale_200), ("500", scale_500)):
        build(volumes[scale])
        volumes[scale]["decisions"] = DECISIONS[scale]
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    for scale in ("100", "200", "500"):
        print(scale, {name: len(volumes[scale][name]) for name in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
