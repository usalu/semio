"""🔎️ Prints a PNG render as character art (the image viewer of the host is unreachable): rework-pct-ascii.py <png> [cols] [x0,y0,x1,y1 as fractions]"""
import subprocess, sys, struct, tempfile, os

RAMP = "@%#*+=-:. "


def load(path):
    out = os.path.join(tempfile.mkdtemp(), "a.bmp")
    subprocess.run(["sips", "-s", "format", "bmp", path, "--out", out], check=True, capture_output=True)
    data = open(out, "rb").read()
    offset = struct.unpack_from("<I", data, 10)[0]
    width, height = struct.unpack_from("<ii", data, 18)
    bits = struct.unpack_from("<H", data, 28)[0]
    step = bits // 8
    stride = (width * step + 3) // 4 * 4
    flip = height > 0
    height = abs(height)
    rows = [data[offset + r * stride: offset + r * stride + width * step] for r in range(height)]
    if flip:
        rows.reverse()
    return width, height, step, rows


def main():
    path = sys.argv[1]
    cols = int(sys.argv[2]) if len(sys.argv) > 2 else 160
    box = [float(v) for v in sys.argv[3].split(",")] if len(sys.argv) > 3 else [0, 0, 1, 1]
    width, height, step, rows = load(path)
    x0, y0, x1, y1 = int(box[0] * width), int(box[1] * height), int(box[2] * width), int(box[3] * height)
    cell = max(1, (x1 - x0) // cols)
    first = rows[y0 + 6]
    ground = (first[(x0 + 6) * step] + first[(x0 + 6) * step + 1] * 4 + first[(x0 + 6) * step + 2] * 3) // 8
    lines = []
    for top in range(y0, y1 - 2 * cell + 1, 2 * cell):
        line = []
        for left in range(x0, x1 - cell + 1, cell):
            total = count = 0
            low = 255
            for r in range(top, top + 2 * cell, max(1, cell // 3)):
                row = rows[r]
                for c in range(left, left + cell, max(1, cell // 3)):
                    p = c * step
                    lum = (row[p] + row[p + 1] * 4 + row[p + 2] * 3) // 8
                    total += lum
                    count += 1
                    low = min(low, lum)
            mean = total // count
            value = (mean + low) // 2
            line.append(" " if abs(mean - ground) < 10 and abs(low - ground) < 28 else RAMP[min(len(RAMP) - 2, value * (len(RAMP) - 1) // 256)])
        lines.append("".join(line).rstrip())
    while lines and not lines[0]:
        lines.pop(0)
    while lines and not lines[-1]:
        lines.pop()
    print("\n".join(lines))


main()
