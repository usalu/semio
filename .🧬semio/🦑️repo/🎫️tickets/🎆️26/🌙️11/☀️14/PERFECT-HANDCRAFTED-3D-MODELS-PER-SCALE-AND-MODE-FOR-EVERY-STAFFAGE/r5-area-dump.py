"""📋 R03 area reviewer: prints the thermal regions of catalog assets (areaM2, basis, body segment, skinFraction, metabolicShare) and the body totals.
Usage: python3 r5-area-dump.py <category>/<slug> [...]"""
import glob, json, os, sys
C = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
def path(spec):
    cat, slug = spec.split("/")
    return glob.glob(os.path.join(C, f"*{cat}", slug, "🔣️.json"))[0]
for spec in sys.argv[1:]:
    a = json.load(open(path(spec)))["asset"]; t = a["thermal"]; b = t.get("body") or {}
    print(f"## {spec} skinAreaM2 {b.get('skinAreaM2')} massKg {b.get('massKg')} species {b.get('species')}")
    sa = sf = sm = 0
    for r in t["regions"]:
        bd = r.get("body") or {}
        print(f"  {r['id']:<22} {r['kind']:<9} {r['model']:<10} {r['areaM2']:<8} {r.get('areaBasis','') :<8} seg {bd.get('segment','')} sf {bd.get('skinFraction','')} ms {bd.get('metabolicShare','')} heatW {r.get('heatW', (r.get('operating') or {}).get('heatW',''))}")
        if r["model"] == "body": sa += r["areaM2"]; sf += bd.get("skinFraction", 0); sm += bd.get("metabolicShare", 0)
    if b: print(f"  body total areaM2 {sa:.4f} sum skinFraction {sf:.4f} sum metabolicShare {sm:.4f}")
