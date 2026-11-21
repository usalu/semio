"""🔍 Finds the WASI code and data addresses for the mounted member's diagnostic markers."""
from bisect import bisect_left
import mmap
import os
from pathlib import Path
import re
import sys


class WasmSlice:
    def __init__(self, blob, start, end):
        self.blob, self.start, self.end = blob, start, end

    def __len__(self):
        return self.end - self.start

    def __getitem__(self, index):
        if isinstance(index, slice):
            start, end, step = index.indices(len(self))
            return self.blob[self.start + start:self.start + end:step]
        return self.blob[self.start + index]

    def find(self, needle, start, end):
        offset = self.blob.find(needle, self.start + start, self.start + end)
        return offset - self.start if offset >= 0 else -1


def integer(blob, position, signed=False):
    start = position
    value = 0
    while True:
        byte = blob[position]
        value |= (byte & 127) << ((position - start) * 7)
        position += 1
        if byte < 128:
            if signed and byte & 64:
                value -= 1 << ((position - start) * 7)
            return value, position


def unsigned(value):
    result = bytearray()
    while value >= 128:
        result.append((value & 127) | 128)
        value >>= 7
    return bytes(result) + bytes([value])


def constant(value):
    return b"\x41" + unsigned(value) if unsigned(value)[-1] < 64 else b"\x41" + unsigned(value)[:-1] + bytes([unsigned(value)[-1] | 128, 0])

def padded(value):
    encoded = unsigned(value)
    return encoded[:-1] + bytes([encoded[-1] | 128]) + b"\x80" * (4 - len(encoded)) + b"\x00"


def sections(blob):
    position = 8
    while position < len(blob):
        section_id = blob[position]
        length, start = integer(blob, position + 1)
        yield section_id, start, start + length
        position = start + length


def data_segments(blob, start):
    count, position = integer(blob, start)
    for _ in range(count):
        flags, position = integer(blob, position)
        address = None
        if flags == 2:
            _, position = integer(blob, position)
        if flags != 1:
            assert blob[position] == 0x41
            address, end = integer(blob, position + 1, signed=True)
            assert blob[end] == 0x0B
            position = end + 1
        length, position = integer(blob, position)
        yield address, position, position + length
        position += length


def functions(blob, start):
    count, position = integer(blob, start)
    for function in range(count):
        length, position = integer(blob, position)
        yield function, position, position + length
        position += length


def receiving_work_sites(blob, start, end, helper):
    call = b"\x10" + padded(helper)
    cursor = start
    while (position := blob.find(call, cursor, end)) != -1:
        cursor = position + len(call)
        position = cursor
        if blob[position] != 0x20:
            continue
        _, position = integer(blob, position + 1)
        if blob[position:position + 2] != b"\x29\x03":
            continue
        field, position = integer(blob, position + 2)
        if field != 440 or blob[position] != 0x22:
            continue
        local, position = integer(blob, position + 1)
        if blob[position] != 0x20:
            continue
        duplicate, position = integer(blob, position + 1)
        if local != duplicate or blob[position:position + 5] != b"\x42\x01\x88\x7d\x21":
            continue
        saved, _ = integer(blob, position + 5)
        if saved == local:
            yield position + 4, local


