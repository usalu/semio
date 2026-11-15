import sys,json,os,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import *
tools=json.load(open(S+'tools5.json'))
a=load('b0');b=load('t5')
rows=[]
for t in tools:
    name=t['name'];view=t.get('view','scheduling')
    ent=t.get('path')
    if ent is None:
        files=[r.replace('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/','LIB/') for r in t['resources']]
        probe=set(['rustfmt.toml','bunfig.toml','tsconfig.json','🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/📜️script.ts'])
        before=sum(1 for s in a['own'].values() if probe&s)
        after_files=sum(1 for s in b['own'].values() if probe&s)
    else:
        before=sum(1 for s in a['own'].values() if ent in s)
        after_files=sum(1 for s in b['own'].values() if ent in s)
    ref=f'toolClosure:{name}:{view}'
    refs=sum(1 for s in b['refs'].values() if ref in s)
    rows.append((before,name,after_files,refs,(ent or 'resources')))
rows.sort(key=lambda x:(-x[0],x[1]))
print('| # | group (tool) | owners embedding its files before | after | owners naming it by reference after |\n|---|---|---|---|---|')
for i,(bf,n,af,rf,e) in enumerate(rows,1):print(f'| {i} | `{n}` | {bf} | {af} | {rf} |')
json.dump(rows,open(S+'table-tools.json','w'),ensure_ascii=False)
