#!/usr/bin/env python3
# 🎨️ Sorts the leading `use` block of selected carve-owned files exactly as rustfmt does (the block alone is formatted; every other line stays byte-identical).
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
CONFIG = "skip_children=true,unstable_features=true,max_width=250,use_small_heuristics=Max,reorder_imports=true"
FILES = ["📐️part21/🧪️tests/🚦️sqlite-cohort/🦀️.rs", "🏗️assembly/🦀️.rs", "🪟️window-kit/🦀️.rs", "✏️editing/🩹️patch/🧪️tests/🦀️.rs", "✏️editing/🪟️details/🧪️tests/🔬️unit/🦀️.rs"]
for relative in FILES:
    path = C + "/" + relative
    lines = read(path).split("\n")
    start = next(i for i, line in enumerate(lines) if line.startswith("use "))
    end = start
    while end < len(lines) and lines[end].strip() != "":
        end += 1
    block = "\n".join(lines[start:end])
    done = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--config", CONFIG, "--emit", "stdout"], input=block + "\n", capture_output=True, text=True)
    assert done.returncode == 0, done.stderr
    new = done.stdout.strip("\n")
    if new != block:
        lines[start:end] = new.split("\n")
        write(path, "\n".join(lines))
        print("sorted", relative)
