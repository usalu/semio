"""🔡 Verify PDF Type 3 placement and TJ spacing against MuPDF."""
from pathlib import Path
import json
import sys

import fitz

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
font = data["type3"]

def words(values):
    return " ".join(str(value) for value in values)

with fitz.open() as document:
    width, height = data["page"]
    page = document.new_page(width=width, height=height)
    glyphs = []
    for name, advance, drawing in zip("AB", font["widths"], font["glyphs"]):
        owner = document.get_new_xref()
        document.update_object(owner, "<< >>")
        document.update_stream(owner, f"{advance} 0 d0 {drawing}".encode("ascii"))
        glyphs.append(f"/{name} {owner} 0 R")
    owner = document.get_new_xref()
    document.update_object(owner, (
        f"<< /Type /Font /Subtype /Type3 /FontBBox [{words(font['bbox'])}] /FontMatrix [{words(font['matrix'])}] "
        f"/FirstChar 65 /LastChar 66 /Widths [{words(font['widths'])}] /Encoding << /Differences [65 /A /B] >> "
        f"/CharProcs << {' '.join(glyphs)} >> /Resources << >> >>"
    ))
    document.xref_set_key(page.xref, "Resources", f"<< /Font << /F1 {owner} 0 R >> >>")
    contents = document.get_new_xref()
    document.update_object(contents, "<< >>")
    document.update_stream(contents, data["pageContent"].encode("ascii"))
    page.set_contents(contents)
    with fitz.open(stream=document.tobytes(), filetype="pdf") as reopened:
        actual = reopened[0].get_pixmap(colorspace=fitz.csRGB)
        for probe in data["probes"]:
            observed = list(actual.pixel(*probe["at"])[:3])
            assert observed == probe["rgb"], (probe, observed)
        actual.save(sys.argv[2])
        print(f"MuPDF verified both authored Type 3 glyph paints and the blank adjusted gap across {len(data['probes'])} raster probes.")
