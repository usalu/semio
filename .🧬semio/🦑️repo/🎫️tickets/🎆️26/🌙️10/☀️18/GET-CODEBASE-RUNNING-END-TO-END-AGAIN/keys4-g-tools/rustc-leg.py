#!/usr/bin/env python3
"""🔬 rustc-leg.py <tag> <target|ALL> <allow-dir> : the COMPILE-time leg without a cargo build. The path-dependency closure of the crates a target tests (the Cargo.toml walk of depinfo.py) is compiled with `rustc --emit=dep-info` per entry point (pathmod-g's rustc-groundtruth probe, all features on, lib+test flavors); every workspace file rustc lists must be inside the declared allow list. Writes evidence/<tag>-<target>-rustc.json. Streams, RSS guard 600 MB."""
import json, os, resource, sys
K = os.path.dirname(os.path.abspath(__file__))
W = "/Users/ueli/Documents/semio"
C = W + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag, which, allow_dir = sys.argv[1], sys.argv[2], sys.argv[3]
depinfo = open(K + "/depinfo.py", encoding="utf-8").read()
probe = open(C + "/pathmod-g-tools/rustc-groundtruth.py", encoding="utf-8").read()
empty = os.environ.get("TMPDIR", "/tmp") + "/keys4r-empty-census.json"
open(empty, "w").write("[]")
sys.argv = ["depinfo", tag, which]
scope = {"__name__": "depinfo"}
exec(compile(depinfo[:depinfo.index("dinfos = {}")], "depinfo-head", "exec"), scope)
sys.argv = ["probe", empty, "/dev/null"]
pscope = {"__name__": "probe"}
exec(compile(probe[:probe.index("report = {")], "probe-head", "exec"), pscope)
package_of, closure, CRATES = scope["package_of"], scope["closure"], scope["CRATES"]
for pair in filter(None, os.environ.get("KEYS4_CRATES", "").split(";")): CRATES[pair.split("=")[0]] = pair.split("=")[1].split(",")
names = list(CRATES) if which == "ALL" else which.split(",")
for target in names:
    declared = set(open(f"{allow_dir}/{target.replace('dash:', '')}.allow", encoding="utf-8").read().split("\n")) - {""}
    members = set()
    for root in CRATES.get(target, []):
        if root in package_of: members |= closure(root)
    reads, errors = set(), []
    for member in sorted(members):
        base = package_of[member]
        row, union, errs = pscope["run"]({"root": os.path.relpath(base, W), "name": member, "dep": {"status": "ok"}})
        reads |= union["named"] | union["dep"]
        errors += [(member, *e) for e in errs]
    reads = {r for r in reads if not r.startswith(".") and "/node_modules/" not in r and "/target/" not in r}
    undeclared = sorted(r for r in reads if r not in declared)
    out = {"target": target, "closureCrates": len(members), "workspaceFilesRead": len(reads), "declared": len(declared), "undeclared": undeclared, "probeErrors": errors[:20]}
    json.dump(out, open(f"{K}/evidence/{tag}-{target.replace(':', '_')}-rustc.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps({k: (v if k != "undeclared" else len(v)) for k, v in out.items() if k != "probeErrors"} | {"probeErrors": len(errors), "sample": undeclared[:4]}, ensure_ascii=False), flush=True)
assert resource.getrusage(resource.RUSAGE_SELF).ru_maxrss < 600 * 1048576, "RSS above 600 MB"
