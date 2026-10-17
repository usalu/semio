import re, collections, os, sys
from lib import *
import rsparse as P
from rs import blank
f = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
t = read(f); c = blank(t)
INC = re.compile(r'include!\(\s*(?:concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"\s*\)|"([^"]+)")\s*\)\s*;')
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
def items_of(path):
    tx = read(path); cl = blank(tx)
    sts = P.statements(tx, cl, 0, len(tx))
    mods = [s for s in sts if s.kind == "mod" and s.inline]
    return tx, cl, sts, mods
tx, cl, sts, mods = items_of(f)
mod = mods[0]
bo = tx.index("{", mod.start)
inner = P.statements(tx, cl, bo + 1, mod.end - 1)
main_defs = {}
for s in inner:
    if s.name and s.kind in ("fn", "struct", "enum", "const", "static", "type", "trait", "mod"):
        main_defs.setdefault(s.name, []).append(s)
print("main defs", len(main_defs))
for s in inner:
    if s.kind == "other" and "include!" in cl[s.start:s.end]:
        m = INC.search(tx[s.start:s.end])
        if not m: print("no-match", repr(tx[s.start:s.start+120])); continue
        rel_ = m.group(1) or m.group(2)
        base = (PLUG + "/" + PKGDIR) if m.group(1) else os.path.dirname(f)
        target = os.path.normpath(base + rel_)
        itx = read(target); icl = blank(itx)
        ids = set(IDENT.findall(icl))
        needs = sorted(n for n in main_defs if n in ids)
        # inner defs
        issince = P.statements(itx, icl, 0, len(itx))
        idefs = [x.name for x in isince] if False else []
        nt = len(re.findall(r"#\[(?:[\w:]*::)?(?:async_)?test\]", itx))
        print(os.path.relpath(target, PLUG)[:70], len(itx), "tests", nt, "uses main defs:", len(needs))
