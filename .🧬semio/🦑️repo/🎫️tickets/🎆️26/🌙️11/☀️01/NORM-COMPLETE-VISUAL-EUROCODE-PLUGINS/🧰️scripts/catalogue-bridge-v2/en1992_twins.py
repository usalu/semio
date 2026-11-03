import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import twins as W, bridge_ext as X
A = Art("en1992")
def proto_oneline(path, struct):
    s = rd(path)
    i = s.index("message %s {" % struct)
    e = s.index(" }", i)
    n = max(int(x) for x in re.findall(r"= (\d+);", s[i:e])) + 1
    s = s[:e] + " repeated CaseRow cases = %d;" % n + s[e:]
    wr(path, s[:i] + W.PROTO_ROWS + s[i:] if False else s[:i] + W.PROTO_ROWS.replace("\n\n", "\n") + s[i:])
def gql(path, struct):
    W.graphql_typed(path, struct)
    s = rd(path)
    wr(path, s.replace("  cases: [CaseRow!]!", "  cases: [CaseRow!]! @state(class: ARTIFACT)", 1))
for d, struct, art in ((A.snap, "En1992Snapshot", "Snapshot"), (A.schema, "En1992Artifact", "Artifact")):
    X.json_typed(A, d + "/🔣️.json", "$defs", formats=["🔣️jsonschema", "🟦️typescript"] if d == A.snap else None)
    X.ts_typed(d + "/🟦️.ts", struct)
    gql(d + "/🔗️.graphql", struct)
    proto_oneline(d + "/🛰️.proto", struct)
W.diff_json(A)
W.diff_ts(A, "En1992Diff")
W.diff_graphql(A, "En1992Diff")
W.diff_proto(A, "En1992Diff")
print(len(LOG))
