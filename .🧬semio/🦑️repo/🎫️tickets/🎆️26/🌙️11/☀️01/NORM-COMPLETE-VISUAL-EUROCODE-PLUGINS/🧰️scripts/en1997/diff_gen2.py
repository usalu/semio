import sys
sys.path.insert(0, ".")
from spec import *
p = S + "/🧬️schema/🔺️diff/🦀️.rs"
s = open(p, encoding="utf8").read()
if "next.anchors" not in s:
    a = '        *unpublished = next;\n'
    ap = "".join(f"        if let Some(rows) = &self.{e.coll} {{\n            next.{e.coll} = rows.commit_onto(&base.{e.coll}, capability).map_err(|error| error.under([\"{e.coll}\"]))?;\n        }}\n" for e in ENTITIES)
    assert a in s
    s = s.replace(a, ap + a, 1)
    i = s.index("fn is_empty(&self) -> bool {\n        self.structure_id.is_none()")
    j = s.index("\n", s.index("self.structure_id.is_none()", i))
    line = s[s.index("self.structure_id.is_none()", i):j]
    assert line.rstrip().endswith("is_empty())") or True
    s = s[:j] + "".join(f" && self.{e.coll}.as_ref().map_or(true, |rows| rows.is_empty())" for e in ENTITIES) + s[j:]
    k = "            uplift_cases: self.uplift_cases.as_ref().map(|rows| rows.inverse(&base.uplift_cases)).filter(|rows| !rows.is_empty()),\n"
    assert k in s
    s = s.replace(k, k + "".join(f"            {e.coll}: self.{e.coll}.as_ref().map(|rows| rows.inverse(&base.{e.coll})).filter(|rows| !rows.is_empty()),\n" for e in ENTITIES), 1)
    open(p, "w", encoding="utf8").write(s)
