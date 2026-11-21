"""🔬 Checks the neutral forward frame, inverse restoration and integer boundaries."""

from hashlib import sha256
import json
from pathlib import Path

from google_crc32c import value as crc32c

TICKET = Path(__file__).resolve().parent
ARCHIVE = json.loads((TICKET / "archive-cases.json").read_text(encoding="utf8"))
OBJECTS = json.loads((TICKET / "archive-object-laws.json").read_text(encoding="utf8"))


def frame(tag, text):
    payload = text.encode()
    value = len(payload)
    prefix = []
    while value >= 128:
        prefix.append(128 | (value & 127))
        value >>= 7
    return bytes([1, tag, *prefix, value]) + payload


def exact(value, maximum):
    if type(value) is not int or not 0 <= value <= maximum:
        raise ValueError("expected an exact unsigned integer")
    return value


def object_roles(text):
    record = json.loads(text)
    exact(record["id"]["num"], 2**32 - 1)
    exact(record["id"]["gen"], 2**16 - 1)
    roles = record["admittedStreamRoles"]
    for name in ("removed", "modified", "added"):
        if type(roles.get(name, [])) is not list:
            raise ValueError("role lane must be a list")
    for index in roles.get("removed", []):
        exact(index, 2**32 - 1)
    for row in roles.get("added", []):
        exact(row["index"], 2**32 - 1)
        if type(row["value"]["dependencies"]) is not list:
            raise ValueError("dependencies must be a list")
        if type(row["value"]["value"]["text"]) is not str:
            raise ValueError("metadata text must be a string")
    return record


forward = object_roles(ARCHIVE["json"])
inverse = object_roles(OBJECTS["inverseJson"])
original = frame(6, ARCHIVE["json"])
assert len(original) == 141 and list(original[2:4]) == ARCHIVE["length"]
assert crc32c(original[:20]) == 0x94C0DF0D
assert int.from_bytes(original[20:24], "little") == 0x4F747265
assert forward["id"] == inverse["id"]
removed = forward["admittedStreamRoles"]["removed"]
added = inverse["admittedStreamRoles"]["added"]
assert removed == [0] and added == [{"index": 0, "value": OBJECTS["priorRole"]}]
remaining = [role for i, role in enumerate([OBJECTS["priorRole"]]) if i not in removed]
assert not remaining
for row in added:
    remaining.insert(row["index"], row["value"])
assert remaining == [OBJECTS["priorRole"]]

for template, rows in [(ARCHIVE["json"], OBJECTS["invalidForward"]), (OBJECTS["inverseJson"], OBJECTS["invalidInverse"])]:
    for previous, replacement in rows:
        changed = template.replace(previous, replacement, 1)
        assert changed != template
        try:
            object_roles(changed)
        except ValueError:
            continue
        raise AssertionError(replacement)

forward["admittedStreamRoles"] = {key: value for key, value in forward["admittedStreamRoles"].items() if value}
canonical = frame(6, json.dumps(forward, separators=(",", ":")))
for name, data in [("original insert", original), ("canonical insert", canonical), ("canonical remove", frame(7, OBJECTS["inverseJson"]))]:
    print(f"[DEBUG] oracle {name}: {len(data)} bytes, CRC32C {crc32c(data):08x}, SHA-256 {sha256(data).hexdigest()}")
