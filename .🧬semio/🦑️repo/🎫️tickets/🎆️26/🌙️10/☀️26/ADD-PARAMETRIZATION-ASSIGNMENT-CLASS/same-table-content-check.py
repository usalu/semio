"""🔎 Verify frozen bilingual contents and inspect the native English example table."""
from pathlib import Path
import hashlib
import sys
import json
import pdfplumber
import pypdfium2 as pdfium
from pypdf import PdfReader

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework/🛍️products/📓️print").is_dir())
TEMPLATE = ROOT / "🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment"
EXPECTED = {
    "de": "a63dad4f88f1c0844b4b61c9cb463100a3b404d53c62c0aa57b94ed5d5a56422",
    "en": "c570fe2d8300b8cc42515233e4c3de3bca8e2336a54688760587f96a5c782448",
}
for language, digest in EXPECTED.items():
    raw = (TEMPLATE / f"parametrization-{language}.content.tex").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == digest, language
    content = raw.decode("utf-8")
    assert content.count("\\begin{AssignmentTable}") == 3
    assert content.count("\\AssignmentWorkedExampleTable") == 1
    assert content.count("\\shortstack[l]") == 4
    assert content.count("\\item[") == 3
    assert content.count("\\clearpage") == 2
    assert "1\\to2\\to3\\to1" in content
    assert content.count("\\AssignmentExerciseTable") == 2
    assert content.count("\\AssignmentSelectedLedger") == 1
if len(sys.argv) == 1:
    print("Frozen bilingual content checks passed.")
    raise SystemExit(0)
source = Path(sys.argv[1]).resolve()
target = TICKET / "🗑️generated/same-table-en-review"
target.mkdir(parents=True, exist_ok=True)
reader = PdfReader(source)
assert len(reader.pages) == 3
summary = []
with pdfplumber.open(source) as pdf:
    for index, page in enumerate(pdf.pages):
        text = page.extract_text()
        outside = [
            char["text"] for char in page.chars
            if char["text"].strip() and (
                char["x0"] < 0 or char["x1"] > page.width + 0.5
                or char["top"] < 0 or char["bottom"] > page.height + 0.5
            )
        ]
        assert not outside, outside
        summary.append({
            "page": index + 1,
            "text": text,
            "lineCount": len(page.lines),
            "rectCount": len(page.rects),
            "curveCount": len(page.curves),
            "outside": outside,
        })
        if index == 0:
            for term in ["Example", "Length", "Width", "Height", "Form", "Unique.", "Consistent.", "Minimal."]:
                assert term in text, term
            assert "\\Delta" not in text
            (target / "page-1-geometry.json").write_text(
                json.dumps({"lines": page.lines, "rects": page.rects, "curves": page.curves}, default=str),
                encoding="utf-8",
            )
        if index == 1:
            assert "Dwelling Capsules" in text
            assert "Table A" in text and "Table B" in text
document = pdfium.PdfDocument(source)
for index in range(len(document)):
    page = document[index]
    page.render(scale=2).to_pil().save(target / f"page-{index + 1}.png")
    page.close()
document.close()
log = source.with_suffix(".log").read_text(encoding="utf-8")
assert "Overfull" not in log
(target / "inspection.json").write_text(json.dumps({
    "pages": len(reader.pages), "bytes": source.stat().st_size,
    "overfull": log.count("Overfull"), "underfull": log.count("Underfull"),
    "pageChecks": summary,
}, indent=2), encoding="utf-8")
print(json.dumps({"pages": len(reader.pages), "bytes": source.stat().st_size, "overfull": log.count("Overfull"), "underfull": log.count("Underfull"), "review": str(target)}))
