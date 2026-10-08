#!/usr/bin/env python3
"""🔬️ Runs the w0 backend-contract probe binary on a pseudo-terminal and prints its log plus the bytes it wrote.

Ticket input of 26/09/23/DASHBOARD-LAUNCH-COCKPIT. Usage (repository root):
  python3 …/w0-tui-contract-probe.py <probe-binary> <log-path>
"""
import fcntl
import os
import pty
import select
import struct
import sys
import termios
import time

binary, log_path = sys.argv[1], sys.argv[2]
if os.path.exists(log_path):
    os.remove(log_path)
pid, master = pty.fork()
if pid == 0:
    os.execv(binary, [binary, log_path])
fcntl.ioctl(master, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 80, 0, 0))
written = b""


def pump(seconds):
    global written
    end = time.time() + seconds
    while time.time() < end:
        ready, _, _ = select.select([master], [], [], 0.02)
        if ready:
            try:
                written += os.read(master, 65536)
            except OSError:
                return


def log_text():
    return open(log_path, encoding="utf-8").read() if os.path.exists(log_path) else ""


deadline = time.time() + 15
while "ready" not in log_text().splitlines() and time.time() < deadline:
    pump(0.05)
steps = [(b"a", 0.2), (b"\x1b[A", 0.2), (b"\x1b[<0;10;5M", 0.2), (b"\x1b[<2;10;5M", 0.2), (b"\x1b[<64;5;5M", 0.2), (b"\x1b[<65;5;5M", 0.2), (b"\x1b[200~pasted\x1b[201~", 0.2), (b"\x1b", 0.4), (b"q", 0.3)]
for data, pause in steps:
    os.write(master, data)
    pump(pause)
pump(0.5)
_, status = os.waitpid(pid, 0)
print(log_text(), end="")
print(f"exit_status={os.waitstatus_to_exitcode(status)}")
print(f"terminal_bytes={written!r}")
