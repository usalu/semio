import json,glob,time,sys,os
base="/Users/Shared/semio-store/v1/origins"
def row(p):
    try:d=json.load(open(p))
    except Exception:return None
    s=d.get('sources',{});o=s.get('owners') or {};cc=s.get('closureCounts') or {};w=o.get('warm') or {};led=o.get('ledger') or {}
    chk=cc.get('checked') or 0
    return dict(origin=p.split('/')[-2],age=int(time.time()-d['atMs']/1000),up=int(d.get('uptimeSeconds') or 0),phase=d.get('phase'),taken=o.get('taken'),jobs=o.get('jobs'),missing=led.get('missing'),warm=(w.get('done'),w.get('total'),w.get('settled')) if w else None,derived=cc.get('derived'),loaded=cc.get('loaded'),stale=cc.get('stale'),fetched=cc.get('fetched'),checked=chk,staleShare=round((cc.get('stale') or 0)/chk,3) if chk else None)
for p in sorted(glob.glob(base+"/g1-m*/slice-memory.json")):
    r=row(p)
    if r:print(json.dumps(r))
for p in sorted(glob.glob(base+"/g1-m*/ready.json")):
    try:d=json.load(open(p));print(p.split('/')[-2],'ready',d.get('ready'),str(d)[:140])
    except Exception:pass
