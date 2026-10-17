import sys, xml.etree.ElementTree as ET
from collections import Counter
t = ET.parse(sys.argv[1])
rows = []
for tc in t.iter("testcase"):
    st = "failed" if (tc.find("failure") is not None or tc.find("error") is not None) else ("skipped" if tc.find("skipped") is not None else "passed")
    rows.append((st, (tc.get("classname") or "").split("/semio/")[-1], tc.get("name") or ""))
open(sys.argv[2], "w", encoding="utf8").write("\n".join("\t".join(r) for r in sorted(rows)) + "\n")
print(len(rows), dict(Counter(r[0] for r in rows)))
