set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import gdb
import struct
program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
gdb.write(f'[DEBUG] executable_base {base:#x}\n')
inferior = gdb.selected_inferior()
oversized = 1572864

class Leave(gdb.FinishBreakpoint):
    def __init__(self, output, offset):
        super().__init__(gdb.newest_frame(), internal=True)
        self.output = output
        self.offset = offset

    def stop(self):
        words = struct.unpack('<8Q', inferior.read_memory(self.output, 64))
        if oversized in words:
            gdb.write(f'[DEBUG] release quote {self.offset:#x} returned {words}\n')
            gdb.write(gdb.execute('bt 8', to_string=True))
            frame = gdb.newest_frame()
            gdb.write(f'[DEBUG] caller {frame.pc() - base:#x}\n')
            return True
        return False

class Enter(gdb.Breakpoint):
    def __init__(self, offset):
        super().__init__(f'*{base + offset:#x}', internal=True)
        self.offset = offset

    def stop(self):
        Leave(int(gdb.parse_and_eval('$rdi')), self.offset)
        return False

gate = gdb.Breakpoint(f'*{base + 0xa4baaa0:#x}', internal=True)
gate.condition = '*(unsigned long long *)($rdi+16) == 1572864'
end
continue
bt 10
python
gate.enabled = False
for address in (0x5504200, 0x55052c0, 0x5525ba0, 0x611b6e0):
    Enter(address)
end
continue
