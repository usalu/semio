#!/usr/bin/env python3
"""🔎️ Third-party parse check of an ordered list of plans: every rewritten (and created) Rust source must parse under rustfmt (stdin, no child resolution) when its original did. usage: verify_rustfmt_final.py <tag,tag,...> [--root DIR]"""
import json, os, subprocess, sys
import apply as A
tags = sys.argv[1].split(",")
root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else A.DEFAULT_ROOT
plans = A.load_plans(",".join(tags))
files, _ = A.collect(plans, set())
def ok(t): return subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=t, capture_output=True, text=True).returncode == 0
bad, pre_bad, n = [], 0, 0
for rel, entries in files.items():
    if not rel.endswith(".rs"): continue
    if entries[0].get("create"):
        n += 1
        if not ok(entries[0]["content"]): bad.append(rel)
        continue
    if entries[0].get("delete"): continue
    old = open(os.path.join(root, rel), encoding="utf-8").read()
    text = old
    for e in entries:
        text = A.new_text_of_file(text, e, A.sha(text.encode("utf-8")) == e["sha256"])
    n += 1
    if ok(text): continue
    if ok(old): bad.append(rel)
    else: pre_bad += 1
print(f"{','.join(tags)}: {n} files, parse regressions {len(bad)}, already unparsable before {pre_bad}")
for b in bad[:12]: print("  ", b)
sys.exit(1 if bad else 0)
