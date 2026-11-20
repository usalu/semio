set pagination off
set debuginfod enabled off
set environment RUST_MIN_STACK 67108864
starti
python
import gdb
pid=gdb.selected_inferior().pid
mapping=next(row for row in open('/proc/%d/maps'%pid) if 'semio_s_artifact_puzzle_5d_v1-f62a359f9cc1d4ad' in row and row.split()[2]=='00000000')
gdb.execute('set $base = %#x'%int(mapping.split('-')[0],16))
end
set $lastouter = 0
set $count = 0
set $inner = 0
set $producer = 0
break *($base+0x4bca1d0)
commands
silent
set $lastouter = 0
continue
end
break *($base+0x5433740)
commands
silent
set $lastouter = 1
set $count = $count + 1
set $inner = 0
set $producer = 0
continue
end
break *($base+0x5464c10)
commands
silent
set $producer = 1
continue
end
break *($base+0x62db110)
commands
silent
set $inner = 1
continue
end
break *($base+0xa8923d0)
commands
silent
printf "last drive count %d entered producer = %d inner = %d outer worker %d\n", $count, $producer, $inner, $lastouter
bt 5
quit
end
continue
