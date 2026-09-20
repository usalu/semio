#!/usr/bin/env python3
"""🧹️ Scan the wgpu renderer files the two killed workers (O3, WG6) were editing for raw control
bytes and rewrite each one as its four-character escape text (``\\x00``, ``\\x7f``). A control byte
in a Rust source file makes ``file(1)`` answer ``data`` and makes plain ``grep`` fall into binary
mode, so the corruption is invisible to every normal search; the four that were found sat inside one
docstring that transcribed a JSON schema character class."""

import pathlib
import sys

ELEMENTS = pathlib.Path(
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
)

TARGETS = (
    "🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs",
    "🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs",
    "🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs",
    "🔐️HubSignIn/🧪️tests/🔬️wgpu-unit/🦀️.rs",
    "🏘️SpaceBrowser/🧪️tests/🔬️wgpu-unit/🦀️.rs",
    "🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs",
    "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
    "🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs",
)


TEXT_CONTROL_BYTES = frozenset(b"\t\n\r")

ESCAPES = {byte: ("\\x%02x" % byte).encode("ascii") for byte in range(0x20)}
ESCAPES[0x7F] = rb"\x7f"
for keep in TEXT_CONTROL_BYTES:
    del ESCAPES[keep]


def main() -> int:
    repaired = 0
    for relative in TARGETS:
        path = ELEMENTS / relative
        raw = path.read_bytes()
        found = {byte: raw.count(bytes([byte])) for byte in ESCAPES if bytes([byte]) in raw}
        if not found:
            print(f"    clean  {relative}")
            continue
        for byte, count in sorted(found.items()):
            offset = raw.find(bytes([byte]))
            print("%5d 0x%02x  %s (first at line %d)" % (count, byte, relative, raw[:offset].count(b"\n") + 1))
            raw = raw.replace(bytes([byte]), ESCAPES[byte])
            repaired += count
        path.write_bytes(raw)
    print(f"repaired {repaired} byte(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
