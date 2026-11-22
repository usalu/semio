import json,sys,os,hashlib
G=sys.argv[1]; ROOT='/Users/ueli/Documents/semio'
MANUAL=('📥️load/🟦️.ts','🧵️interaction/🦀️.rs')
plan=json.load(open(G+'/plan.json'))
seen={}
for r in plan: seen[r['path']]=r
final=[]
for p,r in seen.items():
    if p.endswith('Cargo.lock'): r['action']='lockfile'; final.append(r); continue
    if p.endswith(MANUAL) and 'interaction/🦀️.rs' in p and '🧪️tests' not in p or p.endswith('📥️load/🟦️.ts'):
        r['action']='already-recovered'; r['note']='repaired by hand earlier'; final.append(r); continue
    if r['action']=='merged':
        f=os.path.join(ROOT,p); cur=open(f,'rb').read()
        if hashlib.sha256(cur).hexdigest()[:8]!=r['cur']: r['action']='aborted-changed'; final.append(r); continue
        out=open(r['td']+'/out','rb').read()
        if hashlib.sha256(open(f,'rb').read()).hexdigest()[:8]!=r['cur']: r['action']='aborted-changed'; final.append(r); continue
        open(f,'wb').write(out)
    final.append(r)
json.dump(final,open(G+'/final.json','w'),ensure_ascii=False,indent=1)
import collections; print(collections.Counter(r['action'] for r in final))
