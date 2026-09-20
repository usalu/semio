#!/usr/bin/env python3
"""📏️ HS1 — prices the frames of a macOS crash report's faulting thread, in bytes.

A `.ips` report names the frames but not their size, and "33 frames" alone cannot tell a deep
recursion from a handful of enormous frames. This reads each frame's function start
(`imageOffset - symbolLocation`), seeks to it in the Mach-O on disk and decodes the AArch64
prologue's stack adjustments — `sub sp, sp, #imm12{, lsl #12}` and the `mov xN, #imm; sub sp, sp, xN`
form LLVM emits for frames over 4 KiB — which IS the frame's local-area size.

Usage: 🐍️hs1-frame-sizes.py <report.ips> <binary>
"""
import json
import struct
import sys

report_path, binary_path = sys.argv[1], sys.argv[2]
raw = open(report_path).read()
document = json.loads(raw.split("\n", 1)[1])
thread = document["threads"][document["faultingThread"]]
image = open(binary_path, "rb").read()


def text_delta(data: bytes) -> int:
    """🧭️ Returns vmaddr - fileoff for __TEXT, so an image offset becomes a file offset."""
    magic, _, _, ncmds = struct.unpack_from("<IiiI", data, 0)
    assert magic == 0xFEEDFACF, hex(magic)
    cursor = 32
    for _ in range(ncmds):
        command, size = struct.unpack_from("<II", data, cursor)
        if command == 0x19:
            name = data[cursor + 8 : cursor + 24].rstrip(b"\0")
            vmaddr, _, fileoff = struct.unpack_from("<QQQ", data, cursor + 24)
            if name == b"__TEXT":
                return vmaddr - fileoff
        cursor += size
    raise SystemExit("no __TEXT segment")


delta = text_delta(image)


def prologue_bytes(file_offset: int, limit: int = 24) -> int:
    """🪜️ The stack a function reserves, read out of its AArch64 prologue.

    Frames over one page are not a plain `sub sp, sp, #imm`: LLVM opens them with
    `sub x9, sp, #TOTAL` and then a page-probing loop (`sub sp, sp, #4096; str xzr, [sp]; cmp sp, x9;
    b.ne`) followed by the remainder. Reading only the `sub sp` forms undercounts a 1.6 MiB frame as
    7 KB, so the probe register's immediate is what this trusts when it is present.
    """
    probe = 0
    direct = 0
    for index in range(limit):
        word = struct.unpack_from("<I", image, file_offset + index * 4)[0]
        if (word >> 23) & 0x1FF != 0b110100010:
            continue
        shifted = (word >> 22) & 1
        immediate = (word >> 10) & 0xFFF
        value = immediate << 12 if shifted else immediate
        source = (word >> 5) & 0x1F
        destination = word & 0x1F
        if source != 31:
            continue
        if destination == 31:
            direct += value
        elif probe == 0:
            probe = value
    return max(probe, direct)


print(f"faulting thread: {thread.get('name')}  frames: {len(thread['frames'])}")
print(f"{'#':>3}  {'frame bytes':>12}  symbol")
grand = 0
for index, frame in enumerate(thread["frames"]):
    symbol = frame.get("symbol") or f"img{frame['imageIndex']}+{frame['imageOffset']}"
    if frame.get("imageIndex") != 0 or "symbolLocation" not in frame:
        print(f"{index:>3}  {'(other image)':>12}  {symbol[:120]}")
        continue
    start = frame["imageOffset"] - frame["symbolLocation"]
    try:
        size = prologue_bytes(start)
    except Exception as error:  # noqa: BLE001
        print(f"{index:>3}  {'?':>12}  {symbol[:120]} ({error})")
        continue
    grand += size
    print(f"{index:>3}  {size:>12,}  {symbol[:120]}")
print(f"TOTAL measured prologue reservations: {grand:,} bytes")
