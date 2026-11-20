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
break *($base+0x62cd7ce) if *(unsigned long *)($rsp+0x68328)==55807
continue
printf "[DEBUG] publish: %lx\n", $rax-$base
set $pub = $rax
set $mounted = $rdx
set $ctx = $r8
disable 1
watch *(unsigned long *)($mounted+0x26e50)
continue
bt 12
python
frame = gdb.newest_frame()
for depth in range(12):
    if frame is None:
        break
    print(depth, hex(frame.pc() - base), hex(int(frame.read_register("rsp"))))
    frame = frame.older()
end
x/8gx $mounted+0x26e50
