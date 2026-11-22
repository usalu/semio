set confirm off
set pagination off
set startup-with-shell off
set print thread-events off
set env RUST_MIN_STACK 67108864
starti
python
import collections
import gdb
import os
program = gdb.current_progspace().filename
for line in gdb.execute('info proc mappings', to_string=True).splitlines():
    if line.strip().endswith(program):
        gdb.write(f'[DEBUG] executable_base {line.split()[0]}\n')
        base = int(line.split()[0], 16)
        break

if os.environ.get('SEMIO_EDITOR_SAMPLE_PUBLICATION'):
    stores = collections.Counter()
    class Store(gdb.Breakpoint):
        def __init__(self, offset):
            super().__init__(f'*{base + offset:#x}', internal=True)
            self.offset = offset

        def stop(self):
            stores[hex(self.offset)] += 1
            return False

    class Census(gdb.Breakpoint):
        def __init__(self):
            super().__init__(f'*{base + 0xa75bc00:#x}', internal=True)
            self.counts = collections.Counter()
            self.total = 0

        def stop(self):
            self.counts[int(gdb.parse_and_eval('$edi'))] += 1
            self.total += 1
            if self.total in (1000, 5000, 10000, 20000, 40000):
                gdb.write(f'[DEBUG] publication units {self.total} {dict(self.counts)} store calls {dict(stores)}\n')
            return self.total == 40000

    Census()
    for address in (0x56b96b0, 0x56b9760, 0x56b98c0):
        Store(address)

if os.environ.get('SEMIO_EDITOR_SAMPLE_STORE'):
    inferior = gdb.selected_inferior()
    maximum_turns = int(os.environ.get('SEMIO_EDITOR_SAMPLE_STORE'))
    checkpoints = {1, 100, 1000, 5000, 10000, 20000, 30000, maximum_turns}
    def integer(address, width=8):
        return int.from_bytes(inferior.read_memory(address, width), 'little')

    class Enter(gdb.Breakpoint):
        def __init__(self):
            super().__init__(f'*{base + 0x56b98c0:#x}', internal=True)
            self.turns = 0
            self.current = None
            self.last = None
            self.repeated = 0

        def stop(self):
            self.turns += 1
            publication = int(gdb.parse_and_eval('$rdx'))
            grant = int(gdb.parse_and_eval('$r8'))
            result = int(gdb.parse_and_eval('$rdi'))
            phase = integer(publication + 0x3fa5, 1)
            self.current = (publication, result, phase)
            if self.turns in checkpoints or self.turns % 50000 == 0:
                bounds = tuple(integer(grant + 8 * i) for i in range(5))
                target = integer(base + 0xdede6a0) - base
                gdb.write(f'[DEBUG] Store entry {self.turns} phase {phase} bounds {bounds} batch {target:#x}\n')
            return False

    class Exit(gdb.Breakpoint):
        def stop(self):
            publication, result, phase = entering.current
            next_phase = integer(publication + 0x3fa5, 1)
            work = integer(publication + 0x3f90)
            tag = integer(result + 8, 1)
            current = (phase, next_phase, work, tag)
            entering.repeated = entering.repeated + 1 if current == entering.last and work == 0 else 0
            entering.last = current
            stop = entering.turns == maximum_turns or entering.turns > 18000 and entering.repeated >= 1000
            if entering.turns in checkpoints or entering.turns % 50000 == 0 or phase != next_phase or stop:
                header = inferior.read_memory(publication + 0x3f40, 104).hex()
                receipt = inferior.read_memory(result, 72).hex()
                details = (phase, next_phase, work, tag, header, receipt)
                gdb.write(f'[DEBUG] Store exit {entering.turns} {details}\n')
            return stop

    entering = Enter()
    Exit(f'*{base + 0x56b9958:#x}', internal=True)
end
continue
thread apply all bt 48
