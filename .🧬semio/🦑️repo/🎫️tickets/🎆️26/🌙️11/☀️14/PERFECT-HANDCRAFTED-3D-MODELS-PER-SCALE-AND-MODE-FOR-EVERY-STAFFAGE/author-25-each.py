import json, subprocess, sys, glob, os, re, shutil
asset, slug, sc = sys.argv[1], sys.argv[2], sys.argv[3]
here = os.path.dirname(os.path.abspath(__file__)); bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
src = f"{here}/volumes/{asset.split('/')[0]}/{slug}.json"; tmp = f"{here}/🗑️generated/author-25/{slug}.each.json"
d = json.load(open(src))
res = []
for s in d["volumes"][sc]["structure"]:
    d2 = json.loads(json.dumps(d)); d2["volumes"][sc]["structure"] = [s]; json.dump(d2, open(tmp, "w"))
    out = subprocess.run([bun, "volumes-check.ts", asset, "--scale", sc, "--rules", "volumes-structure-coverage", "--quiet", "--source", tmp], cwd=here, capture_output=True, text=True).stdout
    m = dict(re.findall(r"outsideBodyM:(\w+)=([0-9.e-]+)", out)); res.append((float(m.get(os.environ.get("K","plan"), 0)), float(m.get("front", 0)), s["id"]))
for r in sorted(res, reverse=True)[:6]: print(r)
