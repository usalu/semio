set confirm off
set pagination off
set print frame-arguments none
handle SIGTRAP stop print nopass
start
python
import os, signal, threading
pid = gdb.selected_inferior().pid
timer = threading.Timer(float(os.environ.get("SEMIO_5D_SAMPLE_SECONDS", "8")), lambda: os.kill(pid, signal.SIGTRAP))
timer.daemon = True
timer.start()
end
continue
thread apply all bt 30
kill
