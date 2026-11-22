set pagination off
set auto-load off
set debuginfod enabled off
set env RUST_MIN_STACK 67108864
starti
python
pid = gdb.selected_inferior().pid
binary = gdb.current_progspace().filename
entries = [line.split(maxsplit=5) for line in open("/proc/%d/maps" % pid)]
base = next(int(row[0].split("-")[0],16) for row in entries if len(row) == 6 and row[2] == "00000000" and "semio_s_artifact_puzzle_5d_v1-" in row[5])
print("[DEBUG] base", hex(base))
points = {'TimeTravelExtension, ()))>>:retained_fields_close_step': 86869088, 'TimeTravelExtension, ()))>>:retained_fields_close_demand': 87238752, 'ToolActionBus>:original_close_step': 159112704, 'PagedCommandLog>:close_step': 159280096, 'OriginalDeclaredToolCatalog>:close_step': 164320880, 'DeclaredToolCatalogAlias>:close_step': 164322000, 'AppActionRegistry>:close_step': 165378112, 'CompositionGraph>:close_step': 193078416, 'panicking:panic_fmt': 224010800}
hits = dict.fromkeys(points, 0)
class Count(gdb.Breakpoint):
    def __init__(self, label, address):
        gdb.Breakpoint.__init__(self, "*0x%x" % address, internal=True)
        self.label = label
    def stop(self):
        hits[self.label] += 1
        n = hits[self.label]
        if n in (1, 256, 2048) or self.label.endswith(":panic_fmt"):
            print("[DEBUG]", self.label, n, hits)
        return self.label.endswith(":panic_fmt") or (n == 2048 and "retained_fields_close" not in self.label)
for label, virtual in points.items():
    Count(label, base + virtual)
end
continue
bt 7
x/8i $pc
