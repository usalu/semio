"""🔎️ Finds exact Wasm instructions referencing a retained native refusal."""
import importlib.util
import mmap
import re
import struct
import sys
from pathlib import Path


def signed_leb(value):
    """🔢️ Encodes a nonnegative Wasm i32 immediate."""
    result = bytearray()
    while value >= 64:
        result.append((value & 127) | 128)
        value >>= 7
    result.append(value)
    return bytes(result)


def main():
    """🧾️ Indexes active data references and reports enclosing code functions."""
    source = Path(__file__).with_name("generator-wasm-disassembly.py")
    spec = importlib.util.spec_from_file_location("wasm", source)
    wasm = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(wasm)
    with open(sys.argv[1], "rb") as stream, mmap.mmap(stream.fileno(), 0, access=mmap.ACCESS_READ) as raw:
        table = {kind: (start, end) for kind, start, end in wasm.sections(raw)}
        count, offset = wasm.read(raw, table[11][0])
        segments = []
        for _ in range(count):
            mode, offset = wasm.read(raw, offset)
            base = None
            if mode != 1:
                if mode == 2:
                    _, offset = wasm.read(raw, offset)
                assert raw[offset] == 0x41
                base, offset = wasm.read(raw, offset + 1)
                assert raw[offset] == 0x0B
                offset += 1
            length, start = wasm.read(raw, offset)
            offset = start + length
            if base is not None:
                segments.append((base, start, offset))
        needle = sys.argv[2].encode()
        addresses = [base + position - start for base, start, end in segments
                     if (position := raw.find(needle, start, end)) >= 0]
        print("[DEBUG] Diagnostic string addresses", addresses)
        pointers = set(addresses)
        for address in addresses:
            pattern = re.compile(re.escape(struct.pack("<I", address)))
            for base, start, end in segments:
                pointers.update(base + match.start() - start for match in pattern.finditer(raw, start, end))
        start, end = table[10]
        references = {}
        immediates = [(b"\x41", pointer) for pointer in pointers]
        immediates.extend((b"\x42", (len(needle) << 32) | pointer) for pointer in addresses)
        for opcode, pointer in immediates:
            encoded = signed_leb(pointer)
            for size in range(len(encoded), 6 if opcode == b"\x41" else 11):
                pattern = re.compile(re.escape(opcode + encoded))
                references.update((match.start(), pointer) for match in pattern.finditer(raw, start, end))
                encoded = encoded[:-1] + bytes([encoded[-1] | 128, 0])
        imports, _ = wasm.read(raw, table[2][0])
        count, offset = wasm.read(raw, start)
        for function in range(imports, imports + count):
            length, start = wasm.read(raw, offset)
            offset = start + length
            for location, pointer in references.items():
                if start <= location < offset:
                    print(f"[DEBUG] Function {function} [{start:#x},{offset:#x}) immediate {location:#x} data {pointer:#x}")


if __name__ == "__main__":
    main()
