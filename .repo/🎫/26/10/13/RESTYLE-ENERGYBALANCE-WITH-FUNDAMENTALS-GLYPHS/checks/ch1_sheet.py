import subprocess, sys, glob, os
from PIL import Image, ImageDraw
beat, fps = sys.argv[1], float(sys.argv[2])
per = int(sys.argv[3]) if len(sys.argv) > 3 else 4
ck = os.path.dirname(os.path.abspath(__file__))
mp4 = f"{ck}/ch1_media/videos/scene_1/480p15/{beat}.mp4"
out = f"{ck}/ch1_frames/{beat}"
os.makedirs(out, exist_ok=True)
for f in glob.glob(out + "/*.png"): os.remove(f)
subprocess.run(["ffmpeg", "-loglevel", "error", "-i", mp4, "-vf", f"fps={fps}", f"{out}/f_%04d.png"], check=True)
files = sorted(glob.glob(out + "/*.png"))
os.makedirs(f"{ck}/sheets", exist_ok=True)
for old in glob.glob(f"{ck}/sheets/ch1_{beat}_*.png"): os.remove(old)
cols = 2
for s in range(0, len(files), per):
    chunk = files[s:s + per]
    ims = [Image.open(f) for f in chunk]
    w, h = ims[0].size
    rows = -(-len(ims) // cols)
    sheet = Image.new("RGB", (w * cols, h * rows))
    for k, (im, f) in enumerate(zip(ims, chunk)):
        sheet.paste(im, ((k % cols) * w, (k // cols) * h))
        d = ImageDraw.Draw(sheet)
        d.text(((k % cols) * w + 6, (k // cols) * h + 4), f"t={(s + k) / fps:.1f}s", fill=(255, 255, 0))
    sheet.save(f"{ck}/sheets/ch1_{beat}_{s // per:02d}.png")
print(len(files), "frames", -(-len(files) // per), "sheets")
