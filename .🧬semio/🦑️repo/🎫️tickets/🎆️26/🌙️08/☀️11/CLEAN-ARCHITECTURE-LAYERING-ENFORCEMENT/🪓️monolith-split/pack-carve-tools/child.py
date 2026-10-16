import re,sys,collections,glob,os
from rs import items,tokenize
ROOT='/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/'
def load(rel): return open(ROOT+rel,encoding='utf8').read()
def idents(text): return set(t[1] for t in tokenize(text) if t[0]=='id')
def defs_of(src):
    d={}
    for it in items(src):
        if it['kind'] in('fn','struct','enum','const','static','type','trait','mod','macro_rules','union') and it['name']: d[it['name']]=it
    return d
def imports_of(src):
    names=set()
    for it in items(src):
        if it['kind']=='use':
            body=it['name'] or ''
            for m in re.finditer(r'([A-Za-z_0-9]+)(?:as([A-Za-z_0-9]+))?(?=[,}]|$)',body):
                names.add(m.group(2) or m.group(1))
    return names
parent,child=sys.argv[1],sys.argv[2]
psrc=load(parent);csrc=load(child)
pd=defs_of(psrc);pi=imports_of(psrc)
cids=idents(csrc)
cd=defs_of(csrc)
used=[n for n in pd if n in cids and n not in cd]
imp=[n for n in pi if n in cids and n not in cd and n not in pd]
print('defs used from parent:',len(used)); print(sorted(used))
print('imports used via glob:',sorted(imp))
# parent uses of child defs
pids=idents(psrc)
print('child defs used by parent:',sorted(n for n in cd if n in pids and n not in pd))
