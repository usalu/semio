import os,sys,shutil,re,subprocess,collections,json
T=os.path.dirname(os.path.abspath(__file__))
REPO='/Users/ueli/Documents/semio/'
SC=os.environ.get('RHS_SCRATCH','/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad')
RUSTC=['rustc','+nightly-2026-07-20','--edition','2021','--crate-type','lib','--emit=metadata','-o','/dev/null','--error-format=short','-W','unused-imports']
GLOB=re.compile(r'^(\s*)(pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_][A-Za-z0-9_]*)(::[A-Za-z0-9_:{}, ]*)?::\*\s*;',re.M)
LOCAL={'super','crate','self'}
def patch_text(t,local_crates=()):
    t=re.sub(r'^\s*extern crate [^;]+;\s*$','',t,flags=re.M)
    def g(m):
        if m.group(3) in LOCAL or m.group(3) in local_crates:return m.group(0)
        return m.group(1)+'const _: () = ();'
    return GLOB.sub(g,t)
def copytree(src,dst,skip=()):
    for root,dirs,files in os.walk(src):
        dirs[:]=[d for d in dirs if d not in skip and d!='node_modules']
        rel=os.path.relpath(root,src)
        os.makedirs(os.path.join(dst,rel),exist_ok=True)
        for f in files:
            shutil.copy2(os.path.join(root,f),os.path.join(dst,rel,f))
def build(spec,label,variant_dir=None,orig_file=None):
    """spec: mirror (repo-relative dir copied), target (repo-relative dir receiving the variant/orig root), root (root file name in target)."""
    sb=os.path.join(SC,'rhs-sbx',label)
    shutil.rmtree(sb,ignore_errors=True)
    os.makedirs(sb)
    mir=spec['mirror'] if 'mirror' in spec else spec['real_dir']
    dst=os.path.join(sb,mir)
    os.makedirs(dst)
    copytree(REPO+mir,dst,skip=set(spec.get('skip',['📦️packages'] if 'mirror' not in spec else [])))
    tgt=os.path.join(sb,spec['target'] if 'target' in spec else spec['real_dir'])
    if orig_file:shutil.copy(orig_file,os.path.join(tgt,spec['root_file']))
    if variant_dir:
        for e in os.listdir(variant_dir):
            p=os.path.join(variant_dir,e)
            d=os.path.join(tgt,e)
            if os.path.isdir(p):
                shutil.rmtree(d,ignore_errors=True);shutil.copytree(p,d)
            else:shutil.copy(p,d)
    mods=set()
    for root,dirs,files in os.walk(sb):
        for f in files:
            if f.endswith('.rs'):
                t=open(os.path.join(root,f),encoding='utf-8').read()
                mods|=set(re.findall(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*[;{]',t,flags=re.M))
    for root,dirs,files in os.walk(sb):
        for f in files:
            if f.endswith('.rs'):
                p=os.path.join(root,f)
                t=open(p,encoding='utf-8').read()
                n=patch_text(t,mods)
                if n!=t:open(p,'w',encoding='utf-8').write(n)
    return sb
def run(sb,spec,label,extra,outdir):
    os.makedirs(outdir,exist_ok=True)
    cmd=RUSTC+['--crate-name',spec['crate']]+extra+[spec['root']]
    r=subprocess.run(cmd,cwd=os.path.join(sb,spec['run_dir'] if 'run_dir' in spec else spec['real_dir']),capture_output=True,text=True)
    raw=r.stderr
    open(os.path.join(outdir,label+'.raw'),'w',encoding='utf-8').write(raw)
    return raw
def norm(raw,sb):
    c=collections.Counter()
    for l in raw.split('\n'):
        l=l.strip()
        if not l or l.startswith('error: aborting') or "couldn't create a temp dir" in l or l.startswith('error: could not compile'):continue
        m=re.match(r'^(.*?\.rs):(\d+):(\d+): (.*)$',l)
        if m:
            c[m.group(4)]+=1
        else:c[l]+=1
    return c
def compare(a,b):
    ma=a-b;mb=b-a
    return ma,mb
