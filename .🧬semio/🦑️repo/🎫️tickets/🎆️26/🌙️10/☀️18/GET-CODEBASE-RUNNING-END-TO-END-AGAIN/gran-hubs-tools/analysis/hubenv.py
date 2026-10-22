"""🧭 Shared environment for the hub analyses: gran-value graph, tree walker, reach and per-file churn of a hub (git log pathspec limited to the hub's own mounted files, read-only)."""
import os,sys,json,subprocess,collections
T=os.path.dirname(os.path.abspath(__file__))
GEN=os.path.normpath(T+"/../../🗑️generated/gran-value")
OUT=os.path.normpath(T+"/../../🗑️generated/gran-hubs")
os.environ.setdefault("GRAN_GEN",OUT if os.path.exists(OUT+"/crates2.json") else GEN)
sys.path.insert(0,os.path.normpath(T+"/../../gran-value-tools/analysis"))
from graph import *
from tree import *
GEN=os.path.normpath(T+"/../../🗑️generated/gran-value")
reach=json.load(open(GEN+"/reach.json"))
HUBS=['plugin','os-kernel','kernel','job','os-store-sync']
def hubdir(h): return byname['semio-framework-'+h]
def tree(h):
    d=hubdir(h);return module_tree(libfile(d,C[d]))
def isprod(f): return '🧪️' not in f
def churn_files(files):
    rel=[os.path.relpath(f,ROOT) for f in files]
    res=collections.defaultdict(set);cur=None
    out=subprocess.run(['git','log','--name-only','--format=@@%h','--']+rel,cwd=ROOT,capture_output=True,text=True).stdout
    for l in out.splitlines():
        if l.startswith('@@'): cur=l[2:]
        elif l.strip(): res[os.path.normpath(os.path.join(ROOT,l.strip()))].add(cur)
    return res
def nlines(f):
    try: return open(f,encoding='utf-8',errors='ignore').read().count('\n')
    except Exception: return 0
import re as _re
_PATH=_re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
_INC=_re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
def _gated(t,pos):
    s=t[max(0,pos-240):pos]
    cut=max(s.rfind(';'),s.rfind('}'),s.rfind('{'))
    s=s[cut+1:]
    return 'cfg(test)' in s.replace(' ','')
def prod_tree(lib):
    """module tree without #[cfg(test)]-gated mounts/includes (regex approximation of Rust's cfg stripping)."""
    seen=[];S=set();st=[lib]
    while st:
        f=os.path.normpath(st.pop())
        if f in S or not os.path.isfile(f): continue
        S.add(f);seen.append(f)
        try: t=open(f,encoding='utf-8',errors='ignore').read()
        except Exception: continue
        d=os.path.dirname(f)
        for m in _PATH.finditer(t):
            if not _gated(t,m.start()): st.append(os.path.join(d,m.group(1)))
        for m in _INC.finditer(t):
            if not _gated(t,m.start()): st.append(os.path.join(d,m.group(1)))
    return seen
def ptree(h):
    d=hubdir(h) if h!='umbrella' else byname['semio-framework']
    return prod_tree(libfile(d,C[d]))

def make_world():
    """World over all crates with per-crate dependency-key maps (renamed dependencies resolve to the target lib ident)."""
    from cratemodel import World,Model
    MAN=json.load(open(OUT+'/manifests.json'))
    lib={d:(MAN.get(d,{}).get('lib') or c['name']).replace('-','_') for d,c in C.items()}
    I2L={lib[d]:libfile(d,c) for d,c in C.items()}
    I2D={lib[d]:d for d in C}
    dm={}
    for d,c in C.items():
        m={}
        for k in('normal','build','dev'):
            for key,t in c['deps'][k]:
                if t in C: m[key.replace('-','_')]=lib[t]
        dm[lib[d]]=m
    w=World(I2L,dm);Model.world=w
    return w,I2L,I2D,lib


def libmaps():
    MAN = json.load(open(OUT + '/manifests.json'))
    lib = {d: (MAN.get(d, {}).get('lib') or c['name']).replace('-', '_') for d, c in C.items()}
    return lib, {lib[d]: d for d in C}
