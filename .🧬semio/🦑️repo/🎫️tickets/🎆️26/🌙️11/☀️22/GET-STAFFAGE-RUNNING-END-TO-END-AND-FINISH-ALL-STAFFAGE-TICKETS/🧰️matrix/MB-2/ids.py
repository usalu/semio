"""🪪️ The shape ids of one view per authored band of an asset document: `python3 ids.py <asset.json> <view>`."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]["representations"]
for k, v in a.items():
    if "geometry" in v:
        print("  ", k, [s["id"] for s in v["geometry"][sys.argv[2]]][:9])
