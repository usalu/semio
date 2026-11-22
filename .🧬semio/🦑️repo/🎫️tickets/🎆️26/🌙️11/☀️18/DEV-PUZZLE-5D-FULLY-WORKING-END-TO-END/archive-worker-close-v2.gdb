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
latest = {}

def receipt(address):
    return struct.unpack('<4Q', inferior.read_memory(address, 32))

def progress(context):
    current = struct.unpack('<Q', inferior.read_memory(context + 0x278, 8))[0]
    return receipt(current), receipt(context + 0x1d0)

class CaptureReturn(gdb.FinishBreakpoint):
    def __init__(self, label, context, output):
        super().__init__(gdb.newest_frame(), internal=True)
        self.label, self.context, self.output = label, context, output
        self.before = progress(context)

    def stop(self):
        after = progress(self.context)
        delta = tuple(tuple(b - a for a, b in zip(previous, current)) for previous, current in zip(self.before, after))
        latest[self.label] = (delta, bytes(inferior.read_memory(self.output, 8)).hex())
        return False

class Capture(gdb.Breakpoint):
    def __init__(self, offset, label):
        super().__init__(f'*{base + offset:#x}', internal=True)
        self.label = label

    def stop(self):
        frame = gdb.newest_frame()
        CaptureReturn(self.label, int(frame.read_register('rdx')), int(frame.read_register('rdi')))
        return False

class WorkerCloseShortReceipt(gdb.Breakpoint):
    def stop(self):
        frame = gdb.newest_frame()
        reported, before, after = (int(frame.read_register(reg)) for reg in ('rdx', 'rcx', 'r8'))
        if 0 <= after - before <= reported:
            return False
        parent = frame.older()
        sp = int(parent.read_register('rsp'))
        before_address = struct.unpack('<Q', inferior.read_memory(sp + 0x40, 8))[0]
        print(f'[DEBUG] invalid subtraction reported={reported} before={before} after={after} caller={parent.pc()-base:#x}')
        for name, address in [('whole supplied', sp + 0x68), ('posted current', sp + 0x48), ('before total', before_address), ('before held', before_address + 32)]:
            print(f'[DEBUG] {name} receipt {receipt(address)}')
        print(f'[DEBUG] initializer close phases (whole delta, held delta, result header) {latest}')
        return True

Capture(0x5b38070, 'editor retire_native')
Capture(0x5b45e90, 'artifact retire_native')
Capture(0x5c46500, 'editor replay close')
Capture(0x5c47710, 'artifact replay close')
WorkerCloseShortReceipt(f'*{base + 0xb4ebe00:#x}', internal=True)
end
continue
bt 26
