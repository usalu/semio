"""🔤 Coarse character view of a rendered sheet for sessions whose image reader is unreachable.

Usage: python3 rework-goat-ascii.py <png> [columns] [x0,y0,x1,y1 as fractions of the picture]
Background is blank, black outline `#`, grey seam `+`, shaded surface by brightness `:` `-` `.`, coloured rig `*`.
"""
import sys
import os
import struct
import subprocess
import tempfile

path = sys.argv[1]
columns = int(sys.argv[2]) if len(sys.argv) > 2 else 170
bitmap = os.path.join(tempfile.mkdtemp(), "sheet.bmp")
subprocess.run(["sips", "-s", "format", "bmp", path, "--out", bitmap], check=True, capture_output=True)
with open(bitmap, "rb") as handle:
    data = handle.read()
os.remove(bitmap)
offset, full, tall, depth = struct.unpack_from("<I", data, 10)[0], struct.unpack_from("<i", data, 18)[0], struct.unpack_from("<i", data, 22)[0], struct.unpack_from("<H", data, 28)[0] // 8
stride, flipped, tall = (full * depth + 3) // 4 * 4, tall > 0, abs(tall)
left, top, right, bottom = 0, 0, full, tall
if len(sys.argv) > 3:
    a, b, c, d = (float(v) for v in sys.argv[3].split(","))
    left, top, right, bottom = int(a * full), int(b * tall), int(c * full), int(d * tall)
width, height = right - left, bottom - top


class Pixels:
    def __getitem__(self, at):
        x, y = at[0] + left, at[1] + top
        i = offset + (tall - 1 - y if flipped else y) * stride + x * depth
        return (data[i + 2], data[i + 1], data[i])


cell = width / columns
rows = int(height / (cell * 2))
pixels = Pixels()
background = pixels[5, min(height - 1, 40)]
for r in range(rows):
    line = []
    for c in range(columns):
        dark = seam = fill = colour = total = 0
        light = 0
        for y in range(int(r * cell * 2), int((r + 1) * cell * 2), 2):
            for x in range(int(c * cell), int((c + 1) * cell), 2):
                p = pixels[min(x, width - 1), min(y, height - 1)]
                total += 1
                if max(p) - min(p) > 60:
                    colour += 1
                elif sum(abs(p[i] - background[i]) for i in range(3)) < 24:
                    continue
                elif max(p) < 50:
                    dark += 1
                elif max(p) - min(p) < 7 and max(p) < 150:
                    seam += 1
                else:
                    fill += 1
                    light += sum(p) / 3
        if dark * 6 > total:
            line.append("#")
        elif seam * 8 > total:
            line.append("+")
        elif colour * 3 > total:
            line.append("*")
        elif fill * 2 > total:
            mean = light / fill
            line.append(":" if mean < 90 else "-" if mean < 125 else ".")
        elif fill + dark > 0 and (fill + dark) * 6 > total:
            line.append(",")
        else:
            line.append(" ")
    print("".join(line).rstrip())
