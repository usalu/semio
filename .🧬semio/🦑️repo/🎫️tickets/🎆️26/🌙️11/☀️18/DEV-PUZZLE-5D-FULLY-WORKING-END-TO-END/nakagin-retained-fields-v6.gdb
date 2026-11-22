set pagination off
set debuginfod enabled off
set startup-with-shell off
set print thread-events off
set env RUST_MIN_STACK 67108864
starti
python
import gdb

program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
inferior = gdb.selected_inferior()
samples = [1, 2, 3, 4, 5, 6, 7, 8, 32, 256, 1000, 1001, 1002, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144]

def words(address, count):
    data = bytes(inferior.read_memory(address, count * 8))
    return [int.from_bytes(data[offset:offset+8], 'little') for offset in range(0, len(data), 8)]

class FixtureHorizon(gdb.Breakpoint):
    def stop(self):
        before = int(gdb.parse_and_eval('$rax'))
        if before == 131072:
            gdb.execute('set $rax = 524288')
            gdb.write(f'[DEBUG] diagnostic fixture close horizon {before} => 524288 at breakpoint, unchanged operation grants\n')
        return False

class FieldReturn(gdb.FinishBreakpoint):
    def __init__(self, frame, number, result):
        super().__init__(frame, internal=True)
        self.sample_index = number
        self.result = result
    def stop(self):
        gdb.write(f'[DEBUG] fields step #{self.sample_index} returned words={words(self.result, 8)}\n')
        return False

class Fields(gdb.Breakpoint):
    def __init__(self, address):
        super().__init__(address, internal=True)
        self.previous = 0
    def stop(self):
        number = samples.pop(0)
        self.previous = number
        address = int(gdb.parse_and_eval('$rsi'))
        result = int(gdb.parse_and_eval('$rdi'))
        history = words(address + 0x28, 1)[0]
        gdb.write(f'[DEBUG] fields step #{number} owner={address:#x} history={history:#x} fields_3c0={[hex(value) for value in words(history + 0x3c0, 9)]} fields_458={[hex(value) for value in words(history + 0x458, 9)]}\n')
        if samples:
            self.ignore_count = samples[0] - number - 1
        else:
            self.enabled = False
        FieldReturn(gdb.newest_frame(), number, result)
        return False

FixtureHorizon(f'*{base + 0xb888e8e:#x}', internal=True)
Fields(f'*{base + 0x5e48a70:#x}')
end
continue
