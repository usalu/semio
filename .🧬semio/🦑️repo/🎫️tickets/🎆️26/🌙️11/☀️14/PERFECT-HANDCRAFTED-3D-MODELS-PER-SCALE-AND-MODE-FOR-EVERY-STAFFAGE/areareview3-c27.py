"""Reviewer 3: C27 table of a living subject: per body-model region declared areaM2, skinFraction, metabolicShare, f = areaM2/skinAreaM2 and the measured 1:50 outer area (🗑️generated/areareview3/measure.json),
then the sum-preserving rescale over the measurable (non-exchange) regions. Usage: python3 areareview3-c27.py slug [slug...]"""
import glob, json, os, sys
T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
M = json.load(open(os.path.join(T, "🗑️generated", "areareview3", "measure.json")))
for slug in sys.argv[1:]:
    a = json.load(open(glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]))["asset"]
    phys = a.get("physiology") or a.get("body") or {}
    regs = [r for r in a["thermal"]["regions"] if r.get("body")]
    sk = sum(r["body"]["skinFraction"] for r in regs)
    ms = sum(r["body"]["metabolicShare"] for r in regs)
    S = (a.get("thermal", {}).get("body") or {}).get("skinAreaM2") or phys.get("skinAreaM2")
    print(f"## {slug} skinAreaM2={S} sumDecl={sum(r['areaM2'] for r in regs):.4f} sumSkinFr={sk:.3f}")
    meas = [r for r in regs if r.get("areaBasis") != "exchange" and M.get(slug, {}).get(r["id"], {}).get("outer") is not None]
    sd = sum(r["areaM2"] for r in meas); so = sum(M[slug][r["id"]]["outer"] for r in meas)
    for r in regs:
        m = M.get(slug, {}).get(r["id"], {}).get("outer")
        f = r["areaM2"] / (r["body"]["skinFraction"] / sk * S) if S else None
        new = (m * sd / so) if (m is not None and r in meas) else None
        print(f"  {r['id']:18} {r['kind']:9} a={r['areaM2']:<8} sf={r['body']['skinFraction']:<6} ms={r['body']['metabolicShare']:<6} f={f and round(f,2)} meas={m and round(m,4)} ratio={m and round(m/r['areaM2'],2)} rescaled={new and round(new,4)} share={r['areaM2']/sd:.2f}->{(new or 0)/sd:.2f}" if m is not None else f"  {r['id']:18} {r['kind']:9} a={r['areaM2']:<8} sf={r['body']['skinFraction']:<6} ms={r['body']['metabolicShare']:<6} f={f and round(f,2)} (exchange / not measured)")
    print(f"  measurable: sumDecl={sd:.4f} sumMeasured={so:.4f} factor={sd/so:.3f}")
