import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib2
from lib import *
import registry_ext as R
A = Art("en1992")
R.oracle(A, "remove-anchor-action")
R.taxonomy(A)
H = A.any + "/🧪️tests/🏛️mutate-en1992-1"
R.feature(A, H, "remove-anchor-action")
p = H + "/🐍️.py"
s = rd(p)
a = '    "remove-anchor-action",\n'
assert s.count(a) == 1
s = s.replace(a, a + "".join('    "%s",\n' % k for k in KINDS), 1)
a = '    "remove-anchor-action": ("🏋️remove-anchor-action", "✅apply"),\n'
assert s.count(a) == 1
s = s.replace(a, a + "".join('    "%s": ("%s", "✅apply"),\n' % (k, LEAF_DIRS[k]) for k in KINDS), 1)
wr(p, s)
p = H + "/🦀️.rs"
s = rd(p)
a = '    "remove-anchor-action",\n'
assert s.count(a) == 1
s = s.replace(a, a + "".join('    "%s",\n' % k for k in KINDS), 1)
arms = ""
for k in KINDS:
    arms += '        "%s" => (\n' % k + "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (LEAF_DIRS[k], x) for x in ["before", "mutation", "after", "outcome"]) + "        ),\n"
a = '        other => panic!("mutate-en1992-1: no committed fixture'
assert s.count(a) == 1
s = s.replace(a, arms + a, 1)
s = re.sub(r"current \d+-kind", "current %d-kind" % len(re.findall(r'^    "[a-z0-9-]+",$', s[s.index("const KINDS"):s.index("];", s.index("const KINDS"))], re.M)), s, 1)
wr(p, s)
print(len(LOG))
