import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/cantilever-balcony/🔣️.json"
lines = open(P).read().split("\n")
i = [k for k, l in enumerate(lines) if '"id": "thermal-break"' in l][0]
j = [k for k in range(i, i + 14) if '"areaM2": 0.2,' in lines[k]][0]
lines[j] = lines[j].replace("0.2,", "0.17,")
new = "\n".join(lines)
json.loads(new)
open(P, "w").write(new)
