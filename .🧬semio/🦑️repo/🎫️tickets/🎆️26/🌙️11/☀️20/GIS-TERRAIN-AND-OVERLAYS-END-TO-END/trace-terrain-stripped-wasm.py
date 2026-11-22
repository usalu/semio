import mmap
from pathlib import Path
import re
import sys


def leb(data, offset, signed=False):
    value, shift = 0, 0
    while True:
        byte = data[offset]
        offset += 1
        value |= (byte & 127) << shift
        shift += 7
        if not byte & 128:
            return (value - (1 << shift) if signed and byte & 64 else value), offset
        if shift > 70:
            raise ValueError("long LEB")


def name(data, offset):
    size, start = leb(data, offset)
    return bytes(data[start:start + size]), start + size


def sections(data):
    offset = 8
    while offset < len(data):
        kind = data[offset]
        size, start = leb(data, offset + 1)
        yield kind, start
        offset = start + size


def import_functions(data, start):
    count, p = leb(data, start)
    functions = 0
    for _ in range(count):
        _, p = name(data, p)
        _, p = name(data, p)
        kind = data[p]
        p += 1
        if kind == 0:
            _, p = leb(data, p)
            functions += 1
        elif kind in (1, 2):
            p += kind == 1
            flags, p = leb(data, p)
            _, p = leb(data, p)
            if flags & 1:
                _, p = leb(data, p)
        elif kind == 3:
            p += 2
        elif kind == 4:
            _, p = leb(data, p)
            _, p = leb(data, p)
        else:
            raise ValueError(kind)
    return functions


def detail(data, indices):
    bodies, regions, exports = {}, [], {}
    imports = 0
    for kind, start in sections(data):
        if kind == 2:
            imports = import_functions(data, start)
        elif kind == 7:
            count, cursor = leb(data, start)
            for _ in range(count):
                label, cursor = name(data, cursor)
                kind = data[cursor]
                index, cursor = leb(data, cursor + 1)
                if kind == 0 and index in indices:
                    exports[index] = label.decode()
        elif kind == 10:
            count, cursor = leb(data, start)
            for index in range(imports, imports + count):
                size, cursor = leb(data, cursor)
                if index in indices:
                    bodies[index] = cursor, cursor + size
                cursor += size
        elif kind == 11:
            count, cursor = leb(data, start)
            for _ in range(count):
                flags, cursor = leb(data, cursor)
                address = None
                if flags != 1:
                    if flags == 2:
                        _, cursor = leb(data, cursor)
                    if data[cursor] != 0x41:
                        raise ValueError("unexpected data offset")
                    address, cursor = leb(data, cursor + 1, signed=True)
                    if data[cursor] != 0x0b:
                        raise ValueError("unexpected constant terminator")
                    cursor += 1
                size, cursor = leb(data, cursor)
                if address is not None:
                    regions.append((address, address + size, cursor))
                cursor += size
    return bodies, regions, exports


def citations(data, body, regions):
    start, end = body
    seen = set()
    for offset in range(start, end):
        if data[offset] != 0x41:
            continue
        try:
            value, p = leb(data, offset + 1, signed=True)
        except ValueError:
            continue
        for low, high, memory in regions:
            if not low <= value < high:
                continue
            absolute = memory + value - low
            text = bytes(data[absolute:absolute + min(160, high - value)])
            printable = re.match(rb"[^\x00-\x09\x0b-\x1f\x7f]{14,}", text)
            if not printable:
                continue
            source = printable.group().decode(errors="ignore")
            if not re.search(r"[a-z]{4}", source) or value in seen:
                continue
            seen.add(value)
            yield offset - start, value, source[:145]


if __name__ == "__main__":
    path = Path(sys.argv[1])
    indices = [int(index) for index in sys.argv[2:]]
    with path.open("rb") as file, mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ) as wasm:
        bodies, regions, exports = detail(wasm, indices)
        for index in indices:
            span = bodies[index]
            print(index, "length", span[1] - span[0], "name", exports.get(index))
            for location, address, label in list(citations(wasm, span, regions))[:35]:
                print(f"  +{location} 0x{address:x} {label}")
