"""✅ Import completeness: names a crate's files use that the old parent header supplied but the new root does not. usage: complete.py <oldheader.rs>[,more] <newroot.rs> <file>..."""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from usage import toks,leaves
from rlex2 import lex2
from wsan import top_items,classify
def defined(p):
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);d=set()
    for it in top_items(L,I):
        classify(it)
        if it.get('name'):d.add(it['name'])
        if it['kind']=='macro':
            m=re.match(r'\s*macro_rules!\s*(\w+)',''.join(L[it['head']:it['end']+1]))
            if m:d.add(m.group(1))
    return d
if __name__=="__main__":
    olds=sys.argv[1].split(",");new=sys.argv[2];files=sys.argv[3:]
    cand={}
    for o in olds:
        for line,txt,names,globs,cfg in leaves(o):
            for n in names:cand[n]=(o,txt[:60])
    have=set()
    for line,txt,names,globs,cfg in leaves(new):have|=set(names)
    for f in files+[new]:have|=defined(f)
    used=set()
    for f in files:used|=toks(f)
    miss=sorted(n for n in cand if n in used and n not in have)
    print("missing:",[(n,cand[n][1]) for n in miss])
