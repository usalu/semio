import subprocess, sys, shutil
from pathlib import Path
from PIL import Image, ImageDraw
beat, fps, cols, per = sys.argv[1], float(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
root = Path(__file__).resolve().parent
video = root / "ch7_media/videos/scene_7/480p15" / f"{beat}.mp4"
frames = root / f"ch7_frames_{beat}"
shutil.rmtree(frames, ignore_errors=True); frames.mkdir()
subprocess.run(["ffmpeg","-loglevel","error","-i",str(video),"-vf",f"fps={fps}",str(frames/"f_%04d.png")],check=True)
files = sorted(frames.glob("f_*.png"))
print(len(files), "frames")
(root/"sheets").mkdir(exist_ok=True)
for s in range(0, len(files), per):
    chunk = files[s:s+per]
    im0 = Image.open(chunk[0]); w,h = im0.size
    rows = -(-len(chunk)//cols)
    sheet = Image.new("RGB",(cols*w, rows*h))
    for i,f in enumerate(chunk):
        im = Image.open(f).convert("RGB")
        d = ImageDraw.Draw(im); d.text((6,4), f"t={(s+i)/fps:.1f}s", fill=(255,255,0))
        sheet.paste(im,((i%cols)*w,(i//cols)*h))
    out = root/"sheets"/f"ch7_{beat}_{s//per:02d}.png"; sheet.save(out); print(out)
