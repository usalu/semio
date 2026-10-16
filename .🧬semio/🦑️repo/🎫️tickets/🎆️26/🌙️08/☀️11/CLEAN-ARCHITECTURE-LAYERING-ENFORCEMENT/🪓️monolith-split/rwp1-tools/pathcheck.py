#!/usr/bin/env python3
# 🧭️ Every `#[path]`, `include!`, `include_str!` and `include_bytes!` literal of the carved crates, the renderer hubs and the root resolves to an existing file (cfg ignored).
import json, os, re, sys
from common import *
import modtree
from rs import blank

carved = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/carved.json"))
roots = [T + "/" + CRATES[c] + "/🦀️.rs" for c in carved] + [R + "/🦀️.rs", T + "/" + PKG_REL.split("🧊️wgpu/")[1] + "/📚️library/🦀️.rs"]
files = []
seen = set()
for r in roots:
    for f in modtree.tree(r):
        if f not in seen:
            seen.add(f)
            files.append(f)
bad, total = [], 0
LIT = re.compile(r'(?:#\[\s*path\s*=\s*|include!\(\s*|include_str!\(\s*|include_bytes!\(\s*)"([^"]+)"')
for f in files:
    text = read(f)
    clean = blank(text)
    for m in LIT.finditer(text):
        if clean[m.start():m.start() + 4] in ("    ", "") and False:
            continue
        # skip occurrences inside comments/strings: the blanked text keeps the macro name only outside them
        head = text[m.start():m.start() + 6]
        if not clean[m.start()] == text[m.start()]:
            continue
        total += 1
        target = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
        if not os.path.exists(target):
            bad.append((f.replace(E + "/", ""), m.group(1)))
print("files", len(files), "literals", total, "unresolved", len(bad))
for b in bad[:20]:
    print("  ", b)
