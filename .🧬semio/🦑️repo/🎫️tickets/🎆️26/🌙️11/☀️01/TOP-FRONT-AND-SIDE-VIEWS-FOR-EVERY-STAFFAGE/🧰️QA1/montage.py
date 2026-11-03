"""[DEBUG] QA1 montage: python montage.py <asset> <scale> [cellWidthPx=1100] [perImage=8]
Rasterizes every `--views` SVG sheet of one asset and scale and packs them into grids (2 sheets per row) for reading."""
import glob, os, sys, warnings
warnings.filterwarnings("ignore")
import fitz
from PIL import Image

here = os.path.dirname(os.path.abspath(__file__))
gen = os.path.join(here, "..", "🗑️generated", "QA1")
asset, scale = sys.argv[1], sys.argv[2]
cell = int(sys.argv[3]) if len(sys.argv) > 3 else 1100
per = int(sys.argv[4]) if len(sys.argv) > 4 else 8
files = sorted(glob.glob(os.path.join(gen, asset, scale, "*.svg")))
out = os.path.join(gen, "mont")
os.makedirs(out, exist_ok=True)
images = []
for f in files:
    page = fitz.open(f)[0]
    zoom = cell / page.rect.width
    pm = page.get_pixmap(matrix=fitz.Matrix(zoom, zoom))
    images.append(Image.frombytes("RGB", (pm.width, pm.height), pm.samples))
for n in range(0, len(images), per):
    chunk = images[n:n + per]
    rows = [chunk[i:i + 2] for i in range(0, len(chunk), 2)]
    h = sum(max(i.height for i in r) for r in rows)
    m = Image.new("RGB", (cell * 2 + 10, h), "white")
    y = 0
    for r in rows:
        for j, im in enumerate(r):
            m.paste(im, (j * (cell + 10), y))
        y += max(i.height for i in r)
    name = os.path.join(out, f"{asset}-{scale}-{n // per + 1}.png")
    m.save(name)
    print(os.path.basename(name), m.size)
