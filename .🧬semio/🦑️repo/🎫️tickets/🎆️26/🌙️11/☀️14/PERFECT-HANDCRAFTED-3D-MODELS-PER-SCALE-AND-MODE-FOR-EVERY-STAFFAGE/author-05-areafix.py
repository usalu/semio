"""Usage: python3 author-05-areafix.py <slug> [--apply]
Reads the 1:50 areaDeviation notes (exposed area per region, ruling C1) of volumes-check and, with --apply, rewrites thermal.regions[].areaM2 of the catalog asset (text patch) to the exposed model area (3 significant digits) for every failing region; prints old -> new."""
import glob, json, os, re, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
slug = sys.argv[1]
bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
out = subprocess.run([bun, "volumes-check.ts", "animals/" + slug, "--scale", "50", "--rules", "volumes-thermal-coverage"], cwd=here, capture_output=True, text=True).stdout
changes = {}
for m in re.finditer(r'areaDeviation:([a-z0-9-]+)=\S+ \[exposed ([0-9.]+) m2', out):
    changes[m.group(1)] = float('%.3g' % float(m.group(2)))
for m in re.finditer(r'The exposed surface of region "([a-z0-9-]+)" at 1:50 is ([0-9.]+) m', out):
    changes[m.group(1)] = float("%.3g" % float(m.group(2)))
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/%s/🔣️.json" % slug)[0]
text = open(path, encoding="utf-8").read()
orig = json.loads(text)
old = {r["id"]: r["areaM2"] for r in orig["asset"]["thermal"]["regions"]}
skip = [k for k, v in changes.items() if not (0.5 <= v / old[k] <= 3.6)]
for k in skip: print("SKIPPED (not a plain mis-estimate)", k, old[k], v if False else changes[k])
changes = {k: v for k, v in changes.items() if k not in skip}
for k, v in changes.items(): print(k, old.get(k), "->", v)
if "--apply" in sys.argv:
    start = text.index('"thermal"')
    for rid, v in changes.items():
        pat = re.compile(r'("id": "%s",(?:(?!"id": ).)*?"areaM2": )([0-9.eE+-]+)' % re.escape(rid), re.S)
        m = pat.search(text, start)
        text = text[:m.start(2)] + repr(v) + text[m.end(2):]
    new = json.loads(text)
    strip = lambda d: json.dumps({k: v for k, v in d["asset"].items() if k != "thermal"}, sort_keys=True)
    assert strip(orig) == strip(new)
    for r in new["asset"]["thermal"]["regions"]:
        if r["id"] in changes: assert r["areaM2"] == changes[r["id"]]
    fresh = open(path, encoding="utf-8").read()
    assert fresh == open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(text)
    print("patched", len(changes))
