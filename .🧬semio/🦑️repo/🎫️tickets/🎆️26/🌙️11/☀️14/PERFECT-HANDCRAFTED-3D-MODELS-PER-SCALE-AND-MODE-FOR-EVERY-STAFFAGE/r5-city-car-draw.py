"""🖊️ Writes the round 5 drawings of the city-car (`representations[scale].geometry` of every scale, nothing else) into a candidate asset document (default, with the volumes source) or into the catalog asset (--catalog). Re-reads the asset right before writing and keeps the file's formatting."""
import glob
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
from r5_city_car_2d import drawing

spec = importlib.util.spec_from_file_location("roots", os.path.join(T, "r3-city-car-roots.py"))
roots = importlib.util.module_from_spec(spec)
spec.loader.exec_module(roots)

ASSET = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*vehicles/city-car/*.json")[0]
text = open(ASSET, encoding="utf-8").read()
asset = json.loads(text)["asset"]
for scale in ("50", "100", "200", "500"):
    asset["representations"][scale]["geometry"] = drawing(int(scale))
text = roots.replace(text, "representations", asset["representations"])
json.loads(text)
if "--catalog" in sys.argv:
    open(ASSET, "w", encoding="utf-8").write(text)
    print("catalog written")
else:
    out = os.path.join(T, "🗑️generated", "r5-city-car", "🔣️.json")
    document = json.loads(text)
    document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "vehicles", "city-car.json"), encoding="utf-8"))["volumes"]
    json.dump(document, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(out)
