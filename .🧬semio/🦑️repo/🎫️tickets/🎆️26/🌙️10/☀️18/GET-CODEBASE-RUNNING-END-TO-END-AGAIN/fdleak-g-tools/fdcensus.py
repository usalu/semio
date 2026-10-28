#!/usr/bin/env python3
"""fdcensus.py <out.jsonl> <timeoutS> <nofile> <listfile|-> [file...]: runs each `bun test <file>` ALONE (nice 10, own session, RLIMIT_NOFILE = <nofile>), samples open descriptors (libproc / procfs, no lsof) of the runner and every descendant every 0.25 s, records per file: peak fds of one process, peak total, the descriptor timeline slope, the processes that OUTLIVE the runner (survivors: tracked descendants still alive 1.5 s after exit, plus new ppid-1 processes started after the run began) and kills the whole group at the end (reported). Aborts a file when one process passes 0.8*nofile or the host passes kern.num_files > 120000. Resumes (files already in out.jsonl are skipped). Own RSS aborts above 200 MB. Env: BUN, SCRATCH, ENVX=K=V,K=V."""
import ctypes,json,os,re,resource,signal,subprocess,sys,time,uuid
out,tmo,nofile,lst=sys.argv[1],float(sys.argv[2]),int(sys.argv[3]),sys.argv[4]
files=[l.rstrip('\n') for l in open(lst)] if lst!='-' else []
files+=sys.argv[5:]
done=set()
if os.path.exists(out):
    for l in open(out): done.add(json.loads(l)['file'])
BUN=os.environ.get('BUN','.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun')
SC=os.environ.get('SCRATCH','/private/tmp/claude-501/fd/scratch')
DARWIN=sys.platform=='darwin'
if DARWIN: libproc=ctypes.CDLL('/usr/lib/libproc.dylib',use_errno=True)
def fdcount(pid):
    if DARWIN:
        n=libproc.proc_pidinfo(pid,1,0,None,0)
        if n<=0: return 0
        buffer=ctypes.create_string_buffer(n);got=libproc.proc_pidinfo(pid,1,0,buffer,n)
        return got//8 if got>0 else 0
    try: return len(os.listdir(f'/proc/{pid}/fd'))
    except OSError: return 0
def table():
    rows={}
    for line in subprocess.run(['ps','-axo','pid=,ppid=,pgid=,rss=,lstart=,command='],capture_output=True,text=True).stdout.splitlines():
        a=line.split(None,9)
        if len(a)>=9: rows[int(a[0])]=dict(ppid=int(a[1]),pgid=int(a[2]),rss=int(a[3]),start=' '.join(a[4:9]),cmd=(a[9] if len(a)>9 else '')[:200])
    return rows
def tree(root,rows):
    kids={}
    for pid,r in rows.items(): kids.setdefault(r['ppid'],[]).append(pid)
    seen=[root];i=0
    while i<len(seen): seen.extend(kids.get(seen[i],[]));i+=1
    return seen
def numfiles():
    try: return int(subprocess.run(['sysctl','-n','kern.num_files'],capture_output=True,text=True).stdout)
    except Exception: return 0
def marked(token):
    out=subprocess.run(['ps','-axeww','-o','pid=,ppid=,command='],capture_output=True,text=True,errors='replace',env=dict(os.environ,LC_ALL='C')).stdout
    rows=[]
    for line in out.splitlines():
        if token in line:
            a=line.split(None,2)
            if len(a)==3 and int(a[0])!=os.getpid(): rows.append(dict(pid=int(a[0]),ppid=int(a[1]),cmd=a[2][:160],fds=fdcount(int(a[0])),how='env-marker'))
    return rows
def kill_group(pgid):
    for sig in (signal.SIGTERM,signal.SIGKILL):
        try: os.killpg(pgid,sig)
        except OSError: pass
        time.sleep(0.4)
