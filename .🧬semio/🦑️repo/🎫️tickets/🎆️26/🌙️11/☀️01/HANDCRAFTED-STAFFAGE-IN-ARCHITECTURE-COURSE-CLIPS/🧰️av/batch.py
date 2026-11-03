import sys, glob, os
import pymupdf as fitz

src = sys.argv[1]
assets = sys.argv[2].split(",")
zoom = float(sys.argv[3])
frac = float(sys.argv[4])
suffix = sys.argv[5] if len(sys.argv) > 5 else ""
vp = sys.argv[6] if len(sys.argv) > 6 else "elevation"
for a in assets:
    files = sorted(glob.glob(os.path.join(src, f"{a}--*--{vp}.svg")))
    pix = []
    for f in files:
        page = fitz.open(f)[0]
        r = page.rect
        clip = fitz.Rect(r.x0, r.y0, r.x1, r.y0 + r.height * frac)
        pix.append((os.path.basename(f), page.get_pixmap(matrix=fitz.Matrix(zoom, zoom), clip=clip, alpha=False)))
    if not pix:
        continue
    cw = max(p.width for _, p in pix) + 10
    ch = max(p.height for _, p in pix) + 24
    sheet = fitz.open()
    page = sheet.new_page(width=2 * cw, height=((len(pix) + 1) // 2) * ch)
    for i, (name, p) in enumerate(pix):
        x, y = (i % 2) * cw + 5, (i // 2) * ch + 18
        page.insert_text((x, y - 5), name.replace(".svg", "")[:70], fontsize=10)
        page.insert_image(fitz.Rect(x, y, x + p.width, y + p.height), pixmap=p)
    out = os.path.join(src, f"c-{a}{suffix}.png")
    page.get_pixmap(alpha=False).save(out)
    print(out, len(pix))
