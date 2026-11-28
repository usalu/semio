#!/usr/bin/env python3
"""TS slice of the fleet probe: python3 ts-probe.py <project:target> <scratch-dir> <out.json>.
Phases: cold (--skip-cache), warm (local task cache), fleet (fresh NX_CACHE_DIRECTORY, remote up), down (fresh cache dir, cache+execution endpoints closed)."""
import json,os,re,socket,subprocess,sys,time
target,scratch,out=sys.argv[1:4]
ROOT=subprocess.run(["git","rev-parse","--show-toplevel"],capture_output=True,text=True,cwd=os.path.dirname(os.path.abspath(__file__))).stdout.strip()
def port():
    s=socket.socket();s.bind(("127.0.0.1",0));p=s.getsockname()[1];s.close();return p
down={"SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE":"1","SEMIO_REMOTE_CACHE_ENDPOINT":"http://127.0.0.1:%d"%port(),"SEMIO_REMOTE_EXECUTION_ENDPOINT":"http://127.0.0.1:%d"%port()}
rows=[]
for name,args,env in [("cold",["--skip-cache"],{}),("warm",[],{}),("fleet",[],{"NX_CACHE_DIRECTORY":os.path.join(scratch,"fleet")}),("down",["--skip-cache"],{**down,"NX_CACHE_DIRECTORY":os.path.join(scratch,"down")})]:
    w=time.time();p=subprocess.run(["repo","run",target]+args,capture_output=True,text=True,cwd=ROOT,env={**os.environ,"SEMIO_LANGUAGE":"en",**env})
    text=p.stdout+p.stderr
    rows.append({"phase":name,"exit":p.returncode,"wallSeconds":round(time.time()-w,1),"localCacheHits":len(re.findall(r"local cache",text)),"remoteCacheHits":len(re.findall(r"remote cache",text)),"tail":text.strip().splitlines()[-5:]})
    print(name,p.returncode,rows[-1]["wallSeconds"],rows[-1]["localCacheHits"],rows[-1]["remoteCacheHits"],flush=True)
json.dump(rows,open(out,"w"),indent=1)
