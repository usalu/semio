#!/usr/bin/env python3
# 🧭️ Path-literal resolver: #[path], include!, include_str!, include_bytes! of the WP1 trees, hub, root and lift files (CARGO_MANIFEST_DIR forms are listed apart).
import glob, os, re, sys
from lib import *
import wp1files
from rs import blank
from crates_wp1 import ROOT

files = set()
for crate, rootdir in ROOT.items():
    for f, _ in wp1files.tree(PLUG + "/" + rootdir + "/🦀️.rs"):
        files.add(f)
files |= {PLUG + "/🦀️.rs", PLUG + "/🪝️mounts/🌱️wp1-foundation/🦀️.rs", PLUG + "/📣️macros/🌱️wp1-window/🦀️.rs", PLUG + "/⚛️reactor/🦀️.rs", PLUG + "/🗃️document/🦀️.rs", PLUG + "/⏳️operation-progress/🦀️.rs"}
files |= set(glob.glob(PLUG + "/📜️declaration/🧩️app/🛬️lifted/*/🦀️.rs"))
files |= set(glob.glob(PLUG + "/🪝️mounts/🌱️wp1-foundation/🧪️test-hosts/**/🦀️.rs", recursive=True))
files |= {PLUG + "/🧪️tests/🔬️app-window-kits/🦀️.rs", PLUG + "/🧪️tests/🔬️app-panel-kit/🦀️.rs"}
files |= set(glob.glob(PLUG + "/🪟️window/🎚️config/🧪️tests/**/🦀️.rs", recursive=True))
RX = re.compile(r'(#\[\s*path\s*=\s*"([^"]+)"\s*\]|include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\))')
n = bad = manifest = 0
for f in sorted(files):
    if not os.path.exists(f):
        continue
    text = read(f)
    clean = blank(text)
    for m in RX.finditer(text):
        if clean[m.start():m.start() + 2].strip() == "":
            continue
        lit = m.group(2) or m.group(3)
        target = os.path.normpath(os.path.join(os.path.dirname(f), lit))
        n += 1
        if not os.path.exists(target):
            print("UNRESOLVED", f.replace(PLUG + "/", ""), "->", lit)
            bad += 1
    manifest += len(re.findall(r"CARGO_MANIFEST_DIR", text))
print("literals", n, "unresolved", bad, "files", len(files), "CARGO_MANIFEST_DIR uses", manifest)
