"""🖊️ Side elevation 1:200 of the domestic cat: the per-shape pens of 0.5 to 1.6 mm drowned a figure 3.8 mm long in ink; the shapes draw with the body pen of the scale again."""
import json, re

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/domestic-cat/🔣️.json"
text = open(CATALOG).read()
document = json.loads(text)
for shape in document["asset"]["representations"]["200"]["geometry"]["elevation"]:
    shape.pop("penMm", None)
start = text.index('"elevation": [', text.index('\n      "200": {', text.index('"representations"')))
end = text.index("\n          ]", start)
section, count = re.subn(r'\n\s*"penMm": [0-9.]+,(?=\n)', "", text[start:end])
text = text[:start] + section + text[end:]
if json.loads(text) != document:
    raise SystemExit("spliced text differs from the edited document")
open(CATALOG, "w").write(text)
print("pens removed", count)
