#!/usr/bin/env python3
"""🧲️ Lists impl blocks that live in a different file than the local type they implement for."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm
from carve import Model, impl_self_ident, inline_descendants, rel

def scan(root_path, ws=None):
    ws = ws or wsm.WS()
    hcmod.FS = ws
    model = Model(ws, {'crate': root_path})
    out = []
    for fm in hcmod.walk(model.roots['crate']):
        if fm.inline or getattr(fm, 'missing', False):
            continue
        for S in [fm] + list(inline_descendants(fm)):
            for it in S.all_items:
                if it.kind != 'impl':
                    continue
                ident = impl_self_ident(it)
                if not ident:
                    continue
                r = hcmod.lookup(S, ident)
                if r is None or r.kind != 'item' or r.mod is None:
                    continue
                if r.mod.file_root().file != fm.file:
                    out.append((rel(fm.file), ident, rel(r.mod.file_root().file), it.toks_all[0].sl + 1))
    return out

if __name__ == '__main__':
    for row in scan(sys.argv[1]):
        print(' | '.join(str(x) for x in row))
