import json,sys
p=sys.argv[1]
a=json.load(open(p,encoding='utf8'))['asset']
for k,r in a['representations'].items():
    for kk,v in (r.get('decisions') or {}).items():
        if isinstance(v,dict) and 'en' in v: print(k,kk,'\n  EN',v['en'],'\n  DE',v['de'])
    print(k,'ELEMENTS',[e.get('id') for e in (r.get('elements') or r.get('drawing',{}).get('elements',[]) or [])])
