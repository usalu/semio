import subprocess,sys,time,re
# usage: fp.py <outfile> -- cmd...   samples footprint of every descendant each 2s, writes pid,footprintMB; prints peak per pid at end
out=sys.argv[1];cmd=sys.argv[3:]
t0=time.time()
p=subprocess.Popen(['/usr/bin/time','-l']+cmd,stdout=open(out+'.log','w'),stderr=open(out+'.err','w'))
def kids(pid):
    r=subprocess.run(['pgrep','-P',str(pid)],capture_output=True,text=True).stdout.split()
    res=[]
    for k in r: res.append(int(k)); res+=kids(int(k))
    return res
peak={}
def mb(s):
    m=re.search(r'Footprint:\s+([\d.]+)\s*(KB|MB|GB|B)',s)
    if not m: return None
    v=float(m.group(1));return v*{'B':1/1048576,'KB':1/1024,'MB':1,'GB':1024}[m.group(2)]
f=open(out+'.fp','w')
while p.poll() is None:
    for k in [p.pid]+kids(p.pid):
        r=subprocess.run(['footprint',str(k)],capture_output=True,text=True).stdout
        v=mb(r)
        if v is not None:
            f.write(f'{time.time()-t0:.0f} {k} {v:.0f}\n');f.flush()
            peak[k]=max(peak.get(k,0),v)
    try:
        n=sum(1 for _ in open(out+'.err','rb'))
    except Exception: n=0
    f.write(f'{time.time()-t0:.1f} lines {n}\n');f.flush()
    time.sleep(1)
print('wall',round(time.time()-t0,1),'rc',p.returncode)
print('peak footprint per pid (MB):',sorted(peak.items(),key=lambda x:-x[1])[:4])
