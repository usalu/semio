import re, collections, os
from lib import *
import rsparse as P
from rs import blank
f = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
tx = read(f); cl = blank(tx)
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
INC = re.compile(r'include!\(\s*(?:concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"\s*\)|"([^"]+)")\s*\)\s*;')
sts = P.statements(tx, cl, 0, len(tx))
mod = [s for s in sts if s.kind == "mod" and s.inline][0]
bo = tx.index("{", mod.start)
inner = P.statements(tx, cl, bo + 1, mod.end - 1)
main_ids = {}
incs = []
for s in inner:
    seg = cl[s.start:s.end]
    if s.kind == "other" and "include!" in seg:
        m = INC.search(tx[s.start:s.end])
        rel_ = m.group(1) or m.group(2)
        base = (PLUG + "/" + PKGDIR) if m.group(1) else os.path.dirname(f)
        incs.append((s, os.path.normpath(base + rel_)))
main_text_ids = collections.Counter()
for s in inner:
    if not (s.kind == "other" and "include!" in cl[s.start:s.end]):
        main_text_ids.update(set(IDENT.findall(cl[s.start:s.end])))
for s, target in incs:
    itx = read(target); icl = blank(itx)
    top = P.statements(itx, icl, 0, len(itx))
    defs = [x.name for x in top if x.name and x.kind in ("fn", "struct", "enum", "const", "static", "type", "trait", "mod")]
    used_by_main = [d for d in defs if main_text_ids.get(d)]
    tests = sum(1 for x in top if x.kind == "fn" and re.search(r"test", x.attrs or ""))
    print(os.path.relpath(target, PLUG)[:60], "defs", len(defs), "tests-at-top", tests, "defs used by main:", used_by_main[:12])
