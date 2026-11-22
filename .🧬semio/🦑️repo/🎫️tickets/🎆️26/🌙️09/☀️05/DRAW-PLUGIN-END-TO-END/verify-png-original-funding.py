"""Verify the uniform RGBA fixture with a fixed-code encoder and Python's zlib."""
import hashlib
import json
import struct
import sys
from pathlib import Path
import zlib

draw = Path("✏️s/🔌️plugins/🖍️draw")
fixture = next(draw.rglob("📦️owned/🧪️tests/🧫️fixtures/🔣️.json"))
case = json.loads(fixture.read_text())["pngOriginalFunding"]
width, height = case["width"], case["height"]
total = (width * 4 + 1) * height
assert (total * 9 + 7) // 8 + (total + 4095) // 4096 * 32 + 64 == case["capacity"]

def chunk(kind, data):
    body = kind + data
    return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body))

class FixedBits:
    def __init__(self):
        self.pending = 0
        self.count = 0
        self.output = bytearray(b"\x78\x01")

    def put(self, value, bits):
        self.pending |= value << self.count
        self.count += bits
        while self.count >= 8:
            self.output.append(self.pending & 255)
            self.pending >>= 8
            self.count -= 8

    def symbol(self, code):
        if code == 0:
            value, bits = 0x30, 8
        elif code <= 279:
            value, bits = code - 256, 7
        else:
            value, bits = 0xc0 + code - 280, 8
        self.put(int(f"{value:0{bits}b}"[::-1], 2), bits)

    def block(self, length, final):
        self.put(3 if final else 2, 3)
        self.symbol(0)
        remaining = length - 1
        while remaining:
            run = min(258, remaining)
            if run <= 2:
                for _ in range(run):
                    self.symbol(0)
            else:
                bases = (3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258)
                index = max(index for index, base in enumerate(bases) if base <= run)
                self.symbol(257 + index)
                self.put(run - bases[index], max(0, (index - 4) // 4) if index < 28 else 0)
                self.put(0, 5)
            remaining -= run
        self.symbol(256)
        if final:
            self.put(0, (-self.count) % 8)
            self.output.extend(struct.pack(">I", zlib.adler32(bytes(total))))
        chunk_body = bytes(self.output)
        self.output.clear()
        return chunk_body

bits = FixedBits()
blocks = [bits.block(min(4096, total - offset), offset + 4096 >= total) for offset in range(0, total, 4096)]
signature = bytes((137, 80, 78, 71, 13, 10, 26, 10))
expected = signature + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
expected += b"".join(chunk(b"IDAT", block) for block in blocks) + chunk(b"IEND", b"")
assert zlib.decompress(b"".join(blocks)) == bytes(total)
assert len(expected) == case["encodedBytes"]
print(f"capacity={case['capacity']} encoded={len(expected)} sha256={hashlib.sha256(expected).hexdigest()}")
if len(sys.argv) > 1:
    assert Path(sys.argv[1]).read_bytes() == expected
