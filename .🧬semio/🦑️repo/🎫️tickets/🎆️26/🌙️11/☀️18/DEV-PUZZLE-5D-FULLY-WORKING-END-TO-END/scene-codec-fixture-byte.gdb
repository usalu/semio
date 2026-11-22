set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import gdb
import mmap
import struct

program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
original = b'"deletePartWire": [1, 1, 1, 1, 97, 1, 1, 6, 0]'
corrected = b'"deletePartWire": [1, 1, 1, 1, 97, 1, 0, 6, 0]'
with open(program, 'rb') as binary, mmap.mmap(binary.fileno(), 0, access=mmap.ACCESS_READ) as data:
    offset = data.find(original)
    assert offset >= 0 and data.find(original, offset + 1) < 0
    header = struct.unpack_from('<16sHHIQQQIHHHHHH', data)
    segments = [struct.unpack_from('<IIQQQQQQ', data, header[5] + i * header[9]) for i in range(header[10])]
    address = next(base + vaddr + offset - start for kind, _, start, vaddr, _, size, *_ in segments if kind == 1 and start <= offset < start + size)
    gdb.selected_inferior().write_memory(address, corrected)
    gdb.write(f'[DEBUG] in-memory fixture corrected at {address:#x}; executable remains unchanged\n')
end
break 'semio_framework_dsl_record_model::receiving::owned::contract::invalid'
continue
bt 20
