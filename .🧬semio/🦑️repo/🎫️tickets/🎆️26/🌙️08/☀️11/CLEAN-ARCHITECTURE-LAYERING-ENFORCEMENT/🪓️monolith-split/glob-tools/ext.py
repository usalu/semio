import sys,os,re,pickle
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from roots import *
from rs import *
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
C,IDX=pickle.load(open(S+"/index.pkl","rb"))
byname={}
for p,ns in IDX.items():
    for n in ns: byname.setdefault(n,set()).add(p)
def exported(root_key):
    d=find(roots[root_key]); t=tree(os.path.join(d,'🦀️.rs'))
    rm=strip(t[()][1]); res={}
    for g in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+(\w+)::\*;',rm,re.M):
        m=g.group(1)
        if (m,) in t:
            for i in top_items(t[(m,)][1]):
                if i['vis'] and i['name']!='*': res[i['name']]=(m,i)
    return d,t,res
if __name__=="__main__":
    for k in roots:
        d,t,ex=exported(k)
        treefiles={os.path.relpath(p,R) for p,_ in t.values()}
        # also include fragments? treat dir
        dd=os.path.relpath(d,R)
        out={}
        for n,(m,i) in ex.items():
            fs=[f for f in byname.get(n,()) if not f.startswith(dd+'/')]
            out[n]=fs
        n_ext=sum(1 for v in out.values() if v)
        print(k,len(ex),'names;',n_ext,'with outside-tree files;', sum(len(v) for v in out.values()),'file hits')
