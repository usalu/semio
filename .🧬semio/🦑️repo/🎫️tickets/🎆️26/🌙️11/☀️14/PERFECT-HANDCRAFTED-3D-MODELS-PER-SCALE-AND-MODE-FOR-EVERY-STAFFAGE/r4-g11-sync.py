"""🔁 Round 4 group G11: takes the spliced `volumes` of the catalog asset as the ticket source `volumes/humans/<slug>.json`, so later r4 edits start from what the product carries. Usage: python3 r4-g11-sync.py <slug>..."""
import glob
import json
import sys

S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
for slug in sys.argv[1:]:
    asset = json.load(open(glob.glob(f"{S}/*/{slug}/🔣️.json")[0], encoding="utf-8"))["asset"]
    json.dump({"volumes": asset["volumes"]}, open(f"volumes/humans/{slug}.json", "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
    print(slug, "source = catalog")
