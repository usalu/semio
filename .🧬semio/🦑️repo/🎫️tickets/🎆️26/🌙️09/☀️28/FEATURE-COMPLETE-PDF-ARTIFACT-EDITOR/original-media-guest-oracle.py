"""🖼️ Verifies a real two-page picker PNG with independent Pillow decoding."""

from hashlib import sha256
import json
from pathlib import Path
from struct import pack
import zlib
from PIL import Image

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "paged-media-receiver"


def chunk(kind, data):
    return pack(">I", len(data)) + kind + data + pack(">I", zlib.crc32(kind + data))


def verify():
    rows = json.loads((TICKET / "original-media-guest-cases.json").read_bytes())
    width, height = rows["png"]["width"], rows["png"]["height"]
    raster = bytes(index % 251 for index in range(width * height * 4))
    stride = width * 4
    line = b"".join(b"\x00" + raster[offset:offset + stride] for offset in range(0, len(raster), stride))
    encoded = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
    encoded += chunk(b"IDAT", zlib.compress(line, level=0)) + chunk(b"IEND", b"")
    extent = rows["pageBytes"]
    lengths = [len(encoded[pos:pos + extent]) for pos in range(0, len(encoded), extent)]
    assert lengths == rows["png"]["pageLengths"]
    assert sha256(encoded).hexdigest() == rows["png"]["sha256"]
    OUT.mkdir(parents=True, exist_ok=True)
    path = OUT / "raw-picker.png"
    path.write_bytes(encoded)
    with Image.open(path) as decoded:
        assert decoded.size == (width, height) and decoded.mode == "RGBA"
        rgba = decoded.tobytes()
    assert rgba == raster
    assert sha256(rgba).hexdigest() == rows["png"]["rasterSha256"]
    print(f"[DEBUG] Pillow verified {width}×{height} RGBA from {len(encoded)} PNG bytes with bounded pages {lengths}")


if __name__ == "__main__":
    verify()
