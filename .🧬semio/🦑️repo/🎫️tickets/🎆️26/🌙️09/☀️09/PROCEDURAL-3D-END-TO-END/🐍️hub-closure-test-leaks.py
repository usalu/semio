"""🔍️ Lists test-named crates and test features in the normal-dependency closure of a manifest dir (P production, T test crate). Usage: python3 -I this.py <abs crate dir>"""
import re,os,sys
import subprocess
root=subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip()
def parse(path):
    secs={};sec=''
    for l in open(path,encoding='utf8').read().split('\n'):
        m=re.match(r'\s*\[+([^\]]+)\]+\s*$',l)
        if m: sec=m.group(1).strip(); secs.setdefault(sec,[]); continue
        secs.setdefault(sec,[]).append(l)
    return secs
_wc={}
def wsmap(wr):
    if wr in _wc: return _wc[wr]
    ws=parse(wr+'/Cargo.toml') if os.path.exists(wr+'/Cargo.toml') else {}
    m={}
    for l in ws.get('workspace.dependencies',[]):
        mm=re.match(r'\s*([\w\-]+)\s*=\s*\{.*path\s*=\s*"([^"]+)"',l)
        if mm: m[mm.group(1)]=os.path.normpath(os.path.join(wr,mm.group(2)))
    _wc[wr]=m; return m
def wsroot(d,secs):
    for l in secs.get('package',[]):
        mm=re.match(r'\s*workspace\s*=\s*"([^"]+)"',l)
        if mm: return os.path.normpath(os.path.join(d,mm.group(1)))
    return root
def name(secs):
    for l in secs.get('package',[]):
        m=re.match(r'\s*name\s*=\s*"([^"]+)"',l)
        if m:return m.group(1)
def deps(d,secs):
    out=[];wsdeps=wsmap(wsroot(d,secs))
    for s,ls in secs.items():
        if not (s=='dependencies' or (s.startswith('target.') and s.endswith('.dependencies')) or s=='build-dependencies'): continue
        for i,l in enumerate(ls):
            if l.strip().startswith('#'):continue
            m=re.match(r'\s*"?([\w\-\.]+)"?(?:\.workspace)?\s*=\s*(.*)',l)
            if not m: continue
            k,v=m.groups()
            p=None
            pm=re.search(r'path\s*=\s*"([^"]+)"',v)
            if pm: p=os.path.normpath(os.path.join(d,pm.group(1)))
            elif 'workspace' in l and k in wsdeps: p=wsdeps[k]
            feats=re.search(r'features\s*=\s*\[([^\]]*)\]',v)
            out.append((k,p,feats.group(1) if feats else '',l.strip()))
    return out
start=sys.argv[1]
seen={};stack=[os.path.normpath(start)]
while stack:
    d=stack.pop()
    if d in seen: continue
    f=d+'/Cargo.toml'
    if not os.path.exists(f): seen[d]=None;continue
    secs=parse(f); seen[d]=(name(secs),deps(d,secs))
    for k,p,fe,l in seen[d][1]:
        if p: stack.append(p)
pat=re.compile(r'(-tests?(-[\w\-]+)?|-testkit)$|testkit|test-fixtures|-test-')
names={d:v[0] for d,v in seen.items() if v}
print('closure crates',len(seen),file=sys.stderr)
for d,v in sorted(seen.items(),key=lambda x:x[1][0] if x[1] else ''):
    if not v: continue
    n,ds=v
    isTest=bool(re.search(r'-tests?(-|$)|testkit|-test-',n))
    for k,p,fe,l in ds:
        tn=names.get(p) if p else k
        if (tn and re.search(r'-tests?(-|$)|-test-|testkit',tn)) or 'testkit' in fe or 'artifact-app-testing' in fe or 'protocol-laws' in fe or 'mutation-testing' in fe:
            print(('T ' if isTest else 'P ')+n+'\t'+(tn or k)+'\t'+fe.strip()+'\t'+d.replace(root+'/','')[-60:])
