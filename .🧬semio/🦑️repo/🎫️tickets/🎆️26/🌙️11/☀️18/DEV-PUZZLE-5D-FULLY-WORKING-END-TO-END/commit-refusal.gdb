set pagination off
set debuginfod enabled off
set environment RUST_MIN_STACK 67108864
starti
python
import gdb
pid=gdb.selected_inferior().pid
mapping=next(row for row in open('/proc/%d/maps'%pid) if 'commit-diagnostic' in row and row.split()[2]=='00000000')
gdb.execute('set $base = %#x'%int(mapping.split('-')[0],16))
end
break *($base+0xda2e47e)
continue
p *(unsigned long*)($rsp+0x50)
p *(unsigned long*)($rsp+0x58)
x/24gx $rsp+0x40
bt 5
