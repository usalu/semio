"""📣 Public names a module exposes (items, pub use names, pub use globs of resolvable child modules)."""
import os,re,sys,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify,parse_use,gather_text
def load(p):
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];return L,lex2(L)
def mod_decls(p):
    L,I=load(p);out={}
    for i,l in enumerate(L):
        m=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',l)
        if m and I[i]['depth']==0:
            path=None
            for k in range(i-1,max(i-6,-1),-1):
                pm=re.match(r'^\s*#\[path\s*=\s*"([^"]+)"\]',L[k])
                if pm:path=pm.group(1);break
                if L[k].strip().startswith('#[') or not L[k].strip():continue
                break
            if path is None:
                d=os.path.dirname(p);cands=[os.path.join(d,m.group(2)+'.rs'),os.path.join(d,m.group(2),'mod.rs')]
                f=next((c for c in cands if os.path.exists(c)),None)
            else:f=os.path.normpath(os.path.join(os.path.dirname(p),path))
            out[m.group(2)]=(f,m.group(1) or '')
    return out
def names(p,seen=None):
    seen=seen or set()
    if p in seen:return {}
    seen=seen|{p}
    L,I=load(p);its=top_items(L,I);res={}
    decls=mod_decls(p)
    for it in its:
        classify(it)
        attrs=' '.join(it['attrs'])
        if it['kind'] in('fn','struct','enum','const','static','type','trait','union','mod') and it.get('name') and it['vis']=='pub':
            res[it['name']]=it['kind']
        elif it['kind']=='macro':
            m=re.match(r'\s*macro_rules!\s*(\w+)',gather_text(L,it))
            if m and 'macro_export' in attrs:res[m.group(1)]='macro'
        elif it['kind']=='use' and it['vis']=='pub':
            nm,anon,globs=parse_use(gather_text(L,it))
            for n,full in nm:res[n]='use:'+full
            for g in globs:
                parts=g.split('::')
                tgt=None
                if parts[0] in decls and len(parts)==1:tgt=decls[parts[0]][0]
                if tgt and os.path.exists(tgt):
                    res.update(names(tgt,seen))
                else:res['EXTGLOB:'+g]='glob'
    return res
if __name__=="__main__":
    r=names(sys.argv[1])
    for k in sorted(r):print(k,r[k])
