import glob, os, re, sys
from PIL import Image, ImageDraw
root = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "QA3")
out = os.path.join(root, "montage")
os.makedirs(out, exist_ok=True)
W = 760
count = {}
for asset in sorted(d for d in os.listdir(root) if os.path.isdir(os.path.join(root, d)) and d not in ("montage",) and not d.startswith("profile")):
    for scale in (50, 100, 200, 500):
        files = sorted(glob.glob(os.path.join(root, asset, f"*-1-{scale}.png")))
        if not files:
            continue
        per = 14
        for n in range(0, len(files), per):
            chunk = files[n:n + per]
            ims = []
            for f in chunk:
                im = Image.open(f).convert("RGB")
                h = max(1, round(im.height * W / im.width))
                im = im.resize((W, h), Image.LANCZOS)
                ims.append(im)
            rows = [ims[i:i + 2] for i in range(0, len(ims), 2)]
            H = sum(max(i.height for i in r) for r in rows)
            sheet = Image.new("RGB", (2 * W, H), "white")
            y = 0
            for r in rows:
                x = 0
                for i in r:
                    sheet.paste(i, (x, y))
                    x += W
                y += max(i.height for i in r)
            name = f"{asset}-{scale}-{n // per + 1}.png"
            sheet.save(os.path.join(out, name))
            count[name] = [os.path.basename(f).split("--views")[0] for f in chunk]
for k, v in count.items():
    print(k, v)
