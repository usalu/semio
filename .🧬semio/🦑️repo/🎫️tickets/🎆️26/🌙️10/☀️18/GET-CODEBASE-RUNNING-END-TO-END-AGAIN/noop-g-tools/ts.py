import sys,time
t0=time.time()
for line in sys.stdin:
    sys.stdout.write("%8.2f %s"%(time.time()-t0,line));sys.stdout.flush()
