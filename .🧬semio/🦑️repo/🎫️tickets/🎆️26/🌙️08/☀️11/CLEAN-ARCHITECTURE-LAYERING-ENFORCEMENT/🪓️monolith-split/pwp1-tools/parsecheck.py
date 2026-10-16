#!/usr/bin/env python3
# 🧾️ rustfmt parse check (skip_children) of every .rs file of the WP1 trees plus the hub, root and macro mount.
import subprocess, sys
from lib import *
import wp1files
from crates_wp1 import ROOT

files = set()
for crate, rootdir in ROOT.items():
    for f, _ in wp1files.tree(PLUG + "/" + rootdir + "/🦀️.rs"):
        files.add(f)
files |= {PLUG + "/🦀️.rs", PLUG + "/🪝️mounts/🌱️wp1-foundation/🦀️.rs", PLUG + "/📣️macros/🌱️wp1-window/🦀️.rs", PLUG + "/⚛️reactor/🦀️.rs", PLUG + "/🗃️document/🦀️.rs", PLUG + "/⏳️operation-progress/🦀️.rs"}
import glob
files |= set(glob.glob(PLUG + "/📜️declaration/🧩️app/🛬️lifted/*/🦀️.rs"))
files |= set(glob.glob(PLUG + "/🪝️mounts/🌱️wp1-foundation/🧪️test-hosts/**/🦀️.rs", recursive=True))
files |= {PLUG + "/🧪️tests/🔬️app-window-kits/🦀️.rs", PLUG + "/🧪️tests/🔬️app-panel-kit/🦀️.rs"}
files |= set(glob.glob(PLUG + "/🪟️window/🎚️config/🧪️tests/**/🦀️.rs", recursive=True))
bad = 0
for f in sorted(files):
    if not os.path.exists(f):
        print("MISSING", f.replace(PLUG + "/", ""))
        bad += 1
        continue
    r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true,unstable_features=true", f], capture_output=True, text=True)
    if r.returncode != 0 and "error" in r.stderr:
        print("PARSE-ERROR", f.replace(PLUG + "/", ""), r.stderr.strip().splitlines()[:2])
        bad += 1
print("files", len(files), "bad", bad)
