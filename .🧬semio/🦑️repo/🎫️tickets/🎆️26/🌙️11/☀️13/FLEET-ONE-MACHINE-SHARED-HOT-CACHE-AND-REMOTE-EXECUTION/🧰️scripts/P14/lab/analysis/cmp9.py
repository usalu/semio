import pickle,json,collections
rows=pickle.load(open('p14/rows.pkl','rb'))
by=collections.defaultdict(lambda:collections.defaultdict(list))
for r in rows: by[r['root']][(r['plat'],r['prod'][:22])].append(r)
c=collections.Counter()
def rec(r):
    A=json.loads(r['img']);return A,sorted(json.dumps(x,sort_keys=True) for x in (A.get('closureRecipes') or {}).values()),sorted((A.get('closureRecipes') or {}))
for root,v in by.items():
    ks=sorted(v)
    for i in range(len(ks)):
        for j in range(i+1,len(ks)):
            a=max(v[ks[i]],key=lambda r:r['t']);b=max(v[ks[j]],key=lambda r:r['t'])
            A,ra,ka=rec(a);B,rb,kb=rec(b)
            if not ra and not rb: continue
            pl=(ks[i][0],ks[j][0]) if ks[i][0]!=ks[j][0] else ('same:'+ks[i][0],)
            c[(pl,'recipes-equal' if ra==rb else 'recipes-differ','keys-equal' if ka==kb else 'keys-differ')]+=1
for k,v in sorted(c.items(),key=str):print(k,v)
