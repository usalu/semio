#!/usr/bin/env python3
"""Drives a command through an owned pty (the dashboard binary or the `repo dashboard` launcher): usage drive-task.py <binary> <out-prefix> <step>... ; DRIVE_ARGV (JSON list) gives the argument vector; waitfor:SECONDS:TEXT polls the screen text (stops early when it appears); step = wait:S | key:HEX | text:STR | snap:NAME. Ends the child with Ctrl+B q (quit) then SIGTERM."""
import os, pty, select, sys, time, fcntl, termios, struct, re, signal
binary, out, steps = sys.argv[1], sys.argv[2], sys.argv[3:]
pid, fd = pty.fork()
if pid == 0:
    import json
    os.execvpe(binary, [binary, *json.loads(os.environ.get("DRIVE_ARGV","[]"))], dict(os.environ, TERM="xterm-256color"))
fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", 45, 160, 0, 0))
buffer = b""
def pump(seconds):
    global buffer
    end = time.time() + seconds
    while time.time() < end:
        ready, _, _ = select.select([fd], [], [], 0.1)
        if ready:
            try: buffer += os.read(fd, 65536)
            except OSError: return
def clean(data):
    text = re.sub(r"\x1b\[[0-9;?]*[A-Za-z]|\x1b\][^\x07]*\x07|\x1b[()][0-9A-B]", "\n", data.decode("utf8", "replace"))
    return "\n".join(l.strip(" ┃") for l in text.split("\n") if l.strip(" ┃"))
mark = 0
for step in steps:
    kind, _, value = step.partition(":")
    if kind == "wait": pump(float(value))
    elif kind == "waitfor":
        seconds, _, needle = value.partition(":")
        end = time.time() + float(seconds)
        while time.time() < end and needle not in clean(buffer[mark:]): pump(1.0)
    elif kind == "key": os.write(fd, bytes.fromhex(value)); pump(0.3)
    elif kind == "text":
        for ch in value: os.write(fd, ch.encode()); pump(0.04)
    elif kind == "snap":
        open(f"{out}.{value}.txt", "w").write(clean(buffer[mark:])); mark = len(buffer)
open(out + ".raw", "wb").write(buffer)
try: os.killpg(pid, signal.SIGTERM)
except ProcessLookupError: pass
