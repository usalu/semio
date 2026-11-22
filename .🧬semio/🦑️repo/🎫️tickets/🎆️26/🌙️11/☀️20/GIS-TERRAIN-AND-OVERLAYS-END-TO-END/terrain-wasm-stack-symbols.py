from pathlib import Path
import mmap
import re
import sys


def integer(data, at):
    result = 0
    for shift in range(0, 70, 7):
        digit = data[at]
        at += 1
        result |= (digit & 127) << shift
        if not digit & 128:
            return result, at
    raise ValueError("Unterminated WASM integer")


def name(data, at):
    extent, at = integer(data, at)
    return data[at:at + extent].decode(), at + extent


def inspect_module(data, wanted):
    assert data[:4] == b"\0asm"
    size = len(data)
    at = 8
    named = {}
    while at < size:
        section = data[at]
        extent, start = integer(data, at + 1)
        at = start + extent
        if section == 10:
            count, body = integer(data, start)
            print("Code", count, start, extent)
            for index in range(count):
                extent, body = integer(data, body)
                for offset in wanted:
                    if body <= offset < body + extent:
                        fn = wanted[offset]
                        print(f"Function {fn} body_index {index} relative +{offset - body} bytes {extent}")
                body += extent
        if section != 0:
            continue
        title, cursor = name(data, start)
        print("Custom", title, at - cursor)
        if title != "name":
            continue
        while cursor < at:
            subsection = data[cursor]
            span, field = integer(data, cursor + 1)
            cursor = field + span
            if subsection == 1:
                count, field = integer(data, field)
                for _ in range(count):
                    index, field = integer(data, field)
                    symbol, field = name(data, field)
                    if index in wanted.values():
                        named[index] = symbol
    return named


if __name__ == "__main__":
    wasm, logs = map(Path, sys.argv[1:])
    frames = re.findall(r"wasm-function\[(\d+)\]:0x([0-9a-f]+)", logs.read_text(errors="replace"))
    wanted = {int(offset, 16): int(index) for index, offset in frames}
    with wasm.open("rb") as opened, mmap.mmap(opened.fileno(), 0, access=mmap.ACCESS_READ) as data:
        names = inspect_module(data, wanted)
    for offset, index in wanted.items():
        print(index, hex(offset), names.get(index, "unnamed"))
