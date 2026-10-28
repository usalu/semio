"""🗺️ Streams every tracked 📋️project.json (outside tickets) into a TSV: name, dir, tags, targets, depends."""
import json, subprocess, sys, resource
root = sys.argv[1]
out = open(sys.argv[2], "w", encoding="utf8")
proc = subprocess.Popen(["git", "-C", root, "ls-files", "-z", "--", "*📋️project.json"], stdout=subprocess.PIPE)
n = 0
for raw in proc.stdout.read().split(b"\0"):
    if not raw: continue
    rel = raw.decode()
    if rel.startswith(".🧬semio/"): continue
    try:
        d = json.load(open(f"{root}/{rel}", encoding="utf8"))
    except Exception as e:
        out.write(json.dumps({"rel": rel, "error": str(e)}, ensure_ascii=False) + "\n"); continue
    t = d.get("targets", {}) or {}
    row = {"rel": rel, "name": d.get("name"), "tags": d.get("tags"), "targets": sorted(t), "keys": sorted(d)[:12],
           "impl": d.get("implicitDependencies"), "projectType": d.get("projectType")}
    out.write(json.dumps(row, ensure_ascii=False) + "\n"); n += 1
    assert resource.getrusage(resource.RUSAGE_SELF).ru_maxrss < 500 * 1024 * 1024
print(n)
