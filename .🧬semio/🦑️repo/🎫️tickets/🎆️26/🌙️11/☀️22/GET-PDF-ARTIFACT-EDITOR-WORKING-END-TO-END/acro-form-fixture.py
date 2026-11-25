"""Writes the acro-form fixture: a fillable form as Acrobat and LibreOffice export it (widgets with appearance streams over a shared Helvetica default resource), and the spec (schema first) the Rust laws and the pymupdf oracle both read.

Fields: `address` (a container with the text fields `name` and `email`), `agree` (check box), `country` (combo box), `color` (a radio group of three widget-only kids), `notes` (a multi-line text field whose two widget-only kids sit on page 2 and page 1). `/DR` holds Helvetica and an `/Encoding` entry the reference lifter ignores.

usage: uv run --offline --with pymupdf==1.26.7 python acro-form-fixture.py <fixture directory>
"""
import json
import pathlib
import re
import sys

import pymupdf


def add(page, field_type, name, rect, **kwargs):
    widget = pymupdf.Widget()
    widget.field_type = field_type
    widget.field_name = name
    widget.rect = pymupdf.Rect(*rect)
    for key, value in kwargs.items():
        setattr(widget, key, value)
    return page.add_widget(widget)


def build(path: pathlib.Path):
    document = pymupdf.open()
    one = document.new_page(width=300, height=300)
    name = add(one, pymupdf.PDF_WIDGET_TYPE_TEXT, "name", (20, 20, 200, 40), field_value="Ada", text_fontsize=12)
    email = add(one, pymupdf.PDF_WIDGET_TYPE_TEXT, "email", (20, 50, 200, 70), field_value="ada@example.org", text_fontsize=12)
    agree = add(one, pymupdf.PDF_WIDGET_TYPE_CHECKBOX, "agree", (20, 80, 40, 100), field_value=True)
    country = add(one, pymupdf.PDF_WIDGET_TYPE_COMBOBOX, "country", (20, 110, 200, 130), choice_values=["CH", "DE", "FR"], field_value="CH", text_fontsize=12)
    radios = [add(one, pymupdf.PDF_WIDGET_TYPE_CHECKBOX, f"c{index}", (20, y, 40, y + 15)) for index, y in enumerate((140, 160, 180))]
    two = document.new_page(width=300, height=300)
    notes = add(two, pymupdf.PDF_WIDGET_TYPE_TEXT, "notes", (20, 20, 250, 80), field_value="p2", text_fontsize=11, field_flags=pymupdf.PDF_TX_FIELD_IS_MULTILINE)
    draft = path.with_suffix(".draft.pdf")
    document.save(draft, garbage=0, deflate=False, no_new_id=True)

    document = pymupdf.open(draft)
    catalog = document.pdf_catalog()
    ap_font = re.search(r"/Helv (\d+) 0 R", "".join(document.xref_object(n, compressed=True) for n in range(1, document.xref_length())))
    helvetica = int(ap_font.group(1))
    xrefs = {"name": name.xref, "email": email.xref, "agree": agree.xref, "country": country.xref, "radios": [radio.xref for radio in radios], "notes": notes.xref}

    def create(body: str) -> int:
        number = document.get_new_xref()
        document.update_object(number, body)
        return number

    def unfield(number: int, parent: int):
        body = document.xref_object(number, compressed=True)
        for key in ("FT", "T", "Ff", "V", "DA"):
            body = re.sub(rf"/{key}\s*(\([^)]*\)|/\w+|\d+)", "", body)
        document.update_object(number, body[:-2] + f"/Parent {parent} 0 R>>")

    def adopt(number: int, parent: int):
        document.update_object(number, document.xref_object(number, compressed=True)[:-2] + f"/Parent {parent} 0 R>>")

    address = create("<</T(address)/Kids[%d 0 R %d 0 R]>>" % (xrefs["name"], xrefs["email"]))
    adopt(xrefs["name"], address)
    adopt(xrefs["email"], address)
    color = create("<</FT/Btn/Ff 49152/T(color)/V/Off/Kids[%s]>>" % " ".join(f"{radio} 0 R" for radio in xrefs["radios"]))
    for radio in xrefs["radios"]:
        unfield(radio, color)
    extra = create("<</Type/Annot/Subtype/Widget/Rect[20 200 250 260]/F 4/AP<</N %d 0 R>>>>" % int(re.search(r"/N (\d+) 0 R", document.xref_object(xrefs["notes"], compressed=True)).group(1)))
    field = create("<</FT/Tx/T(notes)/V(p2)/Ff 4096/DA(/Helv 11 Tf 0 g)/Q 1/Kids[%d 0 R %d 0 R]>>" % (xrefs["notes"], extra))
    unfield(xrefs["notes"], field)
    adopt(extra, field)
    document.xref_set_key(extra, "P", f"{document[0].xref} 0 R")
    annots = document.xref_get_key(document[0].xref, "Annots")[1]
    document.xref_set_key(document[0].xref, "Annots", annots[:-1] + f" {extra} 0 R]")
    encoding = create("<</Type/Encoding/Differences[24/breve/caron]>>")
    fields = [address, xrefs["agree"], xrefs["country"], color, field]
    document.xref_set_key(catalog, "AcroForm", "<</Fields[%s]/DR<</Font<</Helv %d 0 R>>/Encoding<</PDFDocEncoding %d 0 R>>>>/DA(/Helv 0 Tf 0 g)/NeedAppearances false/SigFlags 0/Q 0>>" % (" ".join(f"{number} 0 R" for number in fields), helvetica, encoding))
    document.save(path, garbage=0, deflate=False, no_new_id=True)
    draft.unlink()


SPEC = {
    "pages": 2,
    "fields": [
        {"name": "address", "type": "Container", "children": ["name", "email"]},
        {"name": "agree", "type": "Button", "children": []},
        {"name": "country", "type": "Choice", "children": []},
        {"name": "color", "type": "Button", "children": [], "widgets": 3},
        {"name": "notes", "type": "Text", "children": [], "widgets": 2},
    ],
    "widgetsPerPage": [8, 1],
    "defaultFonts": ["Helv"],
}


def main():
    directory = pathlib.Path(sys.argv[1])
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / "acro-form.pdf"
    build(path)
    (directory / "spec.json").write_text(json.dumps(SPEC, indent=2) + "\n")
    document = pymupdf.open(path)
    assert document.page_count == SPEC["pages"] and document.is_form_pdf
    per_page = [len(list(page.widgets())) for page in document]
    assert per_page == SPEC["widgetsPerPage"], per_page
    names = sorted(widget.field_name for page in document for widget in page.widgets())
    assert {"address.name", "address.email", "agree", "country", "color", "notes"} <= set(names), names
    print(f"{path}: {document.page_count} pages, widgets per page {per_page}, fields {names}")


if __name__ == "__main__":
    main()
