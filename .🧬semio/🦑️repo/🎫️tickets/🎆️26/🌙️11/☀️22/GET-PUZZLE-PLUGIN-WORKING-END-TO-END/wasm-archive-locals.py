"""📏 Measure declared locals in every Wasm function in a Rust archive."""
import sys
from pathlib import Path

def integer(data, pos):
    value = shift = 0
    while True:
        byte = data[pos]
        pos += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, pos
        shift += 7

def function_locals(module):
    pos = 8
    while pos < len(module):
        section = module[pos]
        length, start = integer(module, pos + 1)
        pos = start + length
        if section != 10:
            continue
        count, offset = integer(module, start)
        for _ in range(count):
            length, body = integer(module, offset)
            groups, ptr = integer(module, body)
            total = 0
            for _ in range(groups):
                size, ptr = integer(module, ptr)
                total += size
                ptr += 1
            yield total
            offset = body + length

archive = Path(sys.argv[1]).read_bytes()
assert archive.startswith(b'!<arch>\n')
pos = 8
count = peak = 0
while pos < len(archive):
    size = int(archive[pos+48:pos+58])
    body = pos + 60
    if archive[body:body+4] == b'\x00asm':
        for locals_count in function_locals(memoryview(archive)[body:body+size]):
            count += 1
            peak = max(locals_count, peak)
    pos = body + size + size % 2
print(f'{count} functions, maximum {peak} locals')
sys.exit(0 if count and peak <= 50000 else 1)
