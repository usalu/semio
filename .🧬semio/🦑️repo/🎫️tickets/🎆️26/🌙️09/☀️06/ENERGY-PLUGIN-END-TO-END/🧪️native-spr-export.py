"""🧪️ Independent BLAKE3 and binary grammar verify native SPR output."""
import json
import pathlib
import struct
import sys
import blake3

root = pathlib.Path(__file__).resolve().parents[7]
law = json.loads((root / '🧰️framework/🔨️modules/📡️replication/📐️format/🛫️encode/🫳️borrowed/🔣️.json').read_text())
wire = pathlib.Path(sys.argv[1]).read_bytes()
assert wire[:8] == b'\x89SPR\r\n\x1a\n'

def crc32c(data):
    value = 0xffffffff
    for byte in data:
        value ^= byte
        for _ in range(8):
            value = (value >> 1) ^ (0x82f63b78 if value & 1 else 0)
    return value ^ 0xffffffff

assert crc32c(wire[:20]) == struct.unpack_from('<I', wire, 20)[0]
position = 32
frames = []
while position < len(wire):
    start = position
    body = shift = 0
    while True:
        byte = wire[position]
        position += 1
        body |= (byte & 127) << shift
        if byte < 128:
            break
        shift += 7
    first = position
    kind, flags = wire[position:position + 2]
    position += body
    assert crc32c(wire[first:position]) == struct.unpack_from('<I', wire, position)[0]
    position += 8
    assert struct.unpack_from('<I', wire, position - 4)[0] == position - start
    frames.append((kind, flags, wire[first + 2:first + body], wire[start:position]))
assert len(frames) == len(law['records']) + 1
for actual, expected in zip(frames, law['records']):
    assert actual[0] == expected['kind']
    assert actual[2] == (expected['text'] * expected.get('repeat', 1)).encode()
    assert bool(actual[1] & 2) == expected['critical']
commit = frames[-1][2]
assert struct.unpack_from('<QQQI', commit) == (1, 0, sum(len(frame[3]) for frame in frames[:-1]), len(law['records']))
chain = blake3.blake3(wire[:32]).digest() + b''.join(blake3.blake3(frame[3]).digest() for frame in frames[:-1])
assert commit[32:64] == blake3.blake3(chain).digest()
print('Independent native SPR: payloads, framing, CRC32C and BLAKE3 commit chain PASS')
