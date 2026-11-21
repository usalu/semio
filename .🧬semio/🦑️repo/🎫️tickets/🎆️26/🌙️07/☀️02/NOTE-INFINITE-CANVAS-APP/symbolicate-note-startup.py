"""🔬 Locate the Wasm code bodies sampled during Note browser startup."""
from argparse import ArgumentParser
from mmap import ACCESS_READ, mmap
from pathlib import Path


def leb(data, offset):
    value = shift = 0
    while True:
        byte = data[offset]
        offset += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, offset
        shift += 7


def span(data, offset):
    size, start = leb(data, offset)
    return start, start + size


def sections(data, offset, end):
    offset += 8
    while offset < end:
        tag = data[offset]
        start, stop = span(data, offset + 1)
        yield tag, start, stop
        offset = stop


def code_bodies(data, start):
    count, offset = leb(data, start)
    for i in range(count):
        lo, hi = span(data, offset)
        yield i, lo, hi
        offset = hi


def signed_leb(data, offset):
    start = offset
    value, offset = leb(data, offset)
    return (value - (1 << (7 * (offset - start))) if data[offset - 1] & 64 else value), offset


def memory_segments(data):
    _, start, _ = next(section for section in sections(data, 0, len(data)) if section[0] == 11)
    count, position = leb(data, start)
    segments = []
    for _ in range(count):
        flags, position = leb(data, position)
        address = None
        if flags == 2:
            _, position = leb(data, position)
        if flags in (0, 2):
            if data[position] != 65:
                raise ValueError("Expected an i32.const offset in the profiled core")
            address, position = signed_leb(data, position + 1)
            if data[position] != 11:
                raise ValueError("Unexpected data offset expression")
            position += 1
        first, position = span(data, position)
        if address is not None:
            segments.append((address, data[first:position]))
    return segments


def source_literals(body, segments):
    def at(address, size):
        for base, segment in segments:
            if base <= address < base + len(segment):
                start = address - base
                return segment[start:start + size]
        return b""

    def read(address, size=None):
        raw = at(address, size or 170)
        if size is None:
            raw = raw.split(b"\0", 1)[0]
        word = raw.decode(errors="ignore")
        if sum(char.isprintable() for char in word) < len(word) * 0.92:
            return ""
        if len(word) > 9 and any(test in word for test in (".rs", "-0.1.0/", "original", "viewport", "retained", "surface")):
            return word.strip()[:170]
        return ""

    literals = set()
    for index, byte in enumerate(body[:-5]):
        if byte != 65:
            continue
        address, _ = signed_leb(body, index + 1)
        if address < 10000:
            continue
        word = read(address)
        if word:
            literals.add(word)
        raw = at(address, 12)
        if len(raw) != 12:
            continue
        ptr, size, row = (int.from_bytes(raw[i:i+4], "little") for i in (0, 4, 8))
        if 8 < size < 400:
            word = read(ptr, size)
            if word:
                literals.add(f"{word} (line {row})" if word.endswith(".rs") else word)
    return sorted(literals)


def main():
    parser = ArgumentParser()
    parser.add_argument("module", type=Path)
    parser.add_argument("--strings", action="store_true")
    parser.add_argument("offsets", nargs="+", type=int)
    cli = parser.parse_args()
    offsets = sorted(cli.offsets)
    with cli.module.open("rb") as stream, mmap(stream.fileno(), 0, access=ACCESS_READ) as data:
        _, code, _ = next(section for section in sections(data, 0, len(data)) if section[0] == 10)
        segments = memory_segments(data) if cli.strings else []
        for _, first, last in code_bodies(data, code):
            while offsets and first <= offsets[0] < last:
                offset = offsets.pop(0)
                print("[DEBUG] frame", offset, "body", first, "size", last-first, "initial", data[first:first+28].hex())
                for literal in source_literals(data[first:last], segments)[:24]:
                    print(" ", literal)
            if not offsets:
                return
    raise ValueError(f"Frame offsets outside Wasm code bodies: {offsets}")


if __name__ == "__main__":
    main()
