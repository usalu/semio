#!/usr/bin/env python3
"""🧾️ Appends the verdict of every per-asset case of one vitest JSON result to done.txt (`id sha pass|FAIL message`): catalog-done.py <json> <shas.txt> <done.txt> <exit code> <peak MB>"""
import json, re, sys
path, shas, done, code, peak = sys.argv[1:6]
sha = {}
for line in open(shas):
    a, s = line.split(); sha[a] = s
try:
    cases = [c for f in json.load(open(path))["testResults"] for c in f["assertionResults"]]
except Exception as error:
    print(f"no result {path} exit {code}: {error}"); sys.exit(0)
out = open(done, "a"); n = [0, 0]
for c in cases:
    m = re.search(r"every authored scale (\S+) offers", c["fullName"])
    if not m:
        if c["status"] == "failed": print("SHARED", c["title"], " ".join(c["failureMessages"])[:300])
        continue
    ok = c["status"] == "passed"; n[ok] += 1
    out.write(f"{m[1]} {sha.get(m[1])} {'pass' if ok else c['status']} {int(c.get('duration') or 0)}ms {' '.join(' '.join(c['failureMessages']).split())[:1200]}\n")
print(f"{path.split('/')[-1]} exit {code} peak {peak} MB pass {n[1]} other {n[0]}", flush=True)
