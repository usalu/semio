import re,collections,sys
from rs import items,tokenize
ROOT='/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/'
def load(rel): return open(ROOT+rel,encoding='utf8').read()
def ids(t): return set(x[1] for x in tokenize(t) if x[0]=='id')
rootsrc=load('🌱️value/🦀️.rs')
its=items(rootsrc)
def ln(off): return rootsrc.count('\n',0,off)+1
def grp(a):
    if 66<=a<=95: return 'W'
    if 96<=a<=372: return 'canon'
    if 374<=a<=675: return 'enc'
    if 676<=a<=2030: return 'RR'
    if 2031<=a<=2494: return 'dec'
    if 2495<=a<=2749: return 'tbl'
    if 2750<=a<=3127: return 'RS'
    if 3128<=a<=3366: return 'doc'
    if a<66: return 'mounts'
    return 'tail'
defs={}
for it in its:
    if it['kind'] in('fn','struct','enum','const','static','type','trait','enum') and it['name']:
        defs[it['name']]=(grp(ln(it['start'])),it['vis'] or 'priv')
# children public/private names
import os
def childdefs(rel):
    d={}
    for it in items(load(rel)):
        if it['kind'] in('fn','struct','enum','const','static','type','trait') and it['name']: d[it['name']]=it['vis'] or 'priv'
    return d
E=childdefs('🌱️value/🛫️encode/🦀️.rs')
imports=set()
for it in its:
    if it['kind']=='use':
        for m in re.finditer(r'([A-Za-z_0-9]+)(?:as([A-Za-z_0-9]+))?(?=[,}]|$)',it['name'] or ''): imports.add(m.group(2) or m.group(1))
tests=sys.argv[1:]
for t in tests:
    i=ids(load(t))
    by=collections.defaultdict(list)
    for n,(g,v) in defs.items():
        if n in i: by[g].append(n+('' if v!='priv' else '*'))
    print('==',t,len(load(t)))
    for g,ns in sorted(by.items()): print('  ',g,sorted(ns))
    ie=[n for n in E if n in i]
    if ie: print('   E-root names:',sorted(ie))
    print('   root imports used:',sorted(n for n in imports if n in i))
