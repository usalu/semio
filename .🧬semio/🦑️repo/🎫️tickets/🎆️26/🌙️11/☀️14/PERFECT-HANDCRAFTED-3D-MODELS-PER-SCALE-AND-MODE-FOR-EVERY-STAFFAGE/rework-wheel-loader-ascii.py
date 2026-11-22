"""🔡 Prints a PNG sheet as character art (dark = dense) because the image reader of the session was unreachable; sips converts to BMP."""
import struct, subprocess, sys, tempfile, os
path, cols = sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 200
box = [int(v) for v in sys.argv[3].split(",")] if len(sys.argv) > 3 else None
bmp = os.path.join(tempfile.mkdtemp(), "a.bmp")
subprocess.run(["sips", "-s", "format", "bmp", path, "--out", bmp], check=True, capture_output=True)
d = open(bmp, "rb").read()
off, = struct.unpack_from("<I", d, 10)
w, h, _, bpp = struct.unpack_from("<iiHH", d, 18)
flip = h > 0
h = abs(h)
n = bpp // 8
stride = (w * n + 3) // 4 * 4
def lum(x, y):
    r = (h - 1 - y) if flip else y
    i = off + r * stride + x * n
    return (d[i] + d[i + 1] + d[i + 2]) / 3
x0, y0, x1, y1 = box or (0, 0, w, h)
cw = (x1 - x0) / cols
ch = cw * 2
ramp = " .:-=+*#%@"
y = y0
while y + ch <= y1:
    line = ""
    for c in range(cols):
        xs = range(int(x0 + c * cw), int(x0 + (c + 1) * cw), max(1, int(cw // 3)))
        ys = range(int(y), int(y + ch), max(1, int(ch // 4)))
        v = min(lum(a, b) for a in xs for b in ys) * 0.5 + sum(lum(a, b) for a in xs for b in ys) / (len(xs) * len(ys)) * 0.5
        line += ramp[min(9, int((255 - v) / 25.6))]
    if line.strip(): print(line.rstrip())
    y += ch
