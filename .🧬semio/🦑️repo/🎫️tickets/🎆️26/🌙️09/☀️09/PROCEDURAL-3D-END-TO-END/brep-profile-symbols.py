"""🔬 Resolve the Chromium worker profile against the guest's matching Wasm code section."""
import collections
import gzip
import hashlib
import json
import pathlib
import re
import sys

root = pathlib.Path(__file__).parent / "🗑️generated/bridge-current"
base = "semio_s_plugin_flow_brep_component"
samples = collections.Counter()
for line in (root / sys.argv[1]).read_text().splitlines():
    if line.startswith("[DEBUG] Worker CPU samples "):
        for count, name, url in json.loads(line.split("samples ", 1)[1]):
            function = re.fullmatch(r"wasm-function\[(\d+)\]", name)
            if function and base in url:
                samples[int(function[1])] += count

def integer(data, offset):
    result = shift = 0
    while True:
        value = data[offset]
        offset += 1
        result |= (value & 127) << shift
        if value < 128:
            return result, offset
        shift += 7

def sections(data, pos, end):
    while pos < end:
        kind = data[pos]
        size, start = integer(data, pos + 1)
        pos = start + size
        yield kind, start, pos

def code(data):
    return next(hashlib.sha256(memoryview(data)[a:b]).hexdigest() for k,a,b in sections(data, 8, len(data)) if k == 10)

def resolve():
    with gzip.open(root / "modules" / base / (base+".core.wasm.gz"), "rb") as f:
        data = f.read()
    if code(data) != code((root / "brep-profile.wasm").read_bytes()):
        raise ValueError("CPU profile and symbols name different executable code")
    for kind, start, end in sections(data, 8, len(data)):
        if kind != 0 or data[start:start+5] != b"\x04name":
            continue
        for sub, pos, limit in sections(data, start+5, end):
            if sub != 1:
                continue
            count, pos = integer(data, pos)
            for _ in range(count):
                idx, pos = integer(data, pos)
                size, pos = integer(data, pos)
                if idx in samples:
                    yield samples[idx], idx, data[pos:pos+size].decode()
                pos += size

if not samples:
    raise ValueError("No sampled Wasm functions")
for row in sorted(resolve(), reverse=True):
    print(*row)
