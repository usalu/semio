"""📥️ Independently decode the retained stream fixture with PyMuPDF."""
from pathlib import Path
import sys

import fitz


def verify(path):
    with fitz.open(path) as document:
        assert not document.is_repaired
        assert document.page_count == 1
        assert document[0].get_contents() == [4]
        assert document.xref_stream(4) == b"0 0 m 1 1 l S\n" * 192
        assert document.xref_stream(5) == bytes([10, 30, 50, 20, 40, 60])
    print("MuPDF matched all 2,688 content bytes and six predictor output samples")


def verify_xref(path):
    with fitz.open(path) as document:
        assert not document.is_repaired
        assert document.page_count == 2
        assert document.xref_object(3) == "null"
        assert document.xref_get_key(9, "Title") == ("string", "Updated")
        assert document.xref_stream(10) == bytes.fromhex(
            "0000000003ffff0000000000000101000003c4000001000003ea0000"
        )
    print("MuPDF matched all 28 xref bytes, freed entry and updated document info")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
    if len(sys.argv) > 2:
        verify_xref(Path(sys.argv[2]))
