set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import gdb

program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
inferior = gdb.selected_inferior()
samples = 0

def number(address, size=8):
    return int.from_bytes(inferior.read_memory(address, size), 'little')

class Disposer(gdb.Breakpoint):
    def stop(self):
        global samples
        samples += 1
        owner = int(gdb.parse_and_eval('$rsi'))
        root, table = number(owner), number(owner + 8)
        phase, subphase, started = [number(owner + offset, 1) for offset in (16, 17, 18)]
        if samples <= 4 or samples == 32:
            gdb.write(f'[DEBUG] disposer sample {samples}: phase={phase}/{subphase} started={started} active={root:#x} table={table:#x}\n')
            if root and table:
                for offset in (24, 32, 40):
                    gdb.write(gdb.execute(f'info symbol {number(table+offset):#x}', to_string=True))
        return samples == 32

last = Disposer(f'*{base + 0x5084e20:#x}', internal=True)
last.ignore_count = 10000
end
continue
bt 14
