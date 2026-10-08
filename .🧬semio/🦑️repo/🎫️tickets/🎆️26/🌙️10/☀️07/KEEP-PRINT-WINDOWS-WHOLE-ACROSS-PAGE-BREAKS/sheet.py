"""🗂️ Tiles rendered probe pages into one contact sheet."""
import glob, sys
from PIL import Image

folder, columns = sys.argv[1], int(sys.argv[2])
pages = [Image.open(path) for path in sorted(glob.glob(f"{folder}/pg-*.png"))]
width, height = pages[0].size
rows = -(-len(pages) // columns)
sheet = Image.new("RGB", (columns * (width + 6), rows * (height + 6)), "grey")
for index, page in enumerate(pages):
    sheet.paste(page, ((index % columns) * (width + 6), (index // columns) * (height + 6)))
sheet.save(f"{folder}/sheet.png")
