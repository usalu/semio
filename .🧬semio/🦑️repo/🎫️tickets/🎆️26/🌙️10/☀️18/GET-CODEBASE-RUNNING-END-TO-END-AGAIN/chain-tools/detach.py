import subprocess, sys
log = open(sys.argv[1], "ab", buffering=0)
p = subprocess.Popen(sys.argv[2:], stdout=log, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
print(p.pid)
