import json,subprocess,re,sys
asset,scale,key=sys.argv[1:4]
base=json.load(open('volumes/furniture/%s.json'%asset))
BUN=subprocess.check_output("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1",shell=True,text=True).strip()
def run(d):
    json.dump(d,open('🗑️generated/author-16/var.json','w'))
    out=subprocess.run([BUN,'volumes-check.ts','furniture/'+asset,'--scale',scale,'--rules','volumes-structure-coverage','--source','🗑️generated/author-16/var.json'],capture_output=True,text=True).stdout
    m=re.search(key+r'=([0-9.e-]+)',out); return float(m.group(1)) if m else None
ref=run(base); print('base',ref)
for i in [s['id'] for s in base['volumes'][scale]['structure']]:
    d=json.loads(json.dumps(base)); d['volumes'][scale]['structure']=[s for s in d['volumes'][scale]['structure'] if s['id']!=i]
    v=run(d)
    if v is not None and ref is not None and v<ref-1e-4: print(i,v)
