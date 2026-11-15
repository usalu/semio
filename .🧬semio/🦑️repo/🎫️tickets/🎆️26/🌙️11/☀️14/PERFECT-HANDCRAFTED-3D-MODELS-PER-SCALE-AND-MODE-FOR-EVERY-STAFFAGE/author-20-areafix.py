import sys, json, re
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/%s/%s/🔣️.json"
cat, slug = sys.argv[1], sys.argv[2]
fixes = dict(a.split("=") for a in sys.argv[3:])
p = CAT % (cat, slug)
t = open(p, encoding="utf-8").read()
start = t.index('"thermal": {')
log = []
for rid, v in fixes.items():
    i = t.index('"id": "%s",' % rid, start)
    m = re.compile(r'"areaM2": ([0-9.eE+-]+)').search(t, i)
    log.append((rid, m.group(1), v))
    t = t[:m.start(1)] + v + t[m.end(1):]
json.loads(t)
open(p, "w", encoding="utf-8").write(t)
print(slug, log)
