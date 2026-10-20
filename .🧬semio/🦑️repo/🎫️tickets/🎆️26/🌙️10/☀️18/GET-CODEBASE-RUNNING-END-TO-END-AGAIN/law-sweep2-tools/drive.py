import json,sys,os,re,subprocess,time,shlex
C=sys.argv[1];a=int(sys.argv[2]);b=int(sys.argv[3]);only=sys.argv[4] if len(sys.argv)>4 else 'verify'
G=C+'/🗑️generated/law-sweep2'
d=json.load(open(G+'/routes.json'))
if only=='verify': d=[r for r in d if r['command'].startswith('verify ')]
else: d=[r for r in d if not r['command'].startswith('verify ')]
d.sort(key=lambda r:r['command'])
res=open(G+'/results-'+only+'.jsonl','a')
idx=set(map(int,sys.argv[5].split(','))) if len(sys.argv)>5 else None
for i,r in enumerate(d):
    if idx is not None:
        if i not in idx: continue
    elif not (a<=i<b): continue
    o=r['options'];cwd=o['cwd']
    if '{projectRoot}' in cwd: cwd=os.path.dirname(r['file'])
    cmd=o['command']
    m=re.search(r'--cwd (\S+)',cmd)
    wrapper='process/📜️script.ts command' in cmd
    if wrapper and only=='verify':
        cwd=os.path.join(cwd,m.group(1));cmd=cmd.split(' -- ',1)[1]
    args=shlex.split(cmd.split('./📜️script.ts',1)[1]) if not (wrapper and only!='verify') else shlex.split(cmd.split('bun ',1)[1])
    skip=None
    if 'native' in args or '--native' in args: skip='native: cargo'
    elif args==['test-snapshot-sqlite']: skip='cargo repository-tests lease'
    if skip:
        res.write(json.dumps(dict(i=i,law=r['command'],project=r['project'],target=r['target'],cwd=cwd,args=args,rc=None,skip=skip),ensure_ascii=False)+'\n');res.flush();print(i,r['command'],'SKIP',skip);continue
    log=G+'/guard.log';open(log,'w').close()
    env=dict(os.environ,LAW_GUARD_LOG=os.path.abspath(log))
    t=time.time()
    q=subprocess.Popen(['perl','-e','alarm 300; exec @ARGV','--','bun',C+'/law-sweep2-tools/run-law.ts',cwd]+args,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,env=env,start_new_session=True)
    try:
        out,_=q.communicate(timeout=310);rc=q.returncode
    except subprocess.TimeoutExpired:
        os.killpg(q.pid,9);out,_=q.communicate();rc=-1;out='TIMEOUT\n'+out
    try: os.killpg(q.pid,9)
    except Exception: pass
    g=open(log).read().strip()
    name=r['command']
    open(G+('/log-%03d.txt'%i if only=='verify' else '/nlog-%03d.txt'%i),'w').write(out)
    row=dict(i=i,law=name,project=r['project'],target=r['target'],cwd=cwd,args=args,rc=rc,secs=round(time.time()-t),guard=g[:300],tail=out[-500:])
    res.write(json.dumps(row,ensure_ascii=False)+'\n');res.flush()
    print(i,name,'rc',rc,'%ds'%row['secs'],'GUARD:'+g.split('\t')[1][:60] if g else '')
