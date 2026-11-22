"""🔍 Red squirrel: character raster of a rendered sheet (the image viewer is unreachable), cropped to the drawing.

Usage: python3 rework-red-squirrel-look.py <png> [columns] [x0,y0,x1,y1 as fractions]
"""
import struct
import subprocess
import sys

RAMP = "@%#*+=-:. "


def pixels(path):
    bmp = "/private/tmp/claude-501/-Users-ueli-Documents-semio/1f9c8bb1-0e43-4972-9e08-b667c0c1c1c8/scratchpad/look.bmp"
    subprocess.run(["sips", "-s", "format", "bmp", path, "--out", bmp], check=True, capture_output=True)
    data = open(bmp, "rb").read()
    offset, = struct.unpack_from("<I", data, 10)
    width, height = struct.unpack_from("<ii", data, 18)
    bits, = struct.unpack_from("<H", data, 28)
    step = bits // 8
    row = (width * step + 3) // 4 * 4
    flip = height > 0
    height = abs(height)
    return width, height, lambda x, y: data[offset + ((height - 1 - y) if flip else y) * row + x * step: offset + ((height - 1 - y) if flip else y) * row + x * step + 3]


def mark(b, g, r):
    top, low = max(r, g, b), min(r, g, b)
    if top - low < 70:
        return None
    if b == top and r < 150:
        return "B"
    if r == top and b == low and g > 110:
        return "Y" if g > 200 else "O"
    if g == top:
        return "G"
    if r > 180 and b > 150:
        return "M"
    if r == top:
        return "R"
    return None


def main():
    width, height, at = pixels(sys.argv[1])
    columns = int(sys.argv[2]) if len(sys.argv) > 2 else 120
    box = [float(v) for v in sys.argv[3].split(",")] if len(sys.argv) > 3 else [0, 0.04, 1, 0.97]
    x0, y0, x1, y1 = int(box[0] * width), int(box[1] * height), int(box[2] * width), int(box[3] * height)
    if len(sys.argv) <= 3:
        xs, ys = [], []
        for y in range(y0, y1, 4):
            for x in range(x0, x1, 4):
                b, g, r = at(x, y)
                if r + g + b < 600:
                    xs.append(x)
                    ys.append(y)
        x0, x1, y0, y1 = max(0, min(xs) - 8), min(width, max(xs) + 8), max(0, min(ys) - 8), min(height, max(ys) + 8)
    cell = (x1 - x0) / columns
    rows = int((y1 - y0) / (2 * cell))
    print(f"[DEBUG] {width}x{height} crop {x0},{y0} {x1},{y1} cell {cell:.1f}px")
    for j in range(rows):
        line = ""
        for i in range(columns):
            total, count, darkest, marks = 0, 0, 255, {}
            for y in range(int(y0 + j * 2 * cell), int(y0 + (j + 1) * 2 * cell), max(1, int(cell / 3))):
                for x in range(int(x0 + i * cell), int(x0 + (i + 1) * cell), max(1, int(cell / 4))):
                    b, g, r = at(x, y)
                    lum = (r * 3 + g * 6 + b) // 10
                    total += lum
                    count += 1
                    darkest = min(darkest, lum)
                    m = mark(b, g, r)
                    if m:
                        marks[m] = marks.get(m, 0) + 1
            if marks and max(marks.values()) * 5 >= count:
                line += max(marks, key=marks.get)
            else:
                value = (total / count * 0.6 + darkest * 0.4)
                line += RAMP[min(9, int(value / 25.6))]
        print(line.rstrip())


main()
