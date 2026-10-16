#!/usr/bin/env python3
# 🔒️ Which dependency names of each carved crate manifest are missing from (or extra in) its Cargo.lock entry (the lock needs `update --workspace --offline` through the launcher).
import json, re, os
from common import *
carved = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/carved.json"))
lock = read(REPO + "/Cargo.lock")
blocks = {}
for m in re.finditer(r'(?ms)^\[\[package\]\]\nname = "([^"]+)"\nversion = "([^"]+)"\n(.*?)(?=^\[\[package\]\]|\Z)', lock):
    blocks.setdefault(m.group(1), []).append(m.group(3))
tot = 0
for c in carved:
    name = PKG_PREFIX + c
    t = read(T + "/" + CRATES[c] + "/📦️packages/🦀️rust/Cargo.toml")
    want = set()
    sec = ""
    for line in t.split("\n"):
        h = re.match(r"\[(.+)\]\s*$", line.strip())
        if h:
            sec = h.group(1)
            continue
        m = re.match(r"([A-Za-z0-9_\-]+)\s*=\s*(.*)$", line.strip())
        if m and "dependencies" in sec:
            pm = re.search(r'package\s*=\s*"([^"]+)"', m.group(2))
            want.add(pm.group(1) if pm else m.group(1))
    if name not in blocks:
        print(c, "NOT IN LOCK")
        continue
    have = set(re.findall(r'"([A-Za-z0-9_\-]+)(?: [0-9][^"]*)?"', re.search(r"dependencies = \[(.*?)\]", blocks[name][0], re.S).group(1))) if "dependencies = [" in blocks[name][0] else set()
    add, extra = sorted(want - have), sorted(have - want)
    tot += len(add) + len(extra)
    print(f"{c}: lock lacks {add} | lock has extra {extra}")
print("total differences", tot)
