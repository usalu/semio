import sys,json,os,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import *
WS='/Users/Shared/semio/'
LIBR='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
tools=json.load(open(S+'tools5.json'))
a=load('b0')
w=json.load(open(S+'w-tools.json'))
json.dump({'base':{}},open(S+'w-base1.json','w'))
allw={'add':{},'append':{}}
for k,v in w.items():
    allw['add'].update(v['add']);allw['append'].update(v['append'])
json.dump({'all30':allw},open(S+'w-all30.json','w'),ensure_ascii=False)
dirs=[LIBR+'🏗️build/🧪️tests/',LIBR+'🏗️build/',LIBR+'⚡️caching/',LIBR+'⚡️caching/🦀️cargo/',LIBR,'🧰️framework/🛍️products/🦑️repo/','🧰️framework/🔨️modules/🏃️process/','🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/',LIBR+'🏗️build/📥️inputs/',LIBR+'🏃️process/🧭️routing/',LIBR+'🏗️build/🕸️graph/📥️inference/']
json.dump({'unreferenced':{'add':{WS+d+'r29-unreferenced.ts':'export const unreferenced = 1;\n' for d in dirs}}},open(S+'w-unref.json','w'),ensure_ascii=False)
ind={}
for t in tools:
    if t.get('path') is None:continue
    ent=t['path'];name=t['name']
    roots=sorted(r for r,s in a['own'].items() if ent in s)
    if not roots:continue
    v=w['add-'+name];v=dict(v);v['roots']=roots;ind['add-'+name]=v
json.dump(ind,open(S+'w-ind.json','w'),ensure_ascii=False)
print(len(ind),'individual worlds',sum(len(v['roots']) for v in ind.values()),'owner slots')
