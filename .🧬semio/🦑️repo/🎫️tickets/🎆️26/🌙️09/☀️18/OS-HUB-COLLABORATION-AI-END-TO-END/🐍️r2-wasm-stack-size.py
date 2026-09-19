#!/usr/bin/env python3
"""🧊️ Reads the shadow-stack size every staged plugin component was linked with.

`wasm-ld --stack-first` puts the shadow stack at the bottom of linear memory and initialises the
mutable `__stack_pointer` global to its top, so the stack size IS that global's initial
`i32.const`. This scans each component's core-module bytes for the exact `i32.const` encodings of
the two sizes that matter — wasm-ld's default 1 MiB and the repo's declared 8 MiB
(`.cargo/config.toml` `[target.wasm32-wasip2]`) — and reports which one each component carries.
Ticket 26/09/18 slice R2 (`📓️a2-mcp-plugin-host-instance-open.md` §7.5).
"""

import pathlib
import sys


def leb128_i32_const(value: int) -> bytes:
    out = bytearray([0x41])
    more = True
    while more:
        byte = value & 0x7F
        value >>= 7
        if (value == 0 and not byte & 0x40) or (value == -1 and byte & 0x40):
            more = False
        else:
            byte |= 0x80
        out.append(byte)
    return bytes(out)


SIZES = {
    "1 MiB (wasm-ld default)": 1 << 20,
    "8 MiB (repo declared)": 8 << 20,
    "16 MiB": 16 << 20,
}


def main() -> int:
    root = pathlib.Path(sys.argv[1])
    rows = []
    for path in sorted(root.glob("*.wasm")):
        blob = path.read_bytes()
        found = [name for name, size in SIZES.items() if leb128_i32_const(size) in blob]
        rows.append((path.name, ", ".join(found) or "none of the known sizes", len(blob)))
    width = max(len(name) for name, _, _ in rows)
    for name, found, size in rows:
        print(f"{name:<{width}}  {size:>10}  {found}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
