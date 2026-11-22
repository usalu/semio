set confirm off
set pagination off
set print frame-arguments none
set startup-with-shell off
set env RUST_MIN_STACK 67108864
handle SIGTRAP stop print nopass
starti
python
import os, signal, threading
binary = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute("info proc mappings", to_string=True).splitlines() if line.strip().endswith(binary))
inferior = gdb.selected_inferior()
def number(address, size=8):
    return int.from_bytes(inferior.read_memory(address, size), "little")
def reg(name):
    return int(gdb.parse_and_eval("$" + name))
class Finish(gdb.FinishBreakpoint):
    def __init__(self, target, count, label):
        super().__init__(gdb.newest_frame(), internal=True)
        self.target, self.count, self.label = target, count, label
    def stop(self):
        print("[DEBUG]", self.label, "output", self.count, tuple(number(self.target + i*8) for i in range(8)))
        return False
class Call(gdb.Breakpoint):
    def __init__(self, address, label, snapshot=False):
        super().__init__("*" + hex(base + address), internal=True)
        self.label, self.snapshot, self.phase, self.count = label, snapshot, -1, 0
    def stop(self):
        owner, out = reg("rsi"), reg("rdi")
        phase = number(owner + 0x2ad2, 1) if self.snapshot else 0
        if phase != self.phase:
            self.phase, self.count = phase, 0
        self.count += 1
        if self.count in {1, 2, 10, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000}:
            print("[DEBUG]", self.label, "call", phase, self.count, tuple(reg(r) for r in ("rdx","rcx","r8","r9")))
            if self.snapshot or self.label == "store replay":
                Finish(out, self.count, self.label)
        return False
if os.environ.get("SEMIO_GDB_CASE") == "window":
    Call(0x830d6b0, "Window snapshot", True)
    Call(0x70676e0, "Window Record one")
    Call(0x9cf9eb0, "Window Record two")
else:
    Call(0x52b5fa0, "instance replay")
    Call(0x513d260, "store replay")
    Call(0x5ca4640, "derived replay")
    Call(0x5cb57b0, "native report replay")
timer = threading.Timer(float(os.environ.get("SEMIO_5D_SAMPLE_SECONDS", "30")), lambda: os.kill(inferior.pid, signal.SIGTRAP))
timer.daemon = True
timer.start()
end
continue
bt 28
kill
