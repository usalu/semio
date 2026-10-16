import re,sys,collections
from rs import items,tokenize
ROOT='/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/'
def load(rel):
    return open(ROOT+rel,encoding='utf8').read()
def idents(text):
    return set(t[1] for t in tokenize(text) if t[0]=='id')
def lineno(src,off): return src.count('\n',0,off)+1
src=load('🌱️value/🦀️.rs')
its=items(src)
regions=[('prelude',1,65),('tags',66,95),('canon',96,373),('enc',374,675),('ret',676,2032),('dec',2033,2494),('tbl',2495,2749),('hash',2750,3127),('doc',3128,3366),('tail',3367,9999)]
def region_of(ln):
    for n,a,b in regions:
        if a<=ln<=b: return n
defs=collections.defaultdict(set) # name -> regions
itemreg=[]
for it in its:
    ln=lineno(src,it['start']); r=region_of(ln)
    it['region']=r
    if it['kind'] in('fn','struct','enum','const','static','type','trait','mod','macro_rules','union') and it['name']:
        defs[it['name']].add(r)
# usage per region
use=collections.defaultdict(lambda:collections.defaultdict(set))
for it in its:
    ids=idents(it['text'])
    for n,rs in defs.items():
        if n in ids:
            for r in rs:
                if r!=it['region']: use[it['region']][r].add(n)
for r in [x[0] for x in regions]:
    print('==',r)
    for r2,ns in sorted(use[r].items()):
        print('  uses',r2,len(ns),sorted(ns)[:40])
