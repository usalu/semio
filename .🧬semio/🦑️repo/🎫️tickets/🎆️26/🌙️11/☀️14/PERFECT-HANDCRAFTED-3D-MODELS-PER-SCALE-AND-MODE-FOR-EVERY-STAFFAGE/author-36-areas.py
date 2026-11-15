"""🧮 Restores committed areaM2 values of one asset's thermal regions (author 36). Usage: python3 author-36-areas.py <slug> id=value ... ; and optionally id=exchange to write areaBasis. Text-level edit of the asset file, other bytes untouched."""
import glob, json, re, sys
slug = sys.argv[1]
P = glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/{slug}/*.json")[0]
t = open(P, encoding="utf8").read()
th = t.index('"thermal": {')
reg = t.index('"regions": [', th)
for arg in sys.argv[2:]:
    rid, val = arg.split("=")
    a = t.index(f'"id": "{rid}",', reg)
    if val == "exchange":
        m = re.compile(r'"areaM2": [0-9.e-]+,\n( *)').search(t, a)
        if '"areaBasis"' in t[a:m.end() + 80]:
            continue
        t = t[: m.end(0)] + '"areaBasis": "exchange",\n' + m.group(1) + t[m.end(0):]
        continue
    m = re.compile(r'"areaM2": [0-9.e-]+').search(t, a)
    t = t[: m.start()] + f'"areaM2": {val}' + t[m.end():]
json.loads(t)
open(P, "w", encoding="utf8").write(t)
a = json.loads(t)["asset"]["thermal"]["regions"]
print({r["id"]: (r["areaM2"], r.get("areaBasis")) for r in a})
