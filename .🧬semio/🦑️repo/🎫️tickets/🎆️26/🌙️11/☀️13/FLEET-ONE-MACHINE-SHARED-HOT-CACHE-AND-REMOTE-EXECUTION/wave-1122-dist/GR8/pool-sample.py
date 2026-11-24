import json,glob,time,sys
now=time.time()*1000
tot=dict(alive=0,slots=0,target=0,children=0,running=0,claimed=0,published=0,stored=0,failed=0,held=0,opm=0.0,units=0)
rows=[]
for f in sorted(glob.glob('/Users/Shared/semio-store/v1/origins/*/pool.json')):
    try: p=json.load(open(f))
    except Exception: continue
    if (now-p.get('updatedMs',0))/1000>120: continue
    tot['alive']+=1
    for k in ('slots','target','children','running','claimed','published','stored','failed','held'): tot[k]+=p.get(k,0) or 0
    tot['opm']+=p.get('ownersPerMinute',0) or 0
    rows.append((p['origin'],p.get('state'),'ch',p.get('children'),'run',p.get('running'),'tgt',p.get('target'),'clm',p.get('claimed'),'pub',p.get('published'),'sto',p.get('stored'),'fail',p.get('failed'),json.dumps(p.get('failures')),p.get('reason'),'units',p.get('units'),(p.get('wave') or '')[:8],round(p.get('rssMegabytes') or 0)))
print(time.strftime('%H:%M:%S'),json.dumps(tot))
if len(sys.argv)>1:
    for r in rows: print(*r)
