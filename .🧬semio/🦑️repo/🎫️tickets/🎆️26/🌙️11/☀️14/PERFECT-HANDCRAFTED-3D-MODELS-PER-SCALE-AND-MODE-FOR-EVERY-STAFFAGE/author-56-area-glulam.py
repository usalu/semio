import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/glulam-beam/🔣️.json"
lines = open(P).read().split("\n")
i = [k for k, l in enumerate(lines) if '"id": "bearing-steel"' in l][0]
j = [k for k in range(i, i + 12) if '"areaM2": 0.04,' in lines[k]][0]
lines[j] = lines[j].replace("0.04,", "0.06,")
new = "\n".join(lines)
json.loads(new)
open(P, "w").write(new)
