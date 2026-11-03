import sys, glob, os
import fitz

src = sys.argv[1]
out = sys.argv[2]
cols = int(sys.argv[3]) if len(sys.argv) > 3 else 4
zoom = float(sys.argv[4]) if len(sys.argv) > 4 else 3.0
pattern = sys.argv[5] if len(sys.argv) > 5 else "*.svg"
files = sorted(glob.glob(os.path.join(src, pattern)))
pix = []
for f in files:
    doc = fitz.open(f)
    page = doc[0]
    p = page.get_pixmap(matrix=fitz.Matrix(zoom, zoom), alpha=False)
    pix.append((os.path.basename(f), p))
cw = max(p.width for _, p in pix) + 10
ch = max(p.height for _, p in pix) + 24
rows = (len(pix) + cols - 1) // cols
sheet = fitz.open()
page = sheet.new_page(width=cols * cw, height=rows * ch)
for i, (name, p) in enumerate(pix):
    x, y = (i % cols) * cw + 5, (i // cols) * ch + 18
    page.insert_text((x, y - 5), name.replace(".svg", "")[:60], fontsize=8)
    page.insert_image(fitz.Rect(x, y, x + p.width, y + p.height), pixmap=p)
sheet.load_page(0).get_pixmap(alpha=False).save(out)
print(out, len(pix))
