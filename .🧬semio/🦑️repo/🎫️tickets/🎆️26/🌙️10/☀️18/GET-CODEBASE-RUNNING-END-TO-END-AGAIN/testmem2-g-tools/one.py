#!/usr/bin/env python3
"""one.py <out.jsonl> <timeoutS> <ceilingMb> <file>...: runs each `bun test <file>` alone (nice 10, cwd = checkout), samples the PEAK RSS of its process tree every 0.3 s, kills the tree above the ceiling, appends one jsonl row per file. Own RSS is checked (abort above 200 MB)."""
import json,os,resource,signal,subprocess,sys,time
out,tmo,ceil=sys.argv[1],float(sys.argv[2]),int(sys.argv[3]);files=sys.argv[4:]
BUN=os.environ.get('BUN','.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun')
def table():
    rows={}
    for line in subprocess.run(['ps','-axo','pid=,ppid=,rss='],capture_output=True,text=True).stdout.splitlines():
        a=line.split()
        if len(a)==3: rows[int(a[0])]=(int(a[1]),int(a[2]))
    return rows
def tree(root,rows):
    kids={}
    for pid,(pp,_) in rows.items(): kids.setdefault(pp,[]).append(pid)
    seen=[root];i=0
    while i<len(seen): seen.extend(kids.get(seen[i],[]));i+=1
    return seen
for f in files:
    t=time.time();b=resource.getrusage(resource.RUSAGE_CHILDREN);log=out+'.'+str(abs(hash(f))%10**8)+'.log'
    with open(log,'w') as lf:
        env=dict(os.environ,SEMIO_TEST_ARTIFACT_DIR=os.environ.get('SCRATCH','/tmp')+'/art',TMPDIR=os.environ.get('SCRATCH','/tmp')+'/tmp',BUN_RUNTIME_TRANSPILER_CACHE_PATH='0');[os.makedirs(env[k],exist_ok=True) for k in ('SEMIO_TEST_ARTIFACT_DIR','TMPDIR')]
        p=subprocess.Popen(['nice','-n','10',BUN,'test','--timeout',str(int(tmo*1000))]+(['--preload',os.environ['PRELOAD']] if os.environ.get('PRELOAD') else [])+['./'+f],stdout=lf,stderr=subprocess.STDOUT,start_new_session=True,env=env)
        peak=0;tpeak=0;procs=0;why='ok'
        while p.poll() is None:
            rows=table();mem=tree(p.pid,rows);s=sum(rows[m][1] for m in mem if m in rows)//1024
            if s>peak: peak=s;tpeak=time.time()-t
            procs=max(procs,len(mem))
            if s>ceil or time.time()-t>tmo+30:
                why='ceiling' if s>ceil else 'timeout'
                for m in reversed(mem):
                    try: os.kill(m,signal.SIGKILL)
                    except OSError: pass
                break
            time.sleep(0.3)
        p.wait()
    a=resource.getrusage(resource.RUSAGE_CHILDREN);txt=open(log,errors='replace').read()
    import re
    m=re.search(r'(\d+) pass',txt);n=re.search(r'(\d+) fail',txt)
    row={'file':f,'rc':p.returncode,'why':why,'wallS':round(time.time()-t,1),'cpuS':round(a.ru_utime+a.ru_stime-b.ru_utime-b.ru_stime,1),'peakMb':peak,'tPeakS':round(tpeak,1),'procs':procs,'pass':int(m.group(1)) if m else None,'fail':int(n.group(1)) if n else None}
    open(out,'a').write(json.dumps(row,ensure_ascii=False)+'\n');print(json.dumps(row,ensure_ascii=False),flush=True)
    if why=='ok' and p.returncode==0: os.remove(log)
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss>200*1048576: sys.exit('rss')
