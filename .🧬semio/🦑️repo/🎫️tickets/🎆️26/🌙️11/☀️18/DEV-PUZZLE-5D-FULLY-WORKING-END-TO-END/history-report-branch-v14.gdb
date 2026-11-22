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
class Report(gdb.Breakpoint):
    count = 0
    def stop(self):
        self.count += 1
        return self.count == int(os.environ.get("SEMIO_GDB_OCCURRENCE", "1000"))
report = Report("*" + hex(base + int(os.environ["SEMIO_GDB_OFFSET"], 16)), internal=True)
end
continue
python
report.enabled = False
frame = gdb.newest_frame()
arch = frame.architecture()
name = frame.name()
if os.environ.get("SEMIO_GDB_STACK"):
    stack = int(gdb.parse_and_eval("$rsp"))
    for row in os.environ["SEMIO_GDB_STACK"].split(","):
        offset, count = (int(part, 16) for part in row.split("/"))
        bytes_ = gdb.selected_inferior().read_memory(stack+offset, 8*count)
        values = tuple(int.from_bytes(bytes_[8*i:8*(i+1)], "little") for i in range(count))
        print("[DEBUG] stack", hex(offset), values, flush=True)
visited = set()
for _ in range(10000):
    now = gdb.newest_frame()
    if now.name() != name:
        break
    pc = now.pc()
    ins = arch.disassemble(pc, count=1)[0]["asm"]
    if ins.startswith(("call", "jmp", "j", "ret")) and pc not in visited:
        visited.add(pc)
        label = ""
        if ins.startswith("call") and "rip" in ins and "#" in ins:
            try:
                address = int(ins.split("#", 1)[1].split()[0], 16)
                target = int.from_bytes(gdb.selected_inferior().read_memory(address, 8), "little")
                label = gdb.execute("info symbol " + hex(target), to_string=True).strip().split(" in section")[0]
            except (ValueError, gdb.error):
                pass
        print("[DEBUG]", hex(pc-base), ins.split(" <", 1)[0], label[-600:], flush=True)
    gdb.execute("nexti", to_string=True)
end
bt 16
kill
