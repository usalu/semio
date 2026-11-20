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
break *($base+0x60c81aa)
condition 1 *(unsigned long *)(*(unsigned long *)($rsp+0x90)+8)==0 && *(unsigned long *)(*(unsigned long *)($rsp+0x90)+16)==0 && *(unsigned long *)(*(unsigned long *)($rsp+0x90)+24)==0 && *(unsigned long *)(*(unsigned long *)($rsp+0x90)+32)==0
continue
set $close = *(unsigned long *)($rsp+0xed8+0x68)
printf "[DEBUG] first zero OwnedStores quote at stage %u, advance-ready %u\n", *(unsigned char *)($close+0x5e5), *(unsigned char *)($close+0x5e3)
x/5gd *(unsigned long *)($rsp+0x90)
disable 1
break *($base+0xa6803e6) if ($al & 1) == 0
break *($base+0xa6803fc) if ($al & 1) == 0
break *($base+0xa680414) if ($al & 1) == 0
break *($base+0xa68042c) if ($al & 1) == 0
break *($base+0xa680444) if ($al & 1) == 0
continue
printf "[DEBUG] nonempty Window registry witness at +0x%lx\n", $pc-$base
bt 8
