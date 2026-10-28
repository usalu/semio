"""🎰️ usage: outer.py <fifo|pipe|pipe-high|none> <slots> -- <command...>
An independent GNU make style parent: it creates a job-token pool of <slots> slots (the implicit slot plus slots-1 tokens), names it in MAKEFLAGS like make does
(`-jN --jobserver-auth=fifo:PATH` or `--jobserver-auth=R,W`), hands the descriptors to the command under their own numbers (pipe-high moves them away from 3 and 4) and runs it.
`none` names no pool. The command's exit status is the exit status. Environment: JOBSERVER_DIRECTORY is where the fifo lives."""
import os
import subprocess
import sys
import tempfile

kind, slots = sys.argv[1], int(sys.argv[2])
command = sys.argv[sys.argv.index("--") + 1:]
environment = dict(os.environ)
pass_fds = ()
if kind == "fifo":
    directory = tempfile.mkdtemp(prefix="pool-", dir=os.environ.get("JOBSERVER_DIRECTORY"))
    path = directory + "/pool"
    os.mkfifo(path, 0o600)
    keep = os.open(path, os.O_RDWR | os.O_NONBLOCK)
    os.write(keep, b"+" * (slots - 1))
    environment["MAKEFLAGS"] = "-j%d --jobserver-auth=fifo:%s" % (slots, path)
elif kind in ("pipe", "pipe-high"):
    if kind == "pipe-high":
        for _ in range(4):
            os.open("/dev/null", os.O_RDONLY)
    read, write = os.pipe()
    os.write(write, b"+" * (slots - 1))
    os.set_inheritable(read, True)
    os.set_inheritable(write, True)
    environment["MAKEFLAGS"] = "-j%d --jobserver-auth=%d,%d" % (slots, read, write)
    pass_fds = (read, write)
if kind != "none":
    environment["CARGO_BUILD_JOBS"] = str(slots)
sys.exit(subprocess.run(command, env=environment, pass_fds=pass_fds).returncode)
