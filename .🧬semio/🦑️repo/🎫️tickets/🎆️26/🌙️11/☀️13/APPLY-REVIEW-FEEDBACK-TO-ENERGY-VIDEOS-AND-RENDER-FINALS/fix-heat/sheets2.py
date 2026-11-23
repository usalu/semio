import sys, glob, os
from PIL import Image, ImageDraw
src, dst, a, b = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
os.makedirs(dst, exist_ok=True)
files = sorted(glob.glob(os.path.join(src, "f_*.png")))
W, H = 624, 351
for s in range(a, min(b, len(files)), 9):
    sheet = Image.new("RGB", (W * 3, H * 3))
    for k, f in enumerate(files[s:s + 9]):
        im = Image.open(f).convert("RGB").resize((W, H))
        d = ImageDraw.Draw(im)
        t = (s + k) * 4
        d.text((W - 60, 4), f"{t // 60}:{t % 60:02d}", fill=(255, 255, 0))
        sheet.paste(im, ((k % 3) * W, (k // 3) * H))
    sheet.save(os.path.join(dst, f"a_{s // 9:02d}.png"))
