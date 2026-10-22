#!/usr/bin/env python3
"""🔎️ Third-party parse check of a plan: every rewritten Rust source must still parse under rustfmt (stdin, no child resolution) when its original did. usage: verify_rustfmt.py <tag> [--root DIR]"""
import json, os, subprocess, sys, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
tag = sys.argv[1]
root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else os.path.abspath(HERE + "/" + "/".join([".."] * 9))
plan = json.load(open(os.path.join(HERE, "plans", tag + ".json"), encoding="utf-8"))
def ok(t):
    return subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=t, capture_output=True, text=True).returncode == 0
bad, pre_bad = [], 0
for f in plan["files"]:
    old = open(os.path.join(root, f["path"]), encoding="utf-8").read()
    new = old
    for a, b, o, n in [e[:4] for e in sorted(f["edits"], key=lambda e: e[0], reverse=True)]:
        new = new[:a] + n + new[b:]
    if ok(new): continue
    if ok(old): bad.append(f["path"])
    else: pre_bad += 1
print(f"{tag}: {len(plan['files'])} files, parse regressions {len(bad)}, already unparsable before {pre_bad}")
for b in bad[:10]: print("  ", b)
sys.exit(1 if bad else 0)
