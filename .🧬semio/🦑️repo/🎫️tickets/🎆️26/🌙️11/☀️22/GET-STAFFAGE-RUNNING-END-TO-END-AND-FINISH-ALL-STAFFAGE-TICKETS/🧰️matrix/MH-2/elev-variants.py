"""🧪️ Variants of a loft plan that leave the elevation of some shapes of one scale as drawn: elev-variants.py <plan.json> <scale> <out-dir> <name>=<id,id…>… writes <out-dir>/<name>.lofts.json per variant, the named ids of that scale without the view `elevation`."""
import json, sys
plan = json.load(open(sys.argv[1])); scale = sys.argv[2]; out = sys.argv[3]
for spec in sys.argv[4:]:
    name, ids = spec.split("="); ids = set(ids.split(","))
    rows = []
    for e in plan:
        e = dict(e)
        if str(e["scale"]) == scale and e["id"] in ids and "as" not in e:
            e["views"] = [v for v in e["views"] if v != "elevation"]
        if e["views"]: rows.append(e)
    json.dump(rows, open(f"{out}/{name}.lofts.json", "w"), ensure_ascii=False, indent=1)
