import sys, os
# usage: author-57-setarea.py category slug region new   (root edit of thermal.regions[].areaM2, logged)
cat, slug, region, new = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4])
base = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
d = [x for x in os.listdir(base) if x.endswith(cat)][0]
path = f"{base}/{d}/{slug}/🔣️.json"
t = open(path).read()
i = t.index('"thermal": {'); j = t.index(f'"id": "{region}",', t.index('"regions": [', i)); k = t.index('"areaM2":', j); e = t.index(",", k)
old = t[k + 10:e].strip()
open(path, "w").write(t[:k] + f'"areaM2": {new}' + t[e:])
open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-57-dimfix.log"), "a").write(f"{cat}/{slug} {region} areaM2 {old} -> {new}\n")
