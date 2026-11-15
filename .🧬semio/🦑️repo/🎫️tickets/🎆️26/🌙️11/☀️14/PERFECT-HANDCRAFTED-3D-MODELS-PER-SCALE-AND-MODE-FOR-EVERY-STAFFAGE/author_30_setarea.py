import sys, glob, re, json
cat, slug, region, old, new = sys.argv[1:6]
F = glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*{cat}/{slug}/🔣️.json")[0]
t = open(F, encoding="utf-8").read()
i = t.index(f'"id": "{region}",')
j = t.index('"areaM2":', i)
k = t.index("\n", j)
line = t[j:k]
assert line.strip().rstrip(",") == f'"areaM2": {old}', line
t2 = t[:j] + f'"areaM2": {new},' + t[k:]
json.loads(t2)
open(F, "w", encoding="utf-8").write(t2)
print("set", region, old, "->", new)
