import sys, collections
sys.path.insert(0, "."); from dv_lib import *
for code in DIRS:
    es, v = entries(code), vocab(code)
    pairs=0; keys=set(); dup=collections.Counter()
    for e in es:
        for k in e["inputs"]:
            r = spec(v, e["id"], k)
            if r is not None and "example" not in r: pairs+=1; keys.add((k,r["kind"],r.get("scope","")))
    print(code, "missing pairs", pairs, "distinct vocab rows", len(keys), "vocab rows w/ example", sum("example" in r for r in v["inputs"]), "of", len(v["inputs"]))
    print("  sample row", json.dumps(v["inputs"][0], ensure_ascii=False)[:300])
    print("  choice sample", [json.dumps(r,ensure_ascii=False)[:400] for r in v["inputs"] if r["kind"] in("choice","series","vector")][:2])
