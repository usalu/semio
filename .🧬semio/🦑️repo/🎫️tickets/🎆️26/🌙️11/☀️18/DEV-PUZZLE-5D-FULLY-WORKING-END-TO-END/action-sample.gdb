set pagination off
set debuginfod enabled off
set environment RUST_MIN_STACK 67108864
starti
python
import gdb,threading,os,signal
pid=gdb.selected_inferior().pid
for delay in (0.8,0.25,0.25,0.25,0.25,0.25,0.25,0.25):
    timer=threading.Timer(delay,lambda:os.kill(pid,signal.SIGINT))
    timer.start()
    try:
        gdb.execute('continue')
        gdb.execute('thread apply all bt 18')
    except gdb.error:
        break
    finally:
        timer.cancel()
end
