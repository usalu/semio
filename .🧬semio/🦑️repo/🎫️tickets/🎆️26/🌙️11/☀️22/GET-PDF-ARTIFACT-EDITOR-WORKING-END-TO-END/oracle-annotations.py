"""🔮️ Oracle of the annotation/form/resource sweep (pymupdf + pypdf): `create <pdf>`."""
import sys, pymupdf
from pypdf import PdfReader
mode, path = sys.argv[1], sys.argv[2]
doc = pymupdf.open(path)
for p in doc: p.get_pixmap(dpi=40)
if mode == "create":
    page = doc[0]
    kinds = [a.type[1] for a in page.annots()]
    print("kinds", kinds)
    for need in ["Text", "Highlight", "Underline", "StrikeOut", "Squiggly", "Ink"]:
        assert kinds.count(need) >= 1, (need, kinds)
    links = page.get_links(); print("links", [(l.get("uri"), l.get("page")) for l in links])
    assert any(l.get("uri") == "https://example.test/rail" for l in links), links
    assert any(l.get("kind") == pymupdf.LINK_GOTO and l.get("page") == 1 for l in links), links
    notes = [a for a in page.annots() if a.type[1] == "Text" and a.info.get("content") == "Rail note"]
    assert notes, "rail note missing"
    print("note colors", notes[0].colors)
    assert notes[0].colors["stroke"] == (0.0, 0.0, 0.0), notes[0].colors
    hl = [a for a in page.annots() if a.type[1] == "Highlight" and a.info.get("content") == "Rail hl"][0]
    assert len(hl.vertices) == 4, hl.vertices
    ink = [a for a in page.annots() if a.type[1] == "Ink"][0]
    assert len(ink.vertices) == 2, ink.vertices
    r = PdfReader(path); print("pages", len(r.pages))
print("ORACLE OK", mode)
