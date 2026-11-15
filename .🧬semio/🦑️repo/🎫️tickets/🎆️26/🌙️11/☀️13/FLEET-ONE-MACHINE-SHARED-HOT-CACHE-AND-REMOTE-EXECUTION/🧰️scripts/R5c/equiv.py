import json,sys,collections
old=json.load(open(sys.argv[1]));new=json.load(open(sys.argv[2]))
so=old['worlds']['base']['sets'];sn=new['worlds']['base']['sets'];do=old['dict'];dn=new['dict']
assert old['sample']==new['sample']
lost=collections.Counter();under=0;over=0;same=0;extra=collections.Counter();pk=0
for i,(a,b) in enumerate(zip(so,sn)):
    A={do[x] for x in a['f']};B={dn[x] for x in b['f']}
    if A-B:under+=1;print('UNDER-KEY owner',old['sample'][i],sorted(A-B)[:3])
    if B-A:over+=1;extra[(old['sample'][i][0],len(B-A))]+=1
    if A==B:same+=1
    if set(a['p'])-set(b['p']):
        kind=old['sample'][i][0];lost[kind]+=1
        if kind!='target-sources':print('PACKAGES LOST',old['sample'][i],sorted(set(a['p'])-set(b['p']))[:3])
print('owners',len(so),'identical key inputs',same,'new strictly larger (over-key)',over,'UNDER-KEYED files',under,'package sets smaller by kind (target-sources are query-only, never hashed)',dict(lost))
print('extra files when larger:',dict(extra))
