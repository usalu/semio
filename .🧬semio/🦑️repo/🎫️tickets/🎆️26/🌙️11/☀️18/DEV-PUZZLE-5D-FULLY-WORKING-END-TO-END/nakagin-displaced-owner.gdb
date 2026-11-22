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
hits = collections.Counter()
displaced = False

def number(address, width=8):
    return int.from_bytes(inferior.read_memory(address, width), 'little')

class Disposer(gdb.Breakpoint):
    def stop(self):
        global displaced
        owner = int(gdb.parse_and_eval('$rsi'))
        displaced = number(owner + 16, 1) == 1
        return False

class FrontReturn(gdb.FinishBreakpoint):
    def stop(self):
        address = int(gdb.parse_and_eval('$rax'))
        if not address:
            return False
        data, table = number(address), number(address + 8)
        hits[(data, table)] += 1
        count = hits[(data, table)]
        if count in (1, 256, 2048):
            gdb.write(f'[DEBUG] displaced child {count} data={data:#x} table={table:#x} size={number(table + 8)}\n')
            active, active_table = number(data + 0x580), number(data + 0x588)
            factory, pending = number(data + 0x590), number(data + 0x5a0)
            gdb.write(f'[DEBUG] pending={pending:#x} active={active:#x} table={active_table:#x} factory={factory:#x}\n')
            if active:
                gdb.write(f'[DEBUG] nested size={number(active_table + 8)}\n')
                for slot in range(0, 153, 8):
                    gdb.write('[DEBUG] nested ' + gdb.execute(f'info symbol {number(active_table + slot):#x}', to_string=True))
                gdb.write(gdb.execute(f'x/32gx {active:#x}', to_string=True))
            for offset in range(24, 25, 8):
                gdb.write('[DEBUG] ' + gdb.execute(f'info symbol {number(table + offset):#x}', to_string=True))
        return count == 2048

class Front(gdb.Breakpoint):
    def stop(self):
        if displaced:
            FrontReturn(gdb.newest_frame(), internal=True)
        return False

Disposer(f'*{base + 0x5084e20:#x}', internal=True)
Front(f'*{base + 0xdc0c930:#x}', internal=True)
end
continue
bt 10
