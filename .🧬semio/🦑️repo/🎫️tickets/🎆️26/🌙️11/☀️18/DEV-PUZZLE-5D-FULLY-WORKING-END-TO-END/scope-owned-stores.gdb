set pagination off
set debuginfod enabled off
starti
python
import gdb
pid = gdb.selected_inferior().pid
maps = open("/proc/%d/maps" % pid).read().splitlines()
mapping = next(row for row in maps if "semio_s_artifact_puzzle_5d_v1-f62a359f9cc1d4ad" in row and row.split()[2] == "00000000")
gdb.execute("set $base = 0x%x" % int(mapping.split("-")[0], 16))
end
break *($base+0x50d9070)
ignore 1 2048
continue
set $close = *(unsigned long *)($rsi+0x68)
printf "[DEBUG] close owned-store stage %u, advance-ready %u\n", *(unsigned char *)($close+0x5e5), *(unsigned char *)($close+0x5e3)
set $demand = $rdi
finish
x/6gd $demand
bt 9
