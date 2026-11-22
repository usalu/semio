"""🖼️ Renders several r3 sheets one after another and tiles them into one PNG under `🗑️generated/rework-15/`.

Usage: python3 rework-15-sheets.py <name> <columns> <tile width px> "<r3-sheet arguments>" ["<r3-sheet arguments>" …]
"""
import glob
import os
import shlex
import struct
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "rework-15")
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]


def bmp(path):
    data = open(path, "rb").read()
    offset, = struct.unpack_from("<I", data, 10)
    width, height, _, bits = struct.unpack_from("<iiHH", data, 18)
    step, row = bits // 8, ((bits * width + 31) // 32) * 4
    rows = []
    for y in range(abs(height)):
        line = data[offset + y * row: offset + y * row + width * step]
        rows.append(line if step == 3 else b"".join(line[i:i + 3] for i in range(0, len(line), 4)))
    return width, abs(height), rows if height < 0 else rows[::-1]


def main():
    name, columns, width = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    os.makedirs(OUT, exist_ok=True)
    tiles = []
    for index, arguments in enumerate(sys.argv[4:]):
        made = subprocess.run([BUN, "r3-sheet.ts"] + shlex.split(arguments), cwd=T, capture_output=True, text=True)
        path = made.stdout.strip().splitlines()[-1] if made.stdout.strip() else ""
        if not os.path.isfile(path):
            print("FAILED", arguments, made.stderr[-400:])
            continue
        if "NOT DRAWN" in made.stderr:
            print(made.stderr[-600:])
        tile = os.path.join(OUT, "%s-%d.bmp" % (name, index))
        subprocess.run(["sips", "-Z", str(width), "-s", "format", "bmp", path, "--out", tile], capture_output=True)
        os.remove(path)
        tiles.append(bmp(tile))
        os.remove(tile)
    if not tiles:
        raise SystemExit("nothing rendered")
    w, h = max(t[0] for t in tiles), max(t[1] for t in tiles)
    rows = (len(tiles) + columns - 1) // columns
    white = b"\xff" * (w * 3)
    lines = []
    for r in range(rows):
        for y in range(h):
            line = b""
            for c in range(columns):
                k = r * columns + c
                if k < len(tiles) and y < tiles[k][1]:
                    line += tiles[k][2][y] + b"\xff" * ((w - tiles[k][0]) * 3)
                else:
                    line += white
            lines.append(line)
    W, H = w * columns, h * rows
    pad = b"\0" * ((4 - W * 3 % 4) % 4)
    body = b"".join(line + pad for line in reversed(lines))
    raw = os.path.join(OUT, name + ".bmp")
    with open(raw, "wb") as f:
        f.write(b"BM" + struct.pack("<IHHI", 54 + len(body), 0, 0, 54) + struct.pack("<IiiHHIIiiII", 40, W, H, 1, 24, 0, len(body), 2835, 2835, 0, 0) + body)
    out = os.path.join(OUT, name + ".png")
    subprocess.run(["sips", "-s", "format", "png", raw, "--out", out], capture_output=True)
    os.remove(raw)
    print(out)


if __name__ == "__main__":
    main()
