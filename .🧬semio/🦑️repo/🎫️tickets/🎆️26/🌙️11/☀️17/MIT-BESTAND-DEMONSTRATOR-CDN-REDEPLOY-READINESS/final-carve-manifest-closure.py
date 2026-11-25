import os,re,sys,glob,collections
ROOT='.'
paths=[]
for d,ds,fs in os.walk('.'):
    ds[:]=[x for x in ds if x not in('node_modules','target','test-results') and not x.startswith('.')]
    if 'Cargo.toml' in fs: paths.append(os.path.join(d,'Cargo.toml'))
def parse(p):
    sec=None;name=None;deps=[];ws={};cur=None
    for line in open(p):
        s=line.strip()
        if not s or s.startswith('#'): continue
        m=re.match(r'\[(.+)\]$',s)
        if m:
            sec=m.group(1);
            mm=re.match(r'workspace\.dependencies\.(.+)',sec)
            if mm: ws[mm.group(1).strip('"')]=''
            continue
        if sec=='package':
            m=re.match(r'name\s*=\s*"([^"]+)"',s)
            if m and name is None: name=m.group(1)
        elif sec and (sec=='dependencies' or sec.endswith('.dependencies')) and not sec.startswith('workspace') and 'dev-' not in sec and 'build-' not in sec:
            m=re.match(r'([A-Za-z0-9_\-]+)\s*=\s*(.*)',s)
            if m:
                key,val=m.groups(); pkg=key
                mp=re.search(r'package\s*=\s*"([^"]+)"',val)
                if mp: pkg=mp.group(1)
                deps.append((pkg,key,'workspace' in val and 'true' in val,'optional = true' in val))
        elif sec=='workspace.dependencies':
            m=re.match(r'([A-Za-z0-9_\-]+)\s*=\s*(.*)',s)
            if m:
                key,val=m.groups(); mp=re.search(r'package\s*=\s*"([^"]+)"',val)
                ws[key]=mp.group(1) if mp else key
    return name,deps,ws
pk={};wsmap={}
for p in paths:
    n,d,w=parse(p)
    if n: pk[n]=d
    if w: wsmap.update({k:v or k for k,v in w.items()})
def edges(n):
    out=[]
    for pkg,key,isws,opt in pk.get(n,[]):
        if isws: pkg=wsmap.get(key,pkg)
        out.append((pkg,opt))
    return out
def closure(roots,drop=(),dropopt=False):
    seen=set();st=list(roots)
    while st:
        n=st.pop()
        if n in seen or n in drop: continue
        seen.add(n)
        for d,opt in edges(n):
            if dropopt and opt: continue
            st.append(d)
    return seen
import glob
sizes={}
for f in glob.glob('.cache/b/r/m/**/*.rlib',recursive=True):
    m=re.match(r'lib(.+)-[0-9a-f]+\.rlib',os.path.basename(f))
    if m: sizes[m.group(1)]=max(sizes.get(m.group(1),0),os.path.getsize(f)/1e6)
def sz(n): return sizes.get(n.replace('-','_'),0)
def fam(n):
    if n.startswith('semio-s-artifact-stdio-'): return 'stdio-'+n.split('semio-s-artifact-stdio-')[1].split('-')[0]
    if n.startswith('semio-hub'): return 'hub:'+n
    if n.startswith('semio-s-artifact-'):
        r=n[len('semio-s-artifact-'):].split('-'); return 'artifact-'+r[0]
    if n.startswith(('semio-plugin','semio-os')): return 'plugin/os'
    if n.startswith('semio-ui'): return 'ui'
    if n.startswith('semio-framework'): return 'framework'
    if n.startswith('semio'): return 'semio-other'
    return 'external'
def rep(title,cl):
    c=collections.Counter();m=collections.Counter()
    for n in cl: c[fam(n)]+=1; m[fam(n)]+=sz(n)
    print('##',title,len(cl),'crates, known rlib MB',round(sum(sz(n) for n in cl)),'of',sum(1 for n in cl if sz(n)),'crates')
    for k in sorted(c,key=lambda k:(-m[k],-c[k])): print(f'  {k}: {c[k]} crates, {m[k]:.0f} MB')
if __name__=='__main__':
    proc=closure(['semio-hub-process-component']);dem=closure(['semio-hub-demonstrator-component'])
    rep('process',proc);rep('demonstrator',dem)
    open(os.environ.get('OUT','/dev/null'),'w').write('')
    import json
    json.dump({'process':sorted(proc),'demonstrator':sorted(dem)},open(sys.argv[1],'w'))
