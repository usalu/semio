"""🔍 Relate profiled stripped guest functions to unique archive source literals."""

import collections
import json
import mmap
import re
import struct
import sys


def leb(body, offset, signed=False):
    number = shift = 0
    while True:
        byte = body[offset]
        offset += 1
        number |= (byte & 127) << shift
        shift += 7
        if byte < 128:
            return (number - (1 << shift) if signed and byte & 64 else number), offset


def signed_leb(number):
    parts = bytearray()
    while True:
        byte = number & 127
        number >>= 7
        if (number == 0 and not byte & 64) or (number == -1 and byte & 64):
            return bytes(parts + bytes([byte]))
        parts.append(byte | 128)


def padded_leb(number, width):
    return bytes((number >> (7 * byte)) & 255 if byte == width - 1 else
                 ((number >> (7 * byte)) & 127) | 128 for byte in range(width))


def string_end(body, offset):
    length, start = leb(body, offset)
    return start + length


def code_count_from_imports(body, section):
    count, offset = leb(body, section)
    for _ in range(count):
        offset = string_end(body, string_end(body, offset))
        kind = body[offset]
        if kind != 0:
            raise ValueError(f"unexpected non-function Wasm core import {kind}")
        _, offset = leb(body, offset + 1)
    return count


def data_regions(body, section):
    count, offset = leb(body, section)
    for _ in range(count):
        flags, offset = leb(body, offset)
        if flags == 2:
            _, offset = leb(body, offset)
        address = None
        if flags in (0, 2):
            if body[offset] != 0x41:
                raise ValueError("expected i32.const data address")
            address, offset = leb(body, offset + 1, signed=True)
            if body[offset] != 0x0B:
                raise ValueError("expected data address end")
            offset += 1
        length, offset = leb(body, offset)
        if address is not None:
            yield address, offset, offset + length
        offset += length


def addresses(body, regions, pattern):
    for address, start, end in regions:
        position = body.find(pattern, start, end)
        while position != -1:
            yield address + position - start
            position = body.find(pattern, position + 1, end)


wasm, profile = sys.argv[1:3]
markers = [argument for argument in sys.argv[3:] if not argument.startswith("frame=")]
requested_frames = [int(argument.removeprefix("frame=")) for argument in sys.argv[3:] if argument.startswith("frame=")]
with open(profile) as source:
    samples = json.load(source)
nodes = {node["id"]: node for node in samples["nodes"]}
parents = {child: node["id"] for node in nodes.values() for child in node.get("children", [])}
counts = collections.Counter()
for sample in samples["samples"]:
    calls = set()
    while sample:
        frame = nodes[sample]["callFrame"]["functionName"]
        if match := re.fullmatch(r"wasm-function\[(\d+)\]", frame):
            calls.add(int(match[1]))
        sample = parents.get(sample)
    counts.update(calls)

with open(wasm, "rb") as source, mmap.mmap(source.fileno(), 0, access=mmap.ACCESS_READ) as body:
    sections = {}
    offset = 8
    while offset < len(body):
        kind = body[offset]
        length, start = leb(body, offset + 1)
        sections[kind] = (start, start + length)
        offset = start + length
    imports = code_count_from_imports(body, sections[2][0])
    regions = list(data_regions(body, sections[11][0]))
    count, offset = leb(body, sections[10][0])
    functions = []
    for _ in range(count):
        length, start = leb(body, offset)
        offset = start + length
        functions.append((start, offset))
    for frame in requested_frames:
        start, end = functions[frame - imports]
        constants = set()
        for position in range(start, end - 1):
            if body[position] == 0x41:
                number, _ = leb(body, position + 1, signed=True)
                constants.add(number)
        literals = set()
        for constant in constants:
            for address, first, last in regions:
                if not address <= constant < address + last - first:
                    continue
                offset = first + constant - address
                literal = body[offset:min(offset + 180, last)].decode(errors="replace")
                prefix = "".join(re.match(r"[^\0-\x1f\x7f\ufffd]*", literal)[0])
                if len(prefix) >= 12 and any(character.isalnum() for character in prefix):
                    literals.add(prefix)
        print(f"[DEBUG] frame {frame} (samples {counts[frame]}): {sorted(literals)[:24]}")
    for marker in markers:
        literal = set(addresses(body, regions, marker.encode()))
        references = set(literal)
        for address in literal:
            references.update(addresses(body, regions, struct.pack("<I", address)))
        patterns = {b"\x41" + encoding for address in references
                    for encoding in (signed_leb(address), padded_leb(address, 5))}
        for address in literal:
            pair = (len(marker.encode()) << 32) | address
            patterns.update(b"\x42" + encoding for encoding in (signed_leb(pair), padded_leb(pair, 10)))
        hits = []
        for index, (start, end) in enumerate(functions, imports):
            if any(body.find(pattern, start, end) != -1 for pattern in patterns):
                hits.append((index, counts[index]))
        print(f"[DEBUG] {marker}: {sorted(literal)} {sorted(hits, key=lambda pair: -pair[1])[:20]}")
