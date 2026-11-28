"""🧺️ Writes `rich.pdf`, a three-page PDF 1.7 fixture for the action sweeps, authored by a third-party library (pymupdf): text in two fonts, a red rectangle, a blue line, a PNG image, annotations (text note, highlight, square, free text, link), an AcroForm (text, check box, combo box), an outline, a named destination, an embedded file, an optional content group, page labels and document info.

usage: uv run --offline --with pymupdf==1.26.7 python -I rich-fixture.py <out dir>
See ./🗺️feature-inventory.md
"""
import pathlib
import sys

import pymupdf


def widget(page, kind, name, rect, **values):
    field = pymupdf.Widget()
    field.field_type = kind
    field.field_name = name
    field.rect = pymupdf.Rect(*rect)
    for key, value in values.items():
        setattr(field, key, value)
    return page.add_widget(field)


def build(folder: pathlib.Path):
    document = pymupdf.open()
    for width, height in [(612, 792), (612, 792), (595, 842)]:
        document.new_page(width=width, height=height)
    one, two, three = document[0], document[1], document[2]
    one.insert_text((72, 72), "Rich Fixture Helvetica", fontname="helv", fontsize=18)
    one.insert_text((72, 110), "Second font Times", fontname="tiro", fontsize=12)
    one.draw_rect(pymupdf.Rect(72, 150, 272, 250), color=(0, 0, 0), fill=(1, 0, 0), width=2)
    one.draw_line((72, 300), (400, 300), color=(0, 0, 1), width=3)
    pixmap = pymupdf.Pixmap(pymupdf.csRGB, pymupdf.IRect(0, 0, 8, 8), False)
    pixmap.set_rect(pixmap.irect, (0, 128, 255))
    one.insert_image(pymupdf.Rect(400, 150, 480, 230), pixmap=pixmap)
    note = one.add_text_annot((500, 72), "Note body")
    note.set_info(title="Reviewer", content="Note body")
    note.update()
    one.add_highlight_annot(pymupdf.Rect(72, 58, 300, 76)).update()
    square = one.add_rect_annot(pymupdf.Rect(300, 400, 400, 480))
    square.set_colors(stroke=(0, 1, 0))
    square.update()
    one.add_freetext_annot(pymupdf.Rect(72, 500, 300, 540), "Free text annotation", fontsize=11).update()
    one.insert_link({"kind": pymupdf.LINK_GOTO, "from": pymupdf.Rect(72, 600, 200, 620), "page": 2, "to": pymupdf.Point(72, 72)})
    two.insert_text((72, 72), "Form page", fontname="helv", fontsize=14)
    widget(two, pymupdf.PDF_WIDGET_TYPE_TEXT, "name", (72, 100, 300, 120), field_value="Ada", text_fontsize=11)
    widget(two, pymupdf.PDF_WIDGET_TYPE_CHECKBOX, "agree", (72, 140, 90, 158), field_value=False)
    widget(two, pymupdf.PDF_WIDGET_TYPE_COMBOBOX, "country", (72, 180, 300, 200), choice_values=["CH", "DE", "AT"], field_value="CH", text_fontsize=11)
    three.insert_text((72, 72), "Third page", fontname="helv", fontsize=14)
    group = document.add_ocg("Layer One", on=True)
    three.insert_text((72, 120), "Optional content", fontname="helv", fontsize=12, oc=group)
    document.set_toc([[1, "Start", 1], [1, "Form", 2], [2, "Third", 3]])
    document.embfile_add("attached.txt", b"embedded file body", filename="attached.txt", desc="Attachment")
    document.set_page_labels([{"startpage": 0, "prefix": "", "style": "r", "firstpagenum": 1}, {"startpage": 2, "prefix": "A-", "style": "D", "firstpagenum": 1}])
    document.set_metadata({"title": "Rich Fixture", "author": "Sweep", "subject": "Every feature", "keywords": "pdf,sweep"})
    catalog = document.pdf_catalog()
    kind, value = document.xref_get_key(catalog, "Names")
    if kind == "xref":
        document.xref_set_key(int(value.split()[0]), "Dests", f"<< /Names [(chapter-two) [{two.xref} 0 R /XYZ 72 720 0]] >>")
    else:
        document.xref_set_key(catalog, "Names/Dests", f"<< /Names [(chapter-two) [{two.xref} 0 R /XYZ 72 720 0]] >>")
    target = folder / "rich.pdf"
    document.save(str(target), garbage=1, deflate=False)
    check = pymupdf.open(str(target))
    assert check.page_count == 3 and check.embfile_count() == 1 and len(check.get_toc()) == 3, (check.page_count, check.embfile_count(), check.get_toc())
    assert [w.field_name for w in check[1].widgets()], "no widgets"
    assert len(list(check[0].annots())) >= 4, list(check[0].annots())
    print(f"ok {target}: {check.page_count} pages, {len(list(check[0].annots()))} annotations, {len(list(check[1].widgets()))} widgets, toc {check.get_toc()}, embedded {check.embfile_names()}, dests {list(check.resolve_names())}")


build(pathlib.Path(sys.argv[1]))
