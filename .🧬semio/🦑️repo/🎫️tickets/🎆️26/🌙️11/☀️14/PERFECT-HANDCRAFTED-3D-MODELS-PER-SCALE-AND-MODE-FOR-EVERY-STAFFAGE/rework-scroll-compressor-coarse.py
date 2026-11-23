"""🌀 machinery/scroll-compressor: 1:100 and 1:200 modelled for their smallest feature, and two thermal areas corrected to the surface the model has.

1:100 (15 mm): the foot plate is 15 mm thick under the shell (14.5 to 29.5 mm), the 12.9 mm spacer sleeves inside the grommets are left to 1:50, the reed valve with its stop is a 15 mm disc, the orbiting plate is 15 mm thick, and the Oldham ring is the hoop it is (8 x 7.5 mm section, `slender`).
1:200 (30 mm) and 1:500: the feet are the one foot block the drawing shows (ground to 29.5 mm, outline of the plate with its four lugs) with the four grommets standing in it; sleeves and bolts are merged into it.
Root (`--root <asset document json>`, text edit): `thermal.regions.feet.areaM2` 0.106 -> 0.072 (the plate lies under the 241 mm shell, only lugs, rim, underside and grommets meet the air; measured 0.0717 / 0.0724 m2 at 1:50 / 1:100) and `discharge.areaM2` 0.0044 -> 0.004 (measured 0.0037 m2: a 12.7 mm stub of 32 mm on its boss).
One-shot on `volumes/machinery/scroll-compressor.json`; publish with `volumes-insert.py`.
"""
import json
import os
import sys

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "machinery", "scroll-compressor.json")
DECISIONS = {
    "100": {"en": "Modelled for 15 mm: foot plate 15 mm thick, reed valve and stop as one 15 mm disc, orbiting plate 15 mm, the Oldham ring as a slender hoop at its true section; the spacer sleeves inside the grommets are left to 1:50.", "de": "Für 15 mm modelliert: Fußplatte 15 mm dick, Zungenventil und Anschlag als eine 15-mm-Scheibe, Orbiterplatte 15 mm, der Oldham-Ring als schlanker Reif mit wahrem Querschnitt; die Distanzhülsen in den Gummipuffern bleiben dem Maßstab 1:50."},
    "500": {"en": "The feet are one foot block from the ground to the shell with the four grommets in its corners; sleeves and bolts are merged into it.", "de": "Die Füße sind ein Fußblock vom Boden bis zum Gehäuse mit den vier Gummipuffern in seinen Ecken; Hülsen und Schrauben gehen in ihm auf."},
    "200": {"en": "Modelled for 30 mm: the feet are one foot block from the ground to the shell with the outline of the plate and its four lugs, the grommets stand in its corners; sleeves and bolts are merged into it.", "de": "Für 30 mm modelliert: Die Füße sind ein Fußblock vom Boden bis zum Gehäuse mit dem Umriss der Platte und ihrer vier Laschen, die Gummipuffer stehen in seinen Ecken; Hülsen und Schrauben gehen in ihm auf."},
}


def drop(volumes, ids):
    """✂️ Removes body solids and everything that refers to them."""
    for name in ("body", "thermal", "structure"):
        volumes[name] = [solid for solid in volumes[name] if solid["id"] not in ids and solid.get("of") not in ids]


def root(path):
    """📐 Corrects the two declared areas in an asset document, leaving every other byte alone."""
    with open(path, encoding="utf8") as f:
        text = f.read()
    for old, new in (('"areaM2": 0.106,', '"areaM2": 0.072,'), ('"areaM2": 0.0044,', '"areaM2": 0.004,')):
        if text.count(old) != 1:
            raise SystemExit(f"{old}: expected once (already applied?)")
        text = text.replace(old, new)
    with open(path, "w", encoding="utf8") as f:
        f.write(text)
    print("root applied to", path)


def main():
    """▶️ Applies the hand-chosen numbers to 1:100 and 1:200."""
    if len(sys.argv) == 3 and sys.argv[1] == "--root":
        return root(sys.argv[2])
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    volumes = document["volumes"]

    medium = volumes["100"]
    drop(medium, ("mount-sleeves",))
    for name in ("body", "thermal"):
        for solid in medium[name]:
            ident = solid["id"].removeprefix("th-")
            if ident == "base-plate" and "from" in solid:
                solid["from"] = 0.0145
            elif ident == "discharge-valve":
                solid["profile"] = [[0.2915, 0], [0.2915, 0.014], [0.3065, 0.014], [0.3065, 0]]
            elif ident == "orbiting-scroll-plate":
                solid["profile"] = [[-0.003, 0], [-0.003, 0.075], [0.012, 0.075], [0.012, 0]]
            elif ident == "oldham-ring":
                solid["slender"] = True

    for scale in ("200", "500"):
        coarse = volumes[scale]
        drop(coarse, ("mount-sleeves", "mount-bolts"))
        next(solid for solid in coarse["body"] if solid["id"] == "base-plate")["from"] = 0.0

    for scale, sentence in DECISIONS.items():
        if sentence not in volumes[scale]["decisions"]:
            volumes[scale]["decisions"].append(sentence)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1)
    print("applied")


if __name__ == "__main__":
    main()
