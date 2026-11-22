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
gdb.write(f'[DEBUG] executable_base {base:#x}\n')
previous = None
run_length = 0
last_turn = 0
turn = 0

def read_words(address, count):
    return struct.unpack(f'<{count}Q', bytes(inferior.read_memory(address, count * 8)))

class Turn(gdb.Breakpoint):
    def stop(self):
        global turn
        turn += 1
        return False

class Gate(gdb.Breakpoint):
    def __init__(self, address, name):
        super().__init__(f'*{base + address:#x}', internal=True)
        self.name = name

    def stop(self):
        global previous, run_length, last_turn
        run_length = run_length + 1 if previous == self.name and last_turn + 1 == turn else 1
        previous, last_turn = self.name, turn
        if run_length < 8:
            return False
        stack = int(gdb.parse_and_eval('$rsp'))
        future, = read_words(stack + 0x198, 1)
        context, *grant = read_words(future + 0x10, 6)
        context_fuel, = read_words(context + 0x2d0, 1)
        gdb.write(f'[DEBUG] turn {turn} gate {self.name} streak {run_length} raw fuel {context_fuel} grant {grant}\n')
        return True

Turn(f'*{base + 0x4be9050:#x}', internal=True)
Gate(0x4be9205, 'caller-yield')
Gate(0x4be9247, 'local-interaction-receipt')
Gate(0x4be9289, 'constructor-receipt')
Gate(0x4be93ab, 'extension-or-operation-receipt')
end
continue
bt 16
