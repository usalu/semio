set pagination off
set debuginfod enabled off
set environment RUST_MIN_STACK 67108864
starti
python
import gdb
pid=gdb.selected_inferior().pid
mapping=next(row for row in open('/proc/%d/maps'%pid) if 'maintenance-diagnostic' in row and row.split()[2]=='00000000')
gdb.execute('set $base = %#x'%int(mapping.split('-')[0],16))
end
break *($base+0x3f84010)
continue
break *($base+0x59eb5a8)
continue
p/a *(void**)($rax+0x58)
bt 7
