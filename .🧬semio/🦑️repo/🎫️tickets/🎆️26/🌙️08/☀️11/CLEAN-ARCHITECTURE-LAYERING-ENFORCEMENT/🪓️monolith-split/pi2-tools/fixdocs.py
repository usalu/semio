#!/usr/bin/env python3
"""📝️ Hoists the indented inner doc block that follows the first docstring of a cut piece into that docstring, so no later insertion can place an item before an inner doc comment.
usage: fixdocs.py [--apply]"""
import glob, re, sys
from lib import *
import ed

changed = []
for fam in ("🔬️plugin-runtime-plugin-builder-contract", "🔬️tool-run", "🧪️time-travel"):
    for f in sorted(glob.glob(PLUG + "/🧪️tests/" + fam + "-part-*/🦀️.rs")):
        if "-x/" in f:
            continue
        lines = read(f).split("\n")
        if not lines or not lines[0].startswith("//!"):
            continue
        i = 1
        while i < len(lines) and lines[i].startswith("//!"):
            i += 1
        j = i
        while j < len(lines) and not lines[j].strip():
            j += 1
        k = j
        while k < len(lines) and lines[k].lstrip().startswith("//!"):
            k += 1
        if k == j:
            continue
        new = lines[:i] + [l.strip() for l in lines[j:k]] + [""] + lines[k:]
        changed.append((f, k - j))
        if "--apply" in sys.argv:
            ed.put(f, "\n".join(new))
for f, n in changed:
    print(rel(f)[-60:], n)
