#!/usr/bin/env python3
"""Real-bun oracle: stage a scope's manifests in a scratch tree and run a frozen dry-run install.
usage: oracle.py <repo-root> <scope> <scratch-root> [--lock <bun.lock path override>]"""
import json,os,shutil,subprocess,sys
root,scope,scratch=sys.argv[1],sys.argv[2],sys.argv[3]
lock_override=sys.argv[sys.argv.index('--lock')+1] if '--lock' in sys.argv else None
INERT={'scripts','semio'}
def rd(p):
    with open(p,encoding='utf8') as f:return json.load(f)
base=os.path.normpath(os.path.join(root,scope))
manifest=rd(os.path.join(base,'package.json'))
members=manifest.get('workspaces',[])
shutil.rmtree(scratch,ignore_errors=True)
def stage(directory,doc):
    rel=os.path.relpath(directory,root)
    dest=os.path.join(scratch,rel)
    os.makedirs(dest,exist_ok=True)
    patches=doc.get('patchedDependencies',{})
    for name,path in patches.items():
        src=os.path.join(directory,path);dst=os.path.join(dest,path)
        os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(src,dst)
    with open(os.path.join(dest,'package.json'),'w',encoding='utf8') as f:json.dump(doc,f)
    return dest
rootdoc={k:v for k,v in manifest.items() if k not in INERT}
dest=stage(base,rootdoc)
for m in members:
    d=os.path.normpath(os.path.join(base,m))
    doc={k:v for k,v in rd(os.path.join(d,'package.json')).items() if k not in INERT and k!='workspaces'}
    stage(d,doc)
shutil.copyfile(lock_override or os.path.join(base,'bun.lock'),os.path.join(dest,'bun.lock'))
for extra in ('bunfig.toml','.npmrc'):
    p=os.path.join(base,extra)
    if os.path.exists(p):shutil.copyfile(p,os.path.join(dest,extra))
cache=os.path.join(scratch,'.bun-cache')
r=subprocess.run([os.environ.get('BUN_BIN','bun'),'install','--frozen-lockfile','--dry-run','--ignore-scripts','--cache-dir',cache],cwd=dest,capture_output=True,text=True,timeout=900)
out=(r.stdout+r.stderr).strip().splitlines()
print(json.dumps({'scope':scope,'rc':r.returncode,'tail':out[-4:]},ensure_ascii=False))
sys.exit(0)
