"""📊️ Read-only comparison of the 04:50 census baseline with the fresh 11:23 census summary."""
import json, sys
BASE = sys.argv[1]
NOW = sys.argv[2]
def load(p):
    with open(p, encoding="utf-8") as f:
        return json.load(f)
b = load(BASE)
n = load(NOW)
def stats(d):
    rows = d["rows"]
    return {
        "nodes": d.get("nodes", len(rows)),
        "rows": len(rows),
        "overLimit": sorted(r["name"] for r in rows if r.get("overLimit")),
        "closureCompilable": sum(1 for r in rows if r.get("closureCompilable")),
        "members": sum(1 for r in rows if r.get("workspaceMember")),
        "carvableLeaf": sorted(r["name"] for r in rows if r.get("carvableLeaf")),
    }
sb, sn = stats(b), stats(n)
print("BASE nodes", sb["nodes"], "rows", sb["rows"], "members", sb["members"], "closureCompilable", sb["closureCompilable"])
print("BASE overLimit", len(sb["overLimit"]), sb["overLimit"])
print("NOW  nodes", sn["nodes"], "rows", sn["rows"], "members", sn["members"], "closureCompilable", sn["closureCompilable"])
print("NOW  overLimit", len(sn["overLimit"]))
for r in n["rows"]:
    if r.get("overLimit"):
        print("OVER", r["name"], r["manifest"], "files", r["files"], "bytes", r["bytes"], "lines", r["activeLines"], "oversizedDeps", r.get("oversizedDeps"))
print("NOW carvableLeaf", sn["carvableLeaf"])
ov = {r["name"]: r for r in n["rows"]}
for name in ["semio-framework-os-kernel", "semio-framework-os-renderer-wgpu", "semio-framework-plugin-host", "semio-framework-os-store", "semio-framework-os-store-sync", "semio-framework-plugin", "semio-framework-plugin-host-test-component"]:
    r = ov.get(name)
    print("ROW", name, None if r is None else (r["files"], r["bytes"], r["activeLines"], r["overLimit"], r.get("closureCompilable"), r.get("oversizedDeps")))
print("NEW crates count not in base:", len(set(ov) - set(r["name"] for r in b["rows"])))
print("base crates count not in now:", len(set(r["name"] for r in b["rows"]) - set(ov)))
