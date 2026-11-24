"""🔦️ Prints drawn shapes of a view and body solids of a scale whose id matches a regular expression: python3 dump.py <id> <scale> <view> <regex> [width]"""
import glob, json, re, sys

ident, scale, view, rx = sys.argv[1:5]
width = int(sys.argv[5]) if len(sys.argv) > 5 else 700
asset = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{ident}/🔣️.json")[0]))["asset"]
for s in asset["representations"][scale]["geometry"][view]:
    if re.search(rx, s["id"]):
        print("SHAPE", json.dumps(s, ensure_ascii=False)[:width])
for s in asset["volumes"][scale]["body"]:
    if re.search(rx, s["id"]):
        print("SOLID", json.dumps(s, ensure_ascii=False)[:width])
