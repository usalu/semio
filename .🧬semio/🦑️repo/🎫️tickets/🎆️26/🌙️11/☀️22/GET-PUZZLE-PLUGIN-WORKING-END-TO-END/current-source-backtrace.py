from pathlib import Path
import os, shutil, struct, subprocess, sys
binary, output = map(Path, sys.argv[1:3])
symbol = next(row for row in subprocess.check_output(['nm', binary], text=True).splitlines() if row.endswith('source_owner_fault'))
address = int(symbol.split()[0], 16)
shutil.copyfile(binary, output)
output.chmod(0o700)
with output.open('r+b') as image:
    header = image.read(64)
    table = struct.unpack_from('<Q', header, 32)[0]
    size, count = struct.unpack_from('<HH', header, 54)
    for index in range(count):
        image.seek(table + index * size)
        segment, _, offset, start, _, length = struct.unpack_from('<IIQQQQ', image.read(size))
        if segment == 1 and start <= address < start + length:
            image.seek(offset + address - start)
            image.write(bytes([0xcc]))
            break
    else:
        raise ValueError('diagnostic symbol outside load segments')
