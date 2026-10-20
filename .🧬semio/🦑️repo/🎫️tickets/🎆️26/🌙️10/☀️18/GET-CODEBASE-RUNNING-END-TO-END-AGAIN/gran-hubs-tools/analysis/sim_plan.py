"""📉 Churn x reach before/after of staged plans: replays the plan on the feature-aware graph (hub edge replaced by the producer edges for planned crates; manifest-only dead-edge plans drop the edge) and prints reach of the hubs, W over all crates and the hub-family sums. `python3 sim_plan.py <tag>[,<tag>...]`"""
from rewire import *
tags = sys.argv[1].split(",")
base, rv0 = R0.all_reach(); B = W(R0, base)
byn = {c["name"]: d for d, c in R0.C.items()}
R = mk()
changed = 0
for tag in tags:
    plan = json.load(open(os.path.normpath(T + "/../staged/plans/%s.json" % tag)))
    HD = byn[plan["hub_package"]]
    for m in plan["manifests"]:
        d = os.path.dirname(m["path"])
        if d not in R.M: continue
        cr = plan["crates"].get(m["crate"], {})
        tg = {I2D[i] for i in cr.get("owners_prod", [])}
        mm = R.M[d]
        mm["deps"] = [x for x in mm["deps"] if not (x["kind"] in ("normal", "build") and R.tgt.get((d, x["kind"], x["key"])) == HD)]
        have = {R.tgt.get((d, x["kind"], x["key"])) for x in mm["deps"] if x["kind"] in ("normal", "build")}
        for t in tg:
            if t in have or t == d: continue
            key = "__rw_" + R.C[t]["name"]
            mm["deps"].append({"key": key, "pkg": R.C[t]["name"], "kind": "normal", "target": None, "optional": False, "features": [], "defaultFeatures": True, "ws": False, "path": None})
            R.tgt[(d, "normal", key)] = t
        changed += 1
R.cache = {}
reach, _ = R.all_reach()
print("plans", tags, "manifests", changed)
print("W %d -> %d (%.2f%%)" % (B, W(R, reach), (W(R, reach) - B) * 100 / B))
CHn = CH
for n in ["semio-framework", "semio-framework-kernel", "semio-framework-abi", "semio-framework-source-projection", "semio-framework-action-bus", "semio-framework-manifest-actions", "semio-framework-kernel-identity", "semio-framework-plugin", "semio-framework-os-kernel", "semio-framework-job"]:
    d = byn[n]; print("  %-36s churn %3d reach %5d -> %5d  churn*reach %7d -> %7d" % (n.replace("semio-framework-", "f-"), CHn.get(n, 0), base[d], reach.get(d, 0), CHn.get(n, 0) * base[d], CHn.get(n, 0) * reach.get(d, 0)))
rows = sorted(((CHn.get(R.C[d]["name"], 0) * (base[d] - reach.get(d, 0)), R.C[d]["name"].replace("semio-framework-", ""), base[d], reach.get(d, 0)) for d in base if base[d] != reach.get(d, 0)), reverse=True)
print("crates with lower reach:", sum(1 for d in base if reach.get(d, 0) < base[d]), "higher:", sum(1 for d in base if reach.get(d, 0) > base[d]))
for r in rows[:12]: print("   %7d %-38s %5d -> %5d" % r)
