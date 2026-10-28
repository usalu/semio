"""🔍 Verify right-only native assignment chips and render every page."""
from pathlib import Path
import json
import sys
import pypdfium2 as pdfium
import pdfplumber
from pypdf import PdfReader
from PIL import Image

source = Path(sys.argv[1]).resolve()
ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/chip-content"
output.mkdir(parents=True, exist_ok=True)
reader = PdfReader(source)
assert len(reader.pages) == 3, len(reader.pages)
assert reader.trailer["/Root"].get("/Lang") == "en-US"
assert reader.metadata.title == "Finger Exercise: Parametrization"
assert reader.metadata.author == "Architecture"
checks = []
with pdfplumber.open(source) as pdf:
    all_text = chr(10).join(page.extract_text() or "" for page in pdf.pages)
    assert "Table A" not in all_text and "Table B" not in all_text
    assert sum(word["text"] == "Example" for page in pdf.pages for word in page.extract_words()) == 1
    for page_index, number in [(0, "1.1.a"), (1, "1.2.a"), (1, "1.2.b")]:
        page = pdf.pages[page_index]
        words = page.extract_words()
        match = [word for word in words if word["text"] == number]
        assert len(match) == 1, (number, match)
        chip = match[0]
        assert chip["x0"] > 400, chip
        chip_line = [word["text"] for word in words if abs(word["top"] - chip["top"]) < 3]
        assert chip_line == ["Table:", number], (number, chip_line)
        checks.append({"page": page_index + 1, "chip": number, "x": chip["x0"], "top": chip["top"]})
    for page in pdf.pages:
        outside = [char["text"] for char in page.chars if char["text"].strip() and (char["x0"] < -1 or char["x1"] > page.width + 1 or char["top"] < -1 or char["bottom"] > page.height + 1)]
        assert not outside, outside
log = source.with_suffix(".log").read_text(encoding="utf-8")
assert "Overfull" not in log and "Underfull" not in log
rendered = pdfium.PdfDocument(str(source))
thumbs = []
for index in range(len(rendered)):
    page = rendered[index]
    image = page.render(scale=2).to_pil()
    image.save(output / f"english-{index + 1}.png")
    image.thumbnail((420, 600))
    thumbs.append(image.copy())
    page.close()
rendered.close()
contact = Image.new("RGB", (420 * len(thumbs), 600), "#dddddd")
for index, image in enumerate(thumbs):
    contact.paste(image, (420 * index, 0))
contact.save(output / "english-contact.png")
result = {"pages": 3, "bytes": source.stat().st_size, "title": reader.metadata.title, "author": reader.metadata.author, "language": "en-US", "automaticRightChips": checks, "overfull": 0, "underfull": 0}
(output / "english.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
print(json.dumps(result))
