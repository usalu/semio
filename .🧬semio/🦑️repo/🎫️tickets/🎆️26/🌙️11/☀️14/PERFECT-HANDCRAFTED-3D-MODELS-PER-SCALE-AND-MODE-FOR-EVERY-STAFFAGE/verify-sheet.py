"""🖼️ Composes the verification PNGs of one asset into contact sheets (rows scales, columns body/thermal/structure; rows scales, columns two clip instants), at full or half size. usage: python3 verify-sheet.py <shots dir> <asset> [half]"""
import glob, os, struct, sys, zlib

def read(path):
    data = open(path, "rb").read()
    pos, idat = 8, b""
    while pos < len(data):
        n, t = struct.unpack(">I4s", data[pos:pos + 8])
        body = data[pos + 8:pos + 8 + n]
        if t == b"IHDR": w, h = struct.unpack(">II", body[:8])
        if t == b"IDAT": idat += body
        pos += 12 + n
    raw = zlib.decompress(idat)
    rows = [raw[y * (w * 4 + 1) + 1:(y + 1) * (w * 4 + 1)] for y in range(h)]
    return w, h, rows

def write(path, w, h, rows):
    raw = b"".join(b"\0" + r for r in rows)
    def chunk(t, d): return struct.pack(">I", len(d)) + t + d + struct.pack(">I", zlib.crc32(t + d) & 0xffffffff)
    open(path, "wb").write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b""))

def half(w, h, rows):
    out = []
    for y in range(0, h - 1, 2):
        a, b = rows[y], rows[y + 1]
        out.append(bytes(((a[x] + a[x + 4] + b[x] + b[x + 4]) // 4) for x in range(0, w * 4 - 4, 8) for x in [x, x + 1, x + 2, x + 3]))
    return w // 2, h // 2, out

root, asset = sys.argv[1], sys.argv[2]
shrink = len(sys.argv) > 3
def sheet(name, grid):
    tiles = [[read(p) if os.path.exists(p) else None for p in row] for row in grid]
    tiles = [[half(*t) if (t and shrink) else t for t in row] for row in tiles]
    tw, th = next(t for row in tiles for t in row if t)[:2]
    rows = []
    for row in tiles:
        for y in range(th):
            line = bytearray()
            for t in row: line += (t[2][y] if t else b"\xcc\xcc\xcc\xff" * tw)
            rows.append(bytes(line))
        rows.append(b"\x00\x00\x00\xff" * (tw * len(row)))
    write(os.path.join(root, asset, name), tw * len(grid[0]), len(rows), rows)
d = os.path.join(root, asset)
scales = sorted({int(f.split(".s")[1].split(".")[0]) for f in glob.glob(f"{d}/{asset}.s*.body.png")})
sheet(f"{asset}.sheet-modes.png", [[f"{d}/{asset}.s{s}.{m}.png" for m in ("body", "thermal", "structure")] for s in scales])
clip = [f for f in glob.glob(f"{d}/{asset}.clip-*.png")]
if clip:
    c = os.path.basename(clip[0]).split(".")[1]
    sheet(f"{asset}.sheet-clip.png", [[f"{d}/{asset}.{c}.s{s}.t{t}.png" for t in (0, 50)] for s in scales])
