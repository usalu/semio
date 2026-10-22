import sys,time
t0=time.time()
for line in sys.stdin.buffer:
    sys.stdout.buffer.write(("%8.1f "%(time.time()-t0)).encode()+line)
    sys.stdout.buffer.flush()
