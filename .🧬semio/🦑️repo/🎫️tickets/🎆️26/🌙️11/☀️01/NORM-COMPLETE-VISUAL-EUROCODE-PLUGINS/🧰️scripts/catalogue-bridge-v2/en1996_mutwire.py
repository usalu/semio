import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1996")
p = A.mut + "/🔗️.graphql"
s = rd(p)
m = re.search(r"^union En1996Mutation = ([^\n]*)", s, re.M)
s = s.replace(m.group(0), m.group(0) + "".join(" | " + RECORD[k] for k in KINDS), 1)
types = """
type InsertCase {
  index: Int
  case: String
}

type RemoveCase {
  index: Int
}

type SetCaseNumber {
  caseId: String
  key: String
  value: Float
}

type SetCaseText {
  caseId: String
  key: String
  value: String
}

type ChangeCaseLabelEn {
  caseId: String
  newValue: String
}

type ChangeCaseLabelDe {
  caseId: String
  newValue: String
}

type ChangeCaseEntryId {
  caseId: String
  newValue: String
}
"""
wr(p, s.rstrip("\n") + "\n" + types)
p = A.mut + "/🛰️.proto"
s = rd(p)
msgs = """message InsertCase {
  uint32 index = 1;
  string case = 2;
}
message RemoveCase {
  uint32 index = 1;
}
message SetCaseNumber {
  string case_id = 1;
  string key = 2;
  optional double value = 3;
}
message SetCaseText {
  string case_id = 1;
  string key = 2;
  optional string value = 3;
}
message ChangeCaseLabelEn {
  string case_id = 1;
  string new_value = 2;
}
message ChangeCaseLabelDe {
  string case_id = 1;
  string new_value = 2;
}
message ChangeCaseEntryId {
  string case_id = 1;
  string new_value = 2;
}
"""
a = s.index("message En1996Mutation {")
j = s.index("\n  }", a)
n = max(int(x) for x in re.findall(r"= (\d+);", s[a:j]))
oneof = "".join("\n    %s %s = %d;" % (RECORD[k], k.replace("-", "_"), n + 1 + i) for i, k in enumerate(KINDS))
s = s[:j] + oneof + s[j:]
wr(p, s.rstrip("\n") + "\n" + msgs)
print(n)
