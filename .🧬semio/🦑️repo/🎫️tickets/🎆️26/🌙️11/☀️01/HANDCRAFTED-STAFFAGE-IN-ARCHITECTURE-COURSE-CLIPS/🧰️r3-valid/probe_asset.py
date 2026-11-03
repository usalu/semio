import glob
import json
import sys

sys.stdout.reconfigure(encoding="utf-8")
root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
path = glob.glob(root + "/*/" + sys.argv[1] + "/🔣️.json")[0]
asset = json.load(open(path, encoding="utf-8"))["asset"]
print(path)
for key, value in asset.items():
    print(key, json.dumps(value, ensure_ascii=False)[:int(sys.argv[2]) if len(sys.argv) > 2 else 260])
