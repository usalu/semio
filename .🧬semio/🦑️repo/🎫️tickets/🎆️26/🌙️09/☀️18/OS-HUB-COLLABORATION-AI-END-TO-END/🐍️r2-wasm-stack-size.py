#!/usr/bin/env python3
"""🧊️ Reads the shadow-stack size every staged plugin component was linked with.

`wasm-ld --stack-first` places the shadow stack at the bottom of linear memory and initialises the
mutable `__stack_pointer` global — global 0 of the linked core module — to its top, so that global's
initial `i32.const` IS the `-zstack-size` the component carries. This walks each component's
embedded core modules (`\\0asm\\x01\\x00\\x00\\x00`), parses the largest one's global section and
reports that value, which is what `.cargo/config.toml` `[target.wasm32-wasip2]` sets and what
wasm-ld defaults to 1 MiB without. Ticket 26/09/18 slice R2
(`📓️a2-mcp-plugin-host-instance-open.md` §7.5).
"""

from __future__ import annotations

import pathlib
import sys

CORE_MAGIC = b"\x00asm\x01\x00\x00\x00"


def uleb(blob: memoryview, at: int) -> tuple[int, int]:
    value = 0
    shift = 0
    while True:
        byte = blob[at]
        at += 1
        value |= (byte & 0x7F) << shift
        if not byte & 0x80:
            return value, at
        shift += 7


def sleb(blob: memoryview, at: int) -> tuple[int, int]:
    value = 0
    shift = 0
    while True:
        byte = blob[at]
        at += 1
        value |= (byte & 0x7F) << shift
        shift += 7
        if not byte & 0x80:
            if byte & 0x40:
                value -= 1 << shift
            return value, at


def first_global(blob: memoryview, start: int) -> int | None:
    at = start + len(CORE_MAGIC)
    while at < len(blob):
        section = blob[at]
        at += 1
        try:
            size, at = uleb(blob, at)
        except IndexError:
            return None
        if section == 6:
            count, cursor = uleb(blob, at)
            if count == 0:
                return None
            valtype = blob[cursor]
            mutable = blob[cursor + 1]
            cursor += 2
            if valtype != 0x7F or mutable != 0x01 or blob[cursor] != 0x41:
                return None
            value, _ = sleb(blob, cursor + 1)
            return value
        if section > 13:
            return None
        at += size
    return None


def main() -> int:
    root = pathlib.Path(sys.argv[1])
    rows = []
    for path in sorted(root.glob("*.wasm")):
        blob = memoryview(path.read_bytes())
        raw = bytes(blob)
        best: tuple[int, int] | None = None
        at = raw.find(CORE_MAGIC)
        starts = []
        while at != -1:
            starts.append(at)
            at = raw.find(CORE_MAGIC, at + 1)
        for index, start in enumerate(starts):
            end = starts[index + 1] if index + 1 < len(starts) else len(raw)
            if best is None or end - start > best[1]:
                best = (start, end - start)
        stack = first_global(blob, best[0]) if best else None
        rows.append((path.name, stack))
    width = max(len(name) for name, _ in rows)
    for name, stack in rows:
        if stack is None:
            print(f"{name:<{width}}  unreadable (no global 0 in the largest core module)")
        else:
            print(f"{name:<{width}}  {stack:>10} B  {stack / (1 << 20):.2f} MiB")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
