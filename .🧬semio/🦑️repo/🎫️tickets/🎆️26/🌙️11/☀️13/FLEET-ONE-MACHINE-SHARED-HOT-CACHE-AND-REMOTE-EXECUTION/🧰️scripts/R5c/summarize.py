import json,sys,collections
d=json.load(open(sys.argv[1]));S=d['sample'];W=d['worlds']
kinds=collections.Counter(k for k,_ in S);print('sample',len(S),dict(kinds))
base=W['base']
print('base errors',base['errors'])
for name,w in W.items():
    if name=='base':continue
    byk=collections.defaultdict(lambda:[0,0,0])
    for (k,_),bn,wn,bk,wk in zip(S,base['nodes'],w['nodes'],base['keys'],w['keys']):
        byk[k][0]+=1;byk[k][1]+= bn!=wn;byk[k][2]+= bk!=wk
    gch=[g for g in base['groups'] if base['groups'][g]!=w['groups'].get(g)]
    print(name,'errors',w['errors'],'| nodes changed / task keys changed by kind:',{k:(v[1],v[2],'of',v[0]) for k,v in byk.items()},'| groups changed:',gch)
if 'base2' in W:
    b2=W['base2'];print('control base2 vs base: nodes differ',sum(x!=y for x,y in zip(base['nodes'],b2['nodes'])),'keys differ',sum(x!=y for x,y in zip(base['keys'],b2['keys'])),'groups differ',[g for g in base['groups'] if base['groups'][g]!=b2['groups'].get(g)][:3])
