"""🔄 Round 4 group G04: copies the scales that the catalog asset carries but the ticket source still marks `unavailable` into `volumes/animals/<slug>.json`, so the r3 tools render what is spliced. Usage: python3 r4-g04-sync.py <slug>..."""
import glob
import json
import sys

S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
for slug in sys.argv[1:]:
    asset = json.load(open(glob.glob(f"{S}/*/{slug}/🔣️.json")[0], encoding="utf-8"))
    asset = asset.get("asset", asset)
    path = f"volumes/animals/{slug}.json"
    source = json.load(open(path, encoding="utf-8"))
    for scale, entry in asset["volumes"].items():
        if source["volumes"].get(scale, {}).get("status") != "authored" and entry.get("status") == "authored":
            source["volumes"][scale] = entry
            print(slug, "took scale", scale, "from the catalog")
    json.dump(source, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
