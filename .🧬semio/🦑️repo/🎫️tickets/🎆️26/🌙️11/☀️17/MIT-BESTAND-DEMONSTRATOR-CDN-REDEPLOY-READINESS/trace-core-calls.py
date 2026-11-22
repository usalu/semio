"""🔬️ Add an isolated diagnostic call log to a private copy of a stripped Wasm core."""
import json
import mmap
import sys
from pathlib import Path


def leb(n, signed=False, width=0):
    """🧮 Encode a positive Wasm integer."""
    result = bytearray()
    while n >= (64 if signed else 128) or len(result) + 1 < width:
        result.append((n & 127) | 128)
        n >>= 7
    result.append(n)
    return result


def read(data, pos):
    """📖 Decode an unsigned Wasm integer."""
    n = shift = 0
    while data[pos] & 128:
        n |= (data[pos] & 127) << shift
        pos += 1
        shift += 7
    return n | (data[pos] << shift), pos + 1


def exported(name, kind, index):
    """📤 Describe one debug-only export."""
    return leb(len(name)) + name + bytes([kind]) + leb(index)


def trace(original, destination, points=None):
    """🔎 Trace guest entries into a separate memory without modifying guest-owned state."""
    imported = memory = counter = None
    with original.open('rb') as source, mmap.mmap(source.fileno(), 0, access=mmap.ACCESS_READ) as data, destination.open('xb') as out:
        out.write(data[:8])
        pos = 8
        while pos < len(data):
            kind = data[pos]
            size, start = read(data, pos + 1)
            end = start + size
            if kind not in (2, 5, 6, 7, 10):
                out.write(data[pos:end])
            elif kind == 2:
                imported, p = read(data, start)
                for _ in range(imported):
                    for _ in range(2):
                        length, p = read(data, p)
                        p += length
                    assert data[p] == 0, 'expected only imported functions'
                    _, p = read(data, p + 1)
                out.write(data[pos:end])
            elif kind in (5, 6, 7):
                count, p = read(data, start)
                if kind == 5:
                    memory = count
                    extra = bytes([1, 16, 16])
                elif kind == 6:
                    counter = count
                    extra = bytes.fromhex('7f 01 41 00 0b')
                else:
                    extra = exported(b'tracePtr', 3, counter) + exported(b'traceMem', 2, memory)
                body = leb(count + (2 if kind == 7 else 1)) + data[p:end] + extra
                out.write(bytes([kind]) + leb(len(body)) + body)
            else:
                count, p = read(data, start)
                out.write(bytes([10]))
                mark = out.tell()
                out.write(bytes(5))
                out.write(leb(count))
                get = b'\x23' + leb(counter)
                set_ = b'\x24' + leb(counter)
                before = get + bytes.fromhex('04 40') + get + b'\x41'
                after = bytes([0x36, 0x42]) + leb(memory) + bytes([0]) + get + bytes.fromhex('41 04 6a 41') + leb(1048575, True) + bytes([0x71]) + set_ + bytes([11])
                for number in range(imported, imported + count):
                    length, begin = read(data, p)
                    local_count, op = read(data, begin)
                    for _ in range(local_count):
                        _, op = read(data, op)
                        op += 1
                    local_prefix, p_end = op, begin + length
                    if points is None:
                        entry = before + leb(number, True) + after
                    else:
                        entry = bytearray()
                        for absolute, registers in points.get(str(number), []):
                            boundary = op if absolute is None else absolute
                            assert op <= boundary < begin + length
                            entry.extend(data[op:boundary])
                            entry.extend(before + leb(number, True) + after)
                            for register in registers:
                                entry.extend(get + bytes.fromhex('04 40') + get + bytes([0x20]) + leb(register) + after)
                            op = boundary
                        length += len(entry) - (op - local_prefix)
                        out.write(leb(length) + data[begin:local_prefix] + entry + data[op:p_end])
                        p = p_end
                        continue
                    p = begin + length
                    out.write(leb(length + len(entry)) + data[begin:op] + entry + data[op:p])
                assert p == end
                resume = out.tell()
                out.seek(mark)
                out.write(leb(resume - mark - 5, width=5))
                out.seek(resume)
            pos = end


if __name__ == '__main__':
    trace(Path(sys.argv[1]), Path(sys.argv[2]), json.loads(Path(sys.argv[3]).read_text()) if len(sys.argv) > 3 else None)
