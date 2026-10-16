#!/usr/bin/env python3
# 📊️ Markdown tables of the RENDERER-WP1 result: crate graph and census rows (reads rwp1-census-final.tsv).
import json, re, sys
from common import *
import modtree

carved = json.load(open("carved.json"))
rows = {}
for line in open(GEN + "/rwp1-census-final.tsv", encoding="utf8"):
    p = line.rstrip("\n").split("\t")
    if len(p) >= 7 and p[1] == "native" and p[3] != "ERROR":
        rows[(p[0], p[2])] = p
print("| crate | module files owned | first-party deps (declared) | lib files / bytes / lines | test files / bytes / lines |")
print("|---|---:|---|---|---|")
for c in carved:
    t = read(T + "/" + CRATES[c] + "/📦️packages/🦀️rust/Cargo.toml")
    deps = sorted(set(re.findall(r"(?m)^" + re.escape(PKG_PREFIX) + r"([a-z0-9\-]+) *=", t)))
    files = [f for f in modtree.tree(T + "/" + CRATES[c] + "/🦀️.rs") if "/🧪️tests/" not in f and not f.endswith("/tests.rs")]
    lib, test = rows[(PKG_PREFIX + c, "lib")], rows[(PKG_PREFIX + c, "test")]
    print(f"| `{c}` | {len(files)} | {', '.join('`'+d+'`' for d in deps) or '-'} | {lib[3]} / {int(lib[4]):,} / {int(lib[5]):,} | {test[3]} / {int(test[4]):,} / {int(test[5]):,} |")
