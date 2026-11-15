import json,sys
C="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/%s/🔣️.json"
a=json.load(open(C%sys.argv[1]))['asset']
for s in ['50','100','200','500']:
    r=a['representations'][s]
    if r['status']!='authored': continue
    for v in ['elevation','front','plan']:
        ids=[(g['id'],g['bone']) for g in r['geometry'].get(v,[]) if g.get('filled') is not False]
        b={}
        for i,bn in ids: b.setdefault(bn,[]).append(i)
        print(s,v,{k:(len(x) if len(x)>3 else x) for k,x in b.items()})
    dec=r['decisions']
    for k,x in dec.items(): print('   ',k,':',x['en'][:260] if isinstance(x,dict) else str(x)[:200])
