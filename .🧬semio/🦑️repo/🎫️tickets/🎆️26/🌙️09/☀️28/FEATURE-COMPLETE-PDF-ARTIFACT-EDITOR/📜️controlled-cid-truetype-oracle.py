"""🔠 Author and independently inspect a large PDF with a shared CID TrueType font."""

import hashlib
from io import BytesIO
import json
from pathlib import Path
import runpy
import sys
import zlib

import pymupdf
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib import TTFont

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "font-cid-truetype"
CASE = json.loads((TICKET / "🔣️controlled-cid-truetype.json").read_text(encoding="utf8"))
FORM = runpy.run_path(str(TICKET / "controlled-opentype-oracle.py"))
NAME = CASE["fontName"]
PAGES = CASE["pageCount"]


def build_font():
    """🪶 Build deterministic TrueType outlines and verify identity glyph indices."""
    records = CASE["glyphs"]
    names = [".notdef", *(entry["name"] for entry in records)]
    glyphs = {".notdef": TTGlyphPen(None).glyph()}
    for entry in records:
        pen = TTGlyphPen(None)
        start, *remaining = entry["contour"]
        pen.moveTo(tuple(start))
        for point in remaining:
            pen.lineTo(tuple(point))
        pen.closePath()
        glyphs[entry["name"]] = pen.glyph()
    font = FontBuilder(CASE["unitsPerEm"], isTTF=True)
    font.font.recalcTimestamp = False
    font.setupGlyphOrder(names)
    font.setupCharacterMap({entry["unicode"]: entry["name"] for entry in records})
    font.setupGlyf(glyphs)
    font.setupHorizontalMetrics({".notdef": (600, 0), **{
        entry["name"]: (entry["width"], min(point[0] for point in entry["contour"])) for entry in records
    }})
    font.setupHorizontalHeader(ascent=800, descent=-200)
    font.setupNameTable({"familyName": NAME, "styleName": "Regular", "psName": NAME, "fullName": NAME})
    font.setupOS2(sTypoAscender=800, sTypoDescender=-200, usWinAscent=800, usWinDescent=200)
    font.setupPost()
    font.font["head"].created = font.font["head"].modified = 3825360000
    output = BytesIO()
    font.save(output)
    data = output.getvalue()
    with TTFont(BytesIO(data)) as check:
        assert [check.getGlyphID(check.getBestCmap()[entry["unicode"]]) for entry in records] == CASE["codes"]
        assert [check["hmtx"].metrics[name][0] for name in names[1:]] == [entry["width"] for entry in records]
    return data


def inspect(path):
    """🔬 Compare the attached font, Unicode and raster for every page."""
    with pymupdf.open(path) as document:
        assert not document.is_repaired and len(document) == PAGES
        fonts = document[0].get_fonts(full=True)
        assert len(fonts) == 1
        owner, extension, category, name, alias, encoding, *_ = fonts[0]
        assert (extension, category, name, alias, encoding) == ("ttf", "Type0", NAME, "F1", "Identity-H")
        assert document.xref_get_key(owner, "Subtype") == ("name", "/Type0")
        name, extension, _, program = document.extract_font(owner)
        assert name == NAME and extension == "ttf" and program.startswith(b"\x00\x01\x00\x00")
        pages = []
        for page in document:
            text = page.get_text()
            assert text == (CASE["text"] + "\n") * 2
            pages.append({"text": text, "rgb": hashlib.sha256(page.get_pixmap(alpha=False).samples).hexdigest()})
        return {"fontSha256": hashlib.sha256(program).hexdigest(), "pages": pages}


