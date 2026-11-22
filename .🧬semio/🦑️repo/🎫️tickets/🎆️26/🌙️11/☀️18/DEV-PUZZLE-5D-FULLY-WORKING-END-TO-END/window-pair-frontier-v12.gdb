set confirm off
set pagination off
set print frame-arguments none
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
binary = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute("info proc mappings", to_string=True).splitlines() if line.strip().endswith(binary))
inferior = gdb.selected_inferior()
def number(address, size=8):
    return int.from_bytes(inferior.read_memory(address, size), "little")
def reg(name):
    return int(gdb.parse_and_eval("$" + name))
class PrintQuote(gdb.FinishBreakpoint):
    def __init__(self, output, hits):
        super().__init__(gdb.newest_frame(), internal=True)
        self.output, self.hits = output, hits
    def stop(self):
        print("[DEBUG] Native Window pair quote output", self.hits, tuple(number(self.output + 8*i) for i in range(8)))
        return False
class Pair(gdb.Breakpoint):
    def __init__(self, addr, phase, label):
        super().__init__("*" + hex(base + addr), internal=True)
        self.phase, self.count, self.label = phase, 0, label
    def stop(self):
        owner, output = reg("rsi"), reg("rdi")
        phase = number(owner + 0xb6b3, 1)
        if phase != self.phase:
            self.phase, self.count = phase, 0
        self.count += 1
        if self.count < 3 or self.count in {50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 30000}:
            print("[DEBUG] Native Window pair action", self.label, phase, self.count)
            if self.label == "quote":
                PrintQuote(output, self.count)
            else:
                effect = number(reg("rsp") + 24)
                print("[DEBUG] Native Window pair grant", tuple(number(effect + 48 + i*8) for i in range(6)))
        return self.label == "quote" and self.count == 30000
Pair(0x8321580, -1, "step")
Pair(0x83216a0, -1, "quote")
end
continue
bt 26
kill
