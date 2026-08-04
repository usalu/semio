"""🔗 Dumps every /Link annotation with its /GoTo destination for the pages that
carry the converted cross-references, so each one can be checked against the
label it is supposed to target."""

import sys
from pypdf import PdfReader

PDF = sys.argv[1]
PAGES = [int(p) for p in sys.argv[2:]] or [10, 11, 88]

reader = PdfReader(PDF)
names = reader.trailer["/Root"].get("/Names", {})
for page_no in PAGES:
    page = reader.pages[page_no - 1]
    text = page.extract_text() or ""
    print(f"=== page {page_no} ===")
    for annot in page.get("/Annots", []) or []:
        obj = annot.get_object()
        if obj.get("/Subtype") != "/Link":
            continue
        action = obj.get("/A")
        dest = obj.get("/Dest")
        target = None
        if action is not None:
            action = action.get_object()
            target = action.get("/D") or action.get("/URI")
        elif dest is not None:
            target = dest
        rect = [round(float(v)) for v in obj.get("/Rect", [])]
        print(f"  rect={rect} -> {target}")
