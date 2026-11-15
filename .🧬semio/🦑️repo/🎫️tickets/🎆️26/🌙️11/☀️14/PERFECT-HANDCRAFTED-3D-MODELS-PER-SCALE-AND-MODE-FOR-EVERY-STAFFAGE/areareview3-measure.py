"""Reviewer 3: measures the 1:50 outer area of every thermal region of the assets of vegetation, animals, humans, appliances, furniture with the product validator
(volumes-check.ts --scale 50 --rules volumes-thermal-coverage --json) and writes 🗑️generated/areareview3/measure.json: slug -> region -> {decl, outer, exposed, dev}.
Usage: python3 areareview3-measure.py [slug ...]  (no slug: all)"""
import glob, json, os, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
CATS = ["vegetation", "animals", "humans", "appliances", "furniture"]
OUT = os.path.join(T, "🗑️generated", "areareview3")
os.makedirs(OUT, exist_ok=True)

def assets():
    r = []
    for c in CATS:
        for p in sorted(glob.glob(f"{ROOT}/*{c}/*/🔣️.json")):
            r.append((c, p.split("/")[-2], p))
    return r

def one(a):
    c, slug, path = a
    try:
        p = subprocess.run(["nice", "-n", "10", BUN, os.path.join(T, "volumes-check.ts"), f"{c}/{slug}", "--scale", "50", "--rules", "volumes-thermal-coverage", "--json"], capture_output=True, text=True, timeout=900, cwd=T)
        d = json.loads(p.stdout)
    except Exception as e:
        return slug, {"_error": str(e)[:200]}
    doc = json.load(open(path))["asset"]
    regs = {r["id"]: r for r in doc["thermal"]["regions"]}
    out = {}
    for m in d["measurements"]:
        n = m["name"]
        for k, key in (("info:outerAreaM2:", "outer"), ("info:exposedAreaM2:", "exposed"), ("info:exposureMeasured:", "expM"), ("info:exposureTarget:", "expT"), ("areaDeviation:", "dev")):
            if n.startswith(k):
                rid = n[len(k):]
                out.setdefault(rid, {})[key] = m["value"]
    for rid, r in regs.items():
        o = out.setdefault(rid, {})
        o["decl"] = r.get("areaM2")
        o["kind"] = r.get("kind")
        o["model"] = (r.get("model") if isinstance(r.get("model"), str) else None)
        o["basis"] = r.get("areaBasis")
    out["_findings"] = [f.get("message", str(f))[:200] for f in d.get("findings", [])][:5]
    return slug, out

def main():
    al = assets()
    if len(sys.argv) > 1:
        al = [a for a in al if a[1] in sys.argv[1:]]
    path = os.path.join(OUT, "measure.json")
    res = json.load(open(path)) if os.path.exists(path) and len(sys.argv) > 1 else {}
    with ThreadPoolExecutor(3) as ex:
        for slug, out in ex.map(one, al):
            res[slug] = out
            print(slug, len(out), flush=True)
    json.dump(res, open(path, "w"), indent=1, ensure_ascii=False)
main()
