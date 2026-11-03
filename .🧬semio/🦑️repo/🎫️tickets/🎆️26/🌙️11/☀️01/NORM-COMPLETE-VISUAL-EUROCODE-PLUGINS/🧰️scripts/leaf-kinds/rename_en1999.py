import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import model as M, en1999 as E, rename as R, wire as W

art = E.art
shortlim = 255 - len((art.fix[len(SEMIO) + 1:] + "/").encode()) - len("/✅apply/🔣️.before.json".encode()) - 8
M.LIMIT[0] = shortlim
leaves, defs = M.read_entities(art.any + "/🧬️schema/📸️snapshot/🔣️.json", E.ENTS, E.COVERED)
pairs = []
for l in leaves:
    old = "change-%s-%s" % (l.ent.name, kebab(l.field))
    if not l.covered and l.kind != old:
        pairs.append((old, l.kind))
print("limit", shortlim, "renames", len(pairs))
roots = [art.root]
extra = [TAXONOMY]
import sys as _s
if "--parse-only" in _s.argv:
    R.forms = lambda o, n: [("parse" + pascal(o), "parse" + pascal(n))]
    print("files changed", R.apply(pairs, roots, extra))
else:
    print("files changed", R.apply(pairs, roots, extra), "dirs", R.rename_dirs(roots, pairs))
bad = R.guard(art.root)
print("offenders", len(bad))
for b in bad[:5]:
    print(len(b[len(SEMIO) + 1:].encode()), b[-120:])
