import sys, subprocess, glob, os
from PIL import Image, ImageDraw
video, out, step = sys.argv[1], sys.argv[2], float(sys.argv[3])
cols = int(sys.argv[4]) if len(sys.argv) > 4 else 3
t0 = float(sys.argv[5]) if len(sys.argv) > 5 else 0.0
t1 = float(sys.argv[6]) if len(sys.argv) > 6 else 1e9
tmp = out + "_frames"
os.makedirs(tmp, exist_ok=True)
for f in glob.glob(tmp + "/*.png"): os.remove(f)
subprocess.run(["ffmpeg", "-loglevel", "error", "-i", video, "-vf", f"fps={1/step}", tmp + "/f%04d.png"], check=True)
files = sorted(glob.glob(tmp + "/*.png"))
files = [(i * step, f) for i, f in enumerate(files) if t0 <= i * step <= t1]
per = cols * 4
for s in range(0, len(files), per):
    chunk = files[s:s + per]
    ims = [Image.open(f) for _, f in chunk]
    w, h = ims[0].size
    rows = (len(ims) + cols - 1) // cols
    sheet = Image.new("RGB", (w * cols, h * rows))
    for k, ((t, _), im) in enumerate(zip(chunk, ims)):
        d = ImageDraw.Draw(im); d.text((6, 4), f"{t:.1f}s", fill=(255, 255, 0))
        sheet.paste(im, ((k % cols) * w, (k // cols) * h))
    sheet.save(f"{out}_{s // per:02d}.png")
print(len(files))
