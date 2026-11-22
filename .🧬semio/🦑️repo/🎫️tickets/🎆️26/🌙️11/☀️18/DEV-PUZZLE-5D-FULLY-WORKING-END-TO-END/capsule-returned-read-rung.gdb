set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
set env SEMIO_RUNTIME_DIAGNOSTICS 1
starti
python
import gdb
import struct

program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
inferior = gdb.selected_inferior()
reads = 0
inside_read = False
armed = False

def register(name):
    return int(gdb.newest_frame().read_register(name))

def numbers(address, count):
    return struct.unpack(f'<{count}Q', inferior.read_memory(address, count * 8))

class QuoteResult(gdb.FinishBreakpoint):
    def __init__(self, addr):
        super().__init__(gdb.newest_frame(), internal=True)
        self.addr = addr
    def stop(self):
        print(f'[DEBUG] exact read box demand {numbers(self.addr, 5)}')
        return False

class Quote(gdb.Breakpoint):
    def stop(self):
        if inside_read:
            QuoteResult(register('rdi'))
        return False

class ReadResult(gdb.FinishBreakpoint):
    def __init__(self, addr):
        super().__init__(gdb.newest_frame(), internal=True)
        self.addr = addr
    def stop(self):
        global inside_read
        inside_read = False
        quote.enabled = False
        print(f'[DEBUG] document read return receipt {numbers(self.addr, 6)}')
        return reads >= 6

class Read(gdb.Breakpoint):
    def stop(self):
        global reads, inside_read
        reads += 1
        inside_read = True
        quote.enabled = True
        effect = register('rcx')
        print(f'[DEBUG] original read return #{reads} actual effect grant {numbers(effect+0x30, 5)}')
        ReadResult(register('rdi'))
        return False

class Stall(gdb.Breakpoint):
    def stop(self):
        global armed
        count = numbers(register('rdi'), 1)[0]
        if count == 511:
            print('[DEBUG] arming returned-read probes after 511 unchanged publication units')
            read.enabled = True
            armed = True
        return armed and count == 1023

quote = Quote(f'*{base+0xe428040:#x}', internal=True)
read = Read(f'*{base+0x5251aa0:#x}', internal=True)
quote.enabled = False
read.enabled = False
Stall(f'*{base+0xa7917e0:#x}', internal=True)
end
continue
bt 15
