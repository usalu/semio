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
 "owner": 0x9cb5670, "panic": 0xd5a2230, "shared_cursor": 0x9c9ad20,
 "inner_ctrl": 0x9cba120, "catalog_seed": 0x9cb7ee0,
 "registered_bus": 0xa5392b0
}
hits = dict.fromkeys(points, 0)
class Trace(gdb.Breakpoint):
    def __init__(self, label, virtual):
        gdb.Breakpoint.__init__(self, "*0x%x" % (base + virtual), internal=True)
        self.label = label
    def stop(self):
        if hits["owner"] == 0 and self.label != "owner":
            return False
        hits[self.label] += 1
        n = hits[self.label]
        if self.label == "panic" or (self.label == "owner" and n in (1, 500, 1024, 2048, 8192, 16384, 24576, 30000, 32700)) or self.label == "catalog_seed":
            print("[DEBUG]", self.label, n, hits)
        return self.label == "panic"
for label, virtual in points.items():
    Trace(label, virtual)
end
continue
