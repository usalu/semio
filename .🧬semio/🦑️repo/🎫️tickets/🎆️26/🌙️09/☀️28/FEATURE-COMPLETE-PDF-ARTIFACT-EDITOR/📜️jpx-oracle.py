"""🧪️ Encodes JPEG 2000 vectors with Pillow (OpenJPEG) and records OpenJPEG's own decode as the expectation."""

import io
import random
import sys

from PIL import Image


def picture(width, height, mode, seed):
    rng = random.Random(seed)
    image = Image.new(mode, (width, height))
    pixels = image.load()
    channels = 3 if mode == "RGB" else 1
    for y in range(height):
        for x in range(width):
            base = (x * 255 // max(width - 1, 1), y * 255 // max(height - 1, 1), (x * y) % 256)
            noise = rng.randint(-12, 12)
            values = [max(0, min(255, base[c % 3] + noise + (30 if (x // 7 + y // 5) % 2 else 0))) for c in range(channels)]
            pixels[x, y] = tuple(values) if channels == 3 else values[0]
    return image


CASES = [
    ("gray-lossless", ("L", 61, 47), dict(irreversible=False), 0),
    ("rgb-lossless-mct", ("RGB", 64, 48), dict(irreversible=False, mct=1), 0),
    ("rgb-lossless-nomct", ("RGB", 33, 29), dict(irreversible=False, mct=0, num_resolutions=3), 0),
    ("rgb-lossy-97", ("RGB", 64, 48), dict(irreversible=True, quality_mode="rates", quality_layers=[20]), 3),
    ("gray-lossy-layers", ("L", 70, 50), dict(irreversible=True, quality_mode="rates", quality_layers=[40, 10, 1]), 3),
    ("tiles-rpcl", ("RGB", 70, 50), dict(irreversible=False, tile_size=(32, 32), progression="RPCL"), 0),
    ("pcrl", ("RGB", 50, 40), dict(irreversible=False, progression="PCRL", num_resolutions=3), 0),
    ("cprl", ("RGB", 50, 40), dict(irreversible=False, progression="CPRL", num_resolutions=3), 0),
    ("rlcp-layers", ("L", 50, 40), dict(irreversible=False, progression="RLCP", quality_mode="rates", quality_layers=[8, 3, 1]), 0),
    ("precincts", ("RGB", 100, 80), dict(irreversible=False, precinct_size=(32, 32), cblk_size=(16, 16), num_resolutions=3), 0),
    ("style-bypass", ("L", 64, 48), dict(irreversible=False, codeblock_style=1), 0),
    ("style-reset", ("L", 64, 48), dict(irreversible=False, codeblock_style=2), 0),
    ("style-termall", ("L", 64, 48), dict(irreversible=False, codeblock_style=4), 0),
    ("style-vcausal", ("L", 64, 48), dict(irreversible=False, codeblock_style=8), 0),
    ("style-segsym", ("L", 64, 48), dict(irreversible=False, codeblock_style=32), 0),
    ("style-all-rgb-lossy", ("RGB", 48, 40), dict(irreversible=True, codeblock_style=47, quality_mode="rates", quality_layers=[10, 3]), 3),
    ("codeblock-small", ("L", 48, 48), dict(irreversible=True, cblk_size=(8, 8), num_resolutions=2), 3),
]


def main(path):
    rows = []
    for name, (mode, width, height), options, tolerance in CASES:
        source = picture(width, height, mode, hash(name) % 1000)
        buffer = io.BytesIO()
        source.save(buffer, "JPEG2000", **options)
        data = buffer.getvalue()
        decoded = Image.open(io.BytesIO(data))
        decoded.load()
        rows.append("\t".join([name, str(width), str(height), str(1 if mode == "L" else 3), str(tolerance), data.hex(), decoded.convert(mode).tobytes().hex()]))
    open(path, "w").write("\n".join(rows))
    print(len(rows), "JPX cases")


main(sys.argv[1])
