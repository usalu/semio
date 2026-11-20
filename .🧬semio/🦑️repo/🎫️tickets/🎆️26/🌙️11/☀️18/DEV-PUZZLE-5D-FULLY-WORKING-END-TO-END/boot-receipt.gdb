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
break *($base+0x62cd683) if *(unsigned long *)($rsp+0x68328)==55807
commands
silent
set $ctx = *(unsigned long *)(*(unsigned long *)($rsp+0x2f0)+0xe8)
printf "[DEBUG] reserved before "
x/4gd $ctx+0x1d0
continue
end
break *($base+0xa80699c)
continue
bt 30
python
frame = gdb.newest_frame()
for depth in range(29):
    if frame is None:
        break
    print(depth, hex(frame.pc() - base), hex(int(frame.read_register("rsp"))))
    frame = frame.older()
end
info registers rdi rsi rdx rcx r8 rsp
x/30gx $rsp
x/24i $pc-32
frame 3
x/4gd *(unsigned long *)($rsp+0x28)
x/4gd *(unsigned long *)($rsp+0x20)
x/4gd *(unsigned long *)($rsp+0x18)
frame 4
x/30i $pc-60
set $ctx = *(unsigned long *)(*(unsigned long *)($rsp+0x2f0)+0xe8)
printf "[DEBUG] reserved after "
x/4gd $ctx+0x1d0
