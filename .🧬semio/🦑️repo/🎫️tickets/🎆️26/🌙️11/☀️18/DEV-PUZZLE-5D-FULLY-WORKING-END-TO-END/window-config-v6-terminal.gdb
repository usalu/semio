set confirm off
set pagination off
set print frame-arguments none
handle SIGTRAP stop print nopass
tbreak '<semio_plugin_window_config::retained::command::WindowConfigCommandLoad>::advance'
ignore 1 32000
run
python
import os, signal, threading
inferior_pid = gdb.selected_inferior().pid
def sample_after(seconds):
    timer = threading.Timer(seconds, lambda: os.kill(inferior_pid, signal.SIGTRAP))
    timer.daemon = True
    timer.start()
sample_after(0.005)
end
continue
printf "[DEBUG] Window command active stack\n"
thread apply all bt 35
rbreak WindowConfigCommandLoad as.*Drop.*drop
continue
printf "[DEBUG] Window command terminal caller\n"
bt 9
frame 2
x/36i $pc-112
info registers rax rdi rsi rdx
kill
