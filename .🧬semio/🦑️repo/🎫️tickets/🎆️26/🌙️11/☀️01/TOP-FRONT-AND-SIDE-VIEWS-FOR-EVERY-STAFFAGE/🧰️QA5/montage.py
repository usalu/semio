import glob, os, sys
from PIL import Image, ImageDraw
root, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
per = {"50": (2, 3), "100": (2, 4), "200": (3, 4), "500": (4, 6)}
for adir in sorted(glob.glob(os.path.join(root, "*", ""))):
    aid = os.path.basename(os.path.dirname(adir))
    for sc, (cols, rows) in per.items():
        files = sorted(f for f in glob.glob(os.path.join(adir, f"*--views-dark-line-1-{sc}.png")))
        for n in range(0, len(files), cols * rows):
            chunk = files[n:n + cols * rows]
            ims = [Image.open(f).convert("RGB") for f in chunk]
            tw = 1900 // cols
            th = max(int(i.height * tw / i.width) for i in ims)
            sheet = Image.new("RGB", (cols * tw, ((len(ims) + cols - 1) // cols) * th), "white")
            for k, im in enumerate(ims):
                im = im.resize((tw, int(im.height * tw / im.width)), Image.LANCZOS)
                sheet.paste(im, ((k % cols) * tw, (k // cols) * th))
            sheet.save(os.path.join(out, f"{aid}-{sc}-{n // (cols * rows)}.png"))
