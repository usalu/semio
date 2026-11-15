#!/usr/bin/env python3
"""R5c: runs one `repo show projects` read with phase lines, prints the graph lines up to the stale summary, then sends ONE SIGINT and waits. usage: stale-probe.py <label> [limit-seconds]"""
import subprocess,sys,time,os,signal,select
label=sys.argv[1];limit=int(sys.argv[2]) if len(sys.argv)>2 else 420
env=dict(os.environ,SEMIO_GRAPH_PHASES="1");env.pop("SEMIO_FLEET_GROUP",None)
p=subprocess.Popen(["repo","show","projects"],cwd="/Users/Shared/semio",env=env,stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,text=True)
start=time.time();lines=[];done=False
while time.time()-start<limit and p.poll() is None:
    r,_,_=select.select([p.stderr],[],[],5)
    if r:
        l=p.stderr.readline()
        if not l:break
        lines.append(l.rstrip())
        if "graph:" in l and ("stale" in l or "refreshing" in l or "serving generation" in l or "published" in l):print(round(time.time()-start),l.rstrip()[:260],flush=True)
        if "leave" in l and "projects stale" in l:done=True;break
if p.poll() is None:
    p.send_signal(signal.SIGINT)
    try:p.wait(60)
    except subprocess.TimeoutExpired:print("still running after one SIGINT; left alone, pid",p.pid)
print(label,"elapsed",round(time.time()-start),"exit",p.poll(),"summary found",done)
