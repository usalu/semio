"""🖼️ Builds a resource-free large natural-file PDF and independently inspects its page programs."""
from pathlib import Path
import json
import sys

import fitz

spec = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))["largeVector"]
PAGE_COUNT = spec["pageCount"]
LINES = spec["segmentCount"]

def write_pdf(destination):
    rows = [b"<< /Type /Catalog /Pages 2 0 R >>"]
    kids = " ".join(f"{3 + page * 2} 0 R" for page in range(PAGE_COUNT))
    rows.append(f"<< /Type /Pages /Kids [{kids}] /Count {PAGE_COUNT} >>".encode("ascii"))
    for page in range(PAGE_COUNT):
        content = bytearray(b"q 0.2 0.3 0.4 RG 0.5 w\n")
        for index in range(LINES):
            position = 20 + index * 6
            content.extend(f"{position} 40 m {position} {400 + page} l\n".encode("ascii"))
        content.extend(b"S Q\n")
        contents = 4 + page * 2
        rows.append(f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 360 480] /Contents {contents} 0 R >>".encode("ascii"))
        rows.append(f"<< /Length {len(content)} >>\nstream\n".encode("ascii") + content + b"endstream")
    output = bytearray(b"%PDF-1.7\n")
    positions = []
    for number, row in enumerate(rows, 1):
        positions.append(len(output))
        output.extend(f"{number} 0 obj\n".encode("ascii") + row + b"\nendobj\n")
    xref = len(output)
    size = len(rows) + 1
    output.extend(f"xref\n0 {size}\n0000000000 65535 f \n".encode("ascii"))
    for offset in positions:
        output.extend(f"{offset:010} 00000 n \n".encode("ascii"))
    output.extend(f"trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode("ascii"))
    destination.write_bytes(output)

def verify(destination):
    with fitz.open(destination) as pdf:
        assert not pdf.is_repaired
        assert pdf.page_count == PAGE_COUNT
        assert pdf.xref_length() == PAGE_COUNT * 2 + 3
        for index in [0, PAGE_COUNT - 1]:
            assert pdf[index].rect == fitz.Rect(0, 0, 360, 480)
            drawing, = pdf[index].get_drawings()
            assert drawing["width"] == 0.5
            assert len(drawing["items"]) == LINES
            stroke = drawing["items"][-1]
            position = 20 + (LINES - 1) * 6
            assert stroke == ("l", fitz.Point(position, 440), fitz.Point(position, 80 - index))
        size = destination.stat().st_size
        assert size == spec["byteLength"] and size > 32 * 1024
        print(f"{size} bytes, {pdf.page_count} pages, {pdf.xref_length() - 1} live objects, {LINES} stroked segments per page")

if __name__ == "__main__":
    file = Path(sys.argv[1])
    write_pdf(file)
    verify(file)
