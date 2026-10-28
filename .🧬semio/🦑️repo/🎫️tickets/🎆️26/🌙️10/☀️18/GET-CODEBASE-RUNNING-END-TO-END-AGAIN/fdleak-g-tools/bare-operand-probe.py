import ctypes,os,resource,signal,subprocess,sys,time
lp=ctypes.CDLL('/usr/lib/libproc.dylib')
def cnt(pid):
    n=lp.proc_pidinfo(pid,1,0,None,0)
    if n<=0: return 0
    b=ctypes.create_string_buffer(n);r=lp.proc_pidinfo(pid,1,0,b,n);return r//8 if r>0 else 0
arg=sys.argv[1];limit=int(sys.argv[2])
BUN='.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun'
def pre(): resource.setrlimit(resource.RLIMIT_NOFILE,(limit,limit));os.setsid()
p=subprocess.Popen(['nice','-n','10',BUN,'test']+sys.argv[3:]+[arg],stdout=subprocess.PIPE,stderr=subprocess.STDOUT,preexec_fn=pre,text=True)
t=time.time();peak=0;series=[]
while p.poll() is None and time.time()-t<40:
    c=cnt(p.pid);peak=max(peak,c);series.append((round(time.time()-t,1),c));time.sleep(0.5)
if p.poll() is None:
    os.killpg(p.pid,signal.SIGKILL)
out=p.communicate()[0]
print('arg',arg[:60],'limit',limit,'peak',peak,'rc',p.returncode,'wall',round(time.time()-t,1));print(series[::4]);print(out[-600:])
