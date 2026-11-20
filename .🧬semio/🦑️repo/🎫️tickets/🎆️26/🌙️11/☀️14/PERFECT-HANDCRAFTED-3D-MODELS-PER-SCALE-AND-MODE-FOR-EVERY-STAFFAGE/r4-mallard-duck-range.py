"""📐 Round 4 mallard: narrows the valid range of the 1:200 drawing from 1:350 to 1:340 in the catalog file and in the redraw members (`r3-mallard-duck-2d.json`): the bill is 0.07 m wide in plan, exactly 0.200 mm at 1:350, which the print rule counts as below the 0.2 mm minimum of the office laser profile. Only that number changes; the catalog file is re-read right before writing."""
import glob
import os

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/mallard-duck/🔣️.json")[0]
for path, old, new in (
    (CATALOG, '"minDenominator": 150,\n          "maxDenominator": 350', '"minDenominator": 150,\n          "maxDenominator": 340'),
    (os.path.join(T, "r3-mallard-duck-2d.json"), '"minDenominator":150,"maxDenominator":350', '"minDenominator":150,"maxDenominator":340'),
):
    text = open(path, encoding="utf-8").read()
    if text.count(old) != 1:
        print("unchanged", path[-40:], text.count(old), text.count(new))
        continue
    open(path, "w", encoding="utf-8").write(text.replace(old, new))
    print("narrowed", path[-40:])
