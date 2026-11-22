"""🧪️ Builds CCITT G3/G4 vectors with Pillow (libtiff) and writes them as a neutral JSON fixture."""

import io
import json
import random
import sys

from PIL import Image, TiffImagePlugin


def strip(image, compression):
    buffer = io.BytesIO()
    image.save(buffer, format="TIFF", compression=compression)
    buffer.seek(0)
    reopened = Image.open(buffer)
    tags = reopened.tag_v2
    offsets, counts = tags[273], tags[279]
    raw = buffer.getvalue()
    return b"".join(raw[o:o + c] for o, c in zip(offsets, counts)), tags.get(266, 1)


def packed(image):
    width, height = image.size
    stride = (width + 7) // 8
    out = bytearray()
    pixels = image.load()
    for y in range(height):
        row = bytearray(stride)
        for x in range(width):
            if pixels[x, y]:
                row[x >> 3] |= 0x80 >> (x & 7)
        out += row
    return bytes(out)


def pattern(width, height, seed, density):
    rng = random.Random(seed)
    image = Image.new("1", (width, height), 1)
    pixels = image.load()
    for y in range(height):
        x = 0
        color = 1
        while x < width:
            run = rng.choice([1, 2, 3, 7, 20, 63, 64, 100, 700, 1800, 2600]) if rng.random() < density else rng.randint(1, 12)
            for i in range(x, min(width, x + run)):
                pixels[i, y] = color
            x += run
            color ^= 1
        if y and rng.random() < 0.5:
            for i in range(width):
                pixels[i, y] = pixels[i, y - 1] ^ (1 if rng.random() < 0.01 else 0)
    return image


def main(path):
    cases = []
    for name, width, height, seed, density in [("small", 37, 11, 1, 0.1), ("wide", 3000, 9, 2, 0.6), ("typical", 200, 40, 3, 0.2), ("byte", 64, 16, 4, 0.0)]:
        image = pattern(width, height, seed, density)
        for compression, k in (("group4", -1), ("group3", 0)):
            data, fill = strip(image, compression)
            cases.append({"name": f"{name}-{compression}", "k": k, "columns": width, "rows": height, "dataHex": data.hex(), "expectedHex": packed(image).hex()})
    json.dump({"cases": cases}, open(path, "w"), separators=(",", ":"))
    print(len(cases), "CCITT cases")


main(sys.argv[1])
