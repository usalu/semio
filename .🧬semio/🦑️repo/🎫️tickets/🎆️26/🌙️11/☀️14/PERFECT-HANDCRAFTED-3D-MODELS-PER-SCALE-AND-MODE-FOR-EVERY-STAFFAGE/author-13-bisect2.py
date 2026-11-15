import sys, json, importlib
sys.path.insert(0, ".")
V = importlib.import_module("author-13-vacuum-cleaner")
L = V.L
full = V.volumes()
def trial(name, fn):
    v = json.loads(json.dumps(full)); fn(v["500"]["body"])
    L.write("appliances/vacuum-cleaner", v)
    out = L.run("check", "appliances/vacuum-cleaner", "--scale", "500", "--rules", "volumes-cross-scale")
    print(name, "CRASH" if "TypeError" in out else "ok", flush=True)
def f1(b):
    for s in b:
        if s["id"]=="rear-wheel-r": s["transform"]["translation"][2]-=0.0007
def f2(b):
    for s in b:
        if s["id"]=="rear-wheel-r": s["profile"]=[[a*1.0,r] for a,r in s["profile"][:-1]]+[[s["profile"][-1][0],0.0]]
def f3(b):
    for s in b:
        if s["id"] in("rear-wheel-r","rear-wheel-l"): s["profile"]=[[a, r if r<0.05 else r*0.995] for a,r in s["profile"]]
for n,f in (("shift",f1),("same",f2),("shrink",f3)): trial(n,f)
