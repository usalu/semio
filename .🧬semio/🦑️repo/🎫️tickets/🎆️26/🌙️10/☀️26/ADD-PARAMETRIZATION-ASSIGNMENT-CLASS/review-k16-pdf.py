from pathlib import Path
import json
import sys
import pypdfium2 as pdfium
from PIL import Image
source = Path(sys.argv[1])
output = Path(__file__).parent / "🗑️generated" / sys.argv[2]
output.mkdir(parents=True, exist_ok=True)
document = pdfium.PdfDocument(str(source))
thumbs = []
for index in range(len(document)):
    page = document[index]
    image = page.render(scale=1.6).to_pil()
    image.save(output / f"page-{index + 1}.png")
    thumb = image.copy()
    thumb.thumbnail((420, 620))
    thumbs.append(thumb)
    page.close()
canvas = Image.new("RGB", (840, 620 * ((len(thumbs) + 1) // 2)), "#dddddd")
for index, thumb in enumerate(thumbs):
    canvas.paste(thumb, ((index % 2) * 420, (index // 2) * 620))
canvas.save(output / "contact.png")
document.close()
print(json.dumps({"pages": len(thumbs), "output": str(output)}, ensure_ascii=True))
