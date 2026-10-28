"""📐 Independently measure the compiled example grid using pdfplumber."""
from pathlib import Path
import hashlib
import sys
import pdfplumber

ticket = Path(__file__).resolve().parent
source = Path(sys.argv[1]).resolve()
with pdfplumber.open(source) as pdf:
    page = pdf.pages[0]
    boundaries = sorted(
        line["top"] for line in page.lines
        if abs(line["top"] - line["bottom"]) < 0.001
        and line["x1"] - line["x0"] > 400
        and 0.35 < line["linewidth"] < 0.45
    )
    assert len(boundaries) == 9, boundaries
    header = [
        line for line in page.lines
        if abs(line["x1"] - line["x0"]) < 0.001
        and 45 < line["bottom"] - line["top"] < 46
        and 0.35 < line["linewidth"] < 0.45
    ]
    assert len(header) == 5, header
    assert all(abs((line["bottom"] - line["top"]) * 25.4 / 72 - 16) < 0.001 for line in header)
    rows = [
        line for line in page.lines
        if abs(line["x1"] - line["x0"]) < 0.001
        and 17 < line["bottom"] - line["top"] < 17.1
        and 0.35 < line["linewidth"] < 0.45
    ]
    assert len(rows) == 40, len(rows)
    assert all(abs((line["bottom"] - line["top"]) * 25.4 / 72 - 6) < 0.001 for line in rows)
    row_text = [
        page.crop((56.7, boundaries[index] + 0.5, 538.5, boundaries[index + 1] - 0.5)).extract_text()
        for index in range(8)
    ]
    assert all(row_text[:4]), row_text
    assert not any(row_text[4:]), row_text
    assert "2.5 2.5 3.5" in row_text[0]
    assert "1.5 1.5 2.5" in row_text[1]
    assert "2 2 3" in row_text[2]
digest = hashlib.sha256(source.read_bytes()).hexdigest()
report = (
    "\n## Native English Verification\n\n"
    "The frozen sources compiled through repo tools exec with the pinned native Tectonic 0.16.9 and prepared local TeX resources. "
    "The compiler exited 0 and produced three pages, 134,139 bytes, with zero overfull and zero underfull boxes. "
    "All extracted text stays inside each page. Frozen bilingual SHA-256 checks passed again before PDF review.\n\n"
    "Independent pdfplumber measurements found five header grid edges (four columns), nine horizontal lower boundaries (one header plus eight parameter rows), "
    "a 16 mm header, and forty 6 mm vertical row segments (five boundaries across eight rows). "
    "The first four rows contain the original Length/Width/Height/Form values; the last four rows contain no text. "
    "The numeric columns remain 2.5/1.5/2, 2.5/1.5/2, and 3.5/2.5/3.\n\n"
    "All three PDFium-rendered pages were visually inspected at full size. The example shares the exercise table title tabs, outline, column divisions, and row geometry. "
    "Its three solids are centered with no header numbers. The six K3 edge captions, cycle, and three criteria remain readable and contained on page 1. "
    "The two capsule matrices retain their existing page 2 layout. The K16 graph, twenty marked edges and ledger fit page 3 with no clipped labels.\n\n"
    "English PDF SHA-256: " + digest + ".\n\n"
    "Retained verification inputs: same-table-content-check.py and same-table-grid-check.py. "
    "Temporary compiler output and full-page PNGs are under the ticket generated directory for parent cleanup.\n"
)
with (ticket / "same-table-content.md").open("a", encoding="utf-8") as output:
    output.write(report)
print("Native grid passed: four columns, nine rows, 16 mm header, eight 6 mm rows, four populated and four blank. SHA256=" + digest)
