import json,glob,os,sys
d=sys.argv[1]
rows=[]
for i in range(1,7):
    for kind in ('old','new'):
        f=glob.glob(f'{d}/{kind}-{i}-*.json')
        if not f or os.path.getsize(f[0])==0: rows.append((i,kind,None)); continue
        j=json.load(open(f[0])); p=[p for p in j['phases'] if p['label'].startswith('run')][0]
        c=j['profiles'][0]['counts']['closure']; calls=j['calls']
        rows.append((i,kind,dict(name=p['label'][4:],wall=p['wallMs']/1000,cpu=p['cpuMs']/1000,peak=p['peakMB'],derived=c['derived'],loaded=c['loaded'],stale=c['stale'],fetched=c['fetched'],stat=calls.get('p.stat',0),write=calls.get('p.writeFile',0),err=bool(p.get('error')))))
tot={'old':[0,0,0,0],'new':[0,0,0,0]}
for i,kind,r in rows:
    if r is None: print(i,kind,'MISSING'); continue
    print(f"{i} {kind:3} {r['name'][12:44]:32} wall {r['wall']:6.1f}s cpu {r['cpu']:6.1f}s peak {r['peak']:5}MB derived {r['derived']:2} loaded {r['loaded']:2} stale {r['stale']} fetched {r['fetched']:2} stat {r['stat']:6} write {r['write']:6} err {r['err']}")
    t=tot[kind]; t[0]+=r['wall']; t[1]+=r['cpu']; t[2]=max(t[2],r['peak']); t[3]+=1
for k,t in tot.items(): print(k,'owners',t[3],'sum wall %.0fs sum cpu %.0fs max peak %dMB  mean wall %.1fs cpu %.1fs'%(t[0],t[1],t[2],t[0]/max(1,t[3]),t[1]/max(1,t[3])))
