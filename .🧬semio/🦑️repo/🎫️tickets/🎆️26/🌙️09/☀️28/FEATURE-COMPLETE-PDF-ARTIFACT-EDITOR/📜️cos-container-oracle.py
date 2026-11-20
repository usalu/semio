"""🔍 Validate the genuine COS container fixture with PyMuPDF 1.24.14."""
from pathlib import Path
import sys

import fitz


def verify(path):
    with fitz.open(path) as document:
        assert not document.is_repaired
        assert document.page_count == 1
        assert document.xref_get_key(1, "Probe") == ("xref", "4 0 R")
        assert document.xref_get_key(4, "Numbers") == ("array", "[12 0 6 0 R -17 .5 9 602214076]")
        assert document.xref_get_key(4, "Text") == ("string", "a(b)+")
        assert document.xref_get_key(4, "Hex") == ("string", "¤ð")
        assert document.xref_get_key(4, "Flags") == ("array", "[true false null]")
        assert document.xref_get_key(4, "Nested") == (
            "dict", "<</A#20B[/x/#C3#A9/#E9<</Empty[]>>]/First/first/Second/second>>",
        )
        assert document.xref_get_key(6, "Referenced") == ("bool", "true")
    print("MuPDF matched the genuine PDF's typed container, names, numbers, strings, and reference")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
