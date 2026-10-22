#!/usr/bin/env python3
"""usage: runtests.py ROOT OUT.json DIR...   runs every test file below the DIRs (root-relative) one at a time (nice 10, skipped while the 1-minute load >= 45) and records pass/fail counts per file."""
import json, os, re, subprocess, sys, time

root, out, dirs = sys.argv[1], sys.argv[2], sys.argv[3:]
files = []
for d in dirs:
    for b, ds, ns in os.walk(os.path.join(root, d)):
        ds[:] = [x for x in ds if x not in ("node_modules", ".cache")]
        if "🧪️tests" in b.split(os.sep) or b.endswith("🧪️tests") or "/🧪️tests" in b:
            for n in ns:
                if n == "🟦️.ts" or n.endswith(".test.ts"):
                    files.append(os.path.relpath(os.path.join(b, n), root))
files.sort()
result = {}
for f in files:
    while os.getloadavg()[0] >= 45:
        time.sleep(5)
    t0 = time.time()
    try:
        run = subprocess.run(["nice", "-n", "10", "bun", "test", "./" + f], cwd=root, capture_output=True, text=True, timeout=600)
        text = run.stdout + run.stderr
    except subprocess.TimeoutExpired:
        text = "TIMEOUT"
    p = re.search(r"^\s*(\d+) pass", text, re.M); fl = re.search(r"^\s*(\d+) fail", text, re.M)
    result[f] = {"pass": int(p.group(1)) if p else 0, "fail": int(fl.group(1)) if fl else -1, "secs": round(time.time() - t0)}
json.dump(result, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(files), "files;", sum(v["pass"] for v in result.values()), "pass;", sum(max(v["fail"], 0) for v in result.values()), "fail;", sum(1 for v in result.values() if v["fail"] < 0), "crashed")
