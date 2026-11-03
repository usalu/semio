import sys, glob, os, av
from PIL import Image

root, lang = sys.argv[1], sys.argv[2]
for scene in sys.argv[3:]:
    hits = glob.glob(f"{root}/media-{lang}/videos/**/{scene}.mp4", recursive=True)
    if not hits:
        print("missing", scene)
        continue
    c = av.open(hits[0])
    frames = [f.to_image() for f in c.decode(video=0)]
    n = len(frames)
    idx = [2, n // 2, max(0, n - 20)]
    w, h = frames[0].size
    sheet = Image.new("RGB", (w, h * 3))
    for k, i in enumerate(idx):
        sheet.paste(frames[i], (0, h * k))
    out = f"{root}/{lang}-{scene}.png"
    sheet.save(out)
    print(scene, n, idx, out)
