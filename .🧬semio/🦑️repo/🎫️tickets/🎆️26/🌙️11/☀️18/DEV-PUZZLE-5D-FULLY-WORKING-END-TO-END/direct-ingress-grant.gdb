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
break *($base+0x4bbd454) if *(unsigned long *)($rsp+0x86e0)==32 && *(unsigned long *)($rsp+0x86f0)==48 && *(unsigned long *)($rsp+0x86f8)==1
ignore 1 4
continue
printf "[DEBUG] DirectIngress quote, grant and affordability\n"
x/4gd $rsp+0x86e0
x/5gd $rsp+0xa0
p $al
bt 8
disable 1
break *($base+0xa702b90)
continue
set $registry_result = $rdi
printf "[DEBUG] DirectIngress registry producer result\n"
finish
x/10gd $registry_result
