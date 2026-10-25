"""Groups executor source traces by action class: python3 read-traces.py <traceDir>... [--json out.json]
A class is (command, workerKind, executionKind, semioKind) from the `a` record of a process; processes without one (planner, loader boot) are listed under class null."""
import json, os, sys
from collections import defaultdict
args = sys.argv[1:]; out = None
if "--json" in args:
    i = args.index("--json"); out = args[i + 1]; args = args[:i] + args[i + 2:]
classes = defaultdict(lambda: {"processes": 0, "m": set(), "r": set(), "p": set(), "x": set(), "ids": set()})
for directory in args:
    for name in sorted(os.listdir(directory)):
        if not name.endswith(".trace"): continue
        rows = [line.rstrip("\n") for line in open(os.path.join(directory, name), encoding="utf-8")]
        kind = None; ident = None
        for row in rows:
            if row.startswith("a "):
                a = json.loads(row[2:]); kind = (a["command"], a["workerKind"], a["executionKind"], a["semioKind"], a.get("owner", "none")); ident = a["id"]
        entry = classes[kind]; entry["processes"] += 1
        if ident: entry["ids"].add(ident)
        for row in rows:
            k, _, p = row.partition(" ")
            if k in ("m", "r", "p", "x"): entry[k].add(p)
report = {}
for kind, e in sorted(classes.items(), key=lambda kv: str(kv[0])):
    key = "|".join(str(v) for v in kind) if kind else "null"
    report[key] = {"processes": e["processes"], "ids": sorted(e["ids"]), **{k: sorted(e[k]) for k in ("m", "r", "p", "x")}}
    print(key, "processes", e["processes"], "modules", len(e["m"]), "reads", len(e["r"]), "phys", len(e["p"]), "live", len(e["x"]), "ids", len(e["ids"]))
if out: json.dump(report, open(out, "w"), ensure_ascii=False, indent=1)
