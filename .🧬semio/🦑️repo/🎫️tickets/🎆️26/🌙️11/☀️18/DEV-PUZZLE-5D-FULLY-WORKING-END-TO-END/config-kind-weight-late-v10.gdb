set confirm off
set pagination off
set print frame-arguments none
handle SIGTRAP stop print nopass
start
python
import os, signal, threading
pid = gdb.selected_inferior().pid
timer = threading.Timer(15, lambda: os.kill(pid, signal.SIGTRAP))
timer.daemon = True
timer.start()
end
continue
thread apply all bt 36
kill
