"""🔍 Relate a captured guest trap to Rust source locations in its compiled core module."""

import json
import mmap
import re
import struct
import sys
from pathlib import Path


def leb(data, position, signed=False):
    """🔢 Read a bounded WebAssembly integer with its ending file position."""
    value = 0
    for shift in range(0, 35, 7):
        byte = data[position]
        position += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return (value - (1 << (shift + 7)) if signed and byte & 64 else value), position
    raise ValueError("Unterminated WebAssembly integer")


def sections(module):
    """🧭 Find the exact code and data ranges without evaluating the module."""
    assert module[:8] == b"\0asm\1\0\0\0"
    result = {}
    cursor = 8
    while cursor < len(module):
        kind = module[cursor]
        size, cursor = leb(module, cursor + 1)
        assert size <= len(module) - cursor
        if kind:
            result[kind] = cursor, cursor + size
        cursor += size
    return result


def memory_ranges(module, data_section):
    """📚 Resolve constant addresses through the module's active data segments."""
    cursor, end = data_section
    count, cursor = leb(module, cursor)
    ranges = []
    for _ in range(count):
        kind, cursor = leb(module, cursor)
        assert kind in (0, 1, 2)
        address = None
        if kind != 1:
            if kind == 2:
                _, cursor = leb(module, cursor)
            assert module[cursor] == 0x41
            address, cursor = leb(module, cursor + 1, signed=True)
            assert module[cursor] == 0x0b
            cursor += 1
        length, cursor = leb(module, cursor)
        if address is not None:
            ranges.append((address, address + length, cursor))
        cursor += length
    assert cursor == end
    return ranges


def located_bodies(module, code, stack):
    """🎯 Match each reported program counter to a Wasm function body."""
    cursor, end = code
    count, cursor = leb(module, cursor)
    expected = sorted(set(stack), key=lambda pair: pair[1])
    bodies = {}
    imported = None
    for ordinal in range(count):
        size, cursor = leb(module, cursor)
        stop = cursor + size
        while expected and cursor <= expected[0][1] < stop:
            number, address = expected.pop(0)
            if imported is None:
                imported = number - ordinal
            assert number - ordinal == imported
            bodies[number, address] = (cursor, stop)
        cursor = stop
    assert cursor == end and not expected
    return bodies


def string_pointer(module, ranges, pointer, count):
    """📍 Read a printable source filename from a bounded immutable data range."""
    if not 1 <= count <= 256:
        return None
    for start, stop, offset in ranges:
        if start <= pointer and pointer + count <= stop:
            text = module[offset + pointer - start:offset + pointer - start + count].decode("utf-8", errors="replace")
            if text.endswith(".rs") and all(character.isprintable() for character in text):
                return text
    return None


def candidate_sites(module, ranges, begin, end):
    """🧩 Recover plausible Rust Location records used by each trapped function."""
    locations = set()
    for offset in range(begin, end):
        if module[offset] != 0x41:
            continue
        try:
            pointer, _ = leb(module, offset + 1, signed=True)
        except ValueError:
            continue
        for start, stop, data in ranges:
            if start <= pointer and pointer + 16 <= stop:
                ptr, length, line, col = struct.unpack_from("<4I", module, data + pointer - start)
                text = string_pointer(module, ranges, ptr, length)
                if text and 0 < line <= 12000 and 0 < col <= 1000:
                    locations.add(f"{text}:{line}:{col}")
    return sorted(locations)


if __name__ == "__main__":
    binary, trace = map(Path, sys.argv[1:3])
    reply = json.loads(trace.read_text(encoding="utf-8"))["replies"][0]
    stack = [(int(number), int(offset, 16)) for number, offset in re.findall(r"wasm-function\[(\d+)\]:0x([0-9a-f]+)", reply["stack"])]
    assert stack
    with binary.open("rb") as file, mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ) as module:
        ranges = sections(module)
        data = memory_ranges(module, ranges[11])
        bodies = located_bodies(module, ranges[10], stack)
        for number, address in stack:
            begin, end = bodies[number, address]
            close = candidate_sites(module, data, max(begin, address - 24), min(end, address + 12))
            sites = candidate_sites(module, data, begin, end)
            window = module[max(begin, address - 64):min(end, address + 24)].hex(" ")
            print("[DEBUG]", f"{number}@{address:x}", end - begin, json.dumps({"near": close, "sites": sites[:24], "window": window}, ensure_ascii=False))
