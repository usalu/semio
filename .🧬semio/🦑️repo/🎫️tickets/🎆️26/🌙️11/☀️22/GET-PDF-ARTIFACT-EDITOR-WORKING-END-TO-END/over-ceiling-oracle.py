#!/usr/bin/env python3
"""pymupdf oracle for the over-ceiling fixture (slice 13): the independent reader opens the spec's pages, decodes each page's single content stream to the spec's size and finds exactly the spec's operators in it.

usage: uv run --offline --with pymupdf==1.26.7 python -I over-ceiling-oracle.py <fixtures directory>
"""
import json
import sys
from pathlib import Path

import pymupdf

directory = Path(sys.argv[1]) / "🛑️over-ceiling"
spec = json.loads((directory / "spec.json").read_text())
document = pymupdf.open(directory / "over-ceiling.pdf")
assert document.page_count == spec["pageCount"], document.page_count
for page in document:
    contents = page.get_contents()
    assert len(contents) == 1
    decoded = document.xref_stream(contents[0])
    assert len(decoded) == spec["decodedBytesPerPage"], (len(decoded), spec["decodedBytesPerPage"])
    assert decoded.count(b"q") + decoded.count(b"Q") == spec["operatorsPerPage"]
assert spec["fileBytes"] == (directory / "over-ceiling.pdf").stat().st_size
assert spec["pageCount"] * spec["operatorsPerPage"] * 56 > spec["ceiling"] // 2, "the typed operators alone approach the ceiling"
print("over-ceiling fixture agrees with pymupdf:", spec)
