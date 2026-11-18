import json,glob,time,os
root='/Users/Shared/semio-store/v1/origins'
tot={'claimed':0,'published':0,'children':0,'rate':0.0};rows=[]
wave=(json.load(open(root+'/g1-server/pool.json')).get('wave') or '')[:12]
for f in sorted(glob.glob(root+'/*/pool.json')):
    try: d=json.load(open(f))
    except Exception: continue
    o=f.split('/')[-2]
    age=time.time()-os.path.getmtime(f)
    if (d.get('wave') or '').startswith(wave) and age<120:
        tot['claimed']+=d.get('claimed',0);tot['published']+=d.get('published',0);tot['children']+=d.get('children',0);tot['rate']+=d.get('ownersPerMinute',0)
    rows.append('%s:%s/%s/%dc/%s'%(o[-3:],d.get('claimed'),d.get('published'),d.get('children',0),(d.get('reason') or '')[:8]))
print('wave',wave,'claimed',tot['claimed'],'published',tot['published'],'children',tot['children'],'opm',round(tot['rate'],1),'|',' '.join(rows))
