import sys, zlib, struct


def read(path):
    d = open(path, "rb").read()
    pos, idat, w = 8, b"", None
    while pos < len(d):
        n, t = struct.unpack(">I4s", d[pos:pos + 8])
        c = d[pos + 8:pos + 8 + n]
        if t == b"IHDR":
            w, h, bd, ct = struct.unpack(">IIBB", c[:10])
        if t == b"IDAT":
            idat += c
        pos += 12 + n
    ch = {2: 3, 6: 4, 0: 1}[ct]
    raw = zlib.decompress(idat)
    stride = w * ch
    rows, prev = [], bytearray(stride)
    p = 0
    for y in range(h):
        f = raw[p]
        line = bytearray(raw[p + 1:p + 1 + stride])
        p += 1 + stride
        for i in range(stride):
            a = line[i - ch] if i >= ch else 0
            b = prev[i]
            c_ = prev[i - ch] if i >= ch else 0
            if f == 1:
                line[i] = (line[i] + a) & 255
            elif f == 2:
                line[i] = (line[i] + b) & 255
            elif f == 3:
                line[i] = (line[i] + (a + b) // 2) & 255
            elif f == 4:
                pa, pb, pc = abs(b - c_), abs(a - c_), abs(a + b - 2 * c_)
                pr = a if pa <= pb and pa <= pc else (b if pb <= pc else c_)
                line[i] = (line[i] + pr) & 255
        rows.append(line)
        prev = line
    return w, h, ch, rows


def write(path, w, h, ch, rows):
    raw = b"".join(b"\0" + bytes(r) for r in rows)
    def chunk(t, c):
        return struct.pack(">I", len(c)) + t + c + struct.pack(">I", zlib.crc32(t + c) & 0xffffffff)
    ct = {3: 2, 4: 6, 1: 0}[ch]
    open(path, "wb").write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, ct, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b""))


src, dst, x0, y0, x1, y1 = sys.argv[1], sys.argv[2], *[int(a) for a in sys.argv[3:7]]
w, h, ch, rows = read(src)
x1, y1 = min(x1, w), min(y1, h)
write(dst, x1 - x0, y1 - y0, ch, [r[x0 * ch:x1 * ch] for r in rows[y0:y1]])
print(w, h, "->", x1 - x0, y1 - y0)
