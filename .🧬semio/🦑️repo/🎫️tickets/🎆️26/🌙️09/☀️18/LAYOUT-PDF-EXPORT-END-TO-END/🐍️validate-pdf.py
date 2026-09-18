"""📕️ Validates an exported layout PDF the way a reader would: opens it with PyMuPDF, reports page count
and media boxes, renders every page to PNG next to the file and measures the ink (non-white pixels) per
page plus in the regions the demo document paints (rect frame, inherited stroke, text, image placeholder).
Usage: python3 🐍️validate-pdf.py <file.pdf>
"""
import json, sys
import fitz

path = sys.argv[1]
doc = fitz.open(path)
report = {"path": path, "pages": doc.page_count, "boxes": [], "ink": [], "fonts": [], "regions": []}
for index, page in enumerate(doc):
    report["boxes"].append([round(v, 1) for v in page.rect])
    report["fonts"].append([f[3] for f in page.get_fonts()])
    pix = page.get_pixmap(dpi=72, alpha=False)
    out = path.rsplit(".", 1)[0] + f"-page{index + 1}.png"
    pix.save(out)
    samples = pix.samples
    width, height, n = pix.width, pix.height, pix.n
    def ink(x0, y0, x1, y1):
        count = 0
        for y in range(max(0, int(y0)), min(height, int(y1))):
            row = y * width * n
            for x in range(max(0, int(x0)), min(width, int(x1))):
                p = row + x * n
                if samples[p] < 240 or samples[p + 1] < 240 or samples[p + 2] < 240:
                    count += 1
        return count
    report["ink"].append(ink(0, 0, width, height))
    if index == 0:
        report["regions"].append({
            "text-frame(156,220,80x40)": ink(156, 220, 236, 260),
            "inherited-stroke(50,50,100x80)": ink(48, 48, 152, 132),
            "image-placeholder(136,435,60x40)": ink(136, 435, 196, 475),
            "empty-corner(300,300,80x80)": ink(300, 300, 380, 380),
            "added-frame(48,48,120x64)": ink(48, 48, 168, 112),
        })
print(json.dumps(report))
