"""🛵 Round 5 root writer of vehicles/electric-kick-scooter: re-reads the catalog file and replaces only `representations[<scale>].geometry` (shape by shape, keeping the catalog's layer order `z` and list order), `decisions` and `pens` of the scales named on the command line by the drawings the r3 build wrote (`🗑️generated/r3-electric-kick-scooter/drawings.json`); every other member (areas, rig, poses) stays as the catalog has it now.

Usage: python3 r5-electric-kick-scooter-roots.py 50 [100 200] [--apply]   (without --apply: writes the candidate 🗑️generated/r5-electric-kick-scooter/🔣️.json with the volumes source)"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/electric-kick-scooter/🔣️.json"
SRC = os.path.join(T, "volumes", "vehicles", "electric-kick-scooter.json")
DRAWN = os.path.join(T, "🗑️generated", "r3-electric-kick-scooter", "drawings.json")
OUT = os.path.join(T, "🗑️generated", "r5-electric-kick-scooter")


def apply(asset, drawings, scales):
    for key in scales:
        rep, d = asset["representations"][key], drawings[key]
        for view, shapes in d["geometry"].items():
            old = {s["id"]: s for s in rep["geometry"][view]}
            new = {s["id"]: dict(s, z=old[s["id"]]["z"]) if s["id"] in old else s for s in shapes}
            rep["geometry"][view] = [new[i] for i in old if i in new] + [s for i, s in new.items() if i not in old]
        rep["pens"] = d["pens"]
        if d.get("decisions") is not None:
            rep["decisions"] = d["decisions"]
    return asset


if __name__ == "__main__":
    scales = [a for a in sys.argv[1:] if a.isdigit()]
    drawings = json.load(open(DRAWN, encoding="utf8"))
    text = open(ASSET, encoding="utf8").read()
    doc = json.loads(text)
    doc["asset"] = apply(doc["asset"], drawings, scales)
    os.makedirs(OUT, exist_ok=True)
    cand = json.loads(json.dumps(doc))
    cand["asset"]["volumes"] = json.load(open(SRC, encoding="utf8"))["volumes"]
    json.dump(cand, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("candidate", os.path.join(OUT, "🔣️.json"))
    if "--apply" in sys.argv:
        start = text.index('\n  "volumes": {') + 1
        end = text.index('\n  "presentation": {') + 1
        block = text[start:end]
        vol = doc["asset"]["volumes"]
        doc["asset"]["volumes"] = "@@VOLUMES@@"
        out = json.dumps(doc, ensure_ascii=False, indent=1) + "\n"
        out = out.replace('  "volumes": "@@VOLUMES@@",\n', block)
        assert json.loads(out)["asset"]["volumes"] == vol
        open(ASSET, "w", encoding="utf8").write(out)
        print("applied", ASSET)
