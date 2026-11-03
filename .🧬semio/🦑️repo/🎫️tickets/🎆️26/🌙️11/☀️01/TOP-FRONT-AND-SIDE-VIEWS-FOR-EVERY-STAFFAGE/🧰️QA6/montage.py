import glob, os, sys
from PIL import Image, ImageDraw
Image.MAX_IMAGE_PIXELS = None
g = sys.argv[1]; asset = sys.argv[2]; scale = sys.argv[3]; width = int(sys.argv[4]) if len(sys.argv) > 4 else 2200
per = int(sys.argv[5]) if len(sys.argv) > 5 else 4
fs = sorted(glob.glob(f"{g}/{asset}/{scale}/*dark-line*.png"))
ims = []
for f in fs:
    im = Image.open(f).convert("RGB"); h = int(im.height * width / im.width)
    im = im.resize((width, h), Image.LANCZOS)
    d = ImageDraw.Draw(im); d.text((4, 2), os.path.basename(f).split("--")[1], fill=(200, 0, 0))
    ims.append(im)
for i in range(0, len(ims), per):
    grp = ims[i:i + per]; H = sum(x.height for x in grp)
    m = Image.new("RGB", (width, H), "white"); y = 0
    for x in grp: m.paste(x, (0, y)); y += x.height
    m.save(f"{g}/{asset}/m{scale}-{i // per}.png")
