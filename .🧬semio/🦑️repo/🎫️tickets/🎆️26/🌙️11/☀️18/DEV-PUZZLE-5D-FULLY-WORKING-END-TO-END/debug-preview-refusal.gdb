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
points = {"begin": 0x3dbf0f0, "handoff": 0x3db3050, "finish": 0x3dbf130, "refusal": 0x3d95fa0, "panic": 0xd5a2230}
hits = dict.fromkeys(points, 0)
class Report(gdb.Breakpoint):
    def __init__(self, address, result):
        gdb.Breakpoint.__init__(self, "*0x%x" % address, internal=True, temporary=True)
        self.result = result
    def stop(self):
        decoded = set()
        for index in range(0, 192, 8):
            pointer = read64(self.result + index)
            length = read64(self.result + index + 8)
            if not 3 <= length <= 1024 or pointer <= 0x10000:
                continue
            try:
                value = bytes(inferior.read_memory(pointer,length)).decode("utf-8")
                if value.isprintable():
                    decoded.add(value)
            except (gdb.MemoryError,UnicodeDecodeError):
                pass
        print("[DEBUG] preview-or-shell refusal", hits, sorted(decoded))
        return hits["begin"] > 0
class Trace(gdb.Breakpoint):
    def __init__(self, label, virtual):
        gdb.Breakpoint.__init__(self, "*0x%x" % (base + virtual), internal=True)
        self.label = label
    def stop(self):
        hits[self.label] += 1
        if self.label == "refusal":
            Report(read64(int(gdb.parse_and_eval("$rsp"))), int(gdb.parse_and_eval("$rdi")))
        if self.label in ("begin","handoff","panic"):
            print("[DEBUG]", self.label, hits)
        return self.label == "panic"
for label, virtual in points.items():
    Trace(label, virtual)
end
continue
