set pagination off
set debuginfod enabled off
starti
python
import gdb
pid = gdb.selected_inferior().pid
maps = open("/proc/%d/maps" % pid).read().splitlines()
mapping = next(row for row in maps if "semio_s_artifact_puzzle_5d_v1-e4a6430e2d8e83ba" in row and row.split()[2] == "00000000")
base = int(mapping.split("-")[0], 16)
gdb.execute("set $base = 0x%x" % base)
end
break *($base+0xda276cb)
continue
bt 25
python
frame = gdb.newest_frame()
for depth in range(25):
    if frame is None:
        break
    print(depth, hex(frame.pc() - base), hex(int(frame.read_register("rsp"))))
    frame = frame.older()
end
x/4gd $rsp+0xa0
x/5gd $rsp+0xc0
