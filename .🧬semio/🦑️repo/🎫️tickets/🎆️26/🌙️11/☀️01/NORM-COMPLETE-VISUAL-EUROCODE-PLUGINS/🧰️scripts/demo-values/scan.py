import sys, collections
sys.path.insert(0, "."); from dv_lib import *
for code in DIRS:
    es, v = entries(code), vocab(code)
    miss = collections.OrderedDict()
    for e in es:
        ms = []
        for k in e["inputs"]:
            r = spec(v, e["id"], k)
            if r is None: ms.append(k + "?novocab")
            elif "example" not in r: ms.append(k + ":" + r["kind"])
        if ms: miss[e["id"]] = ms
    kinds = collections.Counter(r["kind"] for r in v["inputs"])
    print(code, len(es), "entries", len(v["inputs"]), "vocab", dict(kinds), "entries with missing", len(miss))
    try:
        d = json.loads(demo_doc_path(code).read_text())
        print("  demo doc cases", len(d.get("cases", [])))
    except Exception as x: print("  no demo doc", x)
