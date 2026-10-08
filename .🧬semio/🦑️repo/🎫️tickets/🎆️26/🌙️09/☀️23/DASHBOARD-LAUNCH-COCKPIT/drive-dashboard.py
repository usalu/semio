#!/usr/bin/env python3
"""Drives the native dashboard through a pseudo-terminal: opens the launcher, types a search, optionally runs the first hit."""
import os, pty, select, sys, time, fcntl, termios, struct, re
binary, query, out = sys.argv[1], sys.argv[2], sys.argv[3]
run = len(sys.argv) > 4 and sys.argv[4] == "run"
pid, fd = pty.fork()
if pid == 0:
    os.execvpe(binary, [binary], dict(os.environ, TERM="xterm-256color"))
fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", 45, 200, 0, 0))
buffer = b""
def pump(seconds):
    global buffer
    end = time.time() + seconds
    while time.time() < end:
        ready, _, _ = select.select([fd], [], [], 0.1)
        if ready:
            try: buffer += os.read(fd, 65536)
            except OSError: return
pump(2.0); os.write(fd, b"\r"); pump(1.0)
for character in query:
    os.write(fd, character.encode()); pump(0.05)
pump(1.5)
mark = len(buffer)
if run:
    os.write(fd, b"\r"); pump(float(sys.argv[5]) if len(sys.argv) > 5 else 8.0)
open(out, "wb").write(buffer)
open(out + ".after", "wb").write(buffer[mark:])
os.write(fd, b"\x02"); pump(0.2); os.write(fd, b"Q" if run else b"d"); pump(1.5)
try: os.kill(pid, 15)
except ProcessLookupError: pass
