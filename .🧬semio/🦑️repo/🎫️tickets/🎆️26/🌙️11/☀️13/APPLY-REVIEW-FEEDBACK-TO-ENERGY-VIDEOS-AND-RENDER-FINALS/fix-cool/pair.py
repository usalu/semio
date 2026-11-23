import sys, glob
from PIL import Image
d = sys.argv[1]
fs = sorted(glob.glob(d + "/ready_*/g_*[0-9].png"))
W, H = 800, 450
im = Image.new("RGB", (W * 2, H * ((len(fs) + 1) // 2)))
for k, f in enumerate(fs):
    im.paste(Image.open(f).convert("RGB").resize((W, H)), ((k % 2) * W, (k // 2) * H))
im.save(d + "/ready.png")
print(fs)
