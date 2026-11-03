import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import twins as W, bridge_ext as X
A = Art("en1990")
X.json_typed(A, A.snap + "/🔣️.json", "$defs", formats=["🔣️jsonschema", "🟦️typescript"])
X.ts_typed(A.snap + "/🟦️.ts", "En1990Snapshot")
W.graphql_typed(A.snap + "/🔗️.graphql", "En1990Snapshot")
W.proto_typed(A.snap + "/🛰️.proto", "En1990Snapshot")
X.json_typed(A, A.schema + "/🔣️.json", "definitions")
X.ts_typed(A.schema + "/🟦️.ts", "En1990Artifact")
W.graphql_typed(A.schema + "/🔗️.graphql", "En1990Artifact")
W.proto_typed(A.schema + "/🛰️.proto", "En1990Artifact")
W.diff_json(A)
W.diff_ts(A, "En1990Diff")
X.diff_graphql_bare(A.diff + "/🔗️.graphql", "En1990Diff")
X.diff_proto_bare(A.diff + "/🛰️.proto", "En1990Diff")
print(len(LOG))
