set confirm off
set pagination off
set startup-with-shell off
set print thread-events off
set env RUST_MIN_STACK 67108864
starti
python
import collections
import gdb
import struct

program = gdb.current_progspace().filename
mapping = next(line for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
image = int(mapping.split()[0], 16)
calls = collections.Counter()

class Entry(gdb.Breakpoint):
    def __init__(self, name, offset):
        super().__init__(f'*{image + offset:#x}', internal=True)
        self.name = name

    def stop(self):
        calls[self.name] += 1
        count = calls[self.name]
        if self.name in ('demand', 'work_step') and (count <= 8 or count & (count - 1) == 0):
            owner = int(gdb.parse_and_eval('$rsi'))
            payload = gdb.selected_inferior().read_memory(owner + 0x2a0, 0x48)
            gdb.write(f'[DEBUG] {self.name} calls={count} page={struct.unpack("<9Q", payload)}\n')
        if self.name == 'job' and (count <= 16 or count & (count - 1) == 0):
            owner = int(gdb.parse_and_eval('$rsi'))
            phase = bytes(gdb.selected_inferior().read_memory(owner + 0x74b7, 1))[0]
            gdb.write(f'[DEBUG] job calls={count} phase={phase}\n')
        if self.name == 'advance_session' and (count <= 10 or count & (count - 1) == 0):
            owner = int(gdb.parse_and_eval('$rsi'))
            phase = bytes(gdb.selected_inferior().read_memory(owner + 0x8e29, 1))[0]
            gdb.write(f'[DEBUG] session admission calls={count} phase={phase}\n')
        if self.name == 'prepared_grant' and (count <= 12 or count & (count - 1) == 0):
            stack = int(gdb.parse_and_eval('$rsp'))
            inferior = gdb.selected_inferior()
            grant = struct.unpack('<5Q', inferior.read_memory(stack + 0xce0, 40))
            context = struct.unpack('<Q', inferior.read_memory(stack + 0xb0, 8))[0]
            page_granted = bytes(inferior.read_memory(context + 0x372, 1))[0]
            gdb.write(f'[DEBUG] preparation calls={count} grant={grant} payload_granted={page_granted}\n')
        if self.name == 'result' and (count <= 12 or count & (count - 1) == 0):
            result = int(gdb.parse_and_eval('$rax'))
            fields = struct.unpack('<6Q', gdb.selected_inferior().read_memory(result, 48))
            gdb.write(f'[DEBUG] result calls={count} contents={fields}\n')
            if fields[1] in (1, 2) and fields[2] and 0 < fields[3] < 512:
                reason = bytes(gdb.selected_inferior().read_memory(fields[2], fields[3])).decode(errors='replace')
                gdb.write(f'[DEBUG] reason {reason}\n')
        if self.name == 'worker' and count in (100, 500, 1000, 2000, 3000):
            gdb.write(f'[DEBUG] counts {dict(calls)}\n')
        return self.name == 'worker' and count == 3000

Entry('worker', 0x63802b0)
Entry('result', 0x6376382)
Entry('producer', 0x63977a0)
Entry('job', 0x6691190)
Entry('prepared_grant', 0xcb2bd6f)
Entry('origin_body', 0xcb2bead)
Entry('origin_publication', 0xcb2bf02)
Entry('origin_admitted', 0xcb2c008)
for offset in (0x4fe8210, 0x4fea1e0, 0x92ef320):
    Entry('advance_authority', offset)
for offset in (0x6330910, 0x6338d20, 0x9296190):
    Entry('admit', offset)
for offset in (0x4f058a0, 0x4f10d20, 0x90ff8c0):
    Entry('advance_session', offset)
for offset in (0x5d9a370, 0x5d9c320, 0x90f7ae0):
    Entry('pump', offset)
for offset in (0x5d9ab40, 0x5d9caf0, 0x90f82b0):
    Entry('acknowledge', offset)
for offset in (0x5d9ac60, 0x5d9cbd0, 0x90f8390):
    Entry('resume', offset)
Entry('demand', 0x66a8da0)
Entry('work_step', 0x66a9280)
end
continue
bt 18
