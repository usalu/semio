"""🪙 Observe retained retirement quotes in the exact WASI fixture after its Rust aggregate return."""
from pathlib import Path
import sys


def integer(value):
    data = bytearray()
    while value >= 128:
        data.append((value & 127) | 128)
        value >>= 7
    data.append(value)
    return bytes(data)


def read(data, cursor):
    value, shift = 0, 0
    while True:
        byte = data[cursor]
        cursor += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, cursor
        shift += 7


def chunk(data, cursor):
    length, cursor = read(data, cursor)
    return data[cursor:cursor + length], cursor + length


def vector(entries):
    return integer(len(entries)) + b''.join(entries)


def tagged(tag, data):
    return bytes([tag]) + integer(len(data)) + data


source, target, search = sys.argv[1:]
data = Path(source).read_bytes()
sections = []
cursor = 8
while cursor < len(data):
    kind = data[cursor]
    body, cursor = chunk(data, cursor + 1)
    sections.append((kind, body))
names = next(body for kind, body in sections if kind == 0 and body[:5] == b'\4name')
cursor = 5
found = []
while cursor < len(names):
    kind = names[cursor]
    body, cursor = chunk(names, cursor + 1)
    if kind == 1:
        count, offset = read(body, 0)
        for _ in range(count):
            function, offset = read(body, offset)
            name, offset = chunk(body, offset)
            if search.encode() in name:
                found.append((function, name.decode()))
if len(found) != 1:
    raise SystemExit(found)
function, name = found[0]

by_kind = dict(sections)
functions, first_function = read(by_kind[3], 0)
imports, offset = read(by_kind[2], 0)
for _ in range(imports):
    _, offset = chunk(by_kind[2], offset)
    _, offset = chunk(by_kind[2], offset)
    if by_kind[2][offset] != 0:
        raise SystemExit('Expected only function imports')
    _, offset = read(by_kind[2], offset + 1)
types = []
_, offset = read(by_kind[1], 0)
while offset < len(by_kind[1]):
    if by_kind[1][offset] != 0x60:
        raise SystemExit('Expected function type')
    params, offset = chunk(by_kind[1], offset + 1)
    results, offset = chunk(by_kind[1], offset)
    types.append((params, results))
type_indices = []
offset = first_function
for _ in range(functions):
    type_index, offset = read(by_kind[3], offset)
    type_indices.append(type_index)
type_index = type_indices[function - imports]
params, results = types[type_index]
print(name, params.hex(), results.hex())
if results or not params or params[0] != 0x7f:
    raise SystemExit('Expected aggregate return through first i32 param')

globals_count, globals_start = read(by_kind[6], 0)
exports_count, exports_start = read(by_kind[7], 0)
quote_words = 8
added = len(params) + quote_words
globals_extra = b'\x7f\x01\x41\x00\x0b' * added
exports_extra = bytearray()
for number in range(added):
    label = f'quote{number}'.encode()
    exports_extra += integer(len(label)) + label + b'\x03' + integer(globals_count + number)

wrapper = bytearray(b'\x00')
for parameter in range(len(params)):
    if params[parameter] != 0x7f:
        raise SystemExit('Expected i32 argument')
    wrapper += b'\x20' + integer(parameter)
wrapper += b'\x10' + integer(imports + functions)
for parameter in range(len(params)):
    wrapper += b'\x20' + integer(parameter) + b'\x24' + integer(globals_count + parameter)
for word in range(quote_words):
    wrapper += b'\x20\x00\x28\x02' + integer(word * 4) + b'\x24' + integer(globals_count + len(params) + word)
wrapper += b'\x0b'

_, offset = read(by_kind[10], 0)
code = []
while offset < len(by_kind[10]):
    body, offset = chunk(by_kind[10], offset)
    code.append(body)
code.append(code[function - imports])
code[function - imports] = wrapper
replacements = {
    3: integer(functions + 1) + by_kind[3][first_function:] + integer(type_index),
    6: integer(globals_count + added) + by_kind[6][globals_start:] + globals_extra,
    7: integer(exports_count + added) + by_kind[7][exports_start:] + exports_extra,
    10: vector([integer(len(body)) + body for body in code]),
}
with Path(target).open('wb') as output:
    output.write(data[:8])
    for kind, body in sections:
        output.write(tagged(kind, replacements.get(kind, body)))
