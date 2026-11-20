"""🔎 Prints the roots of the city-car asset: dimensions, rig, anchors, thermal, structure, 2D shape inventory."""
import json, glob, sys
A = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*vehicles/city-car/*.json")[0]
d = json.load(open(A))
a = d["asset"] if "asset" in d else d
what = sys.argv[1] if len(sys.argv) > 1 else "top"
if what == "top":
    print(list(d.keys()))
    for k, v in a.items():
        print(k, type(v).__name__, len(json.dumps(v)))
    for k in ["dimensions", "provenance", "presentation", "mass", "physics"]:
        if k in a: print(k, json.dumps(a[k], ensure_ascii=False)[:3000])
elif what == "rig":
    print(json.dumps(a["rig"], ensure_ascii=False)[:12000])
elif what == "key":
    v = a
    for p in sys.argv[2].split("."):
        v = v[int(p)] if isinstance(v, list) else v[p]
    s = json.dumps(v, ensure_ascii=False, indent=None)
    print(s[: int(sys.argv[3]) if len(sys.argv) > 3 else 6000])
elif what == "shapes":
    r = a["representations"]
    for sc, rep in r.items():
        print("==", sc, {k: (type(v).__name__) for k, v in rep.items()})
        for view, shapes in rep.get("views", rep).items() if isinstance(rep.get("views", None), dict) else []:
            print(" view", view, len(shapes) if isinstance(shapes, list) else list(shapes.keys()))
