"""📥 Takes the catalog's `volumes["500"]` of an asset into its authoring source so that the round-3 sheets can show it (the starting state of the round-4 review; the handcrafted list replaces it)."""
import glob
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
S = os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
for asset in sys.argv[1:]:
    category, slug = asset.split("/")
    path = glob.glob(os.path.join(S, "*" + category, slug, "🔣️.json"))[0]
    cat = json.load(open(path, encoding="utf-8"))["asset"]["volumes"]
    source_path = os.path.join(T, "volumes", asset + ".json")
    source = json.load(open(source_path, encoding="utf-8"))
    source["volumes"]["500"] = cat["500"]
    json.dump(source, open(source_path + ".r4.tmp", "w", encoding="utf-8"), ensure_ascii=False)
    os.replace(source_path + ".r4.tmp", source_path)
    print(asset, "500 taken", len(cat["500"]["body"]))
