"""Exposed area per region and scale (product measure): prints every region, also those that pass (by forcing areaM2 mismatch is not needed: uses --json findings + deviation)."""
import json, re, subprocess, sys, glob, os
here = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
slug = sys.argv[1]
a = json.load(open(glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*appliances/%s/*.json" % slug)[0]))["asset"]
area = {r["id"]: r["areaM2"] for r in a["thermal"]["regions"]}
out = subprocess.run([BUN, "volumes-check.ts", "appliances/" + slug, "--rules", "volumes-thermal-coverage", "--product", "--json"], cwd=here, capture_output=True, text=True).stdout
d = json.loads(out)
got = {}
for f in d["findings"]:
    m = re.search(r'region "([^"]+)" at 1:(\d+) is ([\d.]+) m', f["message"])
    if m: got[(m.group(1), int(m.group(2)))] = float(m.group(3))
for m in d["measurements"]:
    if m["name"].startswith("areaDeviation"):
        r = m["name"].split(":")[1]; s = m["scale"]
        v = got.get((r, s))
        if v is None:
            v = area[r] * (1 + m["value"])
            tag = "(~ ok, either side)"
        else: tag = ""
        print("1:%d" % s, r.ljust(14), "areaM2", area[r], "exposed", round(v, 4), "dev", round(m["value"], 2), "OK" if m["ok"] else "FAIL", tag)
