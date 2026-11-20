"""📨 Verify that MuPDF inherits trailer values across a classic update of an xref-stream PDF."""
from pathlib import Path
import sys

import fitz

source = Path(sys.argv[1]).read_bytes()
prior = int(source.rpartition(b"startxref")[2].split()[0])
object_offset = len(source) + 1
edited = source + b"\n50 0 obj\n155\nendobj\n"
xref = len(edited)
edited += (
    f"xref\n50 1\n{object_offset:010} 00000 n \ntrailer\n"
    f"<< /Size 51 /Prev {prior} >>\nstartxref\n{xref}\n%%EOF\n"
).encode("ascii")
canonical = edited.replace(b"/Size 51 /Prev", b"/Size 51 /Root 1 0 R /Info 6 0 R /Prev")
with fitz.open(stream=source, filetype="pdf") as original, fitz.open(stream=canonical, filetype="pdf") as updated:
    assert updated.page_count == original.page_count == 1
    assert updated.pdf_catalog() == original.pdf_catalog() == 1
    assert updated.metadata["title"] == original.metadata["title"] == "Compressed"
    assert updated.xref_object(50) == "155"
    assert not updated.is_repaired
    print("MuPDF verified the explicitly materialized catalog and title, retained page 1, and read appended object 50.")
