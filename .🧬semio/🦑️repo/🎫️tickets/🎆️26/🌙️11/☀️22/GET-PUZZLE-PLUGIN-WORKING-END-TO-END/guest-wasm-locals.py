"""🔎️ Measure function locals in a core Wasm module or a component's nested modules."""

from mmap import ACCESS_READ, mmap
from sys import argv


def leb(data, position):
    value = 0
    shift = 0
    while True:
        byte = data[position]
        position += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, position
        shift += 7


def scan(data, start, end):
    component = data[start : start + 8] == b"\0asm\x0d\0\x01\0"
    assert component or data[start : start + 8] == b"\0asm\x01\0\0\0"
    position = start + 8
    while position < end:
        section = data[position]
        size, payload = leb(data, position + 1)
        position = payload + size
        assert position <= end
        if component and section in (1, 4):
            scan(data, payload, position)
        if not component and section == 10:
            count, body = leb(data, payload)
            largest = (0, 0, 0)
            for index in range(count):
                length, contents = leb(data, body)
                body = contents + length
                groups, group = leb(data, contents)
                total = 0
                for _ in range(groups):
                    local_count, group = leb(data, group)
                    group += 1
                    total += local_count
                largest = max(largest, (total, index, length))
            assert body == position
            print(f"module offset={start} functions={count} max_locals={largest[0]} code_index={largest[1]} body_bytes={largest[2]}")
            if len(argv) == 3:
                assert largest[0] <= int(argv[2])


with open(argv[1], "rb") as source, mmap(source.fileno(), 0, access=ACCESS_READ) as data:
    scan(data, 0, len(data))
