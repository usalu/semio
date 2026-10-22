#!/usr/bin/env python3
"""usage: parsecheck.py STEP [ROOT]   every created or edited ts/mjs/json/py file of the step parses in ROOT."""
import json, os, subprocess, sys
step = sys.argv[1]
root = sys.argv[2] if len(sys.argv) > 2 else "/private/tmp/i18n-core/t6"
d = "staged" if step == "core" else "staged-" + step
plan = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), d, "plan.json"), encoding="utf8"))
bad = n = 0
for row in plan["created"] + plan["edited"]:
    p = row["path"]
    n += 1
    if p.endswith((".ts", ".mjs", ".mts")):
        r = subprocess.run(["bun", "build", "--no-bundle", p], cwd=root, capture_output=True, text=True)
        if r.returncode:
            bad += 1
            print("BAD", p[-80:], r.stderr[:240])
    elif p.endswith(".json"):
        try:
            json.load(open(os.path.join(root, p), encoding="utf8"))
        except Exception as e:
            bad += 1
            print("BADJSON", p[-70:], e)
    elif p.endswith(".py"):
        if subprocess.run(["python3", "-m", "py_compile", os.path.join(root, p)], capture_output=True).returncode:
            bad += 1
            print("BADPY", p)
print("parsed", n, "bad", bad)
