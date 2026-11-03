"""usage: python montage.py <asset> <scale> <tile-width> <per-page> : stacks sheets of one asset/scale into captioned pages."""
import glob
import os
import sys

from PIL import Image, ImageDraw

HERE = os.path.dirname(os.path.abspath(__file__))
T = os.path.dirname(HERE)
asset, scale, width, per = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
d = os.path.join(T, "🗑️generated", "QA11", asset)
files = sorted(glob.glob(os.path.join(d, f"*-1-{scale}.png")))
files = [f for f in files if "--views-" in f]
out = os.path.join(T, "🗑️generated", "QA11", "montage")
os.makedirs(out, exist_ok=True)
tiles = []
for f in files:
    im = Image.open(f).convert("RGB")
    h = int(im.height * width / im.width)
    im = im.resize((width, h), Image.LANCZOS)
    cap = os.path.basename(f).split("--views")[0].replace(asset + "--", "")
    tile = Image.new("RGB", (width, h + 18), "white")
    tile.paste(im, (0, 18))
    ImageDraw.Draw(tile).text((4, 3), cap, fill="red")
    tiles.append(tile)
for i in range(0, len(tiles), per):
    chunk = tiles[i:i + per]
    page = Image.new("RGB", (width, sum(t.height for t in chunk)), "white")
    y = 0
    for t in chunk:
        page.paste(t, (0, y)); y += t.height
    page.save(os.path.join(out, f"{asset}-{scale}-p{i // per}.png"))
print(len(tiles))
