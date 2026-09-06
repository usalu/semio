#!/usr/bin/env python3
"""🔬️ Third-party validation of the raster io BMP parity fixtures.

CLAUDE.md: "You MUST create the same output of a test with at least one third-party library in order
to validate our own implementation." The bun twin and the Rust leaf agree with EACH OTHER by
construction (they assert the same `🧫️fixtures/*.json`); this script asks a THIRD, independent
implementation — Apple's ImageIO, driven by macOS's own `sips(1)` — whether those bytes really are a
BMP with those pixels.

It writes each fixture's `bmpHex` out as a real `.bmp`, has `sips` transcode it to PNG, decodes that
PNG with nothing but the standard library (`zlib` + the PNG unfilter), and asserts every RGB triple
matches the fixture's own `rgba8` (alpha is the BMP v3 format's documented loss).

macOS-only on purpose: this is a ticket-local oracle, deliberately NOT wired into `bun test`, which
must stay cross-platform. Run it by hand from the repo root:

    python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🐍️w3-bmp-third-party-oracle.py"
"""

import binascii
import json
import struct
import subprocess
import sys
import tempfile
import zlib
from pathlib import Path

FIXTURES = Path("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures")
NAMES = ["🪟️solid-3x2.json", "🌈️gradient-5x3.json"]


def read_png(path: Path) -> tuple[int, int, int, bytes]:
    """🖼️ Minimal stdlib PNG reader — enough for whatever `sips` writes for a 24-bit BMP."""
    data = path.read_bytes()
    assert data[:8] == b"\x89PNG\r\n\x1a\n", f"{path} is not a PNG"
    offset, idat, width, height, depth, color = 8, b"", 0, 0, 0, 0
    while offset < len(data):
        length = struct.unpack(">I", data[offset : offset + 4])[0]
        kind = data[offset + 4 : offset + 8]
        chunk = data[offset + 8 : offset + 8 + length]
        offset += 12 + length
        if kind == b"IHDR":
            width, height, depth, color = struct.unpack(">IIBB", chunk[:10])
        elif kind == b"IDAT":
            idat += chunk
        elif kind == b"IEND":
            break
    raw = zlib.decompress(idat)
    channels = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}[color]
    unit = channels * depth // 8
    stride = width * unit
    out, previous, cursor = bytearray(), bytearray(stride), 0
    for _ in range(height):
        filter_kind = raw[cursor]
        cursor += 1
        line = bytearray(raw[cursor : cursor + stride])
        cursor += stride
        for x in range(stride):
            left = line[x - unit] if x >= unit else 0
            up = previous[x]
            corner = previous[x - unit] if x >= unit else 0
            if filter_kind == 1:
                line[x] = (line[x] + left) & 255
            elif filter_kind == 2:
                line[x] = (line[x] + up) & 255
            elif filter_kind == 3:
                line[x] = (line[x] + (left + up) // 2) & 255
            elif filter_kind == 4:
                pa, pb, pc = abs(up - corner), abs(left - corner), abs(left + up - 2 * corner)
                predictor = left if (pa <= pb and pa <= pc) else (up if pb <= pc else corner)
                line[x] = (line[x] + predictor) & 255
        out += line
        previous = line
    return width, height, channels, bytes(out)


def main() -> int:
    failures = 0
    with tempfile.TemporaryDirectory() as scratch:
        for name in NAMES:
            fixture = json.loads((FIXTURES / name).read_text(encoding="utf-8"))
            bmp = Path(scratch) / f"{name}.bmp"
            png = Path(scratch) / f"{name}.png"
            bmp.write_bytes(binascii.unhexlify(fixture["bmpHex"]))
            subprocess.run(["sips", "-s", "format", "png", str(bmp), "--out", str(png)], check=True, capture_output=True)
            width, height, channels, pixels = read_png(png)
            if (width, height) != (fixture["width"], fixture["height"]):
                print(f"FAIL {name}: sips read {width}x{height}, fixture says {fixture['width']}x{fixture['height']}")
                failures += 1
                continue
            mismatches = 0
            for y in range(height):
                for x in range(width):
                    index = (y * width + x)
                    got = pixels[index * channels : index * channels + 3]
                    want = bytes(fixture["rgba8"][index * 4 : index * 4 + 3])
                    if got != want:
                        mismatches += 1
                        print(f"FAIL {name}: pixel ({x},{y}) is {list(got)}, fixture says {list(want)}")
            if mismatches:
                failures += 1
            else:
                print(f"ok {name}: Apple ImageIO reads {width}x{height} and every RGB triple matches the fixture")
    return 1 if failures else 0


if __name__ == "__main__":
    if sys.platform != "darwin":
        print("skipped: this oracle drives macOS `sips`")
        raise SystemExit(0)
    raise SystemExit(main())
