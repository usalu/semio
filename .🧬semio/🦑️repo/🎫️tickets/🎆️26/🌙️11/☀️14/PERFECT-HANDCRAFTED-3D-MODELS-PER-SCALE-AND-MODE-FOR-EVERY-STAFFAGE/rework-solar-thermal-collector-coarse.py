"""☀️ machinery/solar-thermal-collector: 1:200 and 1:500 modelled for their scale instead of carrying the 1:50 parts list, 1:100 completed, and the collector box carried by its cleats in every structure list.

1:200 and 1:500: the stand of 25 mm angle sections is a stand of 30 mm square bars on the same axes (rails, posts, knees) with 30 mm cleats; the two 2 x 20 mm cross straps stay as slender straps; their bolts are merged into the posts. The collector is the folded tray between two side walls of 30 mm (1:500: 75 mm) that take in the end trims and riser stubs, the absorber a flat 2 mm sheet that runs into the two header tubes, the headers 22 mm tubes of 1 m (slender), the glass pane. 1:500 also leaves the 58 mm insulation mat to the box.
1:100: brace bolts and riser stubs are 15 mm, the embossed absorber is the declared sheet it is.
Structure, every scale: the tray is a member of the housing, so the box rests on the four cleats (the perimeter frame alone hung 22 to 45 mm beside them).
One-shot on `volumes/machinery/solar-thermal-collector.json`; publish with `volumes-insert.py`.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "machinery", "solar-thermal-collector.json")
GONE = ("bolt-brace-rising-left", "bolt-brace-rising-right", "cover-end-upper", "cover-end-lower", "riser-end-upper", "riser-end-lower")
BARS = {
    "rail-left": [[0.0, 0.42], [0.03, 0.42], [0.03, 0.45], [0.0, 0.45]],
    "rail-right": [[0.0, -0.45], [0.03, -0.45], [0.03, -0.42], [0.0, -0.42]],
    "post-rear-left": [[0.42, 0.1821], [0.45, 0.1821], [0.45, 0.2121], [0.42, 0.2121]],
    "post-rear-right": [[-0.45, 0.1821], [-0.42, 0.1821], [-0.42, 0.2121], [-0.45, 0.2121]],
    "post-front-left": [[0.42, 1.2306], [0.45, 1.2306], [0.45, 1.2606], [0.42, 1.2606]],
    "post-front-right": [[-0.45, 1.2306], [-0.42, 1.2306], [-0.42, 1.2606], [-0.45, 1.2606]],
    "knee-left": [[-0.015, 0.42], [0.015, 0.42], [0.015, 0.45], [-0.015, 0.45]],
    "knee-right": [[-0.015, -0.45], [0.015, -0.45], [0.015, -0.42], [-0.015, -0.42]],
}
WALL = {"200": 0.03, "500": 0.075}
SHEET = [[-0.0061, -0.455], [-0.0041, -0.455], [-0.0041, 0.455], [-0.0061, 0.455]]
DECISIONS = {
    "50": {"en": "Structure: the folded tray is a member of the housing and rests on the four cleats.", "de": "Tragwerk: Die gekantete Wanne ist ein Glied des Gehäuses und liegt auf den vier Laschen."},
    "100": {"en": "Modelled for 15 mm: brace bolts and riser stubs are 15 mm, the embossed absorber is a declared sheet. Structure: the tray is a member and rests on the four cleats.", "de": "Für 15 mm modelliert: Strebenschrauben und Steigrohrstutzen sind 15 mm, der geprägte Absorber ist ein erklärtes Blech. Tragwerk: Die Wanne ist ein Glied und liegt auf den vier Laschen."},
    "200": {"en": "Modelled for 30 mm: the stand of 25 mm angles is drawn as 30 mm square bars with 30 mm cleats, the bolts are merged into the posts; the collector is the tray between two 30 mm side walls that take in end trims and riser stubs, a flat absorber sheet that runs into the two header tubes, the insulation mat and the pane. Structure: the tray is a member and rests on the four cleats.", "de": "Für 30 mm modelliert: Das Gestell aus 25-mm-Winkeln ist als 30-mm-Vierkantstäbe mit 30-mm-Laschen gezeichnet, die Schrauben gehen in den Pfosten auf; der Kollektor ist die Wanne zwischen zwei 30-mm-Seitenwänden, in denen Endleisten und Steigrohrstutzen aufgehen, ein ebenes Absorberblech, das in die zwei Sammelrohre läuft, die Dämmmatte und die Scheibe. Tragwerk: Die Wanne ist ein Glied und liegt auf den vier Laschen."},
    "500": {"en": "The stand is drawn as 30 mm square bars with 30 mm cleats; the collector is the tray between two 75 mm side walls with a flat absorber sheet, the two header tubes and the pane, the 58 mm insulation mat is left to the box. Structure: the tray is a member and rests on the four cleats.", "de": "Das Gestell ist als 30-mm-Vierkantstäbe mit 30-mm-Laschen gezeichnet; der Kollektor ist die Wanne zwischen zwei 75-mm-Seitenwänden mit ebenem Absorberblech, den zwei Sammelrohren und der Scheibe, die 58-mm-Dämmmatte bleibt dem Kasten. Tragwerk: Die Wanne ist ein Glied und liegt auf den vier Laschen."},
}


def drop(volumes, ids):
    """✂️ Removes body solids and everything that refers to them."""
    for name in ("body", "thermal", "structure"):
        volumes[name] = [solid for solid in volumes[name] if solid["id"] not in ids and solid.get("of") not in ids]


def main():
    """▶️ Applies the hand-chosen numbers to every scale."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    volumes = document["volumes"]

    for solid in volumes["100"]["body"]:
        if solid["id"].startswith("bolt-brace-rising-"):
            solid["profile"] = [[0.0, 0.0075], [0.016, 0.0075]]
        elif solid["id"].startswith("riser-end-"):
            solid["profile"] = [[0.0, 0.0075], [0.015, 0.0075]]
        elif solid["id"] == "absorber":
            solid["sheet"] = True

    for scale in ("200", "500"):
        v = volumes[scale]
        drop(v, GONE + (("insulation",) if scale == "500" else ()))
        for solid in v["body"]:
            ident = solid["id"]
            if ident in BARS:
                solid["profile"] = BARS[ident]
                solid.pop("slender", None)
            elif ident in ("cleat-rear", "cleat-front"):
                solid.update({"from": -0.45, "to": -0.42})
                solid["repeat"]["translation"] = [0.0, 0.0, 0.87]
            elif ident in ("tray-side-left", "tray-side-right"):
                inner = round(0.4985 - WALL[scale], 4)
                side = 1 if ident.endswith("left") else -1
                solid["profile"] = [[0.0, side * inner], [0.0953, side * inner], [0.0953, side * 0.4985], [0.0, side * 0.4985]][::side]
            elif ident == "absorber":
                solid.update({"profile": SHEET, "from": -0.015, "to": 1.895, "sheet": True})
            elif ident in ("header-upper", "header-lower"):
                solid["slender"] = True

    for scale, sentence in DECISIONS.items():
        v = volumes[scale]
        if all(solid["id"] != "st-tray" for solid in v["structure"]):
            v["structure"].insert(1, {"id": "st-tray", "of": "tray", "member": "housing", "part": "member"})
        if sentence not in v["decisions"]:
            v["decisions"].append(sentence)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1)
    print("applied")


if __name__ == "__main__":
    main()
