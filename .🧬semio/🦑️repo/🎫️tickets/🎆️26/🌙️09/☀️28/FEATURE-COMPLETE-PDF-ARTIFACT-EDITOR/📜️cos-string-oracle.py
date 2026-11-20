"""🗃️ Verify the COS byte-string fixture through MuPDF's independent parser."""
import sys
from pathlib import Path
import fitz


def verify(path):
    for row in path.read_text(encoding="ascii").splitlines():
        source, expected = (bytes.fromhex(field) for field in row.split(" "))
        with fitz.open() as document:
            document.new_page()
            document.xref_set_key(document.pdf_catalog(), "Example", source.decode("ascii"))
            kind, text = document.xref_get_key(document.pdf_catalog(), "Example")
            assert kind == "string" and text.encode("latin-1") == expected, (source, text, expected)
    print("MuPDF verified all seven literal and hexadecimal PDF byte strings")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
