"""🔎 Compact one-line dump of tram solids: rework-tram-dump.py <scale> <list> [regex]"""
import json, re, sys
v = json.load(open("volumes/vehicles/tram.json"))["volumes"][sys.argv[1]][sys.argv[2]]
rx = re.compile(sys.argv[3]) if len(sys.argv) > 3 else None
for s in v:
    if rx and not rx.search(s.get("id", "") + " " + str(s.get("of", ""))):
        continue
    d = {k: x for k, x in s.items() if k not in ("title", "material")}
    for k in ("profile", "sections", "holes"):
        if k in d and len(json.dumps(d[k])) > 150:
            d[k] = "<%d>" % len(d[k])
    print(json.dumps(d, ensure_ascii=False, separators=(",", ":")).replace('"', ""))
