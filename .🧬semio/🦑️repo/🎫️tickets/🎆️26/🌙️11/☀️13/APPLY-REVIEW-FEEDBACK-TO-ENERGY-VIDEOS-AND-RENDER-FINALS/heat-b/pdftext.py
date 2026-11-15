import sys
from pypdf import PdfReader
r = PdfReader(sys.argv[1])
if r.is_encrypted:
    r.decrypt("")
out = []
for p in r.pages:
    out.append(p.extract_text() or "")
open(sys.argv[2], "w", encoding="utf-8").write("\n\f\n".join(out))
