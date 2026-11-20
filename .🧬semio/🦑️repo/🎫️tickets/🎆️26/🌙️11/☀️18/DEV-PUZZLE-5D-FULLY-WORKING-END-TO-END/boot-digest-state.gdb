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
set $seal=0
set $seal_fn=0
set $calls=0
break *($base+0x7c34fe8)
commands
silent
set $seal=$rsi
set $seal_fn=*(unsigned long *)($base+0xdf60378)-$base
set $calls=$calls+1
continue
end
break *($base+0xda276cb)
continue
p/x $seal_fn
p $calls
p/x $seal
x/15gx $seal
x/30gx $seal+0x1700
