"""🔬️ Lists the shapes of bones of an asset per view at a scale with their construction and local extent: `python3 show.py <id|file.json> <scale|all> <bone>…` (a bone `*` lists every bone; `--poses` prints which pose and clip channels turn the bones)."""
import json, sys, glob
arg, scale, *bones = sys.argv[1:]
path = arg if arg.endswith(".json") else glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{arg}/🔣️.json")[0]
a = json.load(open(path))["asset"]
def ext(s):
    if s["kind"] == "poly":
        xs = [p[0] for p in s["points"]]; ys = [p[1] for p in s["points"]]
        return f"x[{min(xs):.3f},{max(xs):.3f}] y[{min(ys):.3f},{max(ys):.3f}] n{len(xs)}"
    return " ".join(f"{k}={s[k]}" for k in ("x", "y", "w", "h", "cx", "cy", "r", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1") if k in s)[:70]
for key, rep in a["representations"].items():
    if rep["status"] != "authored" or (scale != "all" and key != scale): continue
    for view in ("elevation", "front", "plan"):
        for s in rep["geometry"][view]:
            if "*" in bones or s.get("bone") in bones:
                so = s.get("solid"); sd = "flat" if so is None else {k: v for k, v in so.items() if k not in ("weights",)}
                print(f"1:{key} {view[:4]} {s.get('bone')}/{s['id']} {s['kind']} {ext(s)} {json.dumps(sd)}{' followTip' if s.get('followTip') else ''}{' W' if so and 'weights' in so else ''}"[:260])
if "--poses" in sys.argv:
    def walk(n, path):
        if isinstance(n, dict):
            for k, v in n.items():
                if k == "bones" and isinstance(v, dict):
                    for b, o in v.items():
                        if b in bones: print(path, b, o)
                walk(v, path + "/" + str(k) if k in ("poses", "clips") or path.count("/") < 3 else path)
        elif isinstance(n, list):
            for i, v in enumerate(n): walk(v, path + (f"[{v.get('id', i)}]" if isinstance(v, dict) and path.count("[") < 1 else ""))
    walk(a["poses"], "poses"); walk(a["clips"], "clips")
