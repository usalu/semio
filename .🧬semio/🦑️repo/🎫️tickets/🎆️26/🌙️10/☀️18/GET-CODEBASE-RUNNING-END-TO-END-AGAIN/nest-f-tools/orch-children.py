import json, subprocess, sys
files = subprocess.run(["rg", "--files", "-g", "📋️project.json", "-g", "!node_modules", "-g", "!**/🗑️generated/**", "/Users/ueli/Documents/semio"], capture_output=True, text=True).stdout.split("\n")
projects = {}
for f in files:
    if not f: continue
    try: d = json.load(open(f, encoding="utf-8"))
    except Exception: continue
    projects[d.get("name", f)] = d
rows = []
for name, d in projects.items():
    for tname, t in (d.get("targets") or {}).items():
        ex = ((t.get("metadata") or {}).get("semio") or {}).get("execution") or {}
        if ex.get("kind") == "orchestration" and ex.get("children") is not None:
            for c in ex["children"]:
                cp = projects.get(c["project"]); ct = ((cp or {}).get("targets") or {}).get(c["target"]) or {}
                inputs = ct.get("inputs") or []
                objs = [i for i in inputs if isinstance(i, dict)]
                deps = ct.get("dependsOn") or []
                rows.append((name, tname, c["project"], c["target"], c["project"] != name, [o for o in objs if "projects" in o], [x for x in deps if isinstance(x, dict) or (isinstance(x, str) and ":" in x)]))
print(len(rows), "orchestration child declarations")
for r in rows:
    flag = "CROSS" if r[4] else "same "
    extra = ("INPUT-PROJECTS " + json.dumps(r[5], ensure_ascii=False)) if r[5] else ""
    dep = ("DEPS " + json.dumps(r[6], ensure_ascii=False)) if r[6] else ""
    print(flag, r[0], r[1], "->", r[2], r[3], extra, dep)
