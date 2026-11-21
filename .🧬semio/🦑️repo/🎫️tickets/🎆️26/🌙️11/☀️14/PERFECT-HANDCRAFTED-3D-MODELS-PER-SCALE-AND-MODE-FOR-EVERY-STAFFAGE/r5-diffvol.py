"""🔍 Round 5: lists the solids of a volumes source that differ from the catalog asset's spliced volumes (per scale and list).

Usage: python3 r5-diffvol.py <category/slug> <volumes json>
"""
import glob
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CAT = os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")


def main():
    category, slug = sys.argv[1].split("/")
    path = glob.glob(os.path.join(CAT, "*" + category, slug, "🔣️.json"))[0]
    have = json.load(open(path, encoding="utf-8"))["asset"]["volumes"]
    want = json.load(open(sys.argv[2], encoding="utf-8"))["volumes"]
    for scale in want:
        for lst in ("body", "thermal", "structure", "decisions"):
            a, b = have[scale].get(lst, []), want[scale].get(lst, [])
            if lst == "decisions":
                if a != b:
                    print(scale, lst, "differs")
                continue
            ia, ib = {s["id"]: s for s in a}, {s["id"]: s for s in b}
            changed = sorted(i for i in set(ia) | set(ib) if ia.get(i) != ib.get(i))
            if changed:
                print(scale, lst, len(a), "->", len(b), changed)


if __name__ == "__main__":
    main()
