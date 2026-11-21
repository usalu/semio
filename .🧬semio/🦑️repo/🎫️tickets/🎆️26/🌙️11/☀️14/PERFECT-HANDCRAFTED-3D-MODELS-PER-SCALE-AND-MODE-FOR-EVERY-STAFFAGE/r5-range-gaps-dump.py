"""📏 Dumps the valid ranges of every catalog asset's representations (R01 range-gap sweep input)."""
import json, glob, os, sys
root = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
for path in sorted(glob.glob(os.path.join(root, "*", "*", "🔣️.json"))):
    a = json.load(open(path))["asset"]
    reps = a.get("representations", {})
    row = []
    for d in ["50", "100", "200", "500"]:
        r = reps.get(d)
        if r is None: row.append(f"{d}:-")
        elif r["status"] == "authored": row.append(f"{d}:{r['validRange']['minDenominator']}-{r['validRange']['maxDenominator']}")
        else: row.append(f"{d}:U({r.get('cause')};{','.join(x.get('kind','?')+':'+str(x.get('denominator', x.get('asset', x.get('symbol','')))) for x in r.get('alternatives',[]))})")
    print(a["id"], " ".join(row))
