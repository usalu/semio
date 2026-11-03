import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import re
from lib import *
import crates as C
A = T
leaves = [(LEAF_DIRS[k], MODULE[k]) for k in KINDS] + [("🧮️change-uls-equation", "change_uls_equation")]
old = [{"name": A.crate + "mut-cases", "dir": "📇️cases"}, {"name": A.crate + "mut-case-inputs", "dir": "🔢️case-inputs"}]
lock = rd(A.root + "/Cargo.lock")
deps = set()
for o in old:
    m = re.search(r'name = "%s"\nversion = "[^"]+"\ndependencies = \[\n(.*?)\n\]' % re.escape(o["name"]), lock, re.S)
    deps |= {d.strip().strip('",') for d in m.group(1).split("\n")}
cname = A.crate + "mut-" + C.FAMILY
d = C.create(A, leaves, [A.crate + "kernel"], "🗂️ EN 1995 design-case mutation leaves: insert, remove, set number, set text, label, entry id and the ULS equation.", "EN 1995 design-case mutation leaves")
C.register(A, cname, d, deps, old=old)
for o in old:
    C.drop_dirs(A.mut + "/" + o["dir"])
ident = lambda n: n.replace("-", "_")
for dp, dn, fn in os.walk(A.root):
    dn[:] = [x for x in dn if x not in ("target", "node_modules")]
    for f in fn:
        if f.endswith(".rs"):
            p = os.path.join(dp, f)
            s = rd(p)
            n = s
            for o in old:
                n = n.replace(ident(o["name"]), ident(cname))
            if n != s:
                wr(p, n)
print(len(LOG), "files")
