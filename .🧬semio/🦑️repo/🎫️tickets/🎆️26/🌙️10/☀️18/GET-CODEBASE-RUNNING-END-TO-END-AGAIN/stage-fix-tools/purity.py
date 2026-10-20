#!/usr/bin/env python3
# usage: purity.py PLAN.json   runs every set's own `check` and `apply --dry-run` (with and without --ignore-sync) and proves that no target path changed
import hashlib, json, os, subprocess, sys
ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../../../../../../.."))
TICKET = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
plan = json.load(open(sys.argv[1], encoding="utf-8"))
paths = sorted({p for s in plan["sets"] for t in s["targets"] for p in [t["path"], *t.get("alsoTouches", [])]})
def digest():
    h = hashlib.sha256()
    for p in paths:
        full = os.path.join(ROOT, p)
        h.update(p.encode()); h.update((hashlib.sha256(open(full, "rb").read()).hexdigest() if os.path.isfile(full) else "-").encode())
    return h.hexdigest()
before = digest()
print("target paths", len(paths))
for s in plan["sets"]:
    if not s.get("entry"): continue
    cwd = os.path.join(TICKET, s["dir"])
    for extra in ([], ["--ignore-sync"]):
        for args in (["check"], ["apply", "--dry-run", *extra]):
            if args[0] == "check" and extra: continue
            run = subprocess.run([*s["entry"], *args], cwd=cwd, capture_output=True, text=True)
            print("%-15s %-28s rc=%d" % (s["id"], " ".join(args), run.returncode))
after = digest()
print("targets unchanged:", before == after)
sys.exit(0 if before == after else 1)
