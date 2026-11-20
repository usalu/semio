set pagination off
set debuginfod enabled off
set environment RUST_MIN_STACK 67108864
starti
python
import gdb
pid=gdb.selected_inferior().pid
mapping=next(row for row in open('/proc/%d/maps'%pid) if 'native-loan-diagnostic' in row and row.split()[2]=='00000000')
gdb.execute('set $base = %#x'%int(mapping.split('-')[0],16))
end
break *($base+0x5d35e27)
commands
silent
printf "[BRANCH] value=%u return=0x%lx\n",$al,*(unsigned long*)($rsp+0x18)-$base
continue
end
break *($base+0xda7d61d)
continue
bt 3
