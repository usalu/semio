import json,glob,time,subprocess,re
def snap():
    out={}
    for f in glob.glob('/Users/Shared/semio-store/v1/origins/*/pool.json'):
        try:p=json.load(open(f))
        except Exception:continue
        if (time.time()*1000-p.get('updatedMs',0))/1000>120:continue
        out[p['origin']]=(p.get('startedMs'),p.get('published',0) or 0,p.get('running',0) or 0,p.get('children',0) or 0,p.get('slots',0) or 0,p.get('claimed',0) or 0)
    return out
def last():
    t=open('/Users/ueli/Documents/semio/.cache/f/graph-server/read.log',errors='replace').read().split('\n')
    L=[l for l in t if l.startswith('[owners] owners ')]
    return L[-1][:200] if L else ''
prev=snap();t0=time.time();pub=0;claim=0;samples=[]
for i in range(8):
    time.sleep(30)
    cur=snap();dp=0;dc=0
    for o,v in cur.items():
        if o in prev and prev[o][0]==v[0]:
            dp+=max(0,v[1]-prev[o][1]);dc+=max(0,v[5]-prev[o][5])
    pub+=dp;claim+=dc
    samples.append((time.strftime('%H:%M:%S'),sum(v[2] for v in cur.values()),sum(v[3] for v in cur.values()),sum(v[4] for v in cur.values()),dp,dc))
    prev=cur
el=(time.time()-t0)/60
print('minutes',round(el,1),'published delta',pub,'claim delta',claim,'owners/min published',round(pub/el,2))
for s in samples:print(*s)
print(last())
