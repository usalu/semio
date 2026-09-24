"""🪂️ W1: start a command in its own session (setsid) so no caller's process-group teardown reaches it.
usage: python3 w1-detach.py <log> <command…>"""
import subprocess, sys
log = open(sys.argv[1], "a")
proc = subprocess.Popen(sys.argv[2:], stdin=subprocess.DEVNULL, stdout=log, stderr=subprocess.STDOUT, start_new_session=True, cwd="/Users/ueli/Documents/semio/.tmp-ticket/wp-w1")
print(proc.pid)
