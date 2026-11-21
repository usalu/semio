"""🔬 Create raw PNG specimens and compare the production decoder's result to Pillow."""

from hashlib import sha256
import json
from io import BytesIO
from pathlib import Path
from struct import pack
import sys
import zlib
from PIL import Image

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "raw-png"
SIGNATURE = b"\x89PNG\r\n\x1a\n"
PASS = ((0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2))


def chunk(kind, data=b""):
    """📦 Spell the PNG chunk length, type, payload and CRC independently."""
    return pack(">I", len(data)) + kind + data + pack(">I", zlib.crc32(kind + data))


def header(width, height, color, interlace=0):
    """🖼️ Construct a neutral 8-bit specimen's fixed PNG header."""
    return SIGNATURE + chunk(b"IHDR", pack(">IIBBBBB", width, height, 8, color, 0, 0, interlace))


def advance_to(encoded, offset):
    """📏 Pad with a legal ancillary chunk until a framing boundary crosses a fixed page."""
    assert offset >= len(encoded) + 12
    return encoded + chunk(b"vpAg", bytes(offset - len(encoded) - 12))


def fixtures():
    """🎨 Cover multiple PNG color paths and paged chunk structure."""
    delivery = TICKET / "🗑️generated" / "paged-media-receiver" / "raw-picker.png"
    original = delivery.read_bytes()
    original_law = json.loads((TICKET / "original-media-guest-cases.json").read_bytes())["png"]
    assert sha256(original).hexdigest() == original_law["sha256"]
    rgba = [bytes((x * 11 % 251, y * 17 % 251, (x * 31 + y * 41) % 251, (x + y * 3) % 251))
            for y in range(19) for x in range(23)]
    adam7 = b"".join(b"\0" + b"".join(rgba[y * 23 + x] for x in range(sx, 23, dx))
                    for sx, sy, dx, dy in PASS for y in range(sy, 19, dy))
    interlaced = header(23, 19, 6, 1) + chunk(b"IDAT", zlib.compress(adam7, 0)) + chunk(b"IEND")
    interlaced = advance_to(interlaced[:-12], 4093) + chunk(b"IEND")
    palette = bytes((index * 31) % 251 for index in range(768))
    alpha = bytes(range(256))
    indices = bytes(range(256))
    palette_frame = advance_to(header(16, 16, 3), 4096 - 9) + chunk(b"PLTE", palette)
    palette_frame = advance_to(palette_frame, 8192 - 9) + chunk(b"tRNS", alpha)
    palette_frame += chunk(b"IDAT", zlib.compress(b"".join(b"\0" + indices[i:i + 16] for i in range(0, 256, 16)))) + chunk(b"IEND")
    zdata = zlib.compress(b"".join(b"\0" + b"".join(rgba[i:i + 23]) for i in range(0, len(rgba), 23)), 0)
    framing = advance_to(header(23, 19, 6), 4096 - 3)
    framing += chunk(b"IDAT", zdata[:7])
    framing += chunk(b"IDAT", zdata[7:])
    framing = advance_to(framing, 8192 - 10) + chunk(b"IEND")
    altered = bytearray(original)
    altered[-13] ^= 1
    return {"rgba": original, "palette": palette_frame, "interlacedRgba": interlaced,
            "splitFraming": framing, "alteredChecksum": bytes(altered)}


def verify(compare=False):
    """🧬 Derive decoded ground truth and optionally compare actual native output."""
    OUT.mkdir(parents=True, exist_ok=True)
    schema = json.loads((TICKET / "raw-png-handoff.schema.json").read_bytes())
    con = schema["properties"]
    controls = con["controls"]["properties"]
    for name, encoded in fixtures().items():
        assert con["pageBytes"]["const"] < len(encoded) <= con["maximumBytes"]["const"]
        path = OUT / f"{name}.png"
        path.write_bytes(encoded)
        if controls[name]["const"] == "refused":
            try:
                with Image.open(BytesIO(encoded)) as image:
                    image.verify()
            except (OSError, SyntaxError):
                print(f"[DEBUG] Pillow rejected {name}")
                continue
            raise ValueError("Pillow admitted the corrupted PNG")
        with Image.open(BytesIO(encoded)) as image:
            assert image.width * image.height <= con["maximumPixels"]["const"]
            raster = image.convert("RGBA").tobytes()
        if compare:
            assert (OUT / f"{name}.native").read_bytes() == raster
        (OUT / f"{name}.rgba").write_bytes(raster)
        print(f"[DEBUG] Pillow {name}: {len(encoded)} encoded bytes; {len(raster)} RGBA bytes; {sha256(raster).hexdigest()}")


if __name__ == "__main__":
    verify("--compare" in sys.argv)
