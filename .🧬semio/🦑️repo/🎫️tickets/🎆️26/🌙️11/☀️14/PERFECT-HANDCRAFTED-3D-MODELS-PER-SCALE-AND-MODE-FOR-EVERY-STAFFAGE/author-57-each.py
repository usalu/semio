import json, subprocess, sys, glob, os, re
# usage: author-57-each.py category/slug scale  : each structure solid alone against its scale: outsideBodyM per view
asset, sc = sys.argv[1], sys.argv[2]; slug = asset.split("/")[1]
here = os.path.dirname(os.path.abspath(__file__)); bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
src = f"{here}/volumes/{asset}.json"; tmp = f"{here}/🗑️generated/author-57/{slug}.each.json"
d = json.load(open(src)); res = []
for s in d["volumes"][sc]["structure"]:
    d2 = json.loads(json.dumps(d)); d2["volumes"][sc]["structure"] = [s]; json.dump(d2, open(tmp, "w"))
    out = subprocess.run([bun, "volumes-check.ts", asset, "--scale", sc, "--rules", "volumes-structure-coverage", "--quiet", "--source", tmp], cwd=here, capture_output=True, text=True).stdout
    m = dict(re.findall(r"outsideBodyM:(\w+)=([0-9.e-]+)", out)); res.append((float(m.get("elevation", 0)), float(m.get("front", 0)), float(m.get("plan", 0)), s["id"]))
for r in sorted(res, key=lambda r: -max(r[:3]))[:8]: print([round(x*1000,1) if not isinstance(x,str) else x for x in r])
