import json,sys,subprocess,os
BUN=sys.argv[1]; sc=sys.argv[2]; slug=sys.argv[3]; src=sys.argv[4]
variants=json.loads(sys.argv[5])  # name -> list of id prefixes to drop
d=json.load(open(src))
for name,drop in variants.items():
    e=json.loads(json.dumps(d))
    b=e['volumes'][sc]['body']
    e['volumes'][sc]['body']=[s for s in b if not any(s['id'].startswith(p) for p in drop)]
    p=f'🗑️generated/author-27/var-{name}.json'
    json.dump(e,open(p,'w'))
    r=subprocess.run([BUN,'volumes-check.ts','humans/'+slug,'--scale',sc,'--rules','volumes-silhouette','--source',p],capture_output=True,text=True)
    line=[l for l in r.stdout.splitlines() if 'silhouette' in l and 'iou' in l]
    import re
    print(name, re.findall(r'iou:(\w+)=([0-9.]+)',line[0]) if line else r.stdout[-200:])
