import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/glass-balustrade/🔣️.json"
lines = open(P).read().split("\n")
i = [k for k, l in enumerate(lines) if '"id": "handrail-steel"' in l][0]
j = [k for k in range(i, i + 14) if '"areaM2": 0.13,' in lines[k]][0]
lines[j] = lines[j].replace("0.13,", "0.195,")
new = "\n".join(lines)
json.loads(new)
open(P, "w").write(new)
