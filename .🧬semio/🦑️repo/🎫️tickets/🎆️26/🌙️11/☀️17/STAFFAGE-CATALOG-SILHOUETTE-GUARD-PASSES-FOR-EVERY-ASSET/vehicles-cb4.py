import sys, importlib, json, subprocess
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
sc = sys.argv[1]; views = sys.argv[2].split(","); rounds = int(sys.argv[3]) if len(sys.argv) > 3 else 4
only = sys.argv[4].split(",") if len(sys.argv) > 4 else None
def shapes(d, view):
    out = []
    for it in d.at("asset", "representations", sc, "geometry", view)["items"]:
        o = json.loads(d.t[it["s"]:it["e"]]); out.append((it, o))
    return out
def evalall(view):
    t = subprocess.run(["./vehicles-run.sh", "evalall", "cargo-bike", sc, view], capture_output=True, text=True).stdout
    return json.loads(t[t.index("[JSON]") + 6:].splitlines()[0])
targets = {}
for view in views:
    d = m.V("🚜️vehicles/cargo-bike")
    for it, o in shapes(d, view):
        if (only is None or o["id"] in only) and o.get("kind") == "poly" and o.get("bone") == "frame" and "solid" not in o and o.get("closed"):
            n = sum(1 for k in targets if k[0] == view and k[1] == o["id"]); targets[(view, o["id"], n)] = [list(p) for p in o["points"]]
for r in range(rounds):
    worst = 0
    for view in views:
        ev = evalall(view)
        d = m.V("🚜️vehicles/cargo-bike")
        seen = {}
        for it, o in shapes(d, view):
            n = seen.get(o["id"], 0)
            k = (view, o["id"], n)
            if k not in targets: continue
            seen[o["id"]] = n + 1
            e = ev.get(o["id"] + " frame")
            if e is None or len(e) != len(o["points"]) or len(targets[k]) != len(o["points"]): continue
            T = targets[k]
            res = [(T[i][0] - e[i][0], T[i][1] - e[i][1]) for i in range(len(T))]
            worst = max(worst, max(max(abs(a), abs(b)) for a, b in res))
            dmp = 1.0 if r < 3 else 0.5
            o["points"] = [[round(p[0] + max(-0.03, min(0.03, a)) * dmp, 4), round(p[1] + max(-0.03, min(0.03, b)) * dmp, 4)] for p, (a, b) in zip(o["points"], res)]
            d.replace_shape(it, o)
        d.commit()
    print("round", r, "worst residual", round(worst, 5))
    if worst < 0.0004: break
