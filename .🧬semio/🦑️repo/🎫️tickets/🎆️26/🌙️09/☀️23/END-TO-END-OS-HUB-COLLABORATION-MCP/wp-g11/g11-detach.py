"""🪂️ G11 (copy of W2): start a command in its own session (setsid, stdin /dev/null) so no caller's process-group teardown reaches it.
usage: python3 g11-detach.py <log> <command…>; prints the pid."""
import subprocess, sys
log = open(sys.argv[1], "a")
proc = subprocess.Popen(sys.argv[2:], stdin=subprocess.DEVNULL, stdout=log, stderr=subprocess.STDOUT, start_new_session=True, cwd="/Users/ueli/Documents/semio/.tmp-ticket/wp-g11")
print(proc.pid)
