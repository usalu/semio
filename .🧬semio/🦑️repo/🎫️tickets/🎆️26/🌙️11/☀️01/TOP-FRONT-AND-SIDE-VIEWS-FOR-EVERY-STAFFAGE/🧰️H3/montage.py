import sys, os, glob
from PIL import Image, ImageDraw
PV = r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\H3\pv"
for d in sys.argv[1:]:
    files = sorted(glob.glob(os.path.join(PV, d, "*.png")))
    if not files: print("none", d); continue
    ims = [Image.open(f).convert("RGB") for f in files]
    cols = 2
    cw = max(i.width for i in ims); ch = max(i.height for i in ims)
    rows = (len(ims) + cols - 1) // cols
    for part in range(0, rows, 4):
        sub = ims[part * cols:(part + 4) * cols]
        names = files[part * cols:(part + 4) * cols]
        r = (len(sub) + cols - 1) // cols
        sheet = Image.new("RGB", (cw * cols, (ch + 14) * r), "white")
        dr = ImageDraw.Draw(sheet)
        for k, im in enumerate(sub):
            x, y = (k % cols) * cw, (k // cols) * (ch + 14)
            dr.text((x + 2, y + 1), os.path.basename(names[k]).split("--")[1][:40], fill="black")
            sheet.paste(im, (x, y + 14))
        out = os.path.join(PV, f"{d}-m{part // 4}.png")
        sheet.save(out); print(out, sheet.size)
