import sys, os, glob
from PIL import Image, ImageDraw
root, out = sys.argv[1], sys.argv[2]
files = sorted(glob.glob(os.path.join(root, "*", "*.png")))
CELL = 600
def sheet(chunk, name):
    cols = 3; rows = (len(chunk) + cols - 1) // cols
    s = Image.new("RGB", (CELL * cols, CELL * rows), "white"); d = ImageDraw.Draw(s)
    for i, f in enumerate(chunk):
        im = Image.open(f).convert("RGB"); im.thumbnail((CELL - 10, CELL - 10))
        x, y = (i % cols) * CELL, (i // cols) * CELL
        s.paste(im, (x + 5, y + 5)); d.text((x + 8, y + 8), os.path.basename(os.path.dirname(f)) + " " + os.path.basename(f).split("symbol-")[-1][:30], fill="red")
    s.save(os.path.join(out, name))
os.makedirs(out, exist_ok=True)
for k in range(0, len(files), 9): sheet(files[k:k + 9], f"symbols-{k // 9 + 1}.png")
print(len(files), "symbol sheets")
