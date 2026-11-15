import sys, json, importlib
sys.path.insert(0, ".")
V = importlib.import_module("author-13-vacuum-cleaner")
L = V.L
full = V.volumes()
ids = [s["id"] for s in full["500"]["body"]]
import subprocess
res = {}
for i in ids:
    v = json.loads(json.dumps(full))
    v["500"]["body"] = [s for s in v["500"]["body"] if s["id"] != i]
    L.write("appliances/vacuum-cleaner", v)
    out = L.run("check", "appliances/vacuum-cleaner", "--scale", "500", "--rules", "volumes-cross-scale")
    res[i] = "CRASH" if "TypeError" in out else "ok"
    print(i, res[i], flush=True)
