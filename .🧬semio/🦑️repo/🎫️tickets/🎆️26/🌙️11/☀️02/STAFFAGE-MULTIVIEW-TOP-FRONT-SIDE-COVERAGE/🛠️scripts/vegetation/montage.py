import sys, os, glob
from PIL import Image, ImageDraw

root = sys.argv[1]
out = sys.argv[2]
os.makedirs(out, exist_ok=True)
CELL = 900
for asset in sorted(os.listdir(root)):
    folder = os.path.join(root, asset)
    if not os.path.isdir(folder) or asset == "t":
        continue
    cells = []
    for den in ("500", "200", "100", "50"):
        files = glob.glob(os.path.join(folder, f"*third-1-{den}.png"))
        cells.append((den, files[0] if files else None))
    sheet = Image.new("RGB", (CELL * 2, CELL * 2), "white")
    for i, (den, file) in enumerate(cells):
        x, y = (i % 2) * CELL, (i // 2) * CELL
        d = ImageDraw.Draw(sheet)
        if file is None:
            d.text((x + 20, y + 20), f"1:{den} no sheet", fill="red")
            continue
        im = Image.open(file).convert("RGB")
        im.thumbnail((CELL - 10, CELL - 10))
        sheet.paste(im, (x + 5, y + 5))
        d.text((x + 8, y + 8), f"1:{den}", fill="red")
    sheet.save(os.path.join(out, f"{asset}.png"))
    print("montage", asset, [c[0] for c in cells if c[1]])
