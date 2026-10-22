#!/usr/bin/env python3
"""run.py <timeout_s> <outfile> cmd... : run under nice 10, SIGINT then SIGKILL on timeout, print wall ms."""
import subprocess,sys,time,os,signal
t=float(sys.argv[1]);out=sys.argv[2];cmd=sys.argv[3:]
la=os.getloadavg()[0]
s=time.perf_counter()
p=subprocess.Popen(["nice","-n","10"]+cmd,stdout=open(out,"wb"),stderr=subprocess.STDOUT,start_new_session=True)
try:
    rc=p.wait(timeout=t)
except subprocess.TimeoutExpired:
    os.killpg(p.pid,signal.SIGINT)
    try: rc=p.wait(timeout=5)
    except subprocess.TimeoutExpired:
        os.killpg(p.pid,signal.SIGKILL);rc=p.wait();rc="killed"
    rc=f"timeout({rc})"
print(f"wall_ms={(time.perf_counter()-s)*1000:.0f} rc={rc} load1={la:.1f} cmd={' '.join(cmd)}")
