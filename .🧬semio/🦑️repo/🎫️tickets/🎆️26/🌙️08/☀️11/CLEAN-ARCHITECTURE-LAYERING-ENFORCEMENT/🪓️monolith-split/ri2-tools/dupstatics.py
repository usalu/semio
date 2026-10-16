#!/usr/bin/env python3
# 🧬 Statics, thread-locals and constants defined by more than one compilation unit (crate or the monolith): a singleton that exists twice forks silently.
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import modtree
from rs import blank
table = crate_table()
real = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/real.json"))
STATIC = re.compile(r"(?m)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?static\s+(?:mut\s+)?([A-Z][A-Z0-9_]+)\s*:")
owners = collections.defaultdict(lambda: collections.defaultdict(set))
units = {c: set(modtree.tree(T + "/" + table[c] + "/🦀️.rs")) for c in real}
mono = set(modtree.tree(R + "/🦀️.rs"))
units["<monolith>"] = mono
for u, files in units.items():
    for f in files:
        if "/🧪️tests/" in f:
            continue
        for m in STATIC.finditer(blank(read(f))):
            owners[m.group(1)][u].add(f.replace(E + "/", "E/")[-60:])
multi = {k: v for k, v in owners.items() if len(v) > 1}
print(len(owners), "statics;", len(multi), "defined by several units")
for k, v in sorted(multi.items()):
    print("  ", k, {u: sorted(fs)[:2] for u, fs in v.items()})
