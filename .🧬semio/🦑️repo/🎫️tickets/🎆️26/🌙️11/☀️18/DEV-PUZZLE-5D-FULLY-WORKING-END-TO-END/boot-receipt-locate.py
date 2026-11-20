import mmap
import re
import struct
import sys

needle = sys.argv[2].encode() if len(sys.argv) > 2 else b"original lifecycle child receipt differs from its whole producer receipt"
pattern = re.compile(rb"[\x48-\x4f]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d].{4}", re.S)
with open(sys.argv[1], "rb") as executable, mmap.mmap(executable.fileno(), 0, access=mmap.ACCESS_READ) as binary:
    program, size, count = struct.unpack_from("<Q10xHH", binary, 40)
    sections = [struct.unpack_from("<IIQQQQIIQQ", binary, program + row * size) for row in range(count)]
    strings = [found.start() for found in re.finditer(re.escape(needle), binary)]
    virtual = {address + index - start for index in strings for _, kind, _, address, start, extent, *_ in sections if kind == 1 and start <= index < start + extent}
    for _, kind, flags, address, start, extent, *_ in sections:
        if kind != 1 or flags & 4 == 0:
            continue
        for instruction in pattern.finditer(binary, start, start + extent):
            offset = address + instruction.start() - start
            target = offset + 7 + struct.unpack_from("<i", binary, instruction.start() + 3)[0]
            if target in virtual:
                print(f"{offset:#x} points to {target:#x}")
