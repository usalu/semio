import sys,os,re,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex,items,name_kind
def load(p):
    src=open(p,encoding='utf-8').read()
    lines=src.split('\n')
    res=[]
    for it in items(src):
        v,k,nm=name_kind(it['toks'])
        a=it['first_tok_line'];b=it['end_line']
        res.append((k,nm,a+1,b+1,'\n'.join(lines[a:b+1])))
    return res
files=sys.argv[1:]
by=collections.defaultdict(list)
for f in files:
    for k,nm,a,b,t in load(f):
        if k in('fn','const','static','struct','enum','type','macro','impl','trait') and nm:
            by[(k,nm)].append((os.path.basename(os.path.dirname(f)),a,b,t))
for (k,nm),v in sorted(by.items()):
    if len(v)>1:
        same=len({x[3] for x in v})==1
        print(k,nm,[ (x[0],x[1],x[2]) for x in v],'IDENTICAL' if same else 'DIFFERENT')
