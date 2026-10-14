import sys, subprocess, glob, os
from PIL import Image, ImageDraw
video, out, step = sys.argv[1], sys.argv[2], float(sys.argv[3])
cols = int(sys.argv[4]) if len(sys.argv) > 4 else 3
tmp = os.path.dirname(out) + "/ch6_frames"
os.makedirs(tmp, exist_ok=True)
for f in glob.glob(tmp + "/*.png"): os.remove(f)
subprocess.run(["ffmpeg", "-loglevel", "error", "-i", video, "-vf", f"fps=1/{step}", tmp + "/f_%03d.png"], check=True)
files = sorted(glob.glob(tmp + "/*.png"))
ims = [Image.open(f) for f in files]
w, h = ims[0].size
per = cols * 4
for k in range(0, len(ims), per):
    chunk = ims[k:k+per]
    rows = (len(chunk) + cols - 1) // cols
    sheet = Image.new("RGB", (w * cols, h * rows), "black")
    for i, im in enumerate(chunk):
        d = ImageDraw.Draw(im); d.text((6, 6), f"t={(k+i)*step:.1f}s", fill="yellow")
        sheet.paste(im, ((i % cols) * w, (i // cols) * h))
    sheet.save(f"{out}_{k//per}.png")
print(len(ims), "frames", w, h)
