#!/usr/bin/env python3
"""🎰️ usage: outer.py <fifo|pipe|pipe-high> <tokens> -- <command...>
An independent GNU make style parent: it creates a job-token pool of <tokens> slots (the implicit slot plus tokens-1 bytes), names it in MAKEFLAGS exactly like make does
(`-jN --jobserver-auth=fifo:PATH` or `--jobserver-auth=R,W`) and runs the command with the descriptors inherited under their own numbers (pipe-high shifts them away from 3,5)."""
import os, subprocess, sys, tempfile
kind, tokens = sys.argv[1], int(sys.argv[2]); command = sys.argv[sys.argv.index("--") + 1:]
env = dict(os.environ); pass_fds = ()
if kind == "fifo":
    directory = tempfile.mkdtemp(prefix="jg-"); path = directory + "/pool"
    os.mkfifo(path, 0o600); keep = os.open(path, os.O_RDWR | os.O_NONBLOCK); os.write(keep, b"+" * (tokens - 1))
    env["MAKEFLAGS"] = "-j%d --jobserver-auth=fifo:%s" % (tokens, path)
else:
    if kind == "pipe-high":
        for _ in range(4): os.open("/dev/null", os.O_RDONLY)
    r, w = os.pipe(); os.write(w, b"+" * (tokens - 1)); os.set_inheritable(r, True); os.set_inheritable(w, True)
    env["MAKEFLAGS"] = "-j%d --jobserver-auth=%d,%d" % (tokens, r, w); pass_fds = (r, w)
env["CARGO_BUILD_JOBS"] = str(tokens)
sys.exit(subprocess.run(command, env=env, pass_fds=pass_fds).returncode)
