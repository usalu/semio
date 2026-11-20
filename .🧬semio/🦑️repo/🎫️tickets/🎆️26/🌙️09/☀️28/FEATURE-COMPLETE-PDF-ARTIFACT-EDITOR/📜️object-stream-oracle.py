"""📦️ Check the compressed-object and indirect length PDF fixtures with MuPDF."""
from pathlib import Path
import sys

import fitz


def verify_compressed(path):
    with fitz.open(path) as pdf:
        assert not pdf.is_repaired
        assert pdf.page_count == 1
        assert pdf.metadata["title"] == "Compressed"
        assert pdf.metadata["subject"] == "Stream"
        assert pdf.xref_get_key(1, "Probe") == ("xref", "7 0 R")
        assert pdf.xref_object(7) == "[ true /Object (packed) 6 0 R ]"
        body = pdf.xref_stream(4)
        assert body[:26] == b"% packed objects\n6 0 7 44\n"
        assert body[26:70] == b"<< /Title (Compressed) /Subject (Stream) >>\n"
        assert body[70:] == b"[ true /Object (packed) 6 0 R ]\n"


def verify_length(path):
    with fitz.open(path) as pdf:
        assert not pdf.is_repaired
        assert pdf.xref_get_key(4, "Length") == ("xref", "5 0 R")
        assert pdf.xref_object(5).strip() == "53"
        assert pdf.xref_stream(4) == b"BT /F1 12 Tf 10 70 Td (prefix endstream suffix) Tj ET"


if __name__ == "__main__":
    verify_compressed(Path(sys.argv[1]))
    verify_length(Path(sys.argv[2]))
    print("MuPDF matched two typed compressed members and all 53 indirect-length content bytes")
