"""🐿️ Works around the ignored `lateralScale` of revolved 2D shapes (`requests.md`, "three-views part 1 — `lateralScale` of a revolved 2D shape is accepted and ignored") for an asset of batch SAV: every front and plan shape that is a body of revolution about its own bone axis with a `lateralScale` other than 1 gets its profile radius multiplied by that scale and loses the member, so the drawing is as wide laterally as the body solid it draws (the volume is elliptic; the drawn body is then that much too thick along the bone normal, which the front and the plan see only at inclined segment ends). Writes idempotent `shape` ops (computed from the backup of the state before this fixer, or from the catalog file when the backup is gone: on a repaired file nothing is left to widen) at the end of 🗑️generated/silhouette/SAV/ops/<id>.json.
python3 revolved-lateral.py <id> [--fresh] [--only <shape-id-prefix>] [--views front,plan]   (`--fresh` starts the ops of the asset anew)"""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = f"{HERE}/../../🗑️generated/silhouette/SAV"
aid = sys.argv[1]
only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else ""
views = sys.argv[sys.argv.index("--views") + 1].split(",") if "--views" in sys.argv else ["front", "plan"]
import glob
source = f"{GEN}/backup/{aid}.json" if os.path.exists(f"{GEN}/backup/{aid}.json") else glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{aid}/🔣️.json")[0]
asset = json.load(open(source))["asset"]
ops = []
for scale, rep in asset["representations"].items():
    if rep.get("status") != "authored":
        continue
    for view in views:
        for shape in rep["geometry"][view]:
            solid = shape.get("solid")
            if not solid or solid.get("kind") != "revolved" or shape.get("kind") != "poly" or not shape["id"].startswith(only):
                continue
            k = solid.get("lateralScale", 1)
            if abs(k - 1) < 1e-9 or solid.get("axis") != "u" or solid.get("axisBone") != shape["bone"]:
                continue
            new = json.loads(json.dumps(shape))
            new["points"] = [[p[0], round(p[1] * k, 7)] for p in shape["points"]]
            del new["solid"]["lateralScale"]
            ops.append({"op": "shape", "scale": scale, "view": view, "id": shape["id"], "shape": new})
path = f"{GEN}/ops/{aid}.json"
old = [] if "--fresh" in sys.argv or not os.path.exists(path) else json.load(open(path))
sig = lambda o: (o["op"], o.get("scale"), o.get("view"), o.get("id"))
keep = [o for o in old if sig(o) not in {sig(n) for n in ops}]
os.makedirs(os.path.dirname(path), exist_ok=True)
open(path, "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in keep + ops) + "\n]\n")
print(f"{aid}: {len(ops)} revolved shapes widened ({', '.join(sorted({o['scale'] + ' ' + o['view'] for o in ops}))})")
