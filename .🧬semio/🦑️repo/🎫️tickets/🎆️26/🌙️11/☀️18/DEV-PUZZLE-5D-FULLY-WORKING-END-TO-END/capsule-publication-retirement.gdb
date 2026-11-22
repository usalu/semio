set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import collections
import gdb

program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
inferior = gdb.selected_inferior()
gdb.write(f'[DEBUG] executable_base {base:#x}\n')
fault_seen = False
results = collections.Counter()
calls = collections.Counter()

def register(name):
    return int(gdb.parse_and_eval('$' + name))

class Returned(gdb.FinishBreakpoint):
    def __init__(self, label, output):
        super().__init__(gdb.newest_frame(), internal=True)
        self.label = label
        self.output = output

    def stop(self):
        result = bytes(inferior.read_memory(self.output, 64)).hex()
        index = calls[self.label]
        results[self.label, result[:32]] += 1
        if index <= 10 or index in (16, 32, 64, 128, 256):
            gdb.write(f'[DEBUG] return {self.label} #{index} {result}\n')
        return False

class Trace(gdb.Breakpoint):
    def __init__(self, label, address):
        super().__init__(f'*{base + address:#x}', internal=True)
        self.label = label
        self.enabled = False

    def stop(self):
        calls[self.label] += 1
        index = calls[self.label]
        if self.label == 'retirement' and index == 300:
            gdb.write(f'[DEBUG] retirement calls {dict(calls)} outcomes {dict(results)}\n')
            return True
        if index <= 10 or index in (16, 32, 64, 128, 256):
            Returned(self.label, register('rdi'))
        return False

class Fault(gdb.Breakpoint):
    def stop(self):
        self.enabled = False
        gdb.write('[DEBUG] genuine runtime publication fault captured\n')
        for probe in probes:
            probe.enabled = True
        return False

probes = [
    Trace('retirement', 0x55fcce0),
    Trace('granted_demand', 0x4c91050),
    Trace('granted_step', 0x4c8d4f0),
]
Fault(f'*{base + 0x4c7e450:#x}', internal=True)
end
continue
bt 20
