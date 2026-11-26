"""📖 Decode the single-field Semio pack independently and compare its JSON document."""
import hashlib
import json
import math
from pathlib import Path
import struct
import sys


class Document:
    def __init__(self, pack):
        self.pack = memoryview(pack)
        self.offset = 0
        self.symbols = [self.text() for _ in range(self.varint())]

    def read(self, n):
        offset = self.offset
        self.offset += n
        if self.offset > len(self.pack):
            raise ValueError("Truncated value")
        return self.pack[offset:self.offset]

    def byte(self):
        return self.read(1)[0]

    def varint(self):
        value = 0
        for shift in range(0, 64, 7):
            octet = self.byte()
            value |= (octet & 127) << shift
            if octet < 128 and value < 1 << 64:
                return value
        raise ValueError("Invalid varint")

    def text(self):
        return self.read(self.varint()).tobytes().decode("utf-8")

    def string(self, tag):
        if tag == 6:
            return self.symbols[self.varint()]
        if tag == 7:
            return self.text()
        raise ValueError(f"Invalid key tag {tag}")

    def value(self):
        tag = self.byte()
        if tag == 18:
            return None
        if tag in (1, 2):
            return tag == 2
        if tag in (3, 4):
            value = self.varint()
            return (value >> 1) ^ -(value & 1) if tag == 3 else value
        if tag == 5:
            return struct.unpack("<d", self.read(8))[0]
        if tag in (6, 7):
            return self.string(tag)
        if tag == 12:
            return [self.value() for _ in range(self.varint())]
        if tag == 16:
            result = {}
            for _ in range(self.varint()):
                key = self.string(self.byte())
                if key in result:
                    raise ValueError(f"Duplicate pack key {key}")
                result[key] = self.value()
            return result
        raise ValueError(f"Unsupported JSON value tag {tag}")

    def document(self):
        if (self.varint(), self.varint(), self.byte()) != (1, 1, 17):
            raise ValueError("Require a single semantic descriptor field")
        value = self.value()
        if self.offset != len(self.pack):
            raise ValueError("Trailing pack bytes")
        return value


def same(left, right):
    if type(left) is dict and type(right) is dict:
        return left.keys() == right.keys() and all(same(value, right[key]) for key, value in left.items())
    if type(left) is list and type(right) is list:
        return len(left) == len(right) and all(same(a, b) for a, b in zip(left, right))
    if type(left) in (int, float) and type(right) in (int, float):
        return math.isfinite(left) and math.isfinite(right) and left == right
    return type(left) is type(right) and left == right


if __name__ == "__main__":
    pack, mirror = (Path(filename).read_bytes() for filename in sys.argv[1:])
    if len(pack) > 8 * 1024 * 1024 or not same(Document(pack).document(), json.loads(mirror)):
        raise SystemExit("Pack does not match semantic JSON")
    print(json.dumps({"bytes": len(pack), "semanticSha256": hashlib.sha256(pack).hexdigest()}))
