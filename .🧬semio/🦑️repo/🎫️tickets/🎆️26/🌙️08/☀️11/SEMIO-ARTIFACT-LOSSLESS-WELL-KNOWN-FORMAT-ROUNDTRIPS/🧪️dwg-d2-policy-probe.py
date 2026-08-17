from pathlib import Path


FIXTURE = Path("/Users/ueli/Documents/semio/temp/architectural_example.dwg")

PAGES = [
    ("6 RevHistory", 0x161A0 + 32, 135, 16),
    ("7 Objects[0]", 0x16260 + 32, 17145, 0x7400),
    ("8 Objects[1]", 0x1A580 + 32, 11080, 0x7400),
    ("9 Objects[2]", 0x1D100 + 32, 4380, 0x7400),
    ("10 Objects[3]", 0x1E240 + 32, 2246, 0x7400),
    ("11 Objects[4]", 0x1EB40 + 32, 3378, 0x7400),
    ("12 Objects[5]", 0x1F8A0 + 32, 4448, 0x7400),
    ("13 Objects[6]", 0x20A20 + 32, 3490, 0x7400),
    ("14 Objects[7]", 0x21800 + 32, 1711, 213182 - 7 * 0x7400),
    ("15 ObjFreeSpace", 0x21EE0 + 32, 169, 89),
    ("16 Template", 0x21FC0 + 32, 129, 6),
    ("17 Handles", 0x22080 + 32, 1907, 2085),
    ("18 Classes", 0x22820 + 32, 4656, 8194),
    ("19 AuxHeader", 0x23A80 + 32, 205, 123),
    ("20 Header", 0x23B80 + 32, 946, 896),
    ("23 SectionInfo(system)", 0x23F60 + 20, 970, 1684),
    ("24 SectionMap(system)", 0x24360 + 20, 170, 176),
]


class Cursor:
    def __init__(self, data):
        self.data = data
        self.pos = 0

    def byte(self):
        value = self.data[self.pos]
        self.pos += 1
        return value


def variable_count(cursor, opcode, mask):
    count = opcode & mask
    if count == 0:
        value = cursor.byte()
        while value == 0:
            count += 0xFF
            value = cursor.byte()
        count += value + mask
    return count


def decompress(encoded):
    cursor = Cursor(encoded)
    output = bytearray()
    tokens = []
    opcode = cursor.byte()
    if opcode > 0x11:
        length = opcode - 0x11
        output.extend(encoded[cursor.pos:cursor.pos + length])
        cursor.pos += length
        opcode = cursor.byte()
        tokens.append(("initial-special", length, 0, 0))
    if opcode & 0xF0 == 0:
        length = variable_count(cursor, opcode, 0x0F) + 3
        output.extend(encoded[cursor.pos:cursor.pos + length])
        cursor.pos += length
        opcode = cursor.byte()
        tokens.append(("initial", length, 0, 0))
    while opcode != 0x11:
        family = "short"
        if opcode < 0x10 or opcode >= 0x40:
            length = (opcode >> 4) - 1
            second = cursor.byte()
            offset = ((opcode >> 2 & 3) | second << 2) + 1
            tail_code = opcode
        elif opcode < 0x20:
            family = "long-far"
            length = variable_count(cursor, opcode, 0x07) + 2
            offset = (opcode & 8) << 11
            first = cursor.byte()
            second = cursor.byte()
            offset = (offset | first >> 2 | second << 6) + 0x4000
            tail_code = first
        else:
            family = "long-near"
            length = variable_count(cursor, opcode, 0x1F) + 2
            first = cursor.byte()
            second = cursor.byte()
            offset = (first >> 2 | second << 6) + 1
            tail_code = first
        start = len(output) - offset
        for index in range(length):
            output.append(output[start + index])
        literals = tail_code & 3
        if literals == 0:
            opcode = cursor.byte()
            if opcode & 0xF0 == 0:
                literals = variable_count(cursor, opcode, 0x0F) + 3
        if literals:
            output.extend(encoded[cursor.pos:cursor.pos + literals])
            cursor.pos += literals
            opcode = cursor.byte()
        tokens.append((family, length, offset, literals))
    return bytes(output), tokens, cursor.pos


