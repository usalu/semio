"""🧷️ Writes single-feature annotation PDFs (pymupdf) into the directory in argv[1]: one per annotation kind, to find which kinds the browser import admits."""
import sys, pymupdf
target = sys.argv[1]
def make(name, build):
    doc = pymupdf.open()
    page = doc.new_page(width=300, height=400)
    page.insert_text((40, 60), "Annotation host text", fontsize=12)
    build(page)
    doc.save(f"{target}/{name}.pdf")
make("plain", lambda p: None)
make("note", lambda p: p.add_text_annot((200, 40), "Note body"))
make("highlight", lambda p: p.add_highlight_annot(pymupdf.Rect(40, 48, 150, 64)).update())
make("underline", lambda p: p.add_underline_annot(pymupdf.Rect(40, 48, 150, 64)).update())
make("strike", lambda p: p.add_strikeout_annot(pymupdf.Rect(40, 48, 150, 64)).update())
make("squiggly", lambda p: p.add_squiggly_annot(pymupdf.Rect(40, 48, 150, 64)).update())
make("square", lambda p: p.add_rect_annot(pymupdf.Rect(100, 100, 200, 180)).update())
make("circle", lambda p: p.add_circle_annot(pymupdf.Rect(100, 100, 200, 180)).update())
make("line", lambda p: p.add_line_annot((50, 200), (200, 250)).update())
make("ink", lambda p: p.add_ink_annot([[(50, 300), (100, 350), (150, 300)]]).update())
make("freetext", lambda p: p.add_freetext_annot(pymupdf.Rect(40, 250, 200, 290), "Free text", fontsize=11).update())
make("stamp", lambda p: p.add_stamp_annot(pymupdf.Rect(40, 250, 140, 290), stamp=0))
make("link", lambda p: p.insert_link({"kind": pymupdf.LINK_URI, "from": pymupdf.Rect(40, 48, 150, 64), "uri": "https://example.test/"}))
def widgets(p):
    w = pymupdf.Widget(); w.field_type = pymupdf.PDF_WIDGET_TYPE_TEXT; w.field_name = "name"; w.rect = pymupdf.Rect(40, 100, 200, 120); w.field_value = "Ada"; p.add_widget(w)
make("widget", widgets)
print("ok", target)
