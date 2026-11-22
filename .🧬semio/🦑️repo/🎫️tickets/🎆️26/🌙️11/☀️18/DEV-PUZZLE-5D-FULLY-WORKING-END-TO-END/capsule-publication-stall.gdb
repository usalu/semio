set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import collections
import gdb
import struct

program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
inferior = gdb.selected_inferior()
current = {}
outcomes = collections.Counter()
last = collections.deque(maxlen=16)
unit = 0

def register(name):
    return int(gdb.parse_and_eval('$' + name))

def number(address, width=8):
    return int.from_bytes(inferior.read_memory(address, width), 'little')

class Unit(gdb.Breakpoint):
    def stop(self):
        global unit
        unit += 1
        return False

class Enter(gdb.Breakpoint):
    def __init__(self, offset):
        super().__init__(f'*{base + offset:#x}', internal=True)
        self.offset = offset

    def stop(self):
        current[self.offset] = (register('rdx'), register('rdi'), register('rcx'))
        return False

class Exit(gdb.Breakpoint):
    def __init__(self, offset):
        super().__init__(f'*{base + offset + 0xa4:#x}', internal=True)
        self.offset = offset

    def stop(self):
        publication, output, body = current[self.offset]
        phase = number(publication + 0x3fa5, 1)
        work = number(publication + 0x3f90)
        result = bytes(inferior.read_memory(output, 112))
        tag = result[8]
        checkpoint = struct.unpack_from('<II', result, 56)
        receipt = struct.unpack_from('<4Q', result, 64)
        outcome = (hex(self.offset), phase, tag, work)
        outcomes[outcome] += 1
        last.append((unit, outcome, body, checkpoint, receipt))
        return False

class Fault(gdb.Breakpoint):
    def stop(self):
        gdb.write(f'[DEBUG] stall reported after {unit} publication calls; {sum(outcomes.values())} captured Store calls {dict(outcomes)}\n')
        for sample in last:
            gdb.write(f'[DEBUG] Store return {sample}\n')
        return True

for offset in (0x59d9930, 0x59d99e0, 0x59d9b40, 0x87e40e0, 0x87e4190):
    Enter(offset)
    Exit(offset)
Unit(f'*{base + 0x5611810:#x}', internal=True)
Fault(f'*{base + 0x6418d00:#x}', internal=True)
end
continue
bt 22
