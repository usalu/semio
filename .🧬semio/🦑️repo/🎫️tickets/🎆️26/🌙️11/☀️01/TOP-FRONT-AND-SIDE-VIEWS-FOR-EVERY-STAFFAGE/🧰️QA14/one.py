import sys, os, pymupdf
from PIL import Image
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
f = sys.argv[1]; dpi = int(sys.argv[2]); out = T + "/🗑️generated/C2/qa14-m/one-" + os.path.basename(f).replace(".svg", ".png")
pm = pymupdf.open(f)[0].get_pixmap(dpi=dpi, alpha=False)
im = Image.frombytes("RGB", (pm.width, pm.height), pm.samples)
if len(sys.argv) > 3:
    x0, y0, x1, y1 = [float(v) for v in sys.argv[3:7]]; W, H = im.size
    im = im.crop((int(x0*W), int(y0*H), int(x1*W), int(y1*H)))
im.save(out); print(out, im.size)
