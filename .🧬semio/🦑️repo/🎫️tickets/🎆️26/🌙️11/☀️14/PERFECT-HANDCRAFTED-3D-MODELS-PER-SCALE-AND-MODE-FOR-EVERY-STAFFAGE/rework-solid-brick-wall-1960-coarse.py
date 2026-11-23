"""🧱 Solid brick wall of 1960, coarse scales modelled for their own smallest feature (15 / 30 / 75 mm): courses (1:50) become course bands (1:100), bond bands (1:200) and one homogeneous wall (1:500).

Usage: python3 rework-solid-brick-wall-1960-coarse.py   (rewrites the three coarse scales of `volumes/components/solid-brick-wall-1960.json`; 1:50 stays as it is)
Frame of bone `root`: u through the wall from the room face (0) to the outer face (0.4 m), v up (0 to 1 m), w along the wall (-0.5 to 0.5 m).
Layers: 15 mm lime plaster, 365 mm solid brick in alternate header and stretcher courses (twelve courses per metre), 20 mm mineral render.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "components", "solid-brick-wall-1960.json")

PLASTER = 0.015
LEAF = 0.365
RENDER = 0.02
WALL = PLASTER + LEAF + RENDER
COURSE = round(1 / 12, 9)
BOND = round(1 / 6, 9)
CUT = ["-u", "-v", "+v", "-w", "+w"]

DECISIONS = {
    "100": [
        {"en": "Smallest feature 15 mm: the 10 to 12 mm joints are left out, so every course is one band of 83 mm with its bed joint in it and the single bricks are gone; the 15 mm plaster lies in the 380 mm leaf as in the drawing, the 20 mm render stays the weather skin of its own.", "de": "Kleinstes Merkmal 15 mm: Die 10 bis 12 mm starken Fugen entfallen, jede Schicht ist ein 83-mm-Band samt Lagerfuge und die Einzelsteine sind verschwunden; der 15-mm-Innenputz liegt wie in der Zeichnung im 380-mm-Mauerwerk, der 20-mm-Außenputz bleibt als eigene Wetterhaut."},
        {"en": "Header and stretcher courses alternate as two repeated bands; the thermal list keeps plaster, four brick depth cells and render as layer shells.", "de": "Binder- und Läuferschichten wechseln als zwei wiederholte Bänder; die Thermikliste behält Innenputz, vier Ziegel-Tiefenzellen und Außenputz als Schichtschalen."},
    ],
    "200": [
        {"en": "Smallest feature 30 mm: plaster and render join the brick as in the drawing; the wall is six bond bands of 167 mm, each one header and one stretcher course, over the full 400 mm.", "de": "Kleinstes Merkmal 30 mm: Innen- und Außenputz gehen wie in der Zeichnung im Ziegel auf; die Wand besteht aus sechs Verbandsbändern von 167 mm, je einer Binder- und einer Läuferschicht, über die vollen 400 mm."},
        {"en": "The structure list carries the bearing 365 mm leaf as the same six bond bands; the thermal list keeps the three layers as shells.", "de": "Die Tragwerksliste führt das tragende 365-mm-Mauerwerk als dieselben sechs Verbandsbänder; die Thermikliste behält die drei Schichten als Schalen."},
    ],
    "500": [
        {"en": "Smallest feature 75 mm: the wall is one homogeneous 400 mm body, 0.8 mm on paper, drawn as one poché; courses, joints and layer boundaries are gone.", "de": "Kleinstes Merkmal 75 mm: Die Wand ist ein homogener 400-mm-Körper, 0,8 mm auf dem Papier, als eine Poché gezeichnet; Schichten, Fugen und Schichtgrenzen sind verschwunden."},
        {"en": "The structure list carries the bearing 365 mm leaf as one member; the thermal list keeps the three layers as shells.", "de": "Die Tragwerksliste führt das tragende 365-mm-Mauerwerk als ein Bauteil; die Thermikliste behält die drei Schichten als Schalen."},
    ],
}


def box(identifier, material, tone, size, centre, **more):
    return {"id": identifier, "kind": "box", "bone": "root", "material": material, "role": "body", "tone": tone, "size": size, "transform": {"translation": centre}, **more}


def band(identifier, depth, rise, bottom, faces, count):
    return box(identifier, "solid-brick", 0.5, [depth, rise, 1.0], [round(depth / 2, 9), round(bottom + rise / 2, 9), 0.0], sectionFaces=faces, repeat={"count": count, "translation": [0.0, BOND, 0.0]})


def shell(identifier, material, layer, thickness, start):
    return box(identifier, material, 0.5, [thickness, 1.0, 1.0], [round(start + thickness / 2, 9), 0.5, 0.0], sectionFaces=CUT, region="wall-field", layer=layer)


def leaf_member():
    return box("member-leaf", "solid-brick", 0.5, [LEAF, 1.0, 1.0], [round(PLASTER + LEAF / 2, 9), 0.5, 0.0], member="root", part="member")


def by_id(solids):
    return {solid["id"]: solid for solid in solids}


def tail(volume):
    structure = by_id(volume["structure"])
    return [structure["support-base-fixed"], structure["mass-wall-strip"]]


def cells(volume):
    return [solid for solid in volume["thermal"] if solid["id"].startswith("th-brick-cell")]


def tone_of(volume, identifier):
    return by_id(volume["body"])[identifier]["tone"]


def scale_100(volume):
    brick = tone_of(volume, "header-a-02")
    render = by_id(volume["body"])["render"]
    a = band("course-a", PLASTER + LEAF, COURSE, 0.0, ["-v", "-w", "+w"], 6)
    b = band("course-b", PLASTER + LEAF, COURSE, COURSE, ["+v", "-w", "+w"], 6)
    a["tone"] = b["tone"] = brick
    rest = tail(volume)
    volume["thermal"] = [shell("th-plaster", "lime-plaster", "plaster", PLASTER, 0.0)] + cells(volume) + [{"id": "th-render", "of": "render", "region": "wall-field", "layer": "render", "sectionFaces": CUT}]
    volume["body"] = [a, b, render]
    volume["structure"] = [{"id": "member-course-a", "of": "course-a", "member": "root", "part": "member"}, {"id": "member-course-b", "of": "course-b", "member": "root", "part": "member"}] + rest


def scale_200(volume):
    brick = tone_of(volume, "header-a-02")
    bond = box("bond-band", "solid-brick", brick, [WALL, BOND, 1.0], [WALL / 2, round(BOND / 2, 9), 0.0], sectionFaces=["-v", "+v", "-w", "+w"], repeat={"count": 6, "translation": [0.0, BOND, 0.0]})
    rest = tail(volume)
    volume["thermal"] = [shell("th-plaster", "lime-plaster", "plaster", PLASTER, 0.0)] + cells(volume) + [shell("th-render", "mineral-render", "render", RENDER, PLASTER + LEAF)]
    volume["body"] = [bond]
    bands = box("member-bond-band", "solid-brick", brick, [LEAF, BOND, 1.0], [round(PLASTER + LEAF / 2, 9), round(BOND / 2, 9), 0.0], repeat={"count": 6, "translation": [0.0, BOND, 0.0]}, member="root", part="member")
    volume["structure"] = [bands] + rest


def scale_500(volume):
    brick = tone_of(volume, "header-a-02")
    wall = box("wall", "solid-brick", brick, [WALL, 1.0, 1.0], [WALL / 2, 0.5, 0.0], sectionFaces=["-v", "+v", "-w", "+w"])
    rest = tail(volume)
    volume["thermal"] = [shell("th-plaster", "lime-plaster", "plaster", PLASTER, 0.0)] + cells(volume) + [shell("th-render", "mineral-render", "render", RENDER, PLASTER + LEAF)]
    volume["body"] = [wall]
    volume["structure"] = [leaf_member()] + rest


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
