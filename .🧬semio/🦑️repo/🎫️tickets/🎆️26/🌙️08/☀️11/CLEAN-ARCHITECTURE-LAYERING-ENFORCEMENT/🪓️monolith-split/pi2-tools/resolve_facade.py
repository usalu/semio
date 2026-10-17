#!/usr/bin/env python3
"""🔎️ Resolves facade-root paths (`a::b::C`) to crate paths: resolve_facade.py <path>..."""
import sys
from lib import *
import world as Wd, translate as TR, layout as L, uo
W = Wd.World(); LY = L.Layout(W)
tr = TR.Tr(W, LY, "FACADE", flipped=set(uo.real_crates()))
for a in sys.argv[1:]:
    r = W.resolve_path(W.mono, a.split("::"))
    p = tr.emit(r) if r is not None and r.kind not in ("unresolved", "cycle") else None
    print(a, "=>", "::".join(p) if p else repr(r))
