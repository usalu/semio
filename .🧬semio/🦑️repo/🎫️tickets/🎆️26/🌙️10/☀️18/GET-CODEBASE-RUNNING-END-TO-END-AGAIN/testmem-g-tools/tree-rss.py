#!/usr/bin/env python3
"""tree-rss.py: run a command and report wall, user+sys CPU of the whole tree and the PEAK SUM of RSS over the process tree (sampled every TREE_RSS_INTERVAL seconds (default 0.4)).
usage: tree-rss.py <label> <out.json> [--cwd DIR] [--env K=V]... -- <cmd...>   (stdout+stderr go to <out.json>.log; own RSS stays < 50 MB)"""
import json,os,resource,subprocess,sys,time
INTERVAL=float(os.environ.get('TREE_RSS_INTERVAL','0.4'))
label,out=sys.argv[1:3];rest=sys.argv[3:]
cwd=None;env=dict(os.environ)
while rest and rest[0]!='--':
    if rest[0]=='--cwd': cwd=rest[1];rest=rest[2:]
    elif rest[0]=='--env': k,v=rest[1].split('=',1);env[k]=v;rest=rest[2:]
    else: raise SystemExit('bad arg '+rest[0])
cmd=rest[1:]
def table():
    rows={}
    p=subprocess.run(['ps','-axo','pid=,ppid=,rss='],capture_output=True,text=True)
    for line in p.stdout.splitlines():
        a=line.split()
        if len(a)==3: rows[int(a[0])]=(int(a[1]),int(a[2]))
    return rows
def tree(root,rows):
    kids={}
    for pid,(pp,_) in rows.items(): kids.setdefault(pp,[]).append(pid)
    seen=[root];i=0
    while i<len(seen):
        seen.extend(kids.get(seen[i],[]));i+=1
    return seen
t=time.time();b=resource.getrusage(resource.RUSAGE_CHILDREN)
with open(out+'.log','w') as f:
    proc=subprocess.Popen(['nice','-n','10',*cmd],cwd=cwd,env=env,stdout=f,stderr=subprocess.STDOUT)
    peak=0;procs=0
    while proc.poll() is None:
        rows=table();members=tree(proc.pid,rows);s=sum(rows[m][1] for m in members if m in rows)
        peak=max(peak,s);procs=max(procs,len(members));time.sleep(INTERVAL)
wall=time.time()-t;a=resource.getrusage(resource.RUSAGE_CHILDREN)
row={'label':label,'rc':proc.returncode,'wallS':round(wall,1),'cpuS':round(a.ru_utime+a.ru_stime-b.ru_utime-b.ru_stime,1),'peakTreeRssMb':peak//1024,'maxProcesses':procs}
json.dump(row,open(out,'w'));print(json.dumps(row))
if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss>500*1048576: print('own RSS above 500 MB',file=sys.stderr)
