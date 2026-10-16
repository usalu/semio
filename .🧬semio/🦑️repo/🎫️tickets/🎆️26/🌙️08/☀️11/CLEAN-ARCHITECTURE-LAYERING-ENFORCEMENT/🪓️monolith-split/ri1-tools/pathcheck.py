#!/usr/bin/env python3
# 🧭️ Every `#[path]`, `include!`, `include_str!` and `include_bytes!` string literal of the touched files, the real crate trees and the hubs resolves to an existing file; also the longest path.
import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import modtree
from rs import blank
table = crate_table()
real = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/real.json"))
files = set()
for c in real:
    files |= set(modtree.tree(T + "/" + table[c] + "/🦀️.rs"))
files |= set(modtree.tree(R + "/🦀️.rs"))
for l in open(os.path.dirname(os.path.abspath(__file__)) + "/touched.txt", encoding="utf8"):
    l = l.strip()
    if l and not l.startswith("#") and l.endswith(".rs"):
        files.add(E + "/" + l)
PAT = re.compile(r'(?:#\[\s*path\s*=\s*"([^"]+)"\s*\]|\binclude(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\))')
n = miss = 0
longest = (0, "")
for f in sorted(files):
    if not os.path.exists(f):
        continue
    text = read(f)
    clean = blank(text)
    for m in PAT.finditer(text):
        if clean[m.start():m.start() + 2] == "  " :
            continue
        lit = m.group(1) or m.group(2)
        n += 1
        target = os.path.normpath(os.path.join(os.path.dirname(f), lit))
        if not os.path.exists(target):
            miss += 1
            print("MISSING", f.replace(E + "/", "E/")[:120], "->", lit[:100])
        b = len(os.path.relpath(target, REPO).encode("utf8"))
        if b > longest[0]:
            longest = (b, target)
print(n, "literals;", miss, "missing; longest target path", longest[0], "bytes")
