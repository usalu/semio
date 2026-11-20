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
python
import re
arch=gdb.selected_frame().architecture()
for instruction in arch.disassemble(base+0x5482740,base+0x54830f4):
    asm=instruction['asm']
    m=re.match(r'\s*(j(?:mp|e|ne))\s+0x([0-9a-f]+)',asm)
    if not m or int(m[2],16)!=base+0x548275c:
        continue
    bp=gdb.Breakpoint("*0x%x" % instruction["addr"],internal=True)
    if m[1] == "je":
        bp.condition="($eflags & 64) != 0"
    elif m[1] == "jne":
        bp.condition="($eflags & 64) == 0"
end
continue
printf "[DEBUG] first close predicate false at +0x%lx\n", $pc-$base
x/i $pc
bt 8
