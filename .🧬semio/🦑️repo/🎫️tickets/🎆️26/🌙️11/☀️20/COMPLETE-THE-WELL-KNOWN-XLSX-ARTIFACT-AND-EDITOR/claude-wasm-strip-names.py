"""✂️ Removes the `name` custom sections of a wasm component's embedded core modules, the same reduction `strip = "symbols"` applies in the managed prototype profile. Usage: python3 claude-wasm-strip-names.py <in.wasm> <out.wasm>"""

import sys


def leb_read(data: bytes, at: int) -> tuple[int, int]:
    value = shift = 0
    while True:
        byte = data[at]
        at += 1
        value |= (byte & 0x7F) << shift
        shift += 7
        if byte < 0x80:
            return value, at


def leb_write(value: int) -> bytes:
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        out.append(byte | (0x80 if value else 0))
        if not value:
            return bytes(out)


def strip(data: bytes, nested: bool) -> bytes:
    out = bytearray(data[:8])
    at = 8
    while at < len(data):
        section = data[at]
        size, body = leb_read(data, at + 1)
        payload = data[body:body + size]
        at = body + size
        if section == 0:
            length, name_at = leb_read(payload, 0)
            if payload[name_at:name_at + length] == b"name":
                continue
        if not nested and section == 1 and payload[:4] == b"\0asm":
            payload = strip(payload, True)
        out.append(section)
        out += leb_write(len(payload))
        out += payload
    return bytes(out)


source, target = sys.argv[1], sys.argv[2]
original = open(source, "rb").read()
reduced = strip(original, False)
open(target, "wb").write(reduced)
print(f"{source}: {len(original)} -> {len(reduced)} bytes")
