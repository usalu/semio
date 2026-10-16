"""✅ Dependency completeness of carved crates: every known external ident used in a crate's lib or test closure is declared by its manifest (analysis only)."""
import sys,os,json,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from common import *
import deps
from rlex2 import lex2
def toks_by(f):
    t=open(f if os.path.isabs(f) else REPO+f,encoding='utf-8',errors='ignore').read();L=[l+'\n' for l in t.split('\n')];s=set()
    for info in lex2(L):
        for n,_ in info['toks']:s.add(n)
    return s
ALIASES={"dsl":"semio_framework_os_kernel","protocol":"semio_framework_os_kernel","store":"semio_framework_os_kernel","vcs":"semio_framework_os_kernel","value_derive":"semio_framework_value_derive","wasm_bindgen_futures":"semio_framework_async"}
def check(label,entry,pkg,manifest,universe,alias_decl):
    m=deps.idents(REPO+manifest)
    declared=set(m)
    out={}
    for cfgname,extra in [("lib",""),("test","test"),("wasm","target_arch=wasm32,target_os=unknown,target_family=wasm")]:
        files=[r[0] for r in closure(entry,pkg,extra) if r[0].endswith('.rs') and not r[0].startswith('/')]
        used=set()
        for f in files:used|=toks_by(f)
        miss=sorted(n for n in universe if n in used and n not in declared and not (n in ALIASES and ALIASES[n] in declared and n in alias_decl))
        aliasmiss=sorted(n for n in ALIASES if n in used and n not in alias_decl and ALIASES[n] in declared)
        out[cfgname]=(miss,aliasmiss,len(files))
    unused=sorted(n for n in declared if not any(n in toks_by(f) for f in [r[0] for r in closure(entry,pkg,"test") if r[0].endswith('.rs') and not r[0].startswith('/')]))
    print(label,"missing:",{k:v[0] for k,v in out.items() if v[0]},"alias-missing:",{k:v[1] for k,v in out.items() if v[1]},"unused-declared:",unused)
if __name__=="__main__":
    spec=json.load(open(sys.argv[1],encoding='utf-8'))
    universe=set()
    for man in spec["universeManifests"]:universe|=set(deps.idents(REPO+man))
    universe|={"png","tokio"}
    for c in spec["crates"]:
        src=open(REPO+c["entry"],encoding='utf-8').read()
        alias_decl=set(re.findall(r'extern crate [\w:]+ as (\w+);',src))
        check(c["label"],c["entry"],c["pkg"],c["manifest"],universe,alias_decl)
