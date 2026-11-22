"""📐 Decode a bounded Wasm instruction window for Note grant diagnostics."""
from pathlib import Path
from runpy import run_path
import sys

helpers = run_path(str(Path(__file__).with_name("symbolicate-note-startup.py")))
data = Path(sys.argv[1]).read_bytes()
position, end = int(sys.argv[2]), int(sys.argv[3])
unsigned, signed = helpers["leb"], helpers["signed_leb"]

registers = {0x0c: "br", 0x0d: "br_if", 0x10: "call", 0x20: "local.get", 0x21: "local.set", 0x22: "local.tee", 0x23: "global.get", 0x24: "global.set"}
single = {0: "unreachable", 1: "nop", 5: "else", 0xb: "end", 0xf: "return", 0x1a: "drop", 0x1b: "select", 0x45: "eqz", 0x46: "eq", 0x47: "ne", 0x49: "lt_u", 0x4b: "gt_u", 0x4d: "le_u", 0x4f: "ge_u", 0x6a: "add", 0x6b: "sub", 0x71: "and", 0x72: "or", 0x86: "i64.shl", 0xad: "i64.extend_i32_u"}
memory = {0x28: "i32.load", 0x29: "i64.load", 0x2d: "i32.load8_u", 0x36: "i32.store", 0x37: "i64.store", 0x3a: "i32.store8"}
single.update({
    0x48: "i32.lt_s", 0x4a: "i32.gt_s", 0x4c: "i32.le_s", 0x4e: "i32.ge_s",
    0x50: "i64.eqz", 0x51: "i64.eq", 0x52: "i64.ne", 0x53: "i64.lt_s", 0x54: "i64.lt_u",
    0x55: "i64.gt_s", 0x56: "i64.gt_u", 0x57: "i64.le_s", 0x58: "i64.le_u", 0x59: "i64.ge_s", 0x5a: "i64.ge_u",
    0x6c: "i32.mul", 0x73: "i32.xor", 0x74: "i32.shl", 0x75: "i32.shr_s", 0x76: "i32.shr_u",
    0x7c: "i64.add", 0x7d: "i64.sub", 0x7e: "i64.mul", 0x83: "i64.and", 0x84: "i64.or",
    0x85: "i64.xor", 0x87: "i64.shr_s", 0x88: "i64.shr_u", 0xa7: "i32.wrap_i64", 0xac: "i64.extend_i32_s",
})
if "--instructions" not in sys.argv[4:]:
    count, position = unsigned(data, position)
    for _ in range(count):
        _, position = unsigned(data, position)
        position += 1
while position < end:
    offset = position
    op = data[position]
    position += 1
    if op in registers:
        number, position = unsigned(data, position)
        operands = registers[op], number
    elif op in memory:
        alignment, position = unsigned(data, position)
        number, position = unsigned(data, position)
        operands = memory[op], alignment, number
    elif op in (2, 3, 4, 0x41, 0x42):
        number, position = signed(data, position)
        operands = {2: "block", 3: "loop", 4: "if", 0x41: "i32.const", 0x42: "i64.const"}[op], number
    elif op == 0x0e:
        count, position = unsigned(data, position)
        branches = []
        for _ in range(count + 1):
            branch, position = unsigned(data, position)
            branches.append(branch)
        operands = "br_table", branches
    elif op == 0x11:
        signature, position = unsigned(data, position)
        table, position = unsigned(data, position)
        operands = "call_indirect", signature, table
    elif op == 0xfc:
        number, position = unsigned(data, position)
        if number not in (10, 11):
            raise ValueError((offset, op, number))
        source, position = unsigned(data, position)
        destination = None
        if number == 10:
            destination, position = unsigned(data, position)
        operands = "memory.copy" if number == 10 else "memory.fill", source, destination
    elif op in single:
        operands = (single[op],)
    else:
        raise ValueError((offset, hex(op)))
    print(offset, *operands)
