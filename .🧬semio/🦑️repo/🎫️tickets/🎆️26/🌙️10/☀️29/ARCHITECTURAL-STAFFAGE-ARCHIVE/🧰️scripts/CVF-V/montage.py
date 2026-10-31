import sys, subprocess, re, os
# usage: montage.py out.png height_px in1.png in2.png ...  (images side by side scaled to common height, white gap)
out, H = sys.argv[1], int(sys.argv[2]); ins = sys.argv[3:]
els = []; x = 10
for p in ins:
    svg = open(p + ".svg").read() if os.path.exists(p + ".svg") else None
    m = re.search(r'width="(\d+)" height="(\d+)"', svg)
    w, h = int(m.group(1)), int(m.group(2))
    s = H / h; nw = int(w * s)
    els.append(f'<image href="file://{os.path.abspath(p)}" x="{x}" y="10" width="{nw}" height="{H}"/>'); x += nw + 20
svg = f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{x}" height="{H+20}"><rect width="100%" height="100%" fill="#eee"/>{"".join(els)}</svg>'
open(out + ".svg", "w").write(svg)
subprocess.run(["rsvg-convert", out + ".svg", "-o", out], check=True)
