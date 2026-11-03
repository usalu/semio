import sys, os, glob
sys.stdout.reconfigure(encoding="utf8")
from multiprocessing import Pool
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
D = T + "/🗑️generated/C2/qa14/"
R = T + "/🗑️generated/C2/qa14-r/"


def job(f):
    import pymupdf
    from PIL import Image, ImageChops
    out = R + os.path.relpath(f, D).replace("\\", "/").replace(".svg", ".png")
    if os.path.exists(out):
        return
    os.makedirs(os.path.dirname(out), exist_ok=True)
    pm = pymupdf.open(f)[0].get_pixmap(dpi=int(sys.argv[1]) if len(sys.argv) > 1 else 110, alpha=False)
    im = Image.frombytes("RGB", (pm.width, pm.height), pm.samples)
    bb = ImageChops.difference(im, Image.new("RGB", im.size, "white")).getbbox()
    if bb:
        im = im.crop((max(bb[0] - 4, 0), max(bb[1] - 4, 0), bb[2] + 4, bb[3] + 4))
    im.save(out)


if __name__ == "__main__":
    fs = sorted(glob.glob(D + "*/*.svg"))
    with Pool(12) as p:
        p.map(job, fs, chunksize=2)
    print("RASDONE", len(fs))
