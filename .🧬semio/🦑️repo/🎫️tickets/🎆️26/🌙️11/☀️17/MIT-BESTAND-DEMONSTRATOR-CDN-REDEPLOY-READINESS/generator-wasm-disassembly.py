"""🔎️ Disassembles only selected exact core functions and resolves diagnostic data references."""
import mmap
import re
import subprocess
import sys
from pathlib import Path


def read(file, offset):
    """📖️ Read an unsigned LEB from the linked core."""
    result = shift = 0
    while True:
        byte = file[offset]
        offset += 1
        result |= (byte & 127) << shift
        shift += 7
        if byte < 128:
            return result, offset


def sections(raw):
    """📦️ Index the exact section byte ranges."""
    offset = 8
    while offset < len(raw):
        kind = raw[offset]
        length, start = read(raw, offset + 1)
        offset = start + length
        yield kind, start, offset


def cstring(raw, address, segment):
    """📜️ Read only human-readable text at a pointed data location."""
    base, start, length = segment
    if not base <= address < base + length:
        return None
    position = start + address - base
    match = re.match(rb"[ -~]{8,180}", raw[position : position + 180])
    if match and re.search(rb"[a-zA-Z]{4}", match[0]):
        return match[0].decode()
    return None


def main():
    """🧾️ Preserve raw instructions and exact data/call evidence for the requested functions."""
    tool, path, output, *flags = sys.argv[1:]
    functions = {int(value) for value in flags}
    with open(path, "rb") as stream, mmap.mmap(stream.fileno(), 0, access=mmap.ACCESS_READ) as raw:
        table = {kind: (start, end) for kind, start, end in sections(raw)}
        count, offset = read(raw, table[2][0])
        imported = 0
        for _ in range(count):
            for _ in range(2):
                length, start = read(raw, offset)
                offset = start + length
            assert raw[offset] == 0
            imported += 1
            _, offset = read(raw, offset + 1)
        segments = []
        count, offset = read(raw, table[11][0])
        for _ in range(count):
            mode, offset = read(raw, offset)
            base = None
            if mode != 1:
                if mode == 2:
                    _, offset = read(raw, offset)
                assert raw[offset] == 0x41
                base, offset = read(raw, offset + 1)
                assert raw[offset] == 0x0b
                offset += 1
            length, start = read(raw, offset)
            if base is not None:
                segments.append((base, start, length))
            offset = start + length
        count, offset = read(raw, table[10][0])
        for index in range(imported, imported + count):
            length, start = read(raw, offset)
            offset = start + length
            if index not in functions:
                continue
            disassembled = subprocess.run([tool, "-d", f"--start-address=0x{start:x}", f"--stop-address=0x{offset:x}", path], check=True, capture_output=True, text=True, timeout=15).stdout
            Path(output, f"w5-f{index}.txt").write_text(disassembled)
            references = []
            for value in set(map(int, re.findall(r"i32.const\s+(\d+)", disassembled))):
                for segment in segments:
                    if (direct := cstring(raw, value, segment)) is not None:
                        references.append((value, direct))
                    base, first, extent = segment
                    if base <= value < base + extent - 4:
                        p = first + value - base
                        pointed = int.from_bytes(raw[p:p + 4], "little")
                        for data in segments:
                            if (indirect := cstring(raw, pointed, data)) is not None:
                                references.append((value, indirect))
            calls = sorted({int(value) for value in re.findall(r"\bcall\s+(\d+)", disassembled)})
            print(f"{index} [{start:#x},{offset:#x}) calls {calls} strings {sorted(set(references))}")


if __name__ == "__main__":
    main()
