"""🥚️ Lists the shapes whose revolved solid authors `lateralScale`, per scale and view: lateral-list.py <id>…"""
import glob, json, sys
for a in sys.argv[1:]:
    d = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{a}/🔣️.json")[0]))["asset"]
    for s, r in d["representations"].items():
        for v, shapes in (r.get("geometry") or {}).items():
            row = [(x.get("bone"), x["id"], x["solid"]["lateralScale"]) for x in shapes if (x.get("solid") or {}).get("lateralScale", 1) != 1]
            if row: print(a, s, v, " ".join(f"{b}/{i}={l}" for b, i, l in row))
