import json, os, re, sys
from lib import *
import modtree
lists = json.load(open(SCRATCH + "/source-lists.json"))
crates = crate_table()
order = ["shell-vocab","shell-async","shell-chrome-state","shell-surfaces","shell-plugins","shell-history","shell-prefs","shell-directory","shell-sync","shell-docs","shell-doc-open","shell-windows","shell-dock","shell-input","shell-commands","shell-state","shell-session","shell-window-actions","shell-actions","shell-pointer","shell-view-paint","shell-view-chrome","shell-view-overlays","shell-view-panels"]
roots = [SH + "/🦀️.rs"] + [T + "/" + crates[c] + "/🦀️.rs" for c in order]
tree = []
for r in roots:
    tree += [os.path.relpath(f, E) for f in modtree.tree(r)]
tree = list(dict.fromkeys(tree))
const = lists["shell"]
print("tree", len(tree), "const", len(const))
print("in const not in tree:", [f for f in const if f not in tree][:30], len([f for f in const if f not in tree]))
print("in tree not in const:", len([f for f in tree if f not in const]))
