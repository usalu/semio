import sys
from pathlib import Path

import av
from PIL import Image, ImageDraw

out = Path(sys.argv[1])
items = [a.split("@") for a in sys.argv[2:]]
cells = []
for path, fraction in items:
    with av.open(path) as container:
        frames = [f.to_image() for f in container.decode(video=0)]
    image = frames[int(float(fraction) * (len(frames) - 1))].convert("RGB")
    ImageDraw.Draw(image).text((8, 6), Path(path).stem + " @" + fraction, fill=(255, 255, 0))
    cells.append(image)
width, height = cells[0].size
columns = 3
rows = (len(cells) + columns - 1) // columns
sheet = Image.new("RGB", (columns * width, rows * height))
for i, cell in enumerate(cells):
    sheet.paste(cell, ((i % columns) * width, (i // columns) * height))
sheet.save(out)
print(out, sheet.size)
