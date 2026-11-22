set pagination off
set debuginfod enabled off
set auto-solib-add off
set environment RUST_MIN_STACK 67108864
starti
python
import gdb

inferior = gdb.selected_inferior()
mapping = next(row for row in open("/proc/%d/maps" % inferior.pid) if "semio_s_artifact_puzzle_5d_v1-f62a359f9cc1d4ad" in row and row.split()[2] == "00000000")
base = int(mapping.split("-")[0], 16)
gdb.execute("set $base = %d" % base)
end
break *($base+0x6059a30) if *(unsigned long *)($rsp+0x200)>30000
continue
printf "[DEBUG] late close loop iteration and quote\n"
x/gd $rsp+0x200
x/4gd $rcx
set $context=$rsi
break *($base+0x4bbd454)
continue
printf "[DEBUG] affordability late in close\n"
x/5gd $rsp+0xa0
p $al
bt 4
