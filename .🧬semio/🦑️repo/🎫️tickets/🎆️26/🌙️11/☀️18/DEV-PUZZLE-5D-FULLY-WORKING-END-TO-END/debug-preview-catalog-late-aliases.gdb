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
points = {
 "issue": (0x9c9a9f0,"$rdi"), "retire": (0x9c921f0,"$rsi"),
 "unique": (0x9c9a5f0,"$rdi"), "owner": (0x9cb5670,"$rsi"),
 "panic": (0xd5a2230,None)
}
hits = dict.fromkeys(points, 0)
seen = set()
class Trace(gdb.Breakpoint):
    def __init__(self, label, virtual, reg):
        gdb.Breakpoint.__init__(self, "*0x%x" % (base + virtual), internal=True)
        self.label, self.reg = label, reg
    def stop(self):
        hits[self.label] += 1
        n = hits[self.label]
        if self.label == "panic":
            print("[DEBUG]", self.label, hits)
            return True
        slot = int(gdb.parse_and_eval(self.reg))
        address = read64(slot + (8 if self.label == "owner" else 0))
        if not address:
            return False
        strong = read64(address)
        tag = self.label, address, strong
        if tag not in seen or (self.label in ("issue","retire")):
            seen.add(tag)
            print("[DEBUG]", self.label, n, "allocation", hex(address), "strong", strong, hits)
        return self.label == "owner" and n == 8000
for label, (virtual, reg) in points.items():
    Trace(label, virtual, reg)
end
continue
