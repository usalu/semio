import glob, os, sys
from PIL import Image, ImageDraw
Image.MAX_IMAGE_PIXELS = None
g, asset, scale, width, per = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), int(sys.argv[5])
fs = sorted(glob.glob(f"{g}/{asset}/{scale}/*dark-line*.png"))
ims = []
for f in fs:
    im = Image.open(f).convert("RGB"); h = int(im.height * width / im.width)
    im = im.resize((width, h), Image.LANCZOS)
    ImageDraw.Draw(im).text((4, 2), os.path.basename(f).split("--")[1], fill=(200, 0, 0)); ims.append(im)
H = sum(x.height for x in ims)
m = Image.new("RGB", (width, H), "white"); y = 0
for x in ims: m.paste(x, (0, y)); y += x.height
m.save(f"{g}/{asset}/s{scale}.png"); print(m.size)
