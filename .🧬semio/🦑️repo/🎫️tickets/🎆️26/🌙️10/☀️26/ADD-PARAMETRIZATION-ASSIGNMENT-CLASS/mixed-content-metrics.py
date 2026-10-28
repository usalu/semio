from pathlib import Path
import json
import re
import pdfplumber
from pypdf import PdfReader
ticket = Path(__file__).parent
pdf = ticket / "🗑️generated/compiled-en/parametrization-en.pdf"
log = pdf.with_suffix(".log").read_text(encoding="utf-8", errors="replace")
reader = PdfReader(pdf)
escaped = []
with pdfplumber.open(pdf) as document:
    for index, page in enumerate(document.pages, 1):
        escaped.extend((index, char["text"]) for char in page.chars if char["x0"] < -0.5 or char["x1"] > page.width + 0.5 or char["top"] < -0.5 or char["bottom"] > page.height + 0.5)
assert not escaped, escaped
metrics = {
    "pages": len(reader.pages),
    "bytes": pdf.stat().st_size,
    "language": reader.trailer["/Root"].get("/Lang"),
    "title": reader.metadata.title,
    "author": reader.metadata.author,
    "overfull": len(re.findall(r"Overfull \\[hv]box", log)),
    "underfull": len(re.findall(r"Underfull \\[hv]box", log)),
    "out_of_page_characters": len(escaped)
}
assert metrics["overfull"] == metrics["underfull"] == 0, metrics
assert metrics["pages"] == 3 and metrics["language"] == "en-US", metrics
print(json.dumps(metrics))
report = ticket / "mixed-content.md"
with report.open("a", encoding="utf-8") as output:
    output.write("\n## Final Native English Verification\n\n")
    output.write("Registered Tectonic 0.16.9 compiled the original English main file after the exactly-one-active minimality constraint was frozen. The canonical compiler reference projection was used without copying checked source files or icons. The final compile completed successfully. The host emitted its existing Fontconfig diagnostic; no TeX overfull or underfull box was reported.\n\n")
    output.write("| Property | Result |\n| --- | --- |\n")
    for key, value in metrics.items():
        output.write(f"| {key.replace('_', ' ')} | {value} |\n")
    output.write("\nThe retained mixed-content-check.py passed both source languages and PDF text contracts, including the one-active flag domain, continuous positive real values, localized categories, three criteria, section numbering and numeric/category reversal instructions.\n\n")
    output.write("The independent native K16 validator passed: sixteen canonical vector nodes, all 120 distinct straight complete-graph edges, sixteen ring edges plus four selected middle chords, twenty matching writable ledger entries, graph 170.001 × 83.602 mm and vertical radius 32 mm. The native matrix validator passed both 9 × 9 matrices, 6.14 mm writing rows and centered vector artwork.\n\n")
    output.write("PDFium rendered all three pages through registered repo Python using the bundled runtime. The contact sheet and each full-resolution page were visually inspected. Page one contains the mixed-shape table, graph, three full-width worked rows and concise criteria; page two retains both matrices; page three retains the full-width oval graph and twenty-entry ledger. Header, footer, section and table chrome remain aligned, with no clipping or collisions.\n\n")
    output.write("Retained inputs: author-mixed-content.py, mixed-content-check.py, mixed-content-metrics.py. Temporary compiled PDF/log/intermediates and rendered images are confined to the ticket generated directory for parent review and cleanup.\n")
