from pathlib import Path
import json
import sys
import pypdfium2 as pdfium
from pypdf import PdfReader
from PIL import Image, ImageOps, ImageDraw

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
language = sys.argv[1] if len(sys.argv) > 1 else "de"
source = generated / (language + ".pdf")
reader = PdfReader(source)
document = pdfium.PdfDocument(source)
rows = []
images = []
for index, page in enumerate(document):
    bitmap = page.render(scale=1.4)
    image = bitmap.to_pil().convert("RGB")
    image.save(generated / (language + "-page-" + str(index + 1) + ".png"))
    thumb = ImageOps.contain(image, (595, 842))
    images.append(thumb)
    text = reader.pages[index].extract_text()
    rows.append({"page": index + 1, "width": float(reader.pages[index].mediabox.width), "height": float(reader.pages[index].mediabox.height), "characters": len(text), "text": text})
    assert len(text) > 100, "Unexpected almost-empty page"
sheet = Image.new("RGB", (595 * 2, 880 * ((len(images) + 1) // 2)), "#d9dde0")
draw = ImageDraw.Draw(sheet)
for index, image in enumerate(images):
    x = (index % 2) * 595
    y = (index // 2) * 880
    sheet.paste(image, (x, y + 28))
    draw.text((x + 12, y + 8), "Page " + str(index + 1), fill="black")
sheet.save(generated / (language + "-contact.png"))
result = {"pages": len(reader.pages), "metadata": dict(reader.metadata), "language": str(reader.trailer["/Root"].get("/Lang")), "details": rows}
(generated / (language + "-inspection.json")).write_text(json.dumps(result, ensure_ascii=False, indent=2))
print("[DEBUG]", language, len(reader.pages), "pages", source.stat().st_size, "bytes", result["language"])
print("[DEBUG] Text lengths:", [row["characters"] for row in rows])