def instrument(blob, bounds, terms, destination):
    code_start, code_end = bounds[10]
    globals_start, globals_end = bounds[6]
    exports_start, exports_end = bounds[7]
    count, entries = integer(blob, globals_start)
    global_index = count
    ownership_literal = os.environ.get("ENERGY_OWNERSHIP_LITERAL")
    ledger_settlement = os.environ.get("ENERGY_LEDGER_SETTLEMENT")
    work_rows = os.environ.get("ENERGY_WORK_LEDGER_ROWS")
    work_calls = os.environ.get("ENERGY_WORK_CALL_SITES")
    assert sum(bool(mode) for mode in (ownership_literal, ledger_settlement, work_rows, work_calls)) <= 1
    ledger_fields = [
        ("ledger_issued_items", 32, 0, 56), ("ledger_issued_copy", 32, 0, 60),
        ("ledger_issued_capacity", 32, 0, 64), ("ledger_issued_release", 32, 0, 68),
        ("ledger_spent_items", 32, 0, 0), ("ledger_spent_copy", 32, 0, 4),
        ("ledger_spent_capacity", 32, 0, 8), ("ledger_spent_release", 32, 0, 12),
        ("ledger_held_items", 32, 0, 76), ("ledger_held_copy", 32, 0, 80),
        ("ledger_held_capacity", 32, 0, 88), ("ledger_held_release", 32, 0, 84),
        ("ledger_delta_items", 32, 1, 0), ("ledger_delta_copy", 32, 1, 4),
        ("ledger_delta_capacity", 32, 1, 8), ("ledger_delta_release", 32, 1, 12),
        ("ledger_claims", 32, 0, 92), ("ledger_serial", 64, 0, 48),
        ("ledger_issued_work", 64, 0, 32), ("ledger_held_work", 64, 0, 40),
        ("ledger_spent_work_low", 64, 0, 16), ("ledger_spent_work_high", 64, 0, 24),
    ]
    if work_calls:
        observers = [
            ("ledger_work_current_site", 32), ("ledger_work_failed_site", 32),
            ("ledger_debit_budget_pointer", 32),
        ]
    elif work_rows:
        observers = [
            ("ledger_budget_pointer", 32), ("ledger_work_fault_body", 32),
            ("ledger_work_before", 64), ("ledger_work_after", 64),
            ("ledger_work_held_on_fault", 64), ("ledger_work_issued_on_fault", 64),
        ]
    elif ledger_settlement:
        observers = [(name, width) for name, width, _, _ in ledger_fields] + [("ledger_fault_count", 32)]
    else:
        observers = [("ownership_refusal", 64)] if ownership_literal else [("member_stage", 32)]
    encodings = {32: b"\x7f\x01\x41\x00\x0b", 64: b"\x7e\x01\x42\x00\x0b"}
    globals_payload = unsigned(count + len(observers)) + blob[entries:globals_end] + b"".join(encodings[width] for _, width in observers)
    count, entries = integer(blob, exports_start)
    exports_payload = unsigned(count + len(observers)) + blob[entries:exports_end]
    for index, (label, _) in enumerate(observers, start=global_index):
        label = label.encode()
        exports_payload += unsigned(len(label)) + label + b"\x03" + unsigned(index)
    count, first = integer(blob, code_start)
    parts = []
    instructions = {}
    if work_calls:
        assert not terms and not os.environ.get("ENERGY_HYDRATION_LISTING")
        site_index, failure_index, budget_index = (unsigned(global_index + field) for field in range(3))
        for row in Path(work_calls).read_text().splitlines():
            if not row.startswith("work-site "):
                continue
            _, _, field, target = row.split()
            site = int(field, 16)
            assert blob[site:site + 6] == b"\x10" + padded(int(target))
            instructions[site] = constant(site) + b"\x24" + site_index
        assert instructions
        start, end = next((begin, finish) for body, begin, finish in functions(blob, code_start) if body == 687720)
        target = b"\x10" + padded(690223) + b"\x21\x05"
        site = blob.find(target, start, end)
        assert site >= start and blob.find(target, site + 1, end) == -1
        instructions[site + 6] = (
            b"\x22\x05\x41\x03\x46\x04\x40\x23" + budget_index + b"\x45\x04\x40\x20\x01\x24"
            + budget_index + b"\x23" + site_index + b"\x24" + failure_index + b"\x0b\x0b\x20\x05"
        )
    if work_rows:
        assert not terms and not os.environ.get("ENERGY_HYDRATION_LISTING")
        target = b"\x10" + padded(690217) + b"\x1b"
        fault_body = unsigned(global_index + 1)
        for index, start, end in functions(blob, code_start):
            if index == 687723:
                error = b"\x10" + padded(709638)
                site = blob.find(error, start, end)
                assert site >= start and blob.find(error, site + 1, end) == -1
                instructions[site] = b"\x20\x01\x24" + unsigned(global_index)
            if index not in (690148, 690150):
                continue
            site = blob.find(target, start, end)
            assert site >= start and blob.find(target, site + 1, end) == -1
            before = 3 if index == 690148 else 4
            fields = [b"\x20" + unsigned(before), b"\x20\x00\x29\x03\x10", b"\x20\x00\x29\x03\x28", b"\x20\x00\x29\x03\x20"]
            capture = b"".join(value + b"\x24" + unsigned(global_index + field + 2) for field, value in enumerate(fields))
            instructions[site + len(target) - 1] = (
                b"\x04\x7f\x41\x01\x05\x23" + fault_body + b"\x45\x04\x40" + capture
                + constant(index) + b"\x24" + fault_body + b"\x0b\x41\x00\x0b"
            )
        assert len(instructions) == 3
    if ledger_settlement:
        assert not terms and not os.environ.get("ENERGY_HYDRATION_LISTING")
        offset = int(ledger_settlement, 16)
        assert blob[offset - 6:offset + 1] == b"\x10" + padded(690217) + b"\x1b"
        tally = unsigned(global_index + len(ledger_fields))
        fields = []
        for index, (_, width, source, location) in enumerate(ledger_fields, start=global_index):
            load = b"\x28\x02" if width == 32 else b"\x29\x03"
            fields.append(b"\x20" + unsigned(source) + load + unsigned(location) + b"\x24" + unsigned(index))
        instructions[offset] = (
            b"\x04\x7f\x41\x01\x05\x23" + tally + b"\x45\x04\x40" + b"".join(fields) + b"\x0b\x23"
            + tally + b"\x41\x01\x6a\x24" + tally + b"\x41\x00\x0b"
        )
    if ownership_literal:
        assert not terms and not os.environ.get("ENERGY_HYDRATION_LISTING")
        offset = int(ownership_literal, 16)
        assert blob[offset:offset + 4] == b"\x20\x00\x42\x00"
        number = unsigned(global_index)
        instructions[offset] = (
            b"\x20\x01\x41\x02\x46\x04\x40"
            b"\x20\x02\xad\x20\x03\xad\x42" + unsigned(65535) +
            b"\x83\x42\x20\x86\x84"
            b"\x23" + number + b"\x42\x30\x88\x42\x01\x7c\x42\x30\x86\x84\x24" + number + b"\x0b"
        )
    work_cap = os.environ.get("ENERGY_RECEIVING_WORK_CAP")
    release_cap = os.environ.get("ENERGY_RECEIVING_RELEASE_CAP")
    receiving_body, receiving_call = map(int, os.environ.get("ENERGY_RECEIVING_HELPER", "688213:688286").split(":"))
    work_sites = 0
    release_sites = 0
    if listing := os.environ.get("ENERGY_HYDRATION_LISTING"):
        for function in Path(listing).read_text().split("function ")[1:]:
            phase = re.search(r"I32Const \{ value: 33 \}\n[0-9a-f]+ I32Store8 .*offset: (\d+),", function)
            step = re.search(r"^([0-9a-f]+) Call \{ function_index: \d+ \}", function, re.M)
            rejected = re.search(r"I32Load8U .*offset: 4, memory: 0 \} \}\n([0-9a-f]+) I32Store8 .*offset: 1,", function)
            scratch = re.search(r"I32Load8U .*offset: 4, memory: 0", function)
            frame = re.search(r"LocalTee \{ local_index: (\d+) \}", function)
            assert phase and step and rejected and scratch and frame
            global_set = b"\x24" + unsigned(global_index)
            global_get = b"\x23" + unsigned(global_index)
            load8 = lambda local, offset: b"\x20" + unsigned(local) + b"\x2d\x00" + unsigned(offset)
            instructions[int(step[1], 16)] = constant(100) + load8(1, int(phase[1])) + b"\x6a" + global_set
            instructions[int(rejected[1], 16)] = global_get + constant(16) + b"\x6c" + load8(int(frame[1]), 4) + b"\x6a" + global_set
    if listing := os.environ.get("ENERGY_REBIND_LISTING"):
        accessors = os.environ.get("ENERGY_AUTHORITY_ACCESSORS", "543170|543106")
        authority_pairs = rf"Call \{{ function_index: ({accessors}) \}}\n[0-9a-f]+ LocalGet \{{ local_index: 0 \}}\n[0-9a-f]+ I64Load .*offset: (\d+),"
        for function in Path(listing).read_text().split("function ")[1:]:
            entry = re.search(r"^([0-9a-f]+) GlobalGet", function, re.M)
            fields = re.findall(authority_pairs, function)
            assert entry and len(fields) == 2
            instructions[int(entry[1], 16)] = b"".join(
                b"\x20\x00\x20\x01\x10" + unsigned(int(accessor)) + b"\x37\x03" + unsigned(int(slot))
                for accessor, slot in fields
            )
    last_end = first
    code_size = code_end - code_start
    instruction_sites = sorted(instructions)
    for function, start, end in functions(blob, code_start):
        left = bisect_left(instruction_sites, start)
        right = bisect_left(instruction_sites, end, left)
        matches = [(offset, instructions[offset]) for offset in instruction_sites[left:right]]
        if work_cap:
            ceiling = b"\x42" + constant(int(work_cap))[1:]
            for offset, local in receiving_work_sites(blob, start, end, receiving_call):
                matches.append((offset, b"\x22" + unsigned(local) + ceiling + b"\x20" + unsigned(local) + ceiling + b"\x54\x1b"))
                work_sites += 1
        if release_cap and function == receiving_body:
            write_release = b"\x6b\x36\x02\x0c"
            offset = blob.find(write_release, start, end)
            assert offset != -1 and blob.find(write_release, offset + 1, end) == -1
            ceiling = constant(int(release_cap))
            matches.append((offset + 1, b"\x22\x02" + ceiling + b"\x20\x02" + ceiling + b"\x49\x1b"))
            release_sites += 1
        for stage, (name, op) in enumerate(terms.items(), start=1):
            cursor = start
            while (found := blob.find(op, cursor, end)) != -1:
                print(f"[DEBUG] Wasm member stage {stage}: {name}", flush=True) if not matches else None
                matches.append((found, constant(stage) + b"\x24" + unsigned(global_index)))
                cursor = found + len(op)
        if matches:
            chunk = bytearray()
            position = start
            for found, patch in sorted(matches):
                chunk.extend(blob[position:found])
                chunk.extend(patch)
                position = found
            chunk.extend(blob[position:end])
            encoded = unsigned(len(chunk))
            parts.append((last_end, encoded + chunk, end))
            code_size += len(encoded) + len(chunk) - (end - last_end)
        last_end = end
    if work_cap:
        assert work_sites == 2
        print(f"[DEBUG] Capped {work_sites} unquoted receiving work sites at {work_cap} units", flush=True)
    if release_cap:
        assert release_sites == 1
        print(f"[DEBUG] Capped the unquoted receiving release reservation at {release_cap} bytes", flush=True)
    with Path(destination).open("wb") as output:
        output.write(blob[:8])
        for section, start, end in sections(blob):
            output.write(bytes([section]))
            if section in (6, 7) and (work_calls or work_rows or ledger_settlement or ownership_literal or terms or os.environ.get("ENERGY_HYDRATION_LISTING")):
                payload = globals_payload if section == 6 else exports_payload
                output.write(unsigned(len(payload)))
                output.write(payload)
            elif section == 10:
                output.write(unsigned(code_size))
                output.write(blob[start:first])
                cursor = first
                for begin, payload, end in parts:
                    output.write(blob[cursor:begin])
                    output.write(payload)
                    cursor = end
                output.write(blob[cursor:code_end])
            else:
                output.write(unsigned(end - start))
                output.write(blob[start:end])


def main():
    file = Path(sys.argv[1]).open("rb")
    blob = mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ)
    if blob[4:8] == b"\x0d\x00\x01\x00":
        start, end = max(((start, end) for section, start, end in sections(blob) if section == 1), key=lambda span: span[1] - span[0])
        blob = WasmSlice(blob, start, end)
    markers = sys.argv[2:]
    bounds = {section: (start, end) for section, start, end in sections(blob) if section}
    terms = {}
    for address, start, end in data_segments(blob, bounds[11][0]):
        if address is None:
            continue
        for marker in markers:
            match = blob.find(marker.encode(), start, end)
            if match != -1:
                terms[marker] = b"\x41" + padded(address + match - start)
    if destination := os.environ.get("ENERGY_INSTRUMENTED_GUEST"):
        instrument(blob, bounds, {marker: terms[marker] for marker in markers}, destination)
        return
    for function, start, end in functions(blob, bounds[10][0]):
        for marker, op in terms.items():
            pointer = blob.find(op, start, end)
            if pointer != -1:
                print(f"{function} ({start:#x}–{end:#x}) {marker} at {pointer:#x}: {blob[pointer-24:pointer+36].hex(' ')}")


if __name__ == "__main__":
    main()
