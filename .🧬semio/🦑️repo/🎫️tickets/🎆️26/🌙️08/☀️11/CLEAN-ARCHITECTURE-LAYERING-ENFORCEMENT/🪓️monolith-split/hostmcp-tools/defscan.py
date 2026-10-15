import sys,re,os,collections
sys.path.insert(0,'.')
from wsan import *
from wscommon import CHILDREN
class MM: pass
stage=sys.argv[1]
ws=stage
files={}
paths={"P":ws+"/🦀️.rs"}
for c,folder,mod,emoji,doc in CHILDREN:paths[c]=f"{ws}/{folder}/🦀️.rs"
W="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/"
extra={"P":[W+"🧪️tests/🔬️quick/🦀️.rs",W+"🧪️tests/🔬️long/🦀️.rs"],"C08":[W+"🪶️sqlite/🧪️tests/guest-input-slot.rs"]}
toks={}
for k,p in paths.items():
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L)
    files[k]=(L,I)
    s=set()
    for info in I:
        for n,c in info['toks']:s.add(n)
        for st in info['strs']:
            for m in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',st):s.add(m.group(1))
    toks[k]=s
for k,fl in extra.items():
    for f in fl:
        t=open(f,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L)
        for info in I:
            for n,c in info['toks']:toks[k].add(n)
bad=0;count=0
for c,folder,mod,emoji,doc in CHILDREN:
    L,I=files[c]
    its=top_items(L,I)
    for it in its:classify(it)
    other=set().union(*[toks[k] for k in toks if k!=c])
    for it in its:
        if it['name'] and it['vis']=='' and it['kind']!='use':
            count+=1
            if it['name'] in other:print("PRIVATE-ITEM-USED-ELSEWHERE",c,it['kind'],it['name']);bad+=1
        if it['kind']=='impl':
            head=gather_text(L,it).split('{')[0]
            if re.search(r'\bfor\b',head):continue
            for x in range(it['head']+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^(\s*)(pub(?:\([^)]*\))?\s+)?((?:(?:unsafe|async|const)\s+)*)(fn|const|static|type)\s+(\w+)',L[x])
                    if m and not m.group(2):
                        count+=1
                        if m.group(5) in other:print("PRIVATE-METHOD-USED-ELSEWHERE",c,m.group(5),x+1);bad+=1
        if it['kind']=='struct' and it['end']>it['head']:
            for x in range(it['head']+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^(\s*)(pub(?:\([^)]*\))?\s+)?(\w+)\s*:',L[x])
                    if m and not m.group(2):
                        count+=1
                        if m.group(3) in other:print("PRIVATE-FIELD-USED-ELSEWHERE",c,it['name'],m.group(3),x+1);bad+=1
        if it['kind']=='struct' and '(' in L[it['head']] and '{' not in L[it['head']]:
            line=L[it['head']]
            m=re.search(r'struct\s+\w+\s*(<[^>]*>)?\s*\((.*)\)',line)
            if m and re.search(r'(^|,)\s*(?!pub)[A-Za-z_&(\[]',m.group(2)) and it['name'] in other:
                print("TUPLE-FIELD-MAYBE-PRIVATE",c,it['name'],line.strip()[:100]);bad+=1
print("definitions checked",count,"violations",bad)
