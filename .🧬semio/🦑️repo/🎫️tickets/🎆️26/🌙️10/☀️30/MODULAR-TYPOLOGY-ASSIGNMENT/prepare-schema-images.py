"""🖼️ Crop and downscale the three example axonometries of the source docx and the cover image into authored assignment assets."""

import sys
import zipfile
from io import BytesIO
from pathlib import Path

from PIL import Image

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").exists())
DOCX = ROOT / "temp/Aufgabenstellung_ Parametrischer Entwurf modularer Typologien.docx"
TARGET = ROOT / "🎓️teaching/🏛️architecture/🏙️typology/📝️assignment"
NAMES = {"word/media/image1.png": "schema-m-b-1-q.png", "word/media/image3.png": "schema-m-w-2-h.png", "word/media/image2.png": "schema-e-b-2-qh.png"}

sys.stdout.reconfigure(encoding="utf-8")
TARGET.mkdir(parents=True, exist_ok=True)
with zipfile.ZipFile(DOCX) as archive:
    for member, name in NAMES.items():
        image = Image.open(BytesIO(archive.read(member))).convert("RGBA")
        alpha = image.getchannel("A").point(lambda value: 255 if value > 160 else 0)
        box = alpha.getbbox()
        pad = 24
        image = image.crop((max(box[0] - pad, 0), max(box[1] - pad, 0), min(box[2] + pad, image.width), min(box[3] + pad, image.height)))
        image.thumbnail((720, 720), Image.LANCZOS)
        flat = Image.new("RGBA", image.size, (255, 255, 255, 255))
        flat.alpha_composite(image)
        ink = Image.new("LA", image.size, 0)
        ink.putalpha(flat.convert("L").point(lambda value: 255 - value))
        ink.save(TARGET / name, optimize=True)
        print(name, flat.size, (TARGET / name).stat().st_size)
cover = Image.open(ROOT / "temp/Typologie.png").convert("RGB")
cover.thumbnail((2000, 2000), Image.LANCZOS)
cover.save(TARGET / "cover.jpg", quality=88, optimize=True, progressive=True)
print("cover.jpg", cover.size, (TARGET / "cover.jpg").stat().st_size)
