"""🧮 Count selected entries in a private browser core without relinking."""

import mmap
from pathlib import Path
import re
import sys


def read(body, offset):
    value = shift = 0
    while True:
        byte = body[offset]
        offset += 1
        value |= (byte & 127) << shift
        shift += 7
        if byte < 128:
            return value, offset


def encode(value):
    data = bytearray()
    while value >= 128:
        data.append((value & 127) | 128)
        value >>= 7
    return bytes(data + bytes([value]))


def skip_name(body, offset):
    size, start = read(body, offset)
    return start + size


source, target, *frames = sys.argv[1:]
probes = set()
for frame in frames:
    match = re.fullmatch(r"(\d+)(?:([+@])(\d+))?", frame)
    if not match:
        raise ValueError(f"Invalid function probe {frame}")
    index, operator, offset = match.groups()
    probes.add((int(index), -1 if offset is None else int(offset), operator == "@"))
if Path(source).resolve() == Path(target).resolve() or not probes:
    raise ValueError("Provide an independent output path and explicit function indices")
with Path(source).open("rb") as file, mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ) as body:
    sections = []
    offset = 8
    while offset < len(body):
        kind = body[offset]
        size, start = read(body, offset + 1)
        offset = start + size
        sections.append((kind, start, offset))
    known = {kind: (start, end) for kind, start, end in sections}
    import_count, offset = read(body, known[2][0])
    for _ in range(import_count):
        offset = skip_name(body, skip_name(body, offset))
        if body[offset] != 0:
            raise ValueError("Expected only function imports")
        _, offset = read(body, offset + 1)
    global_count, globals_start = read(body, known[6][0])
    export_count, exports_start = read(body, known[7][0])
    globals_extra = b"\x7f\x01\x41\x00\x0b" * len(probes)
    exports_extra = bytearray()
    patches = []
    code_count, cursor = read(body, known[10][0])
    scopes = {}
    for ordinal, (function, offset, gauge) in enumerate(sorted(probes)):
        global_id = encode(global_count + ordinal)
        label = f"archive{'Value' if gauge else 'Count'}{function}{'_' + str(offset) if offset >= 0 else ''}".encode()
        exports_extra += encode(len(label)) + label + b"\x03" + global_id
        instructions = (b"\x24" + global_id + b"\x23" + global_id if gauge else
                        b"\x23" + global_id + b"\x41\x01\x6a\x24" + global_id)
        scopes.setdefault(function, []).append((offset, instructions))
    for function in range(import_count, import_count + code_count):
        length_at = cursor
        length, start = read(body, cursor)
        cursor = start + length
        if function not in scopes:
            continue
        groups, insertion = read(body, start)
        for _ in range(groups):
            _, insertion = read(body, insertion)
            insertion += 1
        patched = bytearray()
        previous = start
        for offset, instructions in scopes[function]:
            position = insertion if offset < 0 else start + offset
            if not insertion <= position < cursor:
                raise ValueError(f"Probe outside function {function} instruction body")
            patched += body[previous:position] + instructions
            previous = position
        growth = len(patched) - (previous - start)
        patches.append((length_at, previous, encode(length + growth) + patched))
    if len(patches) != len(scopes):
        raise ValueError("A requested function was not defined in the linked core")
    code_extra = sum(len(patched) - (insertion - start) for start, insertion, patched in patches)
    with Path(target).open("wb") as output:
        output.write(body[:8])
        for kind, start, end in sections:
            output.write(bytes([kind]))
            if kind == 6:
                declaration = encode(global_count + len(probes))
                output.write(encode(len(declaration) + end - globals_start + len(globals_extra)))
                output.write(declaration)
                output.write(body[globals_start:end])
                output.write(globals_extra)
            elif kind == 7:
                declaration = encode(export_count + len(probes))
                output.write(encode(len(declaration) + end - exports_start + len(exports_extra)))
                output.write(declaration)
                output.write(body[exports_start:end])
                output.write(exports_extra)
            elif kind == 10:
                output.write(encode(end - start + code_extra))
                cursor = start
                for before, after, patched in patches:
                    output.write(body[cursor:before])
                    output.write(patched)
                    cursor = after
                output.write(body[cursor:end])
            else:
                output.write(encode(end - start))
                output.write(body[start:end])
print(f"[DEBUG] installed {len(probes)} guest probes in {len(scopes)} functions")
