import glob, os, sys
from PIL import Image, ImageDraw
src, dst, per = sys.argv[1], sys.argv[2], int(sys.argv[3])
os.makedirs(dst, exist_ok=True)
index = []
for d in sorted(glob.glob(os.path.join(src, "*", "*"))):
    if not os.path.isdir(d):
        continue
    asset, scale = os.path.basename(os.path.dirname(d)), os.path.basename(d)
    files = sorted(glob.glob(os.path.join(d, "*.png")))
    for i in range(0, len(files), per):
        chunk = files[i:i + per]
        ims = [Image.open(f).convert("RGB") for f in chunk]
        w = max(im.width for im in ims)
        label_h = 24
        h = sum(im.height + label_h for im in ims)
        sheet = Image.new("RGB", (w, h), "white")
        y = 0
        draw = ImageDraw.Draw(sheet)
        for f, im in zip(chunk, ims):
            draw.text((6, y + 6), os.path.basename(f), fill="black")
            sheet.paste(im, (0, y + label_h)); y += im.height + label_h
        name = f"{asset}-{scale}-{i // per:02d}.png"
        sheet.save(os.path.join(dst, name))
        index.append(f"{name}: {', '.join(os.path.basename(f) for f in chunk)}")
with open(os.path.join(dst, "index.txt"), "w", encoding="utf-8") as fh:
    fh.write("\n".join(index) + "\n")
print(len(index), "sheets")
