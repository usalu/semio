"""📐 R03 area reviewer: measures the C29/C31 outer area of every thermal region of the named assets at every authored scale with the product validator
(`volumes-check.ts <asset> --rules volumes-thermal-coverage --json`), stores `🗑️generated/r5-area/<category>-<slug>.json` and prints one table per asset
(declared areaM2, outer area per scale, areaDeviation and pass marks, the C15 totals at 1:200 / 1:500).
Usage: python3 r5-area-measure.py <category>/<slug> [...] [--cached]  (--cached: the stored result, else the R03 census `🗑️generated/r4-area/census/r5-r03`, else a new run)
@see ./design.md (C15, C18, C21, C27, C29, C31)"""
import glob, json, os, subprocess, sys
T = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
OUT = os.path.join(T, "🗑️generated", "r5-area")
os.makedirs(OUT, exist_ok=True)

def asset_path(spec):
    c, s = spec.split("/")
    return glob.glob(os.path.join(ROOT, f"*{c}", s, "🔣️.json"))[0]

def measure(spec, cached):
    out = os.path.join(OUT, spec.replace("/", "-") + ".json")
    census = os.path.join(T, "🗑️generated", "r4-area", "census", "r5-r03", spec.replace("/", ".") + ".json")
    if cached and not os.path.exists(out) and os.path.exists(census):
        return json.load(open(census))
    if not (cached and os.path.exists(out)):
        raw = out + ".raw"
        with open(raw, "w") as f:
            subprocess.run(["nice", "-n", "10", BUN, os.path.join(T, "volumes-check.ts"), spec, "--rules", "volumes-thermal-coverage", "--json", "--product"], stdout=f, stderr=subprocess.DEVNULL, timeout=1800, cwd=T)
        json.dump(json.load(open(raw)), open(out, "w"), indent=0)
        os.remove(raw)
    return json.load(open(out))

def show(spec, d):
    regs = json.load(open(asset_path(spec)))["asset"]["thermal"]["regions"]
    by = {}
    for m in d["measurements"]:
        by.setdefault(m["scale"], {})[m["name"]] = m
    scales = sorted(by)
    print(f"## {spec}  scales {scales}")
    for r in regs:
        rid = r["id"]
        cells = []
        for s in scales:
            o = by[s].get(f"info:outerAreaM2:{rid}")
            dv = by[s].get(f"areaDeviation:{rid}")
            mark = "" if dv is None else ("!" if not dv["ok"] else "")
            ov = "-" if o is None else round(o["value"], 4)
            dvs = "" if dv is None else "(%+.2f%s)" % (dv["value"], mark)
            cells.append(f"{s}:{ov}{dvs}")
        bd = r.get("body") or {}
        print(f"  {rid:<22} {r['kind']}/{r['model']}{'/exch' if r.get('areaBasis') else ''} decl {r['areaM2']} sf {bd.get('skinFraction','')} | " + "  ".join(cells))
    for s in scales:
        t = by[s].get("thermal.areaDeviationTotal")
        if t: print(f"  total 1:{s} {t['value']:+.3f} {'ok' if t['ok'] else 'FAIL'}")
    bad = [f"{m['scale']} {m['name']}" for m in d["measurements"] if not m["ok"]]
    print("  failing (as measured):", bad or "none")
    feat = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
    now = []
    for s in scales:
        so = sd = 0
        for r in regs:
            o = by[s].get(f"info:outerAreaM2:{r['id']}")
            if o is None or r.get("areaBasis") == "exchange" or r["areaM2"] < 6 * feat[s] ** 2:
                continue
            so += o["value"]; sd += r["areaM2"]
            if s in (50, 100):
                dv = abs(o["value"] - r["areaM2"]) / r["areaM2"]
                lim = 0.3 if (s == 100 or r["model"] == "body") else 0.15
                if dv > lim: now.append(f"{s}:{r['id']} {dv:+.2f}>{lim}")
        if s in (200, 500) and sd > 0 and abs(so - sd) / sd > 0.3:
            now.append(f"{s}:total {abs(so - sd) / sd:+.2f}")
    print("  failing with the current areaM2 (recomputed):", now or "none")

if __name__ == "__main__":
    cached = "--cached" in sys.argv
    for spec in [a for a in sys.argv[1:] if not a.startswith("--")]:
        show(spec, measure(spec, cached))
