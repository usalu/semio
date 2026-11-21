"""🧾️ Author a large shared OpenType PDF and obtain independent text/raster evidence."""

import hashlib
import json
from pathlib import Path
import zlib
import sys

import pymupdf

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "font-opentype"
FONTS = TICKET / "🗑️generated" / "font-unicode" / "one-byte-simple.pdf"
PAGES = 48
PROGRAM = b"BT /F1 20 Tf 1 0 0 1 20 45 Tm <416752e9> Tj 1 0 0 1 20 75 Tm [<4167> -5 <52e9>] TJ ET\n"
VECTOR = b"q 0.72 0.72 0.72 RG 0.25 w 10 10 m 150 10 l S Q\n" * 15


def stream(dictionary, data):
    return b"<< " + dictionary + f" /Length {len(data)} >>\nstream\n".encode() + data + b"\nendstream"


def pdf(objects):
    output = b"%PDF-1.7\n"
    positions = []
    for number, obj in enumerate(objects, 1):
        positions.append(len(output))
        output += f"{number} 0 obj\n".encode() + obj + b"\nendobj\n"
    offset = len(output)
    size = len(objects) + 1
    output += f"xref\n0 {size}\n0000000000 65535 f \n".encode()
    output += b"".join(f"{position:010} 00000 n \n".encode() for position in positions)
    return output + f"trailer\n<< /Root 1 0 R /Size {size} >>\nstartxref\n{offset}\n%%EOF\n".encode()


def inspect(path):
    with pymupdf.open(path) as document:
        assert not document.is_repaired and len(document) == PAGES
        fonts = document[0].get_fonts(full=True)
        owner, extension, category, name, alias, encoding, *_ = fonts[0]
        assert (owner, extension, category, alias, encoding) == (5, "otf", "Type1", "F1", "WinAnsiEncoding")
        native = document.xref_stream(7)
        assert native.startswith(b"OTTO")
        rows = []
        for page in document:
            text = page.get_text()
            assert text == "AgRé\nAgRé\n"
            rows.append({"text": text, "rgb": hashlib.sha256(page.get_pixmap(alpha=False).samples).hexdigest()})
        return {"fontSha256": hashlib.sha256(native).hexdigest(), "fontName": name, "pages": rows}


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    with pymupdf.open(FONTS) as original:
        font, descriptor = (original.xref_object(index).encode() for index in (5, 6))
        program = original.xref_stream(7)
        unicode = original.xref_stream(8)
        assert original.xref_get_key(5, "ToUnicode") == ("xref", "8 0 R")
    pages = [3] + list(range(9, 9 + (PAGES - 1) * 2, 2))
    kids = " ".join(f"{id} 0 R" for id in pages)
    tree = f"<< /Type /Pages /Kids [{kids}] /Count {PAGES} /MediaBox [0 0 160 110] /Resources << /Font << /F1 5 0 R >> >> >>".encode()
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>", tree,
        b"<< /Type /Page /Parent 2 0 R /Contents 4 0 R >>", stream(b"", PROGRAM + VECTOR),
        font, descriptor, b"", stream(b"", unicode),
    ]
    for page in pages[1:]:
        objects += [f"<< /Type /Page /Parent 2 0 R /Contents {page + 1} 0 R >>".encode(), stream(b"", PROGRAM + VECTOR)]
    receipt = {"oracle": "PyMuPDF " + pymupdf.VersionBind, "fixtures": []}
    for use_flate in (False, True):
        label = "flate" if use_flate else "raw"
        body = zlib.compress(program) if use_flate else program
        objects[6] = stream(b"/Subtype /OpenType" + (b" /Filter /FlateDecode" if use_flate else b""), body)
        data = pdf(objects)
        assert len(data) > 32768 and len(body) < 32640
        output = OUT / (label + ".pdf")
        output.write_bytes(data)
        evidence = inspect(output)
        tail = b'[{"kind":"flate","predictor":null}]' if use_flate else b'[]'
        evidence["identity"] = hashlib.sha256(b"s.stdio.font.opentype" + program + tail).hexdigest()
        evidence["name"] = label
        evidence["byteLength"] = len(data)
        receipt["fixtures"].append(evidence)
    assert receipt["fixtures"][0]["pages"] == receipt["fixtures"][1]["pages"]
    with pymupdf.open(OUT / "raw.pdf") as document:
        shape = (document.xref_object(index) for index in (5, 6))
        lines = ["pdf-simple-opentype-v1", *(" ".join(body.split()) for body in shape),
                 document.xref_stream(7).hex(), document.xref_stream(8).hex(),
                 "416752e9", receipt["fixtures"][0]["pages"][0]["text"].splitlines()[0],
                 *(item["identity"] for item in receipt["fixtures"])]
        (OUT / "simple-opentype.txt").write_text("\n".join(lines) + "\n", encoding="utf8")
    (OUT / "expect.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(receipt["oracle"], [(row["name"], row["byteLength"], row["fontName"], len(row["pages"])) for row in receipt["fixtures"]])


if __name__ == "__main__":
    if sys.argv[1:2] == ["verify"]:
        expected = json.loads((OUT / "expect.json").read_text())["fixtures"][0]
        actual = inspect(Path(sys.argv[2]))
        assert all(actual[key] == expected[key] for key in actual)
        print("MuPDF reopened the emitted OpenType PDF with identical font, text and pixels")
    else:
        prepare()
