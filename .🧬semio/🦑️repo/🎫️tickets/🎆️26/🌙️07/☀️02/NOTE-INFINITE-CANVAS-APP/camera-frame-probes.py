from argparse import ArgumentParser
from bisect import bisect_right
from pathlib import Path
import re
import runpy

tools = runpy.run_path(str(Path(__file__).with_name("symbolicate-note-startup.py")))
leb, sections, code_bodies = (tools[name] for name in ("leb", "sections", "code_bodies"))
parser = ArgumentParser()
parser.add_argument("artifact", type=Path)
parser.add_argument("needles", nargs="*")
parser.add_argument("--function-id", type=int, action="append", default=[])
args = parser.parse_args()
data = args.artifact.read_bytes()
all_sections = {tag: (start, end) for tag, start, end in sections(data, 0, len(data))}
bodies = list(code_bodies(data, all_sections[10][0]))
def imported_functions(data, section):
    position, end = section
    count, position = leb(data, position)
    functions = 0
    for _ in range(count):
        for _ in range(2):
            size, position = leb(data, position)
            position += size
        kind = data[position]
        position += 1
        if kind == 0:
            functions += 1
            _, position = leb(data, position)
        elif kind in (1, 2):
            if kind == 1:
                position += 1
            flags, position = leb(data, position)
            _, position = leb(data, position)
            if flags & 1:
                _, position = leb(data, position)
        elif kind == 3:
            position += 2
        elif kind == 4:
            position += 1
            _, position = leb(data, position)
        else:
            raise ValueError(f"Unexpected import kind {kind}")
    assert position == end
    return functions

base_id = imported_functions(data, all_sections[2])
offset, p = leb(data, all_sections[11][0])
needle_addresses = {}
memory = []
for _ in range(offset):
    mode, p = leb(data, p)
    if mode == 2:
        _, p = leb(data, p)
    address = None
    if mode != 1:
        assert data[p] == 0x41
        address, p = leb(data, p + 1)
        assert data[p] == 0x0b
        p += 1
    size, p = leb(data, p)
    if address is not None:
        memory.append((address, address+size, p))
        for needle in args.needles:
            match = data.find(needle.encode(), p, p+size)
            if match >= 0:
                needle_addresses[needle] = address + match - p
    p += size

def padded(value):
    return bytes([0x80 | (value >> (7*i) & 0x7f) for i in range(4)] + [value >> 28])

starts = [start for _,start,_ in bodies]
for needle, address in needle_addresses.items():
    print(needle, address)
    pattern = b'\x41' + padded(address)
    offset = data.find(pattern, starts[0])
    while offset >= 0 and offset < all_sections[10][1]:
        index = bisect_right(starts, offset)-1
        i, start, end = bodies[index]
        calls = set()
        for pos in range(start,end-6):
            if data[pos] == 0x10 and all(data[pos+j] >= 128 for j in (1,2,3,4)) and data[pos+5] < 128:
                calls.add(leb(data,pos+1)[0])
        print(f"  id {i+base_id} range {start}..{end} literal {offset} calls {sorted(calls)}"[:600])
        offset = data.find(pattern,offset+6)

def memory_bytes(address, count):
    for start, end, data_offset in memory:
        if start <= address < end:
            offset = data_offset + address - start
            return data[offset:min(offset + count, data_offset + end - start)]
    return b''

for function_id in args.function_id:
    _, start, end = bodies[function_id - base_id]
    print("function", function_id, "range", start, end)
    shown = set()
    for offset in range(start, end-6):
        if data[offset] not in (0x10, 0x41) or not all(data[offset+i] >= 128 for i in (1,2,3,4)) or data[offset+5] >= 128:
            continue
        value, _ = leb(data, offset+1)
        if data[offset] == 0x10:
            print(offset, "call", value)
            continue
        body = memory_bytes(value, 180)
        if len(body) < 16:
            continue
        address, size, line, col = (int.from_bytes(body[i:i+4], "little") for i in (0,4,8,12))
        location = memory_bytes(address, size) if size < 300 else b''
        if b'.rs' in location and 0 < line < 9000 and 0 < col < 1800:
            label = f'{location.decode(errors="ignore")}:{line}:{col}'
        else:
            prefix = re.split(bytes([0]), body)[0]
            label = prefix.decode(errors="ignore")[:145] if len(prefix) >= 24 and prefix[:1].isalnum() else ''
        if label and label not in shown:
            print(offset, label)
            shown.add(label)
