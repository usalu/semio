import sys,time
for line in sys.stdin:
    sys.stdout.write(time.strftime("%H:%M:%S ")+line);sys.stdout.flush()
