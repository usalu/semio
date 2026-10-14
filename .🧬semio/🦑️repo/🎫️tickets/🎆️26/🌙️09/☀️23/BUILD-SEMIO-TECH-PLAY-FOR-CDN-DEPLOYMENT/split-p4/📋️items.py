#!/usr/bin/env python3
"""📋️ Lists the top-level items of one Rust file: index, start line, line count, kind, name (impl target/trait), region.

usage: 📋️items.py <file.rs>
"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ns = {"__file__": os.path.join(HERE, "🔀️item-split.py")}
exec(compile(open(os.path.join(HERE, "🔀️item-split.py"), encoding="utf-8").read().split("\nif __name__")[0], "item-split", "exec"), ns)
src, items = ns["parse"](sys.argv[1])
for it in items:
    line = src.count("\n", 0, it.start) + 1
    n = it.text.count("\n") + 1
    name = it.name if it.kind != "impl" else f"impl {it.trait + ' for ' if it.trait else ''}{it.target}"
    print(f"{it.i}\t{line}\t{n}\t{it.kind}\t{name[:90]}\t{'/'.join(it.region)[:40]}")
