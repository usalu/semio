"""Author 29: writes the bilingual `decisions` of one asset's scales into its source (replaces only `decisions`). Usage: python3 author-29-decisions.py <asset> <decisions.json>   ({ "50": [{"en","de"},...], ... })"""
import json, sys
asset, df = sys.argv[1], sys.argv[2]
f = f"volumes/{asset}.json"
d = json.load(open(f))
dec = json.load(open(df))
for sc, lst in dec.items():
    d["volumes"][sc]["decisions"] = lst
json.dump(d, open(f, "w"), separators=(",", ":"), ensure_ascii=False)
print("decisions written for", ", ".join(dec))
