import sys, glob, os
from PIL import Image, ImageDraw
src, out, a, b = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
files = sorted(glob.glob(os.path.join(src, "f_*.png")))
W, H = 640, 360
crop_h = 150
ims = []
for i in range(a, b):
    im = Image.open(files[i]).convert("RGB").resize((W, H)).crop((0, H - crop_h, W, H))
    ImageDraw.Draw(im).text((4, 2), f"{i*4//60}:{i*4%60:02d}", fill=(255, 255, 0))
    ims.append(im)
cols = 2
rows = (len(ims) + 1) // 2
sheet = Image.new("RGB", (W * cols, crop_h * rows))
for k, im in enumerate(ims):
    sheet.paste(im, ((k % 2) * W, (k // 2) * crop_h))
sheet.save(out)
