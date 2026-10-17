#!/usr/bin/env python3
"""🧪️ Parses every Rust source the companion bins, the builder-contract/tool-run/time-travel cuts and the facade reach with `rustfmt --emit stdout` (children skipped): a syntax error fails the file.
usage: parsecheck_w7.py"""
import glob, os, subprocess, sys
from lib import *
import tree as T

files = set()
roots = [PLUG + "/🦀️.rs"] + [d + "/🦀️.rs" for d in sorted(glob.glob(PLUG + "/🧷️companions/*tests-*"))]
for r in roots:
    for f, _, _ in T.tree(r):
        files.add(f)
for fam in ("🔬️plugin-runtime-plugin-builder-contract", "🔬️tool-run", "🧪️time-travel"):
    files |= set(glob.glob(PLUG + "/🧪️tests/" + fam + "*/🦀️.rs"))
bad = []
for f in sorted(files):
    if "-x/" in f:
        continue
    r = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", f], stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, timeout=60)
    if r.returncode != 0:
        bad.append((rel(f)[-70:], r.stderr.decode()[:150].replace("\n", " ")))
print("files", len(files), "bad", len(bad))
for b in bad[:20]:
    print(b)
