"""🧊 Compact a linked WASI core and verify its component interface before encoding."""
from pathlib import Path
import hashlib
import mmap
import os
import subprocess
import sys
import time

limit = 1_000_000

def number(data, offset):
    result = 0
    for shift in range(0, 35, 7):
        byte = data[offset]
        offset += 1
        result |= (byte & 127) << shift
        if byte < 128:
            return result, offset
    raise ValueError("invalid unsigned 32-bit LEB128")

def inventory(path):
    with path.open("rb") as file, mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ) as data:
        if data[:8] != b"\0asm\x01\0\0\0":
            raise ValueError("expected a WebAssembly core")
        count = None
        metadata = {}
        exports = []
        i = 8
        while i < len(data):
            section = data[i]
            size, start = number(data, i + 1)
            i = start + size
            if i > len(data):
                raise ValueError("incomplete WebAssembly section")
            if section == 3:
                count, _ = number(data, start)
            elif section == 0:
                length, at = number(data, start)
                name = data[at:at+length].decode()
                if name.startswith("component-type"):
                    digest = hashlib.sha256(data[at+length:i]).hexdigest()
                    metadata.setdefault(name, []).append(digest)
            elif section == 7:
                total, at = number(data, start)
                for _ in range(total):
                    length, at = number(data, at)
                    name = data[at:at+length]
                    at += length
                    kind = data[at]
                    _, at = number(data, at+1)
                    exports.append((name, kind))
        if count is None or not metadata or not exports:
            raise ValueError("component interface or core function section is missing")
        return count, metadata, sorted(exports)

def wait_for_core(source):
    deadline = time.monotonic() + 1200
    while time.monotonic() < deadline:
        try:
            return inventory(source)
        except (FileNotFoundError, IndexError, ValueError):
            time.sleep(2)
    raise TimeoutError("no complete input core was produced")

if __name__ == "__main__":
    source, target, optimizer = map(Path, sys.argv[1:4])
    original, wit, exports = wait_for_core(source)
    print(f"[DEBUG] Input core has {original} defined functions, {len(exports)} exports and {len(wit)} interface section names", flush=True)
    env = {**os.environ, "BINARYEN_CORES": "1"}
    current = source
    for phase, flags in enumerate([["--duplicate-function-elimination"], ["--merge-similar-functions", "--duplicate-function-elimination", "--remove-unused-module-elements"], ["--inlining-optimizing", "--duplicate-function-elimination", "--remove-unused-module-elements"]], 1):
        output = target.with_suffix(f".pass{phase}.wasm")
        subprocess.run([optimizer, current, "--all-features", *flags, "-o", output], env=env, check=True)
        defined, metadata, actual_exports = inventory(output)
        if metadata != wit or actual_exports != exports:
            raise ValueError("compaction changed the linked component interface")
        print(f"[DEBUG] Pass {phase} retained {defined} defined functions and {output.stat().st_size} bytes", flush=True)
        if current != source:
            current.unlink()
        if defined < limit:
            output.replace(target)
            break
        current = output
    else:
        raise ValueError("compacted core still exceeds the component function limit")
