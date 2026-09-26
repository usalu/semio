"""🧮️ P8 offline census over the committed plugin descriptors: per verb kind, audience, interactive-job
classification, destructive flag and declared args (reuses D1's descriptor walk)."""
import json, sys, collections
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
import importlib.util
spec = importlib.util.spec_from_file_location("d1census", "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1/d1-census.py")
d1 = importlib.util.module_from_spec(spec); spec.loader.exec_module(d1)

rows = []
for plugin, app, cid, shape, row in d1.rows():
    aud, declared = d1.audience(row)
    sem = row.get("semantics", {})
    rows.append({"id": cid, "plugin": plugin, "app": app, "shape": shape, "kind": row["kind"], "audience": aud,
                 "job": sem.get("execution", {}).get("interactiveJob", "unclassified"),
                 "destructive": sem.get("effects", {}).get("destructive", False),
                 "args": [a.get("id") or a.get("key") for a in row.get("args", [])]})
json.dump(rows, open(sys.argv[1], "w"), indent=1)
by = collections.Counter((r["kind"], r["audience"], r["job"]) for r in rows)
for key, n in sorted(by.items()):
    print(n, *key)
print("total", len(rows))
agent_dead = [r for r in rows if r["audience"] == "agent" and r["job"] != "migrated"]
print("agent-facing non-migrated:", len(agent_dead))
per = collections.Counter(r["plugin"] for r in agent_dead)
print(dict(per))
