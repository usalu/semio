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
set $receives = 0
break *($base+0x56345dc)
commands
silent
set $receives = $receives+1
continue
end
break *($base+0x563448b)
ignore 2 100
continue
p $receives
p *(unsigned long *)($rsp+0xcc8)
info breakpoints
bt 9
