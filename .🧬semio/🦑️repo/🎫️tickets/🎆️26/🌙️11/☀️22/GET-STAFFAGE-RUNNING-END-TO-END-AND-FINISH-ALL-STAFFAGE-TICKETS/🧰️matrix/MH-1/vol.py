"""🧱️ Lists the body solids of the volume of bones of an asset per scale with their kind and size: `python3 vol.py <id|file.json> <scale|all> <bone>…` (a bone `*` lists every bone)."""
import json, sys, glob
arg, scale, *bones = sys.argv[1:]
path = arg if arg.endswith(".json") else glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{arg}/🔣️.json")[0]
a = json.load(open(path))["asset"]
for key, vol in a["volumes"].items():
    if vol["status"] != "authored" or (scale != "all" and key != scale): continue
    for s in vol["body"]:
        if "*" in bones or s.get("bone") in bones:
            rest = {k: v for k, v in s.items() if k not in ("id", "kind", "bone", "material", "role", "tone", "sections", "points", "profile")}
            n = len(s.get("sections", s.get("points", s.get("profile", []))))
            print(f"1:{key} vol {s.get('bone')}/{s['id']} {s['kind']} n{n} {json.dumps(rest)}"[:300])
