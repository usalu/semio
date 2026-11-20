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
python
import threading,signal,os
timer=threading.Timer(3.5,lambda:os.kill(pid,signal.SIGINT))
timer.start()
gdb.execute('continue')
timer.cancel()
end
break *($base+0x536ad20)
continue
info registers rdi rsi rdx rcx r8
x/4gx $rdx
set $output=$rdi
finish
x/8gx $output
bt 6
