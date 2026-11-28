#!/usr/bin/env python3
"""H116: run bun test files under the observed-something preload; list files whose tests made no assertion.
Usage: observed-dynamic.py <preload> <out.json> <timeout-seconds> <file>...   (files from stdin when none given)"""
import json, os, re, signal, subprocess, sys, time
pre, out, limit, *files = sys.argv[1:]
files = files or [l.strip() for l in sys.stdin if l.strip()]
rows = []
for f in files:
    t0 = time.time()
    p = subprocess.Popen(["nice", "-n", "10", "bun", "test", "--timeout", "30000", "--preload", pre, "./" + f],
                         stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, start_new_session=True)
    try:
        text, _ = p.communicate(timeout=int(limit))
        status = "ran"
    except subprocess.TimeoutExpired:
        os.killpg(p.pid, signal.SIGTERM)
        text, _ = p.communicate()
        status = "timeout"
    lines = text.splitlines()
    unobserved, cur = [], None
    for i, l in enumerate(lines):
        if "received 0 assertions" in l:
            for m in lines[i + 1:i + 4]:
                g = re.match(r"\(fail\) (.*?)( \[[\d.]+m?s\])?$", m)
                if g:
                    unobserved.append(g.group(1)); break
    pas = re.search(r"^\s*(\d+) pass", text, re.M); fail = re.search(r"^\s*(\d+) fail", text, re.M)
    rows.append({"file": f, "status": status, "pass": int(pas.group(1)) if pas else None, "fail": int(fail.group(1)) if fail else None,
                 "unobserved": unobserved, "seconds": round(time.time() - t0, 1)})
    print(f"{rows[-1]['status']:7} p={rows[-1]['pass']} f={rows[-1]['fail']} unobserved={len(unobserved)} {rows[-1]['seconds']}s {f[-70:]}", flush=True)
json.dump(rows, open(out, "w"), ensure_ascii=False, indent=1)
