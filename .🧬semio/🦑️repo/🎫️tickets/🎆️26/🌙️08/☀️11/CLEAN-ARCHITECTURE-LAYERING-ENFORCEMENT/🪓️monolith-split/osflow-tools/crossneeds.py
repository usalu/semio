"""🔗 Names a crate uses that are defined in OTHER planned crates but neither imported nor glob-provided by its root. usage: crossneeds.py <crate-id>"""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from families import *
from build_crates import CR
from usage import toks,leaves
from pubnames import names
from complete import defined
from rlex2 import lex2
from wsan import top_items,classify
def allnames(p):
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);d={}
    for it in top_items(L,I):
        classify(it)
        if it.get('name') and it['kind'] in('fn','struct','enum','const','static','type','trait','union','mod'):d[it['name']]=it['kind']
        if it['kind']=='macro':
            m=re.match(r'\s*macro_rules!\s*(\w+)',''.join(L[it['head']:it['end']+1]))
            if m:d[m.group(1)]='macro'
    return d
ROOT_OF={ "semio_framework_os_flow_"+cid.replace("-","_"):FL+c.node+"/"+c.lib_rel for cid,c in CR.items()}
ART=FL+"🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs"
def globnames(root):
    """names provided by the glob imports of root file"""
    out=set()
    for line,txt,nm,globs,cfg in leaves(root):
        if not txt.lstrip().startswith(("use","pub use")):continue
        for g in globs:
            parts=g.split("::")
            if parts[0] in ROOT_OF and len(parts)==1:out|=set(k for k in names(ROOT_OF[parts[0]]) if not k.startswith("EXTGLOB"))
            elif parts[0] in ROOT_OF and len(parts)>1:
                # submodule glob e.g. ...::protocol::*  -> resolve module by file lookup in crate lib files
                pass
            elif g.endswith("artifact"):out|=set(k for k in names(ART) if not k.startswith("EXTGLOB"))
    return out
def run(cid):
    files=[FL+p for p in LIB[cid]+TESTS.get(cid,[]) if os.path.exists(FL+p)]
    root=FL+CR[cid].node+"/"+CR[cid].lib_rel
    own=set()
    for f in files:own|=set(allnames(f))
    t=set()
    for f in files:t|=toks(f)
    have=set()
    for line,txt,nm,globs,cfg in leaves(root):have|=set(nm)
    have|=globnames(root)
    # candidate providers: crates this one depends on
    internal={n.replace("semio-framework-os-flow-","") for n,_ in CR[cid].internal}
    miss={}
    for dep in internal:
        for f in LIB[dep]:
            p=FL+f
            if not os.path.exists(p):continue
            for n,k in allnames(p).items():
                if n in t and n not in own and n not in have and len(n)>3:
                    miss.setdefault(dep,set()).add(n)
    return miss
if __name__=="__main__":
    for cid in sys.argv[1:]:
        print(cid)
        for dep,ns in run(cid).items():print("   from",dep,sorted(ns))
