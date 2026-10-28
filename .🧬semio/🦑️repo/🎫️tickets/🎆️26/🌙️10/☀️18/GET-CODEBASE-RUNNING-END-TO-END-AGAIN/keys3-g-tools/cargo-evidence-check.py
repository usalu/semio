#!/usr/bin/env python3
"""keys3-g: every target with oracle evidence of a cargo run must carry the resolution token (lockcov.jsonl from lockcov.mjs on the current generation). Exit 1 on a violation; `after` rows only warn before the batch landed.
usage: cargo-evidence-check.py <evidence.json> <lockcov.jsonl>"""
import json, sys
evidence = json.load(open(sys.argv[1], encoding="utf8"))["targets"]
rows = {f'{r["project"]}:{r["target"]}': r for r in map(json.loads, open(sys.argv[2], encoding="utf8"))}
bad = 0
for t in evidence:
    row = rows.get(t["id"])
    ok = bool(row and row["lock"])
    print(("ok   " if ok else ("WARN " if t["after"] else "FAIL ")) + t["id"] + (" (token)" if ok else " (no token)"))
    bad += (not ok) and not t["after"]
sys.exit(1 if bad else 0)
