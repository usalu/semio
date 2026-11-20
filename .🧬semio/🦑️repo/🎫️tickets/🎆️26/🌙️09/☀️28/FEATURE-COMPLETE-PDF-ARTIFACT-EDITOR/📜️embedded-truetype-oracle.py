"""🔠 Verify the document's embedded TrueType face and advances independently with MuPDF."""
from pathlib import Path
import json
import sys

import fitz

case = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
font = Path(case["font"]).read_bytes()
font_info = case["widths"]
with fitz.open() as doc:
    page = doc.new_page(width=case["page"][0], height=case["page"][1])
    file = doc.get_new_xref()
    doc.update_object(file, f"<< /Length1 {len(font)} >>")
    doc.update_stream(file, font)
    descriptor = doc.get_new_xref()
    doc.update_object(descriptor, (
        f"<< /Type /FontDescriptor /FontName /ShareTechMono-Regular /Flags 32 /FontBBox [0 -250 700 850] "
        f"/ItalicAngle 0 /Ascent 800 /Descent -200 /CapHeight 675 /StemV 80 /FontFile2 {file} 0 R >>"
    ))
    owner = doc.get_new_xref()
    first, last, advance = (font_info[key] for key in ("first", "last", "advance"))
    widths = " ".join([str(advance)] * (last - first + 1))
    doc.update_object(owner, (
        f"<< /Type /Font /Subtype /TrueType /BaseFont /ShareTechMono-Regular /Encoding /WinAnsiEncoding "
        f"/FontDescriptor {descriptor} 0 R /FirstChar {first} /LastChar {last} /Widths [{widths}] >>"
    ))
    doc.xref_set_key(page.xref, "Resources", f"<< /Font << /F1 {owner} 0 R >> >>")
    contents = doc.get_new_xref()
    doc.update_object(contents, "<< >>")
    doc.update_stream(contents, f"BT /F1 32 Tf 1 0 0 1 20 40 Tm <{case['codes']}> Tj ET".encode("ascii"))
    page.set_contents(contents)
    with fitz.open(stream=doc.tobytes(), filetype="pdf") as pdf:
        page = pdf[0]
        assert page.get_text().strip() == case["paintText"]
        text = page.get_text("rawdict")
        chars = [char for block in text["blocks"] for line in block["lines"] for span in line["spans"] for char in span["chars"]]
        assert [char["origin"] for char in chars] == [tuple(origin) for origin in case["origins"]]
        base_name, extension, _, source = pdf.extract_font(page.get_fonts()[0][0])
        assert extension == "ttf" and source == font
        print(f"MuPDF verified embedded {base_name}, Unicode text, and all four PDF glyph origins.")
        page.get_pixmap(colorspace=fitz.csRGB).save(sys.argv[2])
    clip = case["clip"]
    doc.update_stream(contents, clip["content"].encode("ascii"))
    with fitz.open(stream=doc.tobytes(), filetype="pdf") as pdf:
        image = pdf[0].get_pixmap(colorspace=fitz.csRGB)
        for left, top, right, bottom in clip["regions"]:
            assert any(image.pixel(x, y) == (255, 0, 0) for x in range(left, right) for y in range(top, bottom))
        for probe in clip["probes"]:
            assert image.pixel(*probe["at"]) == tuple(probe["rgb"]), probe
        print("MuPDF verified both glyph clip regions, their empty gap, and painting after Q.")
