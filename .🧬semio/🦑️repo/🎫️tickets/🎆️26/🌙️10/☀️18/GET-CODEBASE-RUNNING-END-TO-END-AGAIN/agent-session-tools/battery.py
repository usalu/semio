#!/usr/bin/env python3
"""Runs the test battery (battery.txt) in one overlay, one bun process at a time under nice -n 10, and prints one JSON row per file: python3 battery.py <overlay> [--only substring] [--skip-missing]"""
import json, os, re, subprocess, sys, time
ov = sys.argv[1]
only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else ""
HERE = os.path.dirname(os.path.abspath(__file__))
for line in open(os.path.join(HERE, "battery.txt"), encoding="utf8"):
    path = line.strip()
    if not path or only not in path:
        continue
    if not os.path.exists(os.path.join(ov, path)):
        print(json.dumps({"file": path, "missing": True}, ensure_ascii=False)); continue
    start = time.time()
    p = subprocess.run(["nice", "-n", "10", "bun", "test", "./" + path], cwd=ov, capture_output=True, text=True, timeout=1500)
    out = p.stdout + p.stderr
    passed = int((re.search(r"^\s*(\d+) pass", out, re.M) or [0, 0])[1]); failed = int((re.search(r"^\s*(\d+) fail", out, re.M) or [0, 0])[1])
    fails = sorted(set(re.findall(r"^\(fail\) (.*?)(?: \[[\d.]+ms\])?$", out, re.M)))
    print(json.dumps({"file": path, "pass": passed, "fail": failed, "failing": fails, "seconds": round(time.time() - start, 1), "code": p.returncode}, ensure_ascii=False), flush=True)
