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
inferior = gdb.selected_inferior()
attempts = 0

class CacheReturn(gdb.FinishBreakpoint):
    def __init__(self, output):
        super().__init__(gdb.newest_frame(), internal=True)
        self.output = output

    def stop(self):
        row = struct.unpack('<3Q', inferior.read_memory(self.output, 24))
        gdb.write(f'[DEBUG] media preparation #{attempts} current retained cache {row}\n')
        return attempts >= 8

class CacheEnter(gdb.Breakpoint):
    def stop(self):
        global attempts
        attempts += 1
        CacheReturn(int(gdb.parse_and_eval('$rdi')))
        return False

CacheEnter(f'*{base + 0x65655c0:#x}', internal=True)
end
continue
bt 16
