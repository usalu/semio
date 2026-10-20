#!/usr/bin/env python3
"""Hub proof matrix for `bun repo --tool=bazel run`: per target dry run, cold, warm, after-clean (disk cache kept), one whitespace edit and its revert; spawns are the delta of the Bazel activity ledger that `cache status` reports. Run only after GO, alone on a quiet host. Usage: prove.py [--cold] [--after-clean] [--only N] [--timeout S]."""
import hashlib,json,os,re,shutil,subprocess,sys,time
ROOT='/Users/ueli/Documents/semio'
HERE=os.path.dirname(os.path.abspath(__file__))
C=os.path.dirname(HERE)
OUT=os.path.join(C,'🗑️generated','hub-bzl','prove')
V=chr(0xFE0F)
tracked={f.replace(V,''):f for f in subprocess.run(['git','-c','core.quotepath=false','ls-files','-z'],cwd=ROOT,capture_output=True).stdout.decode().split('\0') if f}
def edit(plain): return tracked[plain]
TARGETS=[
 {'id':'go-service-test','task':'@semio-tech/repo-coordinator-go:test','edit':edit('🧰framework/🛍products/🦑repo/🔨modules/🖥server/🎛coordinator/📦packages/🐹go/📦main/🐹.go')},
 {'id':'hub-rust-lib-check','task':'@semio-tech/hub-auth-client-rs:check','edit':edit('🌎hub/🔐auth/🔌client/🦀.rs')},
 {'id':'hub-frontend-build','task':'os-hub-admin:build','edit':edit('🌎hub/🔨modules/🛡admin/📦packages/🟦typescript/🟦.tsx')},
 {'id':'composition-build','task':'semio-hub-note-component:build','edit':edit('🌎hub/🧩compositions/🗒note/🧩component/🦀.rs')},
 {'id':'hub-publication-test','task':'@semio-tech/hub-stdio-publication:test','edit':edit('🌎hub/🧩compositions/🗄stdio/📇publication/🟦.ts')},
]
args=sys.argv[1:]
timeout=int(args[args.index('--timeout')+1]) if '--timeout' in args else 1800
only=int(args[args.index('--only')+1]) if '--only' in args else None
os.makedirs(OUT,exist_ok=True)
def sh(cmd,log,limit=timeout):
    start=time.time()
    with open(os.path.join(OUT,log),'w',encoding='utf8') as f:
        r=subprocess.run(cmd,cwd=ROOT,stdout=f,stderr=subprocess.STDOUT,timeout=limit)
    return r.returncode,round(time.time()-start,1)
def read(log): return open(os.path.join(OUT,log),encoding='utf8',errors='replace').read()
def status(tag):
    rc,_=sh(['bun','repo','--tool=bazel','cache','status'],f'status-{tag}.log',600)
    text=read(f'status-{tag}.log')
    def num(*keys):
        for k in keys:
            m=re.search(r'"?'+k+r'"?\s*[:=]\s*(\d+)',text,re.I)
            if m: return int(m.group(1))
    directory=re.search(r'"?directory"?\s*[:=]\s*"?([^\s",]+)',text)
    return {'builds':num('builds'),'executed':num('executed'),'cacheHits':num('cacheHits','cache_hits'),'directory':directory.group(1) if directory else None,'raw':text if rc else None}
def delta(a,b):
    return {k:(b[k]-a[k] if a[k] is not None and b[k] is not None else None) for k in ('builds','executed','cacheHits')}
def run(target,tag):
    before=status(tag+'-pre')
    rc,seconds=sh(['bun','repo','--tool=bazel','run',target['task']],f'run-{tag}.log')
    after=status(tag+'-post')
    return {'phase':tag,'rc':rc,'seconds':seconds,**delta(before,after)}
rows=[]
for index,target in enumerate(TARGETS):
    if only is not None and index!=only: continue
    t=target['id']
    rc,s=sh(['bun','repo','--tool=bazel','run',target['task'],'--dry-run'],f'dry-bazel-{t}.log',600)
    planned=len(re.findall(r'^\s*[@\w][^\s]*:[\w-]+',read(f'dry-bazel-{t}.log'),re.M))
    rc2,_=sh(['bun','repo','--tool=nx','run',target['task'],'--dry-run'],f'dry-nx-{t}.log',600)
    rows.append({'target':t,'phase':'dry-run','rc':rc,'seconds':s,'plannedBazel':planned,'plannedNx':len(re.findall(r'^\s*[@\w][^\s]*:[\w-]+',read(f'dry-nx-{t}.log'),re.M))})
    if '--cold' in args:
        sh(['bun','repo','--tool=bazel','cache','clear'],f'clear-{t}.log',900)
    rows.append({'target':t,**run(target,f'{t}-cold')})
    rows.append({'target':t,**run(target,f'{t}-warm')})
    if '--after-clean' in args:
        st=status(f'{t}-dir')
        d=st['directory']
        if d and os.path.isdir(d) and sum(os.path.getsize(os.path.join(a,b)) for a,_,fs in os.walk(d) for b in fs)<3<<30:
            keep=f'/private/tmp/hubbzl/disk-{t}'
            shutil.rmtree(keep,ignore_errors=True); shutil.copytree(d,keep)
            sh(['bun','repo','--tool=bazel','cache','clear'],f'clear2-{t}.log',900)
            shutil.rmtree(d,ignore_errors=True); shutil.copytree(keep,d); shutil.rmtree(keep)
            rows.append({'target':t,**run(target,f'{t}-after-clean')})
        else: rows.append({'target':t,'phase':'after-clean','skipped':'disk cache directory unknown or larger than 3 GiB'})
    path=os.path.join(ROOT,target['edit']); original=open(path,'rb').read(); digest=hashlib.sha256(original).hexdigest()
    try:
        open(path,'wb').write(original+b'\n')
        rows.append({'target':t,**run(target,f'{t}-edit')})
    finally:
        open(path,'wb').write(original)
        assert hashlib.sha256(open(path,'rb').read()).hexdigest()==digest,'revert failed: '+path
    rows.append({'target':t,**run(target,f'{t}-revert')})
print(json.dumps(rows,ensure_ascii=False,indent=1))
open(os.path.join(OUT,'matrix.json'),'w',encoding='utf8').write(json.dumps(rows,ensure_ascii=False,indent=1))
