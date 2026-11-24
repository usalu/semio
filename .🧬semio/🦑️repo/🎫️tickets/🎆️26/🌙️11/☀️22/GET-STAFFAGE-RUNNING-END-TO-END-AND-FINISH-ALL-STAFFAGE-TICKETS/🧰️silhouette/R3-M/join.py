"""🧷️ Joins PNGs (8-bit RGB, as where.ts writes them) side by side with a green separator: join.py <out.png> <in.png>…"""
import struct, sys, zlib


def read(path):
    """📥️ Width, height and rows of an RGB PNG."""
    data = open(path, "rb").read()
    w, h = struct.unpack(">II", data[16:24])
    raw, pos = b"", 8
    while pos < len(data):
        n = struct.unpack(">I", data[pos:pos + 4])[0]
        if data[pos + 4:pos + 8] == b"IDAT":
            raw += data[pos + 8:pos + 8 + n]
        pos += 12 + n
    raw = zlib.decompress(raw)
    return w, h, [raw[y * (w * 3 + 1) + 1:(y + 1) * (w * 3 + 1)] for y in range(h)]


images = [read(f) for f in sys.argv[2:]]
height = max(i[1] for i in images)
width = sum(i[0] for i in images) + 10 * (len(images) - 1)
out = b""
for y in range(height):
    row = b""
    for k, (w, h, rows) in enumerate(images):
        row += (rows[y] if y < h else b"\xff" * (w * 3)) + (b"\x00\xff\x00" * 10 if k < len(images) - 1 else b"")
    out += b"\x00" + row
chunk = lambda t, d: struct.pack(">I", len(d)) + t + d + struct.pack(">I", zlib.crc32(t + d) & 0xFFFFFFFF)
open(sys.argv[1], "wb").write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(out)) + chunk(b"IEND", b""))
