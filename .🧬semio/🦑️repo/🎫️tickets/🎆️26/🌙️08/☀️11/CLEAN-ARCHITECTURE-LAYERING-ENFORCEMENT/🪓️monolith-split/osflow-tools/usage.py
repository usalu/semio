"""📊 Which import leaves of a header (file) are used by which files. usage: usage.py <header.rs> <file>..."""
import os,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify,parse_use,gather_text
def toks(p):
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);s=set()
    for info in I:
        for n,c in info['toks']:s.add(n)
        for st in info['strs']:
            for m in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',st):s.add(m.group(1))
    return s
def leaves(p):
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);out=[]
    for it in top_items(L,I):
        classify(it)
        if it['kind']=='use':
            txt=gather_text(L,it)
            nm,anon,globs=parse_use(txt)
            cfg=' '.join(it['attrs'])
            out.append((it['start']+1,txt.strip().split('\n')[0][:90],[n for n,f in nm],globs,cfg))
    return out
if __name__=="__main__":
    header=sys.argv[1];files=sys.argv[2:]
    allt={f:toks(f) for f in files}
    for line,txt,names,globs,cfg in leaves(header):
        used=[n for n in names if any(n in t for t in allt.values())]
        un=[n for n in names if n not in used]
        print(f"{line:3d} {txt[:70]:70s} used={used} unused={un} globs={globs} {cfg}")
