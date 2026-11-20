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
break *($base+0x546b2d1)
continue
disable 1
break *($base+0x5c01a01) if ($al & 1) != 0
break *($base+0x5c01a10) if ($al & 1) == 0
break *($base+0x5c01a2d) if ($al & 1) == 0
break *($base+0x5c01a43) if ($al & 1) == 0
continue
printf "[DEBUG] first nonempty typing witness at +0x%lx\n", $pc-$base
bt 5
