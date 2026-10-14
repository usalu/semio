import sys, subprocess
from PIL import Image
video, t, out = sys.argv[1], sys.argv[2], sys.argv[3]
box = [float(x) for x in sys.argv[4:8]] if len(sys.argv) >= 8 else [0, 0, 1, 1]
subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-ss", t, "-i", video, "-frames:v", "1", out], check=True)
im = Image.open(out); w, h = im.size
im = im.crop((int(w*box[0]), int(h*box[1]), int(w*box[2]), int(h*box[3])))
im = im.resize((im.width * 2, im.height * 2), Image.LANCZOS); im.save(out)
