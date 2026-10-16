"""🔎 Definition scan: a private item/field/method used in a file outside its defining module subtree. usage: defscan.py <dir>... [--ignore name,...]"""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify
args=[a for a in sys.argv[1:] if not a.startswith("--")]
ignore=set();defs_in=None
for a in sys.argv[1:]:
    if a.startswith("--ignore="):ignore=set(a[9:].split(","))
    if a.startswith("--defs-in="):defs_in=a[10:].split(",")
files=[]
for d in args:
    for dp,dn,fn in os.walk(d):
        dn[:]=[x for x in dn if x not in("node_modules","🕸️bindings","🗿️artifacts","🫀️core","🏛️ownership")]
        for f in fn:
            if f.endswith(".rs"):files.append(os.path.join(dp,f))
files=sorted(set(files))
data={}
for p in files:
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);data[p]=(L,I)
VIS=r'(?:pub(?:\([^)]*\))?\s+)'
def private_defs(p):
    L,I=data[p];out=[]
    its=top_items(L,I)
    for it in its:
        classify(it)
        h=it['text']
        if it['kind'] in('fn','struct','enum','const','static','type','trait','union') and it.get('name') and it['vis']=='':
            out.append((it['name'],'item',it['head']))
        if it['kind']=='struct' and it['end']>it['head']:
            for x in range(it['head']+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?(\w+)\s*:',L[x])
                    if m and not m.group(1):out.append((m.group(2),'field',x))
        if it['kind']=='struct' and re.search(r'struct\s+\w+(<[^>]*>)?\(',h):
            m=re.search(r'struct\s+(\w+)(?:<[^>]*>)?\((.*)\)',h)
            if m and re.search(r'(^|,)\s*(?!pub)[A-Za-z_&(\[]',m.group(2)):out.append((m.group(1),'tuple',it['head']))
        if it['kind']=='impl':
            head=''.join(L[it['head']:it['head']+1])
            if re.search(r'\bfor\b',head):continue
            for x in range(it['head']+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?((?:(?:unsafe|async|const)\s+)*)(fn|const)\s+(\w+)',L[x])
                    if m and not m.group(1):out.append((m.group(4),'method',x))
    return out
def uses(p):
    L,I=data[p];fieldu=set();methu=set();anyu=set()
    for x,info in enumerate(I):
        line=L[x]
        for name,col in info['toks']:
            pre=line[:col].rstrip();post=line[col+len(name):].lstrip()
            anyu.add(name)
            if pre.endswith('.') and not pre.endswith('..'):fieldu.add(name);methu.add(name)
            if pre.endswith('::'):methu.add(name)
            if post.startswith(':') and not post.startswith('::'):fieldu.add(name)
            if post.startswith('(') or post.startswith('::<'):methu.add(name)
        for s in info['strs']:
            for m in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',s):anyu.add(m.group(1))
    return fieldu,methu,anyu
U={p:uses(p) for p in files}
def subtree(p):
    d=os.path.dirname(p)
    return d+os.sep
bad=0
for p in files:
    if defs_in and not any(x in p for x in defs_in):continue
    sub=subtree(p)
    for name,kind,line in private_defs(p):
        if name in ignore or name in('new','default','fmt','from','into','clone','drop','eq'):continue
        hits=[]
        for q in files:
            if q==p or q.startswith(sub):continue
            f,m,a=U[q]
            ok={'item':name in a,'field':name in f,'method':name in m,'tuple':name in a}[kind]
            if ok:hits.append(q)
        if hits:
            bad+=1
            print(f"{os.path.relpath(p,args[0])}:{line+1}: private {kind} `{name}` used in {len(hits)} outside file(s): "+", ".join(os.path.relpath(h,args[0]) for h in hits[:3]))
print("candidates",bad)