def limit(): resource.setrlimit(resource.RLIMIT_NOFILE,(nofile,nofile)); os.setsid()
for f in files:
    if f in done: continue
    t=time.time();log=out+'.'+re.sub(r'\W+','',str(abs(hash(f))%10**8))+'.log'
    token='SEMIO_FDCENSUS_RUN='+uuid.uuid4().hex
    env=dict(os.environ,**{token.split('=')[0]:token.split('=')[1]},SEMIO_TEST_ARTIFACT_DIR=SC+'/art',TMPDIR=SC+'/tmp',BUN_RUNTIME_TRANSPILER_CACHE_PATH='0')
    for kv in filter(None,os.environ.get('ENVX','').split(',')): k,v=kv.split('=',1);env[k]=v
    for k in ('SEMIO_TEST_ARTIFACT_DIR','TMPDIR'): os.makedirs(env[k],exist_ok=True)
    before=set(table())
    seen={};peak1=0;peakT=0;peakpid=None;peakcmd='';why='ok';series=[];lastnf=0;startstr=time.ctime(t)
    with open(log,'w') as lf:
        p=subprocess.Popen(['nice','-n','10',BUN,'test','--timeout',str(int(tmo*1000)),'./'+f],stdout=lf,stderr=subprocess.STDOUT,preexec_fn=limit,env=env)
        pgid=p.pid
        while p.poll() is None:
            rows=table();mem=set(tree(p.pid,rows))|{x for x,r in rows.items() if r['pgid']==pgid}
            tot=0
            for m in mem:
                if m not in rows: continue
                seen[m]=(rows[m]['start'],rows[m]['cmd']);c=fdcount(m);tot+=c
                if c>peak1: peak1=c;peakpid=m;peakcmd=rows[m]['cmd']
            peakT=max(peakT,tot);series.append((round(time.time()-t,1),tot,len(mem)))
            if time.time()-lastnf>2: lastnf=time.time();nf=numfiles()
            else: nf=0
            if peak1>0.8*nofile: why='fd-runaway'
            elif nf>120000: why='host-fd-pressure'
            elif time.time()-t>tmo+30: why='timeout'
            if why!='ok': kill_group(pgid);break
            time.sleep(0.25)
        p.wait()
    time.sleep(1.5)
    rows=table();survivors=[]
    for pid,(start,cmd) in seen.items():
        if pid in rows and rows[pid]['start']==start and pid!=p.pid: survivors.append(dict(pid=pid,ppid=rows[pid]['ppid'],cmd=cmd,fds=fdcount(pid),how='tracked'))
    for pid,r in rows.items():
        if pid not in before and pid not in seen and r['ppid']==1 and r['pgid']==pgid: survivors.append(dict(pid=pid,ppid=1,cmd=r['cmd'],fds=fdcount(pid),how='orphan-group'))
    known={x['pid'] for x in survivors};survivors+=[x for x in marked(token) if x['pid'] not in known and x['pid']!=p.pid]
    for x in survivors:
        try: os.kill(x['pid'],signal.SIGKILL)
        except OSError: pass
    if survivors: kill_group(pgid)
    txt=open(log,errors='replace').read()
    m=re.search(r'(\d+) pass',txt);n=re.search(r'(\d+) fail',txt)
    tail=series[-1][1] if series else 0
    row=dict(file=f,rc=p.returncode,why=why,wallS=round(time.time()-t,1),peakProcFds=peak1,peakTotalFds=peakT,peakProc=peakcmd[:120],maxProcs=max([s[2] for s in series],default=0),survivors=survivors,pass_=int(m.group(1)) if m else None,fail=int(n.group(1)) if n else None,series=series[::max(1,len(series)//12)])
    open(out,'a').write(json.dumps(row,ensure_ascii=False)+'\n');print(json.dumps({k:v for k,v in row.items() if k!='series'},ensure_ascii=False),flush=True)
    if why=='ok' and p.returncode==0 and not survivors: os.remove(log)
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss>200*1048576: sys.exit('rss')
