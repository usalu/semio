import json,sys
a=sys.argv[1]; sc=sys.argv[2:] 
d=json.load(open(f'volumes/{a}.json'))['volumes']
for s in (sc or d):
    v=d[s]
    if v.get('status')!='authored': print(s,'unavailable'); continue
    for l in ('body','thermal','structure'):
        print(s,l,len(v[l]),' '.join((x['id']+':'+(x.get('kind') or 'of='+x.get('of','?'))+(('/'+x['region']) if 'region' in x else '')+(('/'+x['part']) if 'part' in x else '')) for x in v[l]))
