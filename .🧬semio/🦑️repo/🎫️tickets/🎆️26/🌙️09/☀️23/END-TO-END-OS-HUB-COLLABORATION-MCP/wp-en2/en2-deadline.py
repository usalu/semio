"""⏱️ EN2 lane-hold deadline: runs one command in its own process group and kills the WHOLE group (cargo + rustc
children, never orphaned) when the deadline passes, so an overlay/native hold never exceeds its budget.

Usage: en2-deadline.py <seconds> <command> [args…]   exit = the command's rc, or 124 on deadline
"""
import os
import signal
import subprocess
import sys
import time


def main(argv):
    seconds, command = float(argv[0]), argv[1:]
    child = subprocess.Popen(command, start_new_session=True)
    deadline = time.monotonic() + seconds
    while child.poll() is None:
        if time.monotonic() > deadline:
            print("[en2] deadline %ds reached — killing process group %d" % (seconds, child.pid), flush=True)
            os.killpg(child.pid, signal.SIGTERM)
            try:
                child.wait(20)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait()
            return 124
        time.sleep(2)
    return child.returncode


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
