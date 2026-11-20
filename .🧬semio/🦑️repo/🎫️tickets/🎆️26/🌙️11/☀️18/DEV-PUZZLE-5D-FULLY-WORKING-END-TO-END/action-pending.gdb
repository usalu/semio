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
break *($base+0x00000000063496e0)
continue
python
for _ in range(2500):
    ins=gdb.selected_frame().architecture().disassemble(int(gdb.parse_and_eval('$pc')),count=1)[0]['asm']
    if ins.startswith('call') or ins.startswith('ret'):
        print('ROUTE:',hex(int(gdb.parse_and_eval('$pc-$base'))),ins)
    if ins.startswith('ret'):break
    gdb.execute('nexti',to_string=True)
end
