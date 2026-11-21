"""🏥 Round 5, furniture/operating-table: the round 3 table and surgical light with the structural masses made compact, written as the candidate document (🗑️generated/r5-operating-table/🔣️.json) and as the volumes source (volumes/furniture/operating-table.json).

Change against round 3: every bone declares a point mass (`distribution: point`), but round 3 drew the masses of the chassis, the table top, the stand base and the lamp head as flat discs up to 0.6 x 0.3 m that read as plates of the load path. Each mass is now a compact spheroid at the same centre: half width 0.011 m x kg^(1/3) (base 120 kg 54 mm, column 42 kg 38 mm, top 48 kg 40 mm, stand base 18 kg 29 mm, lamp head 9 kg 23 mm), never wider than the round 3 disc, the vertical half axis kept at the round 3 value that fits inside the part (the scale's half feature at 1:200 and 1:500). Slender bones keep their spindles along the tube.

Usage: python3 r5-operating-table-build.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/operating-table/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r5-operating-table")
KG = {"base": 120, "column": 42, "top": 48, "stand-base": 18, "stand-column": 6, "stand-arm": 5, "light-head": 9}


def module(name):
    """📦 Loads one of the round 3 sibling scripts."""
    spec = importlib.util.spec_from_file_location("r3_operating_table_" + name, os.path.join(HERE, f"r3-operating-table-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def patch(model):
    """⚖ The round 3 `lump` with a compact spheroid: horizontal half axes min(round 3, max(0.011 kg^(1/3), vertical half axis))."""
    old = model.lump

    def lump(bone, x, y, radii):
        """⚖ The point mass of a bone as a compact spheroid at its centre of mass."""
        rx, ry, rz = radii
        h = max(0.011 * KG[bone] ** (1 / 3), ry)
        return old(bone, x, y, (min(rx, h), ry, min(rz, h)))

    model.lump = lump


def candidate():
    """🧩 The catalog document with the round 3 roots and the round 5 volumes."""
    model, roots, draw = (module(name) for name in ("model", "roots", "draw"))
    patch(model)
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    roots.apply(asset)
    parts = {}
    for scale in ("50", "100", "200", "500"):
        asset["volumes"][scale], parts[scale] = model.volume(scale)
    draw.apply(asset, parts)
    return document


def main():
    document = candidate()
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    volumes = document["asset"]["volumes"]
    json.dump({"volumes": volumes}, open(os.path.join(HERE, "volumes", "furniture", "operating-table.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("candidate", {k: [len(v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})


if __name__ == "__main__":
    main()
