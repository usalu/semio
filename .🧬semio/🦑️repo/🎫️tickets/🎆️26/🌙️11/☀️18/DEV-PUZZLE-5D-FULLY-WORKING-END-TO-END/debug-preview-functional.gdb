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
 "handoff": 0x3db3050, "finish": 0x3dbf130, "job": 0x3dc03b0,
 "source_close": 0x5a9e2a0, "source_demand": 0x5a9f000,
 "begin_residual": 0x8b1d690, "demand_residual": 0x8b23020,
 "grant": 0xcc982e0, "fuel": 0xcc982a0
}
hits = dict.fromkeys(points, 0)
in_close = False
class Returned(gdb.Breakpoint):
    def __init__(self,label,address,result):
        gdb.Breakpoint.__init__(self,"*0x%x" % address,internal=True,temporary=True)
        self.label,self.result = label,result
    def stop(self):
        global in_close
        if self.label == "source_close":
            in_close = False
        elif self.label == "fuel":
            print("[DEBUG] fuel_return", int(gdb.parse_and_eval("$rax")))
        else:
            count = 5 if self.label == "grant" else 8
            print("[DEBUG] return",self.label, [read64(self.result + i*8) for i in range(count)])
        return False
class Point(gdb.Breakpoint):
    def __init__(self,label,virtual):
        gdb.Breakpoint.__init__(self,"*0x%x" % (base+virtual),internal=True)
        self.label=label
    def stop(self):
        global in_close
        if self.label != "handoff" and not hits["handoff"]:
            return False
        if self.label in ("grant","fuel") and not in_close:
            return False
        hits[self.label] += 1
        n=hits[self.label]
        if self.label == "finish" and n == 128:
            print("[DEBUG] cutoff",hits)
            return True
        if self.label in ("source_close","source_demand","begin_residual","demand_residual","grant","fuel") and n <= 12:
            print("[DEBUG] call",self.label,n)
            if self.label == "source_close":
                in_close=True
            if self.label in ("source_close","source_demand","begin_residual","demand_residual","grant","fuel"):
                Returned(self.label,read64(int(gdb.parse_and_eval("$rsp"))),int(gdb.parse_and_eval("$rdi")))
        return False
for label,virtual in points.items():
    Point(label,virtual)
end
continue
