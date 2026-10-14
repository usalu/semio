#!/usr/bin/env python3
"""📋️ Lists the top-level items of one Rust file: line, kind, name (impl target), region path.

usage: 📋️items.py <file.rs>
"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ITEM_SPLIT = os.path.join(HERE, "..", "e3-format-split", "🔀️item-split.py")
ns = {"__file__": ITEM_SPLIT}
exec(compile(open(ITEM_SPLIT, encoding="utf-8").read().split("\ndef analyse")[0], ITEM_SPLIT, "exec"), ns)
src, items = ns["parse"](sys.argv[1])
for it in items:
    name = it.name if it.kind != "impl" else f"impl {it.target}" + (f" ({it.trait.strip()})" if it.trait else "")
    print(f"{src[:it.start].count(chr(10)) + 1}\t{it.text.count(chr(10)) + 1}\t{it.kind}\t{name}\t{' / '.join(it.region)}")
