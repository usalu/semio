import struct, zlib, sys


def readpng(p):
    d = open(p, "rb").read()
    i = 8
    idat = b""
    while i < len(d):
        n = struct.unpack(">I", d[i : i + 4])[0]
        t = d[i + 4 : i + 8]
        c = d[i + 8 : i + 8 + n]
        i += 12 + n
        if t == b"IHDR":
            w, h, bd, ct = struct.unpack(">IIBB", c[:10])
        if t == b"IDAT":
            idat += c
    raw = zlib.decompress(idat)
    bpp = {2: 3, 6: 4, 0: 1}[ct]
    stride = w * bpp
    rows = []
    prev = bytearray(stride)
    pos = 0
    for y in range(h):
        f = raw[pos]
        line = bytearray(raw[pos + 1 : pos + 1 + stride])
        pos += 1 + stride
        for x in range(stride):
            a = line[x - bpp] if x >= bpp else 0
            b = prev[x]
            c2 = prev[x - bpp] if x >= bpp else 0
            if f == 1:
                line[x] = (line[x] + a) & 255
            elif f == 2:
                line[x] = (line[x] + b) & 255
            elif f == 3:
                line[x] = (line[x] + (a + b) // 2) & 255
            elif f == 4:
                pa, pb, pc = abs(b - c2), abs(a - c2), abs(a + b - 2 * c2)
                pr = a if (pa <= pb and pa <= pc) else (b if pb <= pc else c2)
                line[x] = (line[x] + pr) & 255
        rows.append(line)
        prev = line
    return w, h, bpp, rows


def writepng(p, w, h, bpp, rows):
    raw = b"".join(b"\x00" + bytes(r) for r in rows)

    def ch(t, c):
        return struct.pack(">I", len(c)) + t + c + struct.pack(">I", zlib.crc32(t + c) & 0xFFFFFFFF)

    open(p, "wb").write(b"\x89PNG\r\n\x1a\n" + ch(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, {3: 2, 4: 6, 1: 0}[bpp], 0, 0, 0)) + ch(b"IDAT", zlib.compress(raw)) + ch(b"IEND", b""))


if __name__ == "__main__":
    src, dst, x0, y0, x1, y1, k = sys.argv[1], sys.argv[2], *map(int, sys.argv[3:8])
    w, h, bpp, rows = readpng(src)
    out = []
    for y in range(y0, y1):
        line = bytearray()
        for x in range(x0, x1):
            line += rows[y][x * bpp : (x + 1) * bpp] * k
        for _ in range(k):
            out.append(line)
    writepng(dst, (x1 - x0) * k, (y1 - y0) * k, bpp, out)
