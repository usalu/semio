"""🔬️ Expose the old shadow-stack global in a separate diagnostic core Wasm module."""

from mmap import ACCESS_READ, mmap
from os import link
from pathlib import Path


def decode(data, start):
    value = 0
    offset = 0
    while True:
        byte = data[start + offset]
        value |= (byte & 127) << (7 * offset)
        offset += 1
        if byte < 128:
            return value, start + offset


def encode(value):
    encoded = bytearray()
    while value >= 128:
        encoded.append(128 | (value & 127))
        value >>= 7
    encoded.append(value)
    return bytes(encoded)


def expose(data, output):
    assert data[:8] == b"\0asm\x01\0\0\0"
    position = 8
    stack_found = False
    while position < len(data):
        section = data[position]
        size, start = decode(data, position + 1)
        end = start + size
        if section == 6:
            _, first = decode(data, start)
            assert data[first:first + 3] == b"\x7f\x01\x41"
            value, after = decode(data, first + 3)
            assert value == 1048576 and data[after] == 11
            stack_found = True
        if section == 7:
            assert stack_found
            count, exports = decode(data, start)
            name = b"__semio_diagnostic_stack"
            entry = bytes([len(name)]) + name + b"\x03\x00"
            assert name not in data[exports:end]
            payload = encode(count + 1) + data[exports:end] + entry
            output.write(data[:position])
            output.write(bytes([7]) + encode(len(payload)) + payload)
            with memoryview(data) as view:
                output.write(view[end:])
            return
        position = end
    raise ValueError("missing core export section")


ticket = Path(__file__).resolve().parent
source = ticket / "🗑️generated/guest-browser"
output = source.with_name("guest-browser-stack")
output.mkdir(exist_ok=True)
core = "semio_hub_puzzle_component.core.wasm"
for filename in ("semio_hub_puzzle_component.js", "semio_hub_puzzle_component.core2.wasm", "semio_hub_puzzle_component.core3.wasm", "🌉️bridge.js", "🟨️.js"):
    if not (output / filename).exists():
        link(source / filename, output / filename)
with (source / core).open("rb") as original, mmap(original.fileno(), 0, access=ACCESS_READ) as data, (output / core).open("wb") as destination:
    expose(data, destination)
print("[DEBUG] The isolated core differs by one diagnostic export of the existing stack global.", flush=True)
