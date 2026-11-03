import sys, glob, os
import fitz

src, out, pattern = sys.argv[1], sys.argv[2], sys.argv[3]
zoom = float(sys.argv[4]) if len(sys.argv) > 4 else 6.0
frac = float(sys.argv[5]) if len(sys.argv) > 5 else 0.5
cols = int(sys.argv[6]) if len(sys.argv) > 6 else 2
files = sorted(glob.glob(os.path.join(src, pattern)))
pix = []
for f in files:
    page = fitz.open(f)[0]
    r = page.rect
    clip = fitz.Rect(r.x0, r.y0, r.x1, r.y0 + r.height * frac)
    p = page.get_pixmap(matrix=fitz.Matrix(zoom, zoom), clip=clip, alpha=False)
    pix.append((os.path.basename(f), p))
cw = max(p.width for _, p in pix) + 10
ch = max(p.height for _, p in pix) + 24
rows = (len(pix) + cols - 1) // cols
sheet = fitz.open()
page = sheet.new_page(width=cols * cw, height=rows * ch)
for i, (name, p) in enumerate(pix):
    x, y = (i % cols) * cw + 5, (i // cols) * ch + 18
    page.insert_text((x, y - 5), name.replace(".svg", "")[:70], fontsize=10)
    page.insert_image(fitz.Rect(x, y, x + p.width, y + p.height), pixmap=p)
page.get_pixmap(alpha=False).save(out)
print(out, len(pix))
