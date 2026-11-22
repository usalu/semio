set confirm off
set pagination off
set print frame-arguments none
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import os
binary = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute("info proc mappings", to_string=True).splitlines() if line.strip().endswith(binary))
class Factory(gdb.Breakpoint):
    def stop(self):
        address = int(gdb.parse_and_eval("$rdx"))
        return address > 0x1000 and int.from_bytes(gdb.selected_inferior().read_memory(address, 8), "little") == 0
with open(os.environ["SEMIO_HISTORY_SYMBOLS"]) as symbols:
    for line in symbols:
        if line.rstrip().endswith("::admit_original"):
            Factory("*" + hex(base + int(line.split()[0], 16)), internal=True)
end
continue
bt 26
kill
