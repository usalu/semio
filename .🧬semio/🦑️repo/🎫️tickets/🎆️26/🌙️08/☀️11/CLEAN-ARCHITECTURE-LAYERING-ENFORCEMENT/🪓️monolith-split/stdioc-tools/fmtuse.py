#!/usr/bin/env python3
# 🎨️ Formats exactly the `use` statements this carve wrote or changed (rustfmt on each statement alone), leaving every moved line byte-identical.
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
CONFIG = "skip_children=true,unstable_features=true,max_width=250,use_small_heuristics=Max,reorder_imports=false"
MINE = re.compile(r"^(pub\s+)?use\s+(semio_s_artifact_stdio_contract_[a-z0-9_]+|semio_framework_[a-z0-9_]+\s+as\s+[a-z_]+)|^use\s+semio_framework_plugin::\{[A-Za-z]+\};$")
def fmt(statement):
    done = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--config", CONFIG, "--emit", "stdout"], input=statement + "\n", capture_output=True, text=True)
    assert done.returncode == 0, done.stderr
    return done.stdout.strip("\n")
files = set()
for crate in CRATES:
    files |= set(closure(crate["root"]))
files.discard(C + "/🦀️.rs")
changed = 0
for path in sorted(files):
    text = read(path)
    out = []
    for line in text.split("\n"):
        if line.rstrip().endswith("{"):
            out.append(line)
            continue
        if MINE.match(line) or (line.startswith("use ") and len(line) > 250 and ("semio_s_artifact_stdio_contract_" in line or line.startswith("use semio_framework_plugin::{"))):
            new = fmt(line)
            if new != line:
                changed += 1
                out.append(new)
                continue
        out.append(line)
    new_text = "\n".join(out)
    if new_text != text:
        write(path, new_text)
print("statements reformatted:", changed)
