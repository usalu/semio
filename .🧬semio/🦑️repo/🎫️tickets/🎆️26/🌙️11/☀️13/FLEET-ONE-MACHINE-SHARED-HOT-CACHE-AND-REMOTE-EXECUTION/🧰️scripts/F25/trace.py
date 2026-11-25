import subprocess,sys,time,signal,os
out=sys.argv[1];limit=float(sys.argv[2]);cmd=sys.argv[3:]
p=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,cwd="/Users/ueli/Documents/semio",text=True,errors="replace",bufsize=1)
t0=time.time();sent=False
import threading
def watchdog():
    global sent
    while p.poll() is None:
        if time.time()-t0>limit and not sent:
            sent=True;p.send_signal(signal.SIGINT)
        time.sleep(1)
threading.Thread(target=watchdog,daemon=True).start()
with open(out,"w") as f:
    for line in p.stdout:
        f.write(f"{time.time()-t0:7.1f} {line}");f.flush()
    f.write(f"{time.time()-t0:7.1f} EXIT {p.wait()}\n")
