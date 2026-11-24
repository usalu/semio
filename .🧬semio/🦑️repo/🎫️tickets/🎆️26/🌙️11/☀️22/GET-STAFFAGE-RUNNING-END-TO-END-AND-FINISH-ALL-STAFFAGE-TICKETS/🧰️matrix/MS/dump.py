"""🗒️ Prints the shapes of one view of one representation of an asset as compact JSON lines: `dump.py <id> <scale> <view> [width]`."""
import json, sys, glob
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
d = json.load(open(glob.glob(f"{S}/*/{sys.argv[1]}/🔣️.json")[0]))["asset"]
w = int(sys.argv[4]) if len(sys.argv) > 4 else 400
for s in d["representations"][sys.argv[2]]["geometry"][sys.argv[3]]:
    print(json.dumps(s, ensure_ascii=False)[:w])
