import json,sys,collections
p=json.load(open(sys.argv[1]));nodes={n['id']:n for n in p['nodes']}
tot=collections.Counter();
dt=p['timeDeltas'];s=p['samples']
t0=p['startTime'];
t=0;win=float(sys.argv[2]) if len(sys.argv)>2 else None
by=collections.Counter();
for i,(sid,d) in enumerate(zip(s,dt)):
    n=nodes[sid];cf=n['callFrame'];k=f"{cf['functionName'] or '(anon)'} {cf['url'].split('/')[-2:]} :{cf['lineNumber']}"
    by[k]+=d
print('total ms',sum(dt)/1000)
for k,v in by.most_common(25):print(round(v/1000),k)
