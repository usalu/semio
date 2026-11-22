"""🪵 Hand edit of `components/glulam-beam`: the end-grain faces become a thermal region of their own with a storage skin that closes the tube of face skins at every scale, the bearing plates state their total area with the exposed share per pose, and the 2D views are bound to the new region.

Usage: python3 rework-glulam-beam-ends.py candidate | write
`candidate` writes the whole edited document to `🗑️generated/r3-glulam-beam/🔣️.json`; `write` re-reads and writes the catalog file and the volumes source.
"""
import copy
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/glulam-beam/🔣️.json"
SOURCE = os.path.join(T, "volumes/components/glulam-beam.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-glulam-beam/🔣️.json")
REGION = "beam-end-grain"
SKIN = {"50": 0.036, "100": 0.036, "200": 0.036, "500": 0.075}
CAP = {"50": (0.6, 0.0), "100": (0.6, 0.0), "200": (0.6, 0.0), "500": (0.545, 0.0275)}
LENGTH = 8.0



class insert:
    """🧵 The canonical text form of a catalog asset file, as `volumes-insert.py` writes it."""

    @staticmethod
    def scalar(value):
        return not isinstance(value, (dict, list))

    @staticmethod
    def inline(value):
        if isinstance(value, list):
            return all(insert.scalar(item) for item in value) or (
                all(isinstance(item, list) and all(insert.scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
            )
        return False

    @staticmethod
    def dump(value, depth):
        pad, inner = "  " * depth, "  " * (depth + 1)
        if isinstance(value, dict):
            if not value:
                return "{}"
            rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + insert.dump(item, depth + 1) for key, item in value.items()]
            return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
        if isinstance(value, list):
            if not value:
                return "[]"
            if insert.inline(value):
                return json.dumps(value, ensure_ascii=False)
            return "[\n" + ",\n".join(inner + insert.dump(item, depth + 1) for item in value) + "\n" + pad + "]"
        return json.dumps(value, ensure_ascii=False)


def text_of(document):
    """📄 The catalog text form: plain two-space JSON with the `volumes` member in the compact form of `volumes-insert.py`."""
    marker = "@@volumes@@"
    shell = json.loads(json.dumps(document))
    volumes_member = shell["asset"]["volumes"]
    shell["asset"]["volumes"] = marker
    return json.dumps(shell, ensure_ascii=False, indent=2).replace(json.dumps(marker), insert.dump(volumes_member, 2)) + "\n"


def cap(scale, side):
    depth = SKIN[scale]
    height, centre = CAP[scale]
    x = depth / 2 if side == "left" else LENGTH - depth / 2
    return {
        "id": "th-end-grain-" + side,
        "kind": "box",
        "bone": "beam",
        "material": "glulam",
        "role": "body",
        "tone": 0.6,
        "size": [depth, height, 0.16],
        "transform": {"translation": [x, centre, 0.0]},
        "region": REGION,
        "sectionFaces": ["+v", "-v", "+w", "-w", "+u" if side == "left" else "-u"],
    }


def thermal(scale, solids):
    depth = SKIN[scale]
    out = []
    for solid in solids:
        solid = copy.deepcopy(solid)
        if solid["id"].startswith("th-end-grain"):
            continue
        if solid.get("kind") == "extrude" and solid.get("plane") == "yz":
            solid["from"] = max(solid["from"], depth)
            solid["to"] = min(solid["to"], LENGTH - depth)
        out.append(solid)
    plates = [i for i, solid in enumerate(out) if solid["region"] == "bearing-steel"][0]
    out[plates:plates] = [cap(scale, "left"), cap(scale, "right")]
    return out


DECISIONS = {
    "50": {
        "en": "Thermal ends: the two end-grain faces are a region of their own (0.16 x 0.60 m each). Each is a 36 mm storage skin over the full section that closes the tube of the four face skins, which now run between the two end skins (butt joint; the 36 mm rim of an end skin is a section face, so the end region measures exactly its face and the long faces give up 0.9 % of their length).",
        "de": "Thermische Enden: die beiden Hirnholzflächen sind eine eigene Region (je 0,16 x 0,60 m). Jede ist eine 36-mm-Speicherschale über den vollen Querschnitt, die das Rohr der vier Flächenschalen schließt; diese laufen jetzt zwischen den beiden Stirnschalen (stumpfer Stoß; der 36-mm-Rand einer Stirnschale ist Schnittfläche, sodass die Stirnregion genau ihre Fläche misst und die Längsflächen 0,9 % ihrer Länge abgeben).",
    },
    "100": {
        "en": "Thermal ends: the end-grain faces keep their own 36 mm skins over the full section, as at 1:50; the four face skins run between them.",
        "de": "Thermische Enden: die Hirnholzflächen behalten wie bei 1:50 ihre eigenen 36-mm-Schalen über den vollen Querschnitt; die vier Flächenschalen laufen dazwischen.",
    },
    "200": {
        "en": "Thermal ends: a 36 mm end-grain slab over the full section at each end closes the four face prisms, which run between the slabs.",
        "de": "Thermische Enden: eine 36-mm-Hirnholzscheibe über den vollen Querschnitt schließt an jedem Ende die vier Flächenprismen, die zwischen den Scheiben laufen.",
    },
    "500": {
        "en": "Thermal ends: the end grain is a 75 mm slab (the smallest feature) at each end above the bearing block; the four face prisms run between the slabs.",
        "de": "Thermische Enden: das Hirnholz ist an jedem Ende eine 75-mm-Scheibe (kleinste Einzelheit) über dem Lagerblock; die vier Flächenprismen laufen zwischen den Scheiben.",
    },
}


def volumes(source):
    source = copy.deepcopy(source)
    for scale, volume in source.items():
        volume["thermal"] = thermal(scale, volume["thermal"])
        volume["decisions"] = [d for d in volume["decisions"] if not d["en"].startswith("Thermal ends:")] + [DECISIONS[scale]]
    return source


def root(asset):
    regions = [r for r in asset["thermal"]["regions"] if r["id"] != REGION]
    at = [r["id"] for r in regions].index("bearing-steel")
    regions.insert(at, {
        "id": REGION,
        "title": {"en": "Beam end grain", "de": "Balkenhirnholz"},
        "kind": "surface",
        "model": "passive",
        "material": "glulam",
        "areaM2": 0.192,
        "characteristicLengthM": 0.6,
        "geometry": "vertical-plate",
        "skyViewFactor": 0.5,
        "layers": [],
        "heatCapacityJM2K": 24000,
        "source": "authored-estimate",
        "reference": "en-iso-13786",
    })
    asset["thermal"]["regions"] = regions
    note = {
        "en": "The two square-sawn end faces are the end-grain region; it is given the same effective storage depth as the side faces (one conductivity for the material), although heat travels along the grain about twice as easily.",
        "de": "Die beiden rechtwinklig gesägten Stirnflächen bilden die Hirnholzregion; sie erhält dieselbe wirksame Speichertiefe wie die Seitenflächen (eine Leitfähigkeit für das Material), obwohl Wärme längs der Faser etwa doppelt so leicht fließt.",
    }
    notes = [a for a in asset["thermal"]["assumptions"] if not a["en"].startswith("The two square-sawn end faces")]
    notes.insert(2, note)
    asset["thermal"]["assumptions"] = notes
    for pose in asset["poses"]:
        fractions = {}
        for key, value in pose["thermal"]["regions"].items():
            if key == "bearing-steel":
                fractions[REGION] = {"exposedFraction": 1, "sunlitFraction": 0.5}
            if key != REGION:
                fractions[key] = value
        pose["thermal"]["regions"] = fractions
    for region in regions:
        if region["id"] == "bearing-steel":
            region["areaM2"] = 0.0992
            region["areaBasis"] = "total"
    for pose in asset["poses"]:
        pose["thermal"]["regions"]["bearing-steel"]["exposedFraction"] = 0.23
    for scale, representation in asset["representations"].items():
        for shape in representation["geometry"]["elevation"]:
            if shape["id"] == "beam" and REGION not in shape["regions"]:
                at = shape["regions"].index("bearing-steel") if "bearing-steel" in shape["regions"] else len(shape["regions"])
                shape["regions"].insert(at, REGION)
        for shape in representation["geometry"]["front"]:
            if shape["id"] == "beam-end":
                shape["regions"] = [REGION, "bearing-steel"] if scale == "500" else [REGION]
            elif shape.get("region") == "beam-sides":
                shape["region"] = REGION


def main(mode):
    with open(CATALOG, encoding="utf8") as handle:
        text = handle.read()
    document = json.loads(text)
    if text_of(document) != text:
        raise SystemExit("catalog file is not in the canonical dump form; edit it by hand")
    with open(SOURCE, encoding="utf8") as handle:
        source = json.load(handle)
    source["volumes"] = volumes(source["volumes"])
    root(document["asset"])
    document["asset"]["volumes"] = source["volumes"]
    if mode == "candidate":
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        with open(CANDIDATE, "w", encoding="utf8") as handle:
            handle.write(text_of(document))
        print(CANDIDATE)
    elif mode == "write":
        with open(SOURCE, "w", encoding="utf8") as handle:
            json.dump(source, handle, ensure_ascii=False, indent=1)
            handle.write("\n")
        with open(CATALOG, "w", encoding="utf8") as handle:
            handle.write(text_of(document))
        print("written", CATALOG, SOURCE)
    else:
        raise SystemExit("usage: rework-glulam-beam-ends.py candidate | write")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "")
