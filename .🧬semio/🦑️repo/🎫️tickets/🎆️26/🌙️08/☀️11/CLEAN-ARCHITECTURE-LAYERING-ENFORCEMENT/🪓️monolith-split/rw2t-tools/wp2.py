"""🧪 rw2t: the WP2 crates, their roots, and the test files (cfg(test) mounts) each reaches."""
import os, sys, re
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rtb-tools")
from lib import *
import modtree
WP2 = ["engine-registry", "engine-text", "engine-board", "engine-graph", "engine-attach", "scene-state", "canvas2d-host", "scene-canvas2d", "scene-ink", "scene-feeds", "scene-tables", "scene-text-vfs", "icon-host", "scene-render"]
BINS = ["engine-attach-tests", "scene-ink-tests", "ui-engine-tests", "runtime-mailbox-tests", "frame-tests-1"]
def root(name):
    tab = crate_table()
    return os.path.join(T, tab[name], "🦀️.rs")
def testfiles(name):
    out = []
    for f in modtree.tree(root(name)):
        if "🧪️tests" in f or "🧰️fixtures" in f or "🪤" in f:
            out.append(f)
    return out
if __name__ == "__main__":
    names = sys.argv[1:] or WP2 + BINS
    for n in names:
        fs = modtree.tree(root(n))
        tf = testfiles(n)
        print("==", n, "files", len(fs), "tests", len(tf))
        for f in tf:
            print("   ", os.path.relpath(f, E), sum(1 for _ in open(f, encoding="utf8")))
