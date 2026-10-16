#!/usr/bin/env python3
"""🔬️ Scan: definitions gated by exactly `#[cfg(test)]` in a flipped crate whose name occurs in a file outside the crate. The crate is compiled without cfg(test) when a facade test uses it, so such a name vanishes (the union oracle compiles everything as one crate and cannot see this).
usage: python3 cfgtest.py [crate ...]"""
import re, sys, os
from lib import *
from rs import blank
import uo, tree as T
import widen as WD

DEF = re.compile(r"#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+|unsafe\s+|const\s+)*(?:fn|struct|enum|static|const|type|trait|mod|union)\s+([A-Za-z_]\w*)")
TL = re.compile(r"#\[cfg\(test\)\]\s*(?:thread_local|component_persistent_local)!\s*\{(.*?)\n\}", re.S)
STAT = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?static\s+([A-Za-z_]\w*)\s*:")


def crate_files(root):
    return {f for f, p, n in T.tree(root)}


def main(only):
    reals = uo.real_crates()
    allfiles = {k: crate_files(r) for k, r in reals.items()}
    mine = set(uo.flipped_by_me())
    live = WD.live_rs_files()
    idents = {}
    for f in live:
        idents[f] = WD.file_idents(f)
    for K in sorted(mine & set(reals)):
        if only and K not in only:
            continue
        defs = {}
        for f in allfiles[K]:
            t = read(f)
            c = blank(t)
            for m in DEF.finditer(c):
                defs.setdefault(m.group(1), f)
            for m in TL.finditer(c):
                for s in STAT.finditer(m.group(1)):
                    defs.setdefault(s.group(1), f)
        for name, f in sorted(defs.items()):
            users = [g for g, ids in idents.items() if name in ids and g not in allfiles[K]]
            users = [g for g in users if not any(g in allfiles[o] for o in allfiles if o == K)]
            if users:
                print(K, name, rel(f), "<-", len(users), rel(users[0]))


if __name__ == "__main__":
    main(set(sys.argv[1:]))
