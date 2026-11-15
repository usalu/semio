import json, re, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects"
slug = sys.argv[1]
pairs = dict(a.split("=") for a in sys.argv[2:] if "=" in a)
path = f"{CAT}/{slug}/🔣️.json"
text = open(path).read()
before = json.loads(text)["asset"]["thermal"]["regions"]
for rid, new in pairs.items():
    old = next(r["areaM2"] for r in before if r["id"] == rid)
    assert 1 / 1.5 <= float(new) / old <= 1.5, (rid, old, new)
    pat = re.compile(r'("id": "%s",.{0,900}?"areaM2": )([0-9.]+)' % re.escape(rid), re.S)
    cand = [m for m in pat.finditer(text) if abs(float(m.group(2)) - old) < 1e-9]
    assert cand, rid
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
