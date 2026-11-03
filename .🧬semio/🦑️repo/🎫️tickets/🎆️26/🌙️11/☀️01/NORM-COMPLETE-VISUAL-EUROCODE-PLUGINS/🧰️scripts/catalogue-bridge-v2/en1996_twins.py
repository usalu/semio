import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import twins as W, bridge_ext as X, jsonspan as J
A = Art("en1996")
X.json_typed(A, A.snap + "/🔣️.json", "$defs", formats=["🔣️jsonschema", "🟦️typescript"])
X.ts_typed(A.snap + "/🟦️.ts", "En1996Snapshot")
def gql_state(path, struct):
    s = rd(path)
    i = s.index("type %s" % struct)
    s = s[:i] + W.GQL_ROWS + s[i:]
    j = s.index("\n}", s.index("type %s" % struct))
    wr(path, s[:j] + "\n  cases: [CaseRow!]! @state(lane: ARTIFACT)" + s[j:])
gql_state(A.snap + "/🔗️.graphql", "En1996Snapshot")
W.proto_typed(A.snap + "/🛰️.proto", "En1996Snapshot")
# root: inline object schema (same style as placements)
p = A.schema + "/🔣️.json"
t = rd(p)
item = json.loads(json.dumps(W.tmpl_defs(A)["CaseRow"]).replace("#/$defs/CaseNumber", "#/definitions/CaseNumber"))
inline = {"type": "object", "additionalProperties": False, "required": ["id", "labelEn", "labelDe", "entryId", "numbers", "texts"], "properties": {"id": {"type": "string"}, "labelEn": {"type": "string"}, "labelDe": {"type": "string"}, "entryId": {"type": "string"}, "numbers": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["id", "value"], "properties": {"id": {"type": "string"}, "value": {"type": "number"}}}}, "texts": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["id", "value"], "properties": {"id": {"type": "string"}, "value": {"type": "string"}}}}}}
t = J.add_member(t, ["properties"], "cases", {"type": "array", "items": inline, "x-semio-state": "artifact"})
t = J.add_item(t, ["required"], "cases")
json.loads(t)
wr(p, t)
p = A.schema + "/🟦️.ts"
s = rd(p)
m = re.search(r"export interface En1996Snapshot \{\n(.*?)\n\}\n", s, re.S)
s = s[: m.end() - 2] + "\n  cases: { id: string; labelEn: string; labelDe: string; entryId: string; numbers: { id: string; value: number; }[]; texts: { id: string; value: string; }[]; }[];\n}\n" + s[m.end():]
m = re.search(r"^export const parseEn1996Snapshot: .*?\}\);$", s, re.M)
line = m.group(0)
s = s.replace(line, line[:-3] + ", cases: normWireRequired(normWireArray(normWireObject<{ id: string; labelEn: string; labelDe: string; entryId: string; numbers: { id: string; value: number; }[]; texts: { id: string; value: string; }[]; }>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireObject<{ id: string; value: number; }>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) }))), texts: normWireRequired(normWireArray(normWireObject<{ id: string; value: string; }>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) }))) }))) });", 1)
wr(p, s)
gql_state(A.schema + "/🔗️.graphql", "En1996Artifact")
W.proto_typed(A.schema + "/🛰️.proto", "En1996Artifact")
W.diff_json(A)
W.diff_ts(A, "En1996Diff")
W.diff_graphql(A, "En1996Diff", typed=False)
W.diff_proto(A, "En1996Diff", typed=False)
print(len(LOG))
