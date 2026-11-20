"""🔍 Lists the 2D shapes of the excavator per scale and view (id, bone, role, kind, fill, z, solid hint, geometry head). Usage: python3 r3-excavator-info.py [asset path] > file."""
import glob, json, sys

A = sys.argv[1] if len(sys.argv) > 1 else glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*machinery/excavator/*.json")[0]
a = json.load(open(A))["asset"]
for sc, rep in a["representations"].items():
    for view, shapes in rep["geometry"].items():
        print("=====", sc, view, len(shapes))
        for s in shapes:
            g = {k: v for k, v in s.items() if k not in ("id", "bone", "role", "kind", "filled", "z", "tone", "material", "region", "solid")}
            print(s["id"], "|", s["bone"], s["role"], s["kind"], "F" if s["filled"] else "-", "z", s["z"], "t", s["tone"], s.get("region", ""), "| solid", json.dumps(s.get("solid")), "|", json.dumps(g, ensure_ascii=False)[:int(sys.argv[2]) if len(sys.argv) > 2 else 400])
    print("decisions", json.dumps(rep["decisions"], ensure_ascii=False))
