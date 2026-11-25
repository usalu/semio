"""🔎 Locate the oversized linked function in the originating Rust archive."""
import mmap
import os
import sys
from pathlib import Path

def integer(data, pos):
    value = shift = 0
    while True:
        byte = data[pos]
        pos += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, pos
        shift += 7

root = Path(sys.argv[1])
with (root / "p4-module-names.wasm").open("rb") as file:
    with mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ) as data:
        pos = 8
        while data[pos] != 10:
            size, pos = integer(data, pos + 1)
            pos += size
        size, pos = integer(data, pos + 1)
        count, pos = integer(data, pos)
        for _ in range(count):
            size, body = integer(data, pos)
            if size == 1278625:
                signature = data[body:body+16]
                print(signature.hex(), flush=True)
                break
            pos = body + size

for directory, _, files in os.walk(root / "builder/lane-wasm/b/wasm32-unknown-unknown/prototype/build"):
    for name in files:
        path = Path(directory) / name
        if not name.endswith(".rlib") or path.stat().st_size < 1278625:
            continue
        with path.open("rb") as file, mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ) as data:
            offset = data.find(signature)
            if offset >= 0:
                print(path, offset, flush=True)
