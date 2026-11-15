import json,sys,subprocess,re
BUN,sc,slug,src=sys.argv[1:5]
combos=json.loads(sys.argv[5])
for (ri,ro) in combos:
    d=json.load(open(src))
    for s in d['volumes'][sc]['body']:
        if s['id'].startswith('rear-tyre'):
            lo=min(p[0] for p in s['profile']);hi=max(p[0] for p in s['profile'])
            s['profile']=[[lo,ri],[hi,ri],[hi,ro],[lo,ro],[lo,ri]]
    p='🗑️generated/author-27/ringtest.json'; json.dump(d,open(p,'w'))
    r=subprocess.run([BUN,'volumes-check.ts','humans/'+slug,'--scale',sc,'--rules','volumes-silhouette','--source',p],capture_output=True,text=True)
    line=[l for l in r.stdout.splitlines() if 'iou' in l]
    print(ri,ro,re.findall(r'iou:(\w+)=([0-9.]+)',line[0]) if line else '')
