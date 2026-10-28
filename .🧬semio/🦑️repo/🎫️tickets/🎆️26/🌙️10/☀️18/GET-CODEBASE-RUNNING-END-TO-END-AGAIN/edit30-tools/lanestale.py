"""🕰 Read-only: which units of a unit-cache lane (Cargo new build-dir layout) Cargo would re-run on the next unchanged build by its output-time rule, and why (oldest restored outputs vs newer dependency rmeta).
usage: python3 -I lanestale.py <lane>/b/<profile>   prints counts and the stale units with their reason"""
import json, os, sys

build = os.path.join(sys.argv[1], "build")
units, by_hash = {}, {}
for pkg in os.listdir(build):
    for unit_hash in os.listdir(os.path.join(build, pkg)):
        fp = os.path.join(build, pkg, unit_hash, "fingerprint")
        if not os.path.isdir(fp): continue
        for file in os.listdir(fp):
            if file.startswith("lib-") and file.endswith(".json"):
                crate = file[4:-5]
                outs = [os.path.join(build, pkg, unit_hash, "out", "lib%s-%s.%s" % (crate, unit_hash, ext)) for ext in ("rlib", "rmeta")]
                outs = [p for p in outs if os.path.exists(p)]
                if not outs: continue
                row = json.load(open(os.path.join(fp, file)))
                fingerprint = open(os.path.join(fp, file[:-5])).read().strip()
                key = (pkg, unit_hash)
                units[key] = {"crate": crate, "outs": outs, "deps": [d[3] for d in row["deps"]]}
                by_hash[int.from_bytes(bytes.fromhex(fingerprint), "little")] = key
memo, why = {}, {}
def stale(key):
    if key in memo: return memo[key]
    memo[key] = False
    unit = units[key]; newest = max(os.stat(p).st_mtime for p in unit["outs"])
    for dep_hash in unit["deps"]:
        dep = by_hash.get(dep_hash)
        if dep is None: continue
        if stale(dep): memo[key] = True; why[key] = "dependency %s stale" % units[dep]["crate"]; break
        meta = [p for p in units[dep]["outs"] if p.endswith(".rmeta")] or units[dep]["outs"]
        if os.stat(meta[0]).st_mtime > newest: memo[key] = True; why[key] = "dependency %s newer by %.1f s" % (units[dep]["crate"], os.stat(meta[0]).st_mtime - newest); break
    return memo[key]
bad = [k for k in units if stale(k)]
print("units", len(units), "stale", len(bad), "root causes", sum(1 for k in bad if "newer" in why[k]))
for k in sorted(bad, key=lambda k: units[k]["crate"]): print(" ", units[k]["crate"], "-", why[k])
