"""🔤 Prints a PNG (or a crop `x y w h` of it) as character art, because the image reader of the host did not answer: `python3 rework-domestic-cat-ascii.py <png> <columns> [x y w h]`."""
import os
import struct
import subprocess
import sys
import tempfile


def main():
    png, columns = sys.argv[1], int(sys.argv[2])
    crop = [int(value) for value in sys.argv[3:7]]
    folder = tempfile.mkdtemp()
    cut = os.path.join(folder, "cut.png")
    bmp = os.path.join(folder, "cut.bmp")
    if crop:
        subprocess.run(["sips", "-c", str(crop[3]), str(crop[2]), "--cropOffset", str(crop[1]), str(crop[0]), png, "--out", cut], check=True, capture_output=True)
    else:
        cut = png
    size = subprocess.run(["sips", "-g", "pixelWidth", "-g", "pixelHeight", cut], check=True, capture_output=True, text=True).stdout.split()
    width, height = int(size[size.index("pixelWidth:") + 1]), int(size[size.index("pixelHeight:") + 1])
    rows = max(1, round(columns * height / width / 2))
    subprocess.run(["sips", "-s", "format", "bmp", "-z", str(rows), str(columns), cut, "--out", bmp], check=True, capture_output=True)
    data = open(bmp, "rb").read()
    offset, w, h, bits = struct.unpack_from("<I", data, 10)[0], struct.unpack_from("<i", data, 18)[0], struct.unpack_from("<i", data, 22)[0], struct.unpack_from("<H", data, 28)[0]
    step = bits // 8
    stride = (w * step + 3) // 4 * 4
    ramp = " .:-=+*#%@"
    grid = []
    for row in range(abs(h)):
        line = data[offset + row * stride : offset + row * stride + w * step]
        grid.append([(line[i * step] + line[i * step + 1] + line[i * step + 2]) / 3 for i in range(w)])
    if h > 0:
        grid.reverse()
    flat = sorted(round(value) for line in grid for value in line)
    ground = max(set(flat), key=flat.count)
    for line in grid:
        print("".join(ramp[min(9, int(abs(value - ground) / 256 * 30))] for value in line).rstrip())


if __name__ == "__main__":
    main()
