#!/usr/bin/env python3
"""🧮️ Groups the failing rows of one per-test result file by a normalized panic message.

usage: 🧮️buckets.py <package.jsonl> [--names]
"""
import collections, json, re, sys
rows = {}
for l in open(sys.argv[1]):
    try: r = json.loads(l); rows[r["name"]] = r
    except Exception: pass
b = collections.defaultdict(list)
for n, r in rows.items():
    if r["status"] in ("pass", "ignored"): continue
    if r["status"] == "timeout": b["TIMEOUT"].append(n); continue
    t = r["tail"]; i = t.find("panicked at")
    msg = t[i:].split("\n", 2)[1] if i >= 0 else t.strip().splitlines()[-1] if t.strip() else "?"
    msg = re.sub(r"\d+", "N", msg)[:220]
    b[msg].append(n)
for k, v in sorted(b.items(), key=lambda kv: -len(kv[1])):
    print(f"{len(v):4}  {k}")
    if "--names" in sys.argv:
        for n in v: print("        ", n)
