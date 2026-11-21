"""🔎 Validate PDF graph, foreign stream, trailer identity and sparse generations independently."""
import hashlib
from pathlib import Path
import zlib

import pymupdf

stage = Path(__file__).resolve().parent / "🗑️generated" / "classic-graph"
size = 2_097_177
period = bytes(range(251))
whole, rest = divmod(size, len(period))
digest = hashlib.sha256(period * whole + period[:rest]).digest()

for version in ("graph.pdf", "graph-flate.pdf"):
    path = stage / version
    with path.open("rb") as stream:
        stream.seek(-1350, 2)
        tail = stream.read()
        assert b" 00003 n " in tail and b"/Private 11 0 R" in tail
        assert b"/Prev" not in tail
        assert b"/ID[<000102> (stable)]" in tail
    with pymupdf.open(path) as pdf:
        assert not pdf.is_repaired and pdf.page_count == 1
        assert pdf.metadata["title"] == "Edited title"
        assert pdf[0].get_text().strip() == "Pinned ordinary graph"
        assert pdf.xref_get_key(11, "Nested")[0] == "dict"
        assert len(pdf.xref_stream(17)) == size
        assert hashlib.sha256(pdf.xref_stream(17)).digest() == digest
        if version.endswith("-flate.pdf"):
            assert pdf.xref_get_key(17, "Filter") == ("name", "/FlateDecode")
            assert hashlib.sha256(zlib.decompress(pdf.xref_stream_raw(17))).digest() == digest
            assert zlib.decompress(pdf.xref_stream_raw(19)) == b""
        else:
            assert hashlib.sha256(pdf.xref_stream_raw(17)).digest() == digest
    print(f"[DEBUG] MuPDF verified {version} page, metadata, private trailer/object and exact decoded {size}-byte stream")

for mode, info in (("replace", "8 0 R"), ("add", "18 0 R"), ("clear", None)):
    path = stage / f"graph-graft-{mode}.pdf"
    with pymupdf.open(path) as pdf:
        assert not pdf.is_repaired and pdf.page_count == 1
        assert pdf[0].rotation == 90
        assert pdf[0].get_text().strip() == "Pinned ordinary graph"
        assert pdf.metadata["title"] == ("" if info is None else "Paid updated title")
        trailer = pdf.pdf_trailer()
        assert "/Private 11 0 R" in trailer and "/Prev" not in trailer
        assert ("/Info " + info in trailer) if info is not None else ("/Info" not in trailer)
        assert hashlib.sha256(pdf.xref_stream_raw(17)).digest() == digest
    print(f"[DEBUG] MuPDF verified {mode} Info graft, page rotation and untouched private stream")
