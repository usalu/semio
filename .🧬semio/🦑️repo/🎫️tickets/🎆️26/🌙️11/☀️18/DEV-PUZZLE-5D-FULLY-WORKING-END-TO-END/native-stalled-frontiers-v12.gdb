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
inferior = gdb.selected_inferior()
def value(address, size=8):
    return int.from_bytes(inferior.read_memory(address, size), "little")
def reg(name):
    return int(gdb.parse_and_eval("$" + name))
class WireQuote(gdb.FinishBreakpoint):
    def __init__(self, target):
        super().__init__(gdb.newest_frame(), internal=True)
        self.target = target
    def stop(self):
        result = tuple(value(self.target + i*8) for i in range(5))
        if result[0] == 2**64-2 and result[1] == 2864 and os.environ.get("SEMIO_GDB_PUBLISH_WIRE") == "1":
            inferior.write_memory(self.target + 8, (2880).to_bytes(8, "little"))
        if result not in quotes:
            quotes.add(result)
            print("[DEBUG] binary typed Source quote", result, flush=True)
        return False
quotes = set()
class Sample(gdb.Breakpoint):
    def __init__(self, offset, label, mode):
        super().__init__("*" + hex(base + offset), internal=True)
        self.label, self.mode, self.count, self.phase = label, mode, 0, -1
    def stop(self):
        phase = value(reg("rsi") + 0x3e0c, 1) if self.mode in {"wire","quote"} else 0
        if self.mode == "quote":
            if phase == 5:
                WireQuote(reg("rdi"))
            return False
        if self.mode == "window stage":
            phase = value(reg("rsi") + 0x16739, 1)
        if self.mode == "initializer":
            phase = value(reg("rdi") + 0x11b90, 1)
        if phase != self.phase:
            self.phase, self.count = phase, 0
        self.count += 1
        if self.count in {1, 2, 50, 200, 1000, 2000, 5000, 10000, 20000}:
            if self.mode == "wire":
                grant = reg("r9")
                data = (reg("r8"), tuple(value(grant+i*8) for i in range(5)))
            elif self.mode == "deadline":
                stack = reg("rsp")
                grant = value(stack + 0x378)
                data = (reg("al"), value(stack + 0x2b680), tuple(value(grant+i*8) for i in range(5)))
            else:
                data = ()
            print("[DEBUG]", self.label, phase, self.count, data, flush=True)
        if self.mode == "wire" and phase in {0, 5, 6} and os.environ.get("SEMIO_GDB_LOWER_QUOTE") == "1":
            gdb.execute("set $r8 = $r8 - 1", to_string=True)
        return False
class WindowRefusal(gdb.Breakpoint):
    seen = set()
    def stop(self):
        if window_stage.phase < 19:
            return False
        count = reg("rcx")
        if not 0 < count < 1024:
            return False
        try:
            message = bytes(inferior.read_memory(reg("rdx"), count)).decode()
        except (gdb.MemoryError, UnicodeError):
            return False
        if message not in self.seen:
            self.seen.add(message)
            print("[DEBUG] Window original refusal", message, flush=True)
            gdb.execute("bt 9")
            if "fixture turn bound" in message:
                print("[DEBUG] Window frontier at bound", window_stage.phase, window_stage.count, window_initializer.phase, window_initializer.count, window_close.count, flush=True)
        return False
class WindowOpenFailure(gdb.Breakpoint):
    def stop(self):
        failure = reg("rsp") + 0xee90
        messages = set()
        for offset in range(0, 0xbd0 - 8, 8):
            address, length = value(failure + offset), value(failure + offset + 8)
            if not 12 <= length <= 512 or not (base <= address < base + 0xf000000 or 0x10000000000 < address < 0x800000000000):
                continue
            try:
                message = bytes(inferior.read_memory(address, length)).decode()
            except (gdb.MemoryError, UnicodeError):
                continue
            if message.isprintable() and " " in message and message not in messages:
                messages.add(message)
                print("[DEBUG] Window failure field", hex(offset), message, flush=True)
        return False
if os.environ.get("SEMIO_GDB_CASE") == "window":
    Sample(0x7d77690, "binary receiver advance", "wire")
    Sample(0x7d777e0, "binary receiver demand", "quote")
    window_stage = Sample(0x821acd0, "Window stage", "window stage")
    Sample(0x7d741c0, "binary owned output", "count")
    window_initializer = Sample(0x86e23e0, "Window initializer phase", "initializer")
    Sample(0x88563d0, "Window initialization fold", "count")
    window_close = Sample(0xb695430, "Window command closure", "count")
    WindowRefusal("*" + hex(base + 0xe98c020), internal=True)
    WindowOpenFailure("*" + hex(base + 0xb6963f2), internal=True)
else:
    Sample(0x5cb59bc, "history deadline/work/grant", "deadline")
    Sample(0x6546600, "native edit replay", "count")
    Sample(0x60b6c00, "prepared operation replay", "count")
end
continue
