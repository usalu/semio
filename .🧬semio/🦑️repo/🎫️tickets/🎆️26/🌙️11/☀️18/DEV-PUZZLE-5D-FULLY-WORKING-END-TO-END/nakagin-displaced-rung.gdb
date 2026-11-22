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
root_owner = 0

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
        global root_owner
        address = int(gdb.parse_and_eval('$rax'))
        if not address:
            return False
        data, table = number(address), number(address + 8)
        hits[(data, table)] += 1
        count = hits[(data, table)]
        if count in (1, 256, 270):
            gdb.write(f'[DEBUG] displaced child {count} data={data:#x} table={table:#x} size={number(table + 8)}\n')
            active, active_table = number(data + 0x580), number(data + 0x588)
            factory, pending = number(data + 0x590), number(data + 0x5a0)
            gdb.write(f'[DEBUG] pending={pending:#x} active={active:#x} table={active_table:#x} factory={factory:#x}\n')
            if active:
                root_owner = active
                for probe in probes:
                    probe.enabled = True
                owner = number(active + 0x7b0)
                table = number(active + 0x7b8)
                gdb.write(f'[DEBUG] arc root alias={number(active+0x930):#x} elected tag={number(active+0x938):#x} storage={number(active+0x580):#x} issuer={number(active+0x7c0):#x} closing={number(active):#x} child={owner:#x} table={table:#x}\n')
                if owner:
                    for offset in (0, 32, 40, 96, 136, 152, 176, 208):
                        gdb.write('[DEBUG] Arc child ' + gdb.execute(f'info symbol {number(table+offset):#x}', to_string=True))
        return count == 270

class Result(gdb.FinishBreakpoint):
    def __init__(self, name, out):
        super().__init__(gdb.newest_frame(), internal=True)
        self.name, self.out = name, out
    def stop(self):
        if 256 <= sum(hits.values()) < 259:
            line = ' '.join(f'{number(self.out + offset):#x}' for offset in range(0, 56, 8))
            gdb.write(f'[DEBUG] result {self.name}: {line}\n')
        return False

class ArcTrace(gdb.Breakpoint):
    def __init__(self, name, address):
        super().__init__(f'*{base + address:#x}', internal=True)
        self.name = name
    def stop(self):
        if root_owner and int(gdb.parse_and_eval('$rsi')) == root_owner and 256 <= sum(hits.values()) < 259:
            gdb.write(f'[DEBUG] Arc entered {self.name}\n')
            Result(self.name, int(gdb.parse_and_eval('$rdi')))
        return False

class Front(gdb.Breakpoint):
    def stop(self):
        if displaced:
            FrontReturn(gdb.newest_frame(), internal=True)
        return False

Disposer(f'*{base + 0x5084e20:#x}', internal=True)
Front(f'*{base + 0xdc0c930:#x}', internal=True)
probes = [ArcTrace('work_demand', 0x6b91880), ArcTrace('demands', 0x6b935e0), ArcTrace('close_reply', 0x6b90c50)]
for probe in probes:
    probe.enabled = False
end
continue
bt 10
