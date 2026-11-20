set pagination off
set debuginfod enabled off
set environment RUST_MIN_STACK 67108864
starti
python
import gdb
pid=gdb.selected_inferior().pid
mapping=next(row for row in open('/proc/%d/maps'%pid) if 'voxel-diagnostic-symbols' in row and row.split()[2]=='00000000')
gdb.execute('set $base = %#x'%int(mapping.split('-')[0],16))
end
break *($base+0x4d1bda0)
ignore 1 200
continue
disable 1
break *($base+0x77744c0)
break *($base+0x7786260)
continue
bt 4
continue
bt 4
