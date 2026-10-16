import re,collections
from rs import items,tokenize
ROOT='/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/'
def load(rel): return open(ROOT+rel,encoding='utf8').read()
def idents(text): return set(t[1] for t in tokenize(text) if t[0]=='id')
rootsrc=load('🌱️value/🦀️.rs')
its=items(rootsrc)
def ln(off): return rootsrc.count('\n',0,off)+1
groups={}
def grp(it):
    a=ln(it['start'])
    if it['kind']=='use' and a<66: return 'imports'
    if 66<=a<=95: return 'tags'
    if 96<=a<=372: return 'canon'
    if 374<=a<=675: return 'enc'
    if 676<=a<=2030: return 'ret'
    if 2031<=a<=2494: return 'dec'
    if 2495<=a<=2749: return 'tbl'
    if 2750<=a<=3127: return 'hash'
    if 3128<=a<=3366: return 'doc'
    if a<66: return 'mounts'
    return 'tail'
defs={}
for it in its:
    if it['kind'] in('fn','struct','enum','const','static','type','trait','mod','macro_rules','union') and it['name'] and it['kind']!='mod':
        defs[it['name']]=it; it['g']=grp(it)
    else: it['g']=grp(it)
# external users: child files
children={'E':'🌱️value/🛫️encode','PF':'🌱️value/🫳️preflight','RS':'🌱️value/🛬️decode/🫴️receiving/🦀️.rs','SRC':'🌱️value/🛬️decode/📖️source/🦀️.rs','STO':'🌱️value/🎟️storage/🦀️.rs','SCH':'🌱️value/🏭️schema','INTR':'🌱️value/🌳️intrinsic/🦀️.rs'}
import glob,os
def files(spec):
    p=ROOT+spec
    if p.endswith('.rs'): return [p]
    out=[]
    for dp,dn,fn in os.walk(p):
        if '🧪️tests' in dp: continue
        for f in fn:
            if f=='🦀️.rs': out.append(os.path.join(dp,f))
    return out
usedby=collections.defaultdict(set)
for it in its:
    ids=idents(it['text'])
    for n,d in defs.items():
        if n in ids and d is not it:
            usedby[n].add(it['g'])
for cname,spec in children.items():
    for f in files(spec):
        ids=idents(open(f,encoding='utf8').read())
        for n in defs:
            if n in ids: usedby[n].add(cname)
for n,d in sorted(defs.items(),key=lambda kv:ln(kv[1]['start'])):
    u=usedby[n]-{d['g']}
    print(f"{d['g']:6} {d['kind']:6} {n:45} vis={d['vis'] or '-':10} used_by={sorted(usedby[n])}")
