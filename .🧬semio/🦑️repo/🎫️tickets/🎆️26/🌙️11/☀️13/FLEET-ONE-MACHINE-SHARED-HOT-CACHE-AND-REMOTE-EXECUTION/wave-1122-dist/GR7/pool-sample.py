import json,glob,time,os,sys
now=time.time()*1000
rows=[];tot=dict(children=0,running=0,slots=0,units=0,published=0,opm=0.0,alive=0,idle_free=0)
for f in sorted(glob.glob('/Users/Shared/semio-store/v1/origins/*/pool.json')):
    try: p=json.load(open(f))
    except Exception: continue
    age=(now-p.get('updatedMs',0))/1000
    if age>120: continue
    o=p['origin']; tot['alive']+=1
    for k,t in (('children','children'),('running','running'),('slots','slots'),('published','published')): tot[t]+=p.get(k,0) or 0
    tot['opm']+=p.get('ownersPerMinute',0) or 0
    rows.append((o,p.get('state'),p.get('children'),p.get('running'),p.get('target'),p.get('published'),p.get('stored'),p.get('ownersPerMinute'),p.get('reason'),p.get('units'),p.get('wave','')[:8]))
print(time.strftime('%H:%M:%S'),json.dumps(tot))
for r in rows: print(*r)
