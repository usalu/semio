"""🧪️ Independently validate native Window frame fixtures with Protocol Buffers varints."""
import json
from pathlib import Path
from google.protobuf.internal.decoder import _DecodeVarint
from google.protobuf.internal.encoder import _VarintBytes
from google.protobuf.message import DecodeError

root = Path(__file__).resolve().parents[7]
fixture = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🔏️pack-authority-and-publication/📦️source/🧪️tests/🔣️.json"
vectors = json.loads(fixture.read_text())
for row in vectors["valid"]:
    prefix = bytes(row["prefix"])
    assert prefix == _VarintBytes(row["packLength"])
    frame = prefix + bytes([42]) * row["packLength"] + bytes(row["spr"])
    length, start = _DecodeVarint(frame, 0)
    assert length == row["packLength"]
    assert frame[start:start + length] == bytes([42]) * length
    assert frame[start + length:] == bytes(row["spr"])
for row in vectors["invalid"]:
    frame = bytes(row)
    try:
        length, start = _DecodeVarint(frame, 0)
        accepted = frame[:start] == _VarintBytes(length) and length <= len(frame) - start
    except (IndexError, DecodeError):
        accepted = False
    assert not accepted, row
print("PASS: 5 valid and 9 refused native Window frames match Protocol Buffers")
