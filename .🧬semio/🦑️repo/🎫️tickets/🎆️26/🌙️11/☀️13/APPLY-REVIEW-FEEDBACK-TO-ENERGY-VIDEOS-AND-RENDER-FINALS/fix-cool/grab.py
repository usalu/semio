import subprocess, sys, os
from PIL import Image
ff, video, out = sys.argv[1:4]
os.makedirs(out, exist_ok=True)
for spec in sys.argv[4:]:
    t, _, box = spec.partition("@")
    path = os.path.join(out, f"g_{float(t):08.2f}.png")
    subprocess.run([ff, "-loglevel", "error", "-y", "-ss", t, "-i", video, "-frames:v", "1", path], check=True)
    if box:
        x0, y0, x1, y1 = (int(v) for v in box.split(","))
        Image.open(path).crop((x0, y0, x1, y1)).save(path.replace(".png", "_c.png"))
