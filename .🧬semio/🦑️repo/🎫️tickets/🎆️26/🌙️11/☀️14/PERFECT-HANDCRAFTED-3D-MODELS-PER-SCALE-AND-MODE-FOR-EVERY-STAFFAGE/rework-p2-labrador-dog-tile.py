"""🧩 Tiles PNG renders nine to a 2400 x 1600 sheet without third-party modules (sips scales, zlib decodes).

Usage: python3 rework-p2-labrador-dog-tile.py <list of png paths.txt> <output prefix> [columns]   (3 columns: nine per sheet, 2: four per sheet)
"""
import os
import struct
import subprocess
import sys
import tempfile
import zlib


def read(path):
    data = open(path, 'rb').read()
    at, chunks, head = 8, b'', None
    while at < len(data):
        size, kind = struct.unpack('>I4s', data[at:at + 8])
        body = data[at + 8:at + 8 + size]
        if kind == b'IHDR':
            head = struct.unpack('>IIBBBBB', body)
        elif kind == b'IDAT':
            chunks += body
        at += 12 + size
    width, height, depth, colour = head[:4]
    n = {2: 3, 6: 4, 0: 1, 4: 2}[colour]
    if depth != 8:
        raise SystemExit('unsupported depth in ' + path)
    raw = zlib.decompress(chunks)
    stride = width * n
    rows, prev = [], bytearray(stride)
    for y in range(height):
        f = raw[y * (stride + 1)]
        line = bytearray(raw[y * (stride + 1) + 1:(y + 1) * (stride + 1)])
        if f == 1:
            for i in range(n, stride):
                line[i] = (line[i] + line[i - n]) & 255
        elif f == 2:
            line = bytearray((a + b) & 255 for a, b in zip(line, prev))
        elif f == 3:
            for i in range(stride):
                line[i] = (line[i] + (((line[i - n] if i >= n else 0) + prev[i]) >> 1)) & 255
        elif f == 4:
            for i in range(stride):
                a = line[i - n] if i >= n else 0
                b = prev[i]
                c = prev[i - n] if i >= n else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[i] = (line[i] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        prev = line
        if n == 3:
            rows.append(bytes(line))
        elif n == 4:
            rows.append(bytes(v for i, v in enumerate(line) if i % 4 != 3))
        else:
            rows.append(bytes(v for g in line[::n] for v in (g, g, g)))
    return width, height, rows


def write(path, width, rows):
    def chunk(kind, body):
        return struct.pack('>I', len(body)) + kind + body + struct.pack('>I', zlib.crc32(kind + body))
    raw = b''.join(b'\x00' + r for r in rows)
    open(path, 'wb').write(b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width, len(rows), 8, 2, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(raw, 6)) + chunk(b'IEND', b''))


def main():
    paths = [l.strip() for l in open(sys.argv[1], encoding='utf-8') if l.strip().endswith('.png')]
    n_col = int(sys.argv[3]) if len(sys.argv) > 3 else 3
    w, h, per = 2400 // n_col, 1600 // n_col, n_col * n_col
    tmp = tempfile.mkdtemp(dir=os.path.dirname(sys.argv[1]))
    for n in range(0, len(paths), per):
        tiles = []
        for i, p in enumerate(paths[n:n + per]):
            small = os.path.join(tmp, f'{n + i}.png')
            subprocess.run(['sips', '-z', str(h), str(w), p, '--out', small], check=True, capture_output=True)
            tiles.append(read(small)[2])
            os.remove(small)
        blank = [b'\xff' * (w * 3)] * h
        rows = []
        for r in range(n_col):
            trio = [tiles[r * n_col + c] if r * n_col + c < len(tiles) else blank for c in range(n_col)]
            rows += [b''.join(t[y] for t in trio) for y in range(h)]
        out = f'{sys.argv[2]}-{n // per}.png'
        write(out, w * n_col, rows)
        print(out)
    os.rmdir(tmp)


if __name__ == '__main__':
    main()
