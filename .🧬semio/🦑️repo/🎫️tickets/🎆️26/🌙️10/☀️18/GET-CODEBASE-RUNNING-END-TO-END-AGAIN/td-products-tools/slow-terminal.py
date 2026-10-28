#!/usr/bin/env python3
"""🐌 Slow-terminal oracle for the dashboard journeys: runs `<program> <args>` and, when stdin is a terminal, shows every chunk of its output
`SEMIO_SLOW_DELAY_MS` late and at least `SEMIO_SLOW_SPACING_MS` apart from the previous chunk (order kept), so every intermediate frame is
on the screen long enough for a 40 ms poll to see it. This is what a loaded host does to a pseudo-terminal journey, made deterministic.
Without a terminal on stdin the program is executed unchanged. Use as SEMIO_TEST_CLI: `slow-terminal.py` needs SEMIO_SLOW_TARGET (the real binary)."""
import fcntl, os, pty, select, signal, struct, sys, termios, time, tty

target = os.environ["SEMIO_SLOW_TARGET"]
delay = float(os.environ.get("SEMIO_SLOW_DELAY_MS", "100")) / 1000.0
spacing = float(os.environ.get("SEMIO_SLOW_SPACING_MS", "100")) / 1000.0
argv = [target, *sys.argv[1:]]
if not os.isatty(0):
    os.execv(target, argv)

tty.setraw(0)
pid, master = pty.fork()
if pid == 0:
    os.execv(target, argv)

def copy_size(*_):
    try:
        size = fcntl.ioctl(0, termios.TIOCGWINSZ, struct.pack("HHHH", 0, 0, 0, 0))
        fcntl.ioctl(master, termios.TIOCSWINSZ, size)
        os.kill(pid, signal.SIGWINCH)
    except OSError:
        pass

signal.signal(signal.SIGWINCH, copy_size)
copy_size()
queue = []
last = 0.0
stdin_open = True
status = None
while True:
    now = time.monotonic()
    wait = None
    if queue:
        wait = max(0.0, queue[0][0] - now)
    fds = [master] + ([0] if stdin_open else [])
    try:
        ready, _, _ = select.select(fds, [], [], wait)
    except InterruptedError:
        continue
    except OSError:
        ready = []
    if 0 in ready:
        data = os.read(0, 65536)
        if data:
            os.write(master, data)
        else:
            stdin_open = False
    if master in ready:
        try:
            data = os.read(master, 65536)
        except OSError:
            data = b""
        if not data:
            break
        arrival = time.monotonic()
        if queue and arrival - queue[-1][2] < 0.005:
            queue[-1] = (queue[-1][0], queue[-1][1] + data, arrival)
            continue
        release = max(arrival + delay, last + spacing)
        if release - arrival > 0.6:
            release = arrival + delay
        queue.append((release, data, arrival))
        last = release
    now = time.monotonic()
    while queue and queue[0][0] <= now:
        os.write(1, queue.pop(0)[1])
for release, data, _ in queue:
    time.sleep(max(0.0, release - time.monotonic()))
    os.write(1, data)
_, code = os.waitpid(pid, 0)
sys.exit(os.waitstatus_to_exitcode(code))
