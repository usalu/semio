#!/usr/bin/env python3
"""watch.py <out.jsonl> [interval]: streams every `bun test` process (peak RSS, files, ancestry) as one jsonl row when it ends; aborts above 200 MB own RSS."""
import json,os,resource,subprocess,sys,time
out=sys.argv[1];iv=float(sys.argv[2]) if len(sys.argv)>2 else 2.0
env=dict(os.environ,LC_ALL='en_US.UTF-8')
live={}
def snap():
    p=subprocess.run(['ps','-axo','pid=,ppid=,rss=,etime=,command='],capture_output=True,text=True,env=env,errors='replace')
    rows={}
    for l in p.stdout.splitlines():
        a=l.split(None,4)
        if len(a)==5: rows[int(a[0])]=(int(a[1]),int(a[2]),a[3],a[4])
    return rows
def short(c):
    c=c.replace(os.getcwd()+'/','')
    return c[:260]
def isbun(c):
    t=c.split()
    return len(t)>2 and t[0].endswith('/bun') and (t[1]=='test' or (t[1].startswith('--') and 'test' in t[:4])) 
def emit(pid,r):
    json.dump(r,open(out,'a'),ensure_ascii=False);open(out,'a').write('\n')
while True:
    rows=snap();now=time.time()
    for pid,(pp,rss,et,cmd) in rows.items():
        if isbun(cmd):
            r=live.get(pid)
            if r is None:
                anc=[];q=pp
                for _ in range(5):
                    if q in rows: anc.append(short(rows[q][3])[:140]);q=rows[q][0]
                    else: break
                r=live[pid]={'pid':pid,'cmd':short(cmd),'anc':anc,'peakKb':0,'t0':now,'samples':[]}
            r['peakKb']=max(r['peakKb'],rss);r['last']=now
            if len(r['samples'])<400: r['samples'].append([int(now-r['t0']),rss//1024])
    for pid in [p for p in live if p not in rows or not isbun(rows[p][3])]:
        r=live.pop(pid);r['durS']=int(r['last']-r['t0']);emit(pid,r)
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss>200*1048576: sys.exit('rss')
    time.sleep(iv)
