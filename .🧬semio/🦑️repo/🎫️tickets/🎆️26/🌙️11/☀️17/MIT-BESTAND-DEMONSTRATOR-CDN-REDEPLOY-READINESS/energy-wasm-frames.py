"""🔎️ Match stripped Wasm function indices to references to known Rust diagnostic strings."""
import mmap
import sys


def read(number, offset):
    result = shift = 0
    while True:
        byte = number[offset]
        offset += 1
        result |= (byte & 127) << shift
        shift += 7
        if byte < 128:
            return result, offset


def sleb(value, width=0):
    encoded = bytearray()
    while value >= 64 or len(encoded) + 1 < width:
        encoded.append((value & 127) | 128)
        value >>= 7
    encoded.append(value)
    return bytes(encoded)


def sections(module):
    offset = 8
    while offset < len(module):
        kind = module[offset]
        length, start = read(module, offset + 1)
        yield kind, start, start + length
        offset = start + length


with open(sys.argv[1], "rb") as wasm, mmap.mmap(wasm.fileno(), 0, access=mmap.ACCESS_READ) as module:
    parts = {kind: (start, end) for kind, start, end in sections(module)}
    declared, _ = read(module, parts[3][0])
    imports, offset = read(module, parts[2][0])
    imported = 0
    for _ in range(imports):
        for _ in range(2):
            length, start = read(module, offset)
            offset = start + length
        assert module[offset] == 0, "expected only functions in the core import section"
        _, offset = read(module, offset + 1)
        imported += 1
    markers = tuple(text.encode() for text in sys.argv[2:])
    addresses = {}
    segments, offset = read(module, parts[11][0])
    for _ in range(segments):
        mode, offset = read(module, offset)
        base = None
        if mode != 1:
            if mode == 2:
                _, offset = read(module, offset)
            assert module[offset] == 0x41
            base, offset = read(module, offset + 1)
            assert module[offset] == 0x0b
            offset += 1
        size, start = read(module, offset)
        offset = start + size
        if base is not None:
            for marker in markers:
                at = module.find(marker, start, offset)
                while at != -1:
                    address = base + at - start
                    values = [address]
                    pointer = address.to_bytes(4, "little")
                    reference = module.find(pointer, start, offset)
                    while reference != -1:
                        values.append(base + reference - start)
                        reference = module.find(pointer, reference + 1, offset)
                    addresses.setdefault(marker, []).extend(b"\x41" + sleb(value, width) for value in values for width in (0, 5))
                    addresses[marker].extend(b"\x42" + sleb((size << 32) | address, width) for size in range(len(marker), len(marker) + 2) for width in (0, 10))
                    at = module.find(marker, at + len(marker), offset)
    funcs, offset = read(module, parts[10][0])
    assert funcs == declared
    results = {marker: [] for marker in addresses}
    for index in range(imported, imported + funcs):
        size, start = read(module, offset)
        offset = start + size
        for marker, candidates in addresses.items():
            if any(module.find(candidate, start, offset) != -1 for candidate in candidates):
                results[marker].append(index)
    for marker, matches in results.items():
        print(marker.decode(), "=>", matches)
