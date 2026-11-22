import mmap
import sys


def uint(data, position):
    value = 0
    for shift in range(0, 70, 7):
        byte = data[position]
        position += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, position
    raise ValueError("unterminated unsigned LEB128")


def sections(data, first, end):
    position = first
    while position < end:
        kind = data[position]
        length, body = uint(data, position + 1)
        position = body + length
        if position > end:
            raise ValueError("invalid section extent")
        yield kind, body, position


def string(data, position):
    length, start = uint(data, position)
    return data[start : start + length], start + length


def core_sections(data):
    if data[:8] == b"\x00asm\x01\x00\x00\x00":
        return list(sections(data, 8, len(data)))
    assert data[:8] == b"\x00asm\x0d\x00\x01\x00"
    for kind, body, end in sections(data, 8, len(data)):
        if kind == 1 and data[body : body + 8] == b"\x00asm\x01\x00\x00\x00":
            return list(sections(data, body + 8, end))
    raise ValueError("no embedded core")


def imports(data, body):
    length, position = uint(data, body)
    count = 0
    for _ in range(length):
        _, position = string(data, position)
        _, position = string(data, position)
        kind, position = uint(data, position)
        if kind == 0:
            _, position = uint(data, position)
            count += 1
        elif kind in (1, 2):
            if kind == 1:
                _, position = uint(data, position)
            flags, position = uint(data, position)
            _, position = uint(data, position)
            if flags & 1:
                _, position = uint(data, position)
        elif kind in (3, 4):
            _, position = uint(data, position)
            _, position = uint(data, position)
        else:
            raise ValueError(f"unknown import kind {kind}")
    return count


with open(sys.argv[1], "rb") as source, mmap.mmap(source.fileno(), 0, access=mmap.ACCESS_READ) as wasm:
    wanted = {int(value) for value in sys.argv[2:]}
    imported = 0
    for kind, body, end in core_sections(wasm):
        if kind == 2:
            imported = imports(wasm, body)
            print("imported functions", imported)
        if kind == 0:
            name, position = string(wasm, body)
            print("custom", name, "bytes", end - body)
            if name != b"name":
                continue
            for subsection, start, _ in sections(wasm, position, end):
                if subsection != 1:
                    continue
                length, start = uint(wasm, start)
                for _ in range(length):
                    index, start = uint(wasm, start)
                    size, first = uint(wasm, start)
                    start = first + size
                    if index in wanted:
                        print(index, wasm[first:start].decode())
        if kind == 6:
            _, first = uint(wasm, body)
            if wasm[first:first + 3] == bytes([0x7f, 1, 0x41]):
                initialized, _ = uint(wasm, first + 3)
                print("first mutable i32 global", initialized)
        if kind == 10:
            print("code offset", hex(body), "bytes", end - body)
            length, position = uint(wasm, body)
            for index in range(imported, imported + length):
                size, first = uint(wasm, position)
                position = first + size
                if index in wanted:
                    print("function", index, hex(first), hex(position), size, wasm[first:first + 42].hex())
            assert position == end
