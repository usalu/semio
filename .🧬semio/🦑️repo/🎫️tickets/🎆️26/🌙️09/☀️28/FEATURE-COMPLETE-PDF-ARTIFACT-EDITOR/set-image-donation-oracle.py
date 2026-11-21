"""🔬 Checks the two captured unchanged writer frames with independent wire tools."""

from hashlib import sha256
import json
import os
from pathlib import Path

from google_crc32c import value as crc32c

TICKET = Path(__file__).resolve().parent
WIRE = Path(os.environ.get("SEMIO_TICKET_DONATION_OVERLAY", TICKET / "🗑️generated" / "set-donation"))
CASES = json.loads((WIRE / "7.json").read_text(encoding="utf8"))


def unique(entries):
    value = dict(entries)
    assert len(value) == len(entries)
    return value


def image_metadata(wire):
    image = wire["image"]
    if type(image["width"]) is not int:
        raise ValueError("width is not an exact integer")
    if any(type(value) not in (int, float) for value in image["decode"]):
        raise ValueError("decode vector requires numbers")
    if any(type(value) is not int for value in image["mask"]["ranges"]):
        raise ValueError("mask ranges require exact integers")
    if type(image["extra"][0]["value"]["value"]) is not int:
        raise ValueError("COS integer requires exact integer")
    return image


for lane in ("image", "alpha"):
    case = CASES[lane]
    frame = (WIRE / f"{lane}.bin").read_bytes()
    assert frame[:2] == bytes([1, 26])
    length = 0
    for index, octet in enumerate(frame[2:12]):
        length |= (octet & 127) << (7 * index)
        if octet < 128:
            assert index == 0 or octet != 0
            break
    else:
        raise AssertionError("unterminated length")
    payload = frame[3 + index:]
    assert len(payload) == length
    value = json.loads(payload, object_pairs_hook=unique)
    assert value["mutation"] == "setImage" and value["index"] == 3
    image = value["image"]
    assert image["id"] == case["id"] and image["width"] == case["width"] and image["height"] == case["height"]
    body = image["body"]
    count = case["width"] * case["height"] * case["components"]
    assert body["kind"] == "samples" and body["values"] == [i * case["stride"] % 256 for i in range(count)]
    laws = CASES.get("metadata")
    if laws:
        assert image_metadata(value) == image
        if lane == "image":
            text = payload.decode()
            for old, new in laws["invalid"]:
                mutated = text.replace(old, new, 1)
                assert mutated != text
                try:
                    image_metadata(json.loads(mutated, object_pairs_hook=unique))
                except ValueError:
                    continue
                raise AssertionError(new)
    print(f"[DEBUG] oracle {lane}: {len(frame)} bytes crc32c {crc32c(frame):08x} sha256 {sha256(frame).hexdigest()}")
