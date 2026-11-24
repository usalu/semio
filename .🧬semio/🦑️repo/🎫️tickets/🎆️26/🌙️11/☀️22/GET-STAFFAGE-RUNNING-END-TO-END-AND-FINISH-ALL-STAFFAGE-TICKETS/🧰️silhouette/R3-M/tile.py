"""🧩️ Tiles PNG rasters into one picture without any package (pure zlib), nearest-neighbour scaled to a tile size: python3 tile.py <out.png> <columns> <tile px> <png>…"""
import struct, sys, zlib


def read(path):
    """📥️ Rows of RGB triples of an 8-bit RGB PNG without interlace and with filter 0 rows (as png.ts writes them)."""
    data = open(path, "rb").read()
    pos, width, height, idat = 8, 0, 0, b""
    while pos < len(data):
        (length,), kind = struct.unpack(">I", data[pos:pos + 4]), data[pos + 4:pos + 8]
        body = data[pos + 8:pos + 8 + length]
        if kind == b"IHDR":
            width, height = struct.unpack(">II", body[:8])
        if kind == b"IDAT":
            idat += body
        pos += 12 + length
    raw = zlib.decompress(idat)
    return width, height, [raw[y * (width * 3 + 1) + 1:(y + 1) * (width * 3 + 1)] for y in range(height)]


def chunk(kind, body):
    """📦️ One PNG chunk."""
    return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body) & 0xFFFFFFFF)


out, cols, size, files = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4:]
rows_n = -(-len(files) // cols)
W, H = cols * (size + 4), rows_n * (size + 4)
canvas = [bytearray(b"\x00\xaa\x00" * W) for _ in range(H)]
for i, f in enumerate(files):
    w, h, rows = read(f)
    ox, oy = (i % cols) * (size + 4), (i // cols) * (size + 4)
    for y in range(size):
        src = rows[min(h - 1, y * h // size)]
        line = bytearray()
        for x in range(size):
            k = min(w - 1, x * w // size) * 3
            line += src[k:k + 3]
        canvas[oy + y][ox * 3:(ox + size) * 3] = line
raw = b"".join(b"\x00" + bytes(r) for r in canvas)
open(out, "wb").write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", W, H, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b""))
print(out)
