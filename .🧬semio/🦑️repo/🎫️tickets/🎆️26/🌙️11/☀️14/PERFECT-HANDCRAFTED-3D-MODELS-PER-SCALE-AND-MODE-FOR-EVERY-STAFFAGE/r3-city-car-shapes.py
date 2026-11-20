"""📐 Lists the 2D shapes of the city-car per scale and view (members without their geometry, or in full)."""
import glob
import json
import sys

A = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*vehicles/city-car/*.json")[0]
a = json.load(open(A))["asset"]
full = len(sys.argv) > 1 and sys.argv[1] == "full"
only = sys.argv[2] if len(sys.argv) > 2 else None
for sc, rep in a["representations"].items():
    if only and sc != only:
        continue
    print("==", sc, json.dumps({k: v for k, v in rep.items() if k != "geometry"}, ensure_ascii=False)[:1800])
    for view, shapes in rep["geometry"].items():
        print(" ", view, len(shapes))
        for sh in shapes:
            keys = sh if full else {k: v for k, v in sh.items() if k not in ("points", "d")}
            print("    ", json.dumps(keys, ensure_ascii=False)[: 5000 if full else 260])
