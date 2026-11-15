import sys,json,os,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import *
S2=S
WS='/Users/Shared/semio/'
tools=json.load(open(S+'tools5.json'))
a=load('b0');b=load('t5')
LIB=WS+'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
union=set()
for v in b['sets'].values():union|=v
import re
def leaf(p):
    try:t=open(p,encoding='utf8').read()
    except Exception:return False
    return len(t)<4000 and not re.search(r'\bimport\b|\brequire\s*\(|\bfrom\s*["\']',t)
rows=[]
for t in tools:
    if t.get('path') is None:continue
    ent=t['path']
    before=sum(1 for r,s in a['own'].items() if ent in s)
    roots=sorted(r for r,s in b['sets'].items() if ent in s)
    rows.append((before,t['name'],ent,roots))
rows.sort(key=lambda x:(-x[0],x[1]))
worlds={}
for before,name,ent,roots in rows:
    d=os.path.dirname(WS+ent);cand=None
    for up in range(9):
        for base,_,fs in os.walk(d):
            if 'node_modules' in base or '/.' in base[len(d):]:continue
            for f in sorted(fs):
                full=base+'/'+f;rel=os.path.relpath(full,WS)
                if f.endswith('.ts') and not f.endswith('.d.ts') and rel not in union and full!=WS+ent and leaf(full):cand=full;break
            if cand:break
        if cand:break
        d=os.path.dirname(d)
    assert cand,ent
    added=cand
    rel=os.path.relpath(added,os.path.dirname(WS+ent))
    worlds['add-'+name]=dict(add={},append={WS+ent:'\nimport "'+(rel if rel.startswith('.') else './'+rel)+'";\n'},roots=roots)
json.dump(worlds,open(S+'w-tools.json','w'),ensure_ascii=False)
json.dump([(x[0],x[1],len(x[3])) for x in rows],open(S+'blast-before.json','w'),ensure_ascii=False)
for x in rows:print(x[0],x[1],len(x[3]))
