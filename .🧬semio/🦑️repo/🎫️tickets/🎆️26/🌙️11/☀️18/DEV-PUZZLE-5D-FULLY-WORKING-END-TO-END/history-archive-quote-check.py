import gdb
import struct

base = int(gdb.parse_and_eval("(uintptr_t)&main")) - 0x4448390
inf = gdb.selected_inferior()
def reg(name):
    return int(gdb.parse_and_eval("$" + name))

class CopyQuote(gdb.FinishBreakpoint):
    hits = 0
    def __init__(self):
        self.output = reg("rdi")
        super().__init__(gdb.newest_frame(), internal=True)
    def stop(self):
        tag, copy, capacity, release, depth = struct.unpack("<QQQQQ", inf.read_memory(self.output,40))
        if copy == 0 and capacity == 1872 and depth == 2:
            CopyQuote.hits += 1
            if CopyQuote.hits <= 3:
                print("[DEBUG] corrected metadata quote on v6", tag, copy, capacity, release, depth)
            inf.write_memory(self.output + 8, struct.pack("<Q",2976))
        return False

class Quote(gdb.Breakpoint):
    def stop(self):
        CopyQuote()
        return False

class Refusal(gdb.Breakpoint):
    hits = 0
    def stop(self):
        Refusal.hits += 1
        seen = inf.read_memory(reg("rsi"), min(reg("rdx"), 1024)).tobytes()
        print("[DEBUG] initializer refusal", seen)
        if Refusal.hits <= 3:
            gdb.execute("bt 14")
        return False

Quote("*" + hex(base + 0xd4e1e90), internal=True)
Refusal("*" + hex(base + 0xb8355c0), internal=True)
