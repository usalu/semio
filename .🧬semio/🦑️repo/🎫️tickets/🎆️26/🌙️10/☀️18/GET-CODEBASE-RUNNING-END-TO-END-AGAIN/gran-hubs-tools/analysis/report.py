"""📊 Per-hub usage report from census-<hub>.json: edge hygiene (prod / test-only / unused), how prod users split between hub-owned definitions and re-exports of producer crates, per-file user counts. `python3 report.py <hub>`."""
from hubenv import *
h=sys.argv[1]
d=json.load(open(OUT+'/census-%s.json'%h));R_=d['resolved']
deps=d['deps']
cls=collections.Counter();own_users=collections.defaultdict(set);ext_users=collections.defaultdict(set)
only_ext=0;mixed=0;only_own=0;only_crate=0
per_user_owners={}
for dd,v in deps.items():
    if v['prod']: k='prod'
    elif v['test']: k='test'
    else: k='none'
    cls[k]+=1
    if k!='prod': continue
    kinds=set()
    for p in v['prod']:
        r=R_[p];kinds.add(r[0] if r[0]!='mod' else 'def')
        if r[0] in('def','mod'): own_users[r[1]].add(dd)
        elif r[0]=='ext': ext_users[r[1].split('::')[0]].add(dd)
    if kinds<={'ext'}: only_ext+=1
    elif kinds<={'def'}: only_own+=1
    elif kinds<={'crate'}: only_crate+=1
    else: mixed+=1
print(h,d['hub'],'dependents',len(deps),dict(cls))
print(' prod users: only-reexports',only_ext,'only-own-defs',only_own,'mixed',mixed,'bare-crate/unresolved',only_crate)
print(' own-def files by distinct prod users:')
for f,u in sorted(own_users.items(),key=lambda x:-len(x[1]))[:15]: print('   %5d %s'%(len(u),f[-90:]))
print(' re-exported producer idents by distinct prod users:')
for f,u in sorted(ext_users.items(),key=lambda x:-len(x[1]))[:25]: print('   %5d %s'%(len(u),f))
