"""🧺️ Writes `ann-doc.pdf` (pymupdf), the annotations sweep document openable in the browser import: page 1 text, a note (no pop-up), highlight, square, free text and link; page 2 AcroForm text, check box, combo box, radio group; page 3 a layer and text.
usage: uv run --offline --with pymupdf==1.26.7 python -I annotations-fixture.py <out dir>"""
import pathlib, sys, pymupdf

def widget(page, kind, name, rect, **values):
    field = pymupdf.Widget()
    field.field_type, field.field_name, field.rect = kind, name, pymupdf.Rect(*rect)
    for key, value in values.items(): setattr(field, key, value)
    return page.add_widget(field)

document = pymupdf.open()
for width, height in [(612, 792), (612, 792), (595, 842)]: document.new_page(width=width, height=height)
one, two, three = document[0], document[1], document[2]
one.insert_text((72, 72), "Annotation Fixture", fontname="helv", fontsize=18)
one.insert_text((72, 110), "Second line of body text", fontname="helv", fontsize=12)
note = one.add_text_annot((500, 72), "Note body"); note.update(); document.update_object(note.xref, document.xref_object(note.xref).replace("/Popup null", "").replace(f"/Popup {note.xref + 1} 0 R", "")); document.xref_set_key(one.xref, "Annots", "[" + " ".join(f"{a.xref} 0 R" for a in one.annots() if a.type[0] != 16) + "]")
one.add_highlight_annot(pymupdf.Rect(72, 58, 300, 76)).update()
square = one.add_rect_annot(pymupdf.Rect(300, 400, 400, 480)); square.set_colors(stroke=(0, 1, 0)); square.update()
one.add_freetext_annot(pymupdf.Rect(72, 500, 300, 540), "Free text annotation", fontsize=11).update()
one.insert_link({"kind": pymupdf.LINK_GOTO, "from": pymupdf.Rect(72, 600, 200, 620), "page": 1, "to": pymupdf.Point(72, 72)})
two.insert_text((72, 72), "Form page", fontname="helv", fontsize=14)
widget(two, pymupdf.PDF_WIDGET_TYPE_TEXT, "name", (72, 100, 300, 120), field_value="Ada", text_fontsize=11)
widget(two, pymupdf.PDF_WIDGET_TYPE_CHECKBOX, "agree", (72, 140, 90, 158), field_value=False)
widget(two, pymupdf.PDF_WIDGET_TYPE_COMBOBOX, "country", (72, 180, 300, 200), choice_values=["CH", "DE", "AT"], field_value="CH", text_fontsize=11)
three.insert_text((72, 72), "Third page", fontname="helv", fontsize=14)
group = document.add_ocg("Layer One", on=True)
three.insert_text((72, 120), "Optional content", fontname="helv", fontsize=12, oc=group)
target = pathlib.Path(sys.argv[1]) / "ann-doc.pdf"
document.save(str(target), garbage=1, deflate=False)
check = pymupdf.open(str(target))
print(f"ok {target}: {check.page_count} pages, {len(list(check[0].annots()))} annots, {[w.field_name for w in check[1].widgets()]}")
