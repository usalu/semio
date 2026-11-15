"""Usage: python3 author-59-drop.py <asset> <slug> <scale> <rule> <key>: re-checks the scale with each body solid removed alone and prints the measurement `key` (diagnostic: which solid drives a boundary-distance number)."""
import json, subprocess, sys, glob, os, re
asset, slug, sc, rule, key = sys.argv[1:6]
here = os.path.dirname(os.path.abspath(__file__)); bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
src = f"{here}/volumes/{asset.split('/')[0]}/{slug}.json"; tmp = f"{here}/🗑️generated/author-59/{slug}.drop.json"
d = json.load(open(src)); res = []
for s in d["volumes"][sc]["body"]:
    d2 = json.loads(json.dumps(d)); d2["volumes"][sc]["body"] = [x for x in d2["volumes"][sc]["body"] if x["id"] != s["id"]]; json.dump(d2, open(tmp, "w"))
    out = subprocess.run([bun, "volumes-check.ts", asset, "--scale", sc, "--rules", rule, "--product", "--quiet", "--source", tmp], cwd=here, capture_output=True, text=True).stdout
    m = re.search(re.escape(key) + r"=([0-9.e-]+)", out)
    res.append((float(m.group(1)) if m else -1, s["id"]))
for r in sorted(res)[:8]: print(r)
