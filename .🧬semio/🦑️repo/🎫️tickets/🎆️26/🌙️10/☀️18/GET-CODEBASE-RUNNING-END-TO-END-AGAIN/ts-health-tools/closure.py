import json, os, subprocess, sys, collections
root = "/Users/ueli/Documents/semio"
os.chdir(root)
files = subprocess.run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*project.json"], capture_output=True).stdout.decode().split("\0")
files = [f for f in files if f.endswith("project.json") and "node_modules" not in f and f]
projects = {}
for f in files:
    try:
        d = json.load(open(f))
    except Exception as e:
        continue
    if "name" in d:
        projects[d["name"]] = (os.path.dirname(f), d)
print("projects", len(projects), file=sys.stderr)
pkg_by_dir = {}
for name, (dir_, d) in projects.items():
    pj = os.path.join(dir_, "package.json")
    if os.path.exists(pj):
        try:
            pkg_by_dir[name] = json.load(open(pj))
        except Exception:
            pass
roots = ["@semio-tech/semio-tech-play", "@semio-tech/mit-bestand-demonstrator", "@teaching/architecture-course"]
seen = set()
edges = collections.defaultdict(set)
stack = list(roots)
while stack:
    n = stack.pop()
    if n in seen or n not in projects:
        continue
    seen.add(n)
    dir_, d = projects[n]
    deps = set()
    for t in d.get("targets", {}).values():
        for x in t.get("dependsOn", []) or []:
            if isinstance(x, str) and ":" in x:
                deps.add(x.rsplit(":", 1)[0])
            elif isinstance(x, dict) and x.get("projects"):
                p = x["projects"]
                deps.update(p if isinstance(p, list) else [p])
    for x in d.get("implicitDependencies", []) or []:
        deps.add(x)
    pk = pkg_by_dir.get(n, {})
    for k in ("dependencies", "devDependencies", "peerDependencies", "optionalDependencies"):
        for dn in pk.get(k, {}):
            if dn in projects:
                deps.add(dn)
    deps.discard(n)
    edges[n] = deps
    stack.extend(deps)
out = {}
for n in sorted(seen):
    dir_, d = projects[n]
    tg = d.get("targets", {})
    out[n] = {"dir": dir_, "typecheck": "typecheck" in tg, "targets": sorted(tg.keys()), "deps": sorted(edges[n])}
json.dump(out, open(sys.argv[1], "w"), indent=1, ensure_ascii=False)
print(len(seen), "in closure;", sum(1 for v in out.values() if v["typecheck"]), "with typecheck", file=sys.stderr)
