"""probe: which (entry,key) pairs of a code have no value in its table"""
import sys, importlib
sys.path.insert(0, "."); from dv_lib import *
code = sys.argv[1]; T = importlib.import_module("values_" + code)
es, v = entries(code), vocab(code)
for e in es:
    for k in e["inputs"]:
        r = spec(v, e["id"], k)
        if r is None or "example" in r: continue
        sc = r.get("scope", "")
        keys = [f'{e["id"]}:{k}', f"{k}@{e['id']}", f"{k}@{sc}", k]
        pool = T.VECTORS if r["kind"] == "vector" else T.V
        if r["kind"] in ("choice", "text") and hasattr(T, "TEXTS"): pool = {**pool, **T.TEXTS}
        if k in getattr(T, 'EXTRA', {}).get(e['id'], {}): continue
        if not any(x in pool for x in keys): print("MISSING", e["id"], k, r["kind"], sc)
