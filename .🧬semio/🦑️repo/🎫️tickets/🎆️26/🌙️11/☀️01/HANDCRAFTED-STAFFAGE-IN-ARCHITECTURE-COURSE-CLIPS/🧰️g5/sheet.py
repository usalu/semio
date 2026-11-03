"""python sheet.py in.mp4 out.png t0 t1 step cols : contact sheet of frames"""
import sys, av
from PIL import Image, ImageDraw
src, out, t0, t1, step, cols = sys.argv[1], sys.argv[2], float(sys.argv[3]), float(sys.argv[4]), float(sys.argv[5]), int(sys.argv[6])
with av.open(src) as c:
    s = c.streams.video[0]; rate = float(s.average_rate)
    print("duration", float(s.frames) / rate)
    ts = []; t = t0
    while t <= t1: ts.append(t); t += step
    want = {round(t * rate): t for t in ts}; got = []
    for i, f in enumerate(c.decode(s)):
        if i in want: got.append((want[i], f.to_image()))
w, h = got[0][1].size; sc = 0.5
tw, th = int(w * sc), int(h * sc); rows = (len(got) + cols - 1) // cols
sheet = Image.new("RGB", (tw * cols, th * rows))
for k, (t, im) in enumerate(got):
    im = im.resize((tw, th)); ImageDraw.Draw(im).text((4, 4), f"{t:g}s", fill="white")
    sheet.paste(im, ((k % cols) * tw, (k // cols) * th))
sheet.save(out)
