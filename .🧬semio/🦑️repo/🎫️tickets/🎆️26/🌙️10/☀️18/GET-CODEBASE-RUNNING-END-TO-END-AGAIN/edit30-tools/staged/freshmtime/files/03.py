"""🕰 Independent oracle for Cargo's output-time freshness rule over a plain target directory: a library unit is stale when a dependency is stale or the dependency's `.rmeta` is newer than the unit's newest output (Cargo `Fingerprint::check_filesystem`, pipelined lib edges compare the rmeta only).
usage: python3 -I 🐍️.py <target>/debug   prints the sorted crate names Cargo re-runs on the next unchanged build"""
import json, os, sys

profile = sys.argv[1]
units = {}
for name in os.listdir(os.path.join(profile, ".fingerprint")):
    directory = os.path.join(profile, ".fingerprint", name)
    for file in os.listdir(directory):
        if file.startswith("lib-") and file.endswith(".json"):
            crate, unit_hash = file[4:-5], name.rsplit("-", 1)[1]
            outputs = [os.path.join(profile, "deps", "lib%s-%s.%s" % (crate, unit_hash, ext)) for ext in ("rlib", "rmeta")]
            deps = [row[1] for row in json.load(open(os.path.join(directory, file)))["deps"]]
            units[crate] = {"outputs": [p for p in outputs if os.path.exists(p)], "deps": deps}
memo = {}
def stale(crate):
    if crate not in memo:
        unit = units[crate]
        newest = max(os.stat(p).st_mtime_ns for p in unit["outputs"])
        memo[crate] = any(stale(dep) or os.stat(next(p for p in units[dep]["outputs"] if p.endswith(".rmeta"))).st_mtime_ns > newest for dep in unit["deps"] if dep in units)
    return memo[crate]
print(json.dumps(sorted(crate for crate in units if stale(crate))))
