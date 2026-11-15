"""Usage: python3 author-06-areaapply.py [--apply]  Reads the 1:50 line of 🗑️generated/author-06/<slug>.check.txt and rewrites areaM2 of regions whose exposed model area is more than 15 % off (exposed > 0) to the exposed model area (3 s.f.)."""
import glob, json, re, sys
slugs = ["mallard-duck","pig","pigeon","red-fox","red-squirrel","roe-deer","sheep"]
log = []
ALLOW = {('pig','skin-legs'),('pig','skin-head'),('pig','ears'),('sheep','ears')}
for slug in slugs:
    chk = open("🗑️generated/author-06/%s.check.txt" % slug, encoding="utf-8").read()
    line = [l for l in chk.splitlines() if l.startswith(("FAIL volumes-thermal-coverage    1:50", "ok   volumes-thermal-coverage    1:50"))]
    if not line: continue
    path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/%s/🔣️.json" % slug)[0]
    text = open(path, encoding="utf-8").read()
    changes = {}
    for m in re.finditer(r"areaDeviation:([a-z0-9-]+)=([0-9.e-]+)<=[0-9.]+ ?\[([^\]]*)\]", line[0]):
        rid, dev, note = m.group(1), float(m.group(2)), m.group(3)
        e = re.search(r"exposed ([0-9.]+) m2 of .*areaM2 ([0-9.e-]+)", note)
        if not e: continue
        exp, decl = float(e.group(1)), float(e.group(2))
        if exp <= 0 or abs(exp / decl - 1) <= 0.15: continue
        if (slug, rid) in ALLOW: changes[rid] = (decl, float("%.3g" % exp))
    for rid, (old, new) in changes.items():
        log.append((slug, rid, old, new))
        if "--apply" in sys.argv:
            pat = re.compile(r'("id": "%s",\n(?:\s+"(?:title|kind|model|material|hidden|bones|tags)":.*\n|\s+"(?:en|de)":.*\n|\s+\},?\n|\s+\{\n)*?\s+"areaM2": )([0-9.eE+-]+)' % re.escape(rid))
            m = pat.search(text, text.index('"thermal"'))
            if not m: print("NOT PATCHED", slug, rid); continue
            text = text[:m.start(2)] + repr(new) + text[m.end(2):]
    if "--apply" in sys.argv and changes:
        fresh = json.loads(open(path, encoding="utf-8").read()); patched = json.loads(text)
        rest = lambda d: json.dumps({k: v for k, v in d["asset"].items() if k != "thermal"}, sort_keys=True)
        assert rest(fresh) == rest(patched)
        for r in patched["asset"]["thermal"]["regions"]:
            if r["id"] in changes: assert r["areaM2"] == changes[r["id"]][1]
        open(path, "w", encoding="utf-8").write(text)
for l in log: print("%-14s %-16s %s -> %s" % l)
