"""Root correction of thermal areaM2 (C1): python3 author-09-areafix.py <slug> region=new ... [--apply]"""
import json, re, sys
sys.path.insert(0, ".")
from author_09_lib import CAT
slug = sys.argv[1]
pairs = dict(a.split("=") for a in sys.argv[2:] if "=" in a)
path = f"{CAT}/{slug}/🔣️.json"
text = open(path).read()
before = json.loads(text)["asset"]["thermal"]["regions"]
for rid, new in pairs.items():
    old = next(r["areaM2"] for r in before if r["id"] == rid)
    pat = re.compile(r'("id": "%s",.{0,900}?"areaM2": )([0-9.]+)' % re.escape(rid), re.S)
    ms = list(pat.finditer(text))
    cand = [m for m in ms if abs(float(m.group(2)) - old) < 1e-9]
    assert len(cand) >= 1, (rid, len(ms))
    m = cand[0]
    print(f"{slug} {rid}: {old} -> {new}")
    if "--apply" in sys.argv:
        text = text[: m.start(2)] + new + text[m.end(2):]
if "--apply" in sys.argv:
    after = json.loads(text)["asset"]
    for r in after["thermal"]["regions"]:
        assert r["id"] in pairs or r["areaM2"] == next(b["areaM2"] for b in before if b["id"] == r["id"])
    open(path, "w").write(text)
    print("applied")
