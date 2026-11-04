import struct, sys

TOP_DOWN = [[(255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0)], [(0, 255, 255), (255, 0, 255), (255, 255, 255), (128, 128, 128)]]
PPM = 2835


def build(rows):
    height, width = len(rows), len(rows[0])
    stride = (width * 24 + 31) // 32 * 4
    pixels = b"".join(b"".join(bytes((b, g, r)) for r, g, b in row).ljust(stride, b"\0") for row in reversed(rows))
    header = struct.pack("<2sIHHI", b"BM", 54 + len(pixels), 0, 0, 54)
    dib = struct.pack("<IiiHHIIiiII", 40, width, height, 1, 24, 0, len(pixels), PPM, PPM, 0, 0)
    return header + dib + pixels


def decode(data):
    assert data[:2] == b"BM"
    size, _, _, offset = struct.unpack_from("<IHHI", data, 2)
    dib, width, height, planes, bpp, compression, image_size, xppm, yppm = struct.unpack_from("<IiiHHIIii", data, 14)
    assert (dib, planes, bpp, compression, size) == (40, 1, 24, 0, len(data)) and height > 0
    stride = (width * bpp + 31) // 32 * 4
    rows = []
    for stored in range(height):
        row = data[offset + stored * stride: offset + stored * stride + width * 3]
        rows.append([(row[i + 2], row[i + 1], row[i]) for i in range(0, width * 3, 3)])
    return rows[::-1], (xppm, yppm)


if __name__ == "__main__":
    data = build(TOP_DOWN)
    assert decode(data) == (TOP_DOWN, (PPM, PPM))
    open(sys.argv[1], "wb").write(data)
    print(len(data), "bytes")
