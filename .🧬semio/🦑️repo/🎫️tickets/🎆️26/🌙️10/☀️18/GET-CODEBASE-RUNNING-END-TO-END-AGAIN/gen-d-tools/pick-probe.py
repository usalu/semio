"""Chooses the probe file for edit-proof.py from the declared selections of the eight course actions (selected.mjs output per action, JSON with `files`):
a production TypeScript/CSS file under 🧰️framework that course:build selects and no other action selects, untouched by version control for two days.
usage: python3 pick-probe.py <dir with <name>.json selections> <course-build-name> [limit]"""
import json, os, subprocess, sys, time
directory, course = sys.argv[1], sys.argv[2]; limit = int(sys.argv[3]) if len(sys.argv) > 3 else 10
selections = {n[:-5]: set(json.load(open(os.path.join(directory, n), encoding="utf-8"))["files"]) for n in os.listdir(directory) if n.endswith(".json") and not n.startswith("probe")}
others = set().union(*(v for k, v in selections.items() if k != course))
now = time.time(); found = []
for path in sorted(selections[course] - others):
    if not path.startswith("🧰️framework/") or "🧪️tests" in path or not path.endswith((".ts", ".tsx", ".css")): continue
    out = subprocess.run(["git", "log", "-1", "--format=%ct", "--", path], cwd="/Users/ueli/Documents/semio", capture_output=True, text=True).stdout.strip()
    if out and now - int(out) > 2 * 86400: found.append(path)
    if len(found) >= limit: break
print("\n".join(found))
