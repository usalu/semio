import sys, glob, os
from PIL import Image, ImageDraw
src, dst = sys.argv[1], sys.argv[2]
step = int(sys.argv[3]); cols, rows, w, h = 3, 3, 640, 360
files = sorted(glob.glob(os.path.join(src, "f_*.png")))
os.makedirs(dst, exist_ok=True)
per = cols * rows
for s in range(0, len(files), per):
    sheet = Image.new("RGB", (cols * w, rows * h))
    d = ImageDraw.Draw(sheet)
    for k, f in enumerate(files[s:s + per]):
        im = Image.open(f).resize((w, h))
        x, y = (k % cols) * w, (k // cols) * h
        sheet.paste(im, (x, y))
        t = (s + k) * step
        d.rectangle([x, y + h - 16, x + 70, y + h], fill=(0, 0, 0))
        d.text((x + 3, y + h - 14), f"{t // 60}:{t % 60:02d}", fill=(255, 255, 0))
        d.rectangle([x, y, x + w - 1, y + h - 1], outline=(60, 60, 60))
    sheet.save(os.path.join(dst, f"sheet_{s // per:03d}.png"))
