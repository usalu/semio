from collections import Counter
from hashlib import sha256
from pathlib import Path
import contextlib
import importlib.util
import io


TICKET = Path(__file__).parent
SPEC = importlib.util.spec_from_file_location("frames", TICKET / "🧪️dwg-line-frame-probe.py")
FRAMES = importlib.util.module_from_spec(SPEC)
with contextlib.redirect_stdout(io.StringIO()):
    SPEC.loader.exec_module(FRAMES)


def modular_unsigned(value):
    output = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        output.append(byte | (0x80 if value else 0))
        if not value:
            return bytes(output)


def modular_signed(value):
    negative = value < 0
    value = abs(value)
    output = bytearray()
    while value >= 0x40:
        output.append((value & 0x7F) | 0x80)
        value >>= 7
    output.append(value | (0x40 if negative else 0))
    return bytes(output)


def handle_blocks(data):
    blocks = []
    position = 0
    while position + 2 <= len(data):
        start = position
        size = int.from_bytes(data[position:position + 2], "big")
        position += 2
        handle = 0
        address = 0
        entries = []
        while position < start + size:
            pair_start = position
            handle_delta, position = FRAMES.modular_char(data, position)
            address_delta, position = FRAMES.modular_char(data, position, True)
            handle += handle_delta
            address += address_delta
            entries.append((handle, address, handle_delta, address_delta, position - pair_start))
        checksum = int.from_bytes(data[start + size:start + size + 2], "big")
        assert checksum == FRAMES.crc16(data[start:start + size])
        blocks.append((start, size, checksum, entries))
        position = start + size + 2
        if size == 2:
            break
    assert position == len(data)
    return blocks


def encode_handle_blocks(entries):
    output = bytearray()
    payload = bytearray()
    last_handle = 0
    last_address = 0
    for handle, address in entries:
        pair = modular_unsigned(handle - last_handle) + modular_signed(address - last_address)
        payload.extend(pair)
        last_handle = handle
        last_address = address
        if len(payload) + 2 > 2030:
            block = (len(payload) + 2).to_bytes(2, "big") + payload
            output.extend(block)
            output.extend(FRAMES.crc16(block).to_bytes(2, "big"))
            payload.clear()
            last_handle = 0
            last_address = 0
    if payload:
        block = (len(payload) + 2).to_bytes(2, "big") + payload
        output.extend(block)
        output.extend(FRAMES.crc16(block).to_bytes(2, "big"))
    terminator = b"\x00\x02"
    output.extend(terminator)
    output.extend(FRAMES.crc16(terminator).to_bytes(2, "big"))
    return bytes(output)


objects = FRAMES.objects
handles = FRAMES.handles
blocks = handle_blocks(handles)
entries = [(handle, address) for _, size, _, rows in blocks if size > 2 for handle, address, *_ in rows]
assert encode_handle_blocks(entries) == handles

object_rows = []
type_counts = Counter()
for handle, address in entries:
    payload_size, handle_bits, prefix_bytes, payload = FRAMES.frame_prefix(objects, address)
    reader = FRAMES.Bits(payload)
    object_type = FRAMES.bot(reader)
    _, object_handle = reader.handle()
    end = address + prefix_bytes + payload_size + 2
    stored_crc = int.from_bytes(objects[end - 2:end], "little")
    assert object_handle == handle
    assert stored_crc == FRAMES.crc16(objects[address:end - 2])
    object_rows.append((address, end, handle, object_type, payload_size, handle_bits, prefix_bytes, stored_crc))
    type_counts[object_type] += 1

address_rows = sorted(object_rows)
assert objects[:4] == (0x0DCA).to_bytes(4, "little")
assert address_rows[0][0] == 4
assert address_rows[-1][1] == len(objects)
assert all(left[1] == right[0] for left, right in zip(address_rows, address_rows[1:]))

print(f"objects_length={len(objects)} sha256={sha256(objects).hexdigest()}")
print(f"handles_length={len(handles)} sha256={sha256(handles).hexdigest()}")
print(f"objects_preamble={objects[:4].hex()} frames={len(entries)} contiguous=4..{len(objects)}")
print("object_pages=" + ",".join(f"{page[2]}:{page[1]}" for page in FRAMES.OBJECT_PAGES))
print(f"handles_page={FRAMES.HANDLES_PAGE[2]}:{FRAMES.HANDLES_PAGE[1]}")
for index, (start, size, checksum, rows) in enumerate(blocks):
    signs = Counter("negative" if row[3] < 0 else "zero" if row[3] == 0 else "positive" for row in rows)
    pair_lengths = Counter(row[4] for row in rows)
    first = "none" if not rows else f"{rows[0][0]:#x}@{rows[0][1]}"
    last = "none" if not rows else f"{rows[-1][0]:#x}@{rows[-1][1]}"
    print(f"block[{index}] start={start} size={size} payload={size - 2} entries={len(rows)} first={first} last={last} crc={checksum:04x} signs={dict(signs)} pair_lengths={dict(sorted(pair_lengths.items()))}")
print("second_block=" + ",".join(f"{handle:#x}@{address}:type{object_type}" for address, _, handle, object_type, *_ in object_rows[-11:]))
print("address_order_first=" + ",".join(f"{row[2]:#x}@{row[0]}" for row in address_rows[:8]))
print("address_order_last=" + ",".join(f"{row[2]:#x}@{row[0]}" for row in address_rows[-8:]))
print("type_counts=" + ",".join(f"{key}:{type_counts[key]}" for key in sorted(type_counts)))
