"""🖼️ Tiles up to four tool PNGs, each at half size, into one PNG so several poses can be read at once.

Usage: python3 rework-dairy-cow-montage.py <out.png> <in.png> [<in.png> ...]
"""
import os
import struct
import subprocess
import sys
import tempfile
import zlib


def bitmap(path):
    handle = tempfile.NamedTemporaryFile(suffix=".bmp", delete=False)
    handle.close()
    subprocess.run(["sips", "-s", "format", "bmp", path, "--out", handle.name], check=True, capture_output=True)
    data = open(handle.name, "rb").read()
    os.unlink(handle.name)
    offset, = struct.unpack_from("<I", data, 10)
    width, height = struct.unpack_from("<ii", data, 18)
    bits, = struct.unpack_from("<H", data, 28)
    step = bits // 8
    stride = (width * step + 3) & ~3
    rows = []
    flip = height > 0
    for y in range(0, abs(height), 2):
        line = abs(height) - 1 - y if flip else y
        row = data[offset + line * stride: offset + line * stride + width * step]
        rows.append(bytes(row[2::step * 2]) + b"" if False else b"".join((row[2::step * 2], row[1::step * 2], row[0::step * 2])))
    return width // 2, abs(height) // 2, rows


def main():
    out, sources = sys.argv[1], sys.argv[2:]
    tiles = [bitmap(path) for path in sources]
    width, height = tiles[0][0], tiles[0][1]
    columns = 1 if len(tiles) == 1 else 2
    lines = (len(tiles) + columns - 1) // columns
    raw = bytearray()
    for line in range(lines):
        for y in range(height):
            raw.append(0)
            for column in range(columns):
                index = line * columns + column
                if index < len(tiles) and y < tiles[index][1]:
                    planes = tiles[index][2][y]
                    n = len(planes) // 3
                    pixel = bytearray(n * 3)
                    pixel[0::3], pixel[1::3], pixel[2::3] = planes[:n], planes[n:2 * n], planes[2 * n:]
                    raw += pixel[:width * 3].ljust(width * 3, b"\xff")
                else:
                    raw += b"\xff" * (width * 3)

    def chunk(kind, body):
        return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body))

    png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width * columns, height * lines, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(bytes(raw), 6)) + chunk(b"IEND", b"")
    open(out, "wb").write(png)
    print(out)


main()
