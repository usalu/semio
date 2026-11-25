#!/usr/bin/env python3
"""🎺️ `used-trumpet` 1:200 structure: the three joints (60 mm, twice the 30 mm feature) and the seven mass markers (36 mm) at 32 mm, so the structure triangles fall from 1:100 to 1:200 (guard 🧊️model-catalog, check `budget`). Writes only these radii inside `volumes["200"].structure`, idempotent: trumpet.py <asset document>"""
import json, re, sys
path = sys.argv[1]
text = before = open(path, encoding="utf-8").read()
asset = json.loads(text)["asset"]
assert asset["id"] == "used-trumpet"
ids = [x["id"] for x in asset["volumes"]["200"]["structure"] if x.get("part") in ("joint", "mass")]
start = text.index('"200": {', text.index('"volumes": {')); end = text.index('"500": {', start)
part = text[start:end]
for i in ids:
    at = part.index('"id": "%s"' % i); stop = part.find('"id": "', at + 8); stop = len(part) if stop < 0 else stop
    solid = re.sub(r'("r[01]": )0\.03\b', r"\g<1>0.016", part[at:stop])
    solid = re.sub(r'("radii": \[)([^\]]*)(\])', lambda m: m[1] + re.sub(r"0\.0(3|18)\b", "0.016", m[2]) + m[3], solid)
    part = part[:at] + solid + part[stop:]
text = text[:start] + part + text[end:]
if text != before: open(path, "w", encoding="utf-8").write(text)
print("used-trumpet", "written" if text != before else "unchanged")
