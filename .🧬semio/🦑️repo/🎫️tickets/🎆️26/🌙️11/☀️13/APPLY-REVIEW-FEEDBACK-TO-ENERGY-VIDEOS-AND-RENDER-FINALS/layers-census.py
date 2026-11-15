import glob
import json
import sys

sys.stdout.reconfigure(encoding="utf-8")
files = sorted(glob.glob("C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/videos/ui/h/*.json"))
seen = set()
for path in files:
    data = json.loads(open(path, encoding="utf-8").read())
    if "hierarchy" not in data or "layers" not in data:
        continue
    key = tuple(data["hierarchy"]["opened_path"])
    if key in seen:
        continue
    seen.add(key)
    print(key)
    for layer in data["layers"]:
        print("  ", layer["id"], layer.get("interactive"), [c["id"] for c in layer["cards"]])
    if len(seen) > 6:
        break
