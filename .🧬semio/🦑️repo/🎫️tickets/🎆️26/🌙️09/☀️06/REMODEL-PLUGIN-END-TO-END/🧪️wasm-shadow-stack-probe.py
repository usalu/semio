"""🔬️ Read the exact core WebAssembly functions reported by the staged guest trap."""

import mmap
import sys


def leb(data, offset, signed=False):
    value = 0
    for index in range(10):
        byte = data[offset + index]
        value |= (byte & 127) << (7 * index)
        if byte < 128:
            if signed and byte & 64:
                value -= 1 << (7 * (index + 1))
            return value, offset + index + 1
    raise ValueError("Invalid LEB128")


def sections(data):
    offset = 8
    while offset < len(data):
        section_id = data[offset]
        size, start = leb(data, offset + 1)
        offset = start + size
        yield section_id, start, offset


def bodies(data, code_start):
    count, offset = leb(data, code_start)
    for index in range(count):
        size, start = leb(data, offset)
        offset = start + size
        yield index, start, offset


def instructions(data, start):
    local_groups, offset = leb(data, start)
    for _ in range(local_groups):
        _, offset = leb(data, offset)
        offset += 1
    return offset


def stack_frame(data, start):
    if data[start] != 0x23:
        return None
    global_index, constant = leb(data, start + 1)
    if global_index != 0 or data[constant] != 0x41:
        return None
    frame_bytes, subtraction = leb(data, constant + 1, signed=True)
    return frame_bytes if data[subtraction] == 0x6B else None


with open(sys.argv[1], "rb") as stream:
    data = mmap.mmap(stream.fileno(), 0, access=mmap.ACCESS_READ)
    assert data[:8] == b"\x00asm\x01\x00\x00\x00"
    directory = {kind: start for kind, start, _ in sections(data)}
    global_count, global_zero = leb(data, directory[6])
    assert global_count > 0 and data[global_zero:global_zero + 3] == bytes((0x7F, 1, 0x41))
    stack_base, end = leb(data, global_zero + 3, signed=True)
    assert data[end] == 0x0B
    print(f"stack global 0 initializes to {stack_base} bytes")
    frames = (34319, 34317, 34315, 41161, 21913, 2460, 28303, 2453, 2451, 7351)
    for index, start, _ in bodies(data, directory[10]):
        function_id = index + 73
        if function_id in frames:
            first = instructions(data, start)
            print(f"wasm-function[{function_id}] starts at {first:#x}, frame bytes {stack_frame(data, first)}")
