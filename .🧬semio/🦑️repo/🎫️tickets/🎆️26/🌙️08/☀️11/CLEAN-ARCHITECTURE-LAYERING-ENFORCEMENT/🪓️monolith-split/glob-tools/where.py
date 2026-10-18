import sys,os,re,pickle
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rs import *; from roots import *
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
tg,deps=pickle.load(open(S+"/crates.pkl","rb"))
CH={}
def kids(path):
    if path not in CH:
        try: text=read(path)
        except Exception: text=""
        CH[path]=[(n,f) for n,v,f,a in mod_children(path,text)] if "mod " in text else []
    return CH[path]
def tree_light(path,mod=(),acc=None):
    acc={} if acc is None else acc
    acc[path]=mod
    for name,f in kids(path):
        if f not in acc and os.path.exists(f): tree_light(f,mod+(name,),acc)
    return acc
if __name__=="__main__":
    want={k:os.path.join(find(v),'🦀️.rs') for k,v in roots.items()}
    found={k:[] for k in roots}
    for (manifest,kind,name,path,pkg) in tg:
        if not os.path.exists(path): continue
        acc=tree_light(path)
        for k,w in want.items():
            if w in acc: found[k].append((pkg,kind,name,os.path.relpath(path,R),acc[w]))
    for k,v in found.items():
        print('==',k)
        for x in v: print('   ',x[0],x[1],x[2],'mod=','::'.join(x[4]) or '<root>',' root=',x[3][-70:])
    pickle.dump(found,open(S+"/where.pkl","wb"))
