"""🧺️ Writes document-level fixtures for sweep-document.ts with pymupdf: `doc.pdf` (3 pages, text, outline, named destination, embedded file, optional content, page labels, info; no annotations or forms, which the controlled importer defers) and `plain.pdf` (1 page).

usage: uv run --offline --with pymupdf==1.26.7 python -I document-fixture.py <out dir>
"""
import pathlib
import sys

import pymupdf


def build(folder: pathlib.Path):
    document = pymupdf.open()
    for width, height in [(612, 792), (612, 792), (595, 842)]:
        document.new_page(width=width, height=height)
    one, two, three = document[0], document[1], document[2]
    one.insert_text((72, 72), "Doc Fixture Page One", fontname="helv", fontsize=18)
    two.insert_text((72, 72), "Doc Fixture Page Two", fontname="helv", fontsize=14)
    three.insert_text((72, 72), "Doc Fixture Page Three", fontname="helv", fontsize=14)
    group = document.add_ocg("Layer One", on=True)
    three.insert_text((72, 120), "Optional content", fontname="helv", fontsize=12, oc=group)
    document.set_toc([[1, "Start", 1], [1, "Middle", 2], [2, "End", 3]])
    document.embfile_add("attached.txt", b"embedded file body", filename="attached.txt", desc="Attachment")
    document.set_page_labels([{"startpage": 0, "prefix": "", "style": "r", "firstpagenum": 1}, {"startpage": 2, "prefix": "A-", "style": "D", "firstpagenum": 1}])
    document.set_metadata({"title": "Doc Fixture", "author": "Sweep", "subject": "Document level", "keywords": "pdf,sweep"})
    catalog = document.pdf_catalog()
    kind, value = document.xref_get_key(catalog, "Names")
    dest = f"<< /Names [(chapter-two) [{two.xref} 0 R /XYZ 72 720 0]] >>"
    if kind == "xref":
        document.xref_set_key(int(value.split()[0]), "Dests", dest)
    else:
        document.xref_set_key(catalog, "Names/Dests", dest)
    document.save(str(folder / "doc.pdf"), garbage=1, deflate=False)
    plain = pymupdf.open()
    page = plain.new_page(width=612, height=792)
    page.insert_text((72, 72), "Plain Page", fontname="helv", fontsize=14)
    plain.save(str(folder / "plain.pdf"), deflate=False)
    for name, method in (("encrypted-aes256", pymupdf.PDF_ENCRYPT_AES_256), ("encrypted-rc4", pymupdf.PDF_ENCRYPT_RC4_128)):
        locked = pymupdf.open(str(folder / "plain.pdf"))
        locked.save(str(folder / f"{name}.pdf"), encryption=method, user_pw="user", owner_pw="owner", deflate=False)
    garbage = folder / "garbage.pdf"
    garbage.write_bytes(b"this is not a pdf at all\n" * 40)
    check = pymupdf.open(str(folder / "doc.pdf"))
    print("ok", check.page_count, check.get_toc(), check.embfile_names())


build(pathlib.Path(sys.argv[1]))
