set pagination off
set debuginfod enabled off
set auto-solib-add off
set environment RUST_MIN_STACK 67108864
starti
python
import gdb
import struct
inf = gdb.selected_inferior()
line = next(row for row in open('/proc/%d/maps' % inf.pid) if 'semio_s_artifact_puzzle_5d_v1-f62a359f9cc1d4ad' in row and row.split()[2] == '00000000')
base = int(line.split('-')[0], 16)
def read(addr, length):
    return struct.unpack('<' + 'Q' * length, inf.read_memory(addr, 8 * length))
class Probe(gdb.Breakpoint):
    def __init__(self, at, label):
        self.label = label
        super().__init__('*0x%x' % (base + at))
    def stop(self):
        frame = gdb.newest_frame()
        reg = lambda r: int(frame.read_register(r))
        rsp = reg('rsp')
        if self.label == 'enter':
            print('[DEBUG] clone body, Work ceiling:', reg('rcx'), reg('r8'))
        elif self.label == 'grant':
            print('[DEBUG] grant and demand:', read(reg('rdi'), 5), read(reg('rsi'), 4))
        elif self.label == 'result':
            print('[DEBUG] physical admission:', reg('al'))
        elif self.label == 'work':
            print('[DEBUG] selected and remaining Work:', read(rsp+0x280, 1), read(rsp+0x4c0, 1))
        elif self.label == 'empty':
            print('[DEBUG] empty root clone result, quoted Work and demand:', read(rsp+0x280, 1), read(rsp+0x3d8, 4))
        return False
Probe(0x76b1f80, 'enter')
Probe(0x768c192, 'grant')
Probe(0x768c198, 'result')
Probe(0x768c11e, 'work')
Probe(0x768bfc8, 'empty')
end
continue
