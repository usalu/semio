#!/usr/bin/env python3
"""Cold-rebuild census of the simulated chain (reads the overlay and sim-a/result.json; read only).
For every step: files changed, graph-module-authority files changed (the implementation list of the graph module: any change makes every graph receipt invalid once), observation/inference files changed (project records re-infer),
distinct projects whose inputs changed (nearest 📋️project.json), manifests, lock files. Writes sim-a/impact.json and prints a table.
usage: impact.py [sim-dir] [overlay]"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
CREL = os.path.relpath(C, LIVE)
sim = sys.argv[1] if len(sys.argv) > 1 else C + "/🗑️generated/round6-apply/sim-a"
OV = sys.argv[2] if len(sys.argv) > 2 else "/private/tmp/i6/ov"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
GRAPH = B + "/🕸️graph/🟦️.ts"
text = open(os.path.join(OV, GRAPH), encoding="utf8").read()
line = next(l for l in text.split("\n") if "graphImplementationUrlsV1=" in l)
rels = re.findall(r'new URL\("([^"]+)"', line)
authority = {GRAPH}
for r in rels:
    authority.add(os.path.normpath(os.path.join(os.path.dirname(GRAPH), r)))
policy = re.search(r'graphSnapshotPolicyURLV1\s*=\s*new URL\("([^"]+)"', text)
if policy:
    authority.add(os.path.normpath(os.path.join(os.path.dirname(GRAPH), policy.group(1))))
inference = lambda f: f.startswith(B + "/🕸️graph/📥️inference/") or f.startswith(B + "/👁️observation/") and "/🧪️tests/" not in f
roots = {}


def project_of(rel):
    d = os.path.dirname(rel)
    while d:
        if d in roots:
            return roots[d]
        hit = os.path.isfile(os.path.join(OV, d, "📋️project.json"))
        roots[d] = d if hit else (project_of(d) if d != os.path.dirname(d) else None)
        return roots[d]
    return None


rows = json.load(open(sim + "/result.json", encoding="utf8"))["result"]
out, allprojects = {}, set()
for row in rows:
    files = [f for f in row["changed"] + row.get("removed", []) if not (f == CREL or f.startswith(CREL + "/")) and not f.startswith(".git/") and "/__pycache__/" not in f]
    projects = {p for p in (project_of(f) for f in files) if p}
    allprojects |= projects
    out[row["step"]] = {
        "files": len(files),
        "authority": sorted(f for f in files if f in authority),
        "inference": sum(1 for f in files if inference(f)),
        "projects": len(projects),
        "manifests": sum(1 for f in files if os.path.basename(f) in ("📋️project.json", "package.json")),
    }
json.dump({"authorityFiles": sorted(authority), "steps": out, "projectsTouched": len(allprojects)}, open(sim + "/impact.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("graph authority list: %d files; projects touched by the chain: %d" % (len(authority), len(allprojects)))
for step, v in out.items():
    print("%-28s files %5d  authority %2d  inference %3d  projects %4d  manifests %4d" % (step, v["files"], len(v["authority"]), v["inference"], v["projects"], v["manifests"]))
