import glob
import json
import sys

C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
for slug in sys.argv[1:]:
    found = glob.glob(f"{C}/*/{slug}/🔣️.json")
    if not found:
        print(slug, "MISSING")
        continue
    d = json.load(open(found[0], encoding="utf8"))
    a = d.get("asset", d)
    poses = [x.get("id") for x in a.get("poses", [])]
    clips = [(x.get("id"), x.get("durationS")) for x in a.get("clips", [])]
    print(slug, "poses", poses, "clips", clips)
