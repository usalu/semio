"""Usage: python3 author-04-areafix.py <slug> [--apply]
Prints, per thermal region, areaM2 of the asset against the closed-surface area of the authored thermal solids at every scale (same measure as the area rule).
--set=<region>:<value> forces a region value (with --apply). --apply rewrites areaM2 in the catalog asset (text patch, nothing else changes) to the 1:50 model area (3 significant digits) for every region whose 1:50 deviation exceeds 12 %."""
import glob, json, os, re, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
slug = sys.argv[1]
bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
out = subprocess.run([bun, "author-04-area.ts", "animals/" + slug], cwd=here, capture_output=True, text=True)
areas = json.loads(out.stdout)
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/%s/🔣️.json" % slug)[0]
text = open(path, encoding="utf-8").read()
changes = {}
prod = {}
chk = subprocess.run([bun, "volumes-check.ts", "animals/" + slug, "--scale", "50", "--rules", "volumes-thermal-coverage", "--json"], cwd=here, capture_output=True, text=True)
for m in json.loads(chk.stdout)["measurements"]:
    if m["name"].startswith("areaDeviation:") and m.get("engine") == "p":
        prod[m["name"].split(":", 1)[1]] = m["value"]
for rid, a in areas.items():
    a50 = a.get("50")
    if rid in prod and a50 is not None:
        cands = [a["areaM2"] * (1 + prod[rid]), a["areaM2"] * (1 - prod[rid])]
        a50 = min(cands, key=lambda c: abs(c - a50))
        a["50"] = float("%.4g" % a50)
    row = " ".join("%s=%s" % (s, a.get(s)) for s in ("50", "100", "200", "500") if s in a)
    flag = ""
    if a50 is not None and abs(a50 / a["areaM2"] - 1) > 0.12:
        flag = "  <- 1:50 off %.0f %%" % (100 * (a50 / a["areaM2"] - 1))
        changes[rid] = float("%.3g" % a50)
    print("%-22s areaM2=%-8s %s%s" % (rid, a["areaM2"], row, flag))
for arg in sys.argv[2:]:
    if arg.startswith("--set="):
        rid, v = arg[6:].split(":")
        changes[rid] = float(v)
if "--apply" in sys.argv:
    for rid, value in changes.items():
        pat = re.compile(r'("id": "%s",\n(?:\s+"(?:title|kind|model|material|hidden|bones|tags)":.*\n|\s+"(?:en|de)":.*\n|\s+\},?\n|\s+\{\n)*?\s+"areaM2": )([0-9.eE+-]+)' % re.escape(rid))
        start = text.index('"thermal"')
        m = pat.search(text, start)
        if not m:
            print("NOT PATCHED", rid)
            continue
        text = text[:m.start(2)] + repr(value) + text[m.end(2):]
    fresh = open(path, encoding="utf-8").read()
    original = json.loads(fresh)
    patched = json.loads(text)
    for r in patched["asset"]["thermal"]["regions"]:
        if r["id"] in changes:
            assert r["areaM2"] == changes[r["id"]], r["id"]
    rest = lambda d: json.dumps({k: v for k, v in d["asset"].items() if k != "thermal"}, sort_keys=True)
    assert rest(original) == rest(patched), "other members changed"
    open(path, "w", encoding="utf-8").write(text)
    print("patched", ", ".join(changes) or "nothing")
