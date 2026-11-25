import json,os,re,subprocess,collections
R="/Users/ueli/Documents/semio"
SKIP={'node_modules','.git','target','.cache','🎫️tickets','dist'}
MOD=re.compile(r'((?:[ \t]*#\[[^\]\n]*\]\s*\n)*)[ \t]*(pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*;')
PATHA=re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
class Edits:
    def __init__(s): s.t={}
    def get(s,p):
        if p not in s.t: s.t[p]=open(p,encoding='utf8').read()
        return s.t[p]
    def set(s,p,x): s.t[p]=x
    def new(s,p,x): s.t[p]=x
    def dump_new(s,d):
        for p,x in s.t.items():
            if not os.path.exists(p):
                q=os.path.join(d,p);os.makedirs(os.path.dirname(q),exist_ok=True);open(q,'w',encoding='utf8').write(x)
    def write(s):
        for p,x in s.t.items():
            os.makedirs(os.path.dirname(p) or '.',exist_ok=True);open(p,'w',encoding='utf8').write(x)
def ls(pattern):
    return [l for l in subprocess.run(['git','ls-files','-z','--',pattern],capture_output=True,text=True,cwd=R).stdout.split('\0') if l and '🎫️tickets' not in l]
def index():
    idx={}
    for dp,dn,fn in os.walk('.'):
        dn[:]=[d for d in dn if d not in SKIP]
        if 'Cargo.toml' in fn:
            p=os.path.join(dp,'Cargo.toml')
            m=re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"',open(p,encoding='utf8').read(),re.S|re.M)
            if m: idx[m.group(1)]=p[2:]
    return idx
def libroot(mf,text):
    m=re.search(r'^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"',text,re.S|re.M)
    return os.path.normpath(os.path.join(os.path.dirname(mf),m.group(1) if m else 'src/lib.rs'))
def tree(entry):
    mods={};order=[]
    def walk(f,chain):
        if f in mods or not os.path.exists(f): return
        mods[f]=dict(file=f,chain=chain,children=[]);order.append(f)
        t=open(f,encoding='utf8').read()
        for attrs,vis,name in MOD.findall(t):
            pa=PATHA.search(attrs)
            if not pa: continue
            p=os.path.normpath(os.path.join(os.path.dirname(f),pa.group(1)))
            if os.path.isdir(p) or not p.endswith('.rs'): continue
            mods[f]['children'].append((name,p));walk(p,chain+[name])
    walk(entry,[]);return mods
def sections(text):
    out={};cur=None
    for l in text.split('\n'):
        m=re.match(r'^\[([^\]]+)\]\s*$',l)
        if m: cur=m.group(1);out[cur]=[];continue
        if cur is not None: out[cur].append(l)
    return out
def dep_lines(text,section='dependencies'):
    d={}
    for l in sections(text).get(section,[]):
        m=re.match(r'^([\w\-]+)\s*=\s*(.*)$',l)
        if m: d[m.group(1)]=m.group(2).strip()
    return d
def pkg_of(dep_key,val): 
    m=re.search(r'package\s*=\s*"([^"]+)"',val);return m.group(1) if m else dep_key
def rel(frm_dir,to_path): return os.path.relpath(to_path,frm_dir)
def lock_blocks(text):
    head,*b=text.split("\n[[package]]\n");return head,[(re.match(r'name = "([^"]+)"',x).group(1),x) for x in b]
def lock_join(head,blocks): return head+"".join("\n[[package]]\n"+b for _,b in blocks)
def lock_deps(block):
    m=re.search(r'dependencies = \[\n((?: "[^"]+",?\n)*)\]',block);return m,(re.findall(r'"([^"]+)"',m.group(1)) if m else [])
def set_lock_deps(block,deps):
    m,_=lock_deps(block);body="".join(f' "{d}",\n' for d in sorted(set(deps)))
    if m: return block[:m.start(1)]+body+block[m.end(1):]
    return block.rstrip("\n")+"\ndependencies = [\n"+body+"]\n"
def lockname(entry): return entry.split(' ')[0]
def crate_of(file,libroots,mounted,manifests_dirs,cache={}):
    if file in cache: return cache[file]
    r=None
    if file in libroots: r=libroots[file]
    elif file in mounted: r=crate_of(mounted[file],libroots,mounted,manifests_dirs)
    else:
        d=os.path.dirname(file)
        while d:
            c=os.path.join(d,'📦️packages/🦀️rust/Cargo.toml')
            if c in manifests_dirs: r=manifests_dirs[c];break
            d=os.path.dirname(d)
    cache[file]=r;return r
def world(idx):
    libroots={};mfs={}
    for pkg,mf in idx.items():
        t=open(mf,encoding='utf8').read();libroots[libroot(mf,t)]=pkg;mfs[mf]=pkg
    mounted={}
    out=subprocess.run(['grep','-rIl','--include=*.rs','#\\[path','.']+[f'--exclude-dir={d}' for d in SKIP],capture_output=True,text=True).stdout.split('\n')
    for f in out:
        if not f: continue
        f=f[2:];t=open(f,encoding='utf8').read()
        for attrs,vis,name in MOD.findall(t):
            pa=PATHA.search(attrs)
            if pa: mounted[os.path.normpath(os.path.join(os.path.dirname(f),pa.group(1)))]=f
    return libroots,mounted,mfs
