import glob, json, os, sys
import fitz
from PIL import Image, ImageDraw

root = ".🧬semio/🎓️teaching/⚡️cache/staffage"
out = sys.argv[1]
ids = sys.argv[2].split(",")
n = int(sys.argv[3]) if len(sys.argv) > 3 else 8
os.makedirs(out, exist_ok=True)
rows = []
for cast in ids:
    folder = os.path.dirname(sorted(glob.glob(f"{root}/*/{cast}/🔣️.json"), key=os.path.getmtime)[-1])
    index = json.load(open(f"{folder}/🔣️.json", encoding="utf8"))
    count = len(index["frames"])
    picks = [round(i * (count - 1) / (n - 1)) for i in range(n)]
    cells = []
    for p in picks:
        doc = fitz.open(f"{folder}/{p:04d}.svg")
        page = doc[0]
        pix = page.get_pixmap(matrix=fitz.Matrix(160 / page.rect.height, 160 / page.rect.height), alpha=True)
        img = Image.frombytes("RGBA", (pix.width, pix.height), pix.samples)
        bg = Image.new("RGBA", img.size, (14, 20, 26, 255))
        bg.alpha_composite(img)
        d = ImageDraw.Draw(bg)
        d.text((2, 2), f"{cast[-22:]} {p * index['durationS'] / max(count - 1, 1):.0f}s", fill=(255, 255, 255, 255))
        cells.append(bg)
    width = sum(c.width for c in cells)
    row = Image.new("RGBA", (width, 160), (0, 0, 0, 255))
    x = 0
    for c in cells:
        row.paste(c, (x, 0))
        x += c.width
    rows.append(row)
W = max(r.width for r in rows)
sheet = Image.new("RGBA", (W, 160 * len(rows)), (0, 0, 0, 255))
for i, r in enumerate(rows):
    sheet.paste(r, (0, 160 * i))
path = f"{out}/sheet_{ids[0]}.png"
sheet.convert("RGB").save(path)
print(path, sheet.size)
