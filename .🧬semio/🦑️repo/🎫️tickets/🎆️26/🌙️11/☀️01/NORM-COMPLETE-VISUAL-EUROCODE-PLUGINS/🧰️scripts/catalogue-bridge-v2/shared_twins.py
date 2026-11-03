import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import json, re
from lib import *
SH = NORM + "/⚖️compliance/🧰️design-case/🧬️schema"
DELTA = ["CaseNumberPatch", "CaseNumberRemoval", "CaseNumberInsertion", "CaseNumberRelocation", "CaseNumberModification", "CaseNumberDelta", "CaseTextPatch", "CaseTextRemoval", "CaseTextInsertion", "CaseTextRelocation", "CaseTextModification", "CaseTextDelta", "CaseRowPatch", "CaseRowRemoval", "CaseRowInsertion", "CaseRowRelocation", "CaseRowModification", "CaseRowDelta"]
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1995/snapshot.json#/$defs/"
diff = json.loads(rd(T.diff + "/🔣️.json"))
js = json.loads(rd(SH + "/🔣️.json"))
if "CaseRow" not in js["definitions"]:
    js["definitions"] = {("CaseRow" if k == "DesignCase" else k): v for k, v in js["definitions"].items()}
for name in DELTA:
    if name in js["definitions"]:
        continue
    txt = json.dumps(diff["$defs"][name], ensure_ascii=False)
    txt = txt.replace(SNAP, "#/definitions/").replace("#/$defs/", "#/definitions/")
    d = json.loads(txt)
    d.pop("x-semio-formats", None)
    js["definitions"][name] = d
wr(SH + "/🔣️.json", json.dumps(js, indent=2, ensure_ascii=False) + "\n")
print([k for k in js["definitions"]][:8], rd(SH + "/🔣️.json").count("DesignCase"))

ts_src = rd(T.diff + "/🟦️.ts")
i0 = ts_src.index("export interface CaseNumberPatch")
i1 = ts_src.index("export const parseEn1995Diff")
ifaces = ts_src[i0:i1].rstrip() + "\n"
ps = [l for l in ts_src.splitlines() if re.match(r"export const parseCase(Number|Text|Row)(Patch|Removal|Insertion|Relocation|Modification|Delta) ", l)]
sts = rd(SH + "/🟦️.ts")
if "CaseRowDelta" not in sts:
    sts = sts.replace("DesignCase", "CaseRow")
    sts = re.sub(r"import \{([^}]*)\} from", lambda m: "import {" + ", ".join(sorted(set(x.strip() for x in m.group(1).split(",")) | {"normWireDefault", "normWireInteger", "normWireNullable", "normWireRange", "normWireRef"}, key=str.lower)) + " } from", sts, 1)
    sts = sts.replace("export const parseInputKind", ifaces + "\nexport const parseInputKind", 1)
    sts = sts.rstrip("\n") + "\n" + "\n".join(ps) + "\n"
    wr(SH + "/🟦️.ts", sts)
gq = rd(SH + "/🔗️.graphql")
if "CaseRowDelta" not in gq:
    gq = gq.replace("DesignCase", "CaseRow")
    add = """
type CaseNumberPatch {
  value: Float
}

type CaseTextPatch {
  value: String
}

type CaseRowPatch {
  labelEn: String
  labelDe: String
  entryId: String
  numbers: CaseNumberDelta
  texts: CaseTextDelta
}
"""
    for row, patch in (("CaseNumber", "CaseNumber"), ("CaseText", "CaseText"), ("CaseRow", "CaseRow")):
        add += """
type %(r)sRemoval {
  id: String!
  index: Int!
}

type %(r)sInsertion {
  index: Int!
  row: %(r)s!
}

type %(r)sRelocation {
  id: String!
  from: Int!
  to: Int!
}

type %(r)sModification {
  id: String!
  patch: %(r)sPatch!
}

type %(r)sDelta {
  removed: [%(r)sRemoval!]!
  inserted: [%(r)sInsertion!]!
  moved: [%(r)sRelocation!]!
  modified: [%(r)sModification!]!
}
""" % {"r": row}
    wr(SH + "/🔗️.graphql", gq.rstrip("\n") + "\n" + add)
pr = rd(SH + "/🛰️.proto")
if "CaseRowDelta" not in pr:
    pr = pr.replace("DesignCase", "CaseRow")
    add = """
message CaseNumberPatch {
  optional double value = 1;
}

message CaseTextPatch {
  optional string value = 1;
}

message CaseRowPatch {
  optional string label_en = 1;
  optional string label_de = 2;
  optional string entry_id = 3;
  optional CaseNumberDelta numbers = 4;
  optional CaseTextDelta texts = 5;
}
"""
    for r in ("CaseNumber", "CaseText", "CaseRow"):
        add += """
message %(r)sRemoval {
  string id = 1;
  uint32 index = 2;
}

message %(r)sInsertion {
  uint32 index = 1;
  %(r)s row = 2;
}

message %(r)sRelocation {
  string id = 1;
  uint32 from = 2;
  uint32 to = 3;
}

message %(r)sModification {
  string id = 1;
  %(r)sPatch patch = 2;
}

message %(r)sDelta {
  repeated %(r)sRemoval removed = 1;
  repeated %(r)sInsertion inserted = 2;
  repeated %(r)sRelocation moved = 3;
  repeated %(r)sModification modified = 4;
}
""" % {"r": r}
    wr(SH + "/🛰️.proto", pr.rstrip("\n") + "\n" + add)
