#!/usr/bin/env python3
"""Writes the font fixture: real-world font programs (pdfTeX Type 3 bitmap font, embedded Type 1, TrueType, CFF) lifted out of the bachelor thesis, plus composite fonts (predefined CMap, embedded TrueType with ToUnicode), each showing schema-first text.

usage: uv run --offline --with pymupdf==1.26.7 python font-fixture.py <bachelor-thesis.pdf> <fixture directory>
"""
import json
import re
import sys
from pathlib import Path

import fitz

THESIS_FONTS = {"type3": 154, "type1": 419, "truetype": 791, "cff": 808}

source = fitz.open(sys.argv[1])
target_directory = Path(sys.argv[2])
document = fitz.open()
copied = {}


def copy(xref, origin=source):
    key = (id(origin), xref)
    if key in copied:
        return copied[key]
    new = document.get_new_xref()
    copied[key] = new
    text = origin.xref_object(xref, compressed=True)
    is_stream = origin.xref_is_stream(xref)
    data = origin.xref_stream(xref) if is_stream else None
    text = re.sub(r"/Filter\s*/\w+|/Filter\s*\[[^\]]*\]|/DecodeParms\s*<<[^>]*>>|/Length\s+\d+(\s+0\s+R)?", "", text) if is_stream else text
    text = re.sub(r"(\d+) 0 R", lambda match: f"{copy(int(match.group(1)), origin)} 0 R", text)
    document.update_object(new, text)
    if is_stream:
        document.update_stream(new, data, compress=True)
    return new


def stream(content):
    xref = document.get_new_xref()
    document.update_object(xref, "<<>>")
    document.update_stream(xref, content.encode("latin-1"), compress=True)
    return xref


def page(fonts, content):
    created = document.new_page(width=400, height=300)
    names = " ".join(f"/{name} {xref} 0 R" for name, xref in fonts.items())
    document.xref_set_key(created.xref, "Resources", f"<</Font<<{names}>>/ProcSet[/PDF/Text/ImageB]>>")
    document.xref_set_key(created.xref, "Contents", f"{stream(content)} 0 R")


def show(font, size, x, y, operand):
    return f"BT /{font} {size} Tf {x} {y} Td {operand} Tj ET\n"


fonts = {name: copy(xref) for name, xref in THESIS_FONTS.items()}
consolas_widths = source.xref_get_key(THESIS_FONTS["truetype"], "Widths")[1].strip("[]").split()
consolas = "".join(chr(32 + index) for index, width in enumerate(consolas_widths) if width != "0" and 32 + index > 32)[:12]

scratch = fitz.open()
scratch_page = scratch.new_page()
scratch_page.insert_text((50, 50), "Yes", fontname="china-s")
descriptor = source.xref_get_key(THESIS_FONTS["truetype"], "FontDescriptor")[1].split()[0]
program = source.xref_stream(int(source.xref_get_key(int(descriptor), "FontFile2")[1].split()[0]))
scratch_page.insert_font(fontname="CX", fontbuffer=program)
scratch_page.insert_text((50, 80), consolas[:6], fontname="CX")
composite = {}
for font in scratch_page.get_fonts(full=True):
    composite["embedded" if font[4] == "CX" else "predefined"] = copy(font[0], scratch)
glyphs = fitz.Font(fontbuffer=program)
gids = "".join(f"{glyphs.has_glyph(ord(char)):04X}" for char in consolas[:6])
ucs2 = "".join(f"{ord(char):04X}" for char in "Yes")

page({"CM": fonts["type1"], "TT": fonts["truetype"], "T3": fonts["type3"]}, "".join([
    show("T3", 24, 20, 240, "<414243444546>"),
    show("CM", 18, 20, 180, "(1+2=3)"),
    show("TT", 14, 20, 120, f"({consolas})"),
    "BT /TT 14 Tf 20 80 Td [(" + consolas[:3] + ") -120 (" + consolas[3:6] + ")] TJ ET\n",
]))
page({"MP": fonts["cff"], "CJ": composite["predefined"], "CX": composite["embedded"]}, "".join([
    show("MP", 18, 20, 240, "(Basic chart)"),
    show("CJ", 18, 20, 180, f"<{ucs2}>"),
    show("CX", 18, 20, 120, f"<{gids}>"),
]))
document.set_metadata({"title": "Font fixture", "producer": "font-fixture.py"})
target_directory.mkdir(parents=True, exist_ok=True)
document.save(target_directory / "fonts.pdf", garbage=0, deflate=False, no_new_id=True)

reopened = fitz.open(target_directory / "fonts.pdf")
spec = {
    "pages": [
        {"fonts": ["CM", "TT", "T3"], "kinds": ["Type1", "TrueType", "Type3"], "text": ["ABCDEF", "1+2=3", consolas]},
        {"fonts": ["MP", "CJ", "CX"], "kinds": ["Type1", "Type0", "Type0"], "text": ["Basic chart", "Yes", consolas[:6]]},
    ],
    "fontCount": 6,
    "programs": {"T3": None, "CM": "FontFile", "TT": "FontFile2", "MP": "FontFile3", "CJ": None, "CX": "FontFile2"},
}
for index, (opened, expected) in enumerate(zip(reopened, spec["pages"])):
    names = sorted(font[4] for font in opened.get_fonts(full=True))
    lines = [line.strip() for line in opened.get_text().split("\n") if line.strip()]
    print(index + 1, names, lines)
    assert names == sorted(expected["fonts"]), names
    for text in expected["text"]:
        assert any(text in line for line in lines), (text, lines)
(target_directory / "spec.json").write_text(json.dumps(spec, indent=2) + "\n")
print("wrote", target_directory / "fonts.pdf", "and spec.json")
