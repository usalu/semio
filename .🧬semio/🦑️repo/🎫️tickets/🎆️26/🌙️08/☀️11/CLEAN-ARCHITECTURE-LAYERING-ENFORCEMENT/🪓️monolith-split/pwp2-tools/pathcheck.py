#!/usr/bin/env python3
# 🧭️ Path resolution of the carved WP2 trees: every `#[path]`, `include!`, `include_str!`, `include_bytes!` literal of every file below the seven crate roots, the host files and hub 2 must name an existing file.
import re, os, sys
from common import *
import model as M
import modtree
from rs import blank
import widen as W
import hubgen

PAT = re.compile(r'(?:#\[\s*path\s*=\s*"([^"]+)"\s*\]|\binclude(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)|\binclude!\(concat!\([^)]*\)\))')

def check(files, base):
    missing, total = [], 0
    for f in files:
        t = read(f)
        clean = blank(t)
        for m in PAT.finditer(t):
            if clean[m.start():m.start() + 5].strip() == '':
                continue
            lit = m.group(1) or m.group(2)
            if not lit:
                continue
            total += 1
            p = os.path.normpath(os.path.join(os.path.dirname(f), lit))
            if not os.path.exists(p):
                missing.append((os.path.relpath(f, base), lit))
    return total, missing

def main():
    W.make_view()
    files = set()
    for k in M.WP2:
        files |= set(modtree.tree(W.VIEW + '/' + M.ROOT[k]))
    hub = W.VIEW + '/' + hubgen.HUB_REL
    import engine as E
    E.set_input(W.VIEW)
    write(hub, hubgen.build_hub().replace('//#endregion 📜️DeclarationFacet', hubgen.extra_units() + '\n//#endregion 📜️DeclarationFacet', 1))
    files.add(hub)
    for dp, dn, fn in os.walk(W.VIEW + '/' + M.HOSTS_DIR):
        for f in fn:
            files.add(os.path.join(dp, f))
    total, missing = check(sorted(files), W.VIEW)
    print('path literals checked:', total, '; unresolved:', len(missing))
    for f, l in missing:
        print('  ', f, '->', l)

if __name__ == '__main__':
    main()
