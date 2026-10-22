"""📉 Churn x reach before/after of an ordered list of plans: final manifests (plans composed on the checkout, new crates included) are read with Bun's TOML parser, the feature-aware graph is rebuilt with those dependency lists and W = sum churn(prod) x reach is compared. New macro crates inherit the churn of the file they were cut from. `python3 sim_plans.py tag,tag,...`"""
from hubenv import *
import subprocess, copy
sys.path.insert(0, os.path.normpath(T + "/../staged"))
import apply as A
from graphfeat import Resolver

tags = sys.argv[1].split(",")
plans = A.load_plans(",".join(tags))
CH = json.load(open(OUT + "/churn-all.json"))["prod"]
FC = json.load(open(OUT + "/file-commits.json"))
R0 = Resolver()
base, rv0 = R0.all_reach()


def W(R, reach):
    return sum(CH.get(R.C[d]["name"], R.C[d].get("churn", 0)) * n for d, n in reach.items())


files, manifests = A.collect(plans, set())
created = {}
for p in plans:
    for c in p.get("creates", []):
        created[c["path"]] = c["content"]
final = {}
for rel, entries in manifests.items():
    text = open(os.path.join(A.DEFAULT_ROOT, rel), encoding="utf-8").read()
    for e in entries:
        text = e["new_text"] if A.sha(text.encode()) == e["sha256"] else A.new_text_of_manifest(text, e)
    final[rel] = text
for rel, text in created.items():
    if rel.endswith("Cargo.toml"):
        final[rel] = text
inp = [{"path": r, "text": t} for r, t in final.items()]
res = json.loads(subprocess.run(["bun", T + "/sim_edges.ts"], input=json.dumps(inp), capture_output=True, text=True).stdout)
R = Resolver(M={d: {**m, "deps": list(m["deps"])} for d, m in R0.M.items()}, C={d: dict(c) for d, c in R0.C.items()})
name2dir = {c["name"]: d for d, c in R0.C.items()}
newdirs = {}
for rel, info in res.items():
    d = os.path.dirname(rel)
    if info and d not in R.M and info["name"]:
        newdirs[d] = info["name"]
        name2dir[info["name"]] = d
for d, n in newdirs.items():
    R.C[d] = {"dir": d, "name": n, "libpath": None, "proc_macro": False, "build": None, "deps": {"normal": [], "build": [], "dev": []}, "has_ws_table": False}
    # churn of the file the crate was cut from
    lib = d.rsplit("/📦️packages", 1)[0] + "/🦀️.rs"
    R.C[d]["churn"] = len(FC.get(lib, [])) if lib in FC else 6
    R.M[d] = {"wsroot": ".", "name": n, "features": {}, "lib": None, "deps": []}
changed = 0
for rel, info in res.items():
    d = os.path.dirname(rel)
    if not info or d not in R.M:
        continue
    deps = []
    for x in info["deps"]:
        if x["kind"] == "dev":
            continue
        t = name2dir.get(x["pkg"])
        if not t:
            continue
        key = x["key"]
        deps.append({"key": key, "pkg": x["pkg"], "kind": x["kind"], "target": x["target"], "optional": x["optional"], "features": x["features"], "defaultFeatures": True, "ws": x["ws"], "path": x["path"]})
        R.tgt[(d, x["kind"], key)] = t
    # drop stale targets
    for k in [k for k in R.tgt if k[0] == d and k[1] in ("normal", "build") and k[2] not in {y["key"] for y in deps if y["kind"] == k[1]}]:
        del R.tgt[k]
    R.M[d]["deps"] = deps + [x for x in R0.M.get(d, {"deps": []})["deps"] if x["kind"] == "dev"]
    if info["features"]:
        R.M[d]["features"] = info["features"]
    changed += 1
R.cache = {}
reach, _ = R.all_reach()
B, A_ = W(R0, base), W(R, reach)
print("plans", tags, "manifests", len(final), "new crates", len(newdirs), "dirs changed", changed)
print("W %d -> %d (%.2f%%)" % (B, A_, (A_ - B) * 100 / B))
byn = {c["name"]: d for d, c in R.C.items()}
for n in ["semio-framework", "semio-framework-plugin", "semio-framework-os-kernel", "semio-framework-kernel", "semio-framework-job", "semio-framework-plugin-reactor-turn", "semio-framework-os-store-engine", "semio-framework-os-spr", "semio-framework-plugin-abi", "semio-framework-action-bus"] + sorted(newdirs.values()):
    d = byn[n]
    print("  %-46s churn %3d reach %5d -> %5d" % (n.replace("semio-framework-", "f-"), CH.get(n, R.C[d].get("churn", 0)), base.get(d, 0), reach.get(d, 0)))
low = sum(1 for d in base if reach.get(d, 0) < base[d])
print("crates with lower reach:", low, "higher:", sum(1 for d in base if reach.get(d, 0) > base[d]))
rows = sorted(((CH.get(R.C[d]["name"], 0) * (base[d] - reach.get(d, 0)), R.C[d]["name"].replace("semio-framework-", ""), base[d], reach.get(d, 0)) for d in base if base[d] != reach.get(d, 0)), reverse=True)
for r in rows[:16]:
    print("   %7d %-40s %5d -> %5d" % r)
json.dump({"tags": tags, "W_before": B, "W_after": A_, "reach_hubs": {n: [base[byn[n]], reach.get(byn[n], 0)] for n in ["semio-framework", "semio-framework-plugin", "semio-framework-os-kernel", "semio-framework-kernel"]}}, open(OUT + "/sim-" + "+".join(tags)[:60] + ".json", "w"))
