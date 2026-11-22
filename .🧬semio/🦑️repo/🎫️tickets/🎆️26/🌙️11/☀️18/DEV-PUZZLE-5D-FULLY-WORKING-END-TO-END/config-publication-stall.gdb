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
store_breakpoints = []
unit = 0

def register(name):
    return int(gdb.parse_and_eval('$' + name))

def number(address, width=8):
    return int.from_bytes(inferior.read_memory(address, width), 'little')

class Unit(gdb.Breakpoint):
    def stop(self):
        global unit
        unit += 1
        if unit == 1:
            gdb.write(f'[DEBUG] Store capture enabled after {unit} publication units\n')
            for breakpoint in store_breakpoints:
                breakpoint.enabled = True
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
        receipt = struct.unpack_from('<5Q', result, 64)
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

for offset in (0x59d99e0,):
    for breakpoint in (Enter(offset), Exit(offset)):
        breakpoint.enabled = False
        store_breakpoints.append(breakpoint)
unit_breakpoint = Unit(f'*{base + 0x5611810:#x}', internal=True)
unit_breakpoint.ignore_count = 0
Fault(f'*{base + 0x6418d00:#x}', internal=True)
class Blocked(gdb.Breakpoint):
    def stop(self):
        gdb.write(f'[DEBUG] blocked branch at {register("pc") - base:#x}, publication unit {unit}\n')
        self.enabled = False
        return True

for offset in [92785409, 92785713, 92785992, 92786634, 92789696, 92790793, 92791537, 92797663, 92798622, 92798804, 92802793, 92803246, 92803418, 92804559, 92806488, 92806928, 92807013, 92808034, 92814396, 92815841, 92816073, 92816906, 92817138, 92817223, 92819489, 92819721, 92820539, 92830651]:
    Blocked(f'*{base + offset:#x}', internal=True)
end
continue
bt 22