def write_length(output, length):
    while length > 0xFF:
        length -= 0xFF
        output.append(0)
    output.append(length)


def write_opcode(output, opcode, length, immediate):
    if length <= immediate:
        output.append(opcode | length - 2)
    else:
        output.append(opcode)
        write_length(output, length - immediate)


def literal_length(output, source, start, length):
    if not length:
        return
    if length > 3:
        write_opcode(output, 0, length - 1, 0x11)
    output.extend(source[start:start + length])


def apply_match(output, distance, length, following_literals):
    if length >= 0x0F or distance > 0x400:
        if distance <= 0x4000:
            encoded_distance = distance - 1
            write_opcode(output, 0x20, length, 0x21)
        else:
            encoded_distance = distance - 0x4000
            write_opcode(output, 0x10 | ((encoded_distance >> 11) & 8), length, 0x09)
        first = (encoded_distance & 0xFF) << 2
        second = encoded_distance >> 6
    else:
        encoded_distance = distance - 1
        first = ((length + 1) << 4) | ((encoded_distance & 3) << 2)
        second = encoded_distance >> 2
    if following_literals < 4:
        first |= following_literals
    output.extend((first & 0xFF, second & 0xFF))


def hash4(source, position):
    value = source[position + 3] << 6
    value = value ^ source[position + 2]
    value = value << 5 ^ source[position + 1]
    value = value << 5 ^ source[position]
    return (value + (value >> 5)) & 0x7FFF


def candidate(source, position, end, table):
    index = hash4(source, position)
    previous = table[index]
    distance = position - previous
    if previous >= 0 and distance <= 0xBFFF:
        if distance > 0x400 and source[position + 3] != source[previous + 3]:
            index = (index & 0x7FF) ^ 0b100000000011111
            previous = table[index]
            distance = position - previous
            if previous < 0 or distance > 0xBFFF or (distance > 0x400 and source[position + 3] != source[previous + 3]):
                table[index] = position
                return 0, distance
        if source[position:position + 3] == source[previous:previous + 3]:
            length = 3
            while position + length < end and source[previous + length] == source[position + length]:
                length += 1
            table[index] = position
            return length, distance
    table[index] = position
    return 0, distance


def compress(source):
    output = bytearray()
    table = [-1] * 0x8000
    current_literal = 0
    position = 4
    end = len(source)
    pending_length = 0
    pending_distance = 0
    while position < end - 0x13:
        length, distance = candidate(source, position, end, table)
        if length < 3:
            position += 1
            continue
        literals = position - current_literal
        if pending_length:
            apply_match(output, pending_distance, pending_length, literals)
        literal_length(output, source, current_literal, literals)
        position += length
        current_literal = position
        pending_length = length
        pending_distance = distance
    literals = end - current_literal
    if pending_length:
        apply_match(output, pending_distance, pending_length, literals)
    literal_length(output, source, current_literal, literals)
    output.extend((0x11, 0, 0))
    return bytes(output)


def first_difference(left, right):
    for index, (a, b) in enumerate(zip(left, right)):
        if a != b:
            return index
    return min(len(left), len(right)) if len(left) != len(right) else None


if __name__ == "__main__":
    fixture = FIXTURE.read_bytes()
    print("name | raw | decoded/semantic | tokens | short/near/far | max_length | max_distance | consumed+trailer | rebuilt | first_diff")
    for name, offset, compressed_size, expected_size in PAGES:
        encoded = fixture[offset:offset + compressed_size]
        decoded, tokens, consumed = decompress(encoded)
        rebuilt = compress(decoded)
        families = [sum(token[0] == family for token in tokens) for family in ("short", "long-near", "long-far")]
        matches = [token for token in tokens if token[0] in ("short", "long-near", "long-far")]
        print(f"{name} | {len(encoded)} | {len(decoded)}/{expected_size} | {len(tokens)} | {families[0]}/{families[1]}/{families[2]} | {max(token[1] for token in matches)} | {max(token[2] for token in matches)} | {consumed}+{len(encoded)-consumed} | {len(rebuilt)} | {first_difference(encoded, rebuilt)}")
