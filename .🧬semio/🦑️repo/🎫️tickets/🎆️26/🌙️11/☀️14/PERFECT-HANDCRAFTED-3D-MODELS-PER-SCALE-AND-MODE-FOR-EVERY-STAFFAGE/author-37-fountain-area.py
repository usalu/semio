import glob, json, re
f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/drinking-fountain/🔣️.json")[0]
s = open(f, encoding="utf-8").read()
for rid, old, new in (("bubbler-brass", "0.04", "0.047"), ("water-jet", "0.0057", "0.0071")):
    m = re.search(r'"id": "%s",\n(?:(?!"id": ).*\n)*?\s*"areaM2": %s,' % (rid, re.escape(old)), s)
    assert m, rid
    seg = m.group(0)
    s = s.replace(seg, seg.replace('"areaM2": %s,' % old, '"areaM2": %s,' % new), 1)
d = json.loads(s)
print({r["id"]: r["areaM2"] for r in d["asset"]["thermal"]["regions"]})
open(f, "w", encoding="utf-8").write(s)
