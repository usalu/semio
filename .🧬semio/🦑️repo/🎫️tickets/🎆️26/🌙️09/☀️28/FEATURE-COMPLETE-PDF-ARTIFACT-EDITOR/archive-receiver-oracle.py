"""🔬 Checks the ticket's neutral format-1 frame and refusal specimens."""

import hashlib
import json
from pathlib import Path

from google_crc32c import value as crc32c

TICKET = Path(__file__).resolve().parent
CASE = json.loads((TICKET / "archive-cases.json").read_bytes())


def unsigned(value):
    groups = []
    while value >= 128:
        groups.append(128 | (value & 127))
        value >>= 7
    return bytes([*groups, value])


def unique_object(pairs):
    result = dict(pairs)
    if len(result) != len(pairs):
        raise ValueError("duplicate member")
    return result


def read(frame):
    if frame[0] != 1:
        raise ValueError("version")
    tag = {6: "insertObject", 7: "removeObject", 26: "setImage"}[frame[1]]
    for count, byte in enumerate(frame[2:12], 1):
        if byte < 128:
            break
    else:
        raise ValueError("overlong length")
    encoded = frame[2:2 + count]
    length = sum((part & 127) << (7 * index) for index, part in enumerate(encoded))
    if length >= 1 << 64 or unsigned(length) != encoded:
        raise ValueError("noncanonical length")
    if length > 64 * 1024 * 1024:
        raise OverflowError("payload over 64 MiB")
    payload = frame[2 + count:]
    if len(payload) != length:
        raise ValueError("payload extent")
    value = json.loads(payload, object_pairs_hook=unique_object)
    if value["mutation"] != tag:
        raise ValueError("tag mismatch")
    if tag == "insertObject" and type(value["id"]["num"]) is not int:
        raise ValueError("typed payload mismatch")
    return value


payload = CASE["json"].encode()
frame = bytes([CASE["format"], CASE["tag"], *CASE["length"]]) + payload
assert CASE["length"] == list(unsigned(len(payload)))
assert read(frame) == json.loads(payload)
assert json.dumps(read(frame), separators=(",", ":"), ensure_ascii=False).encode() == payload
assert crc32c(frame[:20]) == 0x94C0DF0D
assert int.from_bytes(frame[20:24], "little") == 0x4F747265
for case in CASE["refusals"]:
    if "header" in case:
        bad = bytes(case["header"]) + payload
    else:
        data = case["json"].encode()
        bad = bytes([1, case["tag"]]) + unsigned(len(data)) + data
    try:
        read(bad)
    except (ValueError, KeyError, OverflowError) as error:
        assert isinstance(error, OverflowError) == (case.get("kind") == "work-limit"), case
    else:
        raise AssertionError(case)
print(f"[DEBUG] {len(CASE['refusals'])} frame refusals and {len(frame)} exact valid bytes, SHA-256 {hashlib.sha256(frame).hexdigest()}")
image = CASE["largeImage"]
count = image["width"] * image["height"]
value = {"mutation": "setImage", "image": {"id": "Gray", "width": image["width"],
    "height": image["height"], "colorSpace": {"kind": "deviceGray"}, "bitsPerComponent": 8,
    "imageMask": False, "interpolate": False, "body": {"kind": "samples", "values": [image["sample"]]}}}
small = json.dumps(value, separators=(",", ":")).encode()
payload = small.replace(b'"values":[0]', b'"values":[0' + b',0' * (count - 1) + b']')
large_frame = bytes([CASE["format"], image["tag"]]) + unsigned(len(payload)) + payload
image_samples = read(large_frame)["image"]["body"]["values"]
assert len(image_samples) == count and all(sample == image["sample"] for sample in image_samples)
assert count * 4 == image["hostMaximumCapacityBytes"]
assert count * 24 > image["hostMaximumCapacityBytes"]
print(f"[DEBUG] {count} grayscale samples, {len(payload)} payload bytes, {len(large_frame)} frame bytes, SHA-256 {hashlib.sha256(large_frame).hexdigest()}")
