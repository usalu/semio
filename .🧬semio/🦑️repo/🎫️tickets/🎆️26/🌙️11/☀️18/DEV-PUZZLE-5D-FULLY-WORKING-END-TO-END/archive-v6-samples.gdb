set confirm off
set pagination off
set print frame-arguments none
handle SIGTRAP stop print nopass
start
python
import os, signal, threading
inferior_pid = gdb.selected_inferior().pid
def sample_after(seconds):
    timer = threading.Timer(seconds, lambda: os.kill(inferior_pid, signal.SIGTRAP))
    timer.daemon = True
    timer.start()
sample_after(20)
end
continue
printf "[DEBUG] first archive sample\n"
thread apply all bt 18
python
sample_after(20)
end
continue
printf "[DEBUG] second archive sample\n"
thread apply all bt 18
kill
