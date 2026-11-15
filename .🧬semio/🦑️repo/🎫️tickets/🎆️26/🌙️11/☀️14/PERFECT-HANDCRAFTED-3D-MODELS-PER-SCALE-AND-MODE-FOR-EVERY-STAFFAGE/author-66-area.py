"""Set areaM2 of thermal regions of one asset (within factor 1.5 only, from the 1:50 model): python3 author-48-area.py <category>/<slug> region=value ..."""
import sys, re, glob, json
cat, slug = sys.argv[1].split("/")
f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*%s/%s/*.json" % (cat, slug))[0]
s = open(f, encoding="utf-8").read()
a = json.loads(s)["asset"]["thermal"]["regions"]
for kv in sys.argv[2:]:
    rid, v = kv.split("=")
    old = [r for r in a if r["id"] == rid][0]["areaM2"]
    assert max(float(v) / old, old / float(v)) <= 1.5, (rid, old, v)
    pat = re.compile(r'("thermal": \{.*?"id": "%s",.*?"areaM2": )([0-9.e+-]+)' % re.escape(rid), re.S)
    m = pat.search(s)
    s = s[:m.start(2)] + v + s[m.end(2):]
    print(rid, old, "->", v)
json.loads(s)
open(f, "w", encoding="utf-8").write(s)
