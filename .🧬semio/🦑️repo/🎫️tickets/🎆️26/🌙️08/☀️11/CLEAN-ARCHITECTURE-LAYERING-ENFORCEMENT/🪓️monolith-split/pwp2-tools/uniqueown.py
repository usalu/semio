#!/usr/bin/env python3
# 1️⃣️ One owner per module file: after the WP2 change (OUT over the build input, hub 2, root edit, reactor mount, interim-mount flips) no `.rs` file is reachable from the module trees of two crates (the facade root counts as a crate).
import os, re, sys, collections
from common import *
import model as M
import engine as E
import hubgen, flips, modtree
import widen as W

def main():
    W.make_view()
    plug = W.VIEW
    E.set_input(plug)
    hub = hubgen.build_hub().replace('//#endregion 📜️DeclarationFacet', hubgen.extra_units() + '\n//#endregion 📜️DeclarationFacet', 1)
    write(plug + '/' + hubgen.HUB_REL, hub)
    root = hubgen.root_edit()
    write(plug + '/🦀️.rs', root)
    rr = read(plug + '/⚛️reactor/🦀️.rs')
    write(plug + '/⚛️reactor/🦀️.rs', rr.replace('#[path = "🪟️surfaces/🦀️.rs"]\npub(crate) mod surface_context;\n', 'pub(crate) use semio_framework_plugin_plugin_program::surface_context;\n'))
    # flips on a copy of every file mentioning a lift
    n = 0
    for dp, dn, fn in os.walk(plug):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target')]
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f)
                t = read(p)
                if '🛬️lifted' in t:
                    rel = os.path.relpath(p, plug)
                    new, hits = flips.flip(t, rel, False)
                    if hits:
                        write(p, new); n += len(hits)
    print('flips applied in the view:', n)
    trees = {}
    carved = {k for k in M.CRATES_JSON if os.path.exists(plug + '/' + M.ROOT[k])}
    def flipped(k):
        pkg = PLUG + '/' + M.CRATES_JSON[k]['pkg'].split('/🔌️plugin/')[1] + '/Cargo.toml'
        return os.path.exists(pkg) and '../../🦀️.rs' in read(pkg).split('[lib]')[1].split('\n')[1] if '[lib]' in read(pkg) else False
    roots = {k: plug + '/' + M.ROOT[k] for k in M.CRATES_JSON if (k in M.WP2 or flipped(k)) and os.path.exists(plug + '/' + M.ROOT[k])}
    roots['<facade>'] = plug + '/🦀️.rs'
    owner = collections.defaultdict(set)
    for k, r in roots.items():
        for f in modtree.tree(r):
            owner[f].add(k)
    dup = {f: o for f, o in owner.items() if len(o) > 1}
    print('crates with a tree:', len(roots), '; files:', len(owner), '; files reachable from two roots:', len(dup))
    for f, o in sorted(dup.items()):
        print('  ', os.path.relpath(f, plug), sorted(o))

if __name__ == '__main__':
    main()
