#!/usr/bin/env python3
"""usage: inc.py STEP [ROOT]   builds one increment against the chain overlay, records the baseline of the test directories nearest to every edited file,
applies the step, re-runs them and prints what changed plus the ratchet numbers. Nothing here writes outside C/ and /private/tmp/i18n-core/."""
import json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
step = sys.argv[1]
root = sys.argv[2] if len(sys.argv) > 2 else "/private/tmp/i18n-core/t6"
res = "/private/tmp/i18n-core/res"
os.makedirs(res, exist_ok=True)


def sh(*args):
    return subprocess.run(list(args), cwd=HERE, capture_output=True, text=True)


b = sh("python3", "build.py", "--step", step, "--root", root)
print(b.stdout.strip(), b.stderr.strip()[-300:])
if b.returncode:
    sys.exit(2)
plan = json.load(open(os.path.join(HERE, "staged-" + step, "plan.json"), encoding="utf8"))
dirs = set()
for row in plan["edited"] + plan["created"]:
    rel = row["path"]
    if "/🧪️tests/" in rel or rel.endswith(".json") and "/📖️texts/" in rel:
        continue
    d = os.path.dirname(rel)
    while d and d.count("/") >= 5:
        if os.path.isdir(os.path.join(root, d, "🧪️tests")):
            dirs.add(os.path.join(d, "🧪️tests"))
            break
        d = os.path.dirname(d)
dirs = sorted(dirs)
print("test directories:", len(dirs))
before, after = os.path.join(res, step + "-before.json"), os.path.join(res, step + "-after.json")
r = sh("python3", "runtests.py", root, before, *dirs)
print("baseline:", r.stdout.strip())
a = sh("python3", "apply.py", "--step", step, "--root", root, "--yes")
print(a.stdout.strip().splitlines()[-1] if a.stdout.strip() else a.stderr[-300:])
if a.returncode:
    sys.exit(3)
new_dirs = sorted(set(dirs) | {os.path.dirname(row["path"]) for row in plan["created"] if row["path"].endswith("/🧪️tests/🟦️.ts")})
r = sh("python3", "runtests.py", root, after, *new_dirs)
print("after:", r.stdout.strip())
print(sh("python3", "compare.py", before, after).stdout.strip())
