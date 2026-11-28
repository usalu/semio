# -*- coding: utf-8 -*-
import os, signal, subprocess, sys, time
ROOT="/Users/ueli/Documents/semio"
file=sys.argv[1]
env={**os.environ,"SEMIO_TEST_FILE_OFFLOAD":"always","SEMIO_LANGUAGE":"en"}
p=subprocess.Popen([os.path.expanduser("~/.local/bin/repo"),"test-file","--json","--",file],cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,start_new_session=True)
start=time.time()
time.sleep(float(sys.argv[2]))
os.killpg(p.pid,signal.SIGINT)
try:
    out,err=p.communicate(timeout=120)
except subprocess.TimeoutExpired:
    os.killpg(p.pid,signal.SIGKILL);out,err=p.communicate()
print("exit",p.returncode,"after",round(time.time()-start,1),"s")
lines=[l for l in err.splitlines() if "test-file]" in l or "cancel" in l.lower() or "abgebrochen" in l]
print("\n".join(lines[-6:])[:1200])
