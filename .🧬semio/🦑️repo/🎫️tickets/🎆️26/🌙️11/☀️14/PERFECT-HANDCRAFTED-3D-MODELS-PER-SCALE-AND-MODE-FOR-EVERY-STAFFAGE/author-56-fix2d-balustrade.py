import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/glass-balustrade/🔣️.json"
lines = open(P).read().split("\n")
n = 0
for name, old, new in (("handrail-cap-left", '"cx": -1,', '"cx": -0.996,'), ("handrail-cap-right", '"cx": 1,', '"cx": 0.996,')):
    i = [k for k, l in enumerate(lines) if f'"id": "{name}"' in l][0]
    j = [k for k in range(i, i + 14) if lines[k].strip() == old][0]
    lines[j] = lines[j].replace(old, new)
    n += 1
new_text = "\n".join(lines)
json.loads(new_text)
open(P, "w").write(new_text)
print("patched", n)