def prepare():
    """📚 Write raw and Flate variants of the same reference PDF."""
    OUT.mkdir(parents=True, exist_ok=True)
    program = build_font()
    ids = [3] + list(range(10, 10 + (PAGES - 1) * 2, 2))
    kids = " ".join(f"{owner} 0 R" for owner in ids)
    width, height = CASE["pageSize"]
    tree = f"<< /Type /Pages /Kids [{kids}] /Count {PAGES} /MediaBox [0 0 {width} {height}] /Resources << /Font << /F1 5 0 R >> >> >>"
    contents = FORM["stream"](b"", CASE["content"].encode() + FORM["VECTOR"])
    charmap = "\n".join(f"<{code:04X}> <{entry['unicode']:04X}>" for code, entry in zip(CASE["codes"], CASE["glyphs"]))
    cmap = (
        "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n"
        "/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n"
        "/CMapName /SemioUnicode def\n/CMapType 2 def\n"
        "1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n"
        f"{len(CASE['codes'])} beginbfchar\n{charmap}\nendbfchar\n"
        "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n"
    )
    widths = " ".join(str(record["width"]) for record in CASE["glyphs"])
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>", tree.encode(),
        b"<< /Type /Page /Parent 2 0 R /Contents 4 0 R >>", contents,
        f"<< /Type /Font /Subtype /Type0 /BaseFont /{NAME} /Encoding /Identity-H /DescendantFonts [6 0 R] /ToUnicode 9 0 R >>".encode(),
        (f"<< /Type /Font /Subtype /CIDFontType2 /BaseFont /{NAME} /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >>"
         f" /DW 1000 /W [1 [{widths}]] /CIDToGIDMap /Identity /FontDescriptor 7 0 R >>").encode(),
        (f"<< /Type /FontDescriptor /FontName /{NAME} /Flags 32 /FontBBox [0 -200 700 800]"
         " /ItalicAngle 0 /Ascent 800 /Descent -200 /CapHeight 740 /StemV 80 /FontFile2 8 0 R >>").encode(),
        b"", FORM["stream"](b"", cmap.encode()),
    ]
    for owner in ids[1:]:
        objects += [f"<< /Type /Page /Parent 2 0 R /Contents {owner + 1} 0 R >>".encode(), contents]
    evidence = {"oracle": "PyMuPDF " + pymupdf.VersionBind, "fixtures": []}
    for flate in CASE["variation"]:
        label = "flate" if flate else "raw"
        filters = b'[{"kind":"flate","predictor":null}]' if flate else b'[]'
        dictionary = f"/Length1 {len(program)}".encode() + (b" /Filter /FlateDecode" if flate else b"")
        body = zlib.compress(program) if flate else program
        objects[7] = FORM["stream"](dictionary, body)
        data = FORM["pdf"](objects)
        assert len(data) > 32_768 and len(program) < 32_640
        target = OUT / (label + ".pdf")
        target.write_bytes(data)
        actual = inspect(target)
        actual["identity"] = hashlib.sha256(b"s.stdio.font.truetype" + program + filters).hexdigest()
        actual.update(name=label, byteLength=len(data))
        evidence["fixtures"].append(actual)
    assert evidence["fixtures"][0]["pages"] == evidence["fixtures"][1]["pages"]
    with pymupdf.open(OUT / "raw.pdf") as document:
        dictionaries = (" ".join(document.xref_object(i).split()) for i in (5, 6, 7))
        codes = "".join(f"{code:04x}" for code in CASE["codes"])
        rows = ["pdf-cid-truetype-v1", *dictionaries, program.hex(), cmap.encode().hex(),
                codes, CASE["text"], *(item["identity"] for item in evidence["fixtures"])]
        (OUT / "cid-truetype.txt").write_text("\n".join(rows) + "\n", encoding="utf8")
    (OUT / "expect.json").write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf8")
    print("[DEBUG]", evidence["oracle"], [(case["name"], case["byteLength"], len(case["pages"])) for case in evidence["fixtures"]])


if __name__ == "__main__":
    if sys.argv[1:2] == ["verify"]:
        expected = json.loads((OUT / "expect.json").read_text(encoding="utf8"))["fixtures"][0]
        actual = inspect(Path(sys.argv[2]))
        assert all(data == expected[key] for key, data in actual.items())
        print("[DEBUG] MuPDF reopened all CID TrueType pages with identical program, Unicode and pixels")
    else:
        prepare()
