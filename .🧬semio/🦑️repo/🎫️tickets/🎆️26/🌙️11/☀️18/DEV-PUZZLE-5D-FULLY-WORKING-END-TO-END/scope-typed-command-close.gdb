set pagination off
set debuginfod enabled off
starti
python
import gdb
pid = gdb.selected_inferior().pid
maps = open("/proc/%d/maps" % pid).read().splitlines()
mapping = next(row for row in maps if "semio_s_artifact_puzzle_5d_v1-f62a359f9cc1d4ad" in row and row.split()[2] == "00000000")
base = int(mapping.split("-")[0], 16)
gdb.execute("set $base = 0x%x" % base)
end
break *($base+0x4e0edbb)
continue
set $command = *(unsigned long *)($rsp+0x58)
disable 1
break *($base+0x4e12c0c) if ($al & 1) == 0
break *($base+0x4e0ee1f)
continue
printf "[DEBUG] typed command terminal stage +0x%lx\n", $pc-$base
printf "[DEBUG] body discriminant %lu, binding first word %lu\n", *(unsigned long *)($command+0x2220), *(unsigned long *)($command+0x25b8)
bt 6
