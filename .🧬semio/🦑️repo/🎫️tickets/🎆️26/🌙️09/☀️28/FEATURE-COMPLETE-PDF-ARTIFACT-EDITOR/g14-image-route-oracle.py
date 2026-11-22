"""🖼️ Pillow oracle for the picked-raster route law: encodes real PNG and JPEG files, measures their pixel extent and encoded size, and decides each route from the declared lane limits."""
import io, json, sys
from pathlib import Path
from PIL import Image

LIMITS = {"inlineBytes": 24546, "inlinePixels": 4096, "pngBytes": 512 * 1024, "pngPixels": 262144, "dctBytes": 4 * 1024 * 1024, "dctPixels": 64 * 1024 * 1024}

def noise(width, height, seed):
    state, out = seed, bytearray()
    while len(out) < width * height * 3:
        state = (state * 6364136223846793005 + 1442695040888963407) & (2**64 - 1)
        out += (state >> 24).to_bytes(5, "little")[:3]
    return Image.frombytes("RGB", (width, height), bytes(out[: width * height * 3]))

def encode(fmt, width, height, quality=None, flat=False):
    image = Image.new("RGB", (width, height), (30, 100, 170)) if flat else noise(width, height, width * 31 + height)
    buffer = io.BytesIO()
    image.save(buffer, "PNG", compress_level=9) if fmt == "png" else image.save(buffer, "JPEG", quality=quality, subsampling=0)
    data = buffer.getvalue()
    with Image.open(io.BytesIO(data)) as opened:
        assert opened.format == fmt.upper() and opened.size == (width, height)
    return len(data)

def route(fmt, width, height, size):
    pixels = width * height
    if width == 0 or height == 0 or size == 0: return "refused"
    if fmt == "png" and size <= LIMITS["inlineBytes"] and pixels <= LIMITS["inlinePixels"]: return "inline"
    if fmt == "png" and size <= LIMITS["pngBytes"] and pixels <= LIMITS["pngPixels"]: return "pagedPng"
    if fmt == "jpeg" and size <= LIMITS["dctBytes"] and pixels <= LIMITS["dctPixels"]: return "pagedDct"
    return "refused"

cases = [
    ("tiny flat PNG", "png", 32, 32, dict(flat=True)),
    ("521 by 257 noise PNG", "png", 521, 257, {}),
    ("flat PNG above the sampled pixel cap but small", "png", 600, 600, dict(flat=True)),
    ("noise PNG above the paged PNG byte cap", "png", 720, 720, {}),
    ("2.1 MB class JPEG", "jpeg", 1600, 1200, dict(quality=min(range(30, 96), key=lambda q: abs(encode("jpeg", 1600, 1200, quality=q) - 2_100_000)))),
    ("small JPEG", "jpeg", 64, 64, dict(quality=90)),
    ("JPEG above the paged DCT byte cap", "jpeg", 2800, 2200, dict(quality=100)),
]
law = dict(schema="semio.picked-raster.route/v1", **LIMITS, cases=[])
for name, fmt, w, h, options in cases:
    size = encode(fmt, w, h, **options)
    law["cases"].append(dict(name=name, format=fmt, width=w, height=h, fileBytes=size, route=route(fmt, w, h, size)))
Path(sys.argv[1]).write_text(json.dumps(law, indent=1) + "\n", encoding="utf8")
print("[DEBUG]", [(c["name"], c["fileBytes"], c["route"]) for c in law["cases"]])
