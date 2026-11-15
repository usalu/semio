import sys, json, glob, os
def load(root):
    rows = {}
    for kind in ("e", "r"):
        for f in glob.glob(f"{root}/{kind}/*/*/*.json"):
            e = json.load(open(f))
            rows[e["key"]] = e
    return rows
a, b = load(sys.argv[1]), load(sys.argv[2])
for key in sorted(set(a) & set(b), key=lambda k: a[k].get("crate", a[k].get("package", ""))):
    ea, eb = a[key], b[key]
    name = ea.get("crate") or ea.get("package")
    oa = {o.get("name") or o.get("path"): o["sha256"] for o in ea["outputs"]}
    ob = {o.get("name") or o.get("path"): o["sha256"] for o in eb["outputs"]}
    diff = [n for n in oa if oa[n] != ob.get(n)]
    sa = (ea.get("stderr"), ea.get("stdout")); sb = (eb.get("stderr"), eb.get("stdout"))
    print(f"{name:32} {key[:10]} same={[n for n in oa if oa[n]==ob.get(n)]} DIFF={diff} streams_equal={sa==sb}")
print("only A:", [a[k].get('crate') for k in set(a)-set(b)], "only B:", [b[k].get('crate') for k in set(b)-set(a)])
