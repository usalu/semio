"""🔐 Confirm a raw browser factory's core bytes against its linked WASI component."""
from collections import Counter
import hashlib
import json
from pathlib import Path
import sys


def byte(source):
    value = source.read(1)
    if not value:
        raise ValueError("Truncated Wasm section")
    return value[0]


def size(source):
    length = 0
    for shift in range(0, 35, 7):
        unit = byte(source)
        length |= (unit & 127) << shift
        if unit < 128 and length < 1 << 32:
            return length
    raise ValueError("Invalid Wasm extent")


def sections(source, start, end):
    while start < end:
        source.seek(start)
        kind = byte(source)
        length = size(source)
        body = source.tell()
        stop = body + length
        if stop > end:
            raise ValueError("Wasm section overflows its owner")
        yield kind, start, body, stop
        start = stop


def hash_range(source, hash_object, start, end):
    source.seek(start)
    while start < end:
        part = source.read(min(end - start, 1024 * 1024))
        if not part:
            raise ValueError("Wasm source shortened during observation")
        hash_object.update(part)
        start += len(part)


def cores(source, start, stop):
    source.seek(start)
    header = source.read(8)
    if header == b"\x00asm\x0d\x00\x01\x00":
        for kind, _, body, end in sections(source, start + 8, stop):
            if kind in (1, 4):
                yield from cores(source, body, end)
    elif header == b"\x00asm\x01\x00\x00\x00":
        digest = hashlib.sha256(header)
        for kind, head, body, end in sections(source, start + 8, stop):
            if kind == 0:
                source.seek(body)
                length = size(source)
                if source.tell() + length > end:
                    raise ValueError("Invalid Wasm name section extent")
                if source.read(min(length, 6)) == b".debug":
                    continue
            hash_range(source, digest, head, end)
        yield digest.hexdigest()
    else:
        raise ValueError("Expected core module or WASI component header")


raw, factory = (Path(path) for path in sys.argv[1:])
with raw.open("rb") as file:
    expected = Counter(cores(file, 0, raw.stat().st_size))
observed = {}
for core in sorted(factory.glob("*.core*.wasm")):
    digest = hashlib.sha256()
    with core.open("rb") as file:
        hash_range(file, digest, 0, core.stat().st_size)
    observed[core.name] = digest.hexdigest()
if not observed or Counter(observed.values()) != expected:
    raise SystemExit(json.dumps({"expected": expected, "observed": observed}))
print(json.dumps(observed))
