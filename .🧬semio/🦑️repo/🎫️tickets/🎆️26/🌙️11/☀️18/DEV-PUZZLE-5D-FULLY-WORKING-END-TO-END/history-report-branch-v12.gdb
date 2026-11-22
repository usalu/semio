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
        if self.count == (1000 if os.environ.get("SEMIO_GDB_CASE") == "window" else 2000):
            self.enabled = False
            return True
        return False
Report("*" + hex(base + (0x8220c50 if os.environ.get("SEMIO_GDB_CASE") == "window" else 0x5cb5a1d)), internal=True)
end
continue
python
frame = gdb.newest_frame()
arch = frame.architecture()
name = frame.name()
for _ in range(6000):
    now = gdb.newest_frame()
    if now.name() != name:
        break
    pc = now.pc()
    ins = arch.disassemble(pc, count=1)[0]["asm"]
    if ins.startswith(("call", "jmp", "j", "ret")):
        label = ""
        if ins.startswith("call") and "rip" in ins and "#" in ins:
            try:
                address = int(ins.split("#", 1)[1].split()[0], 16)
                target = int.from_bytes(gdb.selected_inferior().read_memory(address, 8), "little")
                label = gdb.execute("info symbol " + hex(target), to_string=True).strip().split(" in section")[0]
            except (ValueError, gdb.error):
                pass
        print("[DEBUG]", hex(pc-base), ins.split(" <", 1)[0], label[:420], flush=True)
    gdb.execute("nexti", to_string=True)
end
bt 14
kill
