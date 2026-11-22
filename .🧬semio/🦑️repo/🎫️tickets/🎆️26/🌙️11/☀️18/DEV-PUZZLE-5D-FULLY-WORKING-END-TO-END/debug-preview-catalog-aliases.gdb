set pagination off
set auto-load off
set debuginfod enabled off
set env RUST_MIN_STACK 67108864
starti
python
pid = gdb.selected_inferior().pid
entries = [line.split(maxsplit=5) for line in open("/proc/%d/maps" % pid)]
base = next(int(row[0].split("-")[0],16) for row in entries if len(row) == 6 and row[2] == "00000000" and "semio_s_artifact_puzzle_5d_v1-" in row[5])
inferior = gdb.selected_inferior()
def read64(addr):
    return int.from_bytes(inferior.read_memory(addr, 8), "little")
class Trace(gdb.Breakpoint):
    def __init__(self, label, virtual, reg=None):
        gdb.Breakpoint.__init__(self, "*0x%x" % (base + virtual), internal=True)
        self.label, self.reg, self.count = label, reg, 0
    def stop(self):
        self.count += 1
        if self.label == "unique" and self.count not in (1, 2, 4, 16, 128):
            return False
        if self.label == "owner" and self.count not in (1, 2, 4, 16, 128):
            return False
        ret = read64(int(gdb.parse_and_eval("$rsp")))
        symbol = gdb.execute("info symbol 0x%x" % ret, to_string=True).strip()
        print("[DEBUG]", self.label, self.count, symbol)
        if self.reg:
            try:
                slot = int(gdb.parse_and_eval(self.reg))
                print("[DEBUG] slot", hex(slot), [hex(read64(slot+i)) for i in (0,8,16)])
                if self.label != "owner":
                    data = read64(slot)
                    print("[DEBUG] allocation", hex(data), [hex(read64(data+i)) for i in range(0,176,8)])
            except gdb.MemoryError as error:
                print("[DEBUG]", error)
        return self.label == "owner" and self.count == 128
Trace("issue", 0x9c9a9f0, "$rdi")
Trace("retire", 0x9c921f0, "$rsi")
Trace("unique", 0x9c9a5f0, "$rdi")
Trace("owner", 0x9cb5670, "$rsi")
end
continue
