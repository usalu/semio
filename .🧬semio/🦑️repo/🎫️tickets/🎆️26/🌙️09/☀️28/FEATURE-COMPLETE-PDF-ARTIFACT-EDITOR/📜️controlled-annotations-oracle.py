"""🔎 Verify the controlled annotation corpus through MuPDF."""
import json
import sys
from pathlib import Path

import fitz


def verify(fixture, output):
    data = json.loads(fixture.read_text())
    pdf = bytearray(b"%PDF-1.7\n")
    offsets = ["0000000000 65535 f \n"]
    for number, generation, entries in data["objects"]:
        assert number == len(offsets)
        offsets.append(f"{len(pdf):010d} {generation:05d} n \n")
        pdf.extend(f"{number} {generation} obj\n{entries}\nendobj\n".encode("ascii"))
    start = len(pdf)
    pdf.extend((f"xref\n0 {len(offsets)}\n" + "".join(offsets) +
                f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{start}\n%%EOF\n").encode("ascii"))
    output.write_bytes(pdf)

    with fitz.open(stream=pdf, filetype="pdf") as document:
        page, second = document[0], document[1]
        assert len(document) == 2
        expected = data["expected"]
        note, = page.annots()
        assert note.xref == 5 and note.type[1] == "Text"
        for pdf_key, expected_key in (("Contents", "contents"), ("NM", "name"), ("T", "title"), ("Subj", "subject")):
            assert document.xref_get_key(note.xref, pdf_key)[1] == expected["note"][expected_key]
        assert document.xref_get_key(note.xref, "Name")[1] == "/" + expected["note"]["icon"]
        assert document.xref_get_key(note.xref, "Open")[1] == str(expected["note"]["open"]).lower()
        assert note.flags == expected["note"]["flags"] and note.colors["stroke"] == expected["note"]["color"]
        assert json.loads(document.xref_get_key(note.xref, "Rect")[1].replace(" ", ",")) == expected["note"]["rect"]
        link, = page.get_links()
        assert link["uri"] == expected["link"]["uri"] and link["xref"] == 6
        assert document.xref_get_key(link["xref"], "H")[1] == "/" + expected["link"]["highlight"]
        for key in ("Border", "Rect"):
            assert json.loads(document.xref_get_key(link["xref"], key)[1].replace(" ", ",")) == expected["link"][key.lower()]
        later, = second.annots()
        assert later.info["content"] == expected["secondNote"]["contents"]
        assert json.loads(document.xref_get_key(later.xref, "Rect")[1].replace(" ", ",")) == expected["secondNote"]["rect"]
    print("MuPDF verified both notes, the URI link, authored rectangles, style, and page association.")


if __name__ == "__main__":
    verify(*map(Path, sys.argv[1:]))
