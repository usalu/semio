"""🔍 Render a native K16 PDF and inspect its page metadata independently."""
from pathlib import Path
import json
import sys
import pypdfium2 as pdfium
import pdfplumber
from pypdf import PdfReader
from PIL import Image

TICKET = Path(__file__).resolve().parent
SOURCE = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else TICKET / "🗑️generated/compiled/parametrization-en.pdf"
PREFIX = sys.argv[2] if len(sys.argv) > 2 else "k16-en"
OUTPUT = TICKET / "🗑️generated/review"
OUTPUT.mkdir(parents=True, exist_ok=True)
reader = PdfReader(SOURCE)
assert len(reader.pages) == 3
rendered = pdfium.PdfDocument(SOURCE)
thumbs = []
for index in range(len(rendered)):
    page = rendered[index]
    image = page.render(scale=2).to_pil()
    image.save(OUTPUT / f"{PREFIX}-{index + 1}.png")
    image.thumbnail((420, 600))
    thumbs.append(image.copy())
contact = Image.new("RGB", (420 * len(thumbs), 600), "#dddddd")
for index, image in enumerate(thumbs):
    contact.paste(image, (420 * index, 0))
contact.save(OUTPUT / f"{PREFIX}-contact.png")
with pdfplumber.open(SOURCE) as pdf:
    pages = []
    for page in pdf.pages:
        outside = [char["text"] for char in page.chars if char["text"].strip() and (char["x0"] < 0 or char["x1"] > page.width + 1 or char["top"] < 0 or char["bottom"] > page.height + 1)]
        assert not outside, outside
        pages.append({"text": page.extract_text(), "outside": outside})
log = SOURCE.with_suffix(".log").read_text(encoding="utf-8")
result = {"pages": len(reader.pages), "bytes": SOURCE.stat().st_size, "title": reader.metadata.title, "author": reader.metadata.author, "language": reader.trailer["/Root"].get("/Lang"), "overfull": log.count("Overfull"), "underfull": log.count("Underfull"), "pageText": pages}
(OUTPUT / f"{PREFIX}.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
print(json.dumps({key: value for key, value in result.items() if key != "pageText"}))
