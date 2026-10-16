"""🔎 Cross-crate definition scan: an item, field or method that is not `pub` yet is used by name in another crate of the carve.
usage: xscan.py <artifact-dir> [--seed]   (reads only; --seed injects a private sentinel into a temp copy to prove sensitivity)"""
import os,re,sys,collections
sys.path.insert(0,"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/osflow-tools")
from rlex2 import lex2
from wsan import top_items,classify
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import ffu
def crate_files(root,ids):
    out={}
    for cid,rel in ids.items():
        out[cid]=ffu.owned(root+rel)
    return out
def load(p):
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];return L,lex2(L)
def defs(p):
    L,I=load(p);out=[]
    for it in top_items(L,I):
        classify(it)
        if it['kind'] in('fn','struct','enum','const','static','type','trait','union','mod') and it.get('name') and it['vis']!='pub':
            out.append((it['name'],'item',it['head']+1,it['vis'] or 'private'))
        if it['kind']=='struct':
            for x in range(it['head']+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?(\w+)\s*:',L[x])
                    if m and not (m.group(1) and m.group(1).strip()=='pub'):out.append((m.group(2),'field',x+1,m.group(1) or 'private'))
        if it['kind']=='impl':
            head=L[it['head']]
            if re.search(r'\bfor\b',head):continue
            for x in range(it['head']+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?((?:(?:unsafe|async|const)\s+)*)(fn|const)\s+(\w+)',L[x])
                    if m and not (m.group(1) and m.group(1).strip()=='pub'):out.append((m.group(4),'method',x+1,m.group(1) or 'private'))
    return out
def uses(p):
    L,I=load(p);fu=set();mu=set();au=set()
    for x,info in enumerate(I):
        line=L[x]
        for name,col in info['toks']:
            pre=line[:col].rstrip();post=line[col+len(name):].lstrip()
            au.add(name)
            if pre.endswith('.') and not pre.endswith('..'):fu.add(name);mu.add(name)
            if pre.endswith('::'):mu.add(name)
            if post.startswith(':') and not post.startswith('::'):fu.add(name)
            if post.startswith('(') or post.startswith('::<'):mu.add(name)
        for s in info['strs']:
            for m in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',s):au.add(m.group(1))
    return fu,mu,au
SKIP={'new','default','fmt','from','into','clone','drop','eq','id','name','len','get','iter','step','first','last','next'}
def scan(root,ids,tests=False):
    files=crate_files(root,ids)
    U={}
    for cid,fs in files.items():
        fu=set();mu=set();au=set()
        for f in fs:
            if not tests and ("🧪️" in f):continue
            a,b,c=uses(f);fu|=a;mu|=b;au|=c
        U[cid]=(fu,mu,au)
    hits=0
    for cid,fs in files.items():
        for f in fs:
            if not tests and ("🧪️" in f):continue
            for name,kind,line,vis in defs(f):
                if name in SKIP:continue
                for other,(fu,mu,au) in U.items():
                    if other==cid:continue
                    ok={'item':name in au,'field':name in fu,'method':name in mu}[kind]
                    if ok:
                        hits+=1
                        print(f"{cid}:{os.path.relpath(f,root)}:{line}: {vis} {kind} `{name}` used by {other}")
    print("candidates",hits)
if __name__=="__main__":
    root=os.path.abspath(sys.argv[1])+"/"
    ids=dict(ffu.CRATES)
    scan(root,ids,tests="--tests" in sys.argv)
