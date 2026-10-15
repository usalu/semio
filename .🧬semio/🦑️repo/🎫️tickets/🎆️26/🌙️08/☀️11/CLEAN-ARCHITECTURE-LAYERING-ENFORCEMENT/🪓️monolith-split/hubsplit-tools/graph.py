import sys,json,re,collections
sys.path.insert(0,'.')
from rsitems import *
H='/Users/ueli/Documents/semio/🌎️hub/'
src=open(H+'🏗️bootstrap/🦀️.rs',encoding='utf-8').read()
its=items(src)
defs={}
rows=[]
for it in its:
    v,k,nm=name_kind(it['toks'])
    ids={t[1] for t in it['toks'] if t[0]=='id'}
    rows.append(dict(first=it['first_tok_line']+1,hdr=it['hdr_line']+1,end=it['end_line']+1,vis=v,kind=k,name=nm,attrs=it['attrs'],ids=ids))
    if nm and k not in('impl','use'):
        defs.setdefault(nm,[]).append(len(rows)-1)
tests=''
import glob
tsrc=open(H+'🧪️tests/🔬️bin-unit/🦀️.rs',encoding='utf-8').read()
ttoks=lex(tsrc)
tids=collections.Counter(t[1] for t in ttoks if t[0]=='id')
other={}
for f in ['🧪️tests/🤝️two-client-document/🦀️.rs','🧪️tests/🔬️standalone/🦀️.rs','🧪️tests/🗂️artifact-root/🦀️.rs','🧪️tests/🔬️bin-unit/🪶️count-lease/🦀️.rs','🧪️tests/🎲️hostile-generative/🦀️.rs']:
    try:
        s=open(H+f,encoding='utf-8').read()
        other[f]=collections.Counter(t[1] for t in lex(s) if t[0]=='id')
    except Exception as e: print('ERR',f,e)
json.dump([dict(r,ids=sorted(r['ids'])) for r in rows],open('items.json','w'))
if __name__=='__main__':
    for i,r in enumerate(rows):
        if r['name'] and r['kind'] not in('use',):
            r['tests']=tids.get(r['name'],0)
    # print table: line, kind, name, test refs
    for r in rows:
        if r['kind'] in('use',):continue
        print(r['first'],r['end'],r['vis'],r['kind'],r['name'],'T=%d'%tids.get(r['name'],0) if r['name'] else '')
