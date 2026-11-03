import sys
sys.path.insert(0, "."); from dv_lib import *
code = sys.argv[1]
es, v = entries(code), vocab(code)
used = {}
for e in es:
    for k in e["inputs"]:
        r = spec(v, e["id"], k)
        if r is not None: used.setdefault((r["key"], r.get("scope", "")), []).append(e["id"])
for (k, sc), ids in sorted(used.items()):
    r = next(x for x in v["inputs"] if x["key"] == k and x.get("scope", "") == sc)
    ch = ",".join(c["value"] for c in r.get("choices", []))[:80]
    print(f'{k}|{sc}|{r["kind"]}|{r.get("unit","")}|ex={r.get("example","-")}|{r["en"][:40]}|{ch}|{(r.get("note") or "")[:50]}|used:{",".join(ids)[:40]}')
