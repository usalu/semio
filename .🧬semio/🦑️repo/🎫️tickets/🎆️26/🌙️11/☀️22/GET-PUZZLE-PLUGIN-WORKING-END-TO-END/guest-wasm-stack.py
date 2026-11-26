"""🔍️ Inspect the stack global and fault instructions of a stripped core WebAssembly module."""

from argparse import ArgumentParser
from mmap import ACCESS_READ, mmap


def leb(data, position, signed=False):
    value = 0
    shift = 0
    while True:
        byte = data[position]
        position += 1
        value |= (byte & 127) << shift
        shift += 7
        if byte < 128:
            if signed and byte & 64:
                value -= 1 << shift
            return value, position


def name(data, position):
    length, start = leb(data, position)
    return data[start : start + length].decode("utf-8"), start + length


def limits(data, position):
    flags, position = leb(data, position)
    lower, position = leb(data, position)
    upper = None
    if flags & 1:
        upper, position = leb(data, position)
    return (lower, upper), position


def imported_functions(data, position):
    count, position = leb(data, position)
    imported = 0
    for _ in range(count):
        _, position = name(data, position)
        _, position = name(data, position)
        kind = data[position]
        position += 1
        if kind == 0:
            imported += 1
            _, position = leb(data, position)
        elif kind == 1:
            _, position = limits(data, position + 1)
        elif kind == 2:
            _, position = limits(data, position)
        elif kind == 3:
            position += 2
        elif kind == 4:
            _, position = leb(data, position + 1)
        else:
            raise ValueError(kind)
    return imported


def report(data, traps):
    assert data[:8] == b"\0asm\x01\0\0\0"
    position = 8
    imported = 0
    observed_stack = 0
    stack_start = None
    memory_maximum = None
    while position < len(data):
        section = data[position]
        size, start = leb(data, position + 1)
        position = start + size
        if section == 0:
            label, _ = name(data, start)
            print("custom section", label, size)
        if section == 2:
            imported = imported_functions(data, start)
        if section == 5:
            count, begin = leb(data, start)
            memory, _ = limits(data, begin)
            memory_maximum = memory[1]
            print("memory count", count, "minimum/maximum pages", memory)
        if section == 6:
            count, begin = leb(data, start)
            if data[begin : begin + 3] == b"\x7f\x01\x41":
                stack_start, _ = leb(data, begin + 3, signed=True)
                print("initial mutable i32 global 0 (stack pointer)", stack_start, "total globals", count)
        if section == 10:
            count, body = leb(data, start)
            for function in range(imported, imported + count):
                length, contents = leb(data, body)
                body = contents + length
                if function not in traps:
                    continue
                groups, ops = leb(data, contents)
                for _ in range(groups):
                    _, ops = leb(data, ops)
                    ops += 1
                fault = traps[function]
                assert ops <= fault < body
                print("function", function, "fault offset", hex(fault), "within instructions", fault - ops)
                print(" entry bytes", data[ops : ops + 45].hex(" "))
                print(" fault bytes", data[max(fault - 20, ops) : fault].hex(" "), "|", data[fault : min(fault + 25, body)].hex(" "))
                if data[ops] == 0x23:
                    stack, instruction = leb(data, ops + 1)
                    if stack == 0 and data[instruction] == 0x41:
                        frame, next_instruction = leb(data, instruction + 1, signed=True)
                        if data[next_instruction] == 0x6b:
                            observed_stack += frame
                            print(" stack frame bytes", frame)
    if observed_stack:
        print("observed stack-frame total bytes", observed_stack)
    return stack_start, memory_maximum


parser = ArgumentParser()
parser.add_argument("core")
parser.add_argument("frames", nargs="*")
parser.add_argument("--expected-stack", type=int)
parser.add_argument("--expected-memory-maximum", type=int)
args = parser.parse_args()
with open(args.core, "rb") as source, mmap(source.fileno(), 0, access=ACCESS_READ) as data:
    stack, maximum = report(data, {int(function): int(offset, 16) for function, offset in (trap.split(":") for trap in args.frames)})
    if args.expected_stack is not None:
        assert stack == args.expected_stack, (stack, args.expected_stack)
    if args.expected_memory_maximum is not None:
        assert maximum == args.expected_memory_maximum, (maximum, args.expected_memory_maximum)
