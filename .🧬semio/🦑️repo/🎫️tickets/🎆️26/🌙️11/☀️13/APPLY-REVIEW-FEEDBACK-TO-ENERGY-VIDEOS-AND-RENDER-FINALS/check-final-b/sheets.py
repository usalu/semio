import sys, glob, os
from PIL import Image, ImageDraw
src, dst = sys.argv[1], sys.argv[2]
os.makedirs(dst, exist_ok=True)
files = sorted(glob.glob(os.path.join(src, "f_*.png")))
W, H = 640, 360
COLS, ROWS = 3, 2
PER = COLS * ROWS
for s in range(0, len(files), PER):
    sheet = Image.new("RGB", (W * COLS, H * ROWS))
    for k, f in enumerate(files[s:s + PER]):
        im = Image.open(f).convert("RGB").resize((W, H))
        d = ImageDraw.Draw(im)
        t = int(os.path.basename(f)[2:7])
        label = f"{t // 60}:{t % 60:02d}"
        d.rectangle((W - 62, 2, W - 2, 18), fill=(0, 0, 0))
        d.text((W - 58, 4), label, fill=(255, 255, 0))
        sheet.paste(im, ((k % COLS) * W, (k // COLS) * H))
    sheet.save(os.path.join(dst, f"s_{s // PER:03d}.png"))
