"""📐 Checks the controlled page/catalog fixture against genuine PDF metadata in MuPDF."""
from pathlib import Path
import sys

import fitz


def verify(fixture, output):
    streams = fixture.read_bytes().split(b"\n%% STREAM\n")
    with fitz.open() as document:
        page = document.new_page(width=100, height=90)
        catalog = document.pdf_catalog()
        parent = int(document.xref_get_key(page.xref, "Parent")[1].split()[0])
        xrefs = []
        for stream in streams:
            ref = document.get_new_xref()
            document.update_object(ref, "<< >>")
            document.update_stream(ref, stream)
            xrefs.append(f"{ref} 0 R")
        array = document.get_new_xref()
        document.update_object(array, "[" + " ".join(xrefs) + "]")
        resources = document.get_new_xref()
        document.update_object(resources, "<< >>")
        document.update_object(page.xref, f"<< /Type /Page /Parent {parent} 0 R /CropBox [5 2 98 89] /Rotate 90 /Contents {array} 0 R /Annots [] /AA << >> >>")
        document.xref_set_key(parent, "MediaBox", "[0 0 100 90]")
        document.xref_set_key(parent, "Resources", f"{resources} 0 R")
        for key, value in (("Version", "/1.7"), ("PageLayout", "/SinglePage"), ("PageMode", "/UseNone"), ("Lang", "(en)")):
            document.xref_set_key(catalog, key, value)
        document.set_metadata({"title": "Controlled vector"})
        encoded = document.tobytes()
    output.write_bytes(encoded)
    with fitz.open(stream=encoded, filetype="pdf") as document:
        page = document[0]
        assert tuple(page.mediabox) == (0, 0, 100, 90)
        assert tuple(page.cropbox) == (5, 1, 98, 88)
        assert page.rotation == 90 and list(page.annots() or []) == []
        assert document.metadata["title"] == "Controlled vector"
        catalog = document.pdf_catalog()
        for key, value in (("PageLayout", "/SinglePage"), ("PageMode", "/UseNone"), ("Lang", "en")):
            assert document.xref_get_key(catalog, key)[1] == value
        assert [document.xref_stream(ref) for ref in page.get_contents()] == streams
        stroke, fill = page.get_drawings()
        assert stroke["color"] == (0, 0, 1) and fill["fill"] == (1, 0, 0)
    print("MuPDF matched inherited media, authored crop and rotation, empty annotations, catalog metadata, and split vector paint.")


if __name__ == "__main__":
    verify(*map(Path, sys.argv[1:]))
