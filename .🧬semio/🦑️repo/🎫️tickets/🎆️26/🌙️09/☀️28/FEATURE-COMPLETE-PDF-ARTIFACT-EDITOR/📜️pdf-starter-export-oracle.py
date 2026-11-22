"""🔍 Verifies the actual mounted starter PDF download with MuPDF."""
import sys
from pathlib import Path
import pymupdf

path = Path(sys.argv[1])
with pymupdf.open(path) as document:
    assert document.page_count == 1, document.page_count
    assert document[0].rotation == 90, document[0].rotation
    box = document[0].mediabox
    assert 0 < box.width < 2_000 and 0 < box.height < 2_000, box
    print("[DEBUG] MuPDF reopened the one-page mounted export with exact rotation", document[0].rotation)
