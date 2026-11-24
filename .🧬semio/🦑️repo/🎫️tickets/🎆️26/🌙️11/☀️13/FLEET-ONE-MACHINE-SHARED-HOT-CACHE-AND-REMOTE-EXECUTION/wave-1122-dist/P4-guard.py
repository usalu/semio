import os,signal,subprocess,sys,time,shutil
limit_s=int(sys.argv[1]);stopfinal=os.environ.get('P4_STOP_AT_FINAL');scratch=sys.argv[2];out=sys.argv[3];cmd=sys.argv[4:]
def du(p):
    t=0
    for r,d,f in os.walk(p):
        for x in f:
            try:t+=os.lstat(os.path.join(r,x)).st_size
            except OSError:pass
    return t
o=open(out,'ab')
p=subprocess.Popen(cmd,stdout=o,stderr=subprocess.STDOUT,start_new_session=True)
t0=time.time();why=None
while p.poll() is None:
    for _ in range(15):
        time.sleep(1)
        if stopfinal and p.poll() is None:
            r=subprocess.run(['pgrep','-f','crate-name '+stopfinal+' .*'+scratch],capture_output=True,text=True).stdout.split()
            if r:why='final reached (stop before it)';break
    if not why:
      pass
    free=shutil.disk_usage('/').free/2**30
    size=du(scratch)/2**30
    print(f"[guard] t={int(time.time()-t0)}s free={free:.1f}GiB scratch={size:.2f}GiB",flush=True)
    if why:pass
    elif free<100:why="disk free<100GiB"
    elif size>15:why="scratch>15GiB"
    elif time.time()-t0>limit_s:why="step time limit"
    if why:
        os.killpg(p.pid,signal.SIGTERM);time.sleep(5)
        try:os.killpg(p.pid,signal.SIGKILL)
        except ProcessLookupError:pass
        break
print("[guard] end",why,"rc",p.poll(),flush=True)
