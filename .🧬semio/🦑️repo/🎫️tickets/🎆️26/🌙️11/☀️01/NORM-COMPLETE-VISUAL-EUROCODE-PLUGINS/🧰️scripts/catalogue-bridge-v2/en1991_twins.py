import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import twins as W, bridge_ext as X
A = Art("en1991")
X.json_typed(A, A.snap + "/🔣️.json", "$defs", formats=["🔣️jsonschema", "🟦️typescript"])
X.ts_typed(A.snap + "/🟦️.ts", "En1991Snapshot")
X.json_typed(A, A.schema + "/🔣️.json", "definitions")
X.ts_typed(A.schema + "/🟦️.ts", "En1991Artifact")
for d in (A.snap, A.schema):
    W.graphql_json(d + "/🔗️.graphql", "En1991Snapshot" if d == A.snap else "En1991Artifact")
    p = d + "/🔗️.graphql"
    s = rd(p)
    wr(p, s.replace("  cases: [JSON!]!", "  cases: [JSON!]! @state(class: ARTIFACT)", 1))
    pp = d + "/🛰️.proto"
    s = rd(pp)
    a = s.index("message En1991%s {" % ("Snapshot" if d == A.snap else "Artifact"))
    j = s.index("\n}", a)
    n = max(int(x) for x in re.findall(r"= (\d+);", s[a:j])) + 1
    wr(pp, s[:j] + "\n  string cases = %d;" % n + s[j:])
W.diff_json(A)
W.diff_ts(A, "En1991Diff")
W.diff_graphql(A, "En1991Diff", typed=False)
pp = A.diff + "/🛰️.proto"
s = rd(pp)
a = s.index("message En1991Diff {")
j = s.index("\n}", a)
n = max(int(x) for x in re.findall(r"= (\d+);", s[a:j])) + 1
wr(pp, s[:j] + "\n  string cases = %d;" % n + s[j:])
print(len(LOG))
