"""🧭 Verify prefixed PDF offsets and retained incremental revisions in MuPDF."""
from pathlib import Path
import re
import sys
import fitz

prefix = b"\n% document preamble\n"
source = Path(sys.argv[1]).read_bytes()
delta = len(prefix)

def offset(line):
    return f"{int(line[1]) + delta:010d}".encode("ascii") + line[2]

absolute = re.sub(rb"(?m)^([0-9]{10})( [0-9]{5} n)", offset, source)
absolute = re.sub(rb"(startxref\n|/Prev )([0-9]+)", lambda m: m[1] + str(int(m[2]) + delta).encode("ascii"), absolute)
for label, data in (("absolute", absolute), ("header-relative", source)):
    with fitz.open(stream=prefix + data, filetype="pdf") as doc:
        print(label, doc.page_count, doc.is_repaired, doc.metadata["title"], repr(doc.xref_object(3)))
        assert doc.page_count == 2
        assert doc.metadata["title"] == "Updated"
        assert doc.is_repaired
