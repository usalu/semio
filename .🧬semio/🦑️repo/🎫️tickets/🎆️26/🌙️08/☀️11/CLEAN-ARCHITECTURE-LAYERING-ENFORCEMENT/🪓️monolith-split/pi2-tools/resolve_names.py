#!/usr/bin/env python3
"""🔎️ Resolves names against the module scope of a real crate root and prints the crate path each resolves to: resolve_names.py <crate key> <name>..."""
import sys
from lib import *
import world as Wd
import translate as TR
W = Wd.World()
key = sys.argv[1]
tree = W.real_tree(key)
tr = TR.Tr(W, None, "FACADE", flipped=set(), allowed=None) if False else None
import layout as L
LY = L.Layout(W)
tr = TR.Tr(W, LY, "FACADE", flipped=set(__import__("uo").real_crates()))
for n in sys.argv[2:]:
    b = W.lookup(tree, n)
    if not b:
        print(n, "NOT FOUND")
        continue
    r = W.follow_any(b)
    p = tr.emit(r) if r is not None else None
    print(n, "::".join(p) if p else repr(r))
