"""🎭 Facade content split: for each hub, lines of the hub's own mounted prod files that are pure re-export/mount items (use, mod decls, extern crate, inline mods made only of those) versus own content (types, fns, impls, macros, consts). `python3 facade_content.py`"""
from hubenv import *
from rs import *
def classify_file(f):
    t,sk,items=parse(f);re_l=0;own_l=0;own_items=collections.Counter()
    def pure(it):
        if it['kw'] in('use','extern crate','include'): return True
        if it['kw']=='mod':
            if not it.get('inline'): return True
            a=sk.find('{',it['start'],it['end']);b=sk.rfind('}',it['start'],it['end'])
            t2,sk2,its=parse_text(t[a+1:b],sk[a+1:b])
            return all(pure2(i) for i in its)
        return False
    def pure2(i):
        return i['kw'] in('use','extern crate','include') or (i['kw']=='mod' and not i.get('inline'))
    for it in items:
        n=t.count('\n',it['start'],it['end'])+1
        if pure(it): re_l+=n
        else: own_l+=n;own_items[it['kw']]+=1
    return re_l,own_l,own_items
for h in HUBS+['umbrella']:
    fs=ptree(h)
    R=O=0;oi=collections.Counter()
    for f in fs:
        if not isprod(f): continue
        r,o,i=classify_file(f);R+=r;O+=o;oi+=i
    print('%-14s prod files %2d  re-export/mount lines %5d  own-content lines %5d  own items %s'%(h,len([f for f in fs if isprod(f)]),R,O,dict(oi)))
