"""🖊️ Compare following glyph baselines across text and font edits using MuPDF."""

from math import isclose
import fitz


def following_origin(show):
    with fitz.open() as doc:
        page = doc.new_page(width=200, height=300)
        page.insert_font(fontname="helv")
        page.insert_font(fontname="cour")
        stream = doc.get_new_xref()
        doc.update_object(stream, "<<>>")
        doc.update_stream(stream, f"BT /helv 12 Tf 20 100 Td {show} (B) Tj ET".encode("ascii"))
        page.set_contents(stream)
        return next(char["origin"] for block in page.get_text("rawdict")["blocks"]
                    for line in block["lines"] for span in line["spans"] for char in span["chars"] if char["c"] == "B")


original = following_origin("(A) Tj")
advance = fitz.get_text_length("A", fontname="helv", fontsize=12)
restore = f"1 0 0 1 20 100 Tm [{-advance * 1000 / 12}] TJ"
for show in (f"(Longer replacement) Tj {restore}", f"/cour 12 Tf (A) Tj /helv 12 Tf {restore}"):
    result = following_origin(show)
    assert all(isclose(a, b, abs_tol=0.0001) for a, b in zip(result, original)), (original, result)
print("Both edits preserve the next glyph at", original)
