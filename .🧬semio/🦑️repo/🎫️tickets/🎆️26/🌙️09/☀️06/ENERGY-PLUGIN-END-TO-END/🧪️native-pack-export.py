"""🧪️ Independent zlib and BLAKE3 verify the native snapshot's canonical physical bytes."""
import json
import pathlib
import struct
import sys
import zlib
import blake3

root = pathlib.Path(__file__).resolve().parents[7]
fixture = root / '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/📸️export/🔣️.json'
law = json.loads(fixture.read_text())
wire = pathlib.Path(sys.argv[1]).read_bytes()
assert wire[:8] == b'\x89SEM\r\n\x1a\n'
length = struct.unpack_from('<I', wire, 8)[0]
assert wire[12:12 + length].decode() == law['envelope'] + '.pack v1'
pack = wire[12 + length:]
assert pack[:8] == b'\x89SPK\r\n\x1a\n'

def crc32c(data):
    value = 0xffffffff
    for byte in data:
        value ^= byte
        for _ in range(8):
            value = (value >> 1) ^ (0x82f63b78 if value & 1 else 0)
    return value ^ 0xffffffff

assert crc32c(pack[:20]) == struct.unpack_from('<I', pack, 20)[0]
footer = pack[-84:]
assert crc32c(footer[:80]) == struct.unpack_from('<I', footer, 80)[0]
assert struct.unpack_from('<Q', footer, 32)[0] == len(pack)
position = 32

def varint():
    global position
    value = shift = 0
    while True:
        byte = pack[position]
        position += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value
        shift += 7

segments = {}
while position < len(pack) - 84:
    start = position
    kind, flags = pack[position:position + 2]
    position += 2
    stored = varint()
    raw = varint() if flags & 1 else stored
    payload = pack[position:position + stored]
    position += stored
    assert crc32c(pack[start:position]) == struct.unpack_from('<I', pack, position)[0]
    position += 4
    data = zlib.decompress(payload, -15) if flags & 1 else payload
    assert len(data) == raw
    segments.setdefault(kind, []).append(data)
assert (law['text'] * law['repeat']).encode() in b''.join(data for rows in segments.values() for data in rows)
assert blake3.blake3(b''.join(segments[4])).digest() == footer[40:72]
print('Independent native Pack export: zlib, CRC32C, BLAKE3 and original Unicode symbols PASS')
