import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import registry_ext as R
import leaves as L
A = Art("en1996")
R.oracle(A, "change-concentrated-distance-a1")
R.taxonomy(A)
H = A.any + "/🧪️tests/🪨️mutate-en1996-1"
p = H + "/🐍️.py"
s = rd(p)
a = '    "change-concentrated-distance-a1",\n'
assert s.count(a) == 1
s = s.replace(a, a + "".join('    "%s",\n' % k for k in KINDS), 1)
a = '    "change-concentrated-distance-a1": ("📏change-concentrated-distance-a1", "✅apply"),\n'
assert s.count(a) == 1
s = s.replace(a, a + "".join('    "%s": ("%s", "✅apply"),\n' % (k, LEAF_DIRS[k]) for k in KINDS), 1)
wr(p, s)
p = H + "/🥒️.feature"
s = rd(p)
pat = re.compile(r"(      \| change-concentrated-distance-a1 [^\n]*\n)")
rows = "".join("      | %-34s | %-36s | ✅apply  |\n" % (k, LEAF_DIRS[k]) for k in KINDS)
assert pat.search(s)
wr(p, pat.sub(lambda m: m.group(1) + rows, s))
L.edit_rules(A)
print(len(LOG))
