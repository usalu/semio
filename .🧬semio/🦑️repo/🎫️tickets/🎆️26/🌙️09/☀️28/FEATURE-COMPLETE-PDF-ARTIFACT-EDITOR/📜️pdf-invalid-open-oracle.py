"""📄 Verify that the mounted invalid PDF fixture reaches a real PDF parser and is refused."""
import json
from pathlib import Path
import sys

import pymupdf

ticket = Path(__file__).resolve().parent
law = json.loads((ticket / "🔣️mounted-invalid-pdf.json").read_text(encoding="utf8"))
source = Path(sys.argv[1])
source.write_bytes((law["header"] + "\n" + law["body"]).encode("ascii"))
try:
    with pymupdf.open(source) as document:
        assert document.page_count == 0
except pymupdf.FileDataError:
    pass
print("[DEBUG] MuPDF could not open any pages from the malformed PDF")
